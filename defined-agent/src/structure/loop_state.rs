use anyhow::Context;
use async_openai::{Client, config::OpenAIConfig};

use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, 
    ChatCompletionRequestAssistantMessage, 
    ChatCompletionRequestAssistantMessageContent, 
    ChatCompletionRequestMessage, 
    ChatCompletionRequestSystemMessageArgs, 
    ChatCompletionRequestToolMessage, 
    ChatCompletionRequestToolMessageContent, 
    ChatCompletionTools, 
    CreateChatCompletionRequestArgs, 
    FinishReason,
};
use std::collections::HashMap;
use crate::tools::Tool;

fn get_model() -> anyhow::Result<String> {
    dotenvy::dotenv()?;
    std::env::var("OPENAI_MODEL").context("OPENAI_MODEL is not set")
}

pub fn get_llm_client() -> anyhow::Result<Client<OpenAIConfig>> {
    dotenvy::dotenv()?;
    let client: Client<OpenAIConfig> = Client::with_config(OpenAIConfig::default());
    Ok(client)
}


pub struct LoopState {
    client: Client<OpenAIConfig>,
    pub context: Vec<ChatCompletionRequestMessage>,
    tools: HashMap<String, Box<dyn Tool>>,
    pub system_prompt: String,
    pub max_round: usize,
}

impl LoopState {
    pub fn new(client: Client<OpenAIConfig>,
        tools: HashMap<String, Box<dyn Tool>>,
        system_prompt: impl Into<String>,
        max_round: usize,) -> Self {
        Self {
            client,
            tools,
            context: Vec::new(),
            system_prompt: system_prompt.into(),
            max_round
        }
    }

    async fn execute(&mut self, name: &str, input: &serde_json::Value) -> anyhow::Result<String> {
        let Some(tool) = self.tools.get_mut(name) else {
            return Err(anyhow::anyhow!("Unknown tool: {name}"));
        };

        match tool.invoke(input).await {
            Ok(output) => {
                println!("Command:{}\n arg:{}\n output:\n{}\n", name, input, output);
                Ok(output)
            }
            Err(e) => {
                println!("Error invoking tool {}: {}", name, e);
                Err(anyhow::anyhow!("Error invoking tool {}: {}", name, e))
            }
        }
    }
    pub async fn agent_loop(&mut self) -> anyhow::Result<()> {
        self.context.push(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(self.system_prompt.clone())
                .build()?
                .into(),
        );
        for _ in 0..self.max_round {
            let tool_vec :Vec<ChatCompletionTools> = self.tools.values().map(|t| t.tool_spec().into_openai_tool()).collect();
            let request = CreateChatCompletionRequestArgs::default()
                .model(get_model()?)
                .messages(self.context.clone())
                .max_completion_tokens(8000u32)
                .tools(tool_vec)
                .build()?;

            let response = self.client.chat().create(request).await?;

            let choice = response
                .choices
                .into_iter()
                .next()
                .ok_or_else(|| anyhow::anyhow!("No response choice"))?;

            let msg = choice.message;

            self.context.push(ChatCompletionRequestMessage::Assistant(
                ChatCompletionRequestAssistantMessage {
                    content: msg
                        .content
                        .map(ChatCompletionRequestAssistantMessageContent::Text),
                    refusal: msg.refusal,
                    name: None,
                    audio: None,
                    tool_calls: msg.tool_calls.clone(),
                    #[allow(deprecated)]
                    function_call: None,
                },
            ));

            match choice.finish_reason {
                Some(FinishReason::ToolCalls) => {}
                _ => return Ok(()),
            }

            let Some(tool_calls) = msg.tool_calls else {
                return Ok(());
            };

            if tool_calls.is_empty() {
                return Ok(());
            }

            let tool_results = self.execute_tool_calls(&tool_calls).await?;
            self.context.extend(tool_results);
        }

        Ok(())
    }


    async fn execute_tool_calls(&mut self, tool_calls: &[ChatCompletionMessageToolCalls]) -> anyhow::Result<Vec<ChatCompletionRequestMessage>> {
        let mut results = Vec::new();
        for tc in tool_calls.iter() {
            let ChatCompletionMessageToolCalls::Function(f) = tc else {
                continue;
            };

            let id = f.id.clone();
            let name = f.function.name.clone();
            let cmd = serde_json::from_str::<serde_json::Value>(&f.function.arguments)
                .unwrap_or_default();

            let output_str = self.execute(&name, &cmd)
                .await
                .map_or_else(|e| format!("Error: {}", e), |s| s);

            println!("Command '{}' output: {}", cmd, output_str);

            results.push(ChatCompletionRequestMessage::Tool(ChatCompletionRequestToolMessage {
                content: ChatCompletionRequestToolMessageContent::Text(output_str),
                tool_call_id: id,
            }));
            
        }
        Ok(results)
    }
}



