use std::fmt;

/// AI-related errors that can occur during review card and question generation.
///
/// # Error Types
///
/// * `AnthropicMessageFailed` - When either API call to Anthropic fails due to network
///   issues, authentication problems, or API service unavailability
/// * `QuestionParsingFailed` - When the JSON response from the question generation API
///   cannot be parsed into valid `Question` objects
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum AIError {
    AnthropicMessageFailed,
    QuestionParsingFailed,
}

impl fmt::Display for AIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AIError::AnthropicMessageFailed => write!(f, "Error: Anthropic message push failed."),
            AIError::QuestionParsingFailed => {
                write!(f, "Error: Impossible to parse json question from LLM.")
            }
        }
    }
}
