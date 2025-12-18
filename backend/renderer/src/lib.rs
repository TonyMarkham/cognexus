mod error;

use crate::error::RendererError;
use cognexus_model::geometry::quad::Quad;

pub fn draw_quad(quad: &Quad) -> Result<(), RendererError> {
    println!("Renderer received model::Quad: {:?}", quad);

    Ok(())
}