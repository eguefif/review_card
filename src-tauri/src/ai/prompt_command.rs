#![allow(dead_code)]
#![allow(unused)]
use dotenv::dotenv;
use reqwest;

use crate::ai::anthropic::{Anthropic, Role};

use crate::card::question::{mock_questions, ParsedQuestion, Question};

const MOCKED_CARD: &str = r#"
# Css layout algorithm
There are five algorithms: 
* block
* inline
* flex
* css grid
* floating

They all serves different purpose depending on the goal.
"#;

#[tauri::command]
pub async fn prompt(topic: String) -> (String, Vec<Question>) {
    dotenv().ok();
    //let mut client = Anthropic::new("claude-3-5-haiku-latest".into(), 1000);
    //let card = create_review_card(&mut client, &topic).await;

    //let questions = get_questions(&mut client, &card).await;

    let questions = mock_questions();
    let card = MOCKED_CARD.to_string();

    (card, questions)
}

fn expand_prompt(topic: &str) -> String {
    format!(
        "Can you create a review card for the following topic?\n\n{}",
        topic
    )
}

async fn get_questions(client: &mut Anthropic, card: &str) -> Vec<Question> {
    let example = r#"
    {
        "questions": [
            {
                "question": "What is France's capital city",
                "options": ["Paris", "Lyon", "Marseille"]
            },
            {
                "question": "What is the baby of the bear",
                "options": ["Kitty", "Cub", "Lion"]
            }
        ]
    }
    "#;

    let format = r#"
    {
        "questions": [
            { 
                "question": "string", 
                "options": ["string"]
            }
        ]
    }
    "#;

    let prompt = format!("Based on the following review card, can you generate a few questions and answers for a MCQ quiz? You need to return a json.\n{}, Here is the format of the json {}Here is an example of the json you need to out put: {}.", card, format, example);

    client.push_message(Role::User, prompt);
    client.push_message(Role::Assistant, "The json is: {".into());
    println!("{:?}", client.messages);

    match client.send_message().await {
        Ok(response) => handle_response(response),
        Err(_) => panic!(),
    }
}

fn handle_response(response: String) -> Vec<Question> {
    let json = format!("{{{}", response);
    let parsed_questions: Vec<ParsedQuestion> = serde_json::from_str(&json).expect("not formated");
    let mut questions: Vec<Question> = vec![];

    for (i, parsed_question) in parsed_questions.into_iter().enumerate() {
        let question = Question::new(parsed_question, i);
        questions.push(question);
    }
    questions
}
