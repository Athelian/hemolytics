// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import xml file
use quickxml_to_serde::xml_string_to_json;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

use quickxml_to_serde::Config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_xml() -> Result<Value, String> {
    // Open the XML file, properly handling any errors
    let mut xml_file = File::open("tests/assets/bloods.xml")
        .map_err(|e| format!("Failed to open XML file: {}", e))?;

    let mut xml_contents = String::new();
    // Read the file contents, properly handling any errors
    xml_file
        .read_to_string(&mut xml_contents)
        .map_err(|e| format!("Failed to read XML file: {}", e))?;

    // Convert XML string to JSON, properly handling any errors
    xml_string_to_json(xml_contents, &Config::new_with_defaults())
        .map_err(|e| format!("Failed to convert XML to JSON: {}", e))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_xml])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
