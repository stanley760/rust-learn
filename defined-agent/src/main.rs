use anyhow::Ok;

use defined_agent::llm::complete;
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
        Some("你是一个旅游博主"),
        "我要在最近准备从成都到九寨沟旅游，给出一个详细的旅游攻略").await?;
    println!("{result:#?}");
    Ok(())
}
 