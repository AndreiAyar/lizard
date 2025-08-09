use std::process::{Child, Command};
use std::sync::Mutex;
use tauri::{Manager, State};
use std::path::Path;

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

fn find_backend_path(app: &tauri::App) -> Option<std::path::PathBuf> {
    let resource_dir = app.path().resource_dir().unwrap();
    
    // Try different possible locations
    let possible_paths = vec![
        // Direct in resources (Windows/Linux)
        resource_dir.join("lizard-backend"),
        resource_dir.join("lizard-backend.exe"),
        // In binaries subdirectory
        resource_dir.join("binaries").join("lizard-backend"),
        resource_dir.join("binaries").join("lizard-backend.exe"),
        // macOS app bundle structure
        resource_dir.parent()?.join("Resources").join("lizard-backend"),
    ];
    
    for path in possible_paths {
        println!("Checking for backend at: {:?}", path);
        if path.exists() {
            println!("Found backend at: {:?}", path);
            return Some(path);
        }
    }
    
    None
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(PythonProcess(Mutex::new(None)))
        .setup(|app| {
            println!("Setting up Tauri app...");
            let resource_dir = app.path().resource_dir().unwrap();
            println!("Resource directory: {:?}", resource_dir);
            
            // List contents of resource directory for debugging
            if let Ok(entries) = std::fs::read_dir(&resource_dir) {
                println!("Contents of resource directory:");
                for entry in entries {
                    if let Ok(entry) = entry {
                        println!("  - {:?}", entry.file_name());
                    }
                }
            }
            
            // Try to find the backend executable
            if let Some(backend_path) = find_backend_path(app) {
                // Make sure it's executable on Unix systems
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = std::fs::metadata(&backend_path) {
                        let permissions = metadata.permissions();
                        if (permissions.mode() & 0o111) == 0 {
                            println!("Making backend executable...");
                            let _ = std::process::Command::new("chmod")
                                .args(["+x", backend_path.to_str().unwrap()])
                                .output();
                        }
                    }
                }
                
                match Command::new(&backend_path).spawn() {
                    Ok(python_process) => {
                        let state: State<PythonProcess> = app.state();
                        *state.0.lock().unwrap() = Some(python_process);
                        println!("Python backend started successfully with PID: {}", python_process.id());
                    }
                    Err(e) => {
                        eprintln!("Failed to start Python backend: {}", e);
                    }
                }
            } else {
                println!("Python backend not found in any expected location");
                
                // Development fallback
                #[cfg(debug_assertions)]
                {
                    println!("Trying development mode...");
                    if let Ok(python_process) = Command::new("python3")
                        .args(["-m", "uvicorn", "app.main:app", "--reload", "--port", "8000"])
                        .current_dir("../src-python")
                        .spawn()
                    {
                        let state: State<PythonProcess> = app.state();
                        *state.0.lock().unwrap() = Some(python_process);
                        println!("Development Python backend started");
                    }
                }
            }
            
            Ok(())
        })
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let state: State<PythonProcess> = _window.state();
                let mut process = state.0.lock().unwrap();
                if let Some(ref mut child) = *process {
                    let _ = child.kill();
                    println!("Backend process killed on window close");
                }
            }
        })
        .invoke_handler(tauri::generate_handler![greet, stop_python_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}