use async_trait::async_trait;
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::tools::{Tool, web_search::execute::web_search};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WebSearchArgs {
    pub query: String,

    #[schemars(range(min = 0, max = 20))]
    #[serde(default = "default_max_results")]
    pub max_results: u8,

    #[serde(default = "default_topic")]
    pub topic: String,

    #[serde(default)]
    pub time_range: Option<String>,
}

fn default_max_results() -> u8 {
    2
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
    // pub score: Option<f64>
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
        "search with a third web engine"
    }

    fn parameters(&self) -> Value {
        serde_json::to_value(schema_for!(WebSearchArgs))
            .expect("Failed to serialize CalculatorArgs schema")
    }

    async fn execute(&self, arg_json: &str) -> anyhow::Result<String> {
        let args: WebSearchArgs = serde_json::from_str(arg_json)?;
        match web_search(args).await {
            Ok(output) => Ok(serde_json::to_string(&output)?),
            Err(err) => Ok(format!("Error: {err}")),
        }
        
    }
}