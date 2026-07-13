use alife::bin::sweep_analyzer::{SimResult, detect_warnings};

fn mock_result(collapsed: bool, final_energy: f32, dormant_ticks: u32) -> SimResult {
    SimResult {
        collapsed,
        collapse_tick: if collapsed { Some(10) } else { None },
        dormant_ticks,
        active_ticks: 100 - dormant_ticks,
        stressed_ticks: 0,
        min_energy: final_energy * 0.9,
        max_energy: final_energy * 1.1,
        final_energy,
        mean_energy: final_energy,
        initial_energy: 50.0,
        death_reason: if collapsed {
            "collapsed".to_string()
        } else {
            "none".to_string()
        },
        energy_produced: 0.0,
        passive_energy_received: 0.0,
        energy_spent_upkeep: 0.0,
        energy_spent_dormant_upkeep: 0.0,
        energy_spent_movement: 0.0,
        energy_spent_growth: 0.0,
        energy_spent_repair: 0.0,
        energy_spent_division: 0.0,
        initial_world_resource: 0.0,
        final_world_resource: 0.0,
        resource_regenerated: 0.0,
        resource_absorbed: 0.0,
        resource_metabolized: 0.0,
        internal_resource_final: 0.0,
        resource_released: 0.0,
        resource_explicit_sink: 0.0,
        resource_balance_error: 0.0,
        energy_balance_error: 0.0,
        dormancy_enter_count: if dormant_ticks > 0 { 1 } else { 0 },
        dormancy_exit_count: 0,
        ticks_executed: 100,
        total_resource_consumed: 0.0,
        metabolism_count: 0,
        potential_role: "unknown".to_string(),
        observed_role: "unknown".to_string(),
        behavior_profile: "unknown".to_string(),
        ticks_per_second: 0.0,
        bhv_res: None,
        explicit_energy_loss: 0.0,
        death_cleanup_loss_energy: 0.0,
        death_cleanup_loss_resources: 0.0,
        clamping_loss: 0.0,
        unpaid_mandatory_cost: 0.0,
        resource_decay: 0.0,
        resource_sink: 0.0,
        numerical_error_energy: 0.0,
        numerical_error_resources: 0.0,
        unclassified_loss_energy: 0.0,
        unclassified_loss_resources: 0.0,
        divisions_count: 0,
        births_count: 0,
        dead_cells_count: 0,
        decomposing_cells_count: 0,
        decomposed_cells_count: 0,
        division_attempts: 0,
        division_successes: 0,
        division_rejections: 0,
        decomposition_released_resources: 0.0,
        death_tick: 0,
        first_decomposition_tick: 0,
        first_decomposed_tick: 0,
        decomposition_ticks: 0,
        decomposition_released_resources_per_tick: 0.0,
        time_to_decomposed: 0,
        remaining_dead_cell_resources: 0.0,
        remaining_dead_cell_materials: 0.0,
        contact_pairs_count: 0,
        contact_pressure_pre_total: 0.0,
        contact_pressure_post_total: 0.0,
        contact_pressure_max_over_tick: 0.0,
        contact_exchange_amount: 0.0,
        contact_exchange_pairs_count: 0,
        contact_exchange_rejections_no_capability: 0,
        contact_stimulus_generated_total: 0.0,
        contact_stimulus_readable_total: 0.0,
        reaction_matched_count: 0,
        reaction_executed_count: 0,
        reaction_rejected_count: 0,
        reaction_input_amount: 0.0,
        reaction_output_amount: 0.0,
        reaction_heat_generated: 0.0,
        reaction_energy_output: 0.0,
        reaction_accounting_error: 0.0,
        resource_diffused_amount: 0.0,
        resource_decay_amount: 0.0,
        fragment_created_amount: 0.0,
        fragment_converted_amount: 0.0,
        heat_peak_temperature: 0.0,
        material_degradation_amount: 0.0,
        boundary_leakage_amount: 0.0,
        repair_success_count: 0,
        repair_rejection_count: 0,
    }
}

