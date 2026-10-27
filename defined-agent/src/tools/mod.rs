use crate::tools::calculator::CalculatorTool;
use async_openai::types::chat::{ChatCompletionTool, ChatCompletionTools, FunctionObjectArgs};
use serde_json::Value;
use std::collections::HashMap;
pub mod calculator;

#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;

    fn description(&self) -> &str;

    fn parameters(&self) -> Value;

    async fn execute(&self, arg_json: &str) -> anyhow::Result<String>;

    fn definition(&self) -> anyhow::Result<ChatCompletionTools> {
        let function = FunctionObjectArgs::default()
            .name(self.name())
            .description(self.description())
            .parameters(self.parameters())
            .build()
            .map_err(|e| {
                anyhow::anyhow!("Failed to build tool definition for {}: {e}", self.name())
            })?;

        Ok(ChatCompletionTools::Function(ChatCompletionTool {
            function,
        }))
    }
}

pub type ToolBox = HashMap<String, Box<dyn Tool>>;

pub fn build_toolbox() -> ToolBox {
    let tools: Vec<Box<dyn Tool>> = vec![Box::new(CalculatorTool)];
    tools
        .into_iter()
        .map(|tool| (tool.name().to_string(), tool))
        .collect()
}
