use crate::drawable::Drawable;
use glam::{Mat4, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub position: [f32; 3], // x, y, z
    pub size: [f32; 2],     // width, height
    pub color: [f32; 4],    // r, g, b, a
}

impl Drawable for Quad {
    fn model_matrix(&self) -> Mat4 {
        let transform = Mat4::from_translation(Vec3::from(self.position));
        let scale = Mat4::from_scale(Vec3::new(self.size[0], self.size[1], 1.0));

        transform * scale
    }

    fn color(&self) -> [f32; 4] {
        self.color
    }
}
