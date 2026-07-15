mod params_content;
use std::{fs::{self, File}, io::{BufWriter, Write}, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

use anyhow::{Context, Ok};
use async_openai::types::chat::{ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessageContent, ChatCompletionRequestAssistantMessageContentPart, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageContent, ChatCompletionRequestSystemMessageContentPart, ChatCompletionRequestToolMessageContent, ChatCompletionRequestToolMessageContentPart, ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent, ChatCompletionRequestUserMessageContentPart, CreateChatCompletionRequest};



use crate::{context::params_content::*, structure::{LoopState, get_model}};


#[derive(Debug, Default)]
pub struct CompactState {
    pub has_compacted: bool,
    pub last_summary: Option<String>,
    pub recent_files: Vec<String>,
}

pub fn micro_compact(messages: &mut [ChatCompletionRequestMessage]) {
    let tool_result_positions = collect_tool_result_positions(messages);

    if tool_result_positions.len() <= KEEP_RECENT_TOOL_RESULTS {
        return;
    }

    let compact_until = tool_result_positions.len() - KEEP_RECENT_TOOL_RESULTS;

    for idx in tool_result_positions.into_iter().take(compact_until) {
        let Some(ChatCompletionRequestMessage::Tool(tool_msg)) = messages.get_mut(idx) else {
            continue;
        };

        let text = match &tool_msg.content {
            ChatCompletionRequestToolMessageContent::Text(t) => t.clone(),
            ChatCompletionRequestToolMessageContent::Array(parts) => parts
                .iter()
                .map(|p| match p {
                    ChatCompletionRequestToolMessageContentPart::Text(t) => t.text.as_str(),
                })
                .collect::<Vec<_>>()
                .join(""),
        };

        if text.chars().count() > 120 {
            tool_msg.content =
                ChatCompletionRequestToolMessageContent::Text(COMPACTED_TOOL_RESULT.to_string());
        }
    }
}

pub fn estimate_context_size(messages: &[ChatCompletionRequestMessage]) -> usize {
    serde_json::to_string(messages)
        .map(|s| s.chars().count())
        .unwrap_or_else(|_| {
            messages
                .iter()
                .map(|msg| match msg {
                    ChatCompletionRequestMessage::User(m) => match &m.content {
                        ChatCompletionRequestUserMessageContent::Text(t) => t.chars().count(),
                        ChatCompletionRequestUserMessageContent::Array(parts) => parts
                            .iter()
                            .map(|p| match p {
                                ChatCompletionRequestUserMessageContentPart::Text(t) => {
                                    t.text.chars().count()
                                }
                                _ => 0,
                            })
                            .sum(),
                    },
                    ChatCompletionRequestMessage::Assistant(m) => {
                        let text_len = match &m.content {
                            Some(ChatCompletionRequestAssistantMessageContent::Text(t)) => {
                                t.chars().count()
                            }
                            Some(ChatCompletionRequestAssistantMessageContent::Array(parts)) => {
                                parts
                                    .iter()
                                    .map(|p| match p {
                                        ChatCompletionRequestAssistantMessageContentPart::Text(
                                            t,
                                        ) => t.text.chars().count(),
                                        _ => 0,
                                    })
                                    .sum()
                            }
                            None => 0,
                        };
                        let tool_len = m
                            .tool_calls
                            .as_ref()
                            .map(|calls| {
                                calls
                                    .iter()
                                    .map(|c| match c {
                                        ChatCompletionMessageToolCalls::Function(f) => {
                                            f.function.name.chars().count()
                                                + f.function.arguments.chars().count()
                                        }
                                        ChatCompletionMessageToolCalls::Custom(c) => {
                                            c.custom_tool.name.chars().count()
                                                + c.custom_tool.input.chars().count()
                                        }
                                    })
                                    .sum::<usize>()
                            })
                            .unwrap_or(0);
                        text_len + tool_len
                    }
                    ChatCompletionRequestMessage::Tool(m) => match &m.content {
                        ChatCompletionRequestToolMessageContent::Text(t) => t.chars().count(),
                        ChatCompletionRequestToolMessageContent::Array(parts) => parts
                            .iter()
                            .map(|p| match p {
                                ChatCompletionRequestToolMessageContentPart::Text(t) => {
                                    t.text.chars().count()
                                }
                            })
                            .sum(),
                    },
                    ChatCompletionRequestMessage::System(m) => match &m.content {
                        ChatCompletionRequestSystemMessageContent::Text(t) => t.chars().count(),
                        ChatCompletionRequestSystemMessageContent::Array(parts) => {
                            parts
                                .iter()
                                .map(|p| match p {
                                    ChatCompletionRequestSystemMessageContentPart::Text(t) => {
                                        t.text.chars().count()
                                    }
                                })
                                .sum()
                        }
                    },
                    _ => 0,
                })
                .sum::<usize>()
        })
}

pub fn write_transcript(messages: &[ChatCompletionRequestMessage]) -> anyhow::Result<PathBuf> {
    let transcript_dir = std::env::current_dir()?.join(TRANSCRIPT_DIR);
    fs::create_dir_all(&transcript_dir)
        .with_context(|| format!("failed to create {}", transcript_dir.display()))?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before UNIX_EPOCH")?
        .as_secs();
    let transcript_path = transcript_dir.join(format!("transcript_{timestamp}.jsonl"));

    let file = File::create(&transcript_path)
        .with_context(|| format!("failed to create {}", transcript_path.display()))?;
    let mut writer = BufWriter::new(file);

    for message in messages {
        serde_json::to_writer(&mut writer, message).with_context(|| {
            format!(
                "failed to serialize message to {}",
                transcript_path.display()
            )
        })?;
        writer.write_all(b"\n")?;
    }
    writer.flush()?;

    Ok(transcript_path)
}

pub fn persist_large_output(tool_use_id: &str, output: &str) -> anyhow::Result<String> {
    if output.chars().count() <= PERSIST_THRESHOLD {
        return Ok(output.to_string());
    }

    let output_dir = std::env::current_dir()?.join(OUTPUT_DIR);
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("failed to create {}", output_dir.display()))?;

    let output_path = output_dir.join(format!("{tool_use_id}.txt"));

    fs::write(&output_path, output)
        .with_context(|| format!("failed to write {}", output_path.display()))?;
    let output_path = output_path.display();

    let preview = output.chars().take(PREVIEW_CHARS).collect::<String>();
    Ok(format!(
        "<persisted-output>\nFull output saved to: {output_path}\nPreview:\n{preview}\n</persisted-output>"
    ))
}

