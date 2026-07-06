use anyhow::Context;
use async_openai::{Client, config::OpenAIConfig};

use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessage, ChatCompletionRequestAssistantMessageContent, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestToolMessage, ChatCompletionRequestToolMessageContent, CreateChatCompletionRequestArgs, FinishReason,
};
use std::collections::HashMap;
use crate::tools::{Tool, all_tools};

fn get_model() -> anyhow::Result<String> {
    dotenvy::dotenv()?;
    std::env::var("OPENAI_MODEL").context("OPENAI_MODEL is not set")
}

const SYSTEM: &str = if cfg!(target_os = "windows") {
    r#"You are a coding agent.
Use cmd.exe to inspect and change the workspace. Act first, then report clearly.
IMPORTANT: You are on Windows. Use Windows commands only:
- Use `dir` instead of `ls`, `type` instead of `cat`, `findstr` instead of `grep`.
- Use backslashes in paths or forward slashes (both work).
- Do NOT use Unix-only commands like ls, cat, grep, rm, chmod, etc.
"#
} else {
    r#"You are a coding agent.
Use bash to inspect and change the workspace. Act first, then report clearly.
"#
};

pub struct LoopState {
    client: Client<OpenAIConfig>,
    pub context: Vec<ChatCompletionRequestMessage>,
    tools: HashMap<String, Box<dyn Tool>>,
}

impl LoopState {
    pub fn new(client: Client<OpenAIConfig>, tools: HashMap<String, Box<dyn Tool>>) -> Self {
        Self {
            client,
            context: Vec::new(),
            tools,
        }
    }

    async fn execute(&mut self, name: &str, input: &serde_json::Value) -> anyhow::Result<String> {
        let Some(tool) = self.tools.get_mut(name) else {
            return Err(anyhow::anyhow!("Unknown tool: {name}"));
        };

        match tool.invoke(input).await {
            Ok(output) => {
                println!("Command:{}\n arg:{}\n output:\n{}\n", name, input, output);
                Ok(output)
            }
            Err(e) => {
                println!("Error invoking tool {}: {}", name, e);
                Err(anyhow::anyhow!("Error invoking tool {}: {}", name, e))
            }
        }
    }

    async fn execute_tool_calls(&mut self, tool_calls: &[ChatCompletionMessageToolCalls]) -> Vec<ChatCompletionRequestMessage> {
        let mut results = Vec::new();
        for tc in tool_calls.iter() {
            let ChatCompletionMessageToolCalls::Function(f) = tc else {
                continue;
            };

            let id = f.id.clone();
            let name = f.function.name.clone();
            let cmd = serde_json::from_str::<serde_json::Value>(&f.function.arguments)
                .unwrap_or_default();

            let output_str = self.execute(&name, &cmd)
                .await
                .map_or_else(|e| format!("Error: {}", e), |s| s);

            println!("Command '{}' output: {}", cmd, output_str);

            results.push(ChatCompletionRequestMessage::Tool(ChatCompletionRequestToolMessage {
                content: ChatCompletionRequestToolMessageContent::Text(output_str),
                tool_call_id: id,
            }));
        }
        results
    }
}

pub async fn agent_loop(state: &mut LoopState) -> anyhow::Result<()> {
    state.context.push(
        ChatCompletionRequestSystemMessageArgs::default()
            .content(SYSTEM)
            .build()?
            .into(),
    );

    loop {
        let request = CreateChatCompletionRequestArgs::default()
            .model(get_model()?)
            .messages(state.context.clone())
            .max_completion_tokens(8000u32)
            .tools(all_tools())
            .build()?;

        let response = state.client.chat().create(request).await?;

        let choice = response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No response choice"))?;

        let msg = choice.message;

        state.context.push(ChatCompletionRequestMessage::Assistant(
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

        match choice.finish_reason {
            Some(FinishReason::ToolCalls) => {}
            _ => return Ok(()),
        }

        let Some(tool_calls) = msg.tool_calls else {
            return Ok(());
        };

        if tool_calls.is_empty() {
            return Ok(());
        }

        let tool_results = state.execute_tool_calls(&tool_calls).await;
        state.context.extend(tool_results);
    }
}
