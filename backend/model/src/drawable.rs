use glam::Mat4;

/// Trait for objects that can be rendered in the scene
pub trait Drawable: Send {
    /// Returns the model transformation matrix (position, rotation, scale)
    fn model_matrix(&self) -> Mat4;

    /// Returns the color as [r, g, b, a]
    fn color(&self) -> [f32; 4];
}
