use alife::observer::{
    contract::{
        ObserverConsumerSurface, ObserverReadiness, ObserverSourceOwner, WarningDisposition,
        coverage_status_specs, observer_field_by_id, observer_field_specs, warning_code_specs,
    },
    projection::metrics_summary_features,
};
use std::collections::BTreeSet;

fn field_ids() -> BTreeSet<&'static str> {
    observer_field_specs()
        .iter()
        .map(|spec| spec.field_id)
        .collect()
}

#[test]
fn observer_contract_covers_metrics_summary_feature_fields() {
    let metrics = alife::core::summary::MetricsSummary {
        reaction_matched_count: 1,
        reaction_executed_count: 1,
        reaction_rejected_count: 1,
        reaction_input_amount: 1.0,
        reaction_output_amount: 1.0,
        reaction_heat_generated: 1.0,
        reaction_energy_output: 1.0,
        reaction_accounting_error: 0.0,
        resource_diffused_amount: 1.0,
        resource_decay_amount: 1.0,
        fragment_created_amount: 1.0,
        fragment_converted_amount: 1.0,
        material_degradation_amount: 1.0,
        boundary_leakage_amount: 1.0,
        repair_success_count: 1,
        repair_rejection_count: 1,
        joint_count: 1,
        joint_created_count: 1,
        joint_creation_rejected_count: 1,
        joint_broken_count: 1,
        joint_resource_transfer_amount: 1.0,
        joint_resource_transfer_gross_amount: 1.0,
        joint_resource_transfer_net_amount: 1.0,
        joint_resource_source_final_amount: 1.0,
        joint_resource_target_final_amount: 1.0,
        joint_resource_backflow_amount: 1.0,
        joint_signal_generated_total: 1.0,
        joint_signal_readable_total: 1.0,
        joint_heat_transfer_amount: 1.0,
        joint_degradation_amount: 1.0,
        joint_mechanical_correction_amount: 1.0,
        heat: 1.0,
        ..Default::default()
    };
    let contract_ids = field_ids();

    for feature_id in metrics_summary_features(&metrics).keys() {
        assert!(
            contract_ids.contains(feature_id.as_str()),
            "missing observer field contract for {feature_id}"
        );
    }

    let heat = observer_field_by_id("heat_peak_temperature").expect("heat field should exist");
    assert_eq!(
        heat.source_owner,
        ObserverSourceOwner::CoreCommittedSnapshot
    );
    assert_eq!(
        heat.consumer_surface,
        ObserverConsumerSurface::MetricsProjection
    );
}

#[test]
fn observer_contract_declares_allowed_coverage_statuses() {
    let statuses: BTreeSet<_> = coverage_status_specs()
        .iter()
        .map(|spec| spec.status_id)
        .collect();

    for expected in [
        "covered",
        "partially_covered",
        "registered_but_disabled",
        "not_activated",
        "missing_scenario",
        "missing_metrics",
        "missing_balance_test",
    ] {
        assert!(statuses.contains(expected), "missing status {expected}");
    }
}

#[test]
fn active_sweep_analyzer_warning_codes_are_registered_or_marked_legacy() {
    let warnings: BTreeSet<_> = warning_code_specs()
        .iter()
        .map(|spec| (spec.code, spec.disposition))
        .collect();

    for canonical in [
        "UNTESTED_REGISTERED_MECHANISM",
        "SCENARIO_MECHANISM_NOT_ACTIVATED",
        "PARAMETER_HAS_NO_EFFECT",
        "METRIC_MISSING",
        "SCENARIO_COVERAGE_MISSING",
        "MECHANIC_TRADEOFF_MISSING",
        "CONFIG_TUNING_RECOMMENDED",
        "IMPLEMENTATION_SUSPECTED",
    ] {
        assert!(
            warnings.contains(&(canonical, WarningDisposition::CanonicalObserverWarning)),
            "missing canonical warning {canonical}"
        );
    }

    for legacy in [
        "ENVIRONMENT_DOMINATED_RESULT",
        "LOW_INFORMATION_SWEEP",
        "LOCAL_INTERACTION_NOT_ACTIVATED",
        "LOCAL_INTERACTION_EXCHANGE_FLAT",
        "LOCAL_INTERACTION_STIMULUS_FLAT",
        "BALANCE_ERROR",
        "SCENARIO_TOO_EASY",
        "TOOL_LIMITED_BOUNDARY_RETENTION",
        "NOT_FULL_MECHANISM",
        "TOOL_LIMITED_REPAIR",
        "LOW_MATERIAL_SIGNAL",
        "SCENARIO_TOO_HARD",
        "PROFILE_EFFECT_TOO_SMALL",
        "PROFILE_EFFECT_FLAT",
    ] {
        assert!(
            warnings.contains(&(legacy, WarningDisposition::LegacyAnalyzerWarning)),
            "missing legacy warning disposition for {legacy}"
        );
    }
}

#[test]
fn runner_world_frame_fields_have_observer_contract_mapping() {
    for field in [
        "schema_version",
        "committed_tick",
        "projection_sequence",
        "wall_clock_generated_at_ms",
        "previous_committed_tick",
        "heat",
        "waste",
        "cells",
        "cells.id",
        "cells.x",
        "cells.y",
        "cells.radius",
        "cells.energy",
        "cells.lifecycle",
    ] {
        let spec = observer_field_by_id(field).unwrap_or_else(|| {
            panic!("missing runner frame field contract for {field}");
        });
        assert!(
            matches!(
                spec.source_owner,
                ObserverSourceOwner::RunnerLiveFrame | ObserverSourceOwner::CoreCommittedSnapshot
            ),
            "unexpected source owner for {field}: {:?}",
            spec.source_owner
        );
        assert_eq!(
            spec.consumer_surface,
            ObserverConsumerSurface::LiveFrameProjection
        );
    }

    let sequence = observer_field_by_id("projection_sequence").unwrap();
    assert_eq!(sequence.readiness, ObserverReadiness::Current);
    assert_eq!(sequence.follow_up_plan_id, Some("AL-004-S02"));
}

#[test]
fn observer_contract_is_static_and_read_only() {
    assert!(
        observer_field_specs()
            .iter()
            .all(|spec| spec.mutable == false)
    );
    assert!(
        warning_code_specs()
            .iter()
            .all(|spec| spec.mutable == false)
    );
}

#[test]
fn observer_contract_does_not_enter_genome_runtime_inputs() {
    let source = std::fs::read_to_string("src/core/genome.rs").expect("genome source should read");
    assert!(
        !source.contains("observer::contract"),
        "Genome Runtime must not depend on Observer contract"
    );
}

#[test]
fn observer_contract_does_not_change_runner_frame_projection_hash() {
    let source =
        std::fs::read_to_string("src/runner/projections.rs").expect("runner projection source");
    assert!(
        !source.contains("observer::contract"),
        "Runner frame projection must not derive behavior from Observer contract"
    );
}
