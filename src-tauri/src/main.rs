// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_dots])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Function to generate 100 dots (temporary)
#[tauri::command]
async fn get_dots() -> Result<Vec<(f64, f64)>, String> {
    let mut rng = rand::thread_rng();
    Ok(
        (0..100).map(|_| {
            (rng.gen::<f64>() * 100.0, rng.gen::<f64>() * 100.0)
        }).collect()
    )

}