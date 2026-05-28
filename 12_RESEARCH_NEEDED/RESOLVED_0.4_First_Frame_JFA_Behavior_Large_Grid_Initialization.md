# Resolved Research: 0.4 First-Frame JFA Behavior & Large-Grid Initialization Policy (Phase 0 Tier 1)

**Date Researched**: 2026-05-25  
**Priority**: Phase 0 Tier 1 — Essential (directly affects visual quality, structural plausibility, and user experience on the target 2048²–8192² grids from day one)  
**Status**: Resolved with concrete decisions. Implementation implications documented. Ready for Phase 0 foundation and Phase 2 JFA work.

## Cross-Referenced Documents (Knowledge Base — Read Before Any External Research)

- `03_MATH_FOUNDATIONS/JUMP_FLOOD_ALGORITHM.md` (dedicated "Convergence & First-Frame Behavior" section: log₂(N) passes, INF/large distances on frame 0 cause immediate collapse of unanchored dirt, "often desirable", options listed: accept collapse, extra passes on startup, or temporary raised threshold for important geometry)
- `00_PROJECT_VISION/DGX_SPARK_ADVANTAGES.md` (explicit risk callout: "First-Frame Convergence: JFA needs several passes to propagate. On very large grids this is still fast, but we must handle the first 1–2 seconds gracefully (or pre-compute a stable starting state)")
- `02_ARCHITECTURE/PIPELINE_PHASES.md` (Phase 2 success criteria: "No first-frame explosion or non-convergence artifacts after initial stabilization")
- `02_ARCHITECTURE/HIGH_LEVEL_ARCH.md` ("First-frame JFA convergence: either accept partial collapse on frame 0 or pre-stabilize important structures")
- `12_RESEARCH_NEEDED/RESEARCH_AGENDA.md` (1.5 JFA Convergence, First-Frame Behavior, and Large-Grid Stability: formal/empirical pass count analysis, safe first 1–3 frame strategies, production "pre-stabilizing" techniques)
- `12_RESEARCH_NEEDED/RESOLVED_0.1_Event_Ledger_WorldEvent_Data_Model.md`, `RESOLVED_0.2_Mass_Conservation_Verification_Strategy.md`, and `RESOLVED_0.3_Determinism_Validation_Harness_Design.md` (first-frame behavior must be fully reproducible via the harness; any collapses must preserve mass exactly and emit correct events)
- `03_MATH_FOUNDATIONS/CELLULAR_AUTOMATA_RULES.md` and `RESOLVED_1.2_Fixed_Point_JFA.md` (exact collapse decision rules and integer distance math that will execute on the immature distance field)
- All architecture and vision documents describing emergent dramatic collapses as a desired feature

Full cross-reference of the knowledge base completed before external research.

## External Sources Consulted (After Cross-Reference)

- Production practices in voxel engines and structural simulation (Minecraft worldgen, Teardown, Noita-style engines) for first-frame / world-load stabilization of large destructible geometry.
- JFA convergence literature (theoretical log₂(N) vs practical early-exit when no updates occur in a pass).

## Core Decisions (Adopted — Decisive Language Only)

We will adopt **"Embrace First-Frame Collapse as Default Behavior"** for the large-grid AntVerse world model.

We will adopt the following **exact policy** for all grids 2048² and larger:

**Default (unmarked dirt / procedural or player-modified terrain)**:
- Run the normal JFA + collapse sequence (12 passes for 4096² + 2 safety passes) on the very first frame.
- Any dirt cell whose distance to the nearest anchor exceeds its material max_span after these passes collapses into sand exactly as it would in steady state.
- This produces natural settling of loose or over-spanned dirt on world load / first interaction. It is visually correct and physically plausible.

**For "important" or hand-authored structures** (designer wants them standing at t=0):
- The world loader / editor marks the relevant dirt cells with a temporary "pre_stabilize" flag (one of the 8 flag bits in the u32 cell packing, or a small auxiliary u8 buffer).
- For the first K frames (K=30 default, configurable), these marked cells use a temporarily elevated collapse threshold (2× normal max_span, or absolute "never collapse in first K frames" mode).
- After K frames the flag is automatically cleared and normal thresholds apply. The structure can then fail organically if later conditions (moisture, new excavations, etc.) make it unstable.
- Alternative (cheaper for very large authored worlds): run one extra "stabilization-only" JFA block (full log₂(N) + safety passes with collapse disabled) immediately after world load and before the first interactive frame. This lets distance information fully propagate without any structural change, then normal operation begins on a now-converged field.

We will adopt **empirical + safety-margin pass count** as the permanent rule:

- For any grid dimension D, the normal JFA block always runs `ceil(log2(max(D))) + 2` passes (the +2 is the mandatory safety margin adopted after cross-reference of JFA literature and large-grid behavior notes).
- The harness (0.3) includes a dedicated JFA convergence test that runs until the distance field is stable (no cell updated in a full pass) and records the actual passes required for representative anchor distributions. This number is used to validate or adjust the formula.

We will adopt **"Structurally Valid Initial State" requirement** for all world generators, loaders, and brush tools:

- Any initial or edited state that the creator intends to be stable at t=0 must either:
  - Place anchors (stone or connected dirt clusters) such that no dirt exceeds its max_span, or
  - Explicitly mark the cells for pre_stabilization / first-frame relaxation.
