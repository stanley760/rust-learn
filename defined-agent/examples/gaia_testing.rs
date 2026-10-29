use std::{collections::HashMap, sync::Arc};

use defined_agent::gaia::{
    dataset::load_gaia_level1, evaluator::evaluate_gaia, model::GaiaEvalResponse,
};
use tokio::{sync::Mutex, task::JoinSet};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // 加载 .env：HF_TOKEN 等密钥只存在于 .env 文件中，不会自动进入进程环境变量。
    dotenvy::dotenv()?;
    gaia_level1_experiment().await
}

const MODELS: &[&str] = &[
    "poolside/laguna-s-2.1:free",
    "dots-studio/dots-3-note-preview:free",
];

pub async fn gaia_level1_experiment() -> anyhow::Result<()> {
    let problems = load_gaia_level1().await?;
    let mut set = JoinSet::new();
    for model in MODELS.iter() {
        for problem in problems.iter() {
            let lock = Arc::new(Mutex::new(()));
            let problem = problem.clone();
            set.spawn(async move {
                let guard = lock.lock().await;
                let eval = evaluate_gaia(problem, model).await;
                drop(guard);
                Ok::<_, anyhow::Error>(eval)
            });
        }
    }

    let mut results: HashMap<String, Vec<GaiaEvalResponse>> = HashMap::new();
    while let Some(Ok(result)) = set.join_next().await {
        match result {
            Ok(eval) => {
                tracing::info!("{eval:#?}");
                results.entry(eval.model.clone()).or_default().push(eval);
            }
            Err(err) => tracing::error!("task panicked: {err}"),
        }
    }

    for (model, evals) in &results {
        let correct = evals.iter().filter(|e| e.correct).count();
        let total = evals.len();
        tracing::info!(
            "{model}: {correct}/{total} ({:.1}%)",
            correct as f64 / total as f64 * 100.0
        );
    }

    Ok(())
}
