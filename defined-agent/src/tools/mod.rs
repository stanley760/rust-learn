pub mod calculator;
pub mod web_search;

use std::collections::HashMap;
use std::time::Duration;

use async_openai::types::chat::{ChatCompletionTool, ChatCompletionTools, FunctionObjectArgs};
use async_trait::async_trait;
use backon::{ExponentialBuilder, Retryable};
use serde_json::Value;

use crate::{llm::ExecutionContext, tools::{calculator::calculate::CalculatorTool, web_search::websearch::WebSearchTool}};

/// 工具执行失败后的最大重试次数（不含首次调用）。
pub const MAX_TOOL_RETRIES: usize = 3;

/// 工具执行错误。
///
/// 区分「重试可能自愈的临时故障」与「重试也不会改变的确定性错误」，
/// 让重试策略只作用在真正值得重试的场景上。
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    /// 网络抖动、限流、服务端错误等临时性故障，重试可能成功。
    #[error("临时性故障: {0}")]
    Transient(String),

    /// 参数非法、逻辑不支持等确定性错误，重试无意义。
    #[error("确定性错误: {0}")]
    Permanent(String),
}

impl ToolError {
    /// 是否值得重试。
    pub fn is_retryable(&self) -> bool {
        matches!(self, ToolError::Transient(_))
    }
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;

    fn description(&self) -> &str;

    fn parameters(&self) -> Value;

    /// 同一轮对话内是否只允许调用一次（如联网搜索，避免重复请求同一主题）。
    fn once_per_request(&self) -> bool {
        false
    }

    async fn execute(&self, arg_json: &str, context: &ExecutionContext) -> Result<String, ToolError>;

    fn definition(&self) -> anyhow::Result<ChatCompletionTools> {
        let function = FunctionObjectArgs::default()
            .name(self.name())
            .description(self.description())
            .parameters(self.parameters())
            .build()
            .map_err(|e| {
                anyhow::anyhow!("Failed to build tool definition for {}: {e}", self.name())
            })?;

        Ok(ChatCompletionTools::Function(ChatCompletionTool {
            function,
        }))
    }
}

/// 工具箱：名称 → 工具实例。
///
/// 用 `BTreeMap` 而非 `HashMap`，保证每次请求下发给模型的工具列表顺序稳定。
pub type ToolBox = HashMap<String, Box<dyn Tool>>;

pub fn build_toolbox() -> ToolBox {
    let tools: Vec<Box<dyn Tool>> = vec![Box::new(CalculatorTool), Box::new(WebSearchTool)];
    tools
        .into_iter()
        .map(|tool| (tool.name().to_string(), tool))
        .collect()
}

/// 执行工具调用。
///
/// 临时性故障按指数退避重试至多 [`MAX_TOOL_RETRIES`] 次，成功即返回结果；
/// 重试耗尽则返回最后一次错误。
pub async fn execute_tool(tool: &dyn Tool, arg_json: &str) -> Result<String, ToolError> {
    // Retryable 要求闭包是 FnMut() -> Fut，且 Fut 不能借用闭包的可变捕获。
    // 这里捕获的都是 Copy 引用，每次调用生成独立 future，不存在借用冲突。
    let op = move || async move { tool.execute(arg_json, &ExecutionContext::new()).await };

    op.retry(
        ExponentialBuilder::default()
            .with_min_delay(Duration::from_millis(500))
            .with_max_times(MAX_TOOL_RETRIES)
            .with_jitter(),
    )
    .when(|err: &ToolError| err.is_retryable())
    .notify(|err: &ToolError, dur: Duration| {
        tracing::warn!("工具 {} 执行失败({err})，{dur:?} 后重试", tool.name());
    })
    .await
}

