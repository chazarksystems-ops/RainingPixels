// src/main.rs
// Antverse Phase 0 Demo — Basic Ping-Pong Grid + Render Loop (Step 3)
//
// This is the first runnable milestone of the Phase 0 demo.
// It demonstrates:
// - Clean wgpu 0.19 + winit 0.29 bootstrap on Windows / NVIDIA (DGX Spark target)
// - Two storage buffers ping-ponging at 2048² resolution
// - Minimal identity compute pass (copy shader) inside a single CommandEncoder
// - Full-screen triangle render sampling the current grid with a restrained material palette
// - Stable interactive window (target 60+ FPS on modern hardware)
//
// Per 02_ARCHITECTURE/PHASE0_DEMO_IMPLEMENTATION_PLAN.md
// All later phases (brush, symmetric gravity, events, mass verification) are designed
// to slot into this exact loop and module structure without architectural rework.
//
// Visual goals follow 00_PROJECT_VISION/UI_UX_VISION.md and
// 04_DESIGN_DECISIONS/VISUAL_AND_INTERACTION_DIFFERENCES.md.

mod simulation;
mod render;
mod brush;
mod utils;

use crate::simulation::{SimulationState, SimParams};
use crate::render::RenderContext;
use crate::simulation::pipeline::{self, ComputeContext};

use std::sync::Arc;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};
use wgpu;

fn main() {
    // env_logger can be added later for wgpu debug output if desired.
    // Keeping the dependency list exactly as specified in 06_DEPENDENCIES_AND_REPOS/wgpu_winit.md for Phase 0.

    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Antverse — Phase 0 Demo (2048² ping-pong)")
            .with_inner_size(winit::dpi::PhysicalSize::new(1024, 1024))
            .build(&event_loop)
            .unwrap(),
    );

    // === wgpu initialization (pollster for sync main) ===
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let surface = instance.create_surface(window.clone()).unwrap();

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .expect("Failed to find a suitable wgpu adapter (HighPerformance preferred for DGX Spark class hardware)");

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("antverse-device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
        },
        None,
    ))
    .expect("Failed to create wgpu device");

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: 1024,
        height: 1024,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    // === Simulation state at target resolution (2048² for Phase 0.0) ===
    let sim_width = SimParams::DEFAULT_WIDTH;
    let sim_height = SimParams::DEFAULT_HEIGHT;
    let mut state = SimulationState::new(sim_width, sim_height, 0xDEADBEEF);
    state.initialize_empty();

    // Seed a simple visible test pattern so we can immediately see the render working
    // (a small block of sand and water in the upper-left). Real world seeding comes later.
    for y in 400..600 {
        for x in 400..900 {
            let idx = state.idx(x, y);
            if x < 650 {
                state.grid[idx] = crate::simulation::Cell::new(
                    crate::simulation::MAT_SAND,
                    0,
                    0,
                    0,
                );
            } else {
                state.grid[idx] = crate::simulation::Cell::new(
                    crate::simulation::MAT_WATER,
                    0,
                    200,
                    0,
                );
            }
        }
    }

    // === GPU buffers ===
    let buffer_size = (sim_width * sim_height * std::mem::size_of::<u32>() as u32) as u64;

    let grid_buffers = [
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("grid-0"),
            size: buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }),
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("grid-1"),
            size: buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }),
    ];

    // Upload initial CPU grid to buffer 0 (the starting "current" buffer)
    queue.write_buffer(&grid_buffers[0], 0, bytemuck::cast_slice(&state.grid));

    let params_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sim-params"),
        size: std::mem::size_of::<SimParams>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Initial params upload
    queue.write_buffer(&params_buffer, 0, bytemuck::bytes_of(&state.params));

    // === Contexts ===
    let render_ctx = RenderContext::new(&device, surface_format);
    let compute_ctx = ComputeContext::new(&device);

    // === Main loop state ===
    let mut current_grid_index: usize = 0; // which of the two grid_buffers is the "latest written"
    let mut last_frame_time = std::time::Instant::now();
    let mut frame_count: u64 = 0;

    // === Event loop ===
    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                println!("Antverse Phase 0.0 demo exiting cleanly.");
                elwt.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                },
                ..
            } => {
                println!("Escape pressed — exiting Phase 0.0 demo.");
                elwt.exit();
            }
            Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => {
                if new_size.width > 0 && new_size.height > 0 {
                    config.width = new_size.width;
                    config.height = new_size.height;
                    surface.configure(&device, &config);
                }
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                // === Simulation step (identity copy for now) ===
                current_grid_index = pipeline::submit_frame(
                    &device,
                    &queue,
                    &compute_ctx,
                    &render_ctx,
                    &mut state,
                    &grid_buffers,
                    &params_buffer,
                    current_grid_index,
                );

                // === Render ===
                let output = match surface.get_current_texture() {
                    Ok(o) => o,
                    Err(e) => {
                        eprintln!("Surface error: {:?}", e);
                        return;
                    }
                };
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let render_bind_group = render_ctx.create_bind_group(
                    &device,
                    &grid_buffers[current_grid_index],
                    &params_buffer,
                );

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("render-encoder"),
                });

                render::draw(
                    &mut encoder,
                    &render_ctx,
                    &render_bind_group,
                    &view,
                    config.width,
                    config.height,
                );

                queue.submit(Some(encoder.finish()));
                output.present();

                // === FPS / status reporting (simple) ===
                frame_count += 1;
                let now = std::time::Instant::now();
                if now.duration_since(last_frame_time).as_secs_f32() > 2.0 {
                    let fps = frame_count as f32 / now.duration_since(last_frame_time).as_secs_f32();
                    println!(
                        "Antverse Phase 0.0 — {}x{} grid | ~{:.1} FPS | frame {}",
                        sim_width, sim_height, fps, state.params.frame
                    );
                    last_frame_time = now;
                    frame_count = 0;
                }

                // Request the next frame immediately for continuous animation
                window.request_redraw();
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
