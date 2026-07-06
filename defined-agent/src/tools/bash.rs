use std::{borrow::Cow, time::Duration};

use async_trait::async_trait;
use serde_json::Value;
use tokio::{process::Command, time::timeout};

use super::{Tool, ToolSpec};

pub struct BashTool;

#[async_trait]
impl Tool for BashTool {
    async fn invoke(&self, input: &Value) -> anyhow::Result<String> {
        let command = input["command"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing 'command' field"))?;
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

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("bash")
    }

    fn tool_spec(&self) -> ToolSpec {
        let description = if cfg!(target_os = "windows") {
            "Run a cmd.exe command on Windows. Use dir/type/findstr instead of ls/cat/grep."
        } else {
            "Run a shell command in the current workspace (sh on Unix)."
        };
        ToolSpec {
            name: "bash".into(),
            description: Some(description.into()),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The shell command to execute"
                    }
                },
                "required": ["command"]
            }),
        }
    }
}
