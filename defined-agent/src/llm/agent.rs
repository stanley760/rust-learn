
use async_openai::types::chat::{
    ChatCompletionMessageToolCall, ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessageArgs, ChatCompletionTools, CreateChatCompletionRequestArgs, FunctionCall,
};
use crate::{
    llm::{ContentItem, Event, ExecutionContext, content::ToolResultStatus}, tools::ToolBox,
};

#[derive(Debug)]
pub struct AgentResult {
    pub output: String,
    pub context: ExecutionContext,
}

/// Orchestrator: create an ExecutorContext, and drive a loop of think-act,
/// Util getting the final answer, or over the max step to focus stop the program.
pub struct Agent<'a> {
    model: &'a str,
    instructions: Option<&'a str>,
    toolbox: &'a ToolBox,
    max_steps: u32,
}

impl<'a> Agent<'a> {
    pub fn new(model: &'a str, instructions: Option<&'a str>, toolbox: &'a ToolBox) -> Self {
        Self {
            model,
            instructions,
            toolbox,
            max_steps: 10,
        }
    }

    /// override the max default steps, avoid a lang loop of agent.
    pub fn with_max_steps(mut self, max_steps: u32) -> Self {
        self.max_steps = max_steps;
        self
    }

    pub async fn run(&self, user_prompt: &str) -> anyhow::Result<AgentResult> {
        let mut context = ExecutionContext::new();

        context.add_event(Event::new(
            context.execution_id.clone(),
            "user",
            vec![ContentItem::Message {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            }],
        ));
        let client = async_openai::Client::new();
        // get all tool definitions.
        let tool_definitions: Vec<ChatCompletionTools> = self
            .toolbox
            .values()
            .filter_map(|t| match t.definition() {
                Ok(def) => Some(def),
                Err(e) => {
                    tracing::warn!("Skip tool {},  failed to get its definition: {e}", t.name());
                    None
                }
            })
            .collect();

        loop {
            if context.current_step >= self.max_steps {
                anyhow::bail!(
                    "Agent exceeded the maximum of {} steps,and not a final answer",
                    self.max_steps
                );
            }
            // get some prompt from the type of system, user, assiant
            let messages = self.build_messages(&context)?;

            let request = CreateChatCompletionRequestArgs::default()
                .model(self.model)
                .messages(messages)
                .tools(tool_definitions.clone())
                .max_tokens(2048u32)
                .build()?;

            let response = client.chat().create(request).await?;
            
            let message = response
                .choices
                .into_iter()
                .next()
                .ok_or_else(|| anyhow::anyhow!("No choices in response"))?
                .message;

            if let Some(tool_calls) = message.tool_calls {
                self.record_tool_calls(&mut context, &tool_calls);
                self.execute_tool_calls(&mut context, &tool_calls).await;
            } else {
                let content = message
                    .content
                    .ok_or_else(|| anyhow::anyhow!("no content in response."))?;
                context.add_event(Event::new(
                    context.execution_id.clone(), 
                    "agent", 
                    vec![ContentItem::Message {
                         role: "assiant".to_string(), 
                         content: content.clone(),
                         }]
                ));
            }
            context.increment_step();
        }
        
    }

