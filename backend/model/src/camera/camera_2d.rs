use crate::error::ModelError;
use common::error::error_location::ErrorLocation;
use glam::{Mat4, Vec2, Vec3};
use std::panic::Location as PanicLocation;

pub const MIN_ZOOM: f32 = 0.001;
pub const DEFAULT_MIN_ZOOM: f32 = 0.1;
pub const MIN_ZOOM_MAX: f32 = 0.01;
pub const MAX_ZOOM: f32 = 1000.0;
pub const DEFAULT_MAX_ZOOM: f32 = 100.0;
pub const DEFAULT_ZOOM_RATE: f32 = 0.001;

pub struct Camera2D {
    position: Vec2,
    zoom: f32,
    viewport_size: (u32, u32),
    zoom_min: f32,
    zoom_max: f32,
}

impl Camera2D {
    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn viewport_size(&self) -> (u32, u32) {
        self.viewport_size
    }

    pub fn zoom_min(&self) -> f32 {
        self.zoom_min
    }

    pub fn zoom_max(&self) -> f32 {
        self.zoom_max
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::from_translation(Vec3::new(-self.position.x, -self.position.y, 0.0))
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let aspect_ratio = self.viewport_size.0 as f32 / self.viewport_size.1 as f32;

        let height = 2.0 / self.zoom;
        let width = height * aspect_ratio;

        Mat4::orthographic_rh(
            -width / 2.0,
            width / 2.0,
            -height / 2.0,
            height / 2.0,
            -1.0,
            1.0,
        )
    }

    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    pub fn pan_by_screen_delta(&mut self, delta_x: f32, delta_y: f32) {
        let aspect_ratio = self.viewport_size.0 as f32 / self.viewport_size.1 as f32;
        let height = 2.0 / self.zoom;
        let width = height * aspect_ratio;

        let world_per_pixel_x = width / self.viewport_size.0 as f32;
        let world_per_pixel_y = height / self.viewport_size.1 as f32;

        // Convert screen delta to world delta
        // Negative X: dragging right = pan canvas right (camera moves left)
        // Negative Y: dragging down = pan canvas down (camera moves up)
        self.position.x -= delta_x * world_per_pixel_x;
        self.position.y -= delta_y * world_per_pixel_y;
    }

    pub fn zoom_toward_point(&mut self, scroll_delta: f32, screen_x: f32, screen_y: f32) {
        let world_pos_before = self.screen_to_world(screen_x, screen_y);

        let zoom_factor = 1.0 + (scroll_delta * -DEFAULT_ZOOM_RATE);
        let new_zoom = (self.zoom * zoom_factor).clamp(self.zoom_min, self.zoom_max);

        self.zoom = new_zoom;

        let world_pos_after = self.screen_to_world(screen_x, screen_y);

        let world_delta = world_pos_after - world_pos_before;
        self.position -= world_delta;
    }

    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> Vec2 {
        let ndc_x = (screen_x / self.viewport_size.0 as f32) * 2.0 - 1.0;
        let ndc_y = 1.0 - (screen_y / self.viewport_size.1 as f32) * 2.0;

        let aspect = self.viewport_size.0 as f32 / self.viewport_size.1 as f32;
        let viewport_height_world = 2.0 / self.zoom;
        let viewport_width_world = viewport_height_world * aspect;

        Vec2::new(
            self.position.x + ndc_x * viewport_width_world / 2.0,
            self.position.y + ndc_y * viewport_height_world / 2.0,
        )
    }
}

#[derive(Debug, Default)]
#[must_use = "Call .build() or continue chaining setters; dropping the builder does nothing."]
pub struct Camera2DBuilder {
    viewport_size: Option<(u32, u32)>,
    zoom_min: Option<f32>,
    zoom_max: Option<f32>,
}

impl Camera2DBuilder {
    pub fn with_viewport(mut self, width: u32, height: u32) -> Self {
        self.viewport_size = Some((width, height));
        self
    }

    pub fn with_min_zoom(mut self, min: f32) -> Self {
        self.zoom_min = Some(min);
        self
    }

    pub fn with_max_zoom(mut self, max: f32) -> Self {
        self.zoom_max = Some(max);
        self
    }

    pub fn build(self) -> Result<Camera2D, ModelError> {
        let viewport_size = self.viewport_size.ok_or(ModelError::CameraError {
            message: String::from("Missing Viewport Size!"),
            location: ErrorLocation::from(PanicLocation::caller()),
        })?;

        let zoom_max = match self.zoom_max {
            Some(max) => {
                if max <= MIN_ZOOM_MAX {
                    return Err(ModelError::CameraError {
                        message: format!(
                            "Maximum zoom cannot be less-than or equal to {MIN_ZOOM_MAX}: {max}"
                        ),
                        location: ErrorLocation::from(PanicLocation::caller()),
                    });
                }

                max
            }
            None => DEFAULT_MAX_ZOOM,
        };

        let zoom_min = match self.zoom_min {
            Some(min) => {
                if min <= MIN_ZOOM {
                    return Err(ModelError::CameraError {
                        message: format!(
                            "Minimum zoom cannot be less-than or equal to {MIN_ZOOM}: {min}"
                        ),
                        location: ErrorLocation::from(PanicLocation::caller()),
                    });
                }

                if min >= zoom_max {
                    return Err(ModelError::CameraError {
                        message: format!(
                            "Minimum zoom cannot be greater-than or equal to Maximum zoom: {min} >= {zoom_max}"
                        ),
                        location: ErrorLocation::from(PanicLocation::caller()),
                    });
                }

                min
            }
            None => DEFAULT_MIN_ZOOM,
        };

        Ok(Camera2D {
            position: Vec2::ZERO,
            zoom: 1.0,
            viewport_size,
            zoom_min,
            zoom_max,
        })
    }
}
