use std::{borrow::Cow, sync::Arc};

use anyhow::{Context, Ok};
use async_trait::async_trait;
use serde_json::{Value, json};

use crate::{skills::SkillRegistry, tools::{Tool, ToolSpec}};



pub struct SkillLoadTool {
    registry: Arc<SkillRegistry>,
}

pub fn load_skills_tool (registry: Arc<SkillRegistry>) -> Box<dyn Tool> {
    Box::new(SkillLoadTool {registry}) as Box<dyn Tool>
}

#[async_trait]
impl Tool for SkillLoadTool {

    async fn invoke(&mut self, input: &Value) -> anyhow::Result<String> {
        let name = input.get("name")
            .and_then(|v| v.as_str())
            .context("invalid name")?;

        Ok(self.registry.load_full_text(name))
    }

    fn name(&self) -> Cow<'_, str> {
        "load_skill".into()
    }

    fn tool_spec(&self) -> ToolSpec {
        ToolSpec {
            name: "load_skill".to_string(),
            description: Some("Load the full body of a named skill into the current context.".to_string()),
            input_schema: json!({
                "type": "object",
                "property": {"name": {"type": "string"}},
                "required": ["name"],
            })
        }
    }
}