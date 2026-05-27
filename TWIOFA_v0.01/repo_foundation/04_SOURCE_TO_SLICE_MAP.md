# Source to Slice Map

## Purpose

This map prevents source drift by assigning each source packet or idea to a build slice.

## Current mappings

| Source / idea | Slice destination | Notes |
|---|---|---|
| Packed `u32` cell state | S02 | Authoritative for substrate proof. |
| Corrected math proofs | S02/S03/S06/S07 | Required reading for compute/movement/moisture/collapse. |
| Race scheduling contract | S03/S06/S07 | Must govern movement passes. |
| Moisture conservation contract | S06 | Decides strict transfer vs v0 approximation. |
| Bandwidth measurement plan | S03/S10 | Required before performance claims. |
| Anti-tile / anti-debug-board rules | S04 | Rendering acceptance. |
| World-click visible execution | S05 | Input + visible execution proof. |
| Minimal ant contract | S08 | Avoids full ECS drift. |
| Route consequence loop | S09 | Integrates substrate proof. |
| LBM/full CFD discussion | Rejected registry | Do not reintroduce in v0. |
| Sparse chunking | Deferred registry | Future after proof. |
| Open yard / Deep Hive / POI systems | Future slices S12-S15 | Intent-only until substrate proof passes. |
| DGX / AntVerse experiment lane | S17 / archive | Separate research lane, not main target. |
| Old chunk0 macroquad prototype | Archive/reference only | Historical behavior evidence; not actual game direction. |

## Rule

Every newly harvested source item must be mapped to one of:
- current slice,
- future slice,
- deferred registry,
- rejected registry,
- archive.
