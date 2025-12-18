use crate::error::RendererError;
use crate::shaders::quad::{INDICES, InstanceRaw, LABEL, SHADER_SOURCE, VERTICES, Vertex};
use cognexus_model::camera::camera_2d::{Camera2D, Camera2DBuilder};
use cognexus_model::geometry::quad::Quad;
use common::error::error_location::ErrorLocation;
use std::panic::Location as PanicLocation;
use wgpu::PowerPreference::HighPerformance;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::wgt::TextureViewDescriptor;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BlendState, Buffer, BufferBindingType, BufferUsages, Color,
    ColorTargetState, ColorWrites, CommandEncoderDescriptor, CompositeAlphaMode, Device,
    DeviceDescriptor, Features, FragmentState, FrontFace, IndexFormat, Instance, Limits, LoadOp,
    MemoryHints, MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor,
    ShaderSource, ShaderStages, StoreOp, Surface, SurfaceConfiguration, TextureUsages, VertexState,
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

pub struct Renderer {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    #[allow(dead_code)]
    config: SurfaceConfiguration,
    #[allow(dead_code)]
    size: (u32, u32),
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
    #[allow(dead_code)]
    camera: Camera2D,
    #[allow(dead_code)]
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
}

impl Renderer {
    pub async fn new(
        instance: Instance,
        surface: Surface<'static>,
        width: u32,
        height: u32,
    ) -> Result<Self, RendererError> {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| RendererError::WgpuError {
                message: format!("No suitable GPU adapter found: {e}"),
                location: ErrorLocation::from(PanicLocation::caller()),
            })?;

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("Cognexus Device"),
                required_features: Features::empty(),
                required_limits: Limits::default(),
                memory_hints: MemoryHints::default(),
                ..Default::default()
            })
            .await
            .map_err(|e| RendererError::WgpuError {
                message: format!("Failed to create device: {}", e),
                location: ErrorLocation::from(PanicLocation::caller()),
            })?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: CompositeAlphaMode::PostMultiplied,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some(LABEL),
            source: ShaderSource::Wgsl(SHADER_SOURCE.into()),
        });

        let camera = Camera2DBuilder::default()
            .with_viewport(width, height)
            .build()
            .map_err(|e| RendererError::WgpuError {
                message: format!("Failed to create Camera: {e}"),
                location: ErrorLocation::from(PanicLocation::caller()),
            })?;

        let camera_uniform = CameraUniform {
            view_proj: camera.view_projection_matrix().to_cols_array_2d(),
        };

        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0, // @binding(0) in shader
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout], // No uniforms/textures yet
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // Match fn in .wgsl
                compilation_options: Default::default(),
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                // cull_mode: Some(Face::Back),
                cull_mode: None,
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Quad vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Quad Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size: (width, height),
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            camera,
            camera_buffer,
            camera_bind_group,
        })
    }

    pub fn pan_camera(&mut self, delta_x: f32, delta_y: f32) {
        self.camera.pan_by_screen_delta(delta_x, delta_y);
        self.update_camera_uniform();
    }

    pub fn zoom_camera(&mut self, scroll_delta: f32, screen_x: f32, screen_y: f32) {
        self.camera
            .zoom_toward_point(scroll_delta, screen_x, screen_y);
        self.update_camera_uniform();
    }

    pub fn update_camera_uniform(&self) {
        let camera_uniform = CameraUniform {
            view_proj: self.camera.view_projection_matrix().to_cols_array_2d(),
        };

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    pub fn draw_quad(&self, quad: &Quad) -> Result<(), RendererError> {
        let output = self
            .surface
            .get_current_texture()
            .map_err(|e| RendererError::WgpuError {
                message: format!("Failed to get texture: {e}"),
                location: ErrorLocation::from(PanicLocation::caller()),
            })?;

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let instance_data = InstanceRaw::from_quad(quad);

        let instance_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&[instance_data]),
            usage: BufferUsages::VERTEX,
        });

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        println!("Renderer (WGPU context active) drawing quad {quad:?}");

        Ok(())
    }
}
