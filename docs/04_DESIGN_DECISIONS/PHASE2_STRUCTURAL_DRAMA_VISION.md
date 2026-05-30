# Phase 2 Structural Drama Vision — JFA + Moisture-Dependent Collapse

**Status**: Draft — Phase 0 Tier 1 Complete  
**Date**: 2026-05-25  
**Purpose**: Define the intended visual, emotional, and interactive experience of structural emergence and failure in Phase 2. This is expected to be one of the strongest differentiators from traditional falling sand engines.

---

## Core Intent

Phase 2 introduces **real architectural behavior** to the simulation:
- Dirt can form ceilings, bridges, overhangs, towers, and other spanning structures.
- These structures have physical limits based on material strength (dry vs wet dirt) and distance to stable anchors (stone or connected dirt clusters).
- When limits are exceeded, structures visibly strain and then dramatically collapse, converting dirt into mobile sand.

This moves AntVerse from "particles and fluids" into "**physics with memory and consequence**."

---

## Visual Goals

### Desired Aesthetic
- Collapse events should feel **weighty, consequential, and visually satisfying** — not like pixels simply changing state.
- There should be a visible period of **strain** before full collapse (subtle sagging, small particle releases, or progressive weakening) so the failure feels earned rather than sudden.
- Wet dirt should clearly behave differently from dry dirt — holding larger spans, failing more gracefully, and visually darkening as it absorbs moisture.
- Large collapses should create secondary effects (sand avalanches, dust-like particle bursts, changes in water flow paths).

### Visual Hierarchy
1. **Stable structures** — Read as solid, grounded, intentional.
2. **Strained structures** — Show subtle visual stress (micro-movements, small detachments, moisture concentration at stress points).
3. **Active collapse** — Dramatic, fast, and consequential. Large volumes of dirt convert to sand and fall.
4. **Aftermath** — New sand piles, changed terrain topology, altered water flow, and new structural configurations.

---

## Interaction Goals

### Player Agency
- Players should be able to **deliberately create** situations that lead to interesting structural failure (undermining supports, adding weight via water/sand on top, waiting for moisture to weaken dirt).
- Players should also be able to **protect** important structures using the `pre_stabilize` mechanism or by placing stone anchors.
- The system should reward thoughtful interaction ("I can build a bridge here if I keep it dry and supported") while still allowing chaotic, emergent failure.

### Emotional Tone
- Collapse should feel **dramatic but fair**. It should not feel random or punitive.
- There should be a sense of **physical consequence** — building something large and watching it hold, strain, and eventually fail has emotional weight.
- The first-frame collapse policy (0.4) extends this feeling to world initialization — large, loose structures naturally settle on load.

---

## Technical Enablers (from Research)

- **Jump Flood Algorithm (JFA)** every frame to compute nearest anchor distance for every dirt cell.
- **Fixed-point integer squared distance** (from 1.2) for deterministic, drift-free comparison.
- **Moisture-dependent max span**:
  - Dry dirt: small max span (~6 cells)
  - Wet dirt: significantly larger max span (~35+ cells) with moisture bonus
- **Collapse decision** runs after JFA convergence and before gravity, so newly created sand immediately participates in physics.
- **pre_stabilize flag** + temporary elevated threshold for important/hand-authored structures.

---

## Differentiation from Traditional Falling Sand

| Aspect                    | Traditional / Noita-style          | AntVerse Phase 2                                      |
|---------------------------|------------------------------------|-------------------------------------------------------|
| Structural Spanning       | Very limited or absent             | Core feature — real ceilings, bridges, towers         |
| Collapse Behavior         | Simple local rules or none         | Distance-to-anchor + moisture dependent               |
| Visual Drama              | Usually particle-level             | Architectural scale failure with visible strain phase |
| Player Intentionality     | Low                                | High — players can design for stability or failure    |
| Emergent Narrative        | Mostly fluid/fire based            | Architecture + failure + consequence                  |

This is the point where AntVerse stops being "just another falling sand" and becomes something with genuine architectural and dramatic depth.

---

## Risks to Avoid

- **Too sudden collapse** — without a visible strain phase, failure can feel arbitrary.
- **Too frequent collapse** — if everything collapses constantly, the world never feels stable enough to build upon.
- **Unclear causality** — players should be able to understand why something collapsed (distance to anchor + moisture state). The event system (0.1) and future stress visualization should support this.
- **Performance cost** — JFA every frame at 4096² must remain fast on DGX Spark. The 16×16 + halo tiling decision (1.3) and unified memory are critical enablers here.

---

## Open Questions for Phase 2 Implementation

- What is the minimal but effective "strain visualization" (subtle particle release, micro-movement, color shift, stress overlay)?
- Should collapse propagate (one cell collapsing increases load on neighbors) or be purely distance-based?
- How should very large collapses be throttled or broken into multiple frames for visual clarity and performance?
- What UI/UX support (hover info, stress heatmap toggle, "danger zone" preview) is valuable without breaking the classic aesthetic?

---

**Phase 2 structural drama is the feature most likely to make AntVerse feel qualitatively different from existing falling sand engines. Getting the visual and emotional weight right is as important as the technical correctness of the JFA implementation.**