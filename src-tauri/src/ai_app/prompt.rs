#[tauri::command]
pub fn prompt_ai(prompt: &str) -> String {
    println!("Prompt: {}", prompt);
    format!("{}", prompt)
}
