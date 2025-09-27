#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Question {
    id: usize,
    question: String,
    options: Vec<Option>,
}

impl Question {
    pub fn new(parsed_question: ParsedQuestion, id: usize) -> Self {
        let mut options: Vec<Option> = vec![];
        for (i, string_option) in parsed_question.options.into_iter().enumerate() {
            let option = Option {
                id: i,
                answer: string_option,
            };
            options.push(option);
        }
        Self {
            question: parsed_question.question.clone(),
            options: options,
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
    pub options: Vec<String>,
}

impl ParsedQuestion {
    pub fn new(question: String, options: Vec<String>) -> Self {
        return Self { question, options };
    }
}
