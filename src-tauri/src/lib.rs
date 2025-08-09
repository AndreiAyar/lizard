use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use tauri::{Manager, State};

struct PythonProcess(Mutex<Option<Child>>);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn stop_python_server(state: State<PythonProcess>) {
    let mut process = state.0.lock().unwrap();
    if let Some(ref mut child) = *process {
        println!("Attempting to stop Python backend...");
        
        // Try graceful termination first
        match child.kill() {
            Ok(_) => {
                println!("Kill signal sent successfully");
                
                match child.wait() {
                    Ok(status) => println!("Backend process terminated with status: {}", status),
                    Err(e) => eprintln!("Error waiting for process termination: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Failed to kill backend process: {}", e);
                // Try platform-specific force kill as a last resort
                #[cfg(unix)]
                {
                    println!("Force killing with SIGKILL: {}", child.id());
                    let _ = std::process::Command::new("kill")
                        .args(["-9", &child.id().to_string()])
                        .status();
                }
                #[cfg(windows)]
                {
                    println!("Force killing with taskkill: {}", child.id());
                    let _ = std::process::Command::new("taskkill")
                        .args(["/F", "/PID", &child.id().to_string()])
                        .status();
                }
            }
        }
        *process = None;
    }
}

fn find_backend_path(app: &tauri::App) -> Option<std::path::PathBuf> {
    let resource_dir = app.path().resource_dir().unwrap();
    
    // checking different possible locations..
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
            
            // Only start packaged backend in production mode
            #[cfg(not(debug_assertions))]
            {
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
                    
                    match Command::new(&backend_path)
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                    {
                        Ok(python_process) => {
                            let pid = python_process.id();
                            println!("Python backend started successfully with PID: {}", pid);
                            let state: State<PythonProcess> = app.state();
                            *state.0.lock().unwrap() = Some(python_process);
                        }
                        Err(e) => {
                            eprintln!("Failed to start Python backend: {}", e);
                        }
                    }
                } else {
                    println!("Python backend not found in any expected location");
                }
            }
            
            // In development mode, we expect you to run `python main.py` manually
            #[cfg(debug_assertions)]
            {
                println!("Development mode: Start your Python backend manually with 'python main.py'");
            }
            
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                #[cfg(not(debug_assertions))]
                {
                    println!("Window close requested, attempting to stop backend...");
                    let state: State<PythonProcess> = window.state();
                    let mut process_lock = state.0.lock().unwrap();
                    if let Some(ref mut child) = *process_lock {
                        let pid = child.id();
                        println!("Killing backend process with PID: {}", pid);
                        match child.kill() {
                            Ok(_) => {
                                println!("Kill signal sent to PID: {}", pid);
                                match child.wait() {
                                    Ok(status) => println!("Backend process terminated with status: {}", status),
                                    Err(e) => eprintln!("Error waiting for backend process: {}", e),
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to kill backend process: {}", e);
                                #[cfg(unix)]
                                {
                                    println!("Force killing with SIGKILL: {}", pid);
                                    let _ = std::process::Command::new("kill")
                                        .args(["-9", &pid.to_string()])
                                        .status();
                                }
                                #[cfg(windows)]
                                {
                                    println!("Force killing with taskkill: {}", pid);
                                    let _ = std::process::Command::new("taskkill")
                                        .args(["/F", "/PID", &pid.to_string()])
                                        .status();
                                }
                            }
                        }
                        *process_lock = None;
                    } else {
                        println!("No backend process to kill");
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![greet, stop_python_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}