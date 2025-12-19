use crate::error::RendererError;
use crate::renderer::Renderer;
use cognexus_model::geometry::quad::Quad;
use common::error::error_location::ErrorLocation;
use prost::Message;
use proto::{DrawQuadCommand, PanCameraCommand, ZoomCameraCommand};

pub fn handle_draw_quad(renderer: &mut Renderer, bytes: &[u8]) -> Result<(), RendererError> {
    let command = DrawQuadCommand::decode(bytes).map_err(|e| RendererError::CommandError {
        message: format!("Failed to decode DrawQuadCommand: {e}"),
        location: ErrorLocation::from(std::panic::Location::caller()),
    })?;

    let quad = Quad {
        position: [command.x, command.y, command.z],
        size: [command.width, command.height],
        color: [command.r, command.g, command.b, command.a],
    };

    renderer.add_quad(quad);
    renderer.render()?;

    Ok(())
}

pub fn handle_pan_camera(renderer: &mut Renderer, bytes: &[u8]) -> Result<(), RendererError> {
    let command = PanCameraCommand::decode(bytes).map_err(|e| RendererError::CommandError {
        message: format!("Failed to decode PanCameraCommand: {}", e),
        location: ErrorLocation::from(std::panic::Location::caller()),
    })?;

    renderer.pan_camera(command.delta_x, command.delta_y);
    renderer.render()?;

    Ok(())
}

pub fn handle_zoom_camera(renderer: &mut Renderer, bytes: &[u8]) -> Result<(), RendererError> {
    let command = ZoomCameraCommand::decode(bytes).map_err(|e| RendererError::CommandError {
        message: format!("Failed to decode ZoomCameraCommand: {}", e),
        location: ErrorLocation::from(std::panic::Location::caller()),
    })?;

    renderer.zoom_camera(command.delta, command.pivot_x, command.pivot_y);
    renderer.render()?;

    Ok(())
}
