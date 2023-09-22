use core::slice::SlicePattern;
use std::borrow::Cow;

use futures::{executor, Future};
use once_cell::sync::OnceCell;
use wgpu::{Device, Queue, Surface, ShaderModuleDescriptor, ShaderSource, PipelineLayoutDescriptor, RenderPipelineDescriptor, VertexState, VertexBufferLayout, VertexStepMode, VertexAttribute, VertexFormat, PrimitiveState, PrimitiveTopology, FrontFace, Face, PolygonMode, DepthStencilState, TextureFormat, AstcBlock, AstcChannel, StencilState, StencilFaceState, StencilOperation, CompareFunction, DepthBiasState, MultisampleState, FragmentState, ColorTargetState, ColorWrites, CommandEncoderDescriptor, RenderPassDescriptor, RenderPassColorAttachment, Operations, LoadOp, Color, RenderPassDepthStencilAttachment};
use winit::window::Window;
use wasmtime::{Result, component::Resource};
use crate::{runtime::gpu::{GpuAdapterState, GpuCommandEncoderState}, WizardRuntimeState};

use crate::wizard::runtime::renderer::*;

impl WizardRuntimeState<'_> {
    
}

impl Host for WizardRuntimeState<'_> {
    fn get_gpu<'life0,'async_trait>(&'life0 mut self,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<Gpu> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        Box::pin(async move {
            Result::Ok(Resource::<Gpu>::new_own(0))
        })
    }
}

impl HostGpuQuerySet for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuQuerySet> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn type_<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuQuerySet> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuQueryType> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn count<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuQuerySet> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuSizeU32Out> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn destroy<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuQuerySet> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuQuerySet>) -> Result<()>  {
        todo!()
    }
}

impl HostBufferSource for WizardRuntimeState<'_> {
    fn drop(&mut self,rep:Resource<BufferSource>) -> Result<()>  {
        todo!()
    }
}

impl HostGpu for WizardRuntimeState<'_> {
    #[must_use]
    #[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn request_adapter<'life0,'async_trait>(&'life0 mut self,self_:Resource<Gpu> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuAdapter>>> + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        Box::pin(async move {
            Result::Ok(Resource::<GpuAdapter>::new_own(0))
        })
    }

    fn drop(&mut self,rep:Resource<Gpu>) -> Result<()>  {
        Ok(())
    }
}

impl HostGpuAdapter for WizardRuntimeState<'_> {
    fn request_device<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuAdapter> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuDevice> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        Box::pin(async move {
            Result::Ok(Resource::<GpuDevice>::new_own(0))
        })
    }

    fn drop(&mut self,rep:Resource<GpuAdapter>) -> Result<()>  {
        Result::Ok(())
    }
}

