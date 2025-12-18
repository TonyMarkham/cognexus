// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use prost::Message;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![draw_quad])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn draw_quad(data: Vec<u8>) -> Result<(), String> {
    // Deserialize the protobuf bytes
    let command = proto::DrawQuadCommand::decode(&data[..])
        .map_err(|e| format!("Failed to decode command: {}", e))?;

    // Log it for now (we'll connect to renderer later)
    println!("Received DrawQuad: x={}, y={}, z={}, width={}, height={}, color=({},{},{},{})",
             command.x, command.y, command.z, command.width, command.height,
             command.r, command.g, command.b, command.a);

    Ok(())
}
