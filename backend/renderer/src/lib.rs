#![cfg(target_arch = "wasm32")]
mod commands;
mod error;
mod renderer;
pub mod shaders;

use crate::error::RendererError;
use cognexus_model::geometry::quad::Quad;

pub use crate::renderer::Renderer;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn draw_quad(quad: &Quad) -> Result<(), RendererError> {
    println!("Renderer received model::Quad: {:?}", quad);

    Ok(())
}
