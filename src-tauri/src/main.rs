// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::image::{load_image, load_image_full};
use commands::ocr::process_region;
use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Mutex::new(HashMap::<String, String>::new()))
        .invoke_handler(tauri::generate_handler![
            load_image,
            load_image_full,
            process_region,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
