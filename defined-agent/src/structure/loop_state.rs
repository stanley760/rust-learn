use anyhow::Context;
use async_openai::{Client, config::OpenAIConfig};
use inquire::Select;
use tracing::{error, info};
use crate::context::{CompactState, estimate_context_size, micro_compact, persist_large_output};
use crate::hook::{self, Hook, HookControl, HookTypes, PostToolUseFn, PreToolUseFn, SessionStartFn, ToolResult, ToolUse};
use crate::invoke_hooks;
use crate::permission::{PermissionBehavior, PermissionDecision, PermissionManager, PermissionMode};
use crate::tools::Tool;
use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessage,
    ChatCompletionRequestAssistantMessageContent, ChatCompletionRequestAssistantMessageContentPart,
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestSystemMessageContentPart,
    ChatCompletionRequestToolMessage, ChatCompletionRequestToolMessageContent,
    ChatCompletionRequestToolMessageContentPart,
    ChatCompletionRequestUserMessageContent, ChatCompletionRequestUserMessageContentPart,
    ChatCompletionTools, CreateChatCompletionRequest, FinishReason,
};
use std::collections::HashMap;

const CONTEXT_LIMIT: usize = 50000;

pub fn get_model() -> anyhow::Result<String> {
    dotenvy::dotenv()?;
    std::env::var("OPENAI_MODEL").context("OPENAI_MODEL is not set")
}

pub fn get_llm_client() -> anyhow::Result<Client<OpenAIConfig>> {
    dotenvy::dotenv()?;
    let client: Client<OpenAIConfig> = Client::with_config(OpenAIConfig::default());
    Ok(client)
}

pub fn system_message(content: impl Into<String>) -> ChatCompletionRequestMessage {
    ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
        content: ChatCompletionRequestSystemMessageContent::Text(content.into()),
        name: None,
    })
}

pub fn tool_result_message(
    tool_call_id: impl Into<String>,
    content: impl Into<String>,
) -> ChatCompletionRequestMessage {
    ChatCompletionRequestMessage::Tool(ChatCompletionRequestToolMessage {
        content: ChatCompletionRequestToolMessageContent::Text(content.into()),
        tool_call_id: tool_call_id.into(),
    })
}

pub struct LoopState {
    pub client: Client<OpenAIConfig>,
    pub context: Vec<ChatCompletionRequestMessage>,
    tools: HashMap<String, Box<dyn Tool>>,
    pub compact_state: CompactState,
    pub manager: PermissionManager,
    pub hooks: Vec<Hook>,
}

impl LoopState {
    pub fn new(client: Client<OpenAIConfig>, tools: HashMap<String, Box<dyn Tool>>, manager: PermissionManager) -> Self {
        Self {
            client,
            context: Vec::new(),
            tools,
            compact_state: CompactState::default(),
            manager,
            hooks: Vec::new(),
        }
    }

    pub async fn agent_loop(&mut self) -> anyhow::Result<()> {
        let system = format!(
            r#"You are a coding agent at {}.
Keep working step by step, and use compact if the conversation gets too long.
"#,
            std::env::current_dir()?.display(),
        );

        loop {
            micro_compact(&mut self.context);

            if estimate_context_size(&self.context) > CONTEXT_LIMIT {
                println!("[auto compact]");
                self.compact_history(None).await?;
            }

            // 构造请求：system message 放在 messages 开头
            let mut messages = vec![system_message(&system)];
            messages.extend(self.context.clone());

            let tool_vec: Vec<ChatCompletionTools> = self
                .tools
                .values()
                .map(|t| t.tool_spec().into_openai_tool())
                .collect();

            let request = CreateChatCompletionRequest {
                messages,
                model: get_model()?,
                tools: Some(tool_vec),
                max_completion_tokens: Some(8000),
                ..Default::default()
            };

            let response = self.client.chat().create(request).await?;

            let choice = response
                .choices
                .into_iter()
                .next()
                .context("no response choice")?;

            let msg = choice.message;

            // 把 assistant 回复加入上下文
            self.context.push(ChatCompletionRequestMessage::Assistant(
                ChatCompletionRequestAssistantMessage {
                    content: msg
                        .content
                        .map(ChatCompletionRequestAssistantMessageContent::Text),
                    refusal: msg.refusal,
                    name: None,
                    audio: None,
                    tool_calls: msg.tool_calls.clone(),
                    #[allow(deprecated)]
                    function_call: None,
                },
            ));

            // 判断是否需要继续调用工具
            if !matches!(choice.finish_reason, Some(FinishReason::ToolCalls)) {
                return Ok(());
            }

            self.execute_tool_call(&msg.tool_calls).await?;
        }
    }

