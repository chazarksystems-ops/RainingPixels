# AGENTS.md — TWIOFA Agent Operating Rules

## Prime Directive

Agents work **one slice at a time**.

No implementation begins until a slice contract is accepted.

This repo exists to preserve the current TWIOFA direction, prevent drift, and make future code work bounded and auditable.

## Current Lock

- Active target: `TWIOFA_SUBSTRATE_PROOF_v0`
- Development philosophy: substrate-proof-first, not engine-first
- Runtime direction: Rust + WGSL / WebGPU-style packed-cell GPU CA material simulation
- First proof: one active material chunk / one-screen substrate
- Core player-visible loop: world-click/order → ants visibly act → material visibly changes → route consequence emerges
- Rejected for v0: LBM/full CFD, generic engine, flat tile-grid game, menu-only colony sim
- Deferred: infinite world streaming, full ECS, editor, save/load, procedural yard, production UI, networking

## Required Agent Preflight

Before changing any file, an agent must state:

- assigned slice
- files expected to change
- files forbidden
- docs read
- commands or validation steps
- acceptance criteria
- rollback/recovery note

## Forbidden Drift

Do not:

- create a general-purpose engine plan
- treat old chunk0/macroquad tile work as current implementation target
- reopen LBM/full CFD for v0
- start full ECS before substrate proof
- add infinite world streaming before single-chunk proof
- flatten TWIOFA into a tile-board RTS
- replace material consequence with text-only feedback
- implement gameplay code from the whole repo context

## Corrected Math Boundaries

Agents must preserve these constraints:

- Pure gather passes are race-free only when each invocation writes to its own output cell.
- Movement passes are not automatically race-free.
- Falling, lateral flow, and collapse need block scheduling, pair rules, claim buffers, or reservation/commit.
- Moisture conservation is conditional and requires capacity/excess handling.
- Bandwidth is plausible for proof sizes but must be measured.
- Do not claim all GPU races are mathematically impossible.

## Required Report After Work

Every agent pass must report:

- files read
- files changed
- files intentionally not touched
- design decisions preserved
- drift prevented
- validation performed
- remaining gaps
- next recommended slice
