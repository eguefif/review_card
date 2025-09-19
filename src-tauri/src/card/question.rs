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
    pub id: usize,
    pub answer: String,
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

