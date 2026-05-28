# Resolved Research: 1.3 Shared Memory Halo Tiling + Workgroup Sizing

**Date Researched**: 2026-05-25
**Priority**: Highest
**Status**: Resolved — Standard, well-documented pattern with concrete examples

## Core Recommendation

**Use 16×16 workgroups + explicit halo loading into `var<workgroup>` shared memory** for all neighbor-gather passes (Gravity, JFA steps, Capillary, future heat/scent diffusion).

This is the highest-ROI optimization for cellular automata and image-processing-style compute shaders on modern NVIDIA GPUs, including the DGX Spark's Blackwell architecture.

## Why This Matters on DGX Spark

Neighbor-gather workloads (reading 8–24 neighbors per cell) are heavily bandwidth-bound. Global memory loads dominate execution time. Loading a small halo into fast shared memory once per workgroup dramatically reduces VRAM traffic and improves occupancy/latency hiding on unified memory systems.

## Standard Pattern (Confirmed Across Sources)

1. Declare a shared memory cache sized for the workgroup + halo.
2. Each thread loads its own cell + the halo cells it is responsible for.
3. Call `workgroupBarrier()`.
4. All computation reads from the fast shared cache instead of global memory.

**Concrete Example** (adapted from modern WebGPU reaction-diffusion / image effect tutorials):

```wgsl
@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) id: vec3<u32>,
        @builtin(local_invocation_id) local_id: vec3<u32>) {

    let x = id.x;
    let y = id.y;
    let lx = local_id.x;
    let ly = local_id.y;

    // Shared memory cache with 1-cell halo → 18×18
    var<workgroup> cache: array<array<u32, 18>, 18>;

    // Load center + halo into shared memory
    // (each thread loads its own cell; edge threads also load halo)
    let global_idx = get_idx(x, y);
    cache[ly + 1][lx + 1] = grid_in[global_idx];

    // Halo loading (simplified — full version handles all 4 edges + corners)
    if (lx == 0 && x > 0) {
        cache[ly + 1][0] = grid_in[get_idx(x - 1, y)];
    }
    if (lx == 15 && x < W - 1) {
        cache[ly + 1][17] = grid_in[get_idx(x + 1, y)];
    }
    // ... similar for top/bottom edges and corners

    workgroupBarrier();   // Critical — ensures all loads complete

    // Now read neighbors from fast cache instead of global memory
    let center = cache[ly + 1][lx + 1];
    let up     = cache[ly][lx + 1];
    let down   = cache[ly + 2][lx + 1];
    // ... etc for all 8 neighbors

    // Perform gravity / JFA / capillary logic using cached values
}
```

## Workgroup Size Recommendation: 16×16

- **16×16 is the most frequently recommended size** for 2D image-like and cellular automata workloads on NVIDIA GPUs.
- Good balance between occupancy, register usage, and halo loading efficiency.
- 8×8 is too small (more barrier overhead relative to work).
- 32×8 or larger can reduce occupancy on some architectures due to higher shared memory or register pressure.

## Expected Benefits for AntVerse

- Significant reduction in global memory bandwidth for JFA (which does many neighbor gathers per pass) and gravity/capillary.
- Better performance at 2048²–4096²+ grid sizes on the DGX Spark's unified memory.
- Aligns with the "no compromises" philosophy — we can afford the cleaner, more cache-friendly code because shared memory + high internal bandwidth removes the usual PC GPU constraints.

## Implementation Notes

- Start by implementing halo loading for the most bandwidth-heavy pass first (JFA steps).
- The halo loading code must correctly handle grid boundaries (clamp or skip out-of-bounds loads).
- For the first implementation, loading a full 18×18 halo even on edge workgroups (with boundary checks) is acceptable for correctness; optimize later if needed.

**Conclusion**: 16×16 workgroups + shared memory halo loading is a mature, well-understood optimization with direct examples in modern WebGPU codebases. It should be adopted as the default for all neighbor-intensive shaders in AntVerse.
