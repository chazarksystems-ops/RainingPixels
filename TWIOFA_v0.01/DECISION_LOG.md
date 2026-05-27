# DECISION_LOG.md — Initial Accepted TWIOFA Locks

This log records the accepted decisions at the time of the initial v0.01 foundation seed.

## Accepted Direction

### 1. Substrate-Proof-First

TWIOFA begins with `TWIOFA_SUBSTRATE_PROOF_v0`, not a general engine. The first proof must validate whether the material substrate is compelling before infrastructure expands.

### 2. One Active Chunk First

The first playable proof uses one fixed active material chunk / one-screen substrate. Sparse chunking, streaming, and infinite yard infrastructure are deferred until the substrate loop proves itself.

### 3. Runtime Direction

The current runtime direction is Rust + WGSL / WebGPU-style packed-cell GPU cellular automata.

The cell model is based on a packed `u32`:

- bits 0..7: material id
- bits 8..15: local flags
- bits 16..23: material scalar / moisture / cohesion
- bits 24..31: aux / initial scent channel

### 4. Material Consequence Is Core

The game is not a colony UI with terrain decoration. The terrain/material world is the gameplay substrate.

Digging, moisture, collapse, route blockage, and visible ant execution must affect player decisions.

### 5. LBM Rejected for v0

Lattice Boltzmann / full CFD is rejected for the first proof. V0 uses CA water, lateral spread, capillary moisture, and material state transitions.

### 6. Anti-Tile Lock

The actual game direction rejects flat square tile-board presentation. Internal cells may exist, but presentation must aim at continuous material readability.

### 7. World-Click Visible Execution

The player should act on the world. Ants should visibly execute through the world. Consequence must be visible in the material field, not only described in text.

### 8. Layered Field Identity

TWIOFA has three major layer identities:

- Upper: broad context and yard orientation
- Ground/Open Yard: tactical traversal, surface food/hazards/routes
- Underground: material substrate, tunneling, moisture, collapse, nest vulnerability

The first proof focuses underground/material consequence without erasing the larger layered game identity.

### 9. Corrected Math Boundaries

The accepted math position is conservative:

- packed state is provable by round-trip tests
- pure gather passes are race-free under single-writer output
- movement needs scheduling
- moisture conservation is conditional
- bandwidth must be measured
- old overclaims are rejected

### 10. Historical Chunk0 Status

Old chunk0/macroquad/tile-grid work is historical reference only. It may provide lessons about playability failures, but it is not the active implementation target.
