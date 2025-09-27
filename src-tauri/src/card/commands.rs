use crate::app_data::AppData;
use crate::card::card_content::Card;
use crate::card::question::Question;
use std::sync::Mutex;
use tauri::{AppHandle, State};
use tauri_plugin_store::StoreExt;

use crate::STORE_NAME;

#[tauri::command]
pub async fn save_card(
    app: AppHandle,
    state: State<'_, Mutex<AppData>>,
    content: String,
    questions: Vec<Question>,
) -> Result<(), String> {
    println!("In save_card");
    if let Ok(mut state) = state.lock() {
        let store = app
            .store(STORE_NAME)
            .map_err(|e| format!("Failed to get store: {}", e))?;
        let next_card_id = state.get_next_card_id();
        println!("Saving card with id: {}", next_card_id);

        let card = Card::new(next_card_id, content, questions);

        store.set(
            next_card_id.to_string(),
            serde_json::to_value(card).map_err(|e| format!("Failed to serialize card: {}", e))?,
        );

        if let Ok(()) = store.save() {
            println!("\x1b[32mSaved success - card id: {}\x1b[0m", next_card_id);
        } else {
            println!("\x1b[31mFailed to save store to disk\x1b[0m");
            return Err("Failed to save store to disk".to_string());
        }
    } else {
        return Err("Could not lock app state".to_string());
    }
    Ok(())
}

#[tauri::command]
pub async fn get_all_cards(
    app: AppHandle,
    offset: usize,
    limit: usize,
) -> Result<Vec<Card>, String> {
    let mut all_cards: Vec<Card> = vec![];

    let store = app
        .store(STORE_NAME)
        .map_err(|e| format!("Failed to get store: {}", e))?;

    for value_card in store.values().iter() {
        if let Ok(card) = serde_json::from_value::<Card>(value_card.clone()) {
            all_cards.push(card);
        } else {
            println!("Error: could not deserialize card {:?}", value_card);
        }
    }

    println!("Cards: {:?}", all_cards);
    Ok(all_cards)
}
