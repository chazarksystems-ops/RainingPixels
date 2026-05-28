// src/shaders/render.wgsl
// Antverse Phase 0 — Full-screen triangle + basic material color mapping
//
// Renders the current physics grid (storage buffer) directly.
// Color mapping is deliberately clean and restrained to match the visual goals in
// 00_PROJECT_VISION/UI_UX_VISION.md and 04_DESIGN_DECISIONS/VISUAL_AND_INTERACTION_DIFFERENCES.md
// (no garish neon, good contrast, water vs wet dirt distinction even in Phase 0).
//
// Vertex stage: generates a full-screen triangle from vertex index.
// Fragment stage: computes UV, maps to grid texel, looks up material, outputs color.

struct SimParams {
    width: u32,
    height: u32,
    frame: u32,
    seed: u32,
    event_ledger_index: u32,
    _pad: vec3<u32>,
}

@group(0) @binding(0) var<storage, read> grid: array<u32>;
@group(0) @binding(1) var<uniform> params: SimParams;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // Full-screen triangle (no vertex buffer needed)
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0)
    );
    let p = pos[vertex_index];
    var out: VertexOutput;
    out.position = vec4<f32>(p, 0.0, 1.0);
    out.uv = p * vec2<f32>(0.5, -0.5) + vec2<f32>(0.5);
    return out;
}

fn material_color(mat: u32, moisture: u32) -> vec4<f32> {
    // Clean, restrained palette (Phase 0 baseline)
    if (mat == 0u) { return vec4<f32>(0.08, 0.08, 0.10, 1.0); }           // Air — near black
    if (mat == 1u) { return vec4<f32>(0.85, 0.75, 0.45, 1.0); }           // Sand — warm ochre
    if (mat == 2u) { return vec4<f32>(0.20, 0.45, 0.85, 1.0); }           // Water — clear blue
    if (mat == 3u) {
        // Dirt: darken with moisture (wet dirt distinction visible even in Phase 0)
        let wetness = f32(moisture) / 255.0;
        let base = vec3<f32>(0.35, 0.25, 0.15);
        let wet = vec3<f32>(0.18, 0.14, 0.10);
        let c = mix(base, wet, wetness * 0.7);
        return vec4<f32>(c, 1.0);
    }
    if (mat == 4u) { return vec4<f32>(0.45, 0.45, 0.48, 1.0); }           // Stone — cool gray
    return vec4<f32>(1.0, 0.0, 1.0, 1.0); // error magenta
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let w = f32(params.width);
    let h = f32(params.height);
    let px = u32(clamp(in.uv.x * w, 0.0, w - 1.0));
    let py = u32(clamp(in.uv.y * h, 0.0, h - 1.0));
    let idx = py * params.width + px;
    let packed = grid[idx];
    let mat = packed & 0xFFu;
    let moisture = (packed >> 16u) & 0xFFu;
    return material_color(mat, moisture);
}
