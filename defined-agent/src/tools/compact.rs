use std::borrow::Cow;

use anyhow::Ok;
use async_trait::async_trait;
use serde_json::{Value, json};

use crate::tools::{Tool, ToolSpec};

pub struct ContextCompactTool;

pub fn compact_tool() -> Box<dyn Tool> {
    Box::new(ContextCompactTool {}) as Box<dyn Tool>
}

#[async_trait]
impl Tool for ContextCompactTool {
    async fn invoke(&mut self, _input: &Value) -> anyhow::Result<String> {
        Ok("Compacting conversation...".into())
    }

    fn name(&self) -> Cow<'_, str> {
        "compact".into()
    }

    fn tool_spec(&self) -> ToolSpec {
        ToolSpec {
            name: "compact".to_string(),
            description: Some("".to_string()),
            input_schema: json!({
                "type": "object", 
                "property": {
                    "focus": {"type": "string"},
                }
            })
        }
    }
}
