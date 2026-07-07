use std::borrow::Cow;

use anyhow::Context;
use async_trait::async_trait;
use serde_json::Value;
use tokio::fs;

use crate::tools::{Tool, ToolSpec, safe_path};

pub struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    async fn invoke(&self, input: &Value) -> anyhow::Result<String> {
        let path = input.get("path")
            .and_then(|v| v.as_str())
            .context("Invalid path")?;
        let path = safe_path(path, true)?;

        let limit = input.get("limit").and_then(|v| v.as_u64());

        let content = fs::read_to_string(path)
            .await
            .map_err(|e| anyhow::anyhow!("Error: {}", e))?;

        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        if let Some(limit) = limit
            && (limit as usize) < lines.len()
        {
            let remaining = lines.len() - limit as usize;
            lines.truncate(limit as usize);
            lines.push(format!("... ({} more lines)", remaining));
        }

        let result = lines.join("\n");

        Ok(result.chars().take(50000).collect())
    }

    fn name(&self) -> Cow<'_, str> {
        "read_file".into()
    }

    fn tool_spec(&self) -> ToolSpec {
        ToolSpec {
            name: "read_file".to_string(),
            description: Some("Read file.".to_string()),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string"},
                    "limit": {"type": "integer"}
                },
                "required": ["path"]
            }),
        }
    }
}
