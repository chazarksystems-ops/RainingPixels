# Slice Backlog

## Purpose

This backlog defines future work as bounded slices so agents cannot drift into generic engine work.

## Current core slice chain

### S01 — Substrate Proof v0

Goal:
Define the one-screen playable material-interaction proof.

Player-visible result:
A player can see ants interact with a living material yard where digging, moisture, and collapse matter.

### S02 — Cell State and GPU Buffers

Goal:
Implement or document the packed `u32` cell state and CPU/GPU equivalence.

Player-visible result:
None required yet; this is correctness infrastructure.

Acceptance:
Rust/WGSL pack/unpack constants and test vectors match.

### S03 — Single Chunk Material Runtime

Goal:
One fixed chunk, double-buffered compute passes, material buffer render.

Player-visible result:
A visible terrain/material field appears and updates.

### S04 — Material Rendering Readability

Goal:
Make the material field read as continuous soil/water/collapse, not tile squares.

Player-visible result:
A player sees material state without relying on debug grid.

### S05 — Visible Ant Execution

Goal:
Player click/order produces visible ant/material response.

Player-visible result:
Ants move toward a target and begin changing material.

### S06 — Water / Moisture / Wet Soil

Goal:
Water changes dirt state; moisture affects stability/readability.

Player-visible result:
Water seeps, dirt wets, wet/dry state is visible.

### S07 — Collapse / Tunnel Liability

Goal:
Digging can create unstable material and route blockage.

Player-visible result:
A tunnel can become unsafe or blocked due to material consequence.

### S08 — Minimal Ant Behavior

Goal:
Ants move, dig, deposit/read simple scent, and react to blocked routes.

Player-visible result:
Ants are not just decoration; terrain changes affect them.

### S09 — Route Consequence Loop

Goal:
Combine digging, moisture, collapse, and ant movement into a readable loop.

Player-visible result:
A player sees that route-making is the game.

### S10 — Substrate Playtest Gate

Goal:
Chaz tests whether the substrate feels worth building around.

Decision:
PASS / REVISE / FAIL.

## Future intent-only slices

These should exist but remain deferred until substrate proof passes:

```text
S11_surface_ground_underground_views
S12_open_yard_intent
S13_food_poi_intent
S14_deep_hive_campaign_intent
S15_ecology_weather_rivals_intent
S16_ant_farm_mode_intent
S17_antverse_dgx_lab_intent
```
