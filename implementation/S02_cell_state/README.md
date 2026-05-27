# S02_CELL_STATE_AND_GPU_BUFFERS — Bounded Implementation

**Status**: Accepted + Bounded reference implementation (Phase 0)

This folder contains the authoritative packed `u32` cell state implementation extracted from the AntVerse research lane for the RainingPixels substrate proof.

## Contents
- `cell_state.rs` — Core `Cell(u32)` with const pack/unpack, accessors, mass calculation, flag handling. Uses bytemuck for Pod/Zeroable GPU compatibility.
- `STATE_PACKING.md` — Design rationale for the 4-byte layout (material | flags | moisture | scent).
- `copy.wgsl` — Minimal WGSL compute demonstrating storage buffer round-trip of the packed u32 grid.
- `render.wgsl` — WGSL fragment that unpacks on-GPU for display (material + moisture).

## Pack / Unpack (Rust)
```rust
pub const fn new(material: u8, flags: u8, moisture: u8, scent: u8) -> Self { ... }
pub fn material(self) -> u8 { (self.0 & 0xFF) as u8 }
pub fn moisture(self) -> u8 { ((self.0 >> 16) & 0xFF) as u8 }
...
```

## WGSL Equivalents (for GPU)
- Direct bit ops: `let mat = packed & 0xFFu;`
- No drift: all paths use explicit masks/shifts, values stay in 0-255 for bytes.

## Round-Trip Verification
Comprehensive round-trip tests (property-based + exhaustive for byte fields) live in the AntVerse research lane (`antverse/src/simulation/state.rs` and verification harness). Key invariants maintained here:
- 0 ≤ material ≤ 4 (Phase 0 set)
- flags byte preserves reserved bits
- moisture/scent are pure passthrough 0-255
- `Cell::new(a,b,c,d).0` roundtrips losslessly through accessors
- `mass_contribution()` is deterministic and bounded

See `TWIOFA_v0.01/slice_contracts/S02_CELL_STATE_AND_GPU_BUFFERS/S02_CELL_STATE_CONTRACT.md` for the full acceptance contract.

## Future
This bounded representation is the single source of truth for all future AntVerse / RainingPixels simulation work. Any deviation triggers anti-drift rejection.
