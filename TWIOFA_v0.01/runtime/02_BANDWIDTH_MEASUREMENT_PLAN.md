# Bandwidth Measurement Plan — TWIOFA_SUBSTRATE_PROOF_v0

## Purpose

This document prevents false performance claims.

The packed `u32` layout makes bandwidth plausible. It does not prove large-world performance.

## Measurement principle

Performance claims must be based on measured pass timings, not theoretical peak GPU bandwidth.

## Formula for lower-bound traffic estimate

Let:

```text
C = cell count
B = bytes per cell = 4
R = reads per invocation
W = writes per invocation
P = passes per frame
```

Approximate traffic:

```text
traffic_per_frame ≈ C * B * (R + W) * P
```

This ignores:
- cache behavior,
- dispatch overhead,
- workgroup occupancy,
- render pass,
- staging/readback,
- branch divergence,
- alignment/padding,
- extra buffers.

It is a lower bound, not a guarantee.

## Required proof sizes

Measure at:

```text
256x256
512x512
1024x512
1024x1024 optional
```

Do not optimize sparse chunking before the 512x512 or 1024x512 substrate proof is visually and mechanically validated.

## Required pass timings

Record GPU timings for:

```text
material_update
water_spread
moisture_diffusion
collapse_classification
scent_aux_update
terrain_render
debug_readback optional
```

## Acceptance targets

For substrate proof:

```text
512x512:
  target: stable interactive frame pacing
  hard requirement: visually responsive enough for playtest

1024x512:
  target: plausible high-resolution proof
  hard requirement: no catastrophic stalls under simple material rules
```

Do not promise 60 FPS until measured.

## Required report format

```text
Resolution:
Cell count:
Pass list:
Average frame time:
Worst frame time:
Per-pass GPU timing:
CPU overhead:
Readback enabled? yes/no
Observed artifacts:
Verdict:
```

## Correct performance language

Allowed:

```text
Packed u32 minimizes per-cell traffic.
The proof-size workload is expected to be plausible.
Measured timings show X ms at Y resolution.
```

Forbidden:

```text
Memory bandwidth is eliminated.
60 FPS is mathematically proven.
The 4070 Ti Super will chew through any yard size.
```
