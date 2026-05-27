# Pass Invariants and Cell State — TWIOFA_SUBSTRATE_PROOF_v0

## Cell word invariant

Every cell is one `u32`:

```text
bits  0..7   material_id
bits  8..15  local_flags
bits 16..23  material_scalar_0
bits 24..31  aux_0
```

All fields must round-trip exactly between Rust and WGSL.

## Required test vectors

At minimum:

```text
Air:
M=0, F=0, Q=0, A=0

Dry diggable dirt:
M=1, F=DIGGABLE|ACTIVE, Q=64, A=0

Wet dirt with scent:
M=2, F=DIGGABLE|SATURATED|CHANGED, Q=192, A=40

Water:
M=4, F=ACTIVE|CHANGED, Q=128, A=0

Stone/root:
M=5, F=0, Q=255, A=0
```

Tests must prove:

```text
unpack(pack(M,F,Q,A)) == (M,F,Q,A)
```

in Rust.

WGSL constants must match Rust constants exactly.

## Pass invariant template

Every compute pass must be documented with this table:

```text
Pass name:
Source buffers:
Destination buffers:
Reads:
Writes:
Scheduling model:
Single-writer proof:
Material conservation status:
Known approximations:
Validation hook:
```

## v0 pass ordering

Locked starting order:

```text
1. Input / order application
2. Ant movement + dig intent
3. Scheduled gravity/settling
4. Water spread / pooling
5. Moisture diffusion / wetting
6. Collapse/cohesion classification
7. Aux/scent update
8. Render
```

## Required validation hooks

The runtime must expose debug counters:

```text
cells_by_material
cells_changed_this_frame
water_volume_total_or_approx
dirt_moisture_total_or_approx
falling_cells_count
collapse_candidate_count
ants_blocked_count
ants_dig_actions_count
```

These do not need production UI. They can be console/debug overlay counters.

## Forbidden implementation shortcuts

Do not:
- pack fields differently in Rust and WGSL,
- use unbounded scatter writes,
- rely on thread timing for random choices,
- claim performance before measuring pass timings,
- call a visual grid "material continuity."
