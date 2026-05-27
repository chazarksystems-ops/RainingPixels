// src/shaders/copy.wgsl
// Antverse Phase 0 — Minimal identity copy compute shader (ping-pong sanity check)
//
// This proves:
// - Correct double-buffered storage buffer binding
// - 16x16 workgroup dispatch covering the entire 2048² grid
// - SimParams uniform upload and indexing
// - That we can run a compute pass inside a single CommandEncoder and flip buffers
//
// In Phase 0.2 this file will be replaced / extended with the real symmetric gravity shader.
// The copy pass remains useful as a "no-op physics" mode for debugging render and event ledger paths.

struct SimParams {
    width: u32,
    height: u32,
    frame: u32,
    seed: u32,
    event_ledger_index: u32,
    _pad: vec3<u32>,
}

@group(0) @binding(0) var<storage, read> grid_in: array<u32>;
@group(0) @binding(1) var<storage, read_write> grid_out: array<u32>;
@group(0) @binding(2) var<uniform> params: SimParams;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let x = gid.x;
    let y = gid.y;
    let w = params.width;
    let h = params.height;

    if (x >= w || y >= h) {
        return;
    }

    let idx = y * w + x;
    grid_out[idx] = grid_in[idx];
}
