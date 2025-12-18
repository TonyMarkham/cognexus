use crate::error::RendererError;
use cognexus_model::geometry::quad::Quad;
use common::error::error_location::ErrorLocation;
use std::panic::Location as PanicLocation;
use wgpu::PowerPreference::HighPerformance;
use wgpu::{
    Device, DeviceDescriptor, Features, Instance, Limits, MemoryHints, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};

pub struct Renderer {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: (u32, u32),
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
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size: (width, height),
        })
    }

    pub fn draw_quad(&self, quad: &Quad) -> Result<(), RendererError> {
        println!("Renderer (WGPU context active) drawing quad {quad:?}");

        Ok(())
    }
}
