// src/simulation/events.rs
// Antverse Phase 0 — Event Ledger and WorldEvent types
// Authoritative implementation of RESOLVED_0.1 and 07_TELEMETRY_AND_OBSERVABILITY/EVENT_LEDGER_DESIGN.md
//
// This file defines the exact 16-byte WorldEvent layout, event kinds for Phase 0,
// and the GPU/host EventLedger contract. The design guarantees:
// - Deterministic emission (same seed + brush sequence → identical event stream)
// - Unconditional FrameSummary (type 0) with before/after mass (16.16 fixed-point)
// - Sparse notable events (MaterialTransition, BrushAction)
// - Atomic append on GPU inside the single CommandEncoder
// - Zero-copy host consumption on unified memory
//
// Cross-references:
// - 12_RESEARCH_NEEDED/RESOLVED_0.1_Event_Ledger_WorldEvent_Data_Model.md
// - 07_TELEMETRY_AND_OBSERVABILITY/EVENT_LEDGER_DESIGN.md (full spec)
// - 12_RESEARCH_NEEDED/RESOLVED_0.2_Mass_Conservation_Verification_Strategy.md (mass payload contract)
// - 12_RESEARCH_NEEDED/RESOLVED_0.3_Determinism_Validation_Harness_Design.md (bit-identity requirement)
// - 04_DESIGN_DECISIONS/STATE_PACKING.md (material IDs and moisture/volume fields)

use bytemuck::{Pod, Zeroable};

/// Maximum number of events the ledger can hold per frame.
/// Chosen for Phase 0: large enough for heavy brush + notable transitions on 4096²,
/// small enough to be trivial memory (64 KiB per slot).
pub const EVENT_CAPACITY: usize = 4096;

/// Event kinds for Phase 0 (frozen per 0.1 decision).
pub const EVENT_KIND_FRAME_SUMMARY: u32 = 0;
pub const EVENT_KIND_MATERIAL_TRANSITION: u32 = 1;
pub const EVENT_KIND_BRUSH_ACTION: u32 = 2;

/// 16-byte WorldEvent (exactly as specified in EVENT_LEDGER_DESIGN.md and RESOLVED_0.1).
///
/// Layout (little-endian, matches WGSL struct WorldEvent):
/// - header: bits 0-7 = kind, bits 8-31 = frame (or flags)
/// - pos: low 16 bits = x, high 16 bits = y (packed coordinate; linear index alternative in some paths)
/// - data0 / data1: kind-specific payload (see EVENT_LEDGER_DESIGN.md for precise packing)
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WorldEvent {
    pub header: u32,
    pub pos: u32,
    pub data0: u32,
    pub data1: u32,
}

impl WorldEvent {
    /// Construct a FrameSummary event (type 0) with before/after mass.
    /// Mass values use 16.16 fixed-point encoding (matches 0.2 verification contract).
    #[inline]
    pub fn frame_summary(frame: u32, before_mass: u32, after_mass: u32) -> Self {
        Self {
            header: EVENT_KIND_FRAME_SUMMARY | (frame << 8),
            pos: 0,
            data0: before_mass,
            data1: after_mass,
        }
    }

    /// Returns the event kind (0-255).
    #[inline]
    pub fn kind(&self) -> u32 {
        self.header & 0xFF
    }

    /// Returns the frame number embedded in the header (Phase 0 encoding).
    #[inline]
    pub fn frame(&self) -> u32 {
        (self.header >> 8) & 0x00FF_FFFF
    }
}

/// GPU-side EventLedger layout (matches WGSL).
/// The head is an atomic<u32> on the GPU side; the host reads the written u32 value after submit.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct EventLedgerGpu {
    pub head: u32,
    pub _pad: u32,
    pub events: [WorldEvent; EVENT_CAPACITY],
}

/// Host-side view of a completed ledger slot.
/// After queue.submit the head value tells how many valid events were written.
pub struct EventLedgerView<'a> {
    pub head: u32,
    pub events: &'a [WorldEvent],
}

impl EventLedgerGpu {
    /// Returns a view of the events that were actually written (0..head).
    pub fn as_view(&self) -> EventLedgerView {
        let head = self.head.min(EVENT_CAPACITY as u32);
        EventLedgerView {
            head,
            events: &self.events[..head as usize],
        }
    }
}
