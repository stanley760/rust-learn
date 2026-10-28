use async_openai::types::chat::{
    ChatCompletionMessageToolCall, ChatCompletionMessageToolCallChunk, ChatCompletionMessageToolCalls,
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessageArgs, ChatCompletionTools,
    CreateChatCompletionRequestArgs, FunctionCall,
};
use async_stream::stream;
use backon::{ExponentialBuilder, Retryable};
use futures::{Stream, StreamExt};
use std::{collections::BTreeMap, collections::HashSet, sync::Mutex};

use crate::tools::{ToolBox, build_toolbox, execute_tool};

/// 单次请求内最多允许的工具调用轮次。
const MAX_TOOL_ROUNDS: i16 = 2;

/// 单个 tool call 的分片累积器
/// 同一调用的分片共享 index；id/name 只在首片出现，arguments 逐段追加
#[derive(Default)]
struct ToolCallAcc {
    id: Option<String>,
    name: Option<String>,
    arguments: String,
}

impl ToolCallAcc {
    /// 合并一个分片。
    fn push(&mut self, fragment: &ChatCompletionMessageToolCallChunk) {
        if let Some(id) = &fragment.id {
            self.id = Some(id.clone());
        }
        if let Some(function) = &fragment.function {
            if let Some(name) = &function.name {
                self.name = Some(name.clone());
            }
            if let Some(arguments) = &function.arguments {
                self.arguments.push_str(arguments);
            }
        }
    }

    /// 输出完整的 tool call。
    fn finish(self) -> ChatCompletionMessageToolCalls {
        ChatCompletionMessageToolCalls::Function(ChatCompletionMessageToolCall {
            id: self.id.unwrap_or_default(),
            function: FunctionCall {
                name: self.name.unwrap_or_default(),
                arguments: self.arguments,
            },
        })
    }
}

/// 生成给模型的工具定义列表，跳过本次已用完的一次性工具。
fn available_tools(toolbox: &ToolBox, used: &HashSet<String>) -> Vec<ChatCompletionTools> {
    toolbox
        .values()
        .filter(|tool| !(tool.once_per_request() && used.contains(tool.name())))
        .filter_map(|tool| match tool.definition() {
            Ok(definition) => Some(definition),
            Err(e) => {
                tracing::warn!(
                    "Skip tool {}, failed to get its definition: {e}",
                    tool.name()
                );
                None
            }
        })
        .collect()
}

/// 执行模型的工具调用，返回回传给模型的工具结果文本。
///
/// 工具自身失败时由 [`execute_tool`] 负责重试；重试耗尽后把错误当作结果
/// 回传给模型，让模型有机会自行纠正，而不是直接中断整轮对话。
async fn run_tool_call(
    toolbox: &ToolBox,
    call: &ChatCompletionMessageToolCalls,
    used: &mut HashSet<String>,
) -> String {
    let ChatCompletionMessageToolCalls::Function(function_call) = call else {
        return "Unsupported tool call type".to_string();
    };

    let name = &function_call.function.name;
    let args = &function_call.function.arguments;
    tracing::info!("Tool call: {name}({args})");

    let Some(tool) = toolbox.get(name) else {
        tracing::error!("Tool execution error: unknown tool {name}");
        return format!("Tool execution error: unknown tool {name}");
    };

    // 一次性工具在本次请求内只允许调用一次，重复调用时复用既有结果。
    if tool.once_per_request() && !used.insert(name.clone()) {
        return format!("Tool execution skipped: {name} was already called during this request. Use the existing result.");
    }

    match execute_tool(tool.as_ref(), args).await {
        Ok(result) => {
            tracing::info!("Tool result: {result}");
            result
        }
        Err(err) => {
            let message = format!("Tool execution error: {err}");
            tracing::error!("{message}");
            message
        }
    }
}

fn chat_stream(
    model: &str,
    sys_prompt: Option<&str>,
    user_prompt: &str,
    toolbox: &ToolBox,
) -> impl Stream<Item = anyhow::Result<String>> {
    stream! {
        let cli = async_openai::Client::new();
        let mut msg = vec![];

        if let Some(sys) = sys_prompt {
            msg.push(
                ChatCompletionRequestSystemMessageArgs::default()
                .content(sys)
                .build()?
                .into(),
            );
        }

        msg.push(
            ChatCompletionRequestUserMessageArgs::default()
            .content(user_prompt)
            .build()?
            .into(),
        );

        // 本次请求内已调用过的一次性工具。
        let mut used = HashSet::new();
        let mut round: i16 = 0;

        while round <= MAX_TOOL_ROUNDS {
            // 最后一轮是收尾轮：不再下发工具，强制模型基于已有结果作答。
            let tools = if round < MAX_TOOL_ROUNDS {
                available_tools(toolbox, &used)
            } else {
                Vec::new()
            };

            let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(msg.clone())
            .tools(tools)
            .max_tokens(2048u32)
            .build()?;

            // key = 分片的 index，BTreeMap 天然按 index 有序，省掉最后手动 sort
            let mut tool_acc: BTreeMap<u32, ToolCallAcc> = BTreeMap::new();

            let mut response = cli.chat().create_stream(request).await?;
            while let Some(response_result) = response.next().await {
                match response_result {
                    Ok(chunk) => {
                        for choice in &chunk.choices {
                            // 1. 文本增量
                            if let Some(text) = &choice.delta.content {
                                yield Ok(text.clone());
                            }
                            // 2. tool_call 分片
                            if let Some(fragments) = &choice.delta.tool_calls {
                                for fragment in fragments {
                                    tool_acc.entry(fragment.index).or_default().push(fragment);
                                }
                            }
                        }
                    }
                    Err(err) => yield Err(err.into()),
                }
            }

            // 流结束：碎片拼装成完整 tool call
            if tool_acc.is_empty() {
                break;
            }

            let calls: Vec<ChatCompletionMessageToolCalls> =
                tool_acc.into_values().map(ToolCallAcc::finish).collect();

            // 最后一轮不再允许新的工具调用，模型若仍请求工具则忽略并收尾。
            if round == MAX_TOOL_ROUNDS {
                tracing::warn!("Ignoring tool calls from the final synthesis round");
                break;
            }

            msg.push(
                ChatCompletionRequestAssistantMessageArgs::default()
                    .tool_calls(calls.clone())
                    .build()?
                    .into(),
            );

            for call in calls {
                let tool_result = run_tool_call(toolbox, &call, &mut used).await;

                if let ChatCompletionMessageToolCalls::Function(function_call) = &call {
                    msg.push(
                        ChatCompletionRequestToolMessageArgs::default()
                            .tool_call_id(function_call.id.clone())
                            .content(tool_result)
                            .build()?
                            .into(),
                    );
                }
            }

            round += 1;
        }
    }
}

/// 流式对话：每收到一段增量文本就调用 `on_delta`，可边生成边展示；返回完整回复。
///
/// 失败时按指数退避最多重试 3 次。重试会从头重新生成，此前已通过 `on_delta`
/// 打印出的部分文本不会自动清除（可通过重试日志感知）。
pub async fn chat_stream_with_retry<F>(
    model: &str,
    sys_prompt: Option<&str>,
    user_prompt: &str,
    on_delta: F,
) -> anyhow::Result<String>
where
    F: FnMut(&str),
{
    // Retryable 要求闭包是 FnMut() -> Fut，且 Fut 不能借用闭包捕获的资源（lending 限制）。
    // 把回调包进 Mutex 后，闭包只捕获可 Copy 的 &Mutex<_>，就能反复生成 future。
    let on_delta = Mutex::new(on_delta);

    let op = || async {
        let toolbox = build_toolbox();
        let s = chat_stream(model, sys_prompt, user_prompt, &toolbox);
        futures::pin_mut!(s);
        let mut output = String::new();
        while let Some(res) = s.next().await {
            match res {
                Ok(txt) => {
                    output.push_str(&txt);
                    let mut cb = on_delta.lock().expect("on_delta callback poisoned");
                    (*cb)(&txt);
                }
                Err(err) => {
                    tracing::error!("\nError while streaming:{}", err);
                    return Err(err);
                }
            }
        }
        Ok(output)
    };

    op.retry(ExponentialBuilder::default().with_max_times(3))
        .notify(|err: &anyhow::Error, dur: std::time::Duration| {
            tracing::warn!("生成中断: {err:#}，{dur:?} 后重试");
        })
        .await
}
