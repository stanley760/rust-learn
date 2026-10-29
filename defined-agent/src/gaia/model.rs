use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct HfResponse {
    pub rows: Vec<HfRow>
}

#[derive(Debug, Deserialize)]
pub struct HfRow {
    pub row: GaiaRow
}

#[derive(Debug, Deserialize, Clone)]
pub struct GaiaRow {
    pub task_id: String,

    #[serde(rename = "Question")]
    pub question: String,

    #[serde(rename = "Level", deserialize_with = "de_level")]
    pub level: String,

    #[serde(rename = "Final answer")]
    pub final_answer: String,

    pub file_name: String,

    /// 附件路径由 GAIA.py 加载脚本生成,直接读元数据时可能没有该列
    #[serde(default)]
    pub file_path: Option<String>,

    #[serde(rename = "Annotator Metadata")]
    pub annotator_metadata: AnnotatorMetadata,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AnnotatorMetadata {
    #[serde(rename = "Steps")]
    pub steps: String,

    #[serde(rename = "Number of steps")]
    pub number_of_steps: String,

    #[serde(rename = "How long did this take?")]
    pub how_long_did_this_take: String,

    #[serde(rename = "Tools")]
    pub tools: String,

    #[serde(rename = "Number of tools")]
    pub number_of_tools: String,
}

/// GAIA.py 里 `Level` 声明为 string,而原始 metadata.jsonl 里是整数,两种都要兼容
fn de_level<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum LevelRepr {
        Num(u64),
        Str(String),
    }
    Ok(match LevelRepr::deserialize(deserializer)? {
        LevelRepr::Num(n) => n.to_string(),
        LevelRepr::Str(s) => s,
    })
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[schemars(deny_unknown_fields)]
pub struct GaiaOutput {
    pub is_solvable:bool,
    pub unsolvable_reason: String,
    pub final_answer: String,
}

#[derive(Debug)]
pub struct GaiaEvalResponse {
    pub task_id: String,
    pub model: String,
    pub correct: bool,
    pub is_solvable: Option<bool>,
    pub prediction: Option<String>,
    pub answer: String,
    pub unsolvable_reason: Option<String>,
    pub error: Option<String>
}