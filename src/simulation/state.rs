// src/simulation/state.rs
// Antverse Phase 0 — Core data structures (SimParams, packed cell state, EventLedger ownership)
// Per 02_ARCHITECTURE/PHASE0_DEMO_IMPLEMENTATION_PLAN.md Step 2
// and the resolved research artifacts.
//
// This module owns the pure data model that is shared between CPU reference (0.5),
// host verification (0.4), GPU buffers (pipeline), and event emission (0.3).
//
// Cross-references:
// - 04_DESIGN_DECISIONS/STATE_PACKING.md (authoritative u32 cell layout)
// - 07_TELEMETRY_AND_OBSERVABILITY/EVENT_LEDGER_DESIGN.md (EventLedger + WorldEvent contract from 0.1)
// - 12_RESEARCH_NEEDED/RESOLVED_0.1_Event_Ledger_WorldEvent_Data_Model.md
// - 12_RESEARCH_NEEDED/RESOLVED_0.2_Mass_Conservation_Verification_Strategy.md (mass definition)
// - 05_CODE_TEMPLATES/wgsl/phase0_classic.wgsl (baseline SimParams shape)

use crate::simulation::events::{EventLedgerGpu, WorldEvent, EVENT_CAPACITY};
use bytemuck::{Pod, Zeroable};

/// Material IDs (Phase 0 canonical set — matches STATE_PACKING.md and classic template).
pub const MAT_AIR: u8 = 0;
pub const MAT_SAND: u8 = 1;
pub const MAT_WATER: u8 = 2;
pub const MAT_DIRT: u8 = 3;   // treated as static solid in Phase 0 (becomes mobile in later phases)
pub const MAT_STONE: u8 = 4;

/// Flag bit reserved for future pre-stabilization / structural importance (per PHASE0_PLAN).
/// We allocate bit 7 of the flags byte (0x80) so it does not collide with any current Phase 0 usage.
pub const FLAG_PRE_STABILIZE: u8 = 0x80;

/// Packed cell state (exactly 32 bits per STATE_PACKING.md).
/// Layout:
///   bits  0- 7 : material
///   bits  8-15 : flags (bit 7 = pre_stabilize reserved)
///   bits 16-23 : moisture / volume (0-255)
///   bits 24-31 : scent (0-255)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct Cell(u32);

impl Cell {
    #[inline]
    pub const fn new(material: u8, flags: u8, moisture: u8, scent: u8) -> Self {
        Self(
            (material as u32)
                | ((flags as u32) << 8)
                | ((moisture as u32) << 16)
                | ((scent as u32) << 24),
        )
    }

    #[inline]
    pub const fn air() -> Self {
        Self::new(MAT_AIR, 0, 0, 0)
    }

    #[inline]
    pub fn material(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    #[inline]
    pub fn flags(self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    #[inline]
    pub fn has_pre_stabilize(self) -> bool {
        (self.flags() & FLAG_PRE_STABILIZE) != 0
    }

    #[inline]
    pub fn moisture(self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    #[inline]
    pub fn scent(self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    /// Returns the exact mass contribution of this cell per RESOLVED_0.2 definition.
    /// Solid/mobile cells = 1.0, water volume and dirt moisture contribute fractional ( / 255.0 ).
    #[inline]
    pub fn mass_contribution(self) -> f32 {
        match self.material() {
            MAT_AIR => 0.0,
            MAT_WATER => (self.moisture() as f32) / 255.0,
            MAT_DIRT => 1.0 + (self.moisture() as f32) / 255.0, // 1.0 for the dirt + absorbed water
            _ => 1.0, // Sand, Stone, and any future mobile solids
        }
    }

    /// Set a flag bit (used during initialization or brush).
    #[inline]
    pub fn with_flag(mut self, flag: u8) -> Self {
        let f = self.flags() | flag;
        self.0 = (self.0 & !(0xFF << 8)) | ((f as u32) << 8);
        self
    }
}

/// SimParams uniform buffer layout (extended from the classic template per PHASE0_PLAN).
/// Must remain repr(C) and Pod for direct GPU upload via bytemuck.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SimParams {
    pub width: u32,
    pub height: u32,
    pub frame: u32,
    pub seed: u32,
    /// Which EventLedger slot the GPU should write to this frame (0 or 1).
    /// Host reads the opposite slot after submit.
    pub event_ledger_index: u32,
    /// Reserved for future expansion (alignment / padding).
    pub _pad: [u32; 3],
}

impl SimParams {
    pub fn new(width: u32, height: u32, seed: u32) -> Self {
        Self {
            width,
            height,
            frame: 0,
            seed,
            event_ledger_index: 0,
            _pad: [0; 3],
        }
    }

    /// Target resolution for Phase 0.0 demo (2048² gives excellent interactivity on DGX Spark;
    /// 4096² is the stretch goal once the loop is proven).
    pub const DEFAULT_WIDTH: u32 = 2048;
    pub const DEFAULT_HEIGHT: u32 = 2048;
}

/// Host-side owning structure for the simulation state.
/// In Step 3 this will grow to hold the actual wgpu::Buffer handles.
/// For now it provides the pure data model + helpers used by CPU reference and verification.
pub struct SimulationState {
    pub params: SimParams,
    /// CPU-side grid (row-major). Used for initialization, verification (0.4), and CPU ref (0.5).
    pub grid: Vec<Cell>,
    /// Double-buffered event ledgers (host copies of what the GPU wrote).
    /// After each submit the "readable" slot is populated from the GPU buffer.
    pub event_ledgers: [EventLedgerGpu; 2],
    /// Current readable ledger index (the one the host may safely drain).
    pub readable_ledger_index: u32,
}

impl SimulationState {
    pub fn new(width: u32, height: u32, seed: u32) -> Self {
        let len = (width * height) as usize;
        let params = SimParams::new(width, height, seed);

        Self {
            params,
            grid: vec![Cell::air(); len],
            event_ledgers: [
                EventLedgerGpu {
                    head: 0,
                    _pad: 0,
                    events: [WorldEvent {
                        header: 0,
                        pos: 0,
                        data0: 0,
                        data1: 0,
                    }; EVENT_CAPACITY],
                },
                EventLedgerGpu {
                    head: 0,
                    _pad: 0,
                    events: [WorldEvent {
                        header: 0,
                        pos: 0,
                        data0: 0,
                        data1: 0,
                    }; EVENT_CAPACITY],
                },
            ],
            readable_ledger_index: 0,
        }
    }

    #[inline]
    pub fn idx(&self, x: u32, y: u32) -> usize {
        (y * self.params.width + x) as usize
    }

    /// Exact total mass of the current grid using the 0.2 definition.
    /// This is the ground truth the FrameSummary deltas must match.
    pub fn exact_total_mass(&self) -> f32 {
        self.grid.iter().map(|c| c.mass_contribution()).sum()
    }

    /// Simple full-grid initialization (empty world). Brush and future world-seeding
    /// logic will replace or augment this in Phase 0.1+.
    pub fn initialize_empty(&mut self) {
        self.grid.fill(Cell::air());
        // Event ledgers are implicitly cleared by head=0 on first use.
    }

    /// Advances the frame counter and flips the event ledger write/read slots.
    pub fn begin_frame(&mut self) {
        self.params.frame = self.params.frame.wrapping_add(1);
        // Flip which slot the GPU will write; host will read the previous one.
        self.params.event_ledger_index = 1 - self.params.event_ledger_index;
        self.readable_ledger_index = 1 - self.params.event_ledger_index;
    }
}
