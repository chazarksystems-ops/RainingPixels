# Current Target: TWIOFA_SUBSTRATE_PROOF_v0

## One-Sentence Mission

Prove that ant-scale material interaction with visible physical consequence can form the core of a compelling game before investing in any scaling infrastructure.

## Success Definition

The prototype succeeds if a player can immediately feel, without explanation:

- Soil has state and consequence
- Digging changes the world in meaningful ways
- Water and moisture affect stability and routes
- Collapse is possible and legible
- Ants are affected by terrain changes
- The world feels materially alive rather than like a tile map

## Hard Scope Lock

### Must Include
- One fixed active chunk (recommended 512×512 or 1024×512)
- Packed `u32` cell state on GPU
- GPU compute simulation (CA-style)
- Materials: Air, Dry Dirt, Wet Dirt, Loose Soil, Water, Stone/Root Blocker
- Core behaviors: Gravity, settling, digging, collapse, wetting, moisture diffusion, water spread
- Minimal visible ant group capable of receiving orders, digging, and reacting to blocked routes
- Rendering that prioritizes material continuity and consequence over clean grid aesthetics
- Click-to-dig / route intent interaction loop

### Must Exclude (for this phase)
- Sparse chunking / infinite yard
- Procedural generation
- Full ECS or complex ant AI
- LBM or advanced fluid simulation
- Save/load, editor, networking
- Temperature, chemistry, gases, complex ecology
- Generic engine abstraction or plugin systems

## Naming Rule

Do **not** call this an engine, framework, or runtime in external communication.  
Call it what it is: **TWIOFA_SUBSTRATE_PROOF_v0**

## After Success Criteria Are Met

Only then consider:
- Sparse chunk paging
- Larger yards
- Richer ant logic
- Field notes / minimal UI
- Procedural elements
- Multi-channel scent
- Save/load

The substrate must prove itself worthy of scaling first.