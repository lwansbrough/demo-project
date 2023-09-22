#![feature(slice_pattern)]

use anyhow::{Ok, Result};
use futures::{executor,Future};

pub mod runtime;

wasmtime::component::bindgen!({
    path: "wit",
    async: true
});

use runtime::gpu::{GpuAdapterState, GpuDeviceState};
use uuid::Uuid;
use wasmtime::{Engine, Config, component::{Component, Linker, InstancePre, Resource}, Store, AsContext};
use wasmtime_wasi::preview2::{
    Table,
    WasiCtx,
    WasiCtxBuilder,
    WasiView
};
use wgpu::{Adapter, Device, Queue, Surface, SurfaceConfiguration};
use winit::{window::Window, event_loop::{EventLoop, ControlFlow}, event::{Event, WindowEvent}, dpi::PhysicalSize};
pub use wizard::runtime::*;
pub use exports::wizard::runtime::guest;
use wizard::runtime::renderer::{GpuRenderPipeline, GpuDevice, GpuTextureView};

pub struct WizardRuntimeState<'a> {
    pub id: Uuid,
    pub window_size: PhysicalSize<u32>,
    pub adapter: runtime::gpu::GpuAdapterState,
    pub device: runtime::gpu::GpuDeviceState<'a>,
    pub queue: Queue,
    pub surface: Surface
}

pub struct Context<'a> {
    wasi_ctx: WasiCtx,
    table: Table,
    state: WizardRuntimeState<'a>
}

impl WasiView for Context<'_> {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi_ctx
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl Context<'_> {
    pub fn new(id: Uuid, window_size: PhysicalSize<u32>, adapter: Adapter, device: Device, queue: Queue, surface: Surface) -> Self {
        let mut table = Table::new();
        Context {
            wasi_ctx: WasiCtxBuilder::new()
                .inherit_stderr()
                .inherit_stdout()
                .build(&mut table)
                .unwrap(),
            table,
            state: WizardRuntimeState {
                id,
                window_size,
                adapter: GpuAdapterState::new(adapter),
                device: GpuDeviceState::new(device),
                queue,
                surface
            }
        }
    }

    pub fn state(&self) -> &WizardRuntimeState {
        &self.state
    }

    pub fn state_mut(&'_ mut self) -> &'_ mut WizardRuntimeState {
        &mut self.state
    }
}

/// Game is used to run wasm component

pub struct Game<'a> {
    path: String,
    engine: Engine,
    instance_pre: InstancePre<Context<'a>>,
    surface_config: Option<SurfaceConfiguration>
}

impl std::fmt::Debug for Game<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Game").field("path", &self.path).finish()
    }
}

impl<'a> Game<'a> {
    /// new game
    pub fn new(path: &str) -> Result<Game> {
        let binary = std::fs::read(path)?;
        Self::from_binary(&binary)
    }

    // from_binary is used to create game from bytes
    pub fn from_binary(bytes: &[u8]) -> Result<Game<'_>> {
        // create component
        let mut config = Config::new();
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        config.async_support(true);
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;
        let component = Component::from_binary(&engine, &bytes)?;

        // create linker
        let mut linker: Linker<Context<'_>> = Linker::new(&engine);
        // init wasi context
        wasmtime_wasi::preview2::command::add_to_linker(&mut linker)
            .expect("add wasmtime_wasi::preview2 failed");
        Runtime::add_to_linker(&mut linker, Context::state_mut)?;

        Ok(Self {
            path: "bytes".to_owned(),
            engine,
            instance_pre: linker.instantiate_pre(&component)?,
            surface_config: None
        })
    }

    /// handle_request is used to handle http request
    pub async fn init(&mut self, context: Context<'a>) -> Result<(Store<Context<'a>>, Runtime), anyhow::Error> {
        // create store
        let mut store = Store::new(&self.engine, context);

        // get exports and call handle_request
        let (exports, _instance) =
            Runtime::instantiate_pre(&mut store, &self.instance_pre).await?;
        
        exports
            .wizard_runtime_guest()
            .call_init(&mut store).await?;

        let surface_caps = store.data().state().surface.get_capabilities(&store.data().state().adapter.inner);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);

        self.surface_config = Some(wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: store.data().state().window_size.width,
            height: store.data().state().window_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![]
        });

        let device = &store.data().state().device.inner;

        store.data().state().surface.configure(device, self.surface_config.as_ref().unwrap());

        Ok((store, exports))
    }

    pub async fn render(&'a self, mut store: &'a mut Store<Context<'a>>, runtime: &Runtime) -> Result<(), anyhow::Error> {
        let output = store.data().state().surface.get_current_texture().expect("No frame");
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let view_resource = Resource::<GpuTextureView>::new_own(
            store.data_mut().state_mut().device.texture_views.insert(view) as u32
        );
        let gpu_device = Resource::<GpuDevice>::new_borrow(0);

        let gpu_command_buffer_id = {
            if let Some(gpu_command_buffer) = runtime.wizard_runtime_guest().call_render(&mut store, gpu_device, view_resource).await.expect("Game didn't render") {
                Some(gpu_command_buffer.rep() as usize)
            } else {
                None
            }
        };

        if let Some(gpu_command_buffer_id) = gpu_command_buffer_id {
            let command_buffer = store.data_mut().state_mut().device.command_buffers.remove(gpu_command_buffer_id);
            store.data_mut().state().queue.submit(std::iter::once(command_buffer));
            output.present();
        }

        Ok(())
    }

    pub async fn resize(&mut self, store: &mut Store<Context<'_>>, device: &mut Device, surface: &mut Surface, size: PhysicalSize::<u32>) {
        if let Some(config) = self.surface_config.as_mut() {
            config.width = size.width;
            config.height = size.height;

            surface.configure(&device, &config);
        }
    }
}
