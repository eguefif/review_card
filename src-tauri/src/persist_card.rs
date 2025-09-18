use crate::ai_app::questions_parsing::Question};

pub struct Card {
    content: String,
    questions: Vec<Question>

}
#[tauri::command]
pub async fn save_card(card: Card) {
}
