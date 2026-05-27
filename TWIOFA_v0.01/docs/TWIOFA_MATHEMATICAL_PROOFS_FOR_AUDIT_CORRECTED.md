# TWIOFA Mathematical / Engineering Proofs for Audit — Corrected v0

## Supersession

This file supersedes any earlier `TWIOFA_MATHEMATICAL_PROOFS_FOR_AUDIT.md` that claims unconditional race-freedom, strict mass conservation, or bandwidth elimination.

## Required interpretation

This is an engineering proof document, not a physics proof of the whole game.

It proves narrow implementation safety properties and identifies where scheduling, approximation, or measurement is required.

---

# Corrected Mathematical / Engineering Proof Notes for TWIOFA_SUBSTRATE_PROOF_v0

## 0. What this document proves

This document does **not** prove that the whole simulation is automatically race-free, physically exact, or guaranteed to hit 60 FPS.

It proves a narrower and more useful set of claims:

```text
1. Packed u32 cell state is a valid GPU-friendly representation.
2. Pure gather passes are race-free by construction.
3. Pairwise symmetric movement can conserve material under strict conditions.
4. Scatter updates create write hazards unless scheduled, claimed, or made atomic.
5. Water/moisture diffusion can be made conservative only if capacity and excess are explicitly accounted for.
6. Bandwidth is plausible, not eliminated; real pass cost must be measured.
```

WGSL is the shader language for WebGPU and supports GPU programs, typed values, and storage-buffer-driven compute workflows. Storage buffers are the correct primitive for large mutable simulation state because they can be read/write, unlike uniform buffers. ([W3C][1])

---

# 1. Cell State Representation

Each simulation cell stores one packed `u32`.

```text
bits  0..7   material_id
bits  8..15  local_flags
bits 16..23  material_scalar_0
bits 24..31  aux_0
```

For `TWIOFA_SUBSTRATE_PROOF_v0`:

```text
material_scalar_0 = moisture/cohesion for dirt-like materials
aux_0             = scent strength for v0
```

Let the packed state be:

```text
S = M | (F << 8) | (Q << 16) | (A << 24)
```

Where:

```text
M = material_id        ∈ [0, 255]
F = local_flags        ∈ [0, 255]
Q = material_scalar_0  ∈ [0, 255]
A = aux_0              ∈ [0, 255]
```

Extraction:

```text
M = S & 0xFF
F = (S >> 8)  & 0xFF
Q = (S >> 16) & 0xFF
A = (S >> 24) & 0xFF
```

## Proof of round-trip correctness

Given all fields are clamped to 8 bits:

```text
M, F, Q, A ∈ [0, 255]
```

Packing places each value into a non-overlapping byte range:

```text
M occupies bits 0..7
F occupies bits 8..15
Q occupies bits 16..23
A occupies bits 24..31
```

Because the byte ranges do not overlap:

```text
(M & 0xFF) == M
((F << 8) >> 8) & 0xFF == F
((Q << 16) >> 16) & 0xFF == Q
((A << 24) >> 24) & 0xFF == A
```

Therefore:

```text
unpack(pack(M, F, Q, A)) = (M, F, Q, A)
```

This proves CPU/GPU representation can be made stable if Rust and WGSL use the same constants and masks.

---

# 2. Double Buffer Model

Let:

```text
G_t      = read-only grid at current frame/pass
G_{t+1}  = write-only grid for next frame/pass
c        = one cell coordinate
N(c)     = neighborhood of c
```

A pass is safe if each thread writes to exactly one unique output cell:

```text
Thread(c) writes only to G_{t+1}[c]
```

and reads only from:

```text
G_t[c] and G_t[k] for k ∈ N(c)
```

Then there are no write-write conflicts because:

```text
c_i ≠ c_j  ⇒  address(G_{t+1}[c_i]) ≠ address(G_{t+1}[c_j])
```

This is the core gather-pass safety rule.

WebGPU compute shaders commonly write output to storage buffers, and readback uses copy/staging buffers when CPU inspection is needed. ([HWS Mathematics and Computer Science][2])

---

# 3. Gather Pass Race-Freedom

A gather rule has this form:

```text
G_{t+1}[c] = f(G_t[c], {G_t[k] | k ∈ N(c)})
```

Each invocation computes only its own output cell.

## Proof

For any two distinct cells `a` and `b`:

```text
a ≠ b
```

Thread `a` writes:

```text
G_{t+1}[a]
```

Thread `b` writes:

```text
G_{t+1}[b]
```

Since the addresses are distinct, no two invocations write the same output address.

Therefore:

```text
pure gather pass ⇒ no write-write race
```

This does **not** prove all material motion is race-free. It proves only that passes written in this form are race-free.

---

# 4. Scatter Hazard

A scatter rule has this form:

```text
Thread(c) writes to G_{t+1}[d]
```

where:

```text
d may be a neighbor cell, not c
```

Example:

```text
water cell pushes moisture into adjacent dirt cell
sand cell writes itself into below cell
```

## Race example

Let dirt cell `D` be adjacent to two water cells:

```text
W1 -- D -- W2
```

Both water cells attempt:

```text
G_{t+1}[D].moisture += 5
```

If both read:

```text
G_t[D].moisture = 0
```

then both compute:

```text
0 + 5 = 5
```

and both may write:

```text
G_{t+1}[D].moisture = 5
```

Expected value:

```text
10
```

Actual value:

```text
5
```

So scatter updates require one of:

```text
atomics
claim buffers
deterministic tie-breaking
block cellular automata
checkerboard/Margolus scheduling
multi-pass reservation/commit
```

GPU falling-sand implementations specifically call out this problem: two cells can try to move into the same destination. One documented solution is block cellular automata, where non-overlapping blocks are processed together to avoid that race class. ([GitHub][3])

---

# 5. Corrected Gravity Movement Proof

Gemini’s original proof was too broad. The correct proof is conditional.

Consider a vertical pair:

```text
u = cell above
d = cell below
```

At time `t`:

```text
G_t[u] = Water
G_t[d] = Air
```

A symmetric pairwise rule swaps them:

```text
G_{t+1}[u] = Air
G_{t+1}[d] = Water
```

## Mass before

Let:

```text
mass(Water) = 1
mass(Air)   = 0
```

Before:

```text
mass_t = mass(G_t[u]) + mass(G_t[d])
       = 1 + 0
       = 1
```

After:

```text
mass_{t+1} = mass(G_{t+1}[u]) + mass(G_{t+1}[d])
           = 0 + 1
           = 1
```

Therefore:

```text
mass_{t+1} = mass_t
```

## What this proves

It proves conservation for one isolated vertical swap.

## What it does not prove

It does not prove safety for:

```text
two water cells targeting the same air cell
diagonal movement
random left/right flow
pressure movement
collapse cascades
multi-material reactions
```

Those require scheduling or conflict resolution.

---

# 6. Corrected Movement Scheduling Requirement

For falling material, naive scatter is unsafe.

A safe implementation must enforce:

```text
Each destination cell is claimed by at most one source cell per pass.
```

There are several valid ways.

## Option A — Gather Pull

Each output cell asks:

```text
Should I receive material from one of my neighbors?
```

Example for a cell `c`:

```text
if G_t[c] == Air:
    candidate_above = c + (0, -1)
    if G_t[candidate_above] == Sand:
        G_{t+1}[c] = Sand
```

But the source cell must also know whether it successfully moved. That requires either:

* symmetric complementary rules,
* source-clearing logic,
* reservation state,
* or a block update where the pair is handled together.

## Option B — Block CA / Margolus-style update

Partition the grid into non-overlapping blocks.

Example:

```text
2x2 blocks
```

Only cells inside each block interact during that pass. Since blocks do not overlap, no two blocks write the same cells.

Alternate block offsets between passes:

```text
Pass A: blocks start at even/even
Pass B: blocks start at odd/odd
```

This is a common strategy for GPU falling-sand simulations because it avoids destination conflicts while preserving local motion over time. ([GitHub][3])

## Option C — Reservation / Claim Buffer

Pass 1:

```text
source cells propose target cells
```

Pass 2:

```text
targets choose one source deterministically
```

Pass 3:

```text
commit moves
```

More expensive, but clear and debuggable.

## TWIOFA v0 recommendation

Use:

```text
block CA or deterministic gather/symmetric pair rules
```

Do not use unscheduled scatter.

---

# 7. Corrected Moisture Diffusion Math

We want water to transfer moisture into dirt without write races and without pretending conservation is automatic.

Let:

```text
Q(c) = material_scalar_0 of cell c
```

For dirt, interpret:

```text
Q = moisture
```

For water, interpret:

```text
Q = water volume / available wetting amount
```

Let:

```text
capacity(c) = 255 - Q(c)
```

For a dirt cell `d`, adjacent water cells are:

```text
W(d) = { w ∈ N(d) | material(w) = Water }
```

A gather-style dirt moisture update:

```text
incoming_request(d) = R * |W(d)|
```

where `R` is transfer rate per adjacent water cell.

Capacity-limited incoming moisture:

