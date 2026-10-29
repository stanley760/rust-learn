use defined_agent::llm::stream::chat_stream_with_retry;
use std::io::Write;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志（重试告警等会显示在这里）
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    dotenvy::dotenv()?;
    let model = std::env::var("OPENAI_MODEL")?;

    let prompts = vec![
        "11*11是多少",
        "2026年女子中网公开赛冠军是谁?",
    ];

    // 顺序执行：多路并发时输出会在终端交错，无法逐 token 跟随
    for prompt in prompts {
        println!("\n=== {prompt} ===");

        // 关键：on_delta 每收到一段增量文本就立刻打印并 flush，
        // 于是输出跟着生成实时出现，而不是等全部完成后一次性显示。
        let output =
            chat_stream_with_retry(&model, Some("你是一个全能助理"), prompt, |chunk| {
                print!("{chunk}");
                let _ = std::io::stdout().flush();
            })
            .await?;
        println!();

        // output 是完整回复；上面已实时展示，这里只留给后续处理用
        let _ = output;
    }

    Ok(())
}
