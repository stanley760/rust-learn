use async_trait::async_trait;
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{llm::ExecutionContext, tools::{Tool, ToolError, web_search::execute::web_search}};

/// 检索结果条数的默认值。
///
/// 该值由服务端掌控，不暴露给模型：模型无法决定一次检索拉回多少结果，
/// 避免其凭先验随意填充（实测恒为 5），也便于统一控制成本与上下文长度。
/// 需要调整时改环境变量 `WEB_SEARCH_MAX_RESULTS` 即可，无需改代码。
pub const DEFAULT_MAX_RESULTS: u8 = 2;

/// 读取服务端配置的检索条数，非法或缺省时回退到 [`DEFAULT_MAX_RESULTS`]。
pub fn max_results() -> u8 {
    std::env::var("WEB_SEARCH_MAX_RESULTS")
        .ok()
        .and_then(|raw| raw.trim().parse::<u8>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_MAX_RESULTS)
}

/// 模型的检索入参。
///
/// 注意这里刻意不包含 `max_results`：条数属于服务端策略，而非模型可决策的语义。
/// 即便模型硬塞该字段，serde 也会直接忽略（未知字段默认忽略）。
#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Search query for the web search tool")]
pub struct WebSearchArgs {
    pub query: String,

    #[serde(default = "default_topic")]
    pub topic: String,

    #[serde(default)]
    pub time_range: Option<String>,
}

fn default_topic() -> String {
    "general".to_string()
}

#[derive(Debug, Serialize)]
pub(crate) struct TavilyRequest<'a> {
    pub(crate) api_key: &'a str,
    pub(crate) query: &'a str,
    pub(crate) max_results: u8,
    pub(crate) topic: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) time_range: Option<&'a str>,

    pub(crate) search_depth: &'a str,
    pub(crate) include_answer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct TavilyResponse {
    pub results: Vec<SearchResult>,
    pub answer: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WebSearchOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,

    pub results: Vec<SearchResult>,
}

pub struct WebSearchTool;

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &str {
        "WebSearchTool"
    }

    fn description(&self) -> &str {
        "Search the web for the user's question. Prefer trustworthy results published in the current year, summarize only what the sources support, disclose when current-year information is unavailable, and include the source links."
    }

    fn parameters(&self) -> Value {
        serde_json::to_value(schema_for!(WebSearchArgs))
            .expect("Failed to serialize WebSearchArgs schema")
    }

    /// 一次对话内只搜索一次，避免对同一主题反复请求外部服务。
    fn once_per_request(&self) -> bool {
        true
    }

    async fn execute(&self, arg_json: &str, _content: &ExecutionContext) -> Result<String, ToolError> {
        let args: WebSearchArgs = serde_json::from_str(arg_json).map_err(|e| {
            ToolError::Permanent(format!("invalid web search arguments: {e}"))
        })?;

        let output = web_search(args).await?;
        serde_json::to_string(&output).map_err(|e| {
            ToolError::Transient(format!("failed to serialize web search output: {e}"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description_prioritizes_current_year_and_source_links() {
        let description = WebSearchTool.description();

        assert!(description.contains("current year"));
        assert!(description.contains("source links"));
    }

    #[test]
    fn web_search_is_declared_once_per_request() {
        assert!(WebSearchTool.once_per_request());
    }

    /// 检索条数不应出现在给模型的 schema 中：它属于服务端策略，而非模型可决策的语义。
    #[test]
    fn max_results_is_not_exposed_to_the_model() {
        let schema = serde_json::to_value(schema_for!(WebSearchArgs)).unwrap();
        let properties = schema
            .get("properties")
            .and_then(|value| value.as_object())
            .expect("schema should have properties");

        assert!(
            !properties.contains_key("max_results"),
            "max_results should not be exposed to the model: {schema}"
        );
    }

    /// 模型硬塞 max_results 时应被忽略，不能影响服务端策略。
    #[test]
    fn model_supplied_max_results_is_ignored() {
        let args: WebSearchArgs =
            serde_json::from_str(r#"{"query":"rust","max_results":5}"#).unwrap();

        // 未知字段被 serde 丢弃，入参里根本不存在该值。
        assert_eq!(args.query, "rust");
        assert_eq!(args.topic, "general");
    }
}
