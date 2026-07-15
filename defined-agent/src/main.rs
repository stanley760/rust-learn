use defined_agent::llm::complete;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

// [CLIPPY-WARNING] redundant_static_lifetimes: static str (line 5)
const CHAT_GPT_120B: &'static str = "openai/gpt-oss-120b:free";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let key = std::env::var("OPENAI_BASE_URL")?;
    println!("{:?}", key);
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let res = complete::chat_stream_with_retry(CHAT_GPT_120B,
            Some("你是一个诗人"),
            "请你帮我取一个人名，要求高雅，但是不能与大多数人名重复，可以从诗经等经典名著中取字").await;

    println!("\n--------------------------------");
    match res {
        Ok(result) => {println!("{result:#?}");},
        Err(e) => {eprintln!("exception:{}", e);}
    }
    Ok(())
}
