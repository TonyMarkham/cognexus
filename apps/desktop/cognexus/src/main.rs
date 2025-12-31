// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod plugin_manager;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Get the resource directory path using Tauri's PathResolver
            let resource_dir = app
                .path()
                .resolve("builtin", tauri::path::BaseDirectory::Resource)?;

            println!("Resource directory: {}", resource_dir.display());

            // Initialize plugin manager with the proper resource path
            match plugin_manager::PluginManager::new(resource_dir) {
                Ok(mut manager) => {
                    if let Err(e) = manager.discover_plugins() {
                        eprintln!("Plugin discovery failed: {e}");
                    }
                }
                Err(e) => eprintln!("Failed to create plugin manager: {e}"),
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
