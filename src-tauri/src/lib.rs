mod ai;
mod app_data;
mod card;
mod data;

use crate::data::migrations::get_migrations;
use crate::app_data::AppData;
use dotenv::dotenv;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
use tauri_plugin_sql::Builder;

const STORE_NAME: &str = "cards.json";
// Dependencies: This db name is also hard-written in tauri.conf.json
const DB_NAME: &str = "sqlite:cardreview.db"; 

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
    println!(
        "Init next_id: {} (highest existing: {})",
        next_id, highest_id
    );
    next_id
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = get_migrations();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
            .add_migrations(DB_NAME, migrations)
            .build()
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            dotenv().ok();
            let store = app.store(STORE_NAME)?;
            let keys = store.keys();
            let next_card_id = get_next_card_id(keys);
            app.manage(Mutex::new(AppData { next_card_id }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ai::prompt_command::prompt,
            card::commands::save_card,
            card::commands::get_all_cards,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