fn mock_result_custom(
    collapsed: bool,
    final_energy: f32,
    dormant_ticks: u32,
    ticks_executed: u32,
    death_reason: String,
) -> SimResult {
    SimResult {
        collapsed,
        collapse_tick: if collapsed {
            Some(ticks_executed)
        } else {
            None
        },
        dormant_ticks,
        active_ticks: ticks_executed.saturating_sub(dormant_ticks),
        stressed_ticks: 0,
        min_energy: final_energy * 0.9,
        max_energy: final_energy * 1.1,
        final_energy,
        mean_energy: final_energy,
        initial_energy: 50.0,
        death_reason,
        energy_produced: 0.0,
        passive_energy_received: 0.0,
        energy_spent_upkeep: 0.0,
        energy_spent_dormant_upkeep: 0.0,
        energy_spent_movement: 0.0,
        energy_spent_growth: 0.0,
        energy_spent_repair: 0.0,
        energy_spent_division: 0.0,
        initial_world_resource: 0.0,
        final_world_resource: 0.0,
        resource_regenerated: 0.0,
        resource_absorbed: 0.0,
        resource_metabolized: 0.0,
        internal_resource_final: 0.0,
        resource_released: 0.0,
        resource_explicit_sink: 0.0,
        resource_balance_error: 0.0,
        energy_balance_error: 0.0,
        dormancy_enter_count: if dormant_ticks > 0 { 1 } else { 0 },
        dormancy_exit_count: 0,
        ticks_executed,
        total_resource_consumed: 0.0,
        metabolism_count: 0,
        potential_role: "unknown".to_string(),
        observed_role: "unknown".to_string(),
        behavior_profile: "unknown".to_string(),
        ticks_per_second: 0.0,
        bhv_res: None,
        explicit_energy_loss: 0.0,
        death_cleanup_loss_energy: 0.0,
        death_cleanup_loss_resources: 0.0,
        clamping_loss: 0.0,
        unpaid_mandatory_cost: 0.0,
        resource_decay: 0.0,
        resource_sink: 0.0,
        numerical_error_energy: 0.0,
        numerical_error_resources: 0.0,
        unclassified_loss_energy: 0.0,
        unclassified_loss_resources: 0.0,
        divisions_count: 0,
        births_count: 0,
        dead_cells_count: 0,
        decomposing_cells_count: 0,
        decomposed_cells_count: 0,
        division_attempts: 0,
        division_successes: 0,
        division_rejections: 0,
        decomposition_released_resources: 0.0,
        death_tick: 0,
        first_decomposition_tick: 0,
        first_decomposed_tick: 0,
        decomposition_ticks: 0,
        decomposition_released_resources_per_tick: 0.0,
        time_to_decomposed: 0,
        remaining_dead_cell_resources: 0.0,
        remaining_dead_cell_materials: 0.0,
        contact_pairs_count: 0,
        contact_pressure_pre_total: 0.0,
        contact_pressure_post_total: 0.0,
        contact_pressure_max_over_tick: 0.0,
        contact_exchange_amount: 0.0,
        contact_exchange_pairs_count: 0,
        contact_exchange_rejections_no_capability: 0,
        contact_stimulus_generated_total: 0.0,
        contact_stimulus_readable_total: 0.0,
        reaction_matched_count: 0,
        reaction_executed_count: 0,
        reaction_rejected_count: 0,
        reaction_input_amount: 0.0,
        reaction_output_amount: 0.0,
        reaction_heat_generated: 0.0,
        reaction_energy_output: 0.0,
        reaction_accounting_error: 0.0,
        resource_diffused_amount: 0.0,
        resource_decay_amount: 0.0,
        fragment_created_amount: 0.0,
        fragment_converted_amount: 0.0,
        heat_peak_temperature: 0.0,
        material_degradation_amount: 0.0,
        boundary_leakage_amount: 0.0,
        repair_success_count: 0,
        repair_rejection_count: 0,
    }
}