impl HostGpuDevice for WizardRuntimeState<'_> {
    fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        Box::pin(async move {
            Result::Ok("GPU".to_owned())
        })
    }

    fn create_buffer<'life0,'async_trait>(&'life0 mut self,self_: Resource<GpuDevice> ,descriptor:GpuBufferDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuBuffer> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        let buffer = self.device.inner.create_buffer(&wgpu::BufferDescriptor {
            label: descriptor.label.as_deref(),
            size: descriptor.size,
            // usage: wgpu::BufferUsages::from_bits_retain(descriptor.usage),
            usage: wgpu::BufferUsages::all(),
            mapped_at_creation: descriptor.mapped_at_creation
        });

        Box::pin(async move {
            Result::Ok(Resource::<GpuBuffer>::new_own(self.device.buffers.insert(buffer) as u32))
        })
    }

    fn create_texture<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,descriptor:GpuTextureDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuTexture> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn create_sampler<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,descriptor:GpuSamplerDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuSampler> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn import_external_texture<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,descriptor:GpuExternalTextureDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuExternalTexture> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn create_bind_group_layout<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,descriptor:GpuBindGroupLayoutDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuBindGroupLayout> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn create_pipeline_layout<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,descriptor:GpuPipelineLayoutDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuPipelineLayout> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        let bind_group_layouts: Vec<&wgpu::BindGroupLayout> = descriptor.bind_group_layouts.iter().map(|res| self.device.bind_group_layouts.get(res.rep() as usize).unwrap()).collect();
        
        let pipeline_layout = self.device.inner.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &bind_group_layouts,
            push_constant_ranges: &[],
        });

        Box::pin(async move {
            Result::Ok(Resource::<GpuPipelineLayout>::new_own(self.device.pipeline_layouts.insert(pipeline_layout) as u32))
        })
    }

    fn create_bind_group<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,descriptor:GpuBindGroupDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuBindGroup> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn create_shader_module<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,descriptor:GpuShaderModuleDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuShaderModule> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        let shader_module = self.device.inner.create_shader_module(ShaderModuleDescriptor {
            label: descriptor.label.as_deref(),
            source: ShaderSource::Wgsl(Cow::Owned(descriptor.code)),
        });

        Box::pin(async move {
            Result::Ok(Resource::<GpuShaderModule>::new_own(self.device.shader_modules.insert(shader_module) as u32))
        })
    }

    fn create_compute_pipeline<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,descriptor:GpuComputePipelineDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuComputePipeline> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn create_render_pipeline<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,descriptor:GpuRenderPipelineDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuRenderPipeline> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        let layout = match descriptor.layout {
            GpuLayout::Auto => None,
            GpuLayout::Pipeline(pipeline_layout) => Some(self.device.pipeline_layouts.get(pipeline_layout.rep() as usize).unwrap()),
        };

        let fragment_targets = match descriptor.fragment {
            Some(ref fragment) => fragment.targets.iter().map(|c| Some(ColorTargetState {
                format: c.format.into(),
                blend: match c.blend {
                    None => None,
                    Some(blend) => Some(blend.into())
                },
                write_mask: match c.write_mask {
                    None => ColorWrites::default(),
                    Some(write_mask) => write_mask.into()
                },
            })).collect::<Vec<Option<ColorTargetState>>>(),
            None => Vec::<Option<ColorTargetState>>::new()
        };

        let vertex_buffer_attributes = match descriptor.vertex.buffers {
            Some(ref buffers) => {
                let mut attrs = Vec::<Vec<VertexAttribute>>::with_capacity(buffers.len());
                buffers.iter().enumerate().for_each(|(i, v)| {
                    attrs.insert(i, v.attributes.iter().map(|a| VertexAttribute {
                        format: unsafe { std::mem::transmute(a.format as u32) },
                        offset: a.offset,
                        shader_location: a.shader_location,
                    }).collect::<Vec<VertexAttribute>>());
                });
                attrs
            },
            None => Vec::<Vec<VertexAttribute>>::new()
        };

        let vertex_buffers = match descriptor.vertex.buffers {
            Some(ref buffers) => buffers.iter().enumerate().map(|(i, v)| {
                let attributes = vertex_buffer_attributes.get(i).unwrap();
                
                VertexBufferLayout {
                    array_stride: v.array_stride,
                    step_mode: unsafe { std::mem::transmute(v.step_mode as u32) },
                    attributes: &attributes.as_slice(),
                }
            }).collect::<Vec<VertexBufferLayout>>(),
            None => Vec::<VertexBufferLayout>::new()
        };
        
        let render_pipeline = self.device.inner.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout,
            vertex: VertexState {
                module: self.device.shader_modules.get(descriptor.vertex.module.rep() as usize).unwrap(),
                entry_point: &descriptor.vertex.entry_point,
                buffers: vertex_buffers.as_slice(),
            },
            primitive: match descriptor.primitive {
                None => PrimitiveState::default(),
                Some(primitive) => PrimitiveState {
                    topology: match primitive.topology {
                        None => PrimitiveTopology::default(),
                        Some(topology) => unsafe { std::mem::transmute(topology as u32) }
                    },
                    strip_index_format: match primitive.strip_index_format {
                        None => None,
                        Some(strip_index_format) => unsafe { std::mem::transmute(strip_index_format as u32) }
                    },
                    front_face: match primitive.front_face {
                        None => FrontFace::default(),
                        Some(front_face) => unsafe { std::mem::transmute(front_face as u32) }
                    },
                    cull_mode: match primitive.cull_mode {
                        GpuCullMode::None => None,
                        GpuCullMode::Front => Some(Face::Front),
                        GpuCullMode::Back => Some(Face::Back),
                    },
                    unclipped_depth: primitive.unclipped_depth,
                    polygon_mode: PolygonMode::default(),
                    conservative: false,
                }
            },
            depth_stencil: match descriptor.depth_stencil {
                None => None,
                Some(depth_stencil) => Some(DepthStencilState {
                    format: depth_stencil.format.into(),
                    depth_write_enabled: depth_stencil.depth_write_enabled,
                    depth_compare: depth_stencil.depth_compare.into(),
                    stencil: StencilState {
                        front: match depth_stencil.stencil_front {
                            None => StencilFaceState::default(),
                            Some(front) => front.into()
                        },
                        back: match depth_stencil.stencil_back {
                            None => StencilFaceState::default(),
                            Some(front) => front.into()
                        },
                        read_mask: match depth_stencil.stencil_read_mask {
                            Some(read_mask) => read_mask,
                            None => 0,
                        },
                        write_mask: match depth_stencil.stencil_write_mask {
                            Some(write_mask) => write_mask,
                            None => 0,
                        },
                    },
                    bias: DepthBiasState {
                        constant: depth_stencil.depth_bias.unwrap_or_default(),
                        slope_scale: depth_stencil.depth_bias_slope_scale.unwrap_or_default(),
                        clamp: depth_stencil.depth_bias_clamp.unwrap_or_default()
                    },
                })
            },
            multisample: match descriptor.multisample {
                None => MultisampleState::default(),
                Some(ref multisample) => MultisampleState {
                    count: multisample.count,
                    mask: multisample.mask as u64,
                    alpha_to_coverage_enabled: multisample.alpha_to_coverage_enabled
                }
            },
            fragment: match descriptor.fragment {
                None => None,
                Some(ref fragment) => {
                    Some(FragmentState {
                        module: self.device.shader_modules.get(fragment.module.rep() as usize).unwrap(),
                        entry_point: &fragment.entry_point,
                        targets: fragment_targets.as_slice(),
                    })
                }
            },
            multiview: None,
        });

        Box::pin(async move {
            Result::Ok(Resource::<GpuRenderPipeline>::new_own(self.device.render_pipelines.insert(render_pipeline) as u32))
        })
    }

    fn create_command_encoder<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,descriptor:GpuCommandEncoderDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuCommandEncoder> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        let encoder = self.device.inner.create_command_encoder(&CommandEncoderDescriptor {
            label: descriptor.label.as_deref(),
        });

        let command_encoder_id = self.device.command_encoders.insert(GpuCommandEncoderState::new(encoder));

        Box::pin(async move {
            Result::Ok(Resource::<GpuCommandEncoder>::new_own(command_encoder_id as u32))
        })
    }

    fn create_render_bundle_encoder<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,descriptor:GpuRenderBundleDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuRenderBundleEncoder> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn create_query_set<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuDevice> ,descriptor:GpuQuerySetDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuQuerySet> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuDevice>) -> Result<()>  {
        Result::Ok(())
    }
}

