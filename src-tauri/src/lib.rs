mod ai_app;
use tauri::{Builder, Manager};
use tauri_plugin_store::StoreExt;

fn get_next_card_id(keys: Vec<String>) -> u64 {
    let mut next_id = 0;

    for key in keys.iter() {
        let num: u64 = key.parse::<u64>().expect("Error while parsing keys from store: all keys should be u64 string");
        if num > next_id {
            next_id = num;
        }
    }
    next_id
}

struct AppData {
    next_card_id: u64,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let store = app.store("cards.json")?;
            let keys = store.keys();
            let next_card_id = get_next_card_id(keys);
            println!("Next id: {}", next_card_id);
            app.manage(AppData {
                next_card_id
            });
            Ok(())
         })
        .invoke_handler(tauri::generate_handler![ai_app::prompt::prompt_ai])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
