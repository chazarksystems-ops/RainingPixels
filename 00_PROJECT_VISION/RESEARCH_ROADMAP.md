# AntVerse Research Roadmap

**Version**: 1.0  
**Date**: 2026-05-25  
**Purpose**: This document defines the complete research arc for AntVerse. It is designed to be followed systematically so that every major technical decision is grounded in high-quality information, minimizing ambiguity, rework, and assumptions during implementation.

**Philosophy**:
- Research happens **before** or **in parallel with early implementation**, not after.
- Every research item must produce concrete, documented outputs using the **Mandatory Research Output Template**.
- We prioritize items that reduce **risk of major rework** or have high **leverage** on many other parts of the system.
- The roadmap is living — it will be updated as new information emerges.

**Agent Orchestration Notes**:
This document is written to be followed by an autonomous research agent. Instructions are deliberately strict and prescriptive to prevent drift, vague outputs, or premature conclusions. Follow the "How to Execute This Research Roadmap" section exactly. Do not improvise the process.

---

## Overall Research Arc Structure

The research is divided into **6 major phases**, aligned with the implementation phases in `02_ARCHITECTURE/PIPELINE_PHASES.md`.

Each research phase has:
- **Objectives**
- **Key Research Tasks** (with detailed *how-to* instructions)
- **Deliverables**
- **Success Criteria**
- **Dependencies**

---

## Phase 0: Foundation & Validation Research (Current — In Progress)

**Goal**: Establish an unambiguous technical foundation for determinism, pipeline correctness, performance strategy, and observability before writing production code.

### Current Status (as of 2026-05-25)
- Completed deep research on:
  - 1.1 Multi-pass compute pipeline synchronization (implicit sync in single CommandEncoder)
  - 1.2 Fixed-point integer math for JFA
  - 1.3 Shared memory halo tiling + 16×16 workgroup recommendation
- Created resolved research documents in `12_RESEARCH_NEEDED/`

### Remaining Highest-Value Tasks in Phase 0

#### Task 0.1: Event Ledger / WorldEvent Data Model + Emission Strategy
**Why Early**: This is the highest-leverage remaining item. The design of *what* events we emit and *how* we emit them will influence shader architecture (append buffers vs reduction passes vs host polling) and host integration.

**How to Research**:
1. Review existing patterns in your CedeSystem / Entelechy work for `WorldEvent` or causality structures.
2. Study lightweight event emission techniques in modern GPU simulations (append buffers, atomic counters, ring buffers in unified memory).
3. Define a compact enum of event types relevant to falling sand (e.g., `SandStartedFalling`, `DirtCollapsed`, `WaterAbsorbed`, `StructureFormed`).
4. Decide on emission mechanism (shader-side append, post-pass reduction, or host-side diffing).
5. Document the data model and emission strategy in a new file under `07_TELEMETRY_AND_OBSERVABILITY/`.

**Deliverable**: `07_TELEMETRY_AND_OBSERVABILITY/EVENT_LEDGER_DESIGN.md`

#### Task 0.2: Mass Conservation Verification Strategy
**How to Research**:
1. Study techniques used in other high-fidelity CA / fluid simulations for proving mass conservation.
2. Design both CPU reference counting and GPU-side summary buffer approaches.
3. Define acceptable tolerance and leak detection methods.
4. Document in `09_TESTING_AND_VALIDATION/MASS_CONSERVATION.md`

#### Task 0.3: Determinism Validation Harness Design
**How to Research**:
1. Study existing GPU determinism testing patterns (CPU reference implementation + bit-exact comparison).
2. Define how brush actions and initial world state will be recorded for reproducibility.
3. Outline the harness architecture (test framework, comparison logic, visualization of diffs).
4. Document in `09_TESTING_AND_VALIDATION/DETERMINISM_HARNESS.md`

#### Task 0.4: First-Frame JFA Behavior & Large-Grid Initialization Policy
**How to Research**:
1. Analyze behavior of standard JFA on first frame for large grids.
2. Evaluate options: accept initial collapse, run extra stabilization passes, or use special "structurally important" flags.
3. Decide on policy and document visual/behavioral expectations.
4. Update `03_MATH_FOUNDATIONS/JUMP_FLOOD_ALGORITHM.md`

**Phase 0 Success Criteria**:
- All Tier 1 research items from the refined priority list are resolved with concrete, documented decisions.
- Pipeline, determinism, and observability foundations have minimal ambiguity.

---

## Phase 1: Core Physics Implementation Research

**Goal**: Gather all information needed to implement Phase 1 (Symmetric Reverse-Pull Gravity + Hydrostatics + Sand as mobile solid) cleanly and correctly.

### Key Research Tasks

#### Task 1.1: Refined Symmetric Gravity Implementation Details
- Study your original symmetric gravity shader against the new pipeline synchronization findings.
- Identify any remaining edge cases in diagonal advection and lateral dispersion.
- Produce final polished version of `compute.wgsl` (Phase 1) in `05_CODE_TEMPLATES/wgsl/`

#### Task 1.2: Brush System Design (for good "classic falling sand" feel)
- Research best practices from Noita, Powder Toy, Sandspiel, and modern WebGPU examples.
- Decide between stamp shader vs CPU staging vs hybrid.
- Define material payload packing for brush (especially for future rich state).

#### Task 1.3: Initial World Seeding & Large Grid Generation Strategies
- Research efficient ways to initialize large grids (2048²–4096²) with interesting starting structures (floating dirt ceilings, terrain, etc.).
- Consider both CPU generation + upload and pure GPU initialization shaders.

**Phase 1 Deliverables**:
- Final Phase 1 gravity shader + updated pipeline code template
- Brush system design document
- World seeding strategy notes

---

## Phase 2: Structural Integrity Research

**Goal**: Complete all research needed for robust JFA + collapse implementation.

### Key Tasks
- Final validation of fixed-point JFA implementation (already largely resolved in 1.2)
- Performance benchmarking plan for JFA on DGX Spark (number of passes, workgroup config, halo tiling)
- Visual quality criteria for collapse behavior (what makes a collapse look "good" vs broken)
- Edge case handling (very large structures, moisture gradient effects on collapse)

**Deliverables**:
- Updated `collapse.wgsl` template with fixed-point math
- JFA performance tuning notes
- Collapse visual quality guidelines

---

## Phase 3: Capillary, Scent & Observability Research

**Goal**: Research everything needed for moisture, evaporation, scent diffusion, and telemetry integration.

### Key Tasks
- Physically plausible but simple capillary absorption/evaporation models
- Lightweight scent diffusion algorithms suitable for every-frame execution
- Integration of event emission into capillary/gravity passes
- Host-side consumption patterns for events and state (how CedeSystem / agents will read data)

**Deliverables**:
- Final `capillary.wgsl` (with scent)
- Event emission integration guide
- Host integration notes in `08_INTEGRATION_POINTS/`

---

## Phase 4: Testing, Validation & Quality Research

**Goal**: Build confidence that the simulation is correct, deterministic, and high-quality.

### Key Tasks
- Implement and document mass conservation testing harness
- Build determinism validation system (CPU reference + GPU comparison)
- Define perceptual quality metrics beyond raw FPS
- Create visual regression testing approach for shader changes

**Deliverables**:
- Complete testing & validation documents under `09_TESTING_AND_VALIDATION/`
- Reference CPU implementation of core rules (for validation)

---

## Phase 5: Future Layers & Scaling Research

**Goal**: Prepare research foundation for heat, chemistry, biology (ants), multi-scale simulation, and creative tool export.

### Key Tasks (lower urgency)
- Lightweight heat diffusion + phase change models that remain local CA rules
- Simple chemistry (dissolution, crystallization)
- Ant agent layer on top of physics + scent (TWIOFA integration)
- Multi-scale / hierarchical simulation techniques
- Export pipelines (JFA → SDF, stable structures → meshes/assets for Glazier/Seed Forge/Unreal)

---

## Ongoing / Cross-Cutting Research

These run in parallel throughout the project:

- **DGX Spark Hardware Tuning**: Power/thermal characteristics, sustained occupancy, NVLink-C2C best practices for CA workloads.
- **Performance Profiling & Optimization**: Continuous measurement of bandwidth vs compute, shared memory effectiveness, workgroup sizing.
- **Documentation & Knowledge Capture**: Every significant finding must be written into the appropriate folder with clear rationale and sources.

---

## How to Execute This Research Roadmap (Strict Protocol for Agent Orchestration)

### Mandatory Research Output Template (Use This For Every Task)

Every research task **must** produce output that follows this structure. Do not deviate.

