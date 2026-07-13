use anyhow::Context;
use async_openai::{
    types::chat::{
        ChatCompletionRequestAssistantMessageContent, 
        ChatCompletionRequestMessage, 
        ChatCompletionRequestUserMessageArgs
    }
};
use defined_agent::{
    structure::get_llm_client, 
    structure::LoopState, 
    tools::agent_tools
};
use dialoguer::Input;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;


const SYSTEM: &str = if cfg!(target_os = "windows") {
    r#"You are a coding agent.
Use cmd.exe to inspect and change the workspace. Act first, then report clearly.
IMPORTANT: You are on Windows. Use Windows commands only:
- Use `dir` instead of `ls`, `type` instead of `cat`, `findstr` instead of `grep`.
- Use backslashes in paths or forward slashes (both work).
- Do NOT use Unix-only commands like ls, cat, grep, rm, chmod, etc.

You have a `task` tool that spawns a sub-agent with fresh context. Use it when:
- The user asks you to investigate, explore, or analyze something independently.
- A task is complex enough to benefit from isolated execution.
- You want to delegate a self-contained sub-task (e.g., "find all TODOs", "summarize a module").
Call the task tool with a clear prompt describing what the sub-agent should do.
"#
} else {
    r#"You are a coding agent.
Use bash to inspect and change the workspace. Act first, then report clearly.

You have a `task` tool that spawns a sub-agent with fresh context. Use it when:
- The user asks you to investigate, explore, or analyze something independently.
- A task is complex enough to benefit from isolated execution.
- You want to delegate a self-contained sub-task (e.g., "find all TODOs", "summarize a module").
Call the task tool with a clear prompt describing what the sub-agent should do.
"#
};


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // 初始化日志
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // get tools from the tool registry
    let tools = agent_tools();
    // 创建 OpenAI client
    let client = get_llm_client()?;
    let mut state = LoopState::new(client, tools, SYSTEM, 30);

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
