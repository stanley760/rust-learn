use crate::process::manager::ProcessManager;
use crate::types::ProcessRecord;

// Tauri command handlers will be implemented here

#[tauri::command]
pub async fn get_processes() -> Result<Vec<ProcessRecord>, String> {
    let mut manager = ProcessManager::new();
    manager.get_all_processes().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_by_port(port: String) -> Result<Vec<ProcessRecord>, String> {
    let mut manager = ProcessManager::new();
    manager.search_by_port(&port).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn kill_process(pid: String) -> Result<(), String> {
    let mut manager = ProcessManager::new();
    manager.kill_process(&pid).map_err(|e| e.to_string())
}
