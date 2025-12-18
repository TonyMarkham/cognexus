mod error;
mod renderer;
pub mod shaders;

use crate::error::RendererError;
use cognexus_model::geometry::quad::Quad;

pub use crate::renderer::Renderer;

pub fn draw_quad(quad: &Quad) -> Result<(), RendererError> {
    println!("Renderer received model::Quad: {:?}", quad);

    Ok(())
}
