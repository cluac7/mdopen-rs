// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

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
            Some(&mut "##") => {
                line_modifier = "h2".into();
                if let Some(hash) = words.peek_mut() {
                    *hash = &"";
                }
            }
            Some(&mut "###") => {
                line_modifier = "h3".into();
                if let Some(hash) = words.peek_mut() {
                    *hash = &"";
                }
            }
            Some(&mut ">") => {
                line_modifier = "blockquote".into();
                if let Some(quote) = words.peek_mut() {
                    *quote = &"";
                }
            }
            Some(&mut "-") => {
                line_modifier = "ul".into();
            }
            Some(&mut &_) => (),
            None => (),
        }

        // parse inline commands
        let mut parsed_words = String::new();
        for word in words {
            let parsed_word = String::new();
            if word.len() <= 2 {
                parsed_words += &word;
                parsed_words += " ";
                continue;
            }
            if word == "---" {
                parsed_words += "<hr />";
                continue;
            }
            let mut word_modifier = String::new();
            if &word[0..=1] == "**" || &word[0..=1] == "__" {
                word_modifier = "strong".into();
            } else {
                if word.starts_with("*") || word.starts_with("_") {
                    word_modifier = "i".into();
                } else if word.starts_with("`") {
                    word_modifier = "code".into();
                }
                }
            }
            let mut parsed_word: String = word.to_string();
            if !word_modifier.is_empty() && word_modifier != "" {
                parsed_word = format!("<{}>{}</{}>", &word_modifier, word, &word_modifier);
            }
            parsed_words += &parsed_word;
            parsed_words += " ";
        }
        let parsed_line = format!("<{}>{}</{}>", &line_modifier, parsed_words, &line_modifier);
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
                        println!("{:?}", sourcepath_argdata);
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