fn mock_decomposition_result(
    time_to_decomposed: u32,
    release_per_tick: f32,
    remaining_resources: f32,
    remaining_materials: f32,
) -> SimResult {
    SimResult {
        collapsed: true,
        collapse_tick: Some(5),
        dormant_ticks: 0,
        active_ticks: 0,
        stressed_ticks: 0,
        min_energy: 0.0,
        max_energy: 1.0,
        final_energy: 0.0,
        mean_energy: 0.0,
        initial_energy: 1.0,
        death_reason: "EnergyDepleted".to_string(),
        energy_produced: 0.0,
        passive_energy_received: 0.0,
        energy_spent_upkeep: 0.0,
        energy_spent_dormant_upkeep: 0.0,
        energy_spent_movement: 0.0,
        energy_spent_growth: 0.0,
        energy_spent_repair: 0.0,
        energy_spent_division: 0.0,
        initial_world_resource: 0.0,
        final_world_resource: 0.0,
        resource_regenerated: 0.0,
        resource_absorbed: 0.0,
        resource_metabolized: 0.0,
        internal_resource_final: 0.0,
        resource_released: 0.0,
        resource_explicit_sink: 0.0,
        resource_balance_error: 0.0,
        energy_balance_error: 0.0,
        dormancy_enter_count: 0,
        dormancy_exit_count: 0,
        ticks_executed: 50,
        total_resource_consumed: 0.0,
        metabolism_count: 0,
        potential_role: "unknown".to_string(),
        observed_role: "unknown".to_string(),
        behavior_profile: "unknown".to_string(),
        ticks_per_second: 0.0,
        bhv_res: None,
        explicit_energy_loss: 0.0,
        death_cleanup_loss_energy: 0.0,
        death_cleanup_loss_resources: 0.0,
        clamping_loss: 0.0,
        unpaid_mandatory_cost: 0.0,
        resource_decay: 0.0,
        resource_sink: 0.0,
        numerical_error_energy: 0.0,
        numerical_error_resources: 0.0,
        unclassified_loss_energy: 0.0,
        unclassified_loss_resources: 0.0,
        divisions_count: 0,
        births_count: 0,
        dead_cells_count: 1,
        decomposing_cells_count: 0,
        decomposed_cells_count: 1,
        division_attempts: 0,
        division_successes: 0,
        division_rejections: 0,
        decomposition_released_resources: 24.0,
        death_tick: 5,
        first_decomposition_tick: 6,
        first_decomposed_tick: 5 + time_to_decomposed,
        decomposition_ticks: time_to_decomposed,
        decomposition_released_resources_per_tick: release_per_tick,
        time_to_decomposed,
        remaining_dead_cell_resources: remaining_resources,
        remaining_dead_cell_materials: remaining_materials,
        contact_pairs_count: 0,
        contact_pressure_pre_total: 0.0,
        contact_pressure_post_total: 0.0,
        contact_pressure_max_over_tick: 0.0,
        contact_exchange_amount: 0.0,
        contact_exchange_pairs_count: 0,
        contact_exchange_rejections_no_capability: 0,
        contact_stimulus_generated_total: 0.0,
        contact_stimulus_readable_total: 0.0,
        reaction_matched_count: 0,
        reaction_executed_count: 0,
        reaction_rejected_count: 0,
        reaction_input_amount: 0.0,
        reaction_output_amount: 0.0,
        reaction_heat_generated: 0.0,
        reaction_energy_output: 0.0,
        reaction_accounting_error: 0.0,
        resource_diffused_amount: 0.0,
        resource_decay_amount: 0.0,
        fragment_created_amount: 0.0,
        fragment_converted_amount: 0.0,
        heat_peak_temperature: 0.0,
        material_degradation_amount: 0.0,
        boundary_leakage_amount: 0.0,
        repair_success_count: 0,
        repair_rejection_count: 0,
    }
}

fn mock_division_result(
    collapsed: bool,
    final_energy: f32,
    divisions: u32,
    births: u32,
    division_energy_spent: f32,
) -> SimResult {
    let mut result = mock_result(collapsed, final_energy, 0);
    result.divisions_count = divisions;
    result.births_count = births;
    result.division_attempts = divisions;
    result.division_successes = divisions;
    result.energy_spent_division = division_energy_spent;
    result
}

#[test]
fn test_detects_low_information_sweep() {
    let results = vec![
        mock_result(false, 50.0, 0),
        mock_result(false, 50.0, 0),
        mock_result(false, 50.0, 0),
    ];
    let warnings = detect_warnings(&results, "some_scenario");
    assert!(warnings.contains(&"LOW_INFORMATION_SWEEP".to_string()));
}

