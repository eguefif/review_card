use crate::card::question::Question;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Card {
    id: u64,
    content: String,
    questions: Vec<Question>,
}

impl Card {
    pub fn new(id: u64, content: String, questions: Vec<Question>) -> Self {
        Self {
            id,
            content,
            questions,
        }
    }
}
