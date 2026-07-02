use anyhow::Ok;
use async_openai::types::chat::{ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema};

use crate::structure::act_plan::ActionPlan;



pub async fn chat_complete_structed(model: &str, system:Option<&str>, prompt: &str) -> anyhow::Result<ActionPlan> {
    let client = async_openai::Client::new();
    let mut messages = vec![];

    if let Some(system) = system {
        messages.push(ChatCompletionRequestSystemMessageArgs::default()
            .content(system)
            .build()?
            .into()
        );
    }
    messages.push(ChatCompletionRequestUserMessageArgs::default()
        .content(prompt)
        .build()?
        .into()
    );

    let schema = schemars::schema_for!(ActionPlan);
    let schema_json = schema.as_value().clone();
    let format_str = ResponseFormat::JsonSchema { 
        json_schema: ResponseFormatJsonSchema {
            description: Some("A step-by-step agent action plan with diffifulty and
time estimate".into()),
            name: "action plan".into(),
            schema: schema_json,
            strict: Some(true),
        }
    };

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .response_format(format_str)
        .max_tokens(20480u32)
        .build()?;
    let response = client.chat().create(request).await?;

    let plan: ActionPlan = response.choices.into_iter().next().and_then(|f| f.message.content)
        .ok_or_else(|| anyhow::anyhow!("no content")).and_then(|s|serde_json::from_str(&s).map_err(Into::into))?;
    Ok(plan)
}