```text
incoming(d) = min(capacity(d), incoming_request(d))
```

Then:

```text
Q_{t+1}(d) = Q_t(d) + incoming(d)
```

This avoids dirt-cell write races because the dirt cell computes its own next state.

## But water loss must match dirt gain

For water cell `w`, adjacent dirt cells are:

```text
D(w) = { d ∈ N(w) | material(d) ∈ {DryDirt, WetDirt, LooseSoil} }
```

Naive water loss:

```text
loss_request(w) = R * |D(w)|
```

But this can disagree with actual dirt intake if dirt cells are near capacity.

A conservative version needs per-edge transfer accounting.

For each dirt-water edge `(d, w)`:

```text
edge_request(d, w) = R
edge_capacity_share(d, w) = capacity(d) / max(1, |W(d)|)
edge_transfer(d, w) = min(R, edge_capacity_share(d, w), Q_t(w) / max(1, |D(w)|))
```

Then:

```text
Q_{t+1}(d) = Q_t(d) + Σ edge_transfer(d, w) for w ∈ W(d)
```

and:

```text
Q_{t+1}(w) = Q_t(w) - Σ edge_transfer(d, w) for d ∈ D(w)
```

Now, ignoring rounding:

```text
Σ moisture gained by dirt = Σ volume lost by water
```

So conservation holds for diffusion.

## Practical v0 simplification

For TWIOFA v0, strict conservation may not be necessary. It is acceptable to define:

```text
some water is absorbed as an approximation
excess may disappear into soil microstructure
```

But the doc must state that this is a game approximation, not a formal conservation proof.

Correct statement:

```text
Moisture diffusion is race-free under gather evaluation.
It is conservative only if edge-transfer accounting handles capacity and excess.
```

---

# 8. Material Conversion Thresholds

Dirt material can convert based on moisture:

```text
DryDirt + moisture >= wet_threshold → WetDirt
WetDirt + moisture >= saturated_threshold → Saturated flag
Saturated loose soil + low support → CollapseCandidate
```

Example:

```text
wet_threshold       = 96
saturated_threshold = 192
```

Formal rule:

```text
if material(c) = DryDirt and Q(c) >= 96:
    material_{t+1}(c) = WetDirt
```

```text
if Q(c) >= 192:
    flags_{t+1}(c) = flags_t(c) ∪ SATURATED
```

This is not fluid physics. It is gameplay-oriented material state transition.

---

# 9. Collapse Rule Safety

Collapse can also create races if soil cells scatter into empty cells.

Use gather or block scheduling.

A safe gather-style collapse target rule:

```text
For each output cell c:
    if G_t[c] is Air:
        inspect supported candidate cells above/diagonal
        choose one candidate using deterministic priority
        write candidate material into c
    else:
        retain or clear based on whether this cell was chosen by a paired/scheduled rule
```

But clearing the source remains the hard part. Therefore, for v0:

```text
collapse movement should use block CA or reservation/commit
```

A simpler first collapse rule can avoid movement and use local material conversion:

```text
unsupported wet/loose dirt becomes LooseSoil
LooseSoil participates in scheduled falling pass
```

This separates:

```text
structural failure detection
```

from:

```text
material movement
```

That is cleaner.

---

# 10. Determinism Conditions

A GPU pass can be deterministic if:

```text
1. every output cell has exactly one writer
2. no atomics with order-dependent accumulation are used
3. random choices are generated from stable coordinates/pass index, not thread timing
4. pass order is fixed
5. floating point nondeterminism is avoided or contained
```

For v0, prefer integer math:

```text
u32 fields
integer thresholds
coordinate hash for pseudo-random tie-breaking
fixed pass order
```

Do not use runtime random thread order.

---

# 11. Corrected Bandwidth Estimate

Gemini’s bandwidth estimate was too optimistic.

Let:

```text
W = grid width
H = grid height
C = W * H = number of cells
B = bytes per cell = 4
R = number of cell reads per invocation
W_r = number of cell writes per invocation
P = number of passes per frame
BW = available memory bandwidth
```

Approximate traffic per frame:

```text
Traffic ≈ C * B * (R + W_r) * P
```

Time lower bound:

```text
T_min ≈ Traffic / BW
```

This is only a lower bound because it ignores:

* cache effects
* alignment
* occupancy
* dispatch overhead
* render pass
* synchronization
* CPU/GPU copies
* branch divergence
* extra buffers
* staging/debug reads

## Example: 512x512

```text
C = 512 * 512 = 262,144 cells
B = 4 bytes
```

Assume:

```text
R = 5 reads
W_r = 1 write
P = 4 passes
```

