# Resolved Research: 0.2 Mass Conservation Verification Strategy (Phase 0 Tier 1)

**Date Researched**: 2026-05-25  
**Priority**: Phase 0 Tier 1 — Critical (mathematical foundation for all future layers; non-negotiable for BYTES_ARE_LAW, ArcSystems, and CedeSystem trust)  
**Status**: Resolved with concrete decisions. Implementation implications documented. Ready for Phase 0 integration.

## Cross-Referenced Documents (Knowledge Base — Read Before Any External Research)

- `03_MATH_FOUNDATIONS/CELLULAR_AUTOMATA_RULES.md` (full "Mass Conservation Strategy" section: symmetric pull/vacate 1:1 guarantee for water in Phase 0–1; collapse preserves mass as material change; absorption tracked exactly as moisture volume; explicit mass-accounting telemetry required in Phase 4)
- `12_RESEARCH_NEEDED/RESOLVED_0.1_Event_Ledger_WorldEvent_Data_Model.md` (FrameSummary event **must** carry mass deltas; "any non-zero unexplained delta is a visible bug in the event log"; mandatory per-frame summary)
- `12_RESEARCH_NEEDED/RESEARCH_AGENDA.md` (5.1 exact requirements: automated counts every N frames, assert within bounds, detect/localize leaks)
- `02_ARCHITECTURE/PIPELINE_PHASES.md` (Phase 1 success criteria: "Perfect mass conservation (no spontaneous creation/destruction of water)"; Phase 3: absorbed water accounted as moisture)
- `02_ARCHITECTURE/HIGH_LEVEL_ARCH.md` (mass conservation listed under determinism requirements; single buffer flip at end of frame to avoid race-induced loss)
- `00_PROJECT_VISION/VISION.md` and `DGX_SPARK_ADVANTAGES.md` (unified coherent memory makes full-grid verification reads near-zero cost; 128 GB allows multiple full-resolution snapshots resident)
- `03_MATH_FOUNDATIONS/JUMP_FLOOD_ALGORITHM.md` (collapse converts dirt → sand; mass must be preserved exactly as material ID change)
- All prior RESOLVED files and `05_CODE_TEMPLATES/wgsl/phase0_classic.wgsl` (current state model)

All gaps identified from these documents first. External research performed only after complete cross-reference.

## External Sources Consulted (After Cross-Reference)

- Standard techniques in production particle and CA engines for exact mass accounting (parallel reduction sums + host verification snapshots in unified-memory or high-bandwidth architectures).
- Fixed-point vs floating-point accumulation error analysis for large grids (4096²+).

## Core Decisions (Adopted — Decisive Language Only)

We will adopt a **two-layer mass conservation verification strategy** that runs from the first frame of Phase 0 and remains active for the life of the project.

We will adopt the following **exact definition of conserved mass** (used in all verification):

- Each solid or mobile cell (Sand, Dirt (dry or wet), Stone, Water) contributes exactly 1.0 unit.
- Water volume (bits 16-23 when material is Water) contributes (volume / 255.0).
- Moisture stored in Dirt cells (bits 16-23 when material is DryDirt or WetDirt) contributes (moisture / 255.0) — this represents water that has been absorbed but not destroyed.
- Air and all other states contribute 0.0.
- Total mass = sum of the above across the entire grid. This definition is invariant under all legal transitions in the symmetric design.

We will adopt **Layer 1 (Continuous Verification)**: After every physics submit (or every N frames, default N=1 in debug, N=10 in normal), the host performs an exact full-grid scan over the unified memory buffer and computes the precise total mass using the definition above. The result is compared against:
- The previous frame's verified total, plus
- The net delta reported in the FrameSummary event emitted for that frame (from the 0.1 decision).

Any discrepancy > 0 (using exact integer accumulation where possible, or a tiny epsilon only for the 8-bit fractional fields) is treated as a hard failure.

We will adopt **Layer 2 (Localization & Replay)**: The host keeps one "last known good" full grid snapshot in unified memory (cost: 4096² × 4 bytes = 64 MB — trivial). On any Layer 1 failure, the system can:
- Binary-search backward through saved snapshots (or re-simulate from a checkpoint using the determinism harness from 0.3) to find the exact frame where divergence first appeared.
- Re-run that single frame with per-pass mass accounting (optional lightweight additive deltas emitted from each compute pass) to identify the exact rule or pass responsible for the leak.

We will adopt **GPU-side parallel reduction** as an optional high-frequency early-warning layer: a cheap dedicated compute pass (or integrated at the end of the last physics pass) that produces a single f32/u32 total mass value via workgroup reductions + atomic add to a single location. This runs every frame with negligible cost on unified memory and provides an independent check before the full host read.

We will adopt **integration with the Event Ledger** (0.1): The mandatory FrameSummary event (type 0) **always** carries before_mass and after_mass (as two u32 fixed-point values with 16.16 format or two f32 with explicit note that they are for telemetry only). All individual absorption/transition events contribute to an expected delta that the summary must match exactly.

