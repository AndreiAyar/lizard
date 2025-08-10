use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::fs;
use std::net::TcpListener;
use std::path::PathBuf;
use std::io::Write;
use tauri::{Manager, State};

struct PythonProcess(Mutex<Option<Child>>);

const BACKEND_PORT: u16 = 8000;
const PID_FILE_NAME: &str = "lizard_backend.pid";

fn get_pid_file_path(app: &tauri::App) -> PathBuf {
    app.path().app_data_dir().unwrap_or_else(|_| std::env::temp_dir()).join(PID_FILE_NAME)
}

fn get_pid_file_path_from_handle(app_handle: &tauri::AppHandle) -> PathBuf {
    app_handle.path().app_data_dir().unwrap_or_else(|_| std::env::temp_dir()).join(PID_FILE_NAME)
}

fn is_port_available(port: u16) -> bool {
    TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok()
}

fn is_process_running(pid: u32) -> bool {
    #[cfg(unix)]
    {
        std::process::Command::new("kill")
            .args(["-0", &pid.to_string()])
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
    #[cfg(windows)]
    {
        std::process::Command::new("tasklist")
            .args(["/FI", &format!("PID eq {}", pid)])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).contains(&pid.to_string()))
            .unwrap_or(false)
    }
}

// Replace dangerous kill_all_lizard_backends with safer approach
fn cleanup_our_backend_processes(app_path_resolver: Option<&tauri::App>, app_handle: Option<&tauri::AppHandle>) {
    println!("Cleaning up our backend processes safely...");
    
    // Get PID file path based on what's available
    let pid_file = if let Some(app) = app_path_resolver {
        get_pid_file_path(app)
    } else if let Some(handle) = app_handle {
        get_pid_file_path_from_handle(handle)
    } else {
        return;
    };
    
    // Read our PID file and kill only our process
    if let Ok(pid_str) = fs::read_to_string(&pid_file) {
        if let Ok(pid) = pid_str.trim().parse::<u32>() {
            if is_process_running(pid) {
                println!("Killing our backend process with PID: {}", pid);
                
                #[cfg(unix)]
                {
                    // Try graceful termination first
                    if std::process::Command::new("kill")
                        .args(["-TERM", &pid.to_string()])
                        .status()
                        .map(|s| s.success())
                        .unwrap_or(false)
                    {
                        // Wait a bit for graceful shutdown
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        
                        // Force kill if still running
                        if is_process_running(pid) {
                            let _ = std::process::Command::new("kill")
                                .args(["-9", &pid.to_string()])
                                .status();
                        }
                    }
                }
                
                #[cfg(windows)]
                {
                    let _ = std::process::Command::new("taskkill")
                        .args(["/F", "/PID", &pid.to_string()])
                        .status();
                }
            }
        }
    }
    
    // Clean up PID file
    let _ = fs::remove_file(&pid_file);
    
    // Additional safety: check if our port is still occupied
    if !is_port_available(BACKEND_PORT) {
        println!("Warning: Port {} is still occupied after cleanup", BACKEND_PORT);
    }
}

fn write_pid_file(app: &tauri::App, pid: u32) {
    let pid_file = get_pid_file_path(app);
    if let Some(parent) = pid_file.parent() {
        let _ = fs::create_dir_all(parent);
    }
    
    if let Ok(mut file) = fs::File::create(&pid_file) {
        let _ = write!(file, "{}", pid);
        println!("Written PID {} to file: {:?}", pid, pid_file);
    }
}

fn is_backend_already_running(app: &tauri::App) -> bool {
    // Check if port is in use
    if !is_port_available(BACKEND_PORT) {
        println!("Backend port {} is already in use", BACKEND_PORT);
        return true;
    }
    
    // Check our PID file
    let pid_file = get_pid_file_path(app);
    if let Ok(pid_str) = fs::read_to_string(&pid_file) {
        if let Ok(pid) = pid_str.trim().parse::<u32>() {
            if is_process_running(pid) {
                println!("Our backend already running with PID: {}", pid);
                return true;
            } else {
                // Clean up stale PID file
                let _ = fs::remove_file(&pid_file);
            }
        }
    }
    
    false
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn stop_python_server(state: State<PythonProcess>, app: tauri::AppHandle) {
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
    
    // Clean up only our processes safely
    cleanup_our_backend_processes(None, Some(&app));
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
            
            // Check if our backend is already running (safer than killing everything)
            if is_backend_already_running(app) {
                println!("Backend already running, skipping startup");
                return Ok(());
            }
            
            // Kill any existing lizard-backend processes before starting
            //kill_all_lizard_backends();
            
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
                            
                            // Write PID file for safe tracking
                            write_pid_file(app, pid);
                            
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
                println!("Window close requested, cleaning up our backend processes...");
                
                // Clean up only our processes safely
                cleanup_our_backend_processes(None, Some(&window.app_handle()));
                
                #[cfg(not(debug_assertions))]
                {
                    let state: State<PythonProcess> = window.state();
                    let mut process_lock = state.0.lock().unwrap();
                    if let Some(ref mut child) = *process_lock {
                        let pid = child.id();
                        println!("Also killing tracked process with PID: {}", pid);
                        match child.kill() {
                            Ok(_) => {
                                println!("Kill signal sent to tracked PID: {}", pid);
                                match child.wait() {
                                    Ok(status) => println!("Tracked process terminated with status: {}", status),
                                    Err(e) => eprintln!("Error waiting for tracked process: {}", e),
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to kill tracked process: {}", e);
                            }
                        }
                        *process_lock = None;
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![greet, stop_python_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}