// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod plugin_manager;

fn main() {
    temp_test();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn temp_test() {
    // Print current working directory
    if let Ok(cwd) = std::env::current_dir() {
        println!("Current working directory: {}", cwd.display());
    }

    // Test plugin discovery
    let builtin_path = std::path::PathBuf::from("../../../target/debug/resources/builtin");

    if builtin_path.exists() {
        println!("Testing plugin discovery...");
        match plugin_manager::PluginManager::new(builtin_path) {
            Ok(mut manager) => {
                if let Err(e) = manager.discover_plugins() {
                    eprintln!("Plugin discovery failed: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to create plugin manager: {e}"),
        }
    } else {
        eprintln!("Builtin plugin directory not found. Run 'just build-wasm-debug' first.");
    }
}
