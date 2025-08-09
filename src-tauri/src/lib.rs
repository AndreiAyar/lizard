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
                .args(["-m", "uvicorn", "app.main:app", "--reload", "--port", "8000"])
                .current_dir("../src-python")
                .spawn()
                .expect("Failed to start Python server");
            
            let state: State<PythonProcess> = app.state();
            *state.0.lock().unwrap() = Some(python_process);
            
            Ok(())
        })
        .on_window_event(|_window, event| { // Fix: two parameters
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Kill Python process when window closes
                let state: State<PythonProcess> = _window.state();
                let mut process = state.0.lock().unwrap();
                if let Some(ref mut child) = *process {
                    let _ = child.kill(); // This should work now
                }
            }
        })
        .invoke_handler(tauri::generate_handler![greet, stop_python_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}