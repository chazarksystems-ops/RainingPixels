# Math Claim Status Registry

This registry classifies mathematical and performance claims for TWIOFA_SUBSTRATE_PROOF_v0.

## Claim status labels

```text
PROVEN          mathematically true under stated assumptions
CONDITIONAL     true only if a specific implementation rule is followed
APPROXIMATION   accepted game approximation, not a physics proof
REJECTED        false or unsafe as stated
MEASURED_ONLY   cannot be proven usefully without runtime measurement
```

## Claim table

| Claim | Status | Correct wording |
|---|---|---|
| Packed `u32` state round-trips | PROVEN | Proven if Rust/WGSL masks and shifts match and fields are 8-bit |
| Pure gather pass has no write-write races | PROVEN | True if each invocation writes only its own output cell |
| Double-buffering alone eliminates all races | REJECTED | Double-buffering helps, but movement still needs scheduling |
| Symmetric vertical swap conserves mass | CONDITIONAL | True for one uniquely scheduled pairwise swap |
| Diagonal/lateral flow is automatically race-free | REJECTED | Requires shared deterministic tie-breaker plus destination conflict handling |
| Unscheduled scatter writes are safe | REJECTED | Scatter can create write-write races |
| Moisture gather is race-safe | PROVEN | True if each dirt/water cell writes only itself |
| Moisture diffusion is strictly conservative | CONDITIONAL | Requires capacity-aware edge-transfer and excess handling |
| Capped absorption is acceptable | APPROXIMATION | Allowed if documented as soil absorption/loss |
| LBM is required for water | REJECTED | CA water + diffusion is the v0 target |
| Packed `u32` eliminates bandwidth bottleneck | REJECTED | It reduces traffic but does not eliminate bandwidth limits |
| 60 FPS is proven from peak bandwidth | REJECTED | Must be measured |
| 512x512 proof is likely feasible | CONDITIONAL | Plausible, but validate with pass timings |
| Large yard performance is solved by raw GPU power | REJECTED | Requires active-region/chunk scheduling and measurement |
| Taint/ledger is complete chain of custody | CONDITIONAL | Only true if every movement/reaction copies or combines ledger deterministically |

## Required Grok correction

Any document containing a `REJECTED` claim should be patched or marked superseded.