    pub async fn execute_tool_call(
        &mut self,
        tool_calls: &Option<Vec<ChatCompletionMessageToolCalls>>,
    ) -> anyhow::Result<()> {
        let Some(calls) = tool_calls else {
            return Ok(());
        };

        let mut manual_compact = false;
        let mut compact_focus: Option<String> = None;

        for call in calls {
            let mut result = Vec::new();
            let (id, name, arguments) = match call {
                ChatCompletionMessageToolCalls::Function(f) => {
                    (&f.id, f.function.name.clone(), f.function.arguments.clone())
                }
                ChatCompletionMessageToolCalls::Custom(c) => (
                    &c.id,
                    c.custom_tool.name.clone(),
                    c.custom_tool.input.clone(),
                ),
            };

            let input: serde_json::Value = serde_json::from_str(&arguments).unwrap_or_default();
            let mut tool_use = ToolUse {
                id: id.clone(),
                name: name.clone(),
                input: input.clone(),
            };

            if let HookControl::Block(reason) = invoke_hooks!(PreToolUse, self,&mut tool_use)? {
               
                result.push(ChatCompletionRequestMessage::Tool(
                    ChatCompletionRequestToolMessage {
                        tool_call_id: tool_use.id.clone(),
                        content: ChatCompletionRequestToolMessageContent::Text(format!("Tool blocked by PreToolUse hook: {reason}")),
                    },
                ));
                continue;
            }
            // Permission check
            let decision = self.manager.check(&tool_use.name, &tool_use.input);
            let output = match decision {
                PermissionDecision {
                    behavior: PermissionBehavior::Deny,
                    reason,
                } => {
                    info!("  [DENIED] {}: {}", name, reason);
                    format!("Permission denied: {}", reason)
                }
                PermissionDecision {
                    behavior: PermissionBehavior::Allow,
                    reason: _,
                } => self.execute(id, &name, &input).await,
                PermissionDecision {
                    behavior: PermissionBehavior::Ask,
                    reason: _reason,
                } => {
                    if self.manager.ask_user(&name, &input)? {
                        self.execute(id, &name, &input).await
                    } else {
                        println!("  [USER DENIED] {name}");
                        format!("Permission denied by user for {name}")
                    }
                }
            };
            let mut tool_result = ToolResult {
                tool_use_id: tool_use.id.clone(),
                content: output,
            };
            if let hook::HookControl::Block(reason) =
                invoke_hooks!(PostToolUse, self, &tool_use, &mut tool_result)?
            {
                tool_result.content = format!("Tool blocked by PostToolUse hook: {reason}");
            }
            // OpenAI: 每个 tool result 是一条独立的 Tool role message
            self.context.push(tool_result_message(tool_result.tool_use_id, tool_result.content));

            if name == "read_file" 
                && let Some(path) = input.get("path").and_then(|v| v.as_str()) {
                self.remember_recent_file(path);
            }
            if name == "compact" {
                println!("[manual compact]");
                manual_compact = true;
                compact_focus = input
                    .get("focus")
                    .and_then(|v| v.as_str())
                    .map(String::from);
            }
        }

        if manual_compact {
            self.compact_history(compact_focus.as_deref())
                .await
                .context("manual compact failed")?;
        }
        Ok(())
    }

