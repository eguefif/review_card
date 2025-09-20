use crate::app_data::AppData;
use crate::card::card_content::Card;
use std::sync::Mutex;
use tauri::{AppHandle, State};
use tauri_plugin_store::StoreExt;

use crate::STORE_NAME;

#[tauri::command]
pub async fn save_card(
    app: AppHandle,
    state: State<'_, Mutex<AppData>>,
    card: Card,
) -> Result<(), String> {
    println!("In save_card");
    if let Ok(mut state) = state.lock() {
        let store = app
            .store(STORE_NAME)
            .map_err(|e| format!("Failed to get store: {}", e))?;
        let next_card_id = state.get_next_card_id();
        println!("Saving card with id: {}", next_card_id);

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
