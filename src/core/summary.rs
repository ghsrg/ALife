use crate::core::units::Tick;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurvivalResult {
    Stable,
    Fragile,
    Collapse,
    Invalid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapseReason {
    None,
    InvalidConfig,
    EnergyDepleted,
    MandatoryCostUnpaid,
    CapacityExceeded,
    HeatLimitExceeded,
    WasteLimitExceeded,
    MinimumViabilityMaterialsMissing,
    DeterminismMismatch,
    ViewerAuthorityViolation,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MetricsSummary {
    pub final_energy: f32,
    pub heat: f32,
    pub waste: f32,
    pub min_energy: f32,
    pub max_energy: f32,
    pub final_internal_resources: f32,
    pub final_external_resources: f32,
    pub final_used_capacity: f32,
    pub final_free_capacity: f32,
    pub growth_readiness: bool,
    pub overlap_resolved: f32,
    pub process_attempts: u32,
    pub process_rejections: u32,
    pub alive_cells_count: u32,
    pub dead_cells_count: u32,
    pub divisions_count: u32,
    pub births_count: u32,
    pub decomposed_cells_count: u32,
    pub sensory_input_accumulated: f32,
    pub repair_placeholder_available: bool,
}

use crate::core::process::{ProcessId, RejectionReason};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProcessDiagnostics {
    pub attempts_by_process: HashMap<ProcessId, u32>,
    pub rejections_by_process: HashMap<ProcessId, u32>,
    pub rejections_by_reason: HashMap<RejectionReason, u32>,
    pub tool_limited_mechanisms: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RunSummary {
    pub tick: Tick,
    pub config_hash: u64,
    pub survival_result: SurvivalResult,
    pub collapse_reason: CollapseReason,
    pub metrics: MetricsSummary,
    pub diagnostics: ProcessDiagnostics,
}
