use crate::card::question::Question;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Card {
    content: String,
    questions: Vec<Question>,
}
