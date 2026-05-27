# Slice Goal

Create a precise contract for packed `u32` cell state and GPU buffer layout.

The goal is confidence that CPU-side setup and WGSL-side compute code interpret the same bits identically.

This is the first implementation-adjacent slice because cell-state mismatch would poison every later material rule.
