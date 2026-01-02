// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod logger;

use crate::error::CognexusError;
use crate::logger::initialize as LoggerInitialize;

use cognexus_plugin_manager::{PluginManager, Registry};

use std::fs::create_dir_all;

use log::{error, info};
use tauri::Manager;
use tauri::path::BaseDirectory;

#[tauri::command]
fn list_available_nodes(
    registry: tauri::State<Registry>,
) -> Result<Vec<proto::NodeDefinition>, CognexusError> {
    Ok(registry.list_nodes()?)
}

#[tauri::command]
fn list_available_types(
    registry: tauri::State<Registry>,
) -> Result<Vec<proto::TypeDefinition>, CognexusError> {
    Ok(registry.list_types()?)
}

#[tauri::command]
fn get_node_definition(
    id: String,
    registry: tauri::State<Registry>,
) -> Result<Option<proto::NodeDefinition>, CognexusError> {
    Ok(registry.get_node(&id)?)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_available_nodes,
            list_available_types,
            get_node_definition
        ])
        .setup(|app| {
            // Get app data directory for logs
            let log_dir = app.path().app_log_dir()?;

            // Ensure log directory exists
            create_dir_all(&log_dir)?;

            // Initialize logger FIRST
            LoggerInitialize(log_dir)?;

            // Get the resource directory path using Tauri's PathResolver
            let resource_dir = app.path().resolve("builtin", BaseDirectory::Resource)?;

            info!("Resource directory: {}", resource_dir.display());

            // Create registry for discovered plugins
            let registry = Registry::default();

            // Initialize plugin manager with the proper resource path
            match PluginManager::new(resource_dir) {
                Ok(mut manager) => {
                    if let Err(e) = manager.discover_plugins(&registry) {
                        error!("Plugin discovery failed: {e}");
                    }
                }
                Err(e) => error!("Failed to create plugin manager: {e}"),
            }

            // Store registry in Tauri state for commands to access
            app.manage(registry);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
