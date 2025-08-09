use std::process::{Child, Command};
use std::sync::Mutex;
use tauri::{Manager, State}; // Add Manager trait

struct PythonProcess(Mutex<Option<Child>>);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn stop_python_server(state: State<PythonProcess>) {
    let mut process = state.0.lock().unwrap();
    if let Some(ref mut child) = *process {
        let _ = child.kill();
        *process = None;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(PythonProcess(Mutex::new(None)))
        .setup(|app| {
            // Start Python server
            let python_process = Command::new("python3") // Use python3 explicitly
                .args(["-m", "uvicorn", "app.main:app", "--reload", "