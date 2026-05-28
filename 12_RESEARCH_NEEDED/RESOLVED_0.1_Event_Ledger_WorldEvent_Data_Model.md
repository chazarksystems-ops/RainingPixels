# RESOLVED: 0.1 Event Ledger / WorldEvent Data Model (Phase 0 Tier 1)

**Date**: 2026-05-25  
**Task ID**: 0.1  
**Status**: Resolved  
**Sources**:
- `00_PROJECT_VISION/RESEARCH_ROADMAP.md` (Task 0.1 definition, strict protocol, Mandatory Research Output Template)
- `02_ARCHITECTURE/PHASE0_DEMO_IMPLEMENTATION_PLAN.md` (explicit requirements: 16-byte WorldEvent, double-buffered atomic append, FrameSummary type 0 with mass deltas, try_emit_event WGSL helper, Phase 0.3 integration)
- `12_RESEARCH_NEEDED/RESEARCH_AGENDA.md` (4.1 Lightweight Event Emission requirements; 5.1/5.2 cross-links to mass and determinism)
- `12_RESEARCH_NEEDED/RESOLVED_0.2_Mass_Conservation_Verification_Strategy.md` (mandatory FrameSummary before/after mass in fixed-point; integration contract with 0.1)
- `12_RESEARCH_NEEDED/RESOLVED_0.3_Determinism_Validation_Harness_Design.md` (events + FrameSummary must be bit-identical in CPU ref vs GPU replays)
- `12_RESEARCH_NEEDED/RESOLVED_0.4_First_Frame_JFA_Behavior_Large_Grid_Initialization.md` (first-frame events must be reproducible)
- `02_ARCHITECTURE/HIGH_LEVEL_ARCH.md` (event ledger as ring buffer in unified memory for Cede/Entelechy consumption)
- `02_ARCHITECTURE/PIPELINE_PHASES.md` (Phase 4 telemetry vision: SandStartedFalling, WaterAbsorbed, DirtCollapsed, StructureFormed + host logging)
- `07_TELEMETRY_AND_OBSERVABILITY/EVENT_STREAM_AS_CREATIVE_MATERIAL.md` (16-byte + FrameSummary assumed; creative sonification and narrative uses)
- `08_INTEGRATION_POINTS/CREATIVE_TOOLING_INTEGRATION_VISION.md` (WorldEvent stream as causality record for agents)
- `04_DESIGN_DECISIONS/STATE_PACKING.md` (u32 cell model; events reference packed material + moisture/volume)
- `04_DESIGN_DECISIONS/VISUAL_AND_INTERACTION_DIFFERENCES.md` (structured WorldEvent stream from first frame as differentiator)
- External: webgpufundamentals.org compute shader histogram lesson (atomicAdd + workgroup-local accumulation patterns)
- External: https://www.reddit.com/r/GraphicsProgramming/comments/196h62p/compute_shader_atomic_buffer_appending_vs_prefix/ (real-world comparison of atomic append vs prefix-sum for dynamic output)
- External: https://toji.dev/webgpu-best-practices/compute-vertex-data.html (atomicAdd best practices, quantization, final buffer readability)
- Prior session memory on atomic<u32> + atomicAdd append buffers for GPU→host telemetry (no pre-existing concrete Cede WorldEvent layout found)

## Core Finding (One Sentence)
We will implement the Phase 0 Event Ledger as a double-buffered atomic-append structure consisting of a leading atomic count paired with an array of fixed 16-byte WorldEvent records, with an unconditional FrameSummary (type 0) emitted every frame carrying before_mass and after_mass as two u32 16.16 fixed-point values plus a small set of deterministic sparse notable events (MaterialTransition type 1, BrushAction type 2), all written via atomicAdd slot reservation inside the single physics CommandEncoder.

## Detailed Findings
- Every cross-referenced document in the knowledge base assumes a 16-byte compact WorldEvent with FrameSummary as the mandatory per-frame heartbeat containing mass deltas; the design is a prerequisite for 0.2 verification, 0.3 determinism, and Cede/Entelechy agent consumption.
- Atomic counter + direct indexed write (append) is the dominant lightweight pattern in WGSL compute for variable or sparse output: atomicAdd on a u32 head reserves a slot with minimal passes; workgroup-local accumulation + single global write reduces contention when many threads may emit.
- Prefix-sum approaches guarantee strict write order but require multiple dispatches or complex intra-frame scans; they add overhead and implementation surface for Phase 0 where intra-frame event order has no semantic meaning (FrameSummary aggregates everything; notables are best-effort sparse records).
- Real-world graphics workloads (particle emission, GPU-driven rendering) successfully use atomic append for millions of items per frame on modern GPUs; contention is manageable when emission is sparse (Phase 0 target: 1 FrameSummary + < 64 notables per frame at 2048²–4096²).
- Unified memory on DGX Spark makes host readback of the prior frame's ledger after submit effectively zero-copy; double-buffering (or slot index in SimParams) ensures the GPU writes to the inactive ledger while the host reads the completed one.
- No concrete WorldEvent or causality struct definition existed in Cede/Entelechy prior work within searchable memory or the current knowledge base; the layout must be derived from AntVerse-specific requirements (mass integration, determinism bit-identity, creative/agent utility, 16-byte cap for performance).
- STATE_PACKING u32 model means events can cheaply reference material IDs, 8-bit moisture/volume deltas, and 2D coordinates packed into 32 bits without widening the event struct.
- FrameSummary must be unconditional and always occupy a reserved or guaranteed slot so that 0.2 mass verification and 0.3 harness have a reliable per-frame anchor regardless of other event volume.

## Concrete Recommendation
We will adopt the atomic counter plus indexed 16-byte WorldEvent write pattern for the Event Ledger in Phase 0 and beyond. We will define WorldEvent exactly as four u32 fields (header, pos, data0, data1) with type in header bits 0-7 (0=FrameSummary, 1=MaterialTransition, 2=BrushAction, 3+=reserved), pos as packed u16 x/y or linear index, and payload fields carrying material deltas, 16.16 fixed-point mass values, or brush net deltas. We will emit exactly one FrameSummary per frame with before/after mass matching the 0.2 definition. We will provide a `try_emit_event` WGSL helper that performs atomicAdd on the active ledger head, checks capacity, and writes the struct. We will double-buffer the ledger (or use an explicit slot index) so the host can read the completed ledger after queue.submit with no stalls. We will expose a host `drain_events()` or equivalent that returns the prior frame's events as a zero-copy slice. We will integrate this from the first runnable frame of Phase 0.3 onward. We will document the full layout, emission rules, and host consumption contract in `07_TELEMETRY_AND_OBSERVABILITY/EVENT_LEDGER_DESIGN.md`.

## Rationale
Atomic append via atomicAdd was chosen over prefix-sum because it is simpler, requires only a single compute dispatch for emission, matches proven patterns from webgpufundamentals and production workloads, and introduces the lowest risk of ordering non-determinism or extra pipeline passes that could jeopardize Phase 0 determinism goals. Prefix-sum was rejected because intra-frame event ordering has zero semantic value for the AntVerse use cases (FrameSummary is an aggregate; notables are sparse and timestamped by frame); the added complexity and passes would violate the "lightweight with negligible overhead" mandate in RESEARCH_AGENDA 4.1. Full host-side grid diffing for event generation was rejected because it violates the requirement for GPU-native emission inside the physics passes, adds unacceptable latency on large grids, and cannot feed real-time Cede/Entelechy agents or creative sonification. Dense per-cell reduction passes were rejected for the same overhead and verbosity reasons; they would flood the ledger and destroy the sparse, semantically meaningful stream needed for agents and verification. The 16-byte size was chosen over wider structs because every existing document (PHASE0_PLAN, RESOLVED 0.2/0.3, creative vision, integration points) explicitly references it as the performance/compatibility target; widening would require re-justification against measured contention data. Making FrameSummary unconditional and mass-carrying satisfies the hard integration contracts in 0.2 and 0.3 without negotiation.

## Implementation Implications
- **Shaders / Pipeline**: Add EventLedger bind group (or optional group) containing the active ledger buffer(s) and SimParams.ledger_slot. Insert `try_emit_event` calls at key transition points inside gravity.wgsl (and future passes). FrameSummary emission occurs unconditionally at the end of the physics chain before the single buffer flip. Keep emission behind a uniform flag for zero-cost disable in perf tests.
- **Host Rust side (src/simulation/events.rs and state.rs)**: Define `#[repr(C)] WorldEvent` + `EventLedger` (head: u32 or separate atomic buffer + events array) with bytemuck. Implement double-buffered or slotted storage buffers sized for target capacity (e.g. 4096 events). After each submit, read the prior slot's head + slice of events, provide `drain_events(&self) -> &[WorldEvent]`. Wire mass values from 0.2 scanner into the FrameSummary payload.
- **Module structure (per PHASE0_DEMO_IMPLEMENTATION_PLAN)**: `src/simulation/events.rs` owns the types, packing helpers, host ledger management. `src/simulation/state.rs` owns the GPU buffers and SimParams extension (event_ledger_index, capacity). `src/simulation/pipeline.rs` binds the ledger and manages slot flipping.
- **Testing / Validation**: Every determinism harness run (0.3) must assert identical event streams (including exact FrameSummary mass pairs) between CPU ref and GPU. Mass verification (0.2) consumes FrameSummary deltas as the expected change. Creative/agent consumers receive the identical stream.
- **New/updated files**: Create `12_RESEARCH_NEEDED/RESOLVED_0.1_Event_Ledger_WorldEvent_Data_Model.md` (this file); create `07_TELEMETRY_AND_OBSERVABILITY/EVENT_LEDGER_DESIGN.md` (authoritative layout + code sketches); update `12_RESEARCH_NEEDED/RESEARCH_AGENDA.md` Current Status and 4.1 sections; reference from PHASE0_PLAN and all dependent RESOLVED notes.
- **Capacity and overflow policy**: Start with a compile-time or uniform capacity of 1024–8192 events per frame. On overflow the summary is still written (reserve slot 0 if necessary); excess notables are dropped silently with a "truncated" flag in the summary. Tune after Phase 0 profiling.
- **Future evolution**: The 16-byte layout is Phase 0–3 stable. Richer payloads for major creative events (Phase 4+) will use a parallel "detail" buffer indexed from the main event or a separate high-priority ledger.

## Open Questions / Risks (if any)
- Exact bit-packing of mass values inside FrameSummary (u32 16.16 vs bitcast f32) will be finalized during Phase 0.3 implementation coordination with the 0.2 scanner; both are acceptable provided the 0.2 definition is followed exactly and documented.
- Contention behavior of the single atomic head under worst-case brush storms on 4096² will be measured in Phase 0.3; if serialization becomes visible we will shard the ledger by workgroup or tile as a documented follow-up (not required for initial demo).
- None of the above risks block Phase 0.0–0.2 work; the data model is now unambiguous and the emission mechanism is proven by external sources.

**We will treat the Event Ledger as a first-class, always-on, deterministic output of the physics engine from the first frame of the Phase 0 demo. The 16-byte atomic-append design is the minimal structure that satisfies mass verification, determinism harness, Cede/Entelechy consumption, and creative sonification requirements simultaneously.**

**Deliverable Location**: `12_RESEARCH_NEEDED/RESOLVED_0.1_Event_Ledger_WorldEvent_Data_Model.md` (this file) and `07_TELEMETRY_AND_OBSERVABILITY/EVENT_LEDGER_DESIGN.md`

**Related Agenda Items Resolved**: 4.1 (primary). Completes Phase 0 Tier 1 research foundation. Enables clean implementation of 0.3 events, 0.2 integration, and 0.3 harness assertions.
