use std::sync::Arc;

use anyhow::Context;
use async_openai::{
    types::chat::{
        ChatCompletionRequestAssistantMessageContent, 
        ChatCompletionRequestMessage, 
        ChatCompletionRequestUserMessageArgs
    }
};
use defined_agent::{
    skills::get_skill_registry, structure::{LoopState, get_llm_client}, tools::toolset
};
use dialoguer::Input;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;


const SKILLS_DIR: &str = "skills";
#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // 初始化日志
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // get tools from the tool registry
    // let tools = agent_tools();
    // skill path : /User/xxxx/rust-learn/defined-agent/skills
    let skills_dir = std::env::current_dir()?.join(SKILLS_DIR);
    let registry = Arc::new(get_skill_registry(skills_dir)?);

    let tools = toolset(registry.clone());
    // 创建 OpenAI client
    let client = get_llm_client()?;
    let mut state = LoopState::new(client, tools, registry);

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
        state.agent_loop().await?;

        // 提取并打印最终回复（从最后一条 Assistant 消息提取文本）
        let text = state.context.last().and_then(|msg| match msg {
            ChatCompletionRequestMessage::Assistant(m) => m.content.as_ref().and_then(|c| match c {
                ChatCompletionRequestAssistantMessageContent::Text(t) => Some(t.clone()),
                _ => None,
            }),
            _ => None,
        });
        if let Some(text) = text {
            println!("--- Final response:\n{}", text);
        }
    }

    Ok(())
}