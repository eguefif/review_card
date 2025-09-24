#![allow(dead_code)]
#![allow(unused)]
use reqwest;

use crate::ai::ai_error::AIError;
use crate::ai::mock_data::{MOCKED_CARD, mocked_questions};
use crate::ai::anthropic::{Anthropic, Role};
use crate::ai::prompts::{build_review_card_prompt, build_mcq_prompt};
use crate::card::question::{ParsedQuestion, Question};

/// Generates an AI-powered review card and multiple choice questions for a given topic.
///
/// # Arguments
///
/// * `topic` - A string containing the subject matter for which to generate the review card
///   and questions. This can be any educational topic, concept, or subject area.
///
/// # Returns
///
/// Returns a `Result` containing:
/// * `Ok((String, Vec<Question>))` - A tuple where:
///   - The first element is the generated review card as a formatted text string
///   - The second element is a vector of `Question` objects containing multiple choice questions
/// * `Err(AIError)` - An error if any step in the process fails
///
/// # Side Effects
///
/// This function makes two sequential HTTP requests to the Anthropic Claude API.
///
/// for detailed error descriptions.
/// * Panics if `ANTHROPIC_API_KEY` environment variable is not set
///
/// # Example
///
/// ```rust
/// let result = prompt("photosynthesis".to_string()).await;
/// match result {
///     Ok((review_card, questions)) => {
///         println!("Review card: {}", review_card);
///         println!("Generated {} questions", questions.len());
///     }
///     Err(e) => eprintln!("Failed to generate content: {:?}", e),
/// }
/// ```
#[tauri::command]
pub async fn prompt(topic: String) -> Result<(String, Vec<Question>), AIError> {
    std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");
    //let mut client = Anthropic::new("claude-3-5-haiku-latest".into(), 1000);
    //let card = create_review_card(&mut client, &topic).await?;

    //let questions = get_questions(&mut client, &card).await?;

    let questions = mocked_questions();
    let card = MOCKED_CARD.to_string();

    Ok((card.to_string(), questions))
}

async fn create_review_card(client: &mut Anthropic, topic: &str) -> Result<String, AIError> {
    let prompt = build_review_card_prompt(topic);
    client.push_message(Role::User, prompt);
    if let Ok(response) = client.send_message().await {
        Ok(response)
    } else {
        Err(AIError::AnthropicMessageFailed)
    }
}


async fn get_questions(client: &mut Anthropic, card: &str) -> Result<Vec<Question>, AIError> {
    let prompt = build_mcq_prompt(card);

    client.push_message(Role::User, prompt);
    client.push_message(Role::Assistant, "The json is: [".into());

    match client.send_message().await {
        Ok(response) => handle_response(response),
        Err(_) => Err(AIError::AnthropicMessageFailed),
    }
}

fn handle_response(json: String) -> Result<Vec<Question>, AIError> {
    // The Anthropic API will truncate the first [, we need
    // to add it manually for Serde to deserialize it.
    // See Anthropic documentation for JSON
    let json = format!("[{json}");

    if let Ok(parsed_questions) = serde_json::from_str::<Vec<ParsedQuestion>>(&json) {
        return Ok(create_questions_from_parsed_data(parsed_questions))
    }
    println!("\x1b[31mError: Could not parse JSON response\x1b[0m");
    Err(AIError::QuestionParsingFailed)
}

/// Converts raw parsed questions into domain objects with identifiers.
///
/// This function transforms `ParsedQuestion` objects (deserialized from JSON) into
/// `Question` domain objects, assigning each question an ID based
/// on its position in the input vector.
///
/// # Arguments
///
/// * `parsed_questions` - A vector of `ParsedQuestion` objects containing the raw
///   question data and answer options from the AI response
///
/// # Returns
///
/// A vector of `Question` objects, each assigned a unique ID starting from 0
///
/// # Why This Function Exists
///
/// The AI API returns unstructured question data without identifiers. This function
/// adds the necessary IDs that the application requires for question tracking,
/// user interaction, and state management throughout the quiz interface.
fn create_questions_from_parsed_data(parsed_questions: Vec<ParsedQuestion>) -> Vec<Question> {
    let mut questions: Vec<Question> = vec![];
    for (i, parsed_question) in parsed_questions.into_iter().enumerate() {
        let question = Question::new(parsed_question, i);
        questions.push(question);
    }
    questions
}