impl HostGpuQueue for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuQueue> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn submit<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuQueue> ,command_buffers:Vec<Resource<GpuCommandBuffer> > ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn write_buffer<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuQueue> ,buffer:Resource<GpuBuffer> ,buffer_offset:GpuSizeU64,data:Resource<BufferSource> ,data_offset:GpuSizeU64,size:GpuSizeU64,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn write_texture<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuQueue> ,destination:GpuImageCopyTexture,data:Resource<BufferSource> ,data_layout:GpuImageDataLayout,size:GpuExtentD3,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn copy_external_image_to_texture<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuQueue> ,source:GpuImageCopyExternalImage,destination:GpuImageCopyTextureTagged,copy_size:GpuExtentD3,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuQueue>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuBuffer for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuBuffer> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn size<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuBuffer> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuSizeU64> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn usage<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuBuffer> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuBufferUsage> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn map_state<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuBuffer> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuBufferMapState> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn map<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuBuffer> ,mode:GpuMapMode,offset:GpuSizeU64,size:GpuSizeU64,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn get_mapped_range<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuBuffer> ,offset:GpuSizeU64,size:GpuSizeU64,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<u8> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn unmap<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuBuffer> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn destroy<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuBuffer> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuBuffer>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuTexture for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn width<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuTexture> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuIntegerCoordinate> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn height<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuTexture> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuIntegerCoordinate> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn depth_or_array_layers<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuTexture> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuIntegerCoordinate> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn mip_level_count<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuTexture> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuIntegerCoordinate> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn sample_count<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuTexture> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuSizeU32> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn dimension<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuTexture> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuTextureDimension> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn format<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuTexture> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuTextureFormat> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn usage<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuTexture> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuTextureUsage> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn create_view<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuTexture> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuTextureView> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn destroy<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuTexture> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuTexture>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuTextureView for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuTextureView> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuTextureView>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuExternalTexture for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuExternalTexture> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuExternalTexture>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuSampler for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuSampler> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuSampler>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuBindGroupLayout for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuBindGroupLayout> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuBindGroupLayout>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuBindGroup for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuBindGroup> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Option<String> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuBindGroup>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuPipelineLayout for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuPipelineLayout> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuPipelineLayout>) -> Result<()>  {
        self.device.pipeline_layouts.remove(rep.rep() as usize);
        Result::Ok(())
    }
}

