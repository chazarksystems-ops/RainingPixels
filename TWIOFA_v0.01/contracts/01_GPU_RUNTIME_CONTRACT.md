# GPU Runtime Contract — TWIOFA_SUBSTRATE_PROOF_v0

## Cell State
- Single `u32` per cell using the layout defined in `02_RUNTIME_SUBSTRATE_TRUTH.md`
- Double-buffered simulation (`cell_current` / `cell_next`)

## Compute Passes
Must support at minimum:
- Gravity + Flow (Reverse Pull model)
- Moisture Diffusion + Wetting (Gather model)
- Basic collapse / cohesion check

## Communication
- Perception buffer (GPU → CPU)
- Intent buffer (CPU → GPU)
- Result + Event buffer (GPU → CPU)

## Constraints
- Single fixed chunk only
- No dynamic allocation for world size in v0
- Workgroup size starting at (8,8) or (16,16) — tune only after correctness

All shader code must be written with explicit double-buffering and race-condition mitigation documented.

## Corrected Safety Requirements

No compute pass may claim race-freedom unless it satisfies the single-writer invariant or declares its scheduling strategy.

No movement pass may use unscheduled scatter writes.

No moisture pass may claim strict conservation unless water loss and dirt gain are matched with capacity/excess handling.

No performance claim may be accepted without measured pass timing.

The corrected proof file is required reading:
`docs/TWIOFA_MATHEMATICAL_PROOFS_FOR_AUDIT_CORRECTED.md`