use bytemuck::{Pod, Zeroable};
use cognexus_model::geometry::quad::Quad;
use glam::{Mat4, Quat, Vec3, Vec4};
use wgpu::{VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

pub const LABEL: &str = "Quad Shader";
pub const SHADER_SOURCE: &str = include_str!("quad.wgsl");

// -----------------------------------------------------------------------------
// 1. The Vertex
// Matches: @location(0) position: vec3<f32>
// -----------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: [VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];

    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

// -----------------------------------------------------------------------------
// 2. The Instance
// Matches:
// @location(1)..@location(4) model_matrix (4x vec4)
// @location(5) color (vec4)
// -----------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32; 4]; 4], // 4x4 Matrix
    pub color: [f32; 4],      // r, g, b, a
}

impl InstanceRaw {
    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // Key part: "Instance" mode means "advance one step per quad", not per vertex
            step_mode: VertexStepMode::Instance,
            attributes: &[
                // A mat4 takes 4 slots. WGPU doesn't have "Float32x4x4", so we use 4x Float32x4
                // offset 0, location 1
                VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: VertexFormat::Float32x4,
                },
                // offset 16, location 2
                VertexAttribute {
                    offset: 16,
                    shader_location: 2,
                    format: VertexFormat::Float32x4,
                },
                // offset 32, location 3
                VertexAttribute {
                    offset: 32,
                    shader_location: 3,
                    format: VertexFormat::Float32x4,
                },
                // offset 48, location 4
                VertexAttribute {
                    offset: 48,
                    shader_location: 4,
                    format: VertexFormat::Float32x4,
                },
                // offset 64, location 5 (Color)
                VertexAttribute {
                    offset: 64,
                    shader_location: 5,
                    format: VertexFormat::Float32x4,
                },
            ],
        }
    }

    pub fn from_quad(quad: &Quad) -> Self {
        let translation = Mat4::from_translation(Vec3::new(
            quad.position[0],
            quad.position[1],
            quad.position[2],
        ));

        let scale = Mat4::from_scale(Vec3::new(quad.size[0], quad.size[1], 1.0));

        let model = translation * scale;

        Self {
            model: model.to_cols_array_2d(),
            color: quad.color,
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    // Top Right
    Vertex {
        position: [0.5, 0.5, 0.0],
    },
    // Top Left
    Vertex {
        position: [-0.5, 0.5, 0.0],
    },
    // Bottom Left
    Vertex {
        position: [-0.5, -0.5, 0.0],
    },
    // Bottom Right
    Vertex {
        position: [0.5, -0.5, 0.0],
    },
];

pub const INDICES: &[u16] = &[
    0, 1, 2, // Top-left triangle
    0, 2, 3, // Bottom-right triangle
];
