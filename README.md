# AntVerse — DGX Spark Native Falling Sand Engine

**Project Status**: Phase 0 Foundation (Classic Falling Sand Principles + DGX Scaling Strategy)

**Core Thesis**  
Take the timeless local-rule cellular automata spirit of classic falling sand simulations (old Java applets, early Noita-inspired engines, etc.) and develop it *without compromise* on the NVIDIA DGX Spark (128 GB unified coherent LPDDR5X + NVLink-C2C).  

The goal is to reach simulation scales, state richness, determinism guarantees, and multi-layer interaction depth that are structurally impossible on traditional discrete-GPU PC architectures.

This folder is the single source of truth. It will be filled phase-by-phase until it contains every piece of knowledge, math, design decision, code template, reference, and integration plan required to build and evolve the engine.

---

## Directory Structure

```
antverse/
├── 00_PROJECT_VISION/           # Why we are doing this, goals, DGX advantages
├── 01_CLASSIC_FALLING_SAND_REFERENCE/  # Roots and principles we preserve
├── 02_ARCHITECTURE/             # High-level design, pipeline, memory, scaling
├── 03_MATH_FOUNDATIONS/         # All required mathematics
├── 04_DESIGN_DECISIONS/         # Concrete choices (materials, packing, workgroups, etc.)
├── 05_CODE_TEMPLATES/           # WGSL + Rust skeletons (phase by phase)
├── 06_DEPENDENCIES_AND_REPOS/   # Crates, public repos, papers, inspiration
├── 07_TELEMETRY_AND_OBSERVABILITY/     # Event ledger, causality, host bridge
├── 08_INTEGRATION_POINTS/       # How this connects to your broader systems
├── 09_TESTING_AND_VALIDATION/   # Correctness, determinism, performance tests
├── 10_FUTURE_LAYERS/            # Heat, chemistry, biology, multi-scale
├── 11_REFERENCES/               # Curated papers, books, links
└── README.md                    # This file
```

---

## Development Philosophy

1. **Classic Soul First** — Local neighborhood rules only. Emergent complexity from simplicity.
2. **DGX Spark Native** — Design for 128 GB unified memory and high internal bandwidth from day one. Never downscale for "PC compatibility".
3. **Determinism & Provenance** — Bit-perfect reproducibility and auditable state changes (aligns with BYTES_ARE_LAW and ArcSystems).
4. **Incremental Layering** — Phase 0 = classic. Each subsequent phase adds one powerful layer while preserving the local-rule aesthetic.
5. **Living Knowledge Base** — This folder grows with the project. Every major decision, derivation, and reference lives here.

---

## Current Phase

**Phase 0** — Classic Falling Sand on DGX Spark (large grid, simple local rules, interactive brush, clean foundation ready for symmetric gravity + JFA).

Next phases will be added as we complete previous ones.

---

## How to Use This Folder

- Read `00_PROJECT_VISION/` first for context.
- `02_ARCHITECTURE/PIPELINE_PHASES.md` shows the exact sequence of layers we will implement.
- `05_CODE_TEMPLATES/` contains the actual WGSL + Rust files we will evolve.
- When a new phase begins, new documents and code templates will be added to the relevant sections.

This is not a throwaway repo. It is the permanent engineering memory for AntVerse.

---

**Last Updated**: 2026-05-25  
**Maintainer**: Chaz + Grok (collaborative)  
**Hardware Target**: NVIDIA DGX Spark (your personal 128 GB unified memory system)
