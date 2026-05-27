# Agent Task Prompt

You are refining `S02_CELL_STATE_AND_GPU_BUFFERS`.

Do not implement gameplay.

Create or refine the cell-state contract so Rust and WGSL can later implement the same packed `u32` representation.

Required:
- exact bit layout
- material ids
- flags
- masks and shifts
- scalar/aux v0 meanings
- test vectors
- invariants
- excluded scope

Do not add ant behavior, rendering, sparse chunks, or material simulation rules beyond definitions.
