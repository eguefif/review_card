mod ai_app;
mod app_state;
mod persist_card;

use tauri::Manager;
use tauri_plugin_store::StoreExt;
use crate::app_state::AppData;
use std::sync::Mutex;

const STORE_NAME: &str = "cards.json";

fn get_next_card_id(keys: Vec<String>) -> u64 {
    let mut highest_id = 0;

    for key in keys.iter() {
        if let Ok(num) = key.parse::<u64>() {
            if num > highest_id {
                highest_id = num;
            }
        }
    }
    let next_id = highest_id + 1;
    println!("Init next_id: {} (highest existing: {})", next_id, highest_id);
    next_id
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = app.store(STORE_NAME)?;
            let keys = store.keys();
            let next_card_id = get_next_card_id(keys);
            app.manage(Mutex::new(AppData {
                next_card_id,
            }));
            Ok(())
         })
        .invoke_handler(tauri::generate_handler![
            ai_app::prompt::prompt_ai,
            persist_card::save_card,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