    fn build_messages(
        &self,
        context: &ExecutionContext,
    ) -> anyhow::Result<Vec<ChatCompletionRequestMessage>> {
        let mut messages: Vec<ChatCompletionRequestMessage> = Vec::new();

        if let Some(system) = self.instructions {
            messages.push(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system)
                    .build()?
                    .into(),
            )
        }
        // the iterator of content which match the type of that is the enum object named ContentItem
        // And assemble some message into the collection according the different type.
        for event in &context.events {
            for item in &event.content {
                match item {
                    ContentItem::Message { role, content } => {
                        let message: ChatCompletionRequestMessage = if role == "user" {
                            ChatCompletionRequestUserMessageArgs::default()
                                .content(content.clone())
                                .build()?
                                .into()
                        } else {
                            ChatCompletionRequestAssistantMessageArgs::default()
                                .content(content.clone())
                                .build()?
                                .into()
                        };
                        messages.push(message);
                    }
                    ContentItem::ToolCall {
                        tool_call_id,
                        name,
                        arguments,
                    } => {
                        let tool_call = ChatCompletionMessageToolCalls::Function(
                            ChatCompletionMessageToolCall {
                                id: tool_call_id.clone(),
                                function: FunctionCall {
                                    name: name.clone(),
                                    arguments: arguments.to_string(),
                                },
                            },
                        );

                        // the model maybe request many time to invoke the toolCall at the same time
                        // so this toolCall will be merged into the assiant messsage.
                        if let Some(ChatCompletionRequestMessage::Assistant(last)) = messages.last_mut() {
                            last.tool_calls.get_or_insert_with(Vec::new).push(tool_call);
                        } else {
                            messages.push(
                                ChatCompletionRequestAssistantMessageArgs::default()
                                    .tool_calls(vec![tool_call])
                                    .build()?
                                    .into(),
                            );
                        }
                    }
                    ContentItem::ToolResult {
                        tool_call_id,
                        name: _,
                        status: _,
                        content,
                    } => {
                        messages.push(
                            ChatCompletionRequestToolMessageArgs::default()
                                .tool_call_id(tool_call_id)
                                .content(content.clone())
                                .build()?
                                .into(),
                        )
                    }
                }
            }
        }

        Ok(messages)
    }
    
    /// the process of invoking the toolCall, 
    /// it's be seem with a event of agent's behavior.
    fn record_tool_calls(
        &self,
        context: &mut ExecutionContext,
        tool_calls: &[ChatCompletionMessageToolCalls]
    ) {
        let mut call_items: Vec<ContentItem> = Vec::new();
        for tool_call in tool_calls {
            if let ChatCompletionMessageToolCalls::Function(function) = tool_call {
                let arguments: serde_json::Value = serde_json::from_str(&function
                        .function.arguments)
                        .unwrap_or(serde_json::Value::Null);
                call_items.push(
                    ContentItem::ToolCall { 
                        tool_call_id: function.id.clone(), 
                        name: function.function.name.clone(), 
                        arguments: arguments, 
                    }
                );   
            }
        }
        context.add_event(Event::new(context.execution_id.clone(), "agent", call_items));
    }

    async fn execute_tool_calls(
        &self,
        context: &mut ExecutionContext,
        tool_calls: &[ChatCompletionMessageToolCalls],
    ) {
        let mut result_item : Vec<ContentItem> = Vec::new();
        for tool_call in tool_calls {
            let ChatCompletionMessageToolCalls::Function(function_call) = tool_call else {
                continue;
            };
            let function_name = &function_call.function.name;
            let arguments = &function_call.function.arguments;

            tracing::info!("Tool call which {function_name} invoke with arguments {arguments}");
            let (status, content) = match self.toolbox.get(function_name) {
                Some(tool) => match tool.execute(arguments, context).await {
                    Ok(result) => {
                        tracing::info!("Tool result: {result}");
                        (ToolResultStatus::Success, result)
                    }
                    Err(err) => {
                        let format = format!("Tool execution error: unknown tool {function_name}");
                        tracing::error!("Tool call invoke err: {:#?}",err);
                        (ToolResultStatus::Error, format)
                    }
                }, 
                None => {
                    let msg = format!("Tool execution error: unknown tool {function_name}");
                    tracing::error!("{msg}");
                    (ToolResultStatus::Error, msg)
                }
            };
            result_item.push(ContentItem::ToolResult {
                 tool_call_id: function_call.id.clone(), 
                 name: function_name.clone(), 
                 status, 
                 content }

            )
        }
        context.add_event(Event::new(
            context.execution_id.clone(),
            "tool", 
        result_item));
    }
}
