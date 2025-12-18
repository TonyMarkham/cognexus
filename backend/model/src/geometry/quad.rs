#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub position: [f32; 3], // x, y, z
    pub size: [f32; 2],     // width, height
    pub color: [f32; 4],    // r, g, b, a
}