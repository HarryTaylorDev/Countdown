// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
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
    .invoke_handler(tauri::generate_handler![log_to_console, append_to_file, load_data, save_to_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  
}

use tauri::{command};

#[command]
fn log_to_console(message: String) {
    println!("{}", message);
}

// #[derive(Serialize, Deserialize)]
// struct CountDown {
//   name:String,
//   emoji:String,
//   date:i64,
//   colour:String
// }

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

use std::fs::OpenOptions;
use std::io::Write;

#[command]
async fn append_to_file(string:String){
    let docs_path = get_documents_path();    
    let file_path = format!("{}\\CountDown\\CountDownData.txt", docs_path.unwrap());
    println!("{}",file_path.clone());

    // Open a file with append option
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("cannot open file");

      if let Err(e) = writeln!(data_file, "{}",string) {
          eprintln!("Couldn't write to file: {}", e);
      }
}


#[command]
async fn save_to_file(jsonstrings:Vec<String>){
    let docs_path = get_documents_path();    
    let file_path = format!("{}\\CountDown\\CountDownData.txt", docs_path.unwrap());
    fs::write(file_path, jsonstrings.join("\n")).expect("error wrting file");
}

