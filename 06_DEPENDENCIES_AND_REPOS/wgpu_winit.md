# Core Dependencies — wgpu + winit + bytemuck + pollster

## Why These Crates

- **wgpu** (v0.19+): Modern, safe, cross-platform GPU abstraction. Excellent compute shader support. Works great on NVIDIA (including your DGX Spark). Uses WGSL or SPIR-V.
- **winit** (v0.29+): Clean windowing and input handling. Used in your existing prototypes.
- **bytemuck**: Zero-copy casting between Rust structs and GPU buffers. Essential for `SimParams` and grid data.
- **pollster**: Simple `block_on` for async wgpu initialization in a sync `main()`.

These are the exact same dependencies you already used in the pasted AntVerse prototypes — they are proven and stable for this workload.

## Recommended Cargo.toml (Phase 0)

```toml
[package]
name = "antverse"
version = "0.1.0"
edition = "2021"

[dependencies]
wgpu = "0.19"
winit = "0.29"
pollster = "0.3"
bytemuck = { version = "1.15", features = ["derive"] }
```

## Future Additions (When Needed)

- `rand` or `fastrand` — for any non-deterministic visual effects (never for core physics rules).
- `image` — for loading/saving simulation screenshots or exporting heightmaps.
- `serde` + `serde_json` — for saving/loading world state or event logs.
- `tracing` or simple `log` — for structured telemetry output.

We keep the dependency list intentionally minimal in early phases.

## Version Pinning Strategy

Pin major versions (as above) and document any upgrade rationale in this file when we bump them. This supports reproducibility for your deterministic computing goals.
