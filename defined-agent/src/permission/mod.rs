mod constant;
mod validator;

pub use validator::*;

/// Permission-related utilities and mode definitions.
///
/// This module exposes permission types and helper logic used by the
/// defined agent to control how permission requests are handled.
use anyhow::Context;
use inquire::Select;
use serde_json::Value;
use strum_macros::EnumString;
use wildmatch::WildMatch;

use crate::permission::constant::{is_read_only_tool, is_write_tool};

/// Represents the permission handling mode for the defined agent.
///
/// - `Default`: prompt the user to explicitly allow or deny each request.
/// - `Plan`: operate in read-only mode and do not perform writes.
/// - `Auto`: allow read operations automatically and prompt only for writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PermissionMode {
    /// Ask the user to allow or deny each request.
    Default,
    /// Run in a safe read-only planning mode.
    Plan,
    /// Allow reads automatically and ask for confirmation on writes.
    Auto,
}

impl std::fmt::Display for PermissionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            PermissionMode::Default => "default - ask for allow or deny it",
            PermissionMode::Plan => "plan - read only",
            PermissionMode::Auto => "auto - allow reads, ask for writes",
        };
        write!(f, "{display}")
    }
}

/// Defines the decision that can be taken for a permission request.
///
/// This enum is typically used to decide whether a request should be
/// allowed, denied, or deferred for explicit user confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionBehavior {
    /// Automatically allow the request.
    Allow,
    /// Automatically deny the request.
    Deny,
    /// Ask the user for permission before proceeding.
    Ask,
}

/// Represents the result of a permission evaluation.
///
/// Contains the chosen behavior and an optional explanation reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionDecision {
    pub behavior: PermissionBehavior,
    pub reason: String,
}

/// User-facing choices for a single permission prompt.
///
/// - `AllowOnce`: approve this request only.
/// - `Deny`: reject the request.
/// - `AlwaysAllow`: approve and remember this choice for future requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionUserChoice {
    AllowOnce,
    Deny,
    AlwaysAllow,
}

impl std::fmt::Display for PermissionUserChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            PermissionUserChoice::AllowOnce => "allow to run once",
            PermissionUserChoice::Deny => "deny it",
            PermissionUserChoice::AlwaysAllow => "allow automatically",
        };

        write!(f, "{result}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionRule {
    pub tool: String,
    pub path: Option<String>,
    pub content: Option<String>,
    pub behavior: PermissionBehavior,
}

impl std::fmt::Display for PermissionRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tool={}", self.tool)?;

        if let Some(path) = &self.path {
            write!(f, ", path={path}")?;
        }

        if let Some(content) = &self.content {
            write!(f, ", content={content}")?;
        }

        let behavior = match self.behavior {
            PermissionBehavior::Allow => "allow",
            PermissionBehavior::Deny => "deny",
            PermissionBehavior::Ask => "ask",
        };
        write!(f, ", behavior={behavior}")
    }
}

impl PermissionRule {
    pub fn allow_tool(tool: impl Into<String>) -> Self {
        Self {
            tool: tool.into(),
            path: None,
            content: None,
            behavior: PermissionBehavior::Allow,
        }
    }

    pub fn deny_tool_content(tool: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            tool: tool.into(),
            path: None,
            content: Some(content.into()),
            behavior: PermissionBehavior::Deny,
        }
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    fn matches(&self, tool_name: &str, tool_input: &Value) -> bool {
        if self.tool != "*" && self.tool != tool_name {
            return false;
        }

        if let Some(path_pattern) = &self.path {
            let path = tool_input.get("path").and_then(Value::as_str).unwrap_or("");
            if !WildMatch::new(path_pattern).matches(path) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
pub struct PermissionManager {
    mode: PermissionMode,
    rules: Vec<PermissionRule>,
    bash_validator: BashSecurityValidator,
    consecutive_denials: usize,
    max_consecutive_denials: usize,
}

impl PermissionManager {
    pub fn mode(&self) -> PermissionMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: PermissionMode) {
        self.mode = mode;
    }

    pub fn rules(&self) -> &[PermissionRule] {
        &self.rules
    }

    pub fn try_new(mode: PermissionMode) -> anyhow::Result<Self> {
        Self::try_new_with_rules(mode, default_rules())
    }

    pub fn try_new_with_rules(
        mode: PermissionMode,
        rules: Vec<PermissionRule>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            mode,
            rules,
            bash_validator: BashSecurityValidator::try_new()?,
            consecutive_denials: 0,
            max_consecutive_denials: 3,
        })
    }

