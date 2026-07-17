use anyhow::{Context, Ok};
use async_openai::types::chat::ChatCompletionRequestUserMessageArgs;
use defined_agent::{
    hook::HookControl, invoke_hooks, permission::{PermissionManager, PermissionMode}, structure::{LoopState, extract_text, get_llm_client}, tools::toolset_compact,
};
use inquire::{Select, Text};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

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
    // let skills_dir = std::env::current_dir()?.join(SKILLS_DIR);
    // let registry = Arc::new(get_skill_registry(skills_dir)?);

    let tools = toolset_compact();

    let mode = Select::new(
        "Permission mode:",
        vec![
            PermissionMode::Default,
            PermissionMode::Plan,
            PermissionMode::Auto,
        ],
    )
    .prompt()
    .context("An error happened or user cancelled the input.")?;

    let manager = PermissionManager::try_new(mode)?;
    info!("[Permission mode: {}]", manager.mode());
    
    // 创建 OpenAI client
    let client = get_llm_client()?;
    let mut state = LoopState::new(client, tools, manager);


    state.session_start(|_| {
        Box::pin(async {
            info!("--- Initializing hooks...");  
            Ok(HookControl::Continue)
        })
    });

    state.pre_tool(|_,tool_use |{
        info!("--- Before tool call: {:?}", tool_use);
        Box::pin(async move { Ok(HookControl::Continue) })
    });

    state.post_tool(|_, tool_use, tool_result| {
        info!("--- After tool call: {:?}, result: {:?}", tool_use, tool_result);
        Box::pin(async move { Ok(HookControl::Continue)})
    });

    if let HookControl::Block(reason) = invoke_hooks!(SessionStart, &state)? {
        println!("--- Session blocked: {}", reason);
        return Ok(());
    }

    loop {
        let query = Text::new("--- How can I help you?")
            .prompt()
            .context("An error happened or user cancelled the input.")?;

        // 输入 exit() 退出循环
        if query.trim() == "exit()" {
            break;
        }

        if query.trim() == "/rules" {
            for (index, rule) in state.manager.rules().iter().enumerate() {
                println!("  {index}: {rule}");
            }
            continue;
        }

        if query.trim().starts_with("/mode") {
            state
                .handle_mode_command(&query)
                .context("failed to switch permission mode")?;
            continue;
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
        let Some(final_msg) = state.context.last() else {
            continue;
        };
        println!("--- Final response:\n{}", extract_text(final_msg));
    }

    Ok(())
}
