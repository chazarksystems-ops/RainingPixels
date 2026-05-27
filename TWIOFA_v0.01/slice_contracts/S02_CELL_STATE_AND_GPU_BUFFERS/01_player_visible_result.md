# Player-Visible Result

This slice has no direct player-visible result by itself.

Its value is enabling the first visible substrate proof.

A later player-visible result depends on this slice: material cells render correctly and respond consistently because Rust and WGSL agree on cell state.

## Why This Still Matters to the Player

Although this slice is technical, it directly affects what the player eventually sees. If material ids, flags, moisture, or aux values are misread between Rust and WGSL, then dirt may render as water, wet soil may not behave as wet, collapse markers may be lost, or scent/aux fields may become unreadable to ants.

The player-visible result appears in later slices: material behavior becomes stable, readable, and deterministic enough that ants and terrain can interact without invisible state corruption.
