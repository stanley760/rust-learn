use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentItem {
    #[serde(rename = "message")]
    Message { role: String, content: String},
    #[serde(rename = "tool_call")]
    ToolCall {
        tool_call_id: String, 
        name: String,
        arguments: Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_call_id: String,
        name: String,
        status: ToolResultStatus,
        content: String,
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolResultStatus {
    Success,
    Error,
}