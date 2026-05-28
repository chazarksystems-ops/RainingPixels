// src/brush.rs
// Antverse Phase 0 — Direct manipulation brush (circle stamp + material selection)
// Per 02_ARCHITECTURE/PHASE0_DEMO_IMPLEMENTATION_PLAN.md (Phase 0.1)
//
// Responsibilities:
// - Keyboard material selection (Sand, Water, Dirt, Stone, Air/Eraser)
// - Responsive circle stamp that writes into the "next" buffer or staging
// - Emit BrushAction events with exact net mass delta (0.1 + 0.2 integration)
//
// Phase 0.0 Step 1: File + module declaration only.
// Logic and input handling are implemented in Phase 0.1 after the basic grid exists.