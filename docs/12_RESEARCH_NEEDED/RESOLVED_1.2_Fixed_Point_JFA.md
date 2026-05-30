# Resolved Research: 1.2 Fixed-Point Integer Math for JFA Distance Transform

**Date Researched**: 2026-05-25
**Priority**: Highest
**Status**: Resolved — Clear implementation path identified

## Core Problem

The original JFA implementation in your pasted code uses `length(vec2<f32>(...))` for distance comparison. This introduces floating-point non-determinism, which conflicts with AntVerse's determinism and BYTES_ARE_LAW requirements.

## Authoritative Reference

**Rong, Guodong & Tan, Tiow-Seng (2006)** — "Jump Flooding in GPU with Applications to Voronoi Diagram and Distance Transform" (SI3D '06).

This is the foundational paper that introduced JFA. All modern implementations derive from it.

## Recommended Solution: Integer Squared Euclidean Distance

Replace all floating-point distance calculations with pure integer math:

```wgsl
// Pack anchor as (y << 16) | x  (as already done in your jfa_init.wgsl)
fn get_coord(packed: u32) -> vec2<u32> {
    return vec2<u32>(packed & 0xFFFFu, packed >> 16u);
}

fn dist_sq_fixed(p1: vec2<u32>, p2: vec2<u32>) -> u32 {
    let dx = i32(p1.x) - i32(p2.x);
    let dy = i32(p1.y) - i32(p2.y);
    // Use squared distance to avoid sqrt and floating point entirely
    return u32(dx * dx + dy * dy);
}
```

**In the JFA step shader**:
- Instead of comparing `dist < best_dist` using `length(f32)`, compare `dist_sq_fixed(...) < best_dist_sq`.
- Store the best squared distance alongside (or instead of) the floating-point version during the flood passes.

**In collapse.wgsl**:
```wgsl
let dist_sq = dist_sq_fixed(current_pos, anchor_pos);
let max_dist_sq = max_span * max_span;   // precompute or calculate as u32

if (dist_sq > max_dist_sq) {
    // collapse to sand
}
```

## Why Squared Distance Is Sufficient and Superior Here

- We only need to **compare** distances, never compute the actual Euclidean distance for rendering or physics in the structural pass.
- Integer math is fully deterministic across frames, GPUs, and driver versions.
- Avoids all precision issues with `f32` on large grids (4096²+).
- Extremely cheap on GPU (integer multiply-add).

## Precision Considerations

For a 4096×4096 grid, the maximum possible squared distance is `(4096² + 4096²) ≈ 33.5 million`, which fits comfortably in `u32`.

If we later want sub-cell precision or smoother collapse thresholds, we can scale the coordinates by a small factor (e.g., ×4 or ×8) before squaring, still staying well within `u32`.

## Implementation Notes for AntVerse

1. Keep the packed `u32` anchor format `(y << 16) | x` — it is already excellent.
2. Modify `jfa_step.wgsl` to track and compare **squared integer distance** instead of `length(f32)`.
3. Modify `collapse.wgsl` to use squared distance vs squared max_span.
4. The moisture bonus to max_span can still be applied in floating point for the threshold calculation if desired, or converted to fixed-point.

## Variants & Future Options

- The original paper discusses accuracy improvements via extra passes (JFA+1, JFA+2). We can add these later as an optional "high quality structural" mode without changing the integer distance core.
- For even higher determinism or portability, we could store anchors in a separate `vec2<u32>` buffer, but the current packed format is efficient and sufficient.

**Conclusion**: Switching to integer squared distance is a small, low-risk change that eliminates floating-point non-determinism from the structural integrity system while preserving (or improving) performance. This is the clear path forward for AntVerse.
