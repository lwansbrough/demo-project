use std::sync::{Arc, Mutex};

use anyhow::Result;
use once_cell::sync::OnceCell;
use pollster;
use futures::executor;

mod common;
mod native;

pub use {
    common::*
};

use uuid::Uuid;
use wasmtime::component::Resource;
use winit::{
    event_loop::{EventLoop, ControlFlow, EventLoopBuilder},
    window::{Window, WindowBuilder}, event::{WindowEvent, Event}, dpi::PhysicalSize, error::EventLoopError
};

use crate::{wizard::runtime::renderer::{GpuRenderPipeline, GpuComputePipeline}, Game};

enum GameEvent {
    GameInit
}

async fn run_loop(event_loop: EventLoop<GameEvent>, window: Window) -> Result<(), EventLoopError> {
    let window_size = window.inner_size();
    let instance = wgpu::Instance::default();
    let mut surface = unsafe { instance.create_surface(&window).unwrap() };
    let mut adapter = executor::block_on(instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        }))
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (mut device, mut queue) = executor::block_on(adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        ))
        .expect("Failed to create device");
    
    let context = crate::Context::new(
        Uuid::new_v4(),
        window_size,
        adapter,
        device,
        queue,
        surface
    );

    let mut game = Game::new("../test-game/test-game.wasm").unwrap();
    let (mut store, runtime) = game.init(context).await.expect("Game didn't initialize");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(event) => {

            },
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // game.resize(size);
            }
            Event::RedrawRequested(_) => {
                pollster::block_on(game.render(&mut store, &runtime)).unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    })
}

pub fn run() {
    let event_loop = EventLoopBuilder::<GameEvent>::with_user_event()
        .build()
        .unwrap();

    let window = WindowBuilder::new()
        .with_title("Game")
        .build(&event_loop)
        .unwrap();

    // #[cfg(not(target_arch = "wasm32"))]
    // {
        // env_logger::init();
        // Temporarily avoid srgb formats for the swapchain on the web
        pollster::block_on(run_loop(event_loop, window));
    // }
    // #[cfg(target_arch = "wasm32")]
    // {
    //     std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    //     console_log::init().expect("could not initialize logger");
    //     use winit::platform::web::WindowExtWebSys;
    //     // On wasm, append the canvas to the document body
    //     web_sys::window()
    //         .and_then(|win| win.document())
    //         .and_then(|doc| doc.body())
    //         .and_then(|body| {
    //             body.append_child(&web_sys::Element::from(window.canvas()))
    //                 .ok()
    //         })
    //         .expect("couldn't append canvas to document body");
        
    //     wasm_bindgen_futures::spawn_local(run_loop(event_loop, window));
    // }
}
