#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObserverSourceOwner {
    CoreCommittedSnapshot,
    CoreMetricsSummary,
    ObserverDerivedFeature,
    RunnerLiveFrame,
    AnalyzerAdapter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObserverConsumerSurface {
    ProjectionEnvelope,
    MetricsProjection,
    LiveFrameProjection,
    MechanismCoverage,
    BehaviorProfile,
    Classification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObserverReadiness {
    Current,
    Partial,
    Legacy,
    Planned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WarningDisposition {
    CanonicalObserverWarning,
    LegacyAnalyzerWarning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObserverFieldSpec {
    pub field_id: &'static str,
    pub source_owner: ObserverSourceOwner,
    pub consumer_surface: ObserverConsumerSurface,
    pub readiness: ObserverReadiness,
    pub provenance: &'static str,
    pub follow_up_plan_id: Option<&'static str>,
    pub mutable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoverageStatusSpec {
    pub status_id: &'static str,
    pub readiness: ObserverReadiness,
    pub mutable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WarningCodeSpec {
    pub code: &'static str,
    pub disposition: WarningDisposition,
    pub readiness: ObserverReadiness,
    pub mutable: bool,
}

const METRICS_FIELDS: &[ObserverFieldSpec] = &[
    metric("reaction_matched_count"),
    metric("reaction_executed_count"),
    metric("reaction_rejected_count"),
    metric("reaction_input_amount"),
    metric("reaction_output_amount"),
    metric("reaction_heat_generated"),
    metric("reaction_energy_output"),
    metric("reaction_accounting_error"),
    metric("resource_diffused_amount"),
    metric("resource_decay_amount"),
    metric("fragment_created_amount"),
    metric("fragment_converted_amount"),
    ObserverFieldSpec {
        field_id: "heat_peak_temperature",
        source_owner: ObserverSourceOwner::CoreCommittedSnapshot,
        consumer_surface: ObserverConsumerSurface::MetricsProjection,
        readiness: ObserverReadiness::Current,
        provenance: "MetricsSummary.heat",
        follow_up_plan_id: None,
        mutable: false,
    },
    metric("material_degradation_amount"),
    metric("boundary_leakage_amount"),
    metric("repair_success_count"),
    metric("repair_rejection_count"),
    metric("joint_count"),
    metric("joint_created_count"),
    metric("joint_creation_rejected_count"),
    metric("joint_broken_count"),
    metric("joint_resource_transfer_amount"),
    metric("joint_resource_transfer_gross_amount"),
    metric("joint_resource_transfer_net_amount"),
    metric("joint_resource_source_final_amount"),
    metric("joint_resource_target_final_amount"),
    metric("joint_resource_backflow_amount"),
    metric("joint_signal_generated_total"),
    metric("joint_signal_readable_total"),
    metric("joint_heat_transfer_amount"),
    metric("joint_degradation_amount"),
    metric("joint_mechanical_correction_amount"),
];

const RUNNER_FRAME_FIELDS: &[ObserverFieldSpec] = &[
    runner_meta("schema_version", ObserverReadiness::Current),
    runner_meta("committed_tick", ObserverReadiness::Current),
    runner_meta("projection_sequence", ObserverReadiness::Current),
    runner_meta("wall_clock_generated_at_ms", ObserverReadiness::Current),
    runner_meta("previous_committed_tick", ObserverReadiness::Current),
    runner_core("heat"),
    runner_core("waste"),
    runner_core("cells"),
    runner_core("cells.id"),
    runner_core("cells.x"),
    runner_core("cells.y"),
    runner_core("cells.radius"),
    runner_core("cells.energy"),
    runner_core("cells.lifecycle"),
];

const PROJECTION_ENVELOPE_FIELDS: &[ObserverFieldSpec] = &[
    envelope_field("projection_envelope.schema_version"),
    envelope_field("projection_envelope.projection_kind"),
    envelope_field("projection_envelope.run_id"),
    envelope_field("projection_envelope.tick"),
    envelope_field("projection_envelope.config_hash"),
    envelope_field("projection_envelope.engine_version"),
    envelope_field("projection_envelope.source"),
    envelope_field("projection_envelope.completeness"),
    envelope_field("projection_envelope.generated_at_unix_ms"),
];

const COVERAGE_STATUSES: &[CoverageStatusSpec] = &[
    status("covered"),
    status("partially_covered"),
    status("registered_but_disabled"),
    status("not_activated"),
    status("missing_scenario"),
    status("missing_metrics"),
    status("missing_balance_test"),
];

const WARNING_CODES: &[WarningCodeSpec] = &[
    canonical_warning("UNTESTED_REGISTERED_MECHANISM"),
    canonical_warning("DIRECT_STATE_MUTATION_OUTSIDE_PROCESS_PIPELINE"),
    canonical_warning("SCENARIO_MECHANISM_NOT_ACTIVATED"),
    canonical_warning("PARAMETER_HAS_NO_EFFECT"),
    canonical_warning("METRIC_MISSING"),
    canonical_warning("SCENARIO_COVERAGE_MISSING"),
    canonical_warning("MECHANIC_TRADEOFF_MISSING"),
    canonical_warning("CONFIG_TUNING_RECOMMENDED"),
    canonical_warning("IMPLEMENTATION_SUSPECTED"),
    legacy_warning("ENVIRONMENT_DOMINATED_RESULT"),
    legacy_warning("LOW_INFORMATION_SWEEP"),
    legacy_warning("LOCAL_INTERACTION_NOT_ACTIVATED"),
    legacy_warning("LOCAL_INTERACTION_EXCHANGE_FLAT"),
    legacy_warning("LOCAL_INTERACTION_STIMULUS_FLAT"),
    legacy_warning("BALANCE_ERROR"),
    legacy_warning("SCENARIO_TOO_EASY"),
    legacy_warning("TOOL_LIMITED_BOUNDARY_RETENTION"),
    legacy_warning("NOT_FULL_MECHANISM"),
    legacy_warning("TOOL_LIMITED_REPAIR"),
    legacy_warning("LOW_MATERIAL_SIGNAL"),
    legacy_warning("SCENARIO_TOO_HARD"),
    legacy_warning("PROFILE_EFFECT_TOO_SMALL"),
    legacy_warning("PROFILE_EFFECT_FLAT"),
];

pub fn observer_field_specs() -> &'static [ObserverFieldSpec] {
    const LEN: usize =
        METRICS_FIELDS.len() + RUNNER_FRAME_FIELDS.len() + PROJECTION_ENVELOPE_FIELDS.len();
    static SPECS: std::sync::OnceLock<[ObserverFieldSpec; LEN]> = std::sync::OnceLock::new();
    SPECS.get_or_init(|| {
        let mut specs = [METRICS_FIELDS[0]; LEN];
        let mut index = 0;
        while index < METRICS_FIELDS.len() {
            specs[index] = METRICS_FIELDS[index];
            index += 1;
        }
        let mut runner_index = 0;
        while runner_index < RUNNER_FRAME_FIELDS.len() {
            specs[index] = RUNNER_FRAME_FIELDS[runner_index];
            index += 1;
            runner_index += 1;
        }
        let mut envelope_index = 0;
        while envelope_index < PROJECTION_ENVELOPE_FIELDS.len() {
            specs[index] = PROJECTION_ENVELOPE_FIELDS[envelope_index];
            index += 1;
            envelope_index += 1;
        }
        specs
    })
}

pub fn observer_field_by_id(field_id: &str) -> Option<&'static ObserverFieldSpec> {
    observer_field_specs()
        .iter()
        .find(|spec| spec.field_id == field_id)
}

pub fn coverage_status_specs() -> &'static [CoverageStatusSpec] {
    COVERAGE_STATUSES
}

pub fn warning_code_specs() -> &'static [WarningCodeSpec] {
    WARNING_CODES
}

const fn metric(field_id: &'static str) -> ObserverFieldSpec {
    ObserverFieldSpec {
        field_id,
        source_owner: ObserverSourceOwner::CoreMetricsSummary,
        consumer_surface: ObserverConsumerSurface::MetricsProjection,
        readiness: ObserverReadiness::Current,
        provenance: "MetricsSummary",
        follow_up_plan_id: None,
        mutable: false,
    }
}

const fn runner_meta(field_id: &'static str, readiness: ObserverReadiness) -> ObserverFieldSpec {
    ObserverFieldSpec {
        field_id,
        source_owner: ObserverSourceOwner::RunnerLiveFrame,
        consumer_surface: ObserverConsumerSurface::LiveFrameProjection,
        readiness,
        provenance: "WorldFrameProjection metadata",
        follow_up_plan_id: Some("AL-004-S02"),
        mutable: false,
    }
}

const fn runner_core(field_id: &'static str) -> ObserverFieldSpec {
    ObserverFieldSpec {
        field_id,
        source_owner: ObserverSourceOwner::CoreCommittedSnapshot,
        consumer_surface: ObserverConsumerSurface::LiveFrameProjection,
        readiness: ObserverReadiness::Current,
        provenance: "CommittedSnapshot -> WorldFrameProjection",
        follow_up_plan_id: Some("AL-004-S02"),
        mutable: false,
    }
}

const fn envelope_field(field_id: &'static str) -> ObserverFieldSpec {
    ObserverFieldSpec {
        field_id,
        source_owner: ObserverSourceOwner::ObserverDerivedFeature,
        consumer_surface: ObserverConsumerSurface::ProjectionEnvelope,
        readiness: ObserverReadiness::Current,
        provenance: "ProjectionEnvelope metadata",
        follow_up_plan_id: Some("AL-004-S02"),
        mutable: false,
    }
}

const fn status(status_id: &'static str) -> CoverageStatusSpec {
    CoverageStatusSpec {
        status_id,
        readiness: ObserverReadiness::Current,
        mutable: false,
    }
}

const fn canonical_warning(code: &'static str) -> WarningCodeSpec {
    WarningCodeSpec {
        code,
        disposition: WarningDisposition::CanonicalObserverWarning,
        readiness: ObserverReadiness::Current,
        mutable: false,
    }
}

const fn legacy_warning(code: &'static str) -> WarningCodeSpec {
    WarningCodeSpec {
        code,
        disposition: WarningDisposition::LegacyAnalyzerWarning,
        readiness: ObserverReadiness::Legacy,
        mutable: false,
    }
}