- The engine will never silently repair illegal initial states except through the documented first-frame collapse or stabilization mechanisms. Illegal states are the creator's responsibility to correct; the simulation will simply react according to the rules.

We will adopt **integration with the other Phase 0 Tier 1 decisions**:
- First-frame collapses must emit the correct `DirtBecameSand` / `StructuralChange` events (0.1) and produce zero unexplained mass delta (0.2).
- The determinism harness (0.3) must be able to reproduce first-frame behavior exactly, including the number of JFA passes executed and the exact set of cells that collapse on frame 0.

## Large-Grid Initialization Policy (Adopted)

**World creation / load path** (Phase 0+):
1. Generate or load the initial grid (materials + any pre-placed moisture/scent).
2. (Optional but recommended for large authored content) Run the one-time extra stabilization JFA block with collapse disabled if the "pre_stabilize_on_load" flag is set for the world.
3. Begin normal per-frame pipeline (JFA + collapse + gravity + ...).
4. Any remaining over-span dirt collapses naturally on frame 0 or the first few frames.

**Interactive editing (brush)**:
- Normal brush application during runtime uses the normal JFA + collapse rules with no special first-frame treatment.
- A special "place stable structure" brush mode (future) can set the pre_stabilize flag on the affected region for the next K frames.

This policy keeps the core rules simple and uniform while giving creators explicit, documented control over first-frame drama.

## Implementation Implications (No Code Edits Performed)

- **State packing (STATE_PACKING.md)**: Reserve one flag bit (e.g., bit 8) as "pre_stabilize" for Phase 2 JFA work. The bit is ignored by Phase 0–1 logic.
- **JFA pipeline (when implemented in Phase 2)**: The JFA init/step/collapse shaders must read the pre_stabilize flag (or auxiliary buffer) and apply the temporary threshold multiplier or skip-collapse rule for the first K frames. K and the multiplier are SimParams uniforms.
- **World loader / editor tooling**: Must expose the pre_stabilize marking and the "run extra stabilization passes on load" option. These become part of the saved world metadata.
- **0.3 Harness**: Add first-frame-specific test cases (large floating dirt platforms, long bridges, towers) that assert both the visual outcome and exact reproducibility of collapse sets.
- **0.2 Mass verification**: First-frame collapses are legal mass-preserving material changes; the verification layer must treat them as such.
- **Performance note on DGX Spark**: 12 + 2 passes on 4096² is still very fast in unified memory. The optional extra stabilization block on load is a one-time cost measured in milliseconds.
- **07_TELEMETRY_AND_OBSERVABILITY/** and future layers will reference this policy when discussing "world load" vs "runtime" structural events.

## Open Questions / Risks (Documented — No Guesses)

- Exact default value of K (number of relaxed frames) and the threshold multiplier (2× vs "infinite for first K") — will be tuned visually during Phase 2 JFA implementation using the determinism harness. Current defaults (K=30, 2×) are starting points only.
- Whether the pre_stabilize flag lives in the main u32 cell or a separate small buffer (the latter is cleaner for Phase 3+ but adds a bind group). Decision deferred until state packing is revisited for Phase 2.
- The authoritative RESEARCH_ROADMAP.md containing the exact "How to Execute This Research Roadmap (Strict Protocol for Agent Orchestration)" section and full Mandatory Research Output Template text is absent from the local knowledge base, source zip, Grok memory, and connected MCP sources. This deliverable was produced using the protocol description in the user query + established RESOLVED format + all cross-referenced local documents (including 0.1–0.3).

These are tracked gaps to be closed with empirical data during JFA bring-up (research artifacts only).

## Recommended Next Steps (After This Resolution)

1. Reserve the pre_stabilize flag bit in the Phase 0 state packing document.
2. When JFA work begins (Phase 2), implement the two modes (temporary raised threshold vs one-time extra non-collapsing stabilization block) behind the policy defined here.
3. Add first-frame collapse test cases to the determinism harness (0.3) using representative large-grid anchor patterns.
4. Exercise the policy on 4096²+ test worlds and record actual observed collapse counts and visual quality for different K / multiplier values.
5. Document the final tuned defaults in CELLULAR_AUTOMATA_RULES.md and JUMP_FLOOD_ALGORITHM.md once empirical data exists.

## Conclusion

We will adopt "Embrace First-Frame Collapse as Default" plus an explicit, flag-driven pre_stabilization mechanism for important structures, combined with a mandatory safety-margin pass count and a "structurally valid initial state" creator responsibility. This policy is simple, aligned with the desired emergent drama, fully reproducible via the 0.3 harness, mass-safe via 0.2, and event-rich via 0.1.

The first-frame behavior and large-grid initialization rules are now concrete and will not be a source of surprises when JFA is introduced in Phase 2.

**Deliverable Location**: `12_RESEARCH_NEEDED/RESOLVED_0.4_First_Frame_JFA_Behavior_Large_Grid_Initialization.md` (this file)

**Related Agenda Items Resolved**: 1.5 (primary). Strengthens Phase 2 success criteria and all first-frame notes across the architecture and math documents.

---
**We will treat first-frame JFA behavior as a deliberate, documented, and reproducible feature rather than an artifact to be hidden. The initialization policy gives creators precise control while preserving the "large world settles naturally" aesthetic.**