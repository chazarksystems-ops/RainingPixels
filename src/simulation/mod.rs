// src/simulation/mod.rs
// Antverse Phase 0 — Simulation core module
// Per 02_ARCHITECTURE/PHASE0_DEMO_IMPLEMENTATION_PLAN.md
//
// Declares the internal modules for state, pipeline, events, and verification.
// This structure supports the single-CommandEncoder linear physics chain
// and the foundational event ledger (0.1) + mass verification (0.2).

pub mod state;
pub mod pipeline;
pub mod events;
pub mod verification;

// Core data model re-exports (Step 2 complete)
pub use state::{
    Cell, SimParams, SimulationState, FLAG_PRE_STABILIZE, MAT_AIR, MAT_DIRT, MAT_SAND, MAT_STONE,
    MAT_WATER,
};
pub use events::{EventLedgerGpu, WorldEvent, EVENT_CAPACITY}; // From RESOLVED_0.1 / EVENT_LEDGER_DESIGN.md

