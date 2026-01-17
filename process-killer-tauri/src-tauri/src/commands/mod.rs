use crate::process::manager::ProcessManager;
use crate::types::{PageRequest, PageResponse, ProcessRecord};

// Tauri command handlers will be implemented here

#[tauri::command]
pub async fn get_processes() -> Result<Vec<ProcessRecord>, String> {
    let mut manager = ProcessManager::new();
    manager.get_all_processes().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_processes_paginated(
    page: usize,
    page_size: usize,
) -> Result<PageResponse<ProcessRecord>, String> {
    let mut manager = ProcessManager::new();
    let request = PageRequest { page, page_size };
    manager
        .get_all_processes_paginated(&request)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_by_port(port: String) -> Result<Vec<ProcessRecord>, String> {
    let mut manager = ProcessManager::new();
    manager.search_by_port(&port).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_by_port_paginated(
    port: String,
    page: usize,
    page_size: usize,
) -> Result<PageResponse<ProcessRecord>, String> {
    let mut manager = ProcessManager::new();
    let request = PageRequest { page, page_size };
    manager
        .search_by_port_paginated(&port, &request)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn kill_process(pid: String) -> Result<(), String> {
    let mut manager = ProcessManager::new();
    manager.kill_process(&pid).map_err(|e| e.to_string())
}
