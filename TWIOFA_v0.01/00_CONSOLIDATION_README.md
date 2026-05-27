# TWIOFA Consolidated Substrate Proof — v0

**Purpose**: This is the single authoritative reference for the current execution target of TWIOFA.

It consolidates all high-signal decisions from prior exploration phases into one clean, anti-drift package focused exclusively on:

- TWIOFA_SUBSTRATE_PROOF_v0
- GPU packed-cell material simulation
- Visible ant-scale material consequence
- World-click → visible execution → terrain change loop

This pack deliberately excludes:
- Premature scaling systems
- Generic engine abstraction
- Full colony simulation
- LBM / advanced fluid dynamics
- Infinite world infrastructure

Use this as the primary source of truth until the substrate proof is validated or rejected.

## Required Implementation Reading Order

Before writing code, an implementation agent must read in this order:

1. `01_CURRENT_TARGET.md`
2. `02_RUNTIME_SUBSTRATE_TRUTH.md`
3. `docs/TWIOFA_MATHEMATICAL_PROOFS_FOR_AUDIT_CORRECTED.md`
4. `contracts/04_RACE_SCHEDULING_CONTRACT.md`
5. `contracts/05_MOISTURE_CONSERVATION_CONTRACT.md`
6. `runtime/01_PASS_INVARIANTS_AND_CELL_STATE.md`
7. `runtime/02_BANDWIDTH_MEASUREMENT_PLAN.md`
8. `07_BUILD_SEQUENCE_v0.md`

The corrected mathematical proof file supersedes any previous proof draft that claims unconditional race-freedom, strict mass conservation, or bandwidth elimination.

## How to Use This Pack

1. Read `01_CURRENT_TARGET.md` first.
2. Treat `02_RUNTIME_SUBSTRATE_TRUTH.md` as the technical contract.
3. Use the contracts and registries to prevent drift during implementation.
4. Everything outside this pack that contradicts these documents should be treated as historical.

## Core Identity Preserved

TWIOFA is a **living material yard** game where:
- The substrate (dirt, water, moisture, collapse) *is* the gameplay.
- Ants exist to make the material consequence legible and consequential.
- The player shapes the yard through world-click orders that produce visible physical results.