#[test]
fn test_detects_mechanism_not_activated() {
    let results = vec![
        mock_result(false, 40.0, 0),
        mock_result(false, 50.0, 0),
        mock_result(false, 60.0, 0),
    ];
    let warnings = detect_warnings(&results, "dormancy_survival");
    assert!(warnings.contains(&"SCENARIO_MECHANISM_NOT_ACTIVATED".to_string()));
}

#[test]
fn test_scenario_finite_resource_viability_ticks_responsive() {
    // Range of survival_ticks is significant (min = 10, max = 80 -> spread = 70 > 5)
    // LOW_INFORMATION_SWEEP should NOT be generated.
    let results = vec![
        mock_result_custom(true, 0.0, 0, 10, "collapsed".to_string()),
        mock_result_custom(true, 0.0, 0, 50, "collapsed".to_string()),
        mock_result_custom(true, 0.0, 0, 80, "collapsed".to_string()),
    ];
    let warnings = detect_warnings(&results, "finite_resource_viability");
    assert!(!warnings.contains(&"LOW_INFORMATION_SWEEP".to_string()));

    // If spread was not significant (e.g. min = 10, max = 12 -> spread = 2 <= 5)
    // LOW_INFORMATION_SWEEP should be generated.
    let results_low_spread = vec![
        mock_result_custom(true, 0.0, 0, 10, "collapsed".to_string()),
        mock_result_custom(true, 0.0, 0, 11, "collapsed".to_string()),
        mock_result_custom(true, 0.0, 0, 12, "collapsed".to_string()),
    ];
    let warnings_low_spread = detect_warnings(&results_low_spread, "finite_resource_viability");
    assert!(warnings_low_spread.contains(&"LOW_INFORMATION_SWEEP".to_string()));
}

#[test]
fn test_scenario_decomposition_viability_rate_responsive() {
    let results = vec![
        mock_decomposition_result(24, 1.0, 0.0, 0.0),
        mock_decomposition_result(5, 4.8, 0.0, 0.0),
    ];

    let warnings = detect_warnings(&results, "decomposition_viability");

    assert!(!warnings.contains(&"LOW_INFORMATION_SWEEP".to_string()));
}

#[test]
fn test_scenario_division_viability_clean_activation_is_informative() {
    let results = vec![
        mock_division_result(false, 58.0, 2, 2, 4.0),
        mock_division_result(false, 62.0, 5, 5, 10.0),
        mock_division_result(true, 44.0, 1, 1, 2.0),
    ];

    let warnings = detect_warnings(&results, "division_viability");

    assert!(!warnings.contains(&"LOW_INFORMATION_SWEEP".to_string()));
}

#[test]
fn test_scenario_dormancy_survival_no_activation() {
    // If dormant_ticks is 0 across all runs, SCENARIO_MECHANISM_NOT_ACTIVATED must be returned.
    let results = vec![
        mock_result_custom(false, 50.0, 0, 100, "none".to_string()),
        mock_result_custom(false, 50.0, 0, 100, "none".to_string()),
    ];
    let warnings = detect_warnings(&results, "dormancy_survival");
    assert!(warnings.contains(&"SCENARIO_MECHANISM_NOT_ACTIVATED".to_string()));

    // If at least one has dormant_ticks > 0, SCENARIO_MECHANISM_NOT_ACTIVATED must NOT be returned.
    let results_active = vec![
        mock_result_custom(false, 50.0, 10, 100, "none".to_string()),
        mock_result_custom(false, 50.0, 0, 100, "none".to_string()),
    ];
    let warnings_active = detect_warnings(&results_active, "dormancy_survival");
    assert!(!warnings_active.contains(&"SCENARIO_MECHANISM_NOT_ACTIVATED".to_string()));
}

