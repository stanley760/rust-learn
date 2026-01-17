// Module declarations
pub mod commands;
pub mod error;
pub mod process;
pub mod types;

use commands::{
    get_processes, get_processes_paginated, kill_process, search_by_port, search_by_port_paginated,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_processes,
            get_processes_paginated,
            search_by_port,
            search_by_port_paginated,
            kill_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
