// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![log_to_console])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


use tauri::{command, Runtime};

#[command]
fn log_to_console(message: String) {
    println!("{}", message);
}