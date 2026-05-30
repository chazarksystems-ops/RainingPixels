# Integration Point — TWIOFA (The World Is on FIRE Ants) Scent Lattice

## Relationship to AntVerse

AntVerse Phase 3 introduces a `scent` field (bits 24–31) and simple diffusion in the capillary pass. This is deliberately designed as the **physics + chemistry substrate** for your TWIOFA research.

## How the Layers Connect

**AntVerse (this project)** provides:
- Physical terrain (dirt, sand, stone, water, moisture)
- Capillary wicking and evaporation
- Basic scent diffusion (isotropic or slightly biased by moisture/flow)
- Event stream (collapses, water flow, dirt becoming wet) that can influence scent

**TWIOFA layer** (future, built on top) will add:
- Simple ant agents that read the scent gradient and move accordingly.
- Directed scent deposition by ants (pheromone trails).
- Possibly ant–environment feedback (ants digging dirt, carrying sand, altering moisture locally).
- Higher-level colony behaviors emerging from many local ant + scent + physics interactions.

## Why This Layering Is Powerful

By keeping the core physics in AntVerse clean and deterministic, the ant layer can be developed, tested, and even run at different timescales without touching the expensive JFA/gravity/capillary passes every frame.

The unified memory of DGX Spark makes it trivial for the ant simulation (which may be sparser) to read the dense physics grid with almost zero cost.

## Scent Field Design Notes (for Phase 3+)

- 8-bit scent is sufficient for visual trails and simple gradient following.
- Diffusion can be a cheap separate pass or folded into capillary.
- Later we may add a small velocity/bias term so scent is carried by flowing water or falling sand.
- Ants will read a 3×3 or 5×5 neighborhood of the scent field and choose movement probabilistically or deterministically (depending on desired ant "intelligence" level).

This separation keeps AntVerse true to the "classic falling sand" local-rule spirit while giving TWIOFA a rich, living world to inhabit.

---

**AntVerse is the earth. TWIOFA is the ants that live in and modify that earth.**
