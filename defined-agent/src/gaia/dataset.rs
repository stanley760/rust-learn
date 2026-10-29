use anyhow::Context;
use arrow_array::cast::AsArray;
use arrow_array::{Array, StructArray};
use backon::{ExponentialBuilder, Retryable};
use bytes::Bytes;
use parquet::arrow::arrow_reader::ParquetRecordBatchReader;
use std::time::Duration;

use crate::gaia::model::{AnnotatorMetadata, GaiaRow};

/// HuggingFace 数据集端点。
///
/// 默认走国内可直连的 hf-mirror；官方域名在部分网络环境下不可达，
/// 可用环境变量 `HF_ENDPOINT` 覆盖（如 `https://huggingface.co`）。
fn hf_endpoint() -> String {
    std::env::var("HF_ENDPOINT")
        .unwrap_or_else(|_| "https://hf-mirror.com".to_string())
        .trim_end_matches('/')
        .to_string()
}

/// GAIA 元数据 parquet 的仓库路径。
///
/// 该仓库为 gated 数据集，需带 `HF_TOKEN` 访问；授权为 `auto`，通过后即可下载。
const METADATA_PATH: &str = "2023/validation/metadata.level1.parquet";

/// 读取 GAIA Level 1 验证集。
///
/// 直接下载仓库中的 parquet 并解析，不依赖 HuggingFace 的 datasets-server
/// 服务（该服务在国内网络下通常不可达）。
pub async fn load_gaia_level1() -> anyhow::Result<Vec<GaiaRow>> {
    let token = std::env::var("HF_TOKEN").context("HF_TOKEN not found")?;
    let url = format!(
        "{}/datasets/gaia-benchmark/GAIA/resolve/main/{METADATA_PATH}",
        hf_endpoint()
    );

    // 镜像站点偶有连接中断，重试若干次。
    let op = || async {
        reqwest::Client::new()
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .with_context(|| format!("failed to request {url}"))?
            .error_for_status()
            .with_context(|| format!("failed to download GAIA metadata from {url}"))?
            .bytes()
            .await
            .context("failed to read GAIA metadata body")
    };

    let bytes = op
        .retry(
            ExponentialBuilder::default()
                .with_min_delay(Duration::from_millis(500))
                .with_max_times(3)
                .with_jitter(),
        )
        .notify(|err: &anyhow::Error, dur: Duration| {
            tracing::warn!("下载 GAIA 数据集失败({err:#})，{dur:?} 后重试");
        })
        .await?;

    parse_gaia_parquet(&bytes)
}

/// 解析 GAIA 元数据 parquet。
///
/// 列类型（实测）：`task_id` / `Question` / `Level` / `Final answer` /
/// `file_name` / `file_path` 均为 `Utf8`；`Annotator Metadata` 是嵌套
/// `Struct`，而非 JSON 字符串。
fn parse_gaia_parquet(bytes: &[u8]) -> anyhow::Result<Vec<GaiaRow>> {
    // parquet 的 ChunkReader 需要拥有所有权的字节容器。
    let reader = ParquetRecordBatchReader::try_new(Bytes::copy_from_slice(bytes), 1024)?;
    let mut rows = Vec::new();

    for batch in reader {
        let batch = batch.context("failed to read parquet batch")?;

        let task_id = string_column(&batch, "task_id")?;
        let question = string_column(&batch, "Question")?;
        let level = string_column(&batch, "Level")?;
        let final_answer = string_column(&batch, "Final answer")?;
        let file_name = string_column(&batch, "file_name")?;
        let file_path = string_column(&batch, "file_path")?;
        let metadata = metadata_column(&batch, "Annotator Metadata")?;

        for i in 0..batch.num_rows() {
            rows.push(GaiaRow {
                task_id: task_id.value(i).to_string(),
                question: question.value(i).to_string(),
                level: level.value(i).to_string(),
                final_answer: final_answer.value(i).to_string(),
                file_name: file_name.value(i).to_string(),
                file_path: (!file_path.is_null(i)).then(|| file_path.value(i).to_string()),
                annotator_metadata: metadata_at(&metadata, i),
            });
        }
    }

    Ok(rows)
}

/// 取出一个 `Utf8` 列。
fn string_column<'a>(
    batch: &'a arrow_array::RecordBatch,
    name: &str,
) -> anyhow::Result<&'a arrow_array::StringArray> {
    let column = batch
        .column_by_name(name)
        .with_context(|| format!("missing column: {name}"))?;

    Ok(column.as_string::<i32>())
}

/// 取出 `Annotator Metadata` 结构体列。
fn metadata_column<'a>(
    batch: &'a arrow_array::RecordBatch,
    name: &str,
) -> anyhow::Result<&'a StructArray> {
    let column = batch
        .column_by_name(name)
        .with_context(|| format!("missing column: {name}"))?;

    column
        .as_any()
        .downcast_ref::<StructArray>()
        .with_context(|| format!("column {name} is not a struct column"))
}

/// 读取第 `row` 行的 `Annotator Metadata`。
///
/// 子字段缺失或为 null 时退化为空串，避免个别脏数据导致整批失败。
fn metadata_at(metadata: &StructArray, row: usize) -> AnnotatorMetadata {
    let field = |name: &str| -> String {
        metadata
            .column_by_name(name)
            .map(|column| column.as_string::<i32>())
            .and_then(|column| {
                if column.is_null(row) {
                    None
                } else {
                    Some(column.value(row).to_string())
                }
            })
            .unwrap_or_default()
    };

    AnnotatorMetadata {
        steps: field("Steps"),
        number_of_steps: field("Number of steps"),
        how_long_did_this_take: field("How long did this take?"),
        tools: field("Tools"),
        number_of_tools: field("Number of tools"),
    }
}