    pub fn handle_mode_command(&mut self, query: &str) -> anyhow::Result<()> {
        let parts: Vec<&str> = query.split_whitespace().collect::<Vec<_>>();

        let mode = if parts.len() == 2 {
            parts[1].parse::<PermissionMode>().with_context(|| {
                format!(
                    "unknown mode: {}. Usage: /mode <default|plan|auto>",
                    parts[1]
                )
            })?
        } else {
            Select::new(
                "Switch permission mode:",
                vec![
                    PermissionMode::Default,
                    PermissionMode::Plan,
                    PermissionMode::Auto,
                ],
            )
            .prompt()
            .context("An error happened or user cancelled the input.")?
        };

        self.manager.set_mode(mode);
        println!("[Switched to {}]", self.manager.mode());

        Ok(())
    }


    async fn execute(
        &mut self,
        tool_call_id: &str,
        name: &str,
        input: &serde_json::Value,
    ) -> String {
        let Some(tool) = self.tools.get_mut(name) else {
            return format!("Unknown tool: {name}");
        };

        match tool.invoke(input).await {
            Ok(output) => {
                let output = if name == "bash" {
                    match persist_large_output(tool_call_id, &output) {
                        Ok(compacted) => compacted,
                        Err(e) => format!("Error persisting large output: {}", e),
                    }
                } else {
                    output
                };

                info!(
                    "Command:{}\n arg:{}\n output:\n{}\n",
                    name,
                    input,
                    output.chars().take(200).collect::<String>()
                );
                output
            }
            Err(e) => {
                error!("Error invoking tool {}: {}", name, e);
                format!("Error invoking tool {}: {}", name, e)
            }
        }
    }

    pub fn session_start(&mut self, hook: impl SessionStartFn + 'static) {
        self.hooks.push(Hook::SessionStart(Box::new(hook)));
    }

    pub fn pre_tool(&mut self, hook: impl PreToolUseFn + 'static) {
        self.hooks.push(Hook::PreToolUse(Box::new(hook)));
    }

    pub fn post_tool(&mut self, hook: impl PostToolUseFn + 'static) {
        self.hooks.push(Hook::PostToolUse(Box::new(hook)));
    }

    pub fn hook_by_type(&self, hook_type: HookTypes) -> Vec<&Hook> {
        self.hooks
            .iter()
            .filter(|hook| hook_type == (*hook).into())
            .collect()
    }
}

pub fn extract_text(message: &ChatCompletionRequestMessage) -> String {
    match message {
        ChatCompletionRequestMessage::User(m) => match &m.content {
            ChatCompletionRequestUserMessageContent::Text(t) => t.clone(),
            ChatCompletionRequestUserMessageContent::Array(parts) => parts
                .iter()
                .filter_map(|p| match p {
                    ChatCompletionRequestUserMessageContentPart::Text(t) => Some(t.text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n"),
        },
        ChatCompletionRequestMessage::Assistant(m) => match &m.content {
            Some(ChatCompletionRequestAssistantMessageContent::Text(t)) => t.clone(),
            Some(ChatCompletionRequestAssistantMessageContent::Array(parts)) => parts
                .iter()
                .filter_map(|p| match p {
                    ChatCompletionRequestAssistantMessageContentPart::Text(t) => {
                        Some(t.text.as_str())
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n"),
            None => String::new(),
        },
        ChatCompletionRequestMessage::Tool(m) => match &m.content {
            ChatCompletionRequestToolMessageContent::Text(t) => t.clone(),
// [CLIPPY-WARNING] unnecessary_filter_map (line 254)
            ChatCompletionRequestToolMessageContent::Array(parts) => parts
                .iter()
                .map(|p| match p {
                    ChatCompletionRequestToolMessageContentPart::Text(t) => t.text.as_str(),
                })
                .collect::<Vec<_>>()
                .join("\n"),
        },
        ChatCompletionRequestMessage::System(m) => match &m.content {
// [CLIPPY-WARNING] unnecessary_filter_map (line 264)
            ChatCompletionRequestSystemMessageContent::Text(t) => t.clone(),
            ChatCompletionRequestSystemMessageContent::Array(parts) => parts
                .iter()
                .map(|p| match p {
                    ChatCompletionRequestSystemMessageContentPart::Text(t) => t.text.as_str(),
                })
                .collect::<Vec<_>>()
                .join("\n"),
        },
        _ => String::new(),
    }

}
