# Resolved Research: 0.3 Determinism Validation Harness Design (Phase 0 Tier 1)

**Date Researched**: 2026-05-25  
**Priority**: Phase 0 Tier 1 — Critical (underpins every correctness claim, mass conservation proof, event reproducibility, and BYTES_ARE_LAW / ArcSystems alignment)  
**Status**: Resolved with concrete decisions. Implementation implications documented. Ready for Phase 0 integration.

## Cross-Referenced Documents (Knowledge Base — Read Before Any External Research)

- `12_RESEARCH_NEEDED/RESEARCH_AGENDA.md` (5.2 exact requirements: CPU reference vs GPU bit-for-bit, brush action sequences, visualization/diff tools)
- `12_RESEARCH_NEEDED/RESOLVED_0.1_Event_Ledger_WorldEvent_Data_Model.md` (events must be deterministic; same seed + brush sequence produces identical event stream)
- `12_RESEARCH_NEEDED/RESOLVED_0.2_Mass_Conservation_Verification_Strategy.md` (mass deltas must be reproducible; verification becomes an automated assertion inside the harness)
- `03_MATH_FOUNDATIONS/CELLULAR_AUTOMATA_RULES.md` (full "Determinism Requirements Across All Phases" section: hash_spatial_sync with single global seed, fixed-point JFA only, thresholds as constants/uniforms, explicit boundary handling)
- `03_MATH_FOUNDATIONS/JUMP_FLOOD_ALGORITHM.md` (integer squared Euclidean only; "never length(f32)")
- `12_RESEARCH_NEEDED/RESOLVED_1.2_Fixed_Point_JFA.md` (exact integer dist_sq_fixed implementation and collapse decision rules)
- `02_ARCHITECTURE/HIGH_LEVEL_ARCH.md` and `02_ARCHITECTURE/PIPELINE_PHASES.md` (determinism as non-negotiable success criterion for Phase 1+; single buffer flip to eliminate races)
- `00_PROJECT_VISION/VISION.md` (bit-perfect reproducibility is a top-level success criterion)
- All state packing, pipeline, and symmetric gravity references in the knowledge base

Complete cross-reference performed before any external lookup.

## External Sources Consulted (After Cross-Reference)

- Common patterns in deterministic GPU simulation engines (record/replay of input streams + dual CPU/GPU execution for validation).
- Fixed-point and global-seed PRNG techniques for CA on GPUs (warp-consistent hash functions).

## Core Decisions (Adopted — Decisive Language Only)

We will adopt a **record-and-replay Determinism Validation Harness** as a permanent, first-class component of the AntVerse test suite from Phase 0 onward.

We will adopt the following **exact reproducibility contract**:

Given:
- A single global `seed: u32` (in SimParams)
- An ordered sequence of brush actions (each action = {frame, x, y, material, radius or stamp pattern, optional strength})
- Grid dimensions and all compile-time constants

The following outputs **must be bit-identical** between a pure CPU reference implementation and the GPU implementation:
1. The final grid state (every u32 cell value) after the last frame.
2. The complete sequence of WorldEvents (including FrameSummary mass values) emitted across all frames.
3. The per-frame total mass history (from the 0.2 definition).

Any divergence is a failing test. The harness must report the first diverging frame and the exact cells (or events) that differ.

We will adopt a **dual-path execution model** inside the harness:

- **CPU Reference Path**: A pure-Rust (or scalar WGSL run on CPU via wgpu if desired for simplicity) implementation of the exact same local rules, using the identical `hash_spatial_sync` function (ported to Rust with the same integer math and no platform-specific float behavior). This path never uses atomics, never uses parallelism that could reorder operations, and executes cells in a strictly deterministic order (e.g., row-major or the exact order the GPU would logically see after cross-warp hash agreement).
- **GPU Path**: The real wgpu pipeline (single CommandEncoder, all passes, event ledger, etc.).

We will adopt **input recording** as the primary test authoring method: A simple script or binary log of (frame, brush_action) tuples. The harness replays the identical log on both paths.

We will adopt **three levels of comparison** (configurable per test):

1. Full grid bit-for-bit diff (strongest, used for small-to-medium test cases and regression suite).
2. Event stream identity + mass history identity (very strong; catches most logic bugs even on large grids where full grid diff is slow).
3. Statistical / perceptual (pile shapes, flow reach, collapse timing) — only for visual regression, never as the sole determinism proof.

We will adopt **snapshot + diff tooling**:
- On divergence the harness writes two side-by-side grid dumps (or PNG visualizations using material + moisture encoding) plus the first differing frame number and a list of differing cell coordinates.
- Because of unified memory, the GPU grid buffer can be mapped directly for the diff without extra copies in most cases.

We will adopt **integration with mass (0.2) and events (0.1)**: Every determinism test run automatically enables the mass verification layer and asserts that the entire event stream (including all FrameSummary mass values) is identical between CPU ref and GPU. This turns 0.1 and 0.2 into continuously exercised properties inside 0.3.

