use slab::Slab;
use wgpu::{Device, Queue, Surface, Adapter};
use winit::window::Window;

pub struct GpuAdapterState {
    pub inner: Adapter
}

impl GpuAdapterState {
    pub fn new(adapter: Adapter) -> GpuAdapterState {
        GpuAdapterState {
            inner: adapter
        }
    }
}

pub struct GpuDeviceState<'a> {
    pub inner: wgpu::Device,
    pub buffers: Slab::<wgpu::Buffer>,
    pub command_buffers: Slab::<wgpu::CommandBuffer>,
    pub shader_modules: Slab::<wgpu::ShaderModule>,
    pub pipeline_layouts: Slab::<wgpu::PipelineLayout>,
    pub bind_group_layouts: Slab::<wgpu::BindGroupLayout>,
    pub render_pipelines: Slab::<wgpu::RenderPipeline>,
    pub command_encoders: Slab::<GpuCommandEncoderState<'a>>,
    pub texture_views: Slab::<wgpu::TextureView>,
}

impl GpuDeviceState<'_> {
    pub fn new<'a>(device: wgpu::Device) -> GpuDeviceState<'a> {
        GpuDeviceState {
            inner: device,
            buffers: Slab::new(),
            command_buffers: Slab::new(),
            shader_modules: Slab::new(),
            pipeline_layouts: Slab::new(),
            bind_group_layouts: Slab::new(),
            render_pipelines: Slab::new(),
            command_encoders: Slab::new(),
            texture_views: Slab::new(),
        }
    }
}

pub struct GpuCommandEncoderState<'a> {
    pub encoder: wgpu::CommandEncoder,
    pub render_passes: Slab::<wgpu::RenderPass<'a>>
}

impl<'a> GpuCommandEncoderState<'a> {
    pub fn new(encoder: wgpu::CommandEncoder) -> GpuCommandEncoderState<'a> {
        GpuCommandEncoderState {
            encoder,
            render_passes: Slab::new()
        }
    }
}