use std::time::Duration;

use anyhow::Context;
use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessage, ChatCompletionRequestAssistantMessageContent, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestToolMessage, ChatCompletionRequestToolMessageContent, ChatCompletionTool, ChatCompletionTools, CreateChatCompletionRequestArgs, FinishReason, FunctionObjectArgs,
};
use tokio::{process::Command, time::timeout};

use crate::structure::loop_state::LoopState;

fn get_model() -> anyhow::Result<String> {
    dotenvy::dotenv()?;
    std::env::var("OPENAI_MODEL").context("OPENAI_MODEL is not set")
}

const SYSTEM: &str = r#"You are a coding agent.
Use bash to inspect and change the workspace. Act first, then report clearly.
"#;

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

pub async fn run_bash(command: &str) -> anyhow::Result<String> {
    // 1. command black lists for dangerous.
    let dangerous = ["rm -rf /", "sudo", "shutdown", "reboot", "> /dev/"];
    if dangerous.iter().any(|c| command.contains(c)) {
        return Err(anyhow::anyhow!("Error: Dangerous command blocked".to_string()));
    }
    // 2. struct async command.
    let child = match Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true) // 当 Child 被丢弃时自动杀死进程
        .spawn()
    {
        Result::Ok(c) => c,
        Err(e) => return Err(anyhow::anyhow!("Error: {}", e)),
    };
    // 3. 等待输出，带 120 秒超时
    let output_future = child.wait_with_output();
    match timeout(Duration::from_secs(120), output_future).await {
        Result::Ok(Result::Ok(output)) => {
            // 正常完成，合并 stdout 和 stderr
            let combined = [output.stdout, output.stderr].concat();
            let out_str = String::from_utf8_lossy(&combined);
            let trimmed = out_str.trim();

            if trimmed.is_empty() {
                Ok("(no output)".to_string())
            } else {
                // 截取前 50000 个字符（安全处理 UTF-8 边界）
                Ok(trimmed.chars().take(50000).collect())
            }
        }
        Result::Ok(Err(e)) => {
            // 执行错误（例如命令不存在）
            Err(anyhow::anyhow!("Error: {}", e))
        }
        Err(_) => {
            // 超时发生：由于设置了 kill_on_drop(true)，此时 child 会被自动杀死
            Err(anyhow::anyhow!("Error: Timeout (120s)".to_string()))
        }
    }
}

/// Extract text content from the assistant response message.
#[allow(dead_code)]
fn extract_text(content: &Option<String>) -> String {
    content.as_deref().unwrap_or("").to_string()
}

/// Execute tool calls from the assistant response, returning tool result messages.
async fn execute_tool_calls(
    tool_calls: &[ChatCompletionMessageToolCalls],
) -> Vec<ChatCompletionRequestMessage> {
    let mut results = Vec::new();
    for tc in tool_calls {
        let ChatCompletionMessageToolCalls::Function(func_call) = tc else {
            continue;
        };
        if func_call.function.name != "bash" {
            results.push(ChatCompletionRequestMessage::Tool(
                ChatCompletionRequestToolMessage {
                    content: ChatCompletionRequestToolMessageContent::Text(
                        format!("Unknown tool: {}", func_call.function.name),
                    ),
                    tool_call_id: func_call.id.clone(),
                },
            ));
            continue;
        }

        let command: serde_json::Value =
            serde_json::from_str(&func_call.function.arguments).unwrap_or_default();
        let cmd_str = command
            .get("command")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let output = run_bash(cmd_str).await;
        let output_str = match output {
            Ok(s) => s,
            Err(e) => format!("Error: {}", e),
        };

        println!("Command '{}' output: {}", cmd_str, output_str);

        results.push(ChatCompletionRequestMessage::Tool(
            ChatCompletionRequestToolMessage {
                content: ChatCompletionRequestToolMessageContent::Text(output_str),
                tool_call_id: func_call.id.clone(),
            },
        ));
    }
    results
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