Traffic:

```text
262,144 * 4 * (5 + 1) * 4
= 25,165,824 bytes
≈ 24 MB/frame
```

At 60 FPS:

```text
24 MB * 60 = 1.44 GB/s
```

This is trivially plausible on a modern GPU.

## Example: 1024x512

```text
C = 524,288 cells
```

Same assumptions:

```text
524,288 * 4 * 6 * 4
= 50,331,648 bytes
≈ 48 MB/frame
```

At 60 FPS:

```text
48 MB * 60 = 2.88 GB/s
```

Still plausible.

## Example: 4000x4000

```text
C = 16,000,000 cells
```

Assume only one pass with 5 reads + 1 write:

```text
16,000,000 * 4 * 6 * 1
= 384,000,000 bytes
≈ 366 MB/pass
```

Four passes:

```text
≈ 1.46 GB/frame
```

At 60 FPS:

```text
≈ 87.9 GB/s
```

That may still be below peak bandwidth, but it is no longer “free.” It also ignores render and overhead.

Correct conclusion:

```text
u32 packing makes the design bandwidth-plausible.
It does not eliminate memory bandwidth as a bottleneck.
The single-chunk proof is safely sized.
Large worlds require active-region/chunk scheduling.
```

---

# 12. Corrected Hardware Claim

Do not write:

```text
The u32 layout proves 60 FPS on 4070 Ti Super.
```

Write:

```text
The u32 layout minimizes per-cell bandwidth and makes 512x512 or 1024x512 substrate proof comfortably plausible.
Larger worlds must rely on active chunks and measured pass cost.
```

This distinction matters.

---

# 13. Correct TWIOFA v0 Proof Summary

## Claim A — packed cell state correctness

Proven if:

```text
all fields are 8-bit
Rust and WGSL masks/shifts match
test vectors round-trip
```

Status:

```text
provable with unit tests
```

## Claim B — pure gather pass race-freedom

Proven if:

```text
each invocation writes only G_{t+1}[c]
```

Status:

```text
mathematically true
```

## Claim C — vertical pair swap mass conservation

Proven if:

```text
one water and one air cell swap states
no other cell targets either address
the pair is scheduled uniquely
```

Status:

```text
true for that isolated scheduled transition
```

## Claim D — whole falling-sand movement race-freedom

Requires:

```text
block CA
reservation/commit
deterministic tie-breaking
or atomics
```

Status:

```text
not automatic
must be designed
```

## Claim E — moisture diffusion conservation

Requires:

```text
capacity-aware edge transfer
matching water loss to dirt gain
explicit rounding/excess handling
```

Status:

```text
conditional
not guaranteed by simple min/max formulas
```

## Claim F — performance

For substrate proof sizes:

```text
512x512 or 1024x512
```

Status:

```text
very plausible
```

For large continuous worlds:

```text
requires active chunks and measurement
```

Status:

```text
not proven by bandwidth math alone
```

---

# 14. Corrected lock text for docs

Use this exact lock:

```text
TWIOFA_SUBSTRATE_PROOF_v0 uses double-buffered GPU cellular automata passes over packed u32 cell state.

Pure gather passes are race-free because each invocation writes only its own output cell.

Movement passes are not automatically race-free. Falling material, lateral flow, and collapse must use block CA, deterministic pair rules, or reservation/commit scheduling so that each output cell has at most one writer.

Moisture diffusion uses gather evaluation for safety. Strict conservation requires explicit edge-transfer accounting for dirt capacity and water volume. Simpler capped absorption is allowed only if documented as a gameplay approximation.

Packed u32 state minimizes bandwidth and is appropriate for the substrate proof. It does not prove large-world performance. Large yards require active-region simulation and measured pass budgets.

LBM/full fluid solvers remain rejected for v0.
```

# Bottom line

The corrected math supports the architecture, but with honest boundaries:

```text
Yes to packed u32.
Yes to double-buffering.
Yes to gather passes.
Yes to CA water/diffusion.
No to claiming all races are impossible.
No to claiming strict conservation without excess accounting.
No to claiming bandwidth is eliminated.
```

That is the version that is strong enough to hand to a serious developer.

[1]: https://www.w3.org/TR/WGSL/?utm_source=chatgpt.com "WebGPU Shading Language"
[2]: https://math.hws.edu/graphicsbook/c9/s6.html?utm_source=chatgpt.com "Compute Shaders"
[3]: https://github.com/GelamiSalami/GPU-Falling-Sand-CA?utm_source=chatgpt.com "GPU Falling sand simulation using block cellular automata"
