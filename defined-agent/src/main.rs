use tracing::Level;
use tracing_subscriber::FmtSubscriber;

// [CLIPPY-WARNING] redundant_static_lifetimes: static str (line 5)
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let _key = std::env::var("OPENAI_BASE_URL")?;
    let model = std::env::var("OPENAI_MODEL")?;
    println!("model:{:?}", model);
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
