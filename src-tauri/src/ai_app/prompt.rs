use reqwest;
use dotenv::dotenv;

const KEY: &str = "";

#[derive(serde::Serialize, serde::Deserialize)]
struct MessageContent {
    #[serde(rename = "role")]
    user: String,
    content: String,
}

impl MessageContent {
    pub fn new(user: String, content: String) -> Self {
        Self {
            user,
            content
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
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
            messages
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
    web_search_requets: u64
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
    container: Option<Container>
}

#[tauri::command]
pub async fn prompt_ai(prompt: String) -> String {
    let client = reqwest::Client::new();
    let message_content = MessageContent::new("user".into(), prompt);
    let content = Message::new("claude-3-5-haiku-latest".into(), 1000, vec![message_content]);
    let res = client.post("https://api.anthropic.com/v1/messages")
        .header("Content-Type", "application/json")
        .header("anthropic-version", "2023-06-01")
        .header("x-api-key", KEY)
        .header("Content-Type", "application/json")
        .json(&content)
        .send()
        .await;

    let text = match res {
        Ok(response) => {&response.json::<Response>().await.unwrap().content[0].text}
        Err(e) => {
                println!("Error: {}", e);
                return "Error".to_string();
        }
    };

    return format!("{}", text.clone()); 
}
