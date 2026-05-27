# S02_CELL_STATE_AND_GPU_BUFFERS

## Purpose

Define the first implementation-adjacent slice: the CPU/GPU cell-state contract.

This slice should not implement gameplay. It defines and later validates the packed `u32` cell representation and buffer invariants.

## Why This Comes First

Every material rule, render pass, moisture update, collapse rule, and ant interaction depends on stable cell-state semantics.

## Required Result

Rust and WGSL must agree on:

- material id byte
- local flags byte
- material scalar byte
- aux byte
- masks
- shifts
- test vectors
