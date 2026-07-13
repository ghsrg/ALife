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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub contact_pairs_count: u32,
    pub contact_pressure_pre_total: f32,
    pub contact_pressure_post_total: f32,
    pub contact_pressure_max_over_tick: f32,
    pub contact_exchange_amount: f32,
    pub contact_exchange_pairs_count: u32,
    pub contact_exchange_rejections_no_capability: u32,
    pub contact_stimulus_generated_total: f32,
    pub contact_stimulus_readable_total: f32,
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
    pub reaction_matched_count: u32,
    pub reaction_executed_count: u32,
    pub reaction_rejected_count: u32,
    pub reaction_input_amount: f32,
    pub reaction_output_amount: f32,
    pub reaction_heat_generated: f32,
    pub reaction_energy_output: f32,
    pub reaction_accounting_error: f32,
    pub resource_diffused_amount: f32,
    pub resource_decay_amount: f32,
    pub fragment_created_amount: f32,
    pub fragment_converted_amount: f32,
    pub material_degradation_amount: f32,
    pub boundary_leakage_amount: f32,
    pub repair_success_count: u32,
    pub repair_rejection_count: u32,
    pub joint_count: u32,
    pub joint_created_count: u32,
    pub joint_creation_rejected_count: u32,
    pub joint_broken_count: u32,
    pub joint_resource_transfer_amount: f32,
    pub joint_resource_transfer_gross_amount: f32,
    pub joint_resource_transfer_net_amount: f32,
    pub joint_resource_source_final_amount: f32,
    pub joint_resource_target_final_amount: f32,
    pub joint_resource_backflow_amount: f32,
    pub joint_signal_generated_total: f32,
    pub joint_signal_readable_total: f32,
    pub joint_heat_transfer_amount: f32,
    pub joint_degradation_amount: f32,
    pub joint_mechanical_correction_amount: f32,
    pub integrated_matter_before: f32,
    pub integrated_matter_after: f32,
    pub integrated_matter_unclassified_loss: f32,
    pub integrated_matter_unclassified_gain: f32,
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
