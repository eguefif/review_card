#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseContent {
    pub text: String,
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
pub struct Response {
    id: String,
    pub content: Vec<ResponseContent>,
    model: String,
    role: String,
    stop_reason: Option<String>,
    stop_sequence: Option<String>,
    usage: Usage,
    container: Option<Container>,
}
