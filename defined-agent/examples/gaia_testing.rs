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
    dotenvy::dotenv()?;
    gaia_level1_experiment().await
}

/// 待评测的模型列表。
///
/// 从环境变量 `GAIA_MODELS` 读取，逗号分隔；未设置时回退到默认列表。
/// 这样切换平台只需改 `.env`，无需改代码：
/// - OpenRouter（免费层每日仅 50 次，跑不完 GAIA）：
///   `GAIA_MODELS=dots-studio/dots-3-note-preview:free,nvidia/nemotron-3.5-lightning:free`
/// - 魔搭 ModelScope：
///   `GAIA_MODELS=Qwen/Qwen3.5-35B-A3B,Qwen/Qwen3.8-27B,deepseek-ai/DeepSeek-V4-Pro`
fn models() -> Vec<String> {
    match std::env::var("GAIA_MODELS") {
        Ok(raw) => {
            let list: Vec<String> = raw
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect();
            if list.is_empty() {
                tracing::warn!("GAIA_MODELS 为空，回退到默认模型列表");
                default_models()
            } else {
                list
            }
        }
        Err(_) => default_models(),
    }
}

fn default_models() -> Vec<String> {
    vec![
        "Qwen/Qwen3.5-35B-A3B".to_string(),
        "Qwen/Qwen3.8-27B".to_string(),
    ]
}

pub async fn gaia_level1_experiment() -> anyhow::Result<()> {
    let problems = load_gaia_level1().await?;
    let models = models();
    tracing::info!(
        "开始评测：{} 道题 × {} 个模型",
        problems.len(),
        models.len()
    );

    let mut set = JoinSet::new();
    for model in models.iter() {
        for problem in problems.iter() {
            let lock = Arc::new(Mutex::new(()));
            let problem = problem.clone();
            let model = model.clone();
            set.spawn(async move {
                let guard = lock.lock().await;
                let eval = evaluate_gaia(problem, &model).await;
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
