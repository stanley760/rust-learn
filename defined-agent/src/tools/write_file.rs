use std::borrow::Cow;

use anyhow::Context;
use async_trait::async_trait;
use serde_json::Value;
use tokio::fs;

use crate::tools::{Tool, ToolSpec, safe_path};

pub struct WriteFileTool;

pub fn write_file_tool() -> Box<dyn Tool> {
    Box::new(WriteFileTool {}) as Box<dyn Tool>
}
#[async_trait]
impl Tool for WriteFileTool {
    async fn invoke(&mut self, input: &Value) -> anyhow::Result<String> {
        let path = input
            .get("path")
            .and_then(|v| v.as_str())
            .context("Invalid path")?;
        let path = safe_path(path, false)?;

        let content = input
            .get("content")
            .and_then(|v| v.as_str())
            .context("Invalid content")?;

        if let Some(parent_path) = path.parent() {
            fs::create_dir_all(parent_path).await.ok();
        }

        let _ = fs::write(&path, content)
            .await
            .map_err(|e| anyhow::anyhow!("Error:write content error {}", e));

        Ok(format!(
            "wrote {} bytes to {}",
            content.len(),
            path.display()
        ))
    }

    fn name(&self) -> Cow<'_, str> {
        "write_file".into()
    }

    fn tool_spec(&self) -> ToolSpec {
        ToolSpec {
            name: "write_file".to_string(),
            description: Some("Write content to file".to_string()),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "content": { "type": "string" },
                },
                "required": ["path", "content"]
            }),
        }
    }
}
