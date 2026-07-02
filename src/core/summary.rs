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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RunSummary {
    pub tick: Tick,
    pub config_hash: u64,
    pub survival_result: SurvivalResult,
    pub collapse_reason: CollapseReason,
    pub metrics: MetricsSummary,
}
