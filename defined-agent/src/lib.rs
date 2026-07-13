use async_openai::types::chat::{ChatCompletionRequestAssistantMessageContent, ChatCompletionRequestAssistantMessageContentPart};

pub mod llm;
pub mod structure;
pub mod tools;

pub fn extract_text(content: &ChatCompletionRequestAssistantMessageContent) -> String {
    match content {
        ChatCompletionRequestAssistantMessageContent::Text(c) => {c.clone()},
        ChatCompletionRequestAssistantMessageContent::Array(arr) => {
            arr.iter().filter_map(|a| {
                if let  ChatCompletionRequestAssistantMessageContentPart::Text(c) = a {
                    Some(c.text.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
        },
    } 
}