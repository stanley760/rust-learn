use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::llm::content::ContentItem;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub execution_id: String,
    pub timestamp: i64,
    pub author: String,
    pub content: Vec<ContentItem>
}

impl Event {
    pub fn new(execution_id: impl Into<String>,
        author: impl Into<String>,
        content: Vec<ContentItem>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            execution_id: execution_id.into(),
            timestamp: chrono::Utc::now().timestamp(),
            author: author.into(),
            content,
        }
    }
}