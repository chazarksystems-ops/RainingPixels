# Slice Contracts

This folder holds bounded build contracts.

A slice contract is not an implementation. It is the agreement that defines what a future implementation agent may build, what it must not touch, and what success means.

## Rule

Agents work one slice at a time.

## Current Slice Order

1. `S01_TWIOFA_SUBSTRATE_PROOF_v0`
2. `S02_CELL_STATE_AND_GPU_BUFFERS`
3. Later slices remain in `slice_backlog/` until promoted.

## Slice Acceptance

A slice is accepted only when it has:

- player-visible result
- included scope
- excluded scope
- required inputs
- implementation contract
- acceptance gates
- failure modes
- audit checklist
- agent task prompt
- status

No gameplay code should begin from a slice that has not been accepted.
