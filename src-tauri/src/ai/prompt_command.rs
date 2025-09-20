#![allow(dead_code)]
#![allow(unused)]
use reqwest;

use crate::ai::ai_error::AIError;
use crate::ai::mock_data::{MOCKED_CARD, mocked_questions};
use crate::ai::anthropic::{Anthropic, Role};
use crate::card::question::{ParsedQuestion, Question};

#[tauri::command]
pub async fn prompt(topic: String) -> Result<(String, Vec<Question>), AIError> {
    std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");
    let mut client = Anthropic::new("claude-3-5-haiku-latest".into(), 1000);
    let card = create_review_card(&mut client, &topic).await?;
    println!("Card:\n\n{}", card);
    println!();
    println!();
    println!();

    let questions = get_questions(&mut client, &card).await?;

    //let questions = mocked_questions();
    //let card = MOCKED_CARD.to_string();

    Ok((card.to_string(), questions))
}

async fn create_review_card(client: &mut Anthropic, topic: &str) -> Result<String, AIError> {
    let prompt = create_prompt(topic);
    println!("Prompt\n\n{}", prompt);
    println!();
    println!();
    println!();
    client.push_message(Role::User, prompt);
    if let Ok(response) = client.send_message().await {
        Ok(response)
    } else {
        Err(AIError::AnthropicMessageFailed)
    }
}

fn create_prompt(topic: &str) -> String {
    format!(
        "I want you to create a review card. This review card should only be text and separate in meaningfull sections. Can you create a review card for the following topic?\n\n{}",
        topic
    )
}

async fn get_questions(client: &mut Anthropic, card: &str) -> Result<Vec<Question>, AIError> {
    let example = r#"
        [
            {
                "question": "What is France's capital city",
                "options": ["Paris", "Lyon", "Marseille"]
            },
            {
                "question": "What is the baby of the bear",
                "options": ["Kitty", "Cub", "Lion"]
            }
        ]
    "#;

    let format = r#"
        [
            { 
                "question": "string", 
                "options": ["string"]
            }
        ]
    "#;

    let prompt = format!("Based on the following review card, can you generate a few questions and answers for a MCQ quiz? You MUST return ONLY a JSON array with no wrapper object.\n\nReview card: {}\n\nRequired JSON format: {}\n\nExample output: {}\n\nIMPORTANT: Return ONLY the JSON array, not wrapped in any object like {{\"quizQuestions\": ...}}.", card, format, example);

    client.push_message(Role::User, prompt);
    client.push_message(Role::Assistant, "The json is: [".into());

    match client.send_message().await {
        Ok(response) => Ok(handle_response(response)),
        Err(_) => Err(AIError::AnthropicMessageFailed),
    }
}

fn handle_response(json: String) -> Vec<Question> {
    // The Anthropic API will truncate the first [, we need
    // to add it manually. See Anthropic documentation for JSON
    let json = format!("[{json}");

    println!("Questions:\n\n {}", json);
    if let Ok(parsed_questions) = serde_json::from_str::<Vec<ParsedQuestion>>(&json) {
        return create_questions_from_parsed(parsed_questions);
    }
    println!("\x1b[31mError: Could not parse JSON response\x1b[0m");
}

fn create_questions_from_parsed(parsed_questions: Vec<ParsedQuestion>) -> Vec<Question> {
    let mut questions: Vec<Question> = vec![];
    for (i, parsed_question) in parsed_questions.into_iter().enumerate() {
        let question = Question::new(parsed_question, i);
        questions.push(question);
    }
    questions
}
