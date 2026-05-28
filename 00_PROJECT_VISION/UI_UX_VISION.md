# AntVerse UI/UX Vision

**Status**: Draft — Phase 0 Tier 1 Complete  
**Date**: 2026-05-25  
**Purpose**: Define the intended experiential feel of AntVerse across phases, with emphasis on how the UI/UX should amplify both the classic falling sand soul and the new capabilities enabled by DGX Spark unified memory.

---

## Core Experiential Goal

AntVerse should feel like **classic falling sand elevated** — immediate, tactile, surprising, and joyful — while operating at a scale and richness that creates qualitatively new experiences.

We are not building a scientific visualization tool or a pure game. We are building a **living physical substrate** that can serve as:
- A delightful interactive toy/sandbox
- A source of emergent drama and beauty
- A rich data source for higher-level creative and agent systems (Cede, Entelechy, TWIOFA, Glazier exports, etc.)

The UI/UX must support both **playful immediate interaction** and **deep observability** without one compromising the other.

---

## Phase-by-Phase Feel Targets

### Phase 0–1 (Current Demo Focus): Classic Elevated

**Visual Feel**:
- Large, coherent worlds (2048²–4096²+) that feel like real terrain rather than a small toy.
- Water that has visible weight and momentum — large pools form, flow realistically, and interact with sand in believable ways.
- Sand that piles, slides, and settles with consistent, satisfying behavior.
- Clean, readable material distinction with subtle moisture darkening on dirt.
- Immediate, responsive brush that feels like a natural extension of the hand.

**Interaction Feel**:
- Brush should be fast, precise, and satisfying (similar to good MS Paint / Glazier immediacy).
- The world should react instantly and believably to brush input.
- Large-scale flows should feel consequential — pouring water in one area can visibly affect distant areas over time.
- First-frame "world settling" (from 0.4 policy) should feel like a deliberate, beautiful ritual rather than chaos.

**Key Differentiator from Noita-style engines at this stage**:
- Scale and fluid physicality become the primary "wow".
- Determinism and mass conservation are strong but mostly invisible to the player (they just feel "right").

### Phase 2: Structural Drama

**Visual Feel**:
- Dirt can form real architecture — ceilings, bridges, overhangs, towers.
- These structures visibly strain, sag, and dramatically collapse when they exceed their moisture-dependent span.
- Collapse events should feel impactful and consequential (avalanches of sand, sudden changes in terrain).

**Interaction Feel**:
- Players can deliberately (or accidentally) create situations that lead to structural failure.
- The simulation gains a sense of **history and memory** — the world changes in non-reversible, physically plausible ways.

**Key Differentiator**:
- This is where AntVerse begins to clearly exceed traditional falling sand engines in emergent architectural behavior.

### Phase 3+: Layered Richness & Life

**Visual Feel**:
- Capillary wicking darkens dirt and visibly transports moisture.
- Slow evaporation adds a gentle time-based evolution.
- Scent diffusion (and later ants) introduces biological signaling and trails.

**Interaction Feel**:
- The world starts to feel like a living ecosystem rather than pure physics.
- Observability (event stream) becomes part of the creative experience — major events can be "listened to" or reacted to.

**Key Differentiator**:
- AntVerse transitions from "physics toy" to "expressive substrate" that can feed creative tools and agent systems.

---

## UI/UX Design Principles

1. **Immediacy First**
   - Brush and simulation response must feel instant and delightful, even at large grid sizes.
   - Never sacrifice tactile joy for simulation fidelity.

2. **Readable Emergence**
   - Complex behaviors (large flows, structural collapse, capillary effects) should be visually clear without needing tooltips or external explanation.

3. **Layered Observability**
   - The simulation should be watchable both casually (beautiful visuals) and deeply (event stream, mass deltas, structural stress).
   - Future UI should support both modes gracefully.

4. **Creative Extensibility**
   - The system should feel like raw material for other tools (Glazier, Seed Forge, CedeSystem, Entelechy, TWIOFA).
   - Export paths and event consumption should be designed in from the start, not bolted on later.

5. **Trust Through Determinism**
   - Strong determinism and mass conservation should create a subconscious feeling of "this world behaves fairly and consistently."
   - This builds long-term creative confidence (you can experiment knowing results are reproducible).

---

## Relationship to User's Broader Creative Work

AntVerse is intended to complement and feed:
- **Glazier** — Source of procedural forms, textures, and structural references.
- **Lucent** — Mood, light, and atmospheric studies from large-scale fluid and material behavior.
- **StainedGraph / SUBTERRA COMMAND** — Tactical/strategic overlays on top of a living world.
- **CedeSystem / Entelechy** — Rich, structured event stream for agent reasoning and meaning-making.
- **TWIOFA** — Physics + chemistry substrate for scent-based ant agents.
- **Seed Forge / Unreal** — Export of stable structures and distance fields for asset pipelines.

The UI/UX should eventually support both **playful standalone use** and **deep integration** with these systems.

---

## Open Questions (to be addressed in later phases)

- How should the event stream be visualized or sonified for creative use?
- What minimal UI overlays (stress heatmaps, scent visualization, collapse danger zones) are valuable without breaking the classic falling sand aesthetic?
- How do we balance "toy mode" (maximum playfulness) vs "serious mode" (maximum observability and control) in the same application?
- What export formats and creative tool integrations should be prioritized first?

---

**This document will evolve as we move through phases. It serves as the north star for UI/UX decisions alongside the technical research documents.**