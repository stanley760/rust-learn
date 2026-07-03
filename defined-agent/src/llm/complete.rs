
use std::time::Instant;

use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs
};
use async_stream::stream;
use backon::{ExponentialBuilder, Retryable};
use futures::{Stream, StreamExt};

pub fn chat_complete_structed(
    model: &str,
    system: Option<&str>,
    prompt: &str,
) -> impl Stream<Item = anyhow::Result<String>> {
    stream! {
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

        // let schema = schemars::schema_for!(ActionPlan);
        // let schema_json = schema.as_value().clone();
        // let format_str = ResponseFormat::JsonSchema {
        //     json_schema: ResponseFormatJsonSchema {
        //         description: Some("A step-by-step agent action plan with diffifulty and
        // time estimate".into()),
        //         name: "action plan".into(),
        //         schema: schema_json,
        //         strict: Some(true),
        //     }
        // };

        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(messages)
            .max_tokens(20480u32)
            .build()?;
        // .response_format(format_str)
        // let response = client.chat().create(request).await?;

        // let plan: ActionPlan = response.choices.into_iter().next().and_then(|f| f.message.content)
        //     .ok_or_else(|| anyhow::anyhow!("no content")).and_then(|s|serde_json::from_str(&s).map_err(Into::into))?;
        // Ok(plan)
        let start = Instant::now();
        let mut first_token = true;
        let mut stream_response = client.chat().create_stream(request).await?;

        while let Some(res) = stream_response.next().await {
            match res {
                Ok(response) => {
                    if let Some(choice) = response.choices.first()
                        && let Some(content) = &choice.delta.content {
                            if first_token {
                                let ttft = start.elapsed();
                                println!("首字延迟 (TTFT): {:.3}ms", ttft.as_secs_f64() * 1000.0);
                                first_token = false;
                            }
                            yield Ok(content.clone())
                    }
                },
                Err(e) => yield Err(anyhow::anyhow!("Stream error: {}", e))
            }
        }
    }
}


pub async fn chat_stream_with_retry(model: &str,
    system: Option<&str>,
    prompt: &str,
) -> anyhow::Result<String> {
    let op = || async {
        let result = chat_complete_structed(
            model,
            system,
            prompt
        );

        futures::pin_mut!(result);
        let mut output = String::new();
        while let Some(item) = result.next().await {
            match item {
                Result::Ok(plan) => {
                    output.push_str(&plan);
                    print!("{}", plan);
                }
                Err(e) => {
                    eprintln!("Error receiving streaming: {:?}", e);
                    return Err(e);
                }
            }
        }
        Ok(output)
    };
    op.retry(ExponentialBuilder::default().with_max_times(3)).await
}