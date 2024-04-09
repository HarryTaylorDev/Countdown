// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Serialize, Deserialize};
use std::fs;
use tauri::api::path::document_dir;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
  /////////////////////////////////////////////////////////////////////////
  // check if folder and file exist, make them if not 
  match ensure_folder_exists() {
    Ok(()) => println!("Folder created or already exists"),
    Err(error) => eprintln!("Error creating folder: {}", error),
  }
  /////////////////////////////////////////////////////////////////////////

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![log_to_console, save_count_down,load_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  
}

use tauri::{command};

#[command]
fn save_count_down(data: serde_json::Value) {
  if let Some(object) = data.as_object() {
    // Iterate through key-value pairs
    for (key, value) in object {
        println!("{}: {}",key, value);
    }
  }

}

#[command]
fn log_to_console(message: String) {
    println!("{}", message);
}

#[derive(Serialize, Deserialize)]
struct CountDown {
  name:String,
  emoji:String,
  date:i64,
  colour:String
}

#[tauri::command]
async fn load_data() -> Vec<String>{
  let result = read_file_from_documents();
  result.unwrap()
}

fn get_documents_path() -> Result<String, String> {
    if let Some(documents_path) = document_dir() {
        documents_path.into_os_string()
                      .into_string()
                      .map_err(|_| "Error converting path to string".to_string()) 
    } else {
        Err("Could not determine documents directory path".to_string())
    }
}

fn ensure_folder_exists() -> Result<(), String> {
  let docs_path = get_documents_path()?;
  let folder_path = format!("{}\\CountDown", docs_path);

  fs::create_dir_all(folder_path.clone()).map_err(|err| format!("Error creating folder: {}", err))?;



  let file_path = format!("{}\\CountDownData.txt", folder_path);

  let path = Path::new(&file_path);
  if !path.exists() {
      fs::write(&file_path, "") // Create an empty file
          .map_err(|err| format!("Error creating file: {}", err))?;
  }

  Ok(())
}

fn read_lines(file_path: &str) -> Result<Vec<String>, String> {
  let file = File::open(file_path)
                 .map_err(|err| format!("Error opening file: {}", err))?;
  let reader = BufReader::new(file);

  let mut lines = Vec::new();
  for line in reader.lines() {
      lines.push(line.map_err(|err| format!("Error reading line: {}", err))?);
  }

  Ok(lines)
}

fn read_file_from_documents() -> Result<Vec<String>, String> {
  let docs_path = get_documents_path()?;
  let file_path = format!("{}\\CountDown\\CountDownData.txt", docs_path);

  let data = read_lines(&file_path)?; // Call your read_lines function  
  Ok(data) // Return the result 
}

fn save_file_to_documents() -> Result<(), String> {
    let docs_path = get_documents_path()?;
    let file_path = format!("{}\\CountDown\\CountDownData.txt", docs_path);

    fs::write(file_path, "")
        .map_err(|err| format!("Error creating file: {}", err))?;

    Ok(())
}

