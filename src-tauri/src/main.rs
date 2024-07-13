// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::prelude::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[tauri::command]
fn open_file(path: String) -> String {
    let mut file = File::open(&path).expect("Couldn't open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Couldn't read file");
    s
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            match app.get_cli_matches() {
                Ok(matches) => {
                    if let Some(sourcepath_argdata) = matches.args.get("sourcepath") {
                        print!("{:?}", sourcepath_argdata);
                    } else {
                        panic!("Executable arguments not found");
                    }
                }
                Err(_) => {
                    panic!("Unable to access executable arguments");
                }
            };
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
