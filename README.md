# Raining Pixels — DGX Spark Falling Sand Demo

**Repository**: chazarksystems-ops/RainingPixels  
**Branch**: aining-pixels  
**Focus**: High-performance falling sand simulation built specifically for the NVIDIA DGX Spark (128 GB unified memory)

---

## Project Purpose

This branch contains the research foundation and implementation planning for a deterministic, large-scale falling sand engine designed to fully leverage the DGX Spark’s unified memory architecture. The goal is to achieve simulation scales, state richness, and determinism guarantees that are difficult or impossible on traditional discrete-GPU systems.

**Important Scope Note**: This work is currently focused **only** on the falling sand simulation itself. Broader agent and TWIOFA-related work is being kept separate.

---

## Current Status (as of 2026-05-30)

| Phase | Status | Notes |
|-------|--------|-------|
| Research | **Complete** | All Priority 1 research items resolved |
| Implementation | **Not Started** | Awaiting start of Phase 0.0 |

### Key Research Completed

- Event Ledger & WorldEvent data model (0.1)
- Mass Conservation Verification Strategy (0.2)
- Determinism Validation Harness Design (0.3)
- First-Frame JFA Behavior & Large-Grid Initialization Policy (0.4)
- Multi-pass compute pipeline synchronization (1.1)
- Fixed-point integer math for JFA (1.2)
- Shared memory halo tiling + workgroup sizing (1.3)

---

## How to Navigate This Branch

| Goal | Recommended Document |
|------|----------------------|
| Understand the overall vision and experiential goals | docs/00_PROJECT_VISION/UI_UX_VISION.md |
| See how this differs from traditional engines (e.g. Noita) | docs/04_DESIGN_DECISIONS/VISUAL_AND_INTERACTION_DIFFERENCES.md |
| Review the concrete implementation plan | PHASE0_DEMO_IMPLEMENTATION_PLAN.md (root) |
| Understand the technical decisions already made | docs/12_RESEARCH_NEEDED/RESEARCH_AGENDA.md |
| See the long-term creative/agent integration vision | docs/08_INTEGRATION_POINTS/CREATIVE_TOOLING_INTEGRATION_VISION.md |
| Review Phase 2 direction (structural drama) | docs/04_DESIGN_DECISIONS/PHASE2_STRUCTURAL_DRAMA_VISION.md |

**Start here**: PHASE0_DEMO_IMPLEMENTATION_PLAN.md

---

## Repository Structure
raining-pixels/
├── PHASE0_DEMO_IMPLEMENTATION_PLAN.md   # Main implementation roadmap
├── README.md                            # This file
├── docs/                                # Full research & design knowledge base
│   ├── 00_PROJECT_VISION/
│   ├── 02_ARCHITECTURE/
│   ├── 03_MATH_FOUNDATIONS/
│   ├── 04_DESIGN_DECISIONS/
│   ├── 07_TELEMETRY_AND_OBSERVABILITY/
│   ├── 08_INTEGRATION_POINTS/
│   └── 12_RESEARCH_NEEDED/
└── implementation/                      # (Future) Actual code will live here

---

## What Has Been Done

- Complete research phase for the core technical foundations (determinism, pipeline design, event system, mass verification, etc.).
- Clear design intent documented for both the current demo and future phases.
- Prioritized, phased implementation plan created.

## What Comes Next

Implementation will follow PHASE0_DEMO_IMPLEMENTATION_PLAN.md, starting with:

- Phase 0.0 — Foundation Setup
- Phase 0.1 — Brush & Basic Interaction
- Phase 0.2 — Symmetric Reverse-Pull Gravity
- And so on...

---

## Notes for Reviewers

- This branch is intentionally kept focused on the **falling sand simulation** for DGX Spark.
- The knowledge base is designed to be living — it will be updated as implementation progresses.
- All major technical decisions from the research phase are documented with rationale in the docs/12_RESEARCH_NEEDED/ folder.

---

**Last Updated**: 2026-05-30
