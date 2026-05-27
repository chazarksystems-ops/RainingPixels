# Technical Evidence Index

This file records the external technical ideas that influenced the foundation. It is not a bibliography lock and does not replace implementation measurement.

## Noita / Falling-Sand Direction

TWIOFA uses Noita-like material consequence as a design direction marker: simulated material behavior should produce gameplay consequences. TWIOFA should not copy Noita, but should preserve the idea that material itself is part of the game logic.

## WGSL / WebGPU Direction

The current runtime direction is Rust + WGSL / WebGPU-style compute because the design needs portable GPU compute over packed storage-buffer-like cell data.

## GPU Race Condition Lessons

GPU falling-sand systems can race when multiple cells try to claim the same destination. Double buffering alone is not enough. Movement needs scheduling, pair rules, block CA, claim buffers, or reservation/commit.

## Unified Memory / Future Hardware Note

The main TWIOFA path should remain portable. DGX/CUDA/AntVerse experiments can exist as side lanes, but main TWIOFA should avoid CUDA-only assumptions so future AMD unified-memory mini-PC/APU systems remain plausible targets.