#[test]
fn test_scenario_resource_abundance_environment_dominated() {
    // If all runs collapse, and any has death_reason containing "Heat" or "Waste",
    // ENVIRONMENT_DOMINATED_RESULT should be returned.
    let results = vec![
        mock_result_custom(true, 0.0, 0, 50, "HeatLimitExceeded".to_string()),
        mock_result_custom(true, 0.0, 0, 60, "collapsed".to_string()),
    ];
    let warnings = detect_warnings(&results, "resource_abundance");
    assert!(warnings.contains(&"ENVIRONMENT_DOMINATED_RESULT".to_string()));

    // Same but with Waste
    let results_waste = vec![
        mock_result_custom(true, 0.0, 0, 50, "WasteLimitExceeded".to_string()),
        mock_result_custom(true, 0.0, 0, 60, "collapsed".to_string()),
    ];
    let warnings_waste = detect_warnings(&results_waste, "resource_abundance");
    assert!(warnings_waste.contains(&"ENVIRONMENT_DOMINATED_RESULT".to_string()));

    // If not all collapsed, or no heat/waste death reason, it should NOT return ENVIRONMENT_DOMINATED_RESULT
    let results_not_all_collapsed = vec![
        mock_result_custom(true, 0.0, 0, 50, "HeatLimitExceeded".to_string()),
        mock_result_custom(false, 50.0, 0, 100, "none".to_string()),
    ];
    let warnings_not_all = detect_warnings(&results_not_all_collapsed, "resource_abundance");
    assert!(!warnings_not_all.contains(&"ENVIRONMENT_DOMINATED_RESULT".to_string()));
}

#[test]
fn test_scenario_steady_resource_flow_low_stable_density() {
    // If there are many runs (e.g. 20) and only <= 5% of them are stable (e.g. 1 stable, 19 collapsed),
    // it must generate LOW_INFORMATION_SWEEP and the warning list should include recommendation to narrow parameter range.
    let mut results = Vec::new();
    results.push(mock_result_custom(false, 50.0, 0, 100, "none".to_string())); // 1 stable (5%)
    for _ in 0..19 {
        results.push(mock_result_custom(
            true,
            0.0,
            0,
            20,
            "collapsed".to_string(),
        )); // 19 collapsed
    }
    let warnings = detect_warnings(&results, "steady_resource_flow");
    assert!(warnings.contains(&"LOW_INFORMATION_SWEEP".to_string()));
    assert!(
        warnings
            .iter()
            .any(|w| w.contains("Recommend narrowing the parameter range")
                || w.contains("RECOMMEND_NARROW_RANGE"))
    );
}

#[test]
fn test_scenario_local_interaction_viability_rate_responsive() {
    let mut slow = mock_result(false, 10.0, 0);
    slow.contact_pairs_count = 1;
    slow.contact_pressure_pre_total = 1.0;
    slow.contact_pressure_max_over_tick = 1.0;
    slow.contact_exchange_amount = 0.0;
    slow.contact_stimulus_readable_total = 0.5;

    let mut fast = slow.clone();
    fast.contact_exchange_amount = 2.0;

    let warnings = detect_warnings(&[slow, fast], "local_interaction_viability");
    assert!(!warnings.contains(&"LOW_INFORMATION_SWEEP".to_string()));
    assert!(!warnings.contains(&"LOCAL_INTERACTION_NOT_ACTIVATED".to_string()));
}

#[test]
fn test_scenario_local_interaction_viability_flags_flat_exchange() {
    let mut a = mock_result(false, 10.0, 0);
    a.contact_pairs_count = 1;
    a.contact_pressure_pre_total = 1.0;
    a.contact_pressure_max_over_tick = 1.0;
    a.contact_exchange_amount = 0.0;

    let mut b = a.clone();
    b.contact_exchange_amount = 0.0;

    let warnings = detect_warnings(&[a, b], "local_interaction_viability");
    assert!(warnings.contains(&"LOCAL_INTERACTION_EXCHANGE_FLAT".to_string()));
}

#[test]
fn test_scenario_local_interaction_viability_flags_no_contact() {
    let mut a = mock_result(false, 10.0, 0);
    a.contact_pairs_count = 0;
    a.contact_pressure_pre_total = 0.0;
    a.contact_pressure_max_over_tick = 0.0;

    let mut b = a.clone();
    b.contact_exchange_amount = 1.0;

    let warnings = detect_warnings(&[a, b], "local_interaction_viability");
    assert!(warnings.contains(&"LOCAL_INTERACTION_NOT_ACTIVATED".to_string()));
}