We will adopt **zero-tolerance policy in debug builds**: Any mass delta not explained by a documented transition is a fatal error that halts the simulation with full provenance (frame, pass, approximate location via the last event stream).

## Exact Mass Accounting Rules (Adopted)

**Per-frame invariant**:
expected_after = before + sum(legal_deltas from all emitted events in the frame)
If computed_after != expected_after → violation.

**Legal deltas** (all must be zero-sum by construction in the symmetric design):
- Water move (pull/vacate): +0 / -0 (pure relocation).
- Water absorbed by dirt: water volume decreases by X → dirt moisture increases by X (exact 1:1 in the 8-bit fields).
- Dirt collapses to Sand: material ID change only; the cell still contributes 1.0.
- Evaporation (Phase 3+): explicitly documented as a controlled sink with a matching negative delta in the summary.

**Brush / user input**: Every brush operation must emit a Brush event (0.1) that records exact net mass added or removed so the verification layer can account for it.

## Implementation Implications (No Code Edits Performed — Research Only)

- **Event Ledger (0.1 deliverable)**: FrameSummary payload must be extended (or use the existing payload byte + additional fields if we widen the struct later) to carry the two mass values. The 0.1 decision already requires this; this task ratifies the exact fixed-point format.
- **Host Rust side**: After queue.submit, map/read the final physics buffer (already resident in unified memory), run the exact sum using the definition above, compare against previous + FrameSummary delta. Maintain one rolling "last known good" snapshot buffer.
- **Pipeline ordering (PIPELINE_PHASES.md)**: The verification read happens after the single buffer flip / final physics pass but before or during render. It is part of the linear sequence.
- **Debug vs Release**: Configurable cadence and "halt on violation" flag live in SimParams or a debug uniform. Release builds can skip the full host scan and rely only on the GPU reduction + periodic spot-checks.
- **0.3 Determinism Harness** (next task): The same snapshot + replay mechanism is the foundation for CPU reference vs GPU bit-for-bit comparison. Mass verification becomes one of the automated assertions inside the harness.
- **07_TELEMETRY_AND_OBSERVABILITY/**: This document seeds concrete mass telemetry requirements for that folder.
- **Performance**: On DGX Spark, a full 4096² exact scan (simple loop over 16M u32s with 3-4 bit tests + fractional adds) is microseconds. Multiple snapshots fit comfortably.

## Open Questions / Risks (Documented — No Guesses)

- The precise fixed-point format for the mass values inside FrameSummary (u32 16.16 vs two separate u16 counts vs f32 with documented tolerance) will be finalized during 0.1 + 0.2 implementation coordination. The current 0.1 struct uses a small payload byte; widening or adding a second summary event may be required.
- Whether the optional GPU reduction pass is worth the extra dispatch (even if cheap) versus relying entirely on host unified-memory scans — measurement will decide in Phase 0.
- Interaction with future floating-point layers (heat, velocity) that could introduce non-exact conservation — those layers will be required to declare their own conservation invariants and verification rules when introduced (Phase 5+).
- The authoritative RESEARCH_ROADMAP.md containing the exact "How to Execute This Research Roadmap (Strict Protocol for Agent Orchestration)" section and full Mandatory Research Output Template text is absent from the local knowledge base, source zip, Grok memory, and connected MCP sources (github + notion searches returned no matches). This deliverable was produced using the protocol description in the user query + established RESOLVED format + all cross-referenced local documents.

These risks are tracked and will be closed during the implementation of the verification layer itself (research artifact only; no code written here).

## Recommended Next Steps (After This Resolution)

1. Implement the host-side exact mass scanner + comparison against FrameSummary deltas as part of the Phase 0 Rust host (using the unified buffer).
2. Wire the before/after mass values into the FrameSummary emission (extending the 0.1 model as needed).
3. Add the optional GPU reduction sum pass behind a feature flag for continuous early warning.
4. Use the mass verification as the first automated assertion inside the determinism harness (Task 0.3).
5. Exercise with extreme brush + flow + absorption scenarios on 2048² and 4096² to prove zero unexplained deltas under the symmetric rules.

## Conclusion

We will adopt the two-layer (continuous host exact count + localization snapshots + GPU reduction early warning) verification strategy with the precise mass definition above, integrated directly into the FrameSummary event from Task 0.1. This makes mass conservation a continuously observable, automatically asserted property from the very first frame of Phase 0 rather than a Phase 4 afterthought.

The strategy is now concrete, mathematically aligned with the symmetric design in CELLULAR_AUTOMATA_RULES.md, and ready for implementation. No further research is required before coding the verification layer.

**Deliverable Location**: `12_RESEARCH_NEEDED/RESOLVED_0.2_Mass_Conservation_Verification_Strategy.md` (this file)

**Related Agenda Items Resolved**: 5.1 (primary). Contributes to 4.1 (via FrameSummary) and 5.2 (via shared snapshot/replay infrastructure).

---
**We will treat any unexplained mass delta as a fatal, reproducible bug from Phase 0 onward. The verification strategy makes that guarantee enforceable.**