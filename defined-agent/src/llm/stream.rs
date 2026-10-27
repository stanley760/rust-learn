use async_openai::types::chat::{
    ChatCompletionMessageToolCall, ChatCompletionMessageToolCalls,
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionTools, CreateChatCompletionRequestArgs, FunctionCall,
};
use async_stream::stream;
use backon::{ExponentialBuilder, Retryable};
use futures::{Stream, StreamExt};
use std::{collections::BTreeMap, sync::Mutex};

use crate::tools::{ToolBox, build_toolbox};

/// 流式产出：文本增量，或流结束后一次性给出的工具调用
pub enum StreamItem {
    Text(String),
    ToolCalls(Vec<ChatCompletionMessageToolCalls>),
}

/// 单个 tool call 的分片累积器
/// 同一调用的分片共享 index；id/name 只在首片出现，arguments 逐段追加
#[derive(Default)]
struct ToolCallAcc {
    id: Option<String>,
    name: Option<String>,
    arguments: String,
}

const MAX_TOOL_ROUNDS: i16 = 3;

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
        let tool_definitions: Vec<ChatCompletionTools> = toolbox
            .values()
            .filter_map(|t| match t.definition() {
                Ok(def) => Some(def),
                Err(e) => {
                    tracing::warn!("Skip tool {}, failed to get its definition: {e}", t.name());
                    None
                }
            })
            .collect();

        for _round in 0..MAX_TOOL_ROUNDS {
            let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(msg.clone())
            .tools(tool_definitions.clone())
            .max_tokens(2048u32)
            .build()?;


            // key = 分片的 index，BTreeMap 天然按 index 有序，省掉最后手动 sort
            let mut tool_acc: BTreeMap<u32, ToolCallAcc> = BTreeMap::new();

            let mut response = cli.chat().create_stream(request).await?;
            while let Some(response_result) = response.next().await {
                match response_result {
                    Ok(chunk) => {
                        for choice in &chunk.choices {
                            // 1. As same as the last version
                            if let Some(text) = &choice.delta.content {
                                yield Ok(text.clone());
                            }
                            // 2. tool_call
                            if let Some(fragments) = &choice.delta.tool_calls {
                                for fragment in fragments {
                                    let acc = tool_acc.entry(fragment.index).or_default();
                                    if let Some(id) = &fragment.id {
                                        acc.id = Some(id.clone());
                                    }
                                    if let Some(func) = &fragment.function {

                                        if let Some(name) = &func.name {
                                            acc.name = Some(name.clone());
                                        }
                                        if let Some(args) = &func.arguments {
                                            acc.arguments.push_str(args);
                                        }
                                    }
                                }
                            }
                        }

                    }

                    Err(err) => yield Err(err.into())
                }
            }
            // 流结束：碎片拼装成完整 tool call
            if !tool_acc.is_empty() {
                let calls :Vec<ChatCompletionMessageToolCalls> = tool_acc.into_values().map(|acc| {
                    ChatCompletionMessageToolCalls::Function(ChatCompletionMessageToolCall {
                        id: acc.id.unwrap_or_default(),
                        function: FunctionCall {
                            name: acc.name.unwrap_or_default(),
                            arguments: acc.arguments
                        },
                    })
                }).collect();

                msg.push(ChatCompletionRequestAssistantMessageArgs::default()
                    .tool_calls(calls.clone())
                    .build()?
                    .into());

                for call in calls {
                    if let ChatCompletionMessageToolCalls::Function(function_call) = call {
                        let args = function_call.function.arguments;
                        let name = function_call.function.name;
                        tracing::info!("Tool call: {name}({args})");

                        let tool_result = match toolbox.get(&name) {
                            Some(tool) => match tool.execute(&args).await {
                                Ok(result) => {
                                    tracing::info!("Tool result: {result}");
                                    result
                                }
                                Err(err) => {
                                    let msg = format!("Tool execution error: {err}");
                                    tracing::error!("{msg}");
                                    msg
                                }
                            },
                            None => {
                                let msg = format!("Tool execution error: {name}");
                                tracing::error!("{msg}");
                                msg
                            }
                        };

                        msg.push(
                            ChatCompletionRequestToolMessageArgs::default()
                                .tool_call_id(function_call.id.clone())
                                .content(tool_result)
                                .build()?
                                .into(),
                        );
                    }
                }

            }
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
                    (&mut *cb)(&txt);
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
