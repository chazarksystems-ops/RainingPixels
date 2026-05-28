# AntVerse Implementation Phases

This document defines the exact sequence in which we will build the engine. Each phase adds one major capability while preserving the classic falling sand feel and the DGX-native design.

## Phase 0 — Classic Falling Sand Foundation (Current)

**Goal**: Prove that a large-grid, responsive, interactive falling sand simulation feels alive on DGX Spark with zero architectural compromises.

**Materials**:
- Air (0)
- Sand (1) — falls down or diagonally
- Water (2) — falls and spreads laterally when blocked
- Dirt / Stone (3) — static obstacle (or simple dirt that can be dug later)

**Rules** (purely local 3×3 neighborhood):
- Sand falls down if air below, otherwise diagonally left or right (classic random or deterministic bias).
- Water falls down, then spreads left/right when blocked.
- Simple "in-place" or ping-pong update.

**Deliverables**:
- Clean WGSL compute shader (`phase0_classic.wgsl`)
- Basic Rust host with winit + wgpu (brush painting, keyboard material selection)
- 2048×2048 or 4096×4096 grid running at interactive rates
- Simple but pleasing color mapping
- This folder structure fully seeded with vision, architecture, and math documents

**Success Criteria**:
- Large dirt structures and water pools behave intuitively.
- Brush feels responsive.
- Code is clean enough to serve as the base for all future phases.
- No atomics, no mid-frame buffer flipping, good determinism foundation.

---

## Phase 1 — Symmetric Reverse-Pull Gravity + Hydrostatics + Sand as Mobile Solid

**Goal**: Dramatically improve fluid and granular behavior using your existing mathematically strict design (mirrored air-pull / water-vacate logic with spatial hash tie-breaking).

**New / Changed**:
- Add full symmetric gravity shader (your `compute.wgsl` with fixes from audit).
- Introduce `MAT_SAND` (5) as a mobile solid that gravity can pull (collapsed dirt).
- Update `is_solid()` and `is_mobile()` helpers.
- Water can now pool on top of sand piles.

**Key Math**:
- `hash_spatial_sync` for deterministic diagonal choice.
- Cross-reading of neighbor seeds so air and water threads agree without atomics.

**Deliverables**:
- Updated `compute.wgsl` (Phase 1 version)
- Fixed linear pipeline in `main.rs` (single flip at end of frame)
- Visual confirmation that water cascades realistically and sand piles form stable slopes.

**Success Criteria**:
- Perfect mass conservation (no spontaneous creation/destruction of water).
- Diagonal sliding along obstacles works without race conditions.
- Lateral hydrostatic dispersion creates flat pools.

---

## Phase 2 — JFA Structural Integrity + Moisture-Dependent Collapse

**Goal**: Add emergent architecture — dirt can form ceilings, bridges, and overhangs that eventually collapse under their own span, with wet dirt being much stronger than dry dirt.

**New Components**:
- `jfa_init.wgsl`
- `jfa_step.wgsl` (fixed-point integer distance version)
- `collapse.wgsl` (reads JFA distance + moisture → decides whether to turn dirt into sand)
- Integration into the linear pipeline **before** gravity pass.

**Key Math**:
- Jump Flood Algorithm (log₂(N) passes) to compute nearest stone/dirt-anchor distance per cell.
- Fixed-point squared Euclidean distance to avoid f32 drift.
- Moisture-dependent max span: Dry dirt ~6 cells, Wet dirt ~35+ cells (plus moisture bonus).

**Deliverables**:
- Full JFA + collapse working every frame at target resolution.
- Dramatic visual test: floating dirt ceiling that slowly sags and collapses, creating sand avalanches.
- Updated pipeline ordering documented.

**Success Criteria**:
- Structural collapse is deterministic and visually satisfying.
- Wet dirt holds much larger spans than dry dirt.
- No first-frame explosion or non-convergence artifacts after initial stabilization.

---

## Phase 3 — Capillary Wicking, Evaporation & Scent Diffusion

**Goal**: Add moisture transport and the foundation for biological layers (scent for future ants).

**New / Changed**:
- Full `capillary.wgsl` (your existing version or refined).
- Moisture absorption into dirt from adjacent water.
- Dry dirt → Wet dirt transition at threshold.
- Wet dirt slowly evaporates (frame % N).
- Water volume decreases when touching dirt.
- Scent field diffusion (simple low-cost pass or integrated into capillary).

**State Packing**:
- Bits 16-23 become moisture/volume.
- Bits 24-31 become scent (initially simple diffusion, later directed by ant agents).

**Deliverables**:
- Capillary + scent layers running after gravity.
- Visual: water soaking into dirt piles, wet dirt darkening and holding shape longer, scent "smoke" visualization (optional overlay).
- First scent trails that could later guide simple agents.

**Success Criteria**:
- Mass is still conserved (water that disappears into dirt is accounted for as increased moisture).
- Evaporation is slow and tunable.
- Scent diffuses in a visually pleasing and physically plausible way.

---

## Phase 4 — Telemetry, Event Ledger & Host Bridge

**Goal**: Make the simulation observable and consumable by your higher-level systems (CedeSystem, Entelechy, future LLM agents).

**New Components**:
- Lightweight event emission from compute passes (or a cheap reduction pass).
- `event_ledger` ring buffer in unified memory.
- Host-side reader that can consume events without stalling the GPU.
- Basic `causality_map` style aggregation (e.g., "this collapse was caused by X water + Y moisture").

**Event Types** (examples):
- `SandStartedFalling`, `WaterAbsorbed`, `DirtCollapsed`, `StructureFormed`, `ScentDiffused`

**Deliverables**:
- GPU can emit events with almost zero overhead.
- CPU can read a summary or event stream every N frames.
- First integration test: your host code prints or logs interesting world events in real time.

**Success Criteria**:
- Telemetry has negligible impact on frame rate.
- Events are useful for higher-level reasoning (not just raw cell counts).
- Foundation is solid for future agent orchestration on top of the physics.

---

## Phase 5+ — Future Layers (Documented but Not Yet Scheduled)

- Heat / Thermodynamics (lava, freezing, convection)
- Simple Chemistry (acid dissolving rock, crystallization)
- Biological Agents (ants following scent gradients on top of the physics substrate)
- Multi-scale / Hierarchical simulation (coarse global + fine local regions)
- Export pipelines to Glazier, Seed Forge, Unreal, etc.
- Full provenance tracking and cryptographic receipting of simulation states (ArcSystems alignment)

---

## Phase Ordering Rationale

1. **Physics first** (gravity + structural) — gives the most visually dramatic improvement and validates the core loop.
2. **Moisture / capillary next** — natural extension of dirt behavior and prerequisite for scent.
3. **Telemetry last in the core loop** — because observability is most valuable once the physics is rich and stable.
4. **Future layers** are deliberately deprioritized so we don't over-engineer before the foundation is rock-solid.

Each phase must leave the codebase in a clean, documented, and runnable state. We do not move to Phase N+1 until Phase N meets its success criteria and the relevant documents in this folder are updated.

---

**Current Phase**: 0 (Foundation)  
**Next Milestone**: Complete Phase 0 + seeded folder structure → begin Phase 1 implementation.
