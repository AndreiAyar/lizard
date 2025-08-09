use std::process::{Child, Command};
use std::sync::Mutex;
use tauri::{Manager, State};
use std::env;

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
            // Get the path to the bundled Python executable
            let resource_dir = app.path().resource_dir().unwrap();
            let backend_path = if cfg!(target_os = "windows") {
                resource_dir.join("lizard-backend.exe")
            } else {
                resource_dir.join("lizard-backend")
            };
            
            // Start bundled Python server
            let python_process = Command::new(backend_path)
                .spawn()
                .expect("Failed to start Python server");
            
            let state: State<PythonProcess> = app.state();
            *state.0.lock().unwrap() = Some(python_process);
            
            Ok(())
        })
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let state: State<PythonProcess> = _window.state();
                let mut process = state.0.lock().unwrap();
                if let Some(ref mut child) = *process {
                    let _ = child.kill();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![greet, stop_python_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}