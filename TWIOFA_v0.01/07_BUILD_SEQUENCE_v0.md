# Build Sequence — TWIOFA_SUBSTRATE_PROOF_v0

## Phase 0 — Contract Alignment
- Read all documents in this consolidation pack
- State current understanding of scope and anti-drift rules before editing code

## Phase 1 — Cell State Foundation
- Implement packed `u32` cell state (Rust + WGSL)
- Create pack/unpack functions with test vectors
- Establish material and flag constants

**Deliverable**: `cell_state.rs` + `cell_state.wgsl` with matching behavior

## Phase 1A — Mathematical Safety Gates
After cell-state foundation and before movement implementation:

- Read `docs/TWIOFA_MATHEMATICAL_PROOFS_FOR_AUDIT_CORRECTED.md`.
- Select a movement scheduling strategy (block CA, deterministic pair, reservation/commit, etc.).
- Declare pass invariants for each compute pass.
- Declare moisture transfer mode (conservative edge-transfer or capped absorption approximation).
- Add bandwidth/pass timing instrumentation plan.
- Reject any implementation that claims race-freedom without a single-writer proof or documented scheduling method.

## Phase 2 — Single Chunk Runtime
- Create one fixed-size GPU buffer
- Basic render path for the material field
- Double-buffering infrastructure

**Deliverable**: Runnable window showing one material chunk (no simulation yet)

## Phase 3 — Gravity + Flow
- Implement Reverse Pull gravity and basic water movement
- Add settling for loose soil
- Ensure no dominant race condition artifacts

## Phase 4 — Moisture + Collapse
- Add moisture diffusion (Gather model)
- Add basic collapse when support is removed
- Visual distinction for wet vs dry + disturbance

## Phase 5 — Minimal Ants + Interaction
- Add visible ant group
- Implement click-to-dig intent flow
- Ants react to terrain changes (blocked routes, etc.)

## Phase 6 — Feel Validation
- Playtest against Acceptance Gates
- Verify the loop produces visible material consequence
- Decide: PASS / REVISE / FAIL

Do not advance phases until the previous phase meets its acceptance criteria.