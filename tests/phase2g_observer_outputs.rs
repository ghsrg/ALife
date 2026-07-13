use alife::core::summary::MetricsSummary;
use alife::observer::projection::metrics_summary_features;

fn metrics() -> MetricsSummary {
    MetricsSummary {
        final_energy: 4.0,
        heat: 2.0,
        waste: 1.0,
        min_energy: 3.0,
        max_energy: 5.0,
        final_internal_resources: 0.5,
        final_external_resources: 9.0,
        final_used_capacity: 1.0,
        final_free_capacity: 2.0,
        growth_readiness: false,
        contact_pairs_count: 1,
        contact_pressure_pre_total: 0.2,
        contact_pressure_post_total: 0.1,
        contact_pressure_max_over_tick: 0.3,
        contact_exchange_amount: 0.4,
        contact_exchange_pairs_count: 2,
        contact_exchange_rejections_no_capability: 3,
        contact_stimulus_generated_total: 0.6,
        contact_stimulus_readable_total: 0.7,
        overlap_resolved: 0.8,
        process_attempts: 4,
        process_rejections: 1,
        alive_cells_count: 1,
        dead_cells_count: 0,
        divisions_count: 0,
        births_count: 0,
        decomposed_cells_count: 0,
        sensory_input_accumulated: 0.0,
        repair_placeholder_available: false,
        reaction_matched_count: 7,
        reaction_executed_count: 6,
        reaction_rejected_count: 1,
        reaction_input_amount: 2.0,
        reaction_output_amount: 1.5,
        reaction_heat_generated: 0.3,
        reaction_energy_output: 0.4,
        reaction_accounting_error: 0.0,
        resource_diffused_amount: 0.8,
        resource_decay_amount: 0.9,
        fragment_created_amount: 1.1,
        fragment_converted_amount: 0.2,
        material_degradation_amount: 1.3,
        boundary_leakage_amount: 0.4,
        repair_success_count: 5,
        repair_rejection_count: 6,
    }
}

#[test]
fn shared_observer_projection_exposes_all_phase2g_metrics() {
    let features = metrics_summary_features(&metrics());

    assert_eq!(features["reaction_matched_count"], 7.0);
    assert_eq!(features["reaction_executed_count"], 6.0);
    assert_eq!(features["reaction_rejected_count"], 1.0);
    assert_eq!(features["reaction_input_amount"], 2.0);
    assert_eq!(features["reaction_output_amount"], 1.5);
    assert_eq!(features["reaction_heat_generated"], 0.3);
    assert_eq!(features["reaction_energy_output"], 0.4);
    assert_eq!(features["reaction_accounting_error"], 0.0);
    assert_eq!(features["resource_diffused_amount"], 0.8);
    assert_eq!(features["resource_decay_amount"], 0.9);
    assert_eq!(features["fragment_created_amount"], 1.1);
    assert_eq!(features["fragment_converted_amount"], 0.2);
    assert_eq!(features["heat_peak_temperature"], 2.0);
    assert_eq!(features["material_degradation_amount"], 1.3);
    assert_eq!(features["boundary_leakage_amount"], 0.4);
    assert_eq!(features["repair_success_count"], 5.0);
    assert_eq!(features["repair_rejection_count"], 6.0);
}
