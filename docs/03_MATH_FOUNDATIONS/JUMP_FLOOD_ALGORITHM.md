# Jump Flood Algorithm (JFA) — Structural Distance Transform

## What It Is

The Jump Flood Algorithm is a fast, parallel-friendly method to compute a **distance transform** (nearest seed/anchor point for every pixel/cell) in O(log N) passes instead of O(N) or worse.

In AntVerse it answers the question: "How far is this dirt cell from the nearest structural anchor (stone or attached dirt)?" This distance determines whether the dirt is structurally stable or should collapse into sand.

## Why JFA Fits AntVerse Perfectly

- Fully parallel (GPU friendly).
- Logarithmic passes → cheap enough to run **every frame** even at 4096².
- Produces not just distance but the actual coordinate of the nearest anchor (we pack it as `y << 16 | x`).
- Works beautifully with the unified memory architecture of DGX Spark (no need for complex tiling).

## Algorithm Steps (as used in AntVerse)

### 1. Initialization (`jfa_init.wgsl`)
```wgsl
if (cell is Stone) {
    jfa_out[idx] = (y << 16u) | x;
} else {
    jfa_out[idx] = 0xFFFFFFFFu; // INF
}
```

### 2. Iterative Flooding (`jfa_step.wgsl`)
For step sizes `1 << k` down to `1` (log₂(max_dim) iterations):

```wgsl
for each neighbor offset in {-1,0,1} × {-1,0,1} (skip 0,0) {
    let nx = x + dx * step;
    let ny = y + dy * step;
    if (in bounds) {
        let candidate = jfa_in[nx, ny];
        if (candidate != INF) {
            let dist = fixed_point_dist_sq(x, y, candidate_x, candidate_y);
            if (dist < best_dist) {
                best = candidate;
                best_dist = dist;
            }
        }
    }
}
jfa_out[idx] = best;
```

**Critical**: Use **fixed-point integer squared distance**, never floating-point `length()` for determinism.

### 3. Collapse Decision (`collapse.wgsl`)
After the final JFA pass has converged:

```wgsl
if (cell is DryDirt or WetDirt) {
    let nearest = jfa_in[idx];
    if (nearest == INF) {
        convert to Sand; return;
    }
    let dist = fixed_point_dist(nearest);
    let max_span = (mat == WetDirt) ? 35.0 + moisture_bonus : 6.0;
    if (dist > max_span) {
        convert to Sand;
    }
}
```

## Fixed-Point Distance Implementation (Required for Determinism)

```wgsl
fn dist_sq_fixed(x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
    let dx = i32(x1) - i32(x2);
    let dy = i32(y1) - i32(y2);
    return u32(dx * dx + dy * dy); // or scaled fixed-point for sub-cell precision
}
```

Compare against `max_span * max_span` (pre-computed as integer).

## Convergence & First-Frame Behavior

For a 4096×4096 grid, `log2(4096) = 12` passes are needed for full propagation from any anchor.

On frame 0 many dirt cells will have `INF` or very large distances → they will collapse immediately if they exceed the span limit. This is often desirable (unanchored floating dirt should fall).

For important pre-placed structures, we can either:
- Accept the initial collapse (visually interesting), or
- Run extra JFA passes on startup, or
- Mark "structurally important" dirt with a flag that raises the collapse threshold for the first N frames.

## Performance Characteristics on DGX Spark

- 12 passes × 16×16 workgroups on 4096² is very fast on unified memory.
- Each pass is a simple 3×3 gather with decreasing step size.
- Adding a 1-cell halo in shared memory (18×18 loads) yields significant bandwidth savings and is highly recommended for Phase 2+.

## Relation to Classic Falling Sand

JFA is the one place we step slightly outside "pure local 3×3 rules" (because distance propagation requires multiple passes). However, the **decision** to collapse is still a purely local rule based on the (already computed) distance field. The overall system still feels like classic falling sand — just with much smarter structural awareness.

---

**JFA is the secret sauce that lets AntVerse have dramatic, physically plausible architecture failure while staying GPU-friendly and deterministic.**
