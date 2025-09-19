use crate::card::question::{Question, Option, ParsedQuestion};

pub const MOCKED_CARD: &str = r#"
# Css layout algorithm
There are five algorithms: 
* block
* inline
* flex
* css grid
* floating

They all serves different purpose depending on the goal.
"#;


pub fn mocked_questions() -> Vec<Question> {
    let mocked_questions: Vec<Question> = vec![
        Question::new(
            ParsedQuestion::new(
                "What's France's capital City?".to_string(),
                vec![
                    Option {
                        id: 0,
                        answer: "Paris".to_string(),
                    },
                    Option {
                        id: 1,
                        answer: "Berlin".to_string(),
                    },
                    Option {
                        id: 2,
                        answer: "Madrid".to_string(),
                    },
                ],
            ),
            1,
        ),
        Question::new(
            ParsedQuestion::new(
                "What is the weather".to_string(),
                vec![
                    Option {
                        id: 0,
                        answer: "Cloudy".to_string(),
                    },
                    Option {
                        id: 1,
                        answer: "Rainny".to_string(),
                    },
                    Option {
                        id: 2,
                        answer: "Sunny".to_string(),
                    },
                ],
            ),
            2,
        ),
    ];
    mocked_questions
}
