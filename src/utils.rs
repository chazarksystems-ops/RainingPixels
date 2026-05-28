// src/utils.rs
// Antverse Phase 0 — Shared utilities (packing helpers, deterministic hash, coordinate math)
// Per 02_ARCHITECTURE/PHASE0_DEMO_IMPLEMENTATION_PLAN.md and 03_MATH_FOUNDATIONS/CELLULAR_AUTOMATA_RULES.md
//
// Contains:
// - hash_spatial_sync (global-seed + frame deterministic tie-breaking, used by symmetric gravity)
// - Coordinate packing / index helpers
// - Any fixed-point or saturation arithmetic that must be identical between WGSL and CPU ref (0.3)
//
// Phase 0.0 Step 1: File + module declaration only.
// The hash function and packing routines are implemented alongside the first physics rules (Phase 0.2).