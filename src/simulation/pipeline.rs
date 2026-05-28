// src/simulation/pipeline.rs
// Antverse Phase 0 — Compute pipeline (basic ping-pong copy for Step 3 sanity)
// Per 02_ARCHITECTURE/PHASE0_DEMO_IMPLEMENTATION_PLAN.md
//
// In this step the pipeline only runs an identity copy pass. This proves the entire
// linear CommandEncoder path, buffer ping-pong, and parameter upload before we replace
// the shader body with real symmetric reverse-pull gravity in Phase 0.2.
//
// The public API (create_compute_pipeline, submit_frame) is designed to be extended
// with brush + gravity + event emission passes without changing the call site in main.

use crate::simulation::{SimParams, SimulationState};
use wgpu;

pub struct ComputeContext {
    pub copy_pipeline: wgpu::ComputePipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl ComputeContext {
    pub fn new(device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("antverse-copy"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/copy.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compute-bgl"),
            entries: &[
                // grid_in
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // grid_out
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // SimParams
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("compute-pipeline-layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let copy_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("antverse-copy-pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
        });

        Self {
            copy_pipeline,
            bind_group_layout,
        }
    }

    pub fn create_bind_group(
        &self,
        device: &wgpu::Device,
        grid_in: &wgpu::Buffer,
        grid_out: &wgpu::Buffer,
        params_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("compute-bind-group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: grid_in.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: grid_out.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        })
    }
}

/// Executes one "frame" using the current copy (identity) shader.
/// This is the minimal runnable physics step for Phase 0.0 Step 3.
///
/// In later phases this function will:
/// - Optionally run a brush stamp pass
/// - Run the real gravity shader (Phase 0.2)
/// - Emit FrameSummary + notables into the active EventLedger (Phase 0.3)
/// - Perform a single buffer flip at the very end
pub fn submit_frame(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    compute_ctx: &ComputeContext,
    _render_ctx: &crate::render::RenderContext,
    state: &mut SimulationState,
    grid_buffers: &[wgpu::Buffer; 2],
    params_buffer: &wgpu::Buffer,
    current_read_index: usize,
) -> usize {
    // Update CPU-side params for this frame (frame counter + ledger slot)
    state.begin_frame();

    // Upload the latest SimParams (small uniform)
    queue.write_buffer(params_buffer, 0, bytemuck::bytes_of(&state.params));

    // Choose which buffer is "in" and which is "out" for this dispatch
    let read_idx = current_read_index;
    let write_idx = 1 - current_read_index;

    let bind_group = compute_ctx.create_bind_group(
        device,
        &grid_buffers[read_idx],
        &grid_buffers[write_idx],
        params_buffer,
    );

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("antverse-frame-encoder"),
    });

    // Compute pass — identity copy (will become real gravity)
    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("copy-pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&compute_ctx.copy_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);

        let dispatch_x = (state.params.width + 15) / 16;
        let dispatch_y = (state.params.height + 15) / 16;
        compute_pass.dispatch_workgroups(dispatch_x, dispatch_y, 1);
    }

    // (Future) event ledger clear + emission passes would go here inside the same encoder.

    // Submit
    queue.submit(Some(encoder.finish()));

    // Return the new "current visible" buffer index (the one we just wrote into)
    write_idx
}
