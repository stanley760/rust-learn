use defined_agent::llm::complete;
use futures::StreamExt;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

const CHAT_GPT_120: &'static str = "openai/gpt-oss-120b:free";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let key = std::env::var("OPENAI_BASE_URL")?;
    println!("{:?}", key);
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let result = complete::chat_complete_structed(
        CHAT_GPT_120,
        Some("你是一个天文学家"),
        "请你帮我写一篇关于黑洞的科普文章，要求内容通俗易懂，适合大众阅读。",
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
    println!("--------------------------------");
    Ok(())
}
