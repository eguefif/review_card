#![allow(dead_code)]
#![allow(unused)]
use serde::{Deserialize, Serialize};
use crate::ai::anthropic_message::{MessageContent, Message};
use crate::ai::anthropic_response::Response;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}

pub struct Anthropic {
    api_key: String,
    model: String,
    max_tokens: u64,
    client: reqwest::Client,
    pub messages: Vec<MessageContent>,
}

impl Anthropic {
    pub fn new(model: String, max_tokens: u64) -> Self {
        let api_key = std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");
        let client = reqwest::Client::new();
        Self {
            api_key,
            model,
            max_tokens,
            client,
            messages: vec![],
        }
    }

    pub fn push_message(&mut self, role: Role, content: String) {
        let message = MessageContent::new(role, content.clone());
        self.messages.push(message)
    }

    pub async fn send_message(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let message = Message::new(self.model.clone(), self.max_tokens, self.messages.clone());

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&message)
            .send()
            .await?;

        self.messages.clear();
        Ok(response.json::<Response>().await.unwrap().content[0]
            .text
            .clone())
    }
}