impl HostGpuShaderModule for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn get_compilation_info<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuShaderModule> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<GpuCompilationInfo> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuShaderModule>) -> Result<()>  {
        self.device.shader_modules.remove(rep.rep() as usize);
        Result::Ok(())
    }
}

impl HostGpuComputePipeline for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuComputePipeline> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn get_bind_group_layout<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuComputePipeline> ,index:u32,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuBindGroupLayout> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuComputePipeline>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuRenderPipeline for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPipeline> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn get_bind_group_layout<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPipeline> ,index:u32,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuBindGroupLayout> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuRenderPipeline>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuCommandBuffer for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuCommandBuffer> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuCommandBuffer>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuCommandEncoder for WizardRuntimeState<'_> {
    #[must_use]
    #[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn label<'life0,'async_trait>(&'life0 mut self,self_:wasmtime::component::Resource<GpuCommandEncoder> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = wasmtime::Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn begin_render_pass<'life0,'async_trait>(&'life0 mut self,self_:wasmtime::component::Resource<GpuCommandEncoder> ,descriptor:GpuRenderPassDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = wasmtime::Result<wasmtime::component::Resource<GpuRenderPassEncoder> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        let command_encoder_state = self.device.command_encoders.get_mut(self_.rep() as usize).unwrap();

        let color_attachments = descriptor.color_attachments.iter().map(|a| Some(RenderPassColorAttachment {
            view: self.device.texture_views.get(a.view.rep() as usize).unwrap(),
            resolve_target: match a.resolve_target {
                None => None,
                Some(view) => Some(self.device.texture_views.get(view.rep() as usize).unwrap())
            },
            ops: Operations {
                load: match a.load_op {
                    GpuLoadOp::Load => LoadOp::Load,
                    GpuLoadOp::Clear => LoadOp::Clear(match a.clear_value {
                        None => Color::default(),
                        Some(val) => Color {
                            r: val[0],
                            g: val[1],
                            b: val[2],
                            a: val[3],
                        }
                    }),
                },
                store: match a.store_op {
                    GpuStoreOp::Store => true,
                    GpuStoreOp::Discard => false,
                }
            },
        })).collect::<Vec<Option<RenderPassColorAttachment>>>();
        
        let render_pass_encoder = command_encoder_state.encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &color_attachments,
            depth_stencil_attachment: match descriptor.depth_stencil_attachment {
                None => None,
                Some(depth_stencil_attachment) => Some(RenderPassDepthStencilAttachment {
                    view: self.device.texture_views.get(depth_stencil_attachment.view.rep() as usize).unwrap(),
                    depth_ops: Some(Operations {
                        load: match depth_stencil_attachment.depth_load_op {
                            GpuLoadOp::Load => LoadOp::Load,
                            GpuLoadOp::Clear => LoadOp::Clear(depth_stencil_attachment.depth_clear_value),
                        },
                        store: match depth_stencil_attachment.depth_store_op {
                            GpuStoreOp::Store => true,
                            GpuStoreOp::Discard => false,
                        }
                    }),
                    stencil_ops: Some(Operations {
                        load: match depth_stencil_attachment.stencil_load_op {
                            GpuLoadOp::Load => LoadOp::Load,
                            GpuLoadOp::Clear => LoadOp::Clear(depth_stencil_attachment.stencil_clear_value),
                        },
                        store: match depth_stencil_attachment.depth_store_op {
                            GpuStoreOp::Store => true,
                            GpuStoreOp::Discard => false,
                        }
                    }),
                })
            },
        });

        Box::pin(async move {
            Result::Ok(Resource::<GpuRenderPassEncoder>::new_own(command_encoder_state.render_passes.insert(render_pass_encoder) as u32))
        })
    }

    #[must_use]
    #[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn begin_compute_pass<'life0,'async_trait>(&'life0 mut self,self_:wasmtime::component::Resource<GpuCommandEncoder> ,descriptor:Option<GpuComputePassDescriptor> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = wasmtime::Result<wasmtime::component::Resource<GpuComputePassEncoder> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn copy_buffer_to_buffer<'life0,'async_trait>(&'life0 mut self,self_:wasmtime::component::Resource<GpuCommandEncoder> ,source:wasmtime::component::Resource<GpuBuffer> ,source_offset:GpuSizeU64,destination:wasmtime::component::Resource<GpuBuffer> ,destination_offset:GpuSizeU64,size:GpuSizeU64,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = wasmtime::Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn copy_buffer_to_texture<'life0,'async_trait>(&'life0 mut self,self_:wasmtime::component::Resource<GpuCommandEncoder> ,source:GpuImageCopyBuffer,destination:GpuImageCopyTexture,copy_size:GpuExtentD3,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = wasmtime::Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn copy_texture_to_buffer<'life0,'async_trait>(&'life0 mut self,self_:wasmtime::component::Resource<GpuCommandEncoder> ,source:GpuImageCopyTexture,destination:GpuImageCopyBuffer,copy_size:GpuExtentD3,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = wasmtime::Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn copy_texture_to_texture<'life0,'async_trait>(&'life0 mut self,self_:wasmtime::component::Resource<GpuCommandEncoder> ,source:GpuImageCopyTexture,destination:GpuImageCopyTexture,copy_size:GpuExtentD3,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = wasmtime::Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn clear_buffer<'life0,'async_trait>(&'life0 mut self,self_:wasmtime::component::Resource<GpuCommandEncoder> ,buffer:wasmtime::component::Resource<GpuBuffer> ,offset:Option<GpuSizeU64> ,size:Option<GpuSizeU64> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = wasmtime::Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn write_timestamp<'life0,'async_trait>(&'life0 mut self,self_:wasmtime::component::Resource<GpuCommandEncoder> ,query_set:wasmtime::component::Resource<GpuQuerySet> ,query_index:GpuSizeU32,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = wasmtime::Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn resolve_query_set<'life0,'async_trait>(&'life0 mut self,self_:wasmtime::component::Resource<GpuCommandEncoder> ,query_set:wasmtime::component::Resource<GpuQuerySet> ,first_query:GpuSizeU32,query_count:GpuSizeU32,destination:wasmtime::component::Resource<GpuBuffer> ,destination_offset:GpuSizeU64,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = wasmtime::Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn finish<'life0,'async_trait>(&'life0 mut self,self_:wasmtime::component::Resource<GpuCommandEncoder> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = wasmtime::Result<wasmtime::component::Resource<GpuCommandBuffer> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        let command_encoder = self.device.command_encoders.get_mut(self_.rep() as usize).unwrap();
        let command_buffer = command_encoder.encoder.finish();
        
        Box::pin(async move {
            Result::Ok(Resource::<GpuCommandBuffer>::new_own(self.device.command_buffers.insert(command_buffer) as u32))
        })
    }

    fn drop(&mut self,rep:wasmtime::component::Resource<GpuCommandEncoder>) -> wasmtime::Result<()>  {
        self.device.command_encoders.try_remove(rep.rep() as usize);
        Result::Ok(())
    }
}

impl HostGpuComputePassEncoder for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuComputePassEncoder> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn set_pipeline<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuComputePassEncoder> ,pipeline:Resource<GpuComputePipeline> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn dispatch_workgroups<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuComputePassEncoder> ,workgroup_count_x:GpuSizeU32,workgroup_count_y:Option<GpuSizeU32> ,workgroup_count_z:Option<GpuSizeU32> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn dispatch_workgroups_indirect<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuComputePassEncoder> ,indirect_buffer:Resource<GpuBuffer> ,indirect_offset:GpuSizeU64,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn end<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuComputePassEncoder> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn set_bind_group<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuComputePassEncoder> ,index:GpuIndexU32,bind_group:Option<Resource<GpuBindGroup> > ,dynamic_offsets:Option<Vec<GpuBufferDynamicOffset> > ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn set_bind_group_with_data<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuComputePassEncoder> ,index:GpuIndexU32,bind_group:Option<Resource<GpuBindGroup> > ,dynamic_offsets_data:Vec<u32> ,dynamic_offsets_data_start:GpuSizeU64,dynamic_offsets_data_lengh:GpuSizeU32,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn push_debug_group<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuComputePassEncoder> ,group_label:String,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn pop_debug_group<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuComputePassEncoder> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn insert_debug_marker<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuComputePassEncoder> ,marker_label:String,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuComputePassEncoder>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuRenderPassEncoder for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn set_pipeline<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,pipeline:Resource<GpuRenderPipeline> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn set_index_buffer<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,buffer:Resource<GpuBuffer> ,index_format:GpuIndexFormat,offset:Option<GpuSizeU64> ,size:Option<GpuSizeU64> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn set_vertex_buffer<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,slot:GpuIndexU32,buffer:Option<Resource<GpuBuffer> > ,offset:Option<GpuSizeU64> ,size:Option<GpuSizeU64> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn draw<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,vertex_count:GpuSizeU32,instance_count:Option<GpuSizeU32> ,first_vertex:Option<GpuSizeU32> ,first_instance:Option<GpuSizeU32> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn draw_indexed<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,index_count:GpuSizeU32,instance_count:Option<GpuSizeU32> ,first_index:Option<GpuSizeU32> ,base_vertex:Option<GpuSignedOffsetS32> ,first_instance:Option<GpuSizeU32> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn draw_indirect<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,indirect_buffer:Resource<GpuBuffer> ,indirect_offset:GpuSizeU64,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn draw_indexed_indirect<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,indirect_buffer:Resource<GpuBuffer> ,indirect_offset:GpuSizeU64,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn set_viewport<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,x:f32,y:f32,width:f32,height:f32,min_depth:f32,max_depth:f32,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn set_scissor_rect<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,x:GpuIntegerCoordinate,y:GpuIntegerCoordinate,width:GpuIntegerCoordinate,height:GpuIntegerCoordinate,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn set_blend_constant<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,color:GpuColor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn set_stencil_reference<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,reference:GpuStencilValue,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn begin_occlusion_query<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,query_index:GpuSizeU32,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn end_occlusion_query<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn execute_bundles<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,bundles:Vec<Resource<GpuRenderBundle> > ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn end<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn set_bind_group<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,index:GpuIndexU32,bind_group:Option<Resource<GpuBindGroup> > ,dynamic_offsets:Option<Vec<GpuBufferDynamicOffset> > ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn set_bind_group_with_data<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,index:GpuIndexU32,bind_group:Option<Resource<GpuBindGroup> > ,dynamic_offsets_data:Vec<u32> ,dynamic_offsets_data_start:GpuSizeU64,dynamic_offsets_data_lengh:GpuSizeU32,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn push_debug_group<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,group_label:String,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn pop_debug_group<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn insert_debug_marker<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderPassEncoder> ,marker_label:String,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuRenderPassEncoder>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuRenderBundle for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn label<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderBundle> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<String> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuRenderBundle>) -> Result<()>  {
        todo!()
    }
}

impl HostGpuRenderBundleEncoder for WizardRuntimeState<'_> {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn finish<'life0,'async_trait>(&'life0 mut self,self_:Resource<GpuRenderBundleEncoder> ,descriptor:GpuRenderBundleDescriptor,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Resource<GpuRenderBundle> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn drop(&mut self,rep:Resource<GpuRenderBundleEncoder>) -> Result<()>  {
        todo!()
    }
}
