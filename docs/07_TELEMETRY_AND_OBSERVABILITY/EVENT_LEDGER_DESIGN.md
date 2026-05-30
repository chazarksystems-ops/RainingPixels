# Event Ledger Design — WorldEvent Data Model and Emission Strategy (Task 0.1)

**Status**: Resolved — Phase 0 Foundation  
**Date**: 2026-05-25  
**Task ID**: 0.1  
**Related**: `12_RESEARCH_NEEDED/RESOLVED_0.1_Event_Ledger_WorldEvent_Data_Model.md` (core decision), `RESOLVED_0.2_Mass_Conservation_Verification_Strategy.md`, `RESOLVED_0.3_Determinism_Validation_Harness_Design.md`

## Purpose
This document is the single source of truth for the Phase 0 (and forward) Event Ledger implementation. It defines the exact binary layout of `WorldEvent`, the `EventLedger` GPU buffer structure, the WGSL emission contract (`try_emit_event`), the host-side consumption API, double-buffering / slot management, and integration points with mass verification (0.2), determinism harness (0.3), Cede/Entelechy agents, and creative tooling.

All code in `src/simulation/events.rs`, `state.rs`, `pipeline.rs`, and the gravity (and future) WGSL shaders must follow this specification without deviation.

## Core Design Principles (Non-Negotiable)
- **16 bytes per event**: Fixed size for coalescing, cache friendliness, and simple host mapping. Never widen in Phase 0–3.
- **Atomic append via atomicAdd**: Single u32 head counter reserves slots. Sparse, low-contention emission inside the single CommandEncoder.
- **FrameSummary is unconditional**: Exactly one type-0 event per frame carrying before/after mass (16.16 fixed-point u32) plus activity counters. This is the heartbeat for 0.2 verification and 0.3 replay identity.
- **Determinism first**: Same seed + brush sequence produces bit-identical event streams (including exact mass pairs) on CPU ref and GPU. No per-thread non-determinism in emission decisions.
- **Zero-copy host read on unified memory**: After submit, the prior frame's ledger is directly readable. No staging copies in the common path.
- **Double-buffered / slotted**: GPU writes to inactive ledger while host reads the completed one. Flipped via `SimParams.event_ledger_index`.
- **Sparse notables**: Only semantically meaningful transitions are emitted (absorption, major material changes, brush actions). Per-frame notables are best-effort; overflow drops excess with a flag in the summary.

## WorldEvent Binary Layout (Exactly 16 Bytes)

**Rust (bytemuck, repr(C))**:
```rust
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WorldEvent {
    /// bits 0-7: event kind (0=FrameSummary, 1=MaterialTransition, 2=BrushAction)
    /// bits 8-31: frame number (low 24 bits sufficient for demo runs) or flags
    pub header: u32,
    /// Packed coordinate: low 16 bits = x, high 16 bits = y (or linear index for simplicity)
    pub pos: u32,
    /// Kind-specific payload 0
    pub data0: u32,
    /// Kind-specific payload 1
    pub data1: u32,
}
```

**WGSL equivalent** (exact memory layout match):
```wgsl
struct WorldEvent {
    header: u32,
    pos: u32,
    data0: u32,
    data1: u32,
};
```

### Event Kind Definitions (Phase 0)

- **0 — FrameSummary** (always emitted, occupies one slot):
  - `header`: kind=0 | (frame << 8)
  - `pos`: 0 (unused) or global activity hint
  - `data0`: before_mass as u32 (16.16 fixed-point: integer part in high 16, fractional in low 16)
  - `data1`: after_mass as u32 (same encoding)
  - Interpretation must match the exact mass definition in RESOLVED_0.2.

- **1 — MaterialTransition** (notable cell change caused by physics):
  - `header`: kind=1 | (frame << 8)
  - `pos`: packed (x | (y << 16))
  - `data0`: bits 0-7 = old_material, 8-15 = new_material, 16-23 = moisture_or_volume_delta (signed 8-bit), 24-31 = flags (e.g. was_collapse)
  - `data1`: optional extra (reserved or net mass contribution for quick summation)

- **2 — BrushAction** (user-initiated mass change):
  - `header`: kind=2 | (frame << 8)
  - `pos`: center of brush (packed)
  - `data0`: material painted + radius + strength packed
  - `data1`: exact net mass delta applied by this brush stroke (16.16) — required for 0.2 accounting

Reserved kinds 3–255 are available for Phase 3+ (capillary events, scent birth, structural drama) without changing the 16-byte layout.

## EventLedger GPU Buffer Layout

The ledger is a single storage buffer (or two for explicit double-buffering) with this layout:

```wgsl
struct EventLedger {
    head: atomic<u32>,           // current write index; reset to 0 at start of frame that owns this slot
    _pad: u32,
    events: array<WorldEvent, EVENT_CAPACITY>,
};
```

- Capacity: 4096 recommended for Phase 0 (tunable via uniform or const; 16K is still only 256 KB).
- Two instances (or one large buffer + offset arithmetic) indexed by `SimParams.event_ledger_index (0/1)`.
- At the beginning of the frame that will write to slot N, the compute shader (or host before dispatch) does a non-atomic write or clear of `head = 0` for that slot. (Clear can be a tiny compute pass or host memset on unified memory.)

## WGSL Emission Contract — `try_emit_event`

Inside gravity.wgsl (and future passes) after the physics logic:

```wgsl
// Bind group layout example (optional group for zero-cost disable)
@group(1) @binding(0) var<storage, read_write> event_ledger: EventLedger;
@group(1) @binding(1) var<uniform> params: SimParams;  // contains event_ledger_index, frame, etc.

fn try_emit_event(ev: WorldEvent) -> bool {
    // Simple single-atomic path (Phase 0)
    let idx = atomicAdd(&event_ledger.head, 1u);
    if (idx >= EVENT_CAPACITY) {
        return false;  // overflow; caller may still force FrameSummary
    }
    event_ledger.events[idx] = ev;
    return true;
}

// Unconditional FrameSummary emission (call once per frame at end of physics)
fn emit_frame_summary(before_mass: u32, after_mass: u32) {
    var ev: WorldEvent;
    ev.header = 0u | (params.frame << 8u);
    ev.pos = 0u;
    ev.data0 = before_mass;
    ev.data1 = after_mass;
    // Force write even on overflow by writing to slot 0 if necessary
    let idx = atomicAdd(&event_ledger.head, 1u);
    if (idx < EVENT_CAPACITY) {
        event_ledger.events[idx] = ev;
    } else {
        event_ledger.events[0] = ev;  // overwrite oldest; summary must survive
        // Set a "truncated" flag in a separate summary struct or header bit if needed
    }
}
```

**Rule**: FrameSummary is emitted exactly once per frame. MaterialTransition and Brush events are emitted only for notable, deterministic cases (e.g., water volume absorbed > threshold, dirt→sand collapse, brush stamp completion). Emitters must be pure functions of cell state + global seed so CPU ref produces identical decisions.

## Host-Side Structures and API (Rust)

```rust
// In src/simulation/events.rs
pub const EVENT_CAPACITY: usize = 4096;

#[repr(C)]
pub struct EventLedgerGpu {
    pub head: u32,                    // written by GPU; host reads after submit
    pub _pad: u32,
    pub events: [WorldEvent; EVENT_CAPACITY],
}

pub struct EventLedgerHost {
    // Double-buffered GPU resources
    buffers: [wgpu::Buffer; 2],
    // Current write slot (GPU)
    write_slot: u32,
    // Last completed readable slot
    read_slot: u32,
}

impl EventLedgerHost {
    pub fn new(device: &wgpu::Device) -> Self { ... }

    /// Called before dispatch for the frame that will write to the ledger.
    pub fn prepare_for_frame(&mut self, queue: &wgpu::Queue, frame: u32) {
        // Reset head on the target slot (unified memory: can be direct write or tiny upload)
        self.write_slot = (frame & 1) as u32;  // or explicit uniform
        // zero the head u32 in the buffer for write_slot
    }

    /// After queue.submit, the previous frame's ledger is ready.
    pub fn drain_events(&self) -> &[WorldEvent] {
        // Map or direct read on unified memory the read_slot buffer up to its head count
        // Return slice of valid events (head is the count)
    }

    /// Flip for next frame
    pub fn flip(&mut self) { ... }
}
```

The host never writes the event data — only the GPU appends. Host only clears heads between frames and reads completed ledgers.

## Integration Points

- **Mass Verification (0.2)**: After submit, read FrameSummary from `drain_events()`, extract before/after mass, compare against host exact grid scan + previous total. Any unexplained delta is fatal in debug.
- **Determinism Harness (0.3)**: CPU reference implementation must emit an identical sequence of WorldEvents (same count, same header/pos/data fields) for any recorded brush log. The harness asserts stream identity as one of the three comparison levels.
- **Brush System (0.1 phase)**: Every brush stamp that changes mass emits exactly one BrushAction event with precise net delta so 0.2 accounting stays balanced.
- **Cede / Entelechy / Agents**: Consume the stream in real time or from log. FrameSummary gives cheap global state; notables give causal narrative ("water absorbed at (312, 847) frame 1243 causing moisture +3").
- **Creative Sonification (EVENT_STREAM_AS_CREATIVE_MATERIAL.md)**: Map FrameSummary mass deltas and collapse transitions to musical parameters. Determinism guarantees reproducible performances from identical brush sequences.
- **Pipeline (single CommandEncoder)**: Ledger binding is part of the linear physics sequence. Emission happens inside gravity (and later JFA/capillary) passes. No extra dispatches for telemetry in Phase 0.

## Capacity, Overflow, and Performance

- Start with 4096 events (64 KB per ledger × 2 = 128 KB total). Trivial on DGX Spark.
- Overflow policy: FrameSummary is guaranteed (reserve or overwrite). Excess notables dropped; summary carries a "notable_count_truncated" flag or approximate count.
- Contention: With expected < 100 emissions per frame even under heavy brush, a single atomic head is sufficient. Profile in Phase 0.3 on 4096² with storm scenarios. Sharding (per-tile heads + later merge) is a documented future optimization only.
- Cost: One atomicAdd per emitted event + one write of 16 bytes. Negligible compared to gravity halo loads.

## Versioning and Evolution

The 16-byte layout + kind values 0–2 are frozen for Phase 0–3. Future richer creative events will live in a parallel detail buffer (indexed from the main event) or a second high-priority ledger. Any change to the core 16-byte header must be accompanied by a new RESOLVED note and a version bump in SimParams.

## Verification Checklist (for Phase 0.3 implementation)

- [ ] WGSL and Rust layouts are identical (bytemuck round-trip test).
- [ ] FrameSummary emitted exactly once per frame with correct 0.2 mass values.
- [ ] CPU reference emits identical stream (including mass pairs) for any input log.
- [ ] Host can drain without GPU stall on unified memory.
- [ ] Brush events exactly balance the mass delta they introduce.
- [ ] No event emission outside the single CommandEncoder physics chain.

**This design satisfies every requirement stated across the Phase 0 research artifacts while remaining minimal, deterministic, and directly consumable by agents and creative tools. Implementation may now proceed in strict phase order.**

**Primary references**: RESOLVED_0.1 (decision), RESOLVED_0.2 (mass payload contract), PHASE0_DEMO_IMPLEMENTATION_PLAN.md (module and Phase 0.3 steps), RESEARCH_ROADMAP.md (Task 0.1 deliverable).
