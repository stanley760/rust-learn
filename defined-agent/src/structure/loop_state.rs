use anyhow::Context;
use async_openai:: {Client, config::OpenAIConfig};

use async_openai::types::chat::{ 
    ChatCompletionRequestAssistantMessage, ChatCompletionRequestAssistantMessageContent, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestToolMessage, ChatCompletionRequestToolMessageContent, ChatCompletionTool, ChatCompletionTools, CreateChatCompletionRequestArgs, FinishReason, FunctionObjectArgs,
};

use crate::llm::execute_tool_calls;

fn get_model() -> anyhow::Result<String> {
    dotenvy::dotenv()?;
    std::env::var("OPENAI_MODEL").context("OPENAI_MODEL is not set")
}

const SYSTEM: &str = r#"You are a coding agent.
Use bash to inspect and change the workspace. Act first, then report clearly.
"#;

pub struct LoopState {
    client: Client<OpenAIConfig>,
    pub context: Vec<ChatCompletionRequestMessage>,
    turn_count: usize,
    transition_reason: Option<String>,
}

impl LoopState {
    pub fn new(client: Client<OpenAIConfig>) -> Self {
        Self {
            client,
            context: Vec::new(),
            turn_count: 0,
            transition_reason: None,
        }
    }
}

fn get_tools() -> Vec<ChatCompletionTools> {
    vec![ChatCompletionTools::Function(ChatCompletionTool {
        function: FunctionObjectArgs::default()
            .name("bash")
            .description("Run a shell command in the current workspace.")
            .parameters(serde_json::json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string"
                    }
                },
                "required": ["command"]
            }))
            .build()
            .expect("Failed to build FunctionObject"),
    })]
}

async fn run_one_turn(state: &mut LoopState) -> anyhow::Result<bool> {
    state.context.push(ChatCompletionRequestSystemMessageArgs::default()
                .content(SYSTEM)
                .build()?
                .into()
            );
    let request = CreateChatCompletionRequestArgs::default()
        .model(get_model()?)
        .messages(state.context.clone())
        .max_completion_tokens(8000u32)
        .tools(get_tools())
        .build()?;

    let response = state.client.chat().create(request).await?;

    let choice = response
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No response choice"))?;

    let msg = choice.message;

    // Push assistant message (with tool_calls if any) into context
    state.context.push(ChatCompletionRequestMessage::Assistant(
        ChatCompletionRequestAssistantMessage {
            content: msg.content.map(ChatCompletionRequestAssistantMessageContent::Text),
            refusal: msg.refusal,
            name: None,
            audio: None,
            tool_calls: msg.tool_calls.clone(),
            #[allow(deprecated)]
            function_call: None,
        },
    ));

    // Check finish reason: if not tool_calls, the agent is done
    match choice.finish_reason {
        Some(FinishReason::ToolCalls) => {}
        _ => {
            state.transition_reason = None;
            return Ok(false);
        }
    }

    // Execute tool calls and push tool result messages
    let Some(tool_calls) = msg.tool_calls else {
        state.transition_reason = None;
        return Ok(false);
    };

    if tool_calls.is_empty() {
        state.transition_reason = None;
        return Ok(false);
    }

    let tool_results = execute_tool_calls(&tool_calls).await;
    state.context.extend(tool_results);

    state.turn_count += 1;
    state.transition_reason = Some("tool_calls".to_string());
    Ok(true)
}

pub async fn agent_loop(state: &mut LoopState) -> anyhow::Result<()> {
    while run_one_turn(state).await? {}
    Ok(())
}
