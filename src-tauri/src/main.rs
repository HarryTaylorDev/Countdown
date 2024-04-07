// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  // tauri::Builder::default()
  //   .invoke_handler(tauri::generate_handler![save_count_down])
  //   .run(tauri::generate_context!())
  //   .expect("error while running tauri application");

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![log_to_console, save_count_down])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

use tauri::{command};

#[command]
fn save_count_down(data: serde_json::Value) {
  // println!("save function");
  // println!("{}",data);
  // let name = data["name"].as_str().unwrap_or("default");
  // println!("{}",name);


  if let Some(object) = data.as_object() {
    // Iterate through key-value pairs
    for (key, value) in object {
        println!("Key: {}", value);
    }
  }

}

#[command]
fn log_to_console(message: String) {
    println!("{}", message);
}

// struct countDown{
//   name: String,
//   emoji:String,
//   date:i64,
//   colour:String
// }


