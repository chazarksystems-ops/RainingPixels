# AntVerse Research Agenda

**Purpose**  
This document catalogs every area where deeper research, references, benchmarks, or external knowledge will make development smoother, more correct, and more aligned with your deterministic computing and creative goals.  

We will treat this as a living backlog. Before or during each implementation phase, we will pull items from this list, research them (using web search, papers, existing codebases, hardware docs, etc.), and incorporate the findings into the relevant architecture, math, or code documents.

The more high-quality information we gather here, the fewer surprises and rework we will encounter during coding.

---

## Priority 1: Core Implementation Correctness & Performance (Do These First)

### 1.1 Multi-Pass Compute Pipeline Synchronization in wgpu
- Best practices for linear sequences of compute passes with read-after-write dependencies on the same buffers.
- When (and when not) to use explicit pipeline barriers vs relying on submit ordering.
- Memory visibility and coherence guarantees on NVIDIA (especially unified memory architectures like DGX Spark).
- Recommended patterns for "current read index" vs multiple bind groups.

**Why it matters**: Your original audit correctly identified intertwined buffer flipping as a critical bug. We need authoritative patterns before writing Phase 1+ pipeline code.

### 1.2 Fixed-Point Math for JFA and Physics
- High-quality integer-only implementations of squared Euclidean distance and other geometric operations.
- Techniques to maintain sub-cell precision without floating point.
- Comparison of different fixed-point formats (Q16.16, Q24.8, etc.) for 4096²–8192² grids.
- Handling of overflow/saturation in distance calculations.

**Why it matters**: Floating-point `length()` in JFA/collapse breaks your determinism requirements. We need a proven, clean integer version.

### 1.3 Workgroup Size + Shared Memory Halo Tiling for Cellular Automata
- Optimal workgroup dimensions for neighbor-gather workloads on Blackwell-class GPUs (your DGX Spark).
- 16×16 vs 8×8 vs 32×8 tradeoffs (occupancy, register pressure, halo efficiency).
- Concrete patterns for loading 18×18 or 20×20 halos into `workgroup var` for gravity, JFA, and capillary passes.
- Expected bandwidth savings on unified LPDDR5X memory.

**Why it matters**: This is one of the highest-ROI optimizations noted in your original "Work Remaining" section. Critical for 4096²+ performance.

### 1.4 Deterministic Spatial Hashing & Tie-Breaking
- Analysis of your `hash_spatial_sync` function for warp-level consistency.
- Alternative global-seed + frame-based deterministic PRNG patterns used in other GPU CA engines.
- Techniques to ensure that threads evaluating the same logical decision always see the same random bit even across different warps.

**Why it matters**: Your symmetric gravity design depends on perfect agreement between Air and Water threads. Any non-determinism here undermines the entire mass-conservation guarantee.

### 1.5 JFA Convergence, First-Frame Behavior, and Large-Grid Stability
- Formal or empirical analysis of how many JFA passes are truly needed vs theoretical log₂(N).
- Safe strategies for handling the first 1–3 frames when distance fields have not fully propagated.
- Techniques used in production voxel/structural simulation engines for "pre-stabilizing" important geometry.

**Why it matters**: On 4096² grids, first-frame collapse behavior could look chaotic if not handled gracefully.

---

## Priority 2: Hardware & Architecture Specific Research (DGX Spark / Blackwell)

### 2.1 NVIDIA DGX Spark / Blackwell Compute Characteristics
- Specific SM version and key architectural features relevant to dense neighbor-gather compute.
- Sustained power/thermal limits for long-running high-occupancy kernels.
- Best practices for maximizing NVLink-C2C bandwidth in cellular automata workloads.
- Unified memory coherence model details and any programmer-visible implications.

**Why it matters**: We are targeting a very specific high-end machine. Generic "GPU programming" advice may miss important details.

### 2.2 Memory Layout & Coalescing for Large CA Grids
- AoS vs SoA considerations when we eventually move beyond packed u32.
- Cache line behavior for 2D strided access patterns on LPDDR5X.
- When to use texture sampling vs raw storage buffers for read-only neighbor data.

---

## Priority 3: Simulation Quality, Inspiration & Polish

### 3.1 Classic & Modern Falling Sand References
- Detailed analysis of what made early Java applets feel good (rule ordering, update frequency, visual feedback).
- Study of Noita, Powder Toy, Sandspiel, and other high-quality modern engines — what rules and visual tricks they use.
- Common failure modes in GPU falling sand ports (artifacts, instability, loss of "soul").

**Goal**: Preserve the delightful, intuitive feel of classic falling sand even as we add massive scale and structural complexity.

### 3.2 Capillary Action & Moisture Transport in Granular Media
- Simple but physically plausible models for water absorption into dirt/sand.
- Evaporation rate tuning that looks natural.
- Interaction between moisture and structural strength (already partially specified — needs validation).

### 3.3 Scent / Pheromone Diffusion Models
- Lightweight diffusion algorithms suitable for running every frame alongside physics.
- Directed transport (scent carried by flowing water or falling sand).
- Gradient-following behaviors used in ant colony simulations (for future TWIOFA layer).

