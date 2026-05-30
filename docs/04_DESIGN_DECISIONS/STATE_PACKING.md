# State Packing — AntVerse Cell Representation

## Current (Phase 0–3) Layout: u32 per cell

```
Bits  0–7   : Material ID (0–5 as defined in CELLULAR_AUTOMATA_RULES.md)
Bits  8–15  : Flags (reserved for future use: on_fire, has_root, etc.)
Bits 16–23  : Moisture / Volume (0–255)
              - For Dirt: moisture level (affects strength & appearance)
              - For Water: remaining volume (decreases on absorption)
Bits 24–31  : Scent (0–255) — simple diffusion value for future ant layers
```

**Total**: 32 bits. Extremely compact while supporting all planned Phase 0–3 features.

## Accessor Functions (WGSL)

```wgsl
fn get_material(packed: u32) -> u32 { return packed & 0xFFu; }
fn get_flags(packed: u32) -> u32   { return (packed >> 8u) & 0xFFu; }
fn get_moisture(packed: u32) -> u32 { return (packed >> 16u) & 0xFFu; }
fn get_scent(packed: u32) -> u32   { return packed >> 24u; }

fn pack_state(mat: u32, flags: u32, moisture: u32, scent: u32) -> u32 {
    return mat | (flags << 8u) | (moisture << 16u) | (scent << 24u);
}
```

## Why This Packing Works Well

- Single 32-bit load/store per cell → maximum memory coalescing.
- Moisture and scent are 8-bit which is perceptually and mechanically sufficient for the effects we want.
- Flags byte gives 8 future booleans without growing the cell size.
- Easy to extend to `u64` later if we need 16-bit moisture or additional fields (temperature, age, etc.) without changing the entire memory layout strategy.

## Future Extensions (When We Need Them)

**Option A — u64 per cell** (recommended when we add heat/chemistry):
- Lower 32 bits: current layout
- Upper 32 bits: temperature (16-bit fixed-point) + chemistry flags + agent ID or other data

**Option B — Separate auxiliary buffers** (sometimes cleaner):
- Keep main physics buffer as u32.
- Add `moisture_field: array<u8>`, `scent_field: array<u8>`, `heat_field: array<f16>` etc.
- Slightly more memory traffic but cleaner separation of concerns.

We will stay with the packed u32 model through Phase 3 and only move to u64 or auxiliary buffers when a clear need arises (documented in the relevant phase folder).

## Alignment with Determinism & Provenance

Because every cell is exactly 4 bytes and updates are pure functions of neighbors + global seed, the entire world state at any frame is a deterministic function of the initial seed + frame number. This is extremely valuable for:

- Cryptographic receipting of simulation states (ArcSystems alignment)
- Replaying exact scenarios for debugging or AI training
- Proving mass conservation and structural rules mathematically

---

**State packing is deliberately minimal and future-proof. We add complexity only when the physics demands it.**
