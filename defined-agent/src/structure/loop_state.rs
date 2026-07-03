use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::ChatCompletionRequestMessage,
};

pub struct LoopState {
    pub client: Client<OpenAIConfig>,
    pub context: Vec<ChatCompletionRequestMessage>,
    pub turn_count: usize,
    pub transition_reason: Option<String>,
}

impl LoopState {
    pub fn new(client: Client<OpenAIConfig>) -> Self {
        Self {
            client,
            context: Vec::new(),
            turn_count: 0,
            transition_reason: None,
        }
    }
}
