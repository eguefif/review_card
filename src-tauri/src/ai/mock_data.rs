use crate::card::question::{ParsedQuestion, Question};

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
                    "Paris".to_string(),
                    "Berlin".to_string(),
                    "Madrid".to_string(),
                ],
            ),
            1,
        ),
        Question::new(
            ParsedQuestion::new(
                "What is the weather".to_string(),
                vec![
                    "Cloudy".to_string(),
                    "Rainny".to_string(),
                    "Sunny".to_string(),
                ],
            ),
            2,
        ),
    ];
    mocked_questions
}
