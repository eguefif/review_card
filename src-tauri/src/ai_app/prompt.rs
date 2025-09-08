use reqwest;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};

use crate::ai_app::anthropic::{Anthropic, Role};

const mocked_card: &str = r#"
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
pub async fn prompt_ai(topic: String) -> (String, Vec<Question>) {
    dotenv().ok();
    //let mut client = Anthropic::new("claude-3-5-haiku-latest".into(), 1000);
    //let card = create_review_card(&mut client, &topic).await;

    //let questions = get_questions(&mut client, &card).await;



    let mocked_questions: Vec<Question> = vec![
        Question::new("What's France's capital City?".into(), 
            vec!["Paris".into(), "Berlin".into(), "Madrid".into()]),
        Question::new("What's France's capital City?".into(), 
            vec!["Paris".into(), "Berlin".into(), "Madrid".into()]),
    ];

    let questions = mocked_questions;
    let card = mocked_card.to_string();

    (card, questions)
}

fn expand_prompt(topic: &str) -> String {
    format!("Can you create a review card for the following topic?\n\n{}", topic)
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

    let prompt = format!("Based on the following review card, can genereate a few questions and answers for a MCQ quiz? You need to return a json.\n{}, Here is the format of the json {}Here is an example of the json you need to out put: {}.", card, format, example);


    client.push_message(Role::User, prompt);
    client.push_message(Role::Assistant, "The json is: {".into());
    println!("{:?}", client.messages);

    match client.send_message().await {
        Ok(response) => handle_response(response),
        Err(_) => panic!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question{
    question: String,
    options: Vec::<String>,
}

impl Question {
    pub fn new(question: String, options: Vec::<String>) -> Self {
        return Self{
            question,
            options
        }
    }
}

fn handle_response(response: String) -> Vec<Question>{
    let json = format!("{{{}", response);
    let questions: Vec<Question> = serde_json::from_str(&json).expect("not formated");

    questions
}