impl LoopState {
    pub async fn compact_history(&mut self, focus: Option<&str>) -> anyhow::Result<()> {
        let transcript_path =
            write_transcript(&self.context).context("failed to write transcript")?;
        println!("[transcript saved: {}]", transcript_path.display());

        let mut summary = self
            .summarize_history()
            .await
            .context("failed to summarize history")?;
        if let Some(focus) = focus {
            summary = format!("{summary}\n\nFocus to preserve next:{focus}");
        }
        if !self.compact_state.recent_files.is_empty() {
            let recent_lines = self
                .compact_state
                .recent_files
                .iter()
                .map(|f| format!("- {f}"))
                .collect::<Vec<_>>()
                .join("\n");
            summary = format!("{summary}\n\nRecent files to reopen if needed:\n{recent_lines}");
        }

        self.compact_state.has_compacted = true;
        self.compact_state.last_summary = Some(summary.clone());

        // 压缩后用一条 user message 替代全部历史
        self.context = vec![ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: ChatCompletionRequestUserMessageContent::Text(format!(
                    "This conversation was compacted so the agent can continue working.\n\n{summary}"
                )),
                name: None,
            },
        )];
        Ok(())
    }

    pub async fn summarize_history(&self) -> anyhow::Result<String> {
        let conversation_text = serde_json::to_string(&self.context)
            .context("failed to serialize conversation for summarization")?;
        let truncated = conversation_text.chars().take(80000).collect::<String>();

        let prompt = format!(
            "Summarize this coding-agent conversation so work can continue.\n\
        Preserve:\n\
        1. The current goal\n\
        2. Important findings and decisions\n\
        3. Files read or changed\n\
        4. Remaining work\n\
        5. User constraints and preferences\n\
        Be compact but concrete.\n\n\
        {truncated}"
        );

        let request = CreateChatCompletionRequest {
            model: get_model()?,
            messages: vec![ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessage {
                    content: ChatCompletionRequestUserMessageContent::Text(prompt),
                    name: None,
                },
            )],
            max_completion_tokens: Some(2000),
            ..Default::default()
        };

        let response = self.client.chat().create(request).await?;

        let text = response
            .choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .unwrap_or_default();
        Ok(text)
    }

    pub fn remember_recent_file(&mut self, path: &str) {
        self.compact_state.recent_files.retain(|p| p != path);
        self.compact_state.recent_files.push(path.to_string());

        if self.compact_state.recent_files.len() > 5 {
            let overflow = self.compact_state.recent_files.len() - 5;
            self.compact_state.recent_files.drain(0..overflow);
        }
    }
}

fn collect_tool_result_positions(messages: &[ChatCompletionRequestMessage]) -> Vec<usize> {
    let mut positions = Vec::new();

    for (idx, msg) in messages.iter().enumerate() {
        if matches!(msg, ChatCompletionRequestMessage::Tool(_)) {
            positions.push(idx);
        }
    }

    positions
}