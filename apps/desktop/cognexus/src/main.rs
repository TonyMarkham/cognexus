// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use prost::Message;
use cognexus_model::geometry::quad::Quad;
use cognexus_renderer::draw_quad as renderer_draw_quad;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![draw_quad])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn draw_quad(data: Vec<u8>) -> Result<(), String> {
    let command = proto::DrawQuadCommand::decode(&data[..])
        .map_err(|e| format!("Failed to decode command: {e}"))?;

    let quad = Quad {
        position: [command.x, command.y, command.z],
        size: [command.width, command.height],
        color: [command.r, command.g, command.b, command.a],
    };

    renderer_draw_quad(&quad)
        .map_err(|e| format!("Failed to draw quad: {e}"))?;

    Ok(())
}