    pub fn check(&mut self, tool_name: &str, tool_input: &Value) -> PermissionDecision {
        if let Some(decision) = self.check_bash_security(tool_name, tool_input) {
            return decision;
        }

        if let Some(decision) = self.match_rule(tool_name, tool_input, PermissionBehavior::Deny) {
            return decision;
        }

        if let Some(decision) = self.check_mode(tool_name) {
            return decision;
        }

        if let Some(decision) = self.match_rule(tool_name, tool_input, PermissionBehavior::Allow) {
            self.consecutive_denials = 0;
            return decision;
        }

        PermissionDecision {
            behavior: PermissionBehavior::Ask,
            reason: format!("No rule matched for {tool_name}, asking user"),
        }
    }

    fn check_bash_security(
        &self,
        tool_name: &str,
        tool_input: &Value,
    ) -> Option<PermissionDecision> {
        if tool_name != "bash" {
            return None;
        }

        let command = tool_input
            .get("command")
            .and_then(Value::as_str)
            .unwrap_or("");
        let failures = self.bash_validator.validate(command);
        if failures.is_empty() {
            return None;
        }

        let severe_rules = ["sudo", "rm_rf"];
        let has_severe_hit = failures
            .iter()
            .any(|failure| severe_rules.contains(&failure.name));
        let summary = self.bash_validator.describe_failures(command);

        Some(if has_severe_hit {
            PermissionDecision {
                behavior: PermissionBehavior::Deny,
                reason: format!("Bash validator: {summary}"),
            }
        } else {
            PermissionDecision {
                behavior: PermissionBehavior::Ask,
                reason: format!("Bash validator flagged: {summary}"),
            }
        })
    }

    fn match_rule(
        &self,
        tool_name: &str,
        tool_input: &Value,
        behavior: PermissionBehavior,
    ) -> Option<PermissionDecision> {
        self.rules
            .iter()
            .find(|rule| rule.behavior == behavior && rule.matches(tool_name, tool_input))
            .map(|rule| PermissionDecision {
                behavior,
                reason: match behavior {
                    PermissionBehavior::Allow => format!("Matched allow rule: {rule:?}"),
                    PermissionBehavior::Deny => format!("Blocked by deny rule: {rule:?}"),
                    PermissionBehavior::Ask => format!("Matched ask rule: {rule:?}"),
                },
            })
    }

    fn check_mode(&self, tool_name: &str) -> Option<PermissionDecision> {
        match self.mode {
            PermissionMode::Default => None,
            PermissionMode::Plan => {
                if is_write_tool(tool_name) {
                    Some(PermissionDecision {
                        behavior: PermissionBehavior::Deny,
                        reason: "Plan mode: write operations are blocked".to_string(),
                    })
                } else {
                    Some(PermissionDecision {
                        behavior: PermissionBehavior::Allow,
                        reason: "Plan mode: read-only allowed".to_string(),
                    })
                }
            }
            PermissionMode::Auto => {
                if is_read_only_tool(tool_name) || tool_name == "read_file" {
                    Some(PermissionDecision {
                        behavior: PermissionBehavior::Allow,
                        reason: "Auto mode: read-only tool auto-approved".to_string(),
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn ask_user(&mut self, tool_name: &str, tool_input: &Value) -> anyhow::Result<bool> {
        let preview = truncate_for_prompt(tool_input, 200);
        let prompt = format!("[Permission] {tool_name}: {preview}");

        let choice = Select::new(
            &prompt,
            vec![
                PermissionUserChoice::AllowOnce,
                PermissionUserChoice::Deny,
                PermissionUserChoice::AlwaysAllow,
            ],
        )
        .prompt()
        .context("failed to read permission decision")?;

        let approved = self.apply_user_choice(choice, tool_name);
        if !approved && self.should_suggest_plan_mode() {
            println!(
                "[{} consecutive denials -- consider switching to plan mode]",
                self.consecutive_denials
            );
        }

        Ok(approved)
    }

    fn apply_user_choice(&mut self, choice: PermissionUserChoice, tool_name: &str) -> bool {
        match choice {
            PermissionUserChoice::AllowOnce => {
                self.consecutive_denials = 0;
                true
            }
            PermissionUserChoice::Deny => {
                self.consecutive_denials += 1;
                false
            }
            PermissionUserChoice::AlwaysAllow => {
                self.rules
                    .push(PermissionRule::allow_tool(tool_name).with_path("*"));
                self.consecutive_denials = 0;
                true
            }
        }
    }

    fn should_suggest_plan_mode(&self) -> bool {
        self.consecutive_denials >= self.max_consecutive_denials
    }
}

pub fn default_rules() -> Vec<PermissionRule> {
    vec![
        PermissionRule::deny_tool_content("bash", "rm -rf /"),
        PermissionRule::deny_tool_content("bash", "sudo *"),
        PermissionRule::allow_tool("read_file").with_path("*"),
    ]
}

fn truncate_for_prompt(value: &Value, limit: usize) -> String {
    let text = value.to_string();
    if text.chars().count() <= limit {
        return text;
    }

    let truncated = text.chars().take(limit).collect::<String>();
    format!("{truncated}...")
}
