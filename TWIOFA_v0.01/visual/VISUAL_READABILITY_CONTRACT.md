# Visual Readability Contract — TWIOFA_SUBSTRATE_PROOF_v0

## Primary Goal

The renderer must make the player feel they are interacting with a living, reactive material substrate at ant scale, not a grid of colored squares.

## Core Requirements

- The material field should read as **continuous masses** of soil, water, and loose material where appropriate, rather than emphasizing hard square boundaries.
- **Dry dirt, wet dirt, loose soil, water, stone/root blockers, recently disturbed areas, and collapse zones** must be visually distinguishable at a glance.
- **Wetness and moisture** must be readable enough to support player decision-making (e.g., “this area looks riskier to dig”).
- **Collapse** must be visually legible both before it happens (cracks, sagging, unsupported overhangs) and after (fallen material, blocked space, changed routes).
- Recently disturbed or dug areas should carry a visible “recent change” signal (dust, altered texture, temporary highlight, or edge softening).
- Ants and ant clusters must remain clearly readable against the terrain.

## Noita-Floor Direction

The minimum visual target for material presentation is **Noita-floor** material consequence and readability. This is a directional floor, not a ceiling and not a requirement to copy Noita’s exact art style.

The prototype fails its visual goal if it primarily reads as:
- A clean square tile board
- A colored debug grid
- Abstract symbols rather than material masses

## Prototype vs Final Visual Target

- Early prototypes may use simplified or scaffold visuals.
- The intended direction is continuous material readability and consequence.
- Internal cell structure is allowed, but it must not dominate the player’s perception of the yard.

## Underground Cross-Section Clarity

Because underground material consequence is central to the first proof, the renderer must support clear readability of tunnels, moisture, structural weakness, and collapse within a cross-section or layered view. Underground should not feel like a flat top-down tile map.