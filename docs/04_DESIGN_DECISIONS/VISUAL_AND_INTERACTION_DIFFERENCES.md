# Visual and Interaction Differences — AntVerse Demo vs Traditional Falling Sand (Noita-style)

**Status**: Phase 0 Tier 1 Complete — Demo Focus  
**Date**: 2026-05-25  
**Scope**: This document breaks down specific visual and interaction differences expected in the **current demo** (Phase 0 foundation + early Phase 1 symmetric gravity) compared to high-quality traditional falling sand engines like Noita.

---

## 1. Scale & World Coherence

### Visual Differences
- **Noita-style**: Worlds typically feel contained and intimate. Even at higher resolutions, the player usually experiences a relatively small playable area at once. Large bodies of water or sand tend to be localized.
- **AntVerse Demo**: 2048²–4096²+ grids from the start. The player can create and observe **large-scale coherent phenomena** — wide rivers, expansive sand dunes, distant water pooling that visibly affects terrain over time. The world feels more like a landscape than a contained box.

### Interaction Differences
- Pouring water in one area of AntVerse can create visible, consequential effects hundreds of cells away as pressure and flow propagate across a large connected body.
- Sand avalanches and large-scale settling feel more physically substantial because of the grid scale.

**Demo Impact**: This is one of the most immediately noticeable differences. The simulation feels "bigger" and more physically consequential even before adding JFA structural systems.

---

## 2. Fluid & Granular Physicality (Symmetric Gravity)

### Visual Differences
- **Noita-style**: Excellent fluid simulation. Water flows, spreads, and interacts with solids in very pleasing ways. Sand has good piling and sliding behavior.
- **AntVerse Demo**: The symmetric reverse-pull gravity design + deterministic spatial hashing produces:
  - More consistent and physically plausible **diagonal sliding** along slopes.
  - Stronger **hydrostatic leveling** — water surfaces settle flatter and more reliably across large areas.
  - Better preservation of momentum and pressure in confined spaces.

### Interaction Differences
- Water "feels heavier" and more consequential when moving large volumes.
- Sand piles form more stable and predictable slopes.
- There is less visual "jitter" or inconsistent behavior in tight spaces due to the mirrored air/water logic and global seed determinism.

**Demo Impact**: Fluid and sand behavior should feel more "serious" and physically believable than most traditional implementations, while still remaining fun and immediate.

---

## 3. Mass Conservation & Trust

### Visual / Perceptual Differences
- **Noita-style**: Mass is generally well conserved visually. Small leaks or inconsistencies are rarely noticeable to players.
- **AntVerse Demo**: Mass is **continuously verified** (0.2) with FrameSummary events carrying before/after deltas. While this is mostly invisible to a casual player, it creates a subconscious feeling that "the world is fair and consistent."

### Interaction Differences
- Players can trust that pouring water or creating large structures will not have mysterious disappearance/creation of matter.
- This becomes especially valuable when building complex setups or when the simulation runs for long periods.

**Demo Impact**: Stronger underlying trust, even if the player cannot directly see the verification system.

---

## 4. Determinism & Reproducibility

### Visual / Perceptual Differences
- **Noita-style**: Good visual determinism within a single run. Replaying the exact same sequence on different hardware or after code changes can produce small visual differences.
- **AntVerse Demo**: Bit-perfect reproducibility between CPU reference and GPU (0.3). The same seed + brush sequence produces **identical** grids, event streams, and mass histories.

### Interaction Differences
- Players and future agent systems can rely on experiments being exactly repeatable.
- This enables reliable creative workflows (e.g., "this brush sequence always produces this terrain feature").

**Demo Impact**: Mostly invisible to casual play, but foundational for any serious creative or research use.

---

## 5. First-Frame / World Initialization Behavior (0.4 Policy)

### Visual Differences
- **Noita-style**: Worlds are usually designed or pre-settled so they look stable from frame 0.
- **AntVerse Demo**: Large grids will exhibit deliberate **first-frame collapse** of over-spanned or unanchored dirt (per the adopted policy). This creates a visible "world settling" moment on load or major brush changes.

### Interaction Differences
- Players may see loose dirt structures collapse naturally on the first few frames — this is intended as a feature, not a bug.
- Important/hand-authored structures can be protected via the `pre_stabilize` mechanism (to be implemented in Phase 2).

**Demo Impact**: This is a new experiential element. The world can "breathe" and settle on initialization in a physically plausible way.

---

## 6. Observability & Event Stream (0.1)

### Visual / Perceptual Differences
- **Noita-style**: Observation is almost entirely visual. There is no structured way to know "this collapse happened because of X water + Y moisture at frame Z."
- **AntVerse Demo**: A structured `WorldEvent` stream (including `FrameSummary` with mass deltas) is emitted from the first frame. Major events (absorption, notable flows, structural changes) are recorded deterministically.

### Interaction Differences
- Future UI can surface these events (text log, visual highlights, sonification, or feeding into Cede/Entelechy).
- Even without UI, the existence of this stream changes the relationship between the simulation and external systems.

**Demo Impact**: The simulation becomes **observable as data**, not just visually. This is a foundational difference even if the UI for it is minimal in early demos.

---

## 7. Brush & Immediate Interaction

### Visual Differences
- Both should feel responsive. AntVerse benefits from unified memory, so brush stamping and simulation response can remain very fast even at 4096².

### Interaction Differences
- AntVerse brush can eventually support richer payloads (moisture, scent, pre_stabilize flags) without performance penalty.
- The brush becomes not just a painting tool but a way to inject structured state into a rich simulation.

**Demo Impact (Phase 0–1)**: Brush feel should be at least as good as high-quality traditional implementations, with headroom for richer interaction later.

---

## Summary Table — Demo Stage Differences

| Area                        | Noita-style Feel                          | AntVerse Demo Feel (Phase 0–1)                     | Difference Strength | When It Becomes Major |
|-----------------------------|-------------------------------------------|----------------------------------------------------|---------------------|-----------------------|
| World Scale                 | Intimate, contained                       | Large, landscape-like                              | High                | Immediate            |
| Fluid Physicality           | Excellent                                 | Heavier, more consistent, better hydrostatics      | Moderate-High       | Immediate            |
| Structural Drama            | Limited                                   | Still mostly classic (JFA in Phase 2)              | Low                 | Phase 2              |
| Mass Conservation           | Good (visual)                             | Strong + continuously verified                     | Moderate            | Immediate (subtle)   |
| Determinism                 | Good visual                               | Bit-perfect + harness                              | High (for trust)    | Immediate (invisible)|
| First-Frame Behavior        | Usually stable                            | Deliberate "world settling" collapse               | New experiential    | Immediate            |
| Observability               | Visual only                               | Structured event stream from day one               | Foundational        | Immediate            |
| Creative/Agent Integration  | Low                                       | Designed in from the start                         | High                | Growing over phases  |

---

## Key Takeaways for the Current Demo

1. **The biggest immediate differences** will be **scale + fluid physicality + first-frame settling behavior**.
2. **Structural architectural drama** is intentionally deferred to Phase 2 (JFA). The demo should not be judged against Noita on this axis yet.
3. **Observability and determinism** are foundational differences that may not be visually obvious but enable entirely new use cases (agent consumption, reproducible creative workflows, long-term trust).
4. The demo should prioritize **tactile joy and visual clarity** while establishing the technical foundation that makes later phases dramatically more powerful than traditional engines.

---

**This document should be updated after Phase 1 implementation with actual observed differences from running builds.**