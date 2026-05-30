# Phase 0 Demo Implementation Plan

**Status**: Draft — Phase 0 Tier 1 Research Complete  
**Date**: 2026-05-25  
**Purpose**: Provide a concrete, prioritized implementation roadmap for building the initial runnable demo (Phase 0 foundation + early Phase 1 symmetric gravity). This bridges the research decisions and design vision into actionable coding steps.

---

## Demo Scope (What We Are Building First)

**Goal**: A clean, large-grid (starting at 2048² or 4096²), interactive falling sand simulation with:
- Classic materials (Air, Sand, Water, Dirt, Stone)
- Symmetric reverse-pull gravity + hydrostatics (Phase 1)
- Basic brush interaction
- Foundational event emission (FrameSummary + notable transitions)
- Mass verification hooks
- Determinism harness hooks (at minimum, the ability to support it later)

**Explicitly Out of Scope for Initial Demo**:
- Full JFA + structural collapse (Phase 2)
- Capillary, moisture, and scent (Phase 3)
- Rich UI overlays or creative tooling exports (these come after core physics is solid)

---

## Core Technical Decisions to Implement

From Phase 0 Tier 1 research, the demo must incorporate:

| Decision | Source | Implementation Requirement |
|----------|--------|---------------------------|
| Single CommandEncoder for entire physics chain | 1.1 | One linear sequence per frame with one final buffer flip |
| Integer squared distance for any future JFA (prepare now) | 1.2 | Use fixed-point math patterns from the start where relevant |
| 16×16 workgroups + shared memory halo | 1.3 | Apply to gravity and any neighbor-gather passes |
| Event Ledger (16-byte WorldEvent + double-buffered atomic append) | 0.1 | Integrate FrameSummary + basic notable event emission |
| Mass definition + FrameSummary deltas | 0.2 | Emit before/after mass in FrameSummary |
| Determinism harness support | 0.3 | Keep CPU ref in mind; make rules pure and deterministic |
| First-frame collapse policy | 0.4 | Implement basic version even if JFA is minimal in demo |

---

## Recommended Implementation Order (Prioritized)

### Phase 0.0 — Foundation Setup (Do First)

1. **Project Structure & Dependencies**
   - Set up clean Rust + wgpu + winit project (pin versions as in `wgpu_winit.md`).
   - Define module structure:
     - `src/main.rs` (entry + event loop)
     - `src/simulation/` (core engine)
     - `src/simulation/state.rs` (grid buffers, SimParams)
     - `src/simulation/pipeline.rs` (compute pipeline management)
     - `src/simulation/events.rs` (event ledger types + helpers)
     - `src/simulation/verification.rs` (mass verification)
     - `src/render.rs` (full-screen triangle rendering)
     - `src/brush.rs` (brush logic)
   - Add `bytemuck` for zero-copy casting.

2. **Core Data Structures**
   - Define `SimParams` uniform (width, height, frame, seed, event_ledger_index, etc.).
   - Define packed `u32` cell state + accessor functions (material, flags, moisture, scent).
   - Reserve one flag bit for future `pre_stabilize` (even if unused in demo).

3. **Basic Ping-Pong Grid + Render Loop**
   - Create two storage buffers for the grid.
   - Implement minimal WGSL compute shader that copies input to output (sanity check).
   - Full-screen triangle render that samples the current grid and maps materials to colors.
   - Basic window + input loop with winit.

**Success Criteria**: Window opens, grid renders, simple CPU-side grid initialization works, FPS is stable at target resolution.

### Phase 0.1 — Brush & Basic Interaction

1. Implement basic brush (circle stamp) that can paint materials.
2. Support keyboard material selection (Sand, Water, Dirt, Stone, Air/Eraser).
3. Brush should write directly to the "next" buffer or use a staging buffer + compute stamp pass.
4. Keep brush fast even at 4096² (unified memory helps here).

**Success Criteria**: Responsive brush that feels good. Large brush strokes on water/sand produce believable local behavior.

### Phase 0.2 — Symmetric Reverse-Pull Gravity (Core Physics)

1. Implement the symmetric gravity shader (air pull + water vacate logic with `hash_spatial_sync`).
2. Use 16×16 workgroups + shared memory halo (18×18) for neighbor reads.
3. Integrate into single `CommandEncoder`:
   - Optional brush/stamp pass
   - Gravity pass
   - (Future) event emission
   - Single buffer flip at end
4. Add basic boundary handling (hard walls or clamp).

**Success Criteria**:
- Water flows and levels correctly with good hydrostatic behavior.
- Sand piles and slides diagonally in a consistent, deterministic way.
- No obvious mass leaks or visual artifacts in normal use.
- Large-scale flows feel physically plausible.

### Phase 0.3 — Foundational Event Emission (0.1)

1. Create double-buffered `EventLedger` buffers (atomic count + array of 16-byte `WorldEvent`).
2. Implement `try_emit_event` helper in WGSL.
3. Emit `FrameSummary` (type 0) every frame with mass deltas (integrate with 0.2 mass definition).
4. Emit basic notable events where easy and deterministic (e.g., significant water absorption into dirt, brush actions).
5. Host-side: After submit, read previous frame’s ledger (zero-copy on unified memory) and expose via `drain_events()`.

**Success Criteria**:
- FrameSummary is reliably emitted with plausible mass deltas.
- Basic events appear for notable actions.
- Host can consume events without stalling the GPU.

### Phase 0.4 — Mass Verification Hooks (0.2)

1. Implement host-side exact mass calculation using the defined formula (1.0 per solid/mobile cell + fractional water/moisture).
2. After each submit (or every N frames in release), compare current mass against previous + FrameSummary delta.
3. Log or assert on unexplained deltas (fatal in debug builds).
4. Keep one rolling "last known good" grid snapshot for localization (optional but recommended).

**Success Criteria**: Any obvious mass violation is immediately detected during normal play and brush testing.

### Phase 0.5 — Determinism Harness Foundation (0.3)

1. Begin writing a minimal CPU reference simulator (scalar, deterministic, same rules as WGSL).
2. Make sure gravity rules are pure functions of neighbors + global seed.
3. Set up basic input logging (brush actions + seed + frame) so replay is possible later.
4. Do not block demo progress on full harness — just lay the groundwork.

**Success Criteria**: Core rules are written in a way that makes future CPU reference implementation straightforward and 1:1.

### Phase 0.6 — Polish & Demo Readiness

1. Improve color mapping and visual clarity (especially water vs wet dirt, sand piles).
2. Add simple performance metrics (frame time, event count).
3. Test at target resolution (2048² or 4096²) for responsiveness.
4. Ensure first-frame behavior is acceptable (per 0.4 policy — even without full JFA, basic settling should not look broken).
5. Clean documentation and comments referencing the research decisions.

**Success Criteria**: A stable, visually pleasing, interactive demo that demonstrates large-scale coherent physics with good fluid behavior and the beginnings of observability.

---

## Recommended Code Structure (High Level)

```
src/
├── main.rs                 # Event loop, submit, render, basic UI
├── simulation/
│   ├── mod.rs
│   ├── state.rs            # Grid buffers, SimParams, EventLedger types
│   ├── pipeline.rs         # Compute pipeline creation + single-encoder update()
│   ├── gravity.wgsl        # Symmetric gravity shader (main physics)
│   ├── events.rs           # WorldEvent, EventLedger, try_emit_event helper
│   ├── verification.rs     # Mass calculation + comparison logic
│   └── cpu_reference.rs    # (Future) scalar CPU ref for harness
├── render.rs               # Full-screen triangle + material color mapping
├── brush.rs                # Brush stamping logic
└── utils.rs                # Hash functions, packing helpers, etc.
```

**Key Principles**:
- Keep the physics update strictly linear inside one `CommandEncoder`.
- Make event emission optional in shaders via bind group (zero cost when unused).
- Design for easy extension (Phase 2 JFA, Phase 3 capillary) without major rewrites.

---

## Success Criteria for the Overall Demo

- Large grid (≥2048²) runs interactively with good fluid/granular behavior.
- Symmetric gravity produces visibly better hydrostatics and diagonal sliding than classic implementations.
- Basic event emission (FrameSummary + some notable events) works and is consumable by host.
- Mass verification catches obvious leaks during testing.
- Code is clean enough to serve as the foundation for Phase 2 (JFA) without major refactoring.
- The demo feels responsive and visually coherent, aligning with the experiential goals in `UI_UX_VISION.md` and `VISUAL_AND_INTERACTION_DIFFERENCES.md`.

---

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Performance at 4096² with halo + events | Start at 2048², profile early, rely on unified memory + 16×16 halo tiling |
| Event ledger contention under heavy load | Start with single atomic counter; shard later only if measured |
| Brush feeling sluggish at large scale | Use compute stamp pass or direct unified writes; test early |
| Over-engineering early | Strictly follow the phased order above. Defer polish and advanced features. |

---

**This plan prioritizes getting a solid, observable, large-scale physics foundation running quickly while embedding the key research decisions from the start. It deliberately defers structural complexity (JFA) to Phase 2 once the core loop is proven.**