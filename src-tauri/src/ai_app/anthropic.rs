#![allow(dead_code)]
#![allow(unused)]
use serde::{Deserialize, Serialize};

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
struct Message {
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

#[derive(serde::Serialize, serde::Deserialize)]
struct ResponseContent {
    text: String,
    #[serde(rename = "type")]
    content_type: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ServerToolUse {
    web_search_requets: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CacheCreation {
    ephemeral_1h_input_tokens: u64,
    ephemeral_5m_input_tokens: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Usage {
    cache_creation: Option<CacheCreation>,
    cache_creation_input_tokens: Option<u64>,
    cache_read_input_tokens: Option<u64>,
    input_tokens: u64,
    output_tokens: u64,
    server_tool_use: Option<ServerToolUse>,
    service_tier: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Container {
    expires_at: String,
    id: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Response {
    id: String,
    content: Vec<ResponseContent>,
    model: String,
    role: String,
    stop_reason: Option<String>,
    stop_sequence: Option<String>,
    usage: Usage,
    container: Option<Container>,
}
