use anyhow::{Context, Result};
use regex::Regex;

#[derive(Debug)]
pub struct ValidationRule {
    pub name: &'static str,
    pub pattern: &'static str,
    pub regex: Regex,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationFail {
    pub name: &'static str,
    pub pattern: &'static str,
}

#[derive(Debug, Default)]
pub struct BashSecurityValidator {
    validators: Vec<ValidationRule>,
}

impl BashSecurityValidator {
    pub fn try_new() -> Result<Self> {
        Ok(Self {
            validators: vec![
                ValidationRule {
                    name: "shell_metachar",
                    pattern: r"[;&|`$]",
                    regex: Regex::new(r"[;&|`$]")
                        .context("failed to compile shell_metachar regex")?,
                },
                ValidationRule {
                    name: "sudo",
                    pattern: r"\bsudo\b",
                    regex: Regex::new(r"\bsudo\b").context("failed to compile sudo regex")?,
                },
                ValidationRule {
                    name: "rm_rf",
                    pattern: r"\brm\s+(-[a-zA-Z]*)?r",
                    regex: Regex::new(r"\brm\s+(-[a-zA-Z]*)?r")
                        .context("failed to compile rm_rf regex")?,
                },
                ValidationRule {
                    name: "cmd_substitution",
                    pattern: r"\$\(",
                    regex: Regex::new(r"\$\(")
                        .context("failed to compile cmd_substitution regex")?,
                },
                ValidationRule {
                    name: "ifs_injection",
                    pattern: r"\bIFS\s*=",
                    regex: Regex::new(r"\bIFS\s*=")
                        .context("failed to compile ifs_injection regex")?,
                },
            ],
        })
    }

    pub fn validate(&self, command: &str) -> Vec<ValidationFail> {
        self.validators
            .iter()
            .filter(|rule| rule.regex.is_match(command))
            .map(|rule| ValidationFail {
                name: rule.name,
                pattern: rule.pattern,
            })
            .collect()
    }

    pub fn is_safe(&self, command: &str) -> bool {
        self.validate(command).is_empty()
    }

    pub fn describe_failures(&self, command: &str) -> String {
        let failures = self.validate(command);
        if failures.is_empty() {
            return "No issues detected".to_string();
        }

        let parts = failures
            .iter()
            .map(|failure| format!("{} (pattern: {})", failure.name, failure.pattern))
            .collect::<Vec<_>>()
            .join(", ");

        format!("Security flags: {parts}")
    }
}
