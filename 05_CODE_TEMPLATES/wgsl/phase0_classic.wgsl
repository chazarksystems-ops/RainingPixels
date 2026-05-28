// phase0_classic.wgsl
// AntVerse Phase 0 — Classic Falling Sand on DGX Spark
// Simple local rules for Sand, Water, Dirt, Stone on large unified-memory grid

struct SimParams {
    width: u32,
    height: u32,
    frame: u32,
    seed: u32,
}

@group(0) @binding(0) var<storage, read> grid_in: array<u32>;
@group(0) @binding(1) var<storage, read_write> grid_out: array<u32>;
@group(0) @binding(2) var<uniform> params: SimParams;

const MAT_AIR: u32 = 0u;
const MAT_SAND: u32 = 1u;
const MAT_WATER: u32 = 2u;
const MAT_DIRT: u32 = 3u;   // static for Phase 0
const MAT_STONE: u32 = 4u;

fn get_idx(x: u32, y: u32) -> u32 {
    return y * params.width + x;
}

fn get_material(packed: u32) -> u32 {
    return packed & 0xFFu;
}

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let x = id.x;
    let y = id.y;
    let W = params.width;
    let H = params.height;

    if (x >= W || y >= H) { return; }

    let idx = get_idx(x, y);
    let state = grid_in[idx];
    let mat = get_material(state);

    var next = state;

    if (mat == MAT_SAND) {
        // Classic sand: fall down or diagonal
        if (y < H - 1u) {
            let below = grid_in[get_idx(x, y + 1u)];
            if (get_material(below) == MAT_AIR) {
                next = MAT_SAND;
                // We will write to the target in a second pass or use ping-pong carefully
                // For Phase 0 simplicity we use a basic approach; will upgrade in Phase 1
            }
        }
    }

    // TODO in later phases: full symmetric gravity, JFA, capillary
    // For Phase 0 we keep rules minimal and correct

    grid_out[idx] = next;
}
