use std::time::Duration;

use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, ChatCompletionRequestMessage, ChatCompletionRequestToolMessage,
    ChatCompletionRequestToolMessageContent,
};
use futures::future::join_all;
use tokio::{process::Command, time::timeout};

pub async fn run_bash(command: &str) -> anyhow::Result<String> {
    // 1. command black lists for dangerous.
    let dangerous = [
        "rm -rf /",
        "sudo",
        "shutdown",
        "reboot",
        "> /dev/",
        // Windows-specific
        "format ",
        "del /s /q C:",
        "rd /s /q C:",
        "Remove-Item -Recurse -Force C:\\",
    ];
    if dangerous.iter().any(|c| command.contains(c)) {
        return Err(anyhow::anyhow!(
            "Error: Dangerous command blocked".to_string()
        ));
    }

    // 2. struct async command — pick shell based on OS.
    let (shell, shell_arg) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    // On Windows, prefix with `chcp 65001` to switch code page to UTF-8
    // so that output (especially CJK error messages) decodes correctly.
    let full_command = if cfg!(target_os = "windows") {
        format!("chcp 65001 >nul && {}", command)
    } else {
        command.to_string()
    };

    let child = match Command::new(shell)
        .arg(shell_arg)
        .arg(&full_command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()
    {
        Result::Ok(c) => c,
        Err(e) => return Err(anyhow::anyhow!("Error: {}", e)),
    };
    // 3. 等待输出，带 120 秒超时
    timeout(Duration::from_secs(120), child.wait_with_output())
        .await
        // 超时发生：由于设置了 kill_on_drop(true)，此时 child 会被自动杀死
        .map_err(|_| anyhow::anyhow!("Timeout (120s)"))?
        // 执行错误（例如命令不存在）
        .map_err(|e| anyhow::anyhow!("Error: {}", e))
        .map(|output| {
            // 正常完成，合并 stdout 和 stderr
            let binding = [output.stdout, output.stderr].concat();
            let s = String::from_utf8_lossy(&binding);
            let t = s.trim();
            if t.is_empty() {
                "(no output)".to_string()
            } else {
                // 截取前 50000 个字符（安全处理 UTF-8 边界）
                t.chars().take(50000).collect()
            }
        })
}

/// Extract text content from the assistant response message.
#[allow(dead_code)]
fn extract_text(content: &Option<String>) -> String {
    content.as_deref().unwrap_or("").to_string()
}

/// Execute tool calls from the assistant response, returning tool result messages.
pub async fn execute_tool_calls(
    tool_calls: &[ChatCompletionMessageToolCalls],
) -> Vec<ChatCompletionRequestMessage> {
    join_all(tool_calls.iter().filter_map(|tc| {
        let ChatCompletionMessageToolCalls::Function(f) = tc else {
            return None;
        };

        let id = f.id.clone();
        let name = f.function.name.clone();
        let cmd = serde_json::from_str::<serde_json::Value>(&f.function.arguments)
            .unwrap_or_default()
            .get("command")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        Some(async move {
            if name != "bash" {
                return ChatCompletionRequestMessage::Tool(ChatCompletionRequestToolMessage {
                    content: ChatCompletionRequestToolMessageContent::Text(format!(
                        "Unknown tool: {}",
                        name
                    )),
                    tool_call_id: id,
                });
            }

// [CLIPPY-WARNING] unnecessary_result_map_or_else (line 112)
            let output_str = run_bash(&cmd)
                .await
                .map_or_else(|e| format!("Error: {}", e), |s| s);

            println!("Command '{}' output: {}", cmd, output_str);

            ChatCompletionRequestMessage::Tool(ChatCompletionRequestToolMessage {
                content: ChatCompletionRequestToolMessageContent::Text(output_str),
                tool_call_id: id,
            })
        })
    }))
    .await
}
