# CURRENT_PRIORITY.md — TWIOFA v0.01

## Current Priority

The current priority is:

> Refine and validate `TWIOFA_SUBSTRATE_PROOF_v0` slice contracts before writing gameplay code.

The repo is ready to seed GitHub as a documentation-first foundation. It is not yet ready for broad implementation.

## Immediate Work Order

1. Confirm the GitHub seed contains the real populated documents.
2. Refine `S01_TWIOFA_SUBSTRATE_PROOF_v0`.
3. Confirm acceptance gates for the first playable substrate proof.
4. Refine `S02_CELL_STATE_AND_GPU_BUFFERS`.
5. Only then create a bounded implementation prompt for the first code-adjacent slice.

## First Implementation-Adjacent Slice

The first implementation-adjacent slice is:

`S02_CELL_STATE_AND_GPU_BUFFERS`

That slice should implement or specify:

- packed `u32` cell state
- Rust/WGSL masks and shifts
- matching test vectors
- pass invariants
- no gameplay logic yet

## Not Current Priority

These are not current priority:

- full engine
- full game code
- full colony AI
- procedural yard
- infinite streaming
- editor
- save/load
- production renderer
- full ECS
- LBM/full CFD

## Current Success Condition

A future implementation should be considered valuable only if it moves toward this proof:

> Ants visibly interact with a living material yard where digging, moisture, collapse, and route consequence are player-readable.
