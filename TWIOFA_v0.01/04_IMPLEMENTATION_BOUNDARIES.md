# Implementation Boundaries — TWIOFA_SUBSTRATE_PROOF_v0

## Runtime Boundary

**In Scope**:
- One fixed-size cell buffer (GPU storage buffer)
- Packed `u32` cell state
- GPU compute passes for material rules
- Double-buffered simulation state
- Minimal perception + intent + result buffers for ant interaction
- Basic rendering of material field + ants

**Out of Scope**:
- Sparse chunk management
- Multi-chunk residency or streaming
- Dynamic memory allocation for world size
- Generic simulation scheduler or plugin system

## Ant Boundary

**In Scope**:
- Small group of visible ants/particles
- Ability to receive a dig or route target
- Basic movement through open cells
- Ability to remove diggable material
- Simple scent deposit and following
- Reaction to blocked routes or collapse

**Out of Scope**:
- Full Entity Component System
- Individual ant state machines beyond minimum
- Colony-level job scheduling
- Complex pathfinding or long-term planning

## Rendering Boundary

**In Scope**:
- Material-continuous presentation (softened grid, color/texture variation by state)
- Visual distinction between dry/wet/loose materials
- Visible recent disturbance and collapse
- Readable ant clusters over terrain

**Out of Scope**:
- Production PBR lighting
- Advanced post-processing
- Multiple render passes for beauty
- Asset pipeline or texture streaming

## Simulation Rule Boundary

**In Scope**:
- Gravity + settling
- Basic water spread and pooling
- Moisture diffusion into dirt (Gather model)
- Simple collapse when support is removed
- Phase change (Dry → Wet Dirt)

**Out of Scope**:
- LBM or pressure-based fluid simulation
- Temperature or phase changes beyond wetting
- Chemical reactions or gas simulation
- Complex structural engineering (beams, supports)