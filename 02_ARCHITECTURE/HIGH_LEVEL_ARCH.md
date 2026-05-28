# High-Level Architecture — AntVerse

## Guiding Principle

"Local rules, global emergence, DGX-native scale."

Every major component must preserve the classic falling sand aesthetic (simple per-cell decisions based on immediate neighbors) while exploiting the DGX Spark's unified memory to support much larger grids and richer state.

## Core Data Model

**Primary Simulation Buffer**
- One (or ping-ponged) 2D array of `u32` (or `u64` later) per cell.
- Bit-packed or small struct layout (see `04_DESIGN_DECISIONS/STATE_PACKING.md`).
- Lives permanently in unified memory.

**Auxiliary Buffers** (all same resolution, kept resident):
- `jfa_current` / `jfa_next` — Jump Flood Algorithm distance-to-anchor field (packed u32: y<<16 | x or INF).
- `scent_field` (future) — low-precision diffusion field for ant-like agents.
- `event_ledger` (ring buffer or per-frame summary) — lightweight events emitted by simulation for host consumption.
- Optional: heat, pressure, velocity fields when we reach thermodynamics layers.

Because of 128 GB unified memory, having 6–8 full-resolution auxiliary buffers is not a meaningful constraint until we reach extreme grid sizes (8K²+).

## Simulation Pipeline (Linear, Single Flip at End)

The update must be a clean linear sequence with **one buffer swap at the very end** of the frame (see Audit point #1 from previous analysis). No mid-chain flipping.

Recommended order per frame:

1. **Brush / User Input** (optional compute stamp or CPU staging copy)
2. **JFA Structural Pass** (Init → log₂(N) Steps → Collapse)
   - Reads current physics grid
   - Writes JFA buffers + produces post-collapse "next" grid
3. **Gravity + Advection Pass** (symmetric reverse-pull)
   - Reads post-collapse grid
   - Writes gravity result
4. **Capillary + Diffusion Pass**
   - Reads gravity result
   - Writes final next grid (moisture, scent, evaporation)
5. **Telemetry / Event Reduction** (optional lightweight pass or host read)
6. **Single buffer flip** (`is_flipped = !is_flipped`)
7. **Render** (full-screen triangle sampling the *current* final buffer)

This ordering ensures structural collapse happens *before* gravity acts on the newly created sand, and capillary acts on the post-gravity state.

## Memory & Binding Strategy (Recommended)

**Option A (Simple & Robust — Recommended for Phase 0–2)**:
- Pre-create a modest number of bind groups (as in your Phase 3 complete code).
- Use clear naming: `bind_physics_ab`, `bind_jfa_init_a`, etc.
- Document ownership explicitly.

**Option B (More Scalable — Future)**:
- Single "mega" bind group containing all buffers + a small uniform `u32 read_index`.
- All shaders read `grid[read_index]` and write `grid[1 - read_index]`.
- Only the uniform changes between passes. Extremely clean for many layers.

We will start with Option A (your existing code is already close) and migrate to Option B when the number of passes grows.

## Workgroup & Dispatch Strategy

- **Workgroup size**: 16×16 is excellent for your target (good occupancy on Blackwell, easy halo loading).
- **Halo / Shared Memory**: For gravity and JFA neighborhood reads, load a 18×18 tile into `workgroup var` to reduce global memory traffic. This is a high-ROI optimization on unified memory architectures.
- **Dispatch**: `(width + 15) / 16` × `(height + 15) / 16`

## Render Path

- Simple full-screen triangle (your existing `render.wgsl` is perfect).
- Fragment shader samples the final physics buffer and maps material + moisture/scent to color.
- Future enhancements:
  - Subtle flow animation using frame + cell velocity (when we have it).
  - SDF visualization from the JFA field (beautiful structural rendering).
  - Overlay modes (scent heatmaps, collapse danger zones).

## Host ↔ GPU Boundary

- **Zero-copy friendly** on DGX Spark.
- Host can map the final physics buffer or a small summary buffer for telemetry.
- Event ledger can be a ring buffer in unified memory that the GPU appends to and the CPU consumes.
- This is the foundation for feeding live simulation state into CedeSystem, Entelechy, or LLM agents.

## Scaling Plan (Grid Size)

| Phase | Target Grid | Notes |
|-------|-------------|-------|
| 0     | 2048×2048   | Classic falling sand, prove responsiveness |
| 1     | 2048×2048   | Add symmetric gravity + JFA |
| 2     | 4096×4096   | Full structural + capillary, comfortable on 128 GB |
| 3+    | 4096×4096 or 8192×8192 | Add scent, heat, telemetry layers |

We design the code so changing `GRID_WIDTH` / `GRID_HEIGHT` constants (or runtime uniforms) is trivial.

## Determinism Requirements

- All random tie-breaking uses `hash_spatial_sync` with a **global frame seed** from `SimParams` (not per-thread coordinate mixing that could vary by warp execution order).
- JFA distance uses **fixed-point integer math** (squared Euclidean) instead of `length(f32)`.
- Gravity and capillary rules are purely deterministic functions of neighbor state + global seed.
- Mass conservation is enforced by the symmetric air/water design (every pull has a corresponding vacate).

## Error & Edge Case Handling

- Boundary conditions: hard walls or periodic (document choice per layer).
- First-frame JFA convergence: either accept partial collapse on frame 0 or pre-stabilize important structures.
- Out-of-bounds neighbor reads: explicit clamping in WGSL (your existing `get_neighbor_mat` pattern is good).
- Integer overflow in fixed-point or packing: use saturating arithmetic or wider types where needed.

---

**This architecture is designed to be extended layer by layer without ever breaking the "local rules → emergent behavior" contract.**
