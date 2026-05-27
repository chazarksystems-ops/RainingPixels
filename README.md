# RainingPixels

**Repository for the RainingPixels substrate proof foundation (TWIOFA v0.01 + S02 bounded cell state)**

This is the clean, anti-drift seed repository for the pixel-scale raining / falling-sand style cellular automata research originally developed under the AntVerse knowledge base.

## Contents

- `TWIOFA_v0.01/` — The full corrected substrate proof foundation + accepted S01/S02 work
  - S01_TWIOFA_SUBSTRATE_PROOF_v0 (parent contract) — Accepted
  - S02_CELL_STATE_AND_GPU_BUFFERS (packed u32 cell state) — Accepted
  - Complete slice contracts, anti-drift rules, acceptance scorecards, design docs, and runtime substrate truth.

- `implementation/S02_cell_state/` — Bounded reference implementation
  - Rust `Cell(u32)` pack/unpack with accessors and mass conservation helpers (bytemuck-compatible for GPU)
  - WGSL bit-unpack examples (copy + render passes)
  - Standalone round-trip verification (exhaustive sampling of byte fields)
  - Direct mapping to the S02 contract

## Key Guarantees

- Exactly 32 bits per cell (material + flags + moisture + scent)
- Lossless round-trip pack → unpack → pack for all Phase 0 values
- No drift: all transformations are pure functions of the packed state
- GPU-friendly: `Pod + Zeroable`, perfect for storage buffers and compute
- Future synergy with AntVerse research lane (event ledger, JFA, structural drama, scent lattice)

## Build / Verify (implementation)

The `implementation/S02_cell_state/roundtrip_tests.rs` can be run directly:

```bash
rustc roundtrip_tests.rs -o rt && ./rt
```

Full integration lives in the AntVerse repo (cross-reference for ongoing development).

## License / Usage

Research seed. See `TWIOFA_v0.01/AGENTS.md` and anti-drift rules before extending.

## Next

This repo is the permanent home for the RainingPixels visual substrate. AntVerse continues as the broader research vehicle.

---

**Initial seed commit**: TWIOFA v0.01 substrate proof foundation + accepted S01/S02
