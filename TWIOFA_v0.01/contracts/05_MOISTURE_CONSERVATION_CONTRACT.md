# Moisture Conservation Contract — TWIOFA_SUBSTRATE_PROOF_v0

## Purpose

This contract defines the difference between race-safe moisture diffusion, conservative moisture transfer, and gameplay approximation.

## Key distinction

A gather pass can be race-free while still not conserving moisture.

Race-free means:

```text
No two invocations write the same output address.
```

Conservative means:

```text
Total water volume lost by water cells equals total moisture gained by dirt cells,
except for explicitly declared sinks/sources.
```

These are different properties.

## Definitions

For each cell `c`:

```text
material(c) ∈ {Air, DryDirt, WetDirt, LooseSoil, Water, StoneRoot}
Q(c) = material_scalar_0(c)
```

Interpretation:

```text
if material is dirt-like:
    Q = moisture/cohesion

if material is water:
    Q = available volume/wetting amount
```

## Race-safe gather moisture intake

For dirt cell `d`:

```text
W(d) = adjacent water neighbors
capacity(d) = 255 - Q(d)
incoming_request(d) = R * |W(d)|
incoming(d) = min(capacity(d), incoming_request(d))
Q_next(d) = Q(d) + incoming(d)
```

This is race-safe because the dirt cell writes only itself.

## Why this is not automatically conservative

Water cells may separately compute:

```text
loss_request(w) = R * adjacent_dirt_count
```

But this may not match actual dirt intake if:
- dirt is near capacity,
- water has insufficient volume,
- several dirt cells share one water cell,
- clamping occurs,
- integer rounding occurs.

Therefore:

```text
gather-safe does not imply conservation.
```

## Conservative edge-transfer policy

For strict conservation, define transfers per dirt-water adjacency edge.

For edge `(d,w)`:

```text
edge_capacity_share(d,w) = capacity(d) / max(1, water_neighbor_count(d))
edge_volume_share(d,w) = Q(w) / max(1, dirt_neighbor_count(w))
edge_transfer(d,w) = min(R, edge_capacity_share(d,w), edge_volume_share(d,w))
```

Then:

```text
Q_next(d) = Q(d) + Σ edge_transfer(d,w)
Q_next(w) = Q(w) - Σ edge_transfer(d,w)
```

Ignoring rounding:

```text
Σ dirt_gain = Σ water_loss
```

## v0 permitted approximation

For v0, strict conservation is optional.

The engine may allow:

```text
water absorbed into soil microstructure
```

as an explicit sink if documented.

Allowed approximation:

```text
excess water is discarded as absorption loss when dirt capacity clamps.
```

Forbidden wording:

```text
This proves strict mass conservation.
```

Correct wording:

```text
This is a race-safe moisture approximation with capped absorption.
```

## Required implementation declaration

Every moisture pass must declare one of:

```text
Mode A: Conservative edge-transfer
Mode B: Capped absorption approximation
Mode C: Hybrid approximation with measured loss tracking
```

For first substrate proof, `Mode B` is acceptable if visual/gameplay feel is the priority and if the sink is documented.
