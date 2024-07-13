// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::{prelude::*, BufReader};

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

#[tauri::command]
fn parse_file(path: String) -> String {
    let file = File::open(&path).expect("Couldn't open file");
    let mut parsed_string = String::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap_or("".into());
        let mut words = line.split_whitespace().peekable();

        // parse line modifiers
        let mut line_modifier = String::from("div");
        match words.peek_mut() {
            Some(&mut "#") => {
                line_modifier = "h1".into();
                if let Some(hash) = words.peek_mut() {
                    *hash = &"";
                }
            }
            Some(&mut "##") => line_modifier = "h2".into(),
            Some(&mut "###") => line_modifier = "h3".into(),
            Some(&mut ">") => line_modifier = "blockquote".into(),
            Some(&mut &_) => (),
            None => (),
        }

        // parse inline commands
        for word in &mut words {
            if word.len() <= 2 {
                continue;
            }
            let mut word_modifier = String::new();
            match &word[0..=1] {
                "**" => word_modifier = "strong".into(),
                "__" => word_modifier = "strong".into(),
                "*" => word_modifier = "i".into(),
                "_" => word_modifier = "i".into(),
                &_ => word_modifier = "".into(),
            }
            if !word_modifier.is_empty() {
                *word = &format!("<{}>{}</{}>", &word_modifier, word, &word_modifier);
            }
        }
        let parsed_line = format!("<{}>{}</{}>", &line_modifier, line, &line_modifier);
        parsed_string += &parsed_line;
    }
    parsed_string
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
        .invoke_handler(tauri::generate_handler![open_file, parse_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
