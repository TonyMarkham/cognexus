// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cognexus_model::geometry::quad::Quad;
use cognexus_renderer::Renderer;
use prost::Message;
use tauri::{Manager, State};
use tokio::sync::Mutex;
use wgpu::Instance;

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            renderer: Mutex::new(None),
        })
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .ok_or("Failed to find main window")?;

            // 1. Main Thread: Create WGPU objects
            let instance = Instance::default();
            // window.clone() is safe here
            let surface = instance
                .create_surface(window.clone())
                .expect("Failed to create surface on main thread");

            let handle = app.handle().clone();
            let size = window
                .inner_size()
                .unwrap_or(tauri::PhysicalSize::new(800, 600));

            // 2. Background Thread: Initialize Renderer
            tauri::async_runtime::spawn(async move {
                let state = handle.state::<AppState>();

                println!("Initializing WGPU Renderer...");
                // Pass instance and surface (MOVED into this closure)
                match Renderer::new(instance, surface, size.width, size.height).await {
                    Ok(renderer) => {
                        let mut guard = state.renderer.lock().await;
                        *guard = Some(renderer);
                        println!("WGPU Renderer initialized successfully!");
                    }
                    Err(e) => {
                        eprintln!("CRITICAL: Failed to initialize renderer: {}", e);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![draw_quad])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct AppState {
    renderer: Mutex<Option<Renderer>>,
}

// -------------------------------------------------------------------------- //

#[tauri::command]
async fn draw_quad(data: Vec<u8>, state: State<'_, AppState>) -> Result<(), String> {
    let command = proto::DrawQuadCommand::decode(&data[..])
        .map_err(|e| format!("Failed to decode command: {e}"))?;

    let quad = Quad {
        position: [command.x, command.y, command.z],
        size: [command.width, command.height],
        color: [command.r, command.g, command.b, command.a],
    };

    let guard = state.renderer.lock().await;

    if let Some(renderer) = guard.as_ref() {
        renderer
            .draw_quad(&quad)
            .map_err(|e| format!("Failed to draw quad: {e}"))?;
    } else {
        return Err(String::from("Renderer is not initialized yet"));
    }

    Ok(())
}
