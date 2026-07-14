use std::borrow::Cow;

use anyhow::Context;

use async_trait::async_trait;
use serde_json::{Value, json};

use crate::{
    tools::{Tool, ToolSpec}
};

pub struct SubAgentTool;

pub fn sub_agent_tool() -> Box<dyn Tool> {
    Box::new(SubAgentTool {}) as Box<dyn Tool>
}

async fn sub_agent_loop(prompt: &str, description: Option<&str>) -> anyhow::Result<String> {
    println!("> task ----- ({}): {}", description.unwrap_or_default(), prompt);
    // let client = get_llm_client()?;
    // let tools = subagent_tools();
    // let system_prompt = format!(
    //     "You are a coding subagent at {}. Complete the given task, then summarize your findings.",
    //     std::env::current_dir()?.display()
    // );
    // let mut state = LoopState::new(client, tools, system_prompt, 30);
    // state.context.push(
    //     ChatCompletionRequestUserMessageArgs::default()
    //         .content(prompt)
    //         .build()?
    //         .into(),
    // );

    // state.agent_loop().await?;

    // let summary = state
    //     .context
    //     .iter()
    //     .rev()
    //     .find(|message| matches!(message, ChatCompletionRequestMessage::Assistant(_)))
    //     .and_then(|message| if let ChatCompletionRequestMessage::Assistant(m) = message { m.content.as_ref() } else { None })
    //     .map(extract_text)
    //     .filter(|text| !text.is_empty())
    //     .unwrap_or_else(|| "(no summary)".to_string());

    // Ok(summary)
    todo!("LoopState new() parameters changes.")
}

#[async_trait]
impl Tool for SubAgentTool {
    async fn invoke(&mut self, input: &Value) -> anyhow::Result<String> {
        let prompt = input
            .get("prompt")
            .and_then(|v| v.as_str())
            .context("Invalid prompt")?;

        let description = input.get("description").and_then(|v| v.as_str());

        sub_agent_loop(prompt, description).await
    }

    fn name(&self) -> Cow<'_, str> {
        "task".into()
    }

    fn tool_spec(&self) -> ToolSpec {
        ToolSpec { 
            name: "task".to_string(), 
            description: Some("Spawn a sub-agent with fresh context to handle a self-contained task. Use this when: (1) the task involves multiple steps that should be isolated from the main conversation, (2) you need to explore or analyze code independently, (3) you want to delegate a sub-task like 'find all TODO comments' or 'summarize a directory'. The sub-agent shares the filesystem but NOT conversation history. Provide a clear, specific prompt describing what the sub-agent should accomplish.".to_string()), 
            input_schema: json!({
                "type": "object", 
                "properties": {
                    "prompt": {"type": "string"},
                    "description": {"type": "string", "description": "Short description of the task"}
                },
                "required": ["prompt"]
            }) 
        }
    }
}
