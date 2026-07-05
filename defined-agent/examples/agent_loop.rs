use anyhow::Context;
use async_openai::types::chat::{
    ChatCompletionRequestAssistantMessageContent,
    ChatCompletionRequestAssistantMessageContentPart,
    ChatCompletionRequestMessage, ChatCompletionRequestToolMessageContent,
    ChatCompletionRequestToolMessageContentPart,
    ChatCompletionRequestUserMessageArgs,
};
use async_openai::{Client, config::OpenAIConfig};
use defined_agent::structure::{LoopState, agent_loop};
use dialoguer::Input;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// Extract text from the last message in the context.
fn extract_last_text(context: &[ChatCompletionRequestMessage]) -> String {
    context.last().map_or(String::new(), |msg| match msg {
        ChatCompletionRequestMessage::Assistant(a) => match &a.content {
            Some(ChatCompletionRequestAssistantMessageContent::Text(t)) => t.clone(),
            Some(ChatCompletionRequestAssistantMessageContent::Array(parts)) => parts
                .iter()
                .filter_map(|p| {
                    if let ChatCompletionRequestAssistantMessageContentPart::Text(t) = p {
                        Some(t.text.as_str())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("\n"),
            None => String::new(),
        },
        ChatCompletionRequestMessage::Tool(t) => match &t.content {
            ChatCompletionRequestToolMessageContent::Text(s) => s.clone(),
            ChatCompletionRequestToolMessageContent::Array(parts) => parts
                .iter()
                .filter_map(|p| match p {
                    ChatCompletionRequestToolMessageContentPart::Text(t) => {
                        Some(t.text.as_str())
                    }
                })
                .collect::<Vec<_>>()
                .join("\n"),
        },
        _ => String::new(),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    // 初始化日志
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // 创建 OpenAI client
    let client = Client::with_config(OpenAIConfig::default());
    let mut state = LoopState::new(client);

    loop {
        let query: String = Input::new()
            .with_prompt("--- How can I help you?")
            .interact_text()
            .context("An error happened or user cancelled the input.")?;

        // 输入 exit() 退出循环
        if query.trim() == "exit()" {
            break;
        }

        // 将用户输入作为 User message 推入上下文
        state.context.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(query)
                .build()?
                .into(),
        );

        // 运行 agent loop
        agent_loop(&mut state).await?;

        // 提取并打印最终回复
        let final_text = extract_last_text(&state.context);
        if !final_text.is_empty() {
            println!("--- Final response:\n{}", final_text);
        }
    }

    Ok(())
}