---

## Priority 4: Observability, Telemetry & Integration

### 4.1 Lightweight Event Emission from Compute Shaders
- Efficient patterns for GPU → CPU event streaming on unified memory (append buffers, reduction passes, ring buffers, atomic counters with host readback).
- Designing a compact `WorldEvent` enum/struct that is useful for higher-level agents without being too verbose.
- Integration patterns with your existing `causality_map` / provenance thinking.

**Why it matters**: Phase 4 telemetry is a major deliverable. Getting the data model right early will make the AI/Host bridge much smoother.

### 4.2 Provenance & Deterministic State Tracking
- Techniques for cryptographic or Merkle-tree style receipting of large simulation states.
- How to expose "this cell changed because of X neighbor at frame Y" style causality without exploding memory use.
- Alignment with your ArcSystems / BYTES_ARE_LAW goals.

### 4.3 Host-Side Consumption Patterns
- How CedeSystem, AgentCede, or Entelechy would ideally consume live simulation state and events.
- Latency vs throughput tradeoffs for the host bridge.

---

## Priority 5: Testing, Validation & Correctness

### 5.1 Mass Conservation Verification
- Automated tests that count total water/sand/dirt mass every N frames and assert conservation within expected bounds.
- Techniques to detect and localize leaks (which rule or edge case is responsible).

### 5.2 Determinism & Reproducibility Testing Harness
- Framework to run the same seed + sequence of brush actions on CPU reference implementation vs GPU and compare final grids bit-for-bit.
- Tools to visualize or diff two simulation states.

### 5.3 Visual & Perceptual Quality Metrics
- Beyond FPS: what makes a falling sand simulation *feel* good?
- Quantitative proxies (pile stability, flow smoothness, collapse drama) that we can track over development.

---

## Priority 6: Future Layers & Long-Term Vision

### 6.1 Heat, Thermodynamics, and Chemistry Extensions
- Lightweight heat diffusion + phase change models that still feel like local CA rules.
- Simple chemistry (dissolution, crystallization, burning) suitable for this substrate.

### 6.2 Multi-Scale / Hierarchical Simulation
- Techniques for running coarse global simulation + fine local regions without visible seams.
- When and how to switch fidelity based on player/camera focus (relevant for game or SUBTERRA COMMAND use cases).

### 6.3 Export & Creative Tooling Pipelines
- Converting stable structures or heightmaps from AntVerse into assets usable by Glazier, Seed Forge, Unreal Engine 5, or Blender.
- Signed Distance Field generation from JFA data for high-quality rendering or collision.

---

## How We Will Use This Agenda

1. Before starting any new phase, we will review relevant items from this list.
2. I will use tools (web_search, browse_page, etc.) to gather high-quality references, code examples, papers, or hardware documentation.
3. Findings will be summarized and incorporated into the appropriate document in `02_ARCHITECTURE/`, `03_MATH_FOUNDATIONS/`, `04_DESIGN_DECISIONS/`, or new code templates.
4. Once researched, items will be marked as "Resolved" with links or summaries pointing to the new knowledge.

This systematic approach ensures we build on solid foundations rather than discovering critical issues mid-implementation.

---

**Current Status (2026-05-25, updated during gap-0.1 resolution)**: 
- **All Phase 0 Tier 1 tasks (0.1–0.4) now fully resolved** in strict order.
  - 0.1 Event Ledger / WorldEvent: `RESOLVED_0.1_Event_Ledger_WorldEvent_Data_Model.md` (created 2026-05-25 in this session; previously missing despite cross-references throughout the KB)
  - 0.2 Mass Conservation Verification: `RESOLVED_0.2_Mass_Conservation_Verification_Strategy.md`
  - 0.3 Determinism Validation Harness: `RESOLVED_0.3_Determinism_Validation_Harness_Design.md`
  - 0.4 First-Frame JFA & Large-Grid Init: `RESOLVED_0.4_First_Frame_JFA_Behavior_Large_Grid_Initialization.md`
- Primary 0.1 deliverable per RESEARCH_ROADMAP also created: `07_TELEMETRY_AND_OBSERVABILITY/EVENT_LEDGER_DESIGN.md`
- Also resolved: 4.1 (via 0.1), 5.1, 5.2, plus prior 1.1/1.2/1.3.
- 1.5 (JFA Convergence/First-Frame) resolved as part of 0.4.
- All Phase 0 Tier 1 tasks now complete with concrete decisions, artifacts, and agenda links. The false prior claim that 0.1 was already resolved has been corrected.

**Next Action (updated 2026-05-25)**: Phase 0.0 Foundation Setup is complete. The first runnable Phase 0 demo (2048² ping-pong + render with clean module structure and all 0.1/0.2 data model contracts in place) now exists and compiles cleanly. Proceed strictly to Phase 0.1 (Brush & Basic Interaction) per the Implementation Plan. Update this agenda upon completion of Phase 0.1 and every subsequent phase. No JFA or Phase 2 structural work until Phase 0 success criteria are fully met.
