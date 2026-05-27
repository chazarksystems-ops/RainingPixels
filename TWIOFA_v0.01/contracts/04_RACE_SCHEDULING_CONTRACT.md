# Race Scheduling Contract — TWIOFA_SUBSTRATE_PROOF_v0

## Purpose

This contract prevents implementation agents from assuming that double-buffering alone makes falling-sand style movement race-free.

## Core rule

A compute pass may claim race-freedom only if every output address has exactly one writer.

Formal invariant:

```text
for every invocation i:
    write_set(i) ⊆ { G_next[cell_i] }

for any invocations i, j:
    i ≠ j ⇒ write_set(i) ∩ write_set(j) = ∅
```

Pure gather passes satisfy this by construction.

Movement passes usually do not.

## Safe pass categories

### Category A — Pure gather

Allowed.

Each output cell computes its own next state from current-state neighbors:

```text
G_next[c] = f(G_current[c], neighbors(G_current, c))
```

Examples:
- scent decay,
- local wetness threshold,
- local support classification,
- visual/activity marking,
- dirt moisture intake computed by dirt cell.

### Category B — Scheduled pair/block movement

Allowed if documented.

The pass partitions the grid into non-overlapping interaction regions, such as:
- 2x2 block cellular automata,
- checkerboard offsets,
- Margolus neighborhood,
- even/odd row or column pairing.

Requirement:

```text
No cell may belong to two active movement groups in the same pass.
```

### Category C — Reservation/commit

Allowed if documented.

A multi-pass system:
1. sources propose destination,
2. destination selects one source deterministically,
3. commit pass moves material and clears source.

Requirement:

```text
The destination-selection rule must be deterministic.
```

### Category D — Atomics

Allowed only with explicit justification.

Atomics may be used for counters or low-volume event buffers, but should not be the default movement model for v0 material simulation.

## Forbidden pass shape

Unscheduled scatter is forbidden.

Do not implement:

```text
Thread(source_cell) writes directly into arbitrary destination cell.
```

unless:
- the destination is uniquely scheduled,
- a claim buffer resolves conflicts,
- or the write is atomic and its order-dependence is acceptable.

## Required implementation note per movement pass

Every movement pass must state:

```text
1. pass name
2. source buffer
3. destination buffer
4. scheduling method
5. destination conflict rule
6. source clearing rule
7. deterministic tie-breaker rule
8. known approximation
```

## v0 recommended approach

For the first build, prefer:

```text
block CA / deterministic pair rules
```

over reservation/commit unless correctness becomes unclear.
