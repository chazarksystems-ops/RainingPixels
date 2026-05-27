# Runtime Substrate Truth — GPU Packed Cell Model

## Core Principle

The material world is the authoritative simulation. Ant behavior and player intent are secondary to, and must react to, the physical state of the yard.

## Cell State (Locked)

All simulation state uses a single `u32` per cell:

| Bits    | Field              | v0 Interpretation                  |
|---------|--------------------|------------------------------------|
| 0–7     | `material_id`      | Air (0), Dry Dirt (1), Wet Dirt (2), Loose Soil (3), Water (4), Stone/Root (5) |
| 8–15    | `local_flags`      | Falling, Diggable, Saturated, Changed, Active, etc. |
| 16–23   | `material_scalar_0`| Moisture (dirt) / Volume (water)   |
| 24–31   | `aux_0`            | Simple scent strength (v0)         |

**Semantic Rule**: The 16–23 byte is interpreted as **Moisture** for dirt-like materials and **Volume** for water.

## Pass Ordering (Locked for v0)

1. Input / Order Application
2. Ant Movement + Dig Intent
3. Gravity + Flow (Reverse Pull / Gather model)
4. Water Spread + Pooling
5. Moisture Diffusion + Wetting (Gather model)
6. Collapse / Cohesion Check
7. Aux/Scent Update
8. Render

Gravity + Flow must run **before** moisture diffusion in every frame.

## Race Condition Policy

Pure gather passes are race-free by construction because each invocation writes only its own output cell.

Movement passes are **not** automatically race-free. Falling material, lateral flow, and collapse must use one of:
- block cellular automata,
- deterministic pair rules,
- checkerboard/Margolus scheduling,
- reservation/commit,
- atomics with explicit justification.

Double-buffering alone does not eliminate movement races.

Moisture diffusion is gather-safe, but strict conservation requires capacity-aware edge-transfer and excess handling.

Packed `u32` state minimizes per-cell bandwidth. It does not prove large-world performance. Performance claims require measurement.

## Communication Model (CPU ↔ GPU)

- **Perception**: Small per-ant sensory buffer generated on GPU (material underfoot, local scent gradient, moisture, danger).
- **Intent**: CPU sends high-level commands (e.g. "Dig at X,Y"). GPU validates and executes.
- **Result + Events**: GPU returns action results and important world events (major collapses) via small buffers.

Heavy iteration over the full grid must stay on the GPU. The CPU only sees summarized perception and event data.