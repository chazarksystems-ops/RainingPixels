# TWIOFA v0.01 — Documentation-First Substrate Proof Foundation

This repository is the initial documentation-first foundation for **The World Is On FIRE Ants (TWIOFA)**.

The active target is **TWIOFA_SUBSTRATE_PROOF_v0**: a deliberately narrow vertical slice intended to prove whether ant-scale material interaction is compelling enough to build the game around.

This is **not** a gameplay-code implementation repo yet. It is a slice-contract and design-intent repo. Agents should use this repo to preserve, refine, and audit the design contracts before implementation begins.

## Current Active Target

TWIOFA_SUBSTRATE_PROOF_v0 must prove one player-visible question:

> Can a player order ants inside a living material yard, dig through real material, see water/moisture/collapse change the world, and feel that tunnel geometry itself is gameplay?

The first proof is:

- one active material chunk / one-screen substrate proof
- Rust + WGSL / WebGPU-style packed-cell GPU cellular material simulation
- packed `u32` cell state
- CA-style water/moisture/collapse, not LBM
- visible ants
- visible digging
- visible material consequence
- route consequence from collapse, moisture, flooding, or blockage

## What This Repo Is Not

This repo is not:

- a general game engine plan
- a tile-grid RTS plan
- a terminal colony simulator
- an LBM/full CFD research plan
- an infinite-world streaming plan
- a full ECS/gameplay-code implementation repo
- a place to dump old archives into root

Historical material may be referenced, but the current implementation direction is substrate-proof-first.

## Reading Order for New Agents

Read in this order:

1. `00_CONSOLIDATION_README.md`
2. `01_CURRENT_TARGET.md`
3. `03_ANTI_DRIFT_RULES.md`
4. `AGENTS.md`
5. `CURRENT_PRIORITY.md`
6. `FINAL_PATCH_REPORT.md`
7. `contracts/01_GPU_RUNTIME_CONTRACT.md`
8. `contracts/04_RACE_SCHEDULING_CONTRACT.md`
9. `contracts/05_MOISTURE_CONSERVATION_CONTRACT.md`
10. `playability/PLAYABILITY_FEEL_LOCKS.md`
11. `visual/VISUAL_READABILITY_CONTRACT.md`
12. `design/LAYERED_FIELD_IDENTITY.md`
13. `slice_contracts/README.md`

## Current Priority

The immediate priority is to refine and validate slice contracts before gameplay code begins.

The first implementation-adjacent slice is:

`S02_CELL_STATE_AND_GPU_BUFFERS`

But it should not be implemented until the parent substrate-proof slice contract is accepted.

## Agent Rule

Agents work **one slice at a time**. No agent should use the entire repo as permission to build a broad engine.