```markdown
# RESOLVED: [Task Name]

**Date**: YYYY-MM-DD
**Task ID**: [e.g. 0.1, 1.2]
**Status**: Resolved | Partially Resolved | Blocked (with reason)
**Sources**:
- [Link or citation 1]
- [Link or citation 2]

## Core Finding (One Sentence)
[Single, unambiguous sentence stating the decision or key insight]

## Detailed Findings
- Bullet 1
- Bullet 2
- ...

## Concrete Recommendation
[Specific, actionable recommendation. Use "We will..." or "Adopt..." language. No "could", "should consider", or "maybe".]

## Rationale
Why this over alternatives. Explicitly name at least one alternative that was rejected and why.

## Implementation Implications
- Impact on shaders / pipeline
- Impact on host code
- Impact on testing / validation
- New files or sections to create/update

## Open Questions / Risks (if any)
- Only list true remaining unknowns. If none, write "None".
```

### General Process for Each Research Task (Do Not Skip Steps)

**Step 1: Identify & Scope**
- Read the task description in this roadmap.
- Read any linked/related documents mentioned (e.g. `RESEARCH_AGENDA.md`, previous RESOLVED files).
- Explicitly write down: "What decision am I trying to enable?"

**Step 2: Information Gathering (Mandatory Minimum)**
- Perform at least one broad `web_search`.
- Browse at least 2–3 high-quality sources from the results using `browse_page` (with specific instructions).
- Review relevant existing code in `05_CODE_TEMPLATES/` and prior resolved research.
- If the topic relates to your past work (CedeSystem, Entelechy, PlasmaForge, TWIOFA), explicitly search your own context/memory for relevant patterns.

**Step 3: Synthesis & Decision**
- Synthesize into the **Mandatory Research Output Template** above.
- Force yourself to make a decision. If information is conflicting, document the conflict and pick the option with the strongest supporting evidence + lowest risk to determinism / mass conservation / DGX performance.
- Never leave a task as "more research needed" without a clear next step and owner.

**Step 4: Artifact Creation / Update**
- Create or update the exact deliverable file listed in the task.
- If creating a new RESOLVED file, place it in `12_RESEARCH_NEEDED/RESOLVED_[TaskID]_[ShortName].md`.
- Update `RESEARCH_AGENDA.md` to mark the item resolved with a link to the new file.
- If the finding affects architecture, math, or design, also update the canonical document in `02_ARCHITECTURE/`, `03_MATH_FOUNDATIONS/`, or `04_DESIGN_DECISIONS/`.

**Step 5: Validation**
- Re-read the new/updated document(s).
- Ask: "Could a future agent or version of me misinterpret this and make a wrong assumption?" If yes, clarify further.

**Step 6: Close the Loop**
- Update this `RESEARCH_ROADMAP.md` only if a new high-value dependency or phase adjustment is discovered.
- Move to the next task in priority order.

### Anti-Patterns (Strictly Forbidden)

- Do **not** produce vague recommendations ("we should consider...", "it might be good to...").
- Do **not** skip the Mandatory Research Output Template.
- Do **not** research in a vacuum — always cross-reference existing documents in this knowledge base first.
- Do **not** declare a task "done" if there is still material ambiguity in the core decision.
- Do **not** jump ahead to later phases until Phase 0 Tier 1 tasks are resolved.

### Documentation Standards (Enforced)

Every document must contain:
- Clear **Sources** section with links/citations.
- **Concrete recommendations** using decisive language.
- Explicit **Rationale** (including rejected alternatives).
- **Implementation Implications** section.
- Dates and task IDs for traceability.

### When to Stop Researching a Topic and Move Forward

Stop when you have:
- A concrete, documented decision.
- Clear implementation implications.
- Acceptable risk documented (or "None").
- The Mandatory Research Output Template filled out.

If you cannot reach a decision after reasonable effort, document the blocking conflict clearly and escalate (do not silently move on).

---

## Success Criteria for the Overall Research Arc

By the end of the research phases, the knowledge base should allow a competent developer (or future version of you) to implement the full engine with minimal ambiguity, knowing:

- Why every major technical decision was made
- How to validate correctness (mass, determinism)
- How to achieve good performance on DGX Spark
- How the system integrates with your broader ArcSystems / creative tooling ecosystem

---

**This roadmap is the master plan.** Follow it systematically. Update it when better information emerges. The goal is not speed of research, but **reduction of downstream implementation risk and ambiguity**.

The zip package you requested contains the current state of this entire knowledge base as of 2026-05-25.
