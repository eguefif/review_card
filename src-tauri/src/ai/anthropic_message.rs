use super::anthropic::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent {
    role: Role,
    content: String,
}

impl MessageContent {
    pub fn new(role: Role, content: String) -> Self {
        Self { role, content }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    model: String,
    max_tokens: u64,
    messages: Vec<MessageContent>,
}

impl Message {
    pub fn new(model: String, max_tokens: u64, messages: Vec<MessageContent>) -> Self {
        Self {
            model,
            max_tokens,
            messages,
        }
    }
}
