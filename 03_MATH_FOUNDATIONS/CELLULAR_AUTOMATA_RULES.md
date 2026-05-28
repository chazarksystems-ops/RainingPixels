# Cellular Automata Rules — AntVerse

## Core Philosophy

All rules must remain **strictly local** (3×3 or at most 5×5 neighborhood) and **pure functions** of the current cell + its neighbors + a small amount of global state (frame number + seed). This preserves the classic falling sand soul even as we add layers.

## Material Definitions (Phase 0–3)

| ID | Name       | Mobile? | Solid? | Notes |
|----|------------|---------|--------|-------|
| 0  | Air        | No      | No     | Empty space |
| 1  | Dry Dirt   | No      | Yes    | Brittle structural material |
| 2  | Wet Dirt   | No      | Yes    | Stronger structural material, absorbs water |
| 3  | Stone      | No      | Yes    | Permanent anchor / obstacle |
| 4  | Water      | Yes     | No     | Flows, wets dirt, can be absorbed |
| 5  | Sand       | Yes     | Yes    | Collapsed dirt; gravity acts on it, water pools on top |

## Phase 0 Classic Rules (Baseline)

**Sand (1)**:
- If cell directly below is Air → move down.
- Else if diagonal-left is Air → move diagonal-left (or random bias).
- Else if diagonal-right is Air → move diagonal-right.

**Water (4)**:
- If below is Air → move down.
- Else if below is solid and left is Air → move left.
- Else if below is solid and right is Air → move right.
- (Classic implementations often do multiple sub-steps per frame for faster spreading.)

These rules are deliberately simple. All future phases build on top of or replace sections of this logic.

## Phase 1 Symmetric Reverse-Pull Rules (Your Core Innovation)

Instead of the classic "matter looks for space below" approach (which creates race conditions on GPU), we use **mirrored logic**:

**Air cells (pull logic)**:
- Check directly above for Water → pull it down.
- Check diagonal-above-left and diagonal-above-right (with deterministic tie-break via `hash_spatial_sync`) for Water that is sitting on a solid obstacle → pull it diagonally.
- Check left and right neighbors for Water that is trapped (solid below + solid diagonal-outer) → pull laterally due to hydrostatic pressure.

**Water cells (vacate logic)**:
- Check directly below for Air → vacate downward.
- If below is solid, check diagonal-below spaces with mirrored PRNG check from the target Air cells.
- If all vertical and diagonal options blocked, check left/right for Air (lateral dispersion).

**Key Properties**:
- Every "pull" by an Air thread has a corresponding "vacate" by a Water thread → perfect mass conservation.
- No atomics required.
- Deterministic via spatial hash cross-comparison.

See `05_CODE_TEMPLATES/wgsl/symmetric_gravity.wgsl` (to be created in Phase 1) for the exact implementation.

## Phase 2 JFA + Collapse Rules

**Jump Flood Algorithm** (distance transform):
- Initialize: Stone cells write their own (y << 16 | x); all others write `0xFFFFFFFF`.
- For `log2(max(width, height))` steps with decreasing step size:
  - Each cell looks in a 3×3 neighborhood at `step` distance.
  - Keeps the closest anchor coordinate (using fixed-point squared distance).

**Collapse Decision** (per dirt cell):
- Read nearest anchor distance from converged JFA field.
- If distance > material-specific max_span:
  - Dry Dirt (1): max_span ≈ 6
  - Wet Dirt (2): max_span ≈ 35 + (moisture - 100) * 0.1
- Convert cell to Sand (5) + preserve moisture/flags.

This creates beautiful emergent failure of large dirt structures.

## Phase 3 Capillary Rules

**Dirt cells**:
- Count orthogonal water neighbors.
- Add `water_neighbors * ABSORB_RATE` to moisture (saturate at 255).
- If Dry Dirt and moisture ≥ WET_THRESHOLD → become Wet Dirt.
- If Wet Dirt and no water neighbors and (frame % 10 == 0) → decrement moisture; if moisture ≤ DRY_THRESHOLD → become Dry Dirt.

**Water cells**:
- Count orthogonal dirt neighbors.
- Lose `dirt_neighbors * ABSORB_RATE` volume.
- If volume reaches 0 → become Air.

**Scent (future extension)**:
- Simple diffusion: each cell averages scent with neighbors + small decay.
- Later: directed transport by ant agents.

## Determinism Requirements Across All Phases

1. Use `hash_spatial_sync(x, y, frame)` with a single global `seed` from `SimParams` for all tie-breaking.
2. JFA distance must use integer squared Euclidean, never `length(f32)`.
3. All thresholds and rates are compile-time constants or uniform parameters (never per-thread randomness).
4. Boundary handling must be explicit and identical on every thread.

## Mass Conservation Strategy

- Phase 0–1: Symmetric pull/vacate guarantees 1:1 conservation for water.
- Phase 2: Collapse converts dirt → sand (mass preserved as different material).
- Phase 3: Water absorbed into dirt is tracked as increased moisture volume. Evaporated moisture returns to air or is intentionally lost (documented as a tunable "evaporation sink").

We will add explicit mass-accounting telemetry in Phase 4 so any leakage is visible and explainable.

---

**This rule set is designed to be implemented incrementally while always remaining a pure cellular automaton.**
