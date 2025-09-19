#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Question {
    id: usize,
    question: String,
    options: Vec<Option>,
}

impl Question {
    pub fn new(parsed_question: ParsedQuestion, id: usize) -> Self {
        Self {
            question: parsed_question.question.clone(),
            options: parsed_question.options.clone(),
            id: id,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Option {
    id: usize,
    answer: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ParsedQuestion {
    pub question: String,
    pub options: Vec<Option>,
}

impl ParsedQuestion {
    pub fn new(question: String, options: Vec<Option>) -> Self {
        return Self { question, options };
    }
}

pub fn mock_questions() -> Vec<Question> {
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
