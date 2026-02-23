// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
mod commands;

use commands::image::{load_image, load_image_full};
use commands::ocr::{build_engine, process_region, OcrEngineState};
use std::collections::HashMap;

fn main() {
    let engine = build_engine().expect("Failed to load OCR engine");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(HashMap::<String, String>::new()))
        .manage(OcrEngineState(Arc::new(Mutex::new(engine))))
        .invoke_handler(tauri::generate_handler![
            load_image,
            load_image_full,
            process_region,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
