use alife::runner::config_parser::RawScenarioConfig;
use std::collections::{HashMap, HashSet};

const PHASE2H_SCENARIOS: &[(&str, &str)] = &[
    ("joint_creation_viability", "joint_mechanical_strength"),
    ("joint_resource_channel", "joint_resource_transfer_rate"),
    ("joint_signal_delay", "joint_signal_conductivity"),
    ("joint_heat_channel", "joint_heat_conductivity"),
    ("joint_degradation_break", "joint_decay_rate"),
    ("joint_lifecycle_division", "joint_mechanical_strength"),
];

#[derive(Debug, serde::Deserialize)]
struct AnalyzerScenarioListConfig {
    sweep: Option<Vec<AnalyzerScenarioRef>>,
}

#[derive(Debug, serde::Deserialize)]
struct AnalyzerScenarioRef {
    scenario: String,
    param: String,
    steps: usize,
}

fn analyzer_sweeps(path: &str) -> HashMap<String, AnalyzerScenarioRef> {
    let contents = std::fs::read_to_string(path).unwrap();
    let config: AnalyzerScenarioListConfig = toml::from_str(&contents).unwrap();
    config
        .sweep
        .unwrap_or_default()
        .into_iter()
        .map(|entry| (entry.scenario.clone(), entry))
        .collect()
}

fn joint_fixture() -> String {
    r#"
scenario_id = "phase2h"
seed = 7
tick_count = 10

[world]
size = [32.0, 32.0]
[space]
spatial_grid_size = 8.0
[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [10.0]
optional_decay_rate = 0.0
[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = { nutrient_A = 3.0 }
initial_materials = { boundary = 2.0, transport = 2.0, structural = 2.0, sensory = 2.0 }
initial_energy = 5.0
energy_capacity = 10.0
mandatory_cost_per_tick = 0.0
capacity_limit = 20.0
[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.1
heat_warning_threshold = 10.0
heat_death_threshold = 20.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.1
waste_warning_threshold = 10.0
waste_death_threshold = 20.0
[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 2.0

[joints]
enabled = true
creation_distance_margin = 0.25
creation_material_cost = 1.5
creation_resource_cost = 2.0
creation_energy_cost = 0.5
upkeep_material_decay_per_tick = 0.02
break_damage_threshold = 1.0
max_joints_per_cell = 3
mechanical_strength = 0.4
resource_transfer_rate = 0.5
max_resource_transfer_per_tick = 1.0
signal_conductivity = 0.75
signal_decay = 0.2
heat_conductivity = 0.3
"#
    .to_string()
}

#[test]
fn parses_phase2h_joint_config_from_toml() {
    let config = RawScenarioConfig::parse(&joint_fixture()).unwrap();

    assert!(config.joints.enabled);
    assert_eq!(config.joints.creation_material_cost.raw(), 1.5);
    assert_eq!(config.joints.creation_resource_cost.raw(), 2.0);
    assert_eq!(config.joints.creation_energy_cost.raw(), 0.5);
    assert_eq!(config.joints.max_joints_per_cell, 3);
    assert_eq!(config.joints.signal_decay, 0.2);
}

#[test]
fn analyzer_configs_include_phase2h_full_and_smoke_scenarios() {
    let full = analyzer_sweeps("config/analyzer/sweep_analyzer.toml");
    let smoke = analyzer_sweeps("config/analyzer/sweep_analyzer_smoke.toml");
    let expected = PHASE2H_SCENARIOS
        .iter()
        .map(|(scenario, _)| (*scenario).to_string())
        .collect::<HashSet<_>>();

    assert!(expected.is_subset(&full.keys().cloned().collect()));
    assert!(expected.is_subset(&smoke.keys().cloned().collect()));

    for (scenario, param) in PHASE2H_SCENARIOS {
        let full_sweep = full.get(*scenario).unwrap();
        let smoke_sweep = smoke.get(*scenario).unwrap();
        assert_eq!(full_sweep.param, *param);
        assert_eq!(smoke_sweep.param, *param);
        assert!(
            smoke_sweep.steps <= full_sweep.steps,
            "smoke sweep must stay smaller than full sweep for {scenario}"
        );
    }
}