## Harness Architecture (Adopted)

**Core components** (all research artifacts only; no code written):

- `DeterminismHarness` (Rust struct) that owns:
  - A CPU reference simulator (struct or trait that implements one step given current grid + params + brush actions for that frame).
  - The real GPU `AntVerseEngine` (or thin wrapper).
  - An `InputLog` (Vec of timed brush actions + initial seed + grid size).
  - Snapshot buffers (last known good grid + event log + mass history) for both paths.

- Test runner that:
  1. Seeds both simulators identically.
  2. For each recorded frame: apply identical brush actions (CPU ref applies them synchronously; GPU queues them for the next submit).
  3. Steps both (CPU ref does a full serial pass; GPU does its normal submit).
  4. After the step, compares using the chosen level.
  5. On mismatch: dumps artifacts and fails the test (or halts with full provenance for interactive debugging).

- Input log format: simple, human-readable + binary. Example text form:
  ```
  seed: 0xDEADBEEF
  size: 512 512
  frame 12 brush 240 180 MAT_WATER 8
  frame 47 brush 300 210 MAT_DIRT 3
  ```

- Visualization helper: dumps differing cells as a text grid or color-coded PNG (differing cells highlighted, with before/after values).

## Implementation Implications (No Code Edits Performed)

- The CPU reference simulator **must** be kept in 1:1 sync with the WGSL rules at all times. Any rule change (Phase 1 symmetric gravity, Phase 2 collapse, etc.) requires a matching CPU ref update before the GPU implementation is considered valid.
- The `hash_spatial_sync` function (and its Rust port) becomes the single source of truth for all tie-breaking; it must live in a shared location (e.g., a small Rust module + WGSL include or const-eval equivalent).
- Event emission (0.1) and mass accounting (0.2) become mandatory side-effects of the CPU ref step as well, so the three layers are verified together.
- The harness gives us a safe way to develop large-grid features: write and validate the CPU ref first on small grids, then enable the GPU path and demand bit-identity.
- 09_TESTING_AND_VALIDATION/ folder will host the harness, the input log corpus, and the diff visualizer.
- Because the CPU ref is deliberately serial and simple, it will be slow on large grids — the harness therefore supports "subset verification" (run full determinism on a 256² or 512² region while the full 4096² runs on GPU only for visual/ perf testing).

## Open Questions / Risks (Documented — No Guesses)

- Performance of the CPU reference on 2048²+ during long replay tests (may require running the harness only on 512²–1024² canonical test cases and using event+mass identity for larger grids). Measurement during Phase 0 will decide the policy.
- Whether to keep the CPU ref as pure Rust or as a scalar WGSL shader dispatched to a wgpu device with 1×1 workgroups (latter reuses more code but may be slower and less "reference-like").
- Exact storage format and versioning for input logs and golden snapshots (must survive rule changes without constant regeneration).
- The authoritative RESEARCH_ROADMAP.md containing the exact "How to Execute This Research Roadmap (Strict Protocol for Agent Orchestration)" section and full Mandatory Research Output Template text is absent from the local knowledge base, source zip, Grok memory, and connected MCP sources. This deliverable was produced using the protocol description in the user query + established RESOLVED format + all cross-referenced local documents (including 0.1 and 0.2).

These are real gaps that will be closed during harness implementation (research only).

## Recommended Next Steps (After This Resolution)

1. Create the CPU reference simulator (scalar, deterministic, side-by-side with the WGSL rules) as the first piece of the harness.
2. Implement the input log recorder/player and the three-level comparator.
3. Add a small corpus of canonical determinism tests (empty world + single brush stroke, water pouring on dirt, small dirt ceiling collapse once JFA is in, etc.).
4. Wire the harness so that `cargo test -- determinism` runs the full suite and fails on any divergence (including mass and event mismatches).
5. Use the harness as the primary guardrail while implementing Phase 1 symmetric gravity and Phase 2 JFA.

## Conclusion

We will adopt the record-and-replay Determinism Validation Harness with dual CPU-reference + GPU execution, three comparison levels, and mandatory integration of the 0.1 event stream and 0.2 mass verification as the permanent foundation for all correctness claims. Bit-perfect reproducibility is now an automated, continuously enforced property rather than a manual hope.

The design is concrete and directly satisfies 5.2 while strengthening 0.1, 0.2, and the core symmetric rules.

**Deliverable Location**: `12_RESEARCH_NEEDED/RESOLVED_0.3_Determinism_Validation_Harness_Design.md` (this file)

**Related Agenda Items Resolved**: 5.2 (primary). Strengthens 4.1, 5.1, and all determinism requirements in CELLULAR_AUTOMATA_RULES.md and architecture docs.

---
**We will accept no implementation as correct until it produces bit-identical grids, event streams, and mass histories with its CPU reference on every recorded input sequence. The harness makes that rule executable.**