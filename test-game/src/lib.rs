wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "runtime",
    path: "../runtime-host/wit",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        "wizard:runtime/guest": Game
    },
});

use once_cell::sync::OnceCell;

use crate::exports::wizard::runtime::guest::Guest;
use crate::wizard::runtime::renderer::*;
use crate::wizard::runtime::debug::log;

struct Game;

static RENDER_PIPELINE: OnceCell<GpuRenderPipeline> = OnceCell::new();

impl Guest for Game {
    /// Say hello!
    fn init() {
        let gpu = crate::wizard::runtime::renderer::get_gpu();
        let adapter = gpu.request_adapter();
        let device = adapter.request_device();

        let device_name = device.label();
        log(&device_name);

        let shader = device.create_shader_module(&GpuShaderModuleDescriptor {
            label: None,
            code: "
                @vertex
                fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                    let x = f32(i32(in_vertex_index) - 1);
                    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
                    return vec4<f32>(x, y, 0.0, 1.0);
                }
                
                @fragment
                fn fs_main() -> @location(0) vec4<f32> {
                    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
                }
            ".to_string(),
            hints: vec![]
        });

        let pipeline_layout = device.create_pipeline_layout(&GpuPipelineLayoutDescriptor {
            bind_group_layouts: vec![]
        });

        RENDER_PIPELINE.set(device.create_render_pipeline(&GpuRenderPipelineDescriptor {
            layout: GpuLayout::Pipeline(&pipeline_layout),
            vertex: &GpuVertexState {
                module: &shader,
                entry_point: "vs_main".to_owned(),
                constants: Vec::new(),
                buffers: None
            },
            fragment: Some(&GpuFragmentState {
                module: &shader,
                entry_point: "fs_main".to_owned(),
                constants: Vec::new(),
                targets: Vec::new()
            }),
            primitive: Some(GpuPrimitiveState {
                topology: Some(GpuPrimitiveTopology::TriangleList),
                strip_index_format: None,
                front_face: Some(GpuFrontFace::Ccw),
                cull_mode: GpuCullMode::Back,
                unclipped_depth: false  
            }),
            depth_stencil: None,
            multisample: Some(GpuMultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            })
        }));
    }

    fn render(device: &GpuDevice, view: GpuTextureView) -> Option<GpuCommandBuffer> {
        log("render");

        let mut encoder = device.create_command_encoder(&GpuCommandEncoderDescriptor {
            label: None
        });

        let mut render_pass = encoder.begin_render_pass(&GpuRenderPassDescriptor {
            color_attachments: vec![&GpuRenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                load_op: GpuLoadOp::Clear,
                store_op: GpuStoreOp::Store,
                clear_value: Some(vec![0.0f64, 1.0f64, 0.0f64, 1.0f64]).as_ref()
            }],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            max_draw_count: None
        });

        render_pass.set_pipeline(RENDER_PIPELINE.get().unwrap());
        render_pass.draw(3, Some(1), Some(0), Some(0));

        Some(encoder.finish())
    }
}
