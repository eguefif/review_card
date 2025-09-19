use std::fmt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum AIError {
    AnthropicMessageFailed,
}

impl fmt::Display for AIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AIError::AnthropicMessageFailed => write!(f, "Error: Anthropic message push failed."),
        }
    }
}
