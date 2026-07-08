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
    }
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
