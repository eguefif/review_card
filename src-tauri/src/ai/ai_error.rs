use std::fmt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum AIError {
    AnthropicMessageFailed,
    QuestionParsingFailed,
}

impl fmt::Display for AIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AIError::AnthropicMessageFailed => write!(f, "Error: Anthropic message push failed."),
            AIError::QuestionParsingFailed => write!(f, "Error: Impossible to parse json question from LLM."),
        }
    }
}
