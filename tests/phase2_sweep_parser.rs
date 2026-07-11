use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct RawScenarioPreset {
    pub world_size: Vec<f32>,
    pub initial_resources: Vec<f32>,
    pub decay_rate: f32,
    pub cell_position: Vec<f32>,
    pub cell_radius: f32,
    pub initial_energy: f32,
    pub initial_cell_resources: Option<f32>,
    pub energy_capacity: f32,
    pub mandatory_cost_per_tick: f32,
    pub passive_energy_income: f32,
    pub capacity_limit: f32,
    pub stress_energy_threshold: f32,
    pub dormancy_allowed: bool,
    pub dormant_mandatory_cost_modifier: f32,
    pub critical_capacity_overrun: f32,
    pub heat_dissipation_rate: f32,
    pub heat_warning_threshold: f32,
    pub heat_death_threshold: f32,
    pub waste_sink_rate: f32,
    pub waste_warning_threshold: f32,
    pub waste_death_threshold: f32,
    pub max_uptake_per_tick: Option<f32>,
    pub metabolism_resource_per_tick: Option<f32>,
    pub growth_enabled: bool,
    pub division_energy_cost: Option<f32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TestConfig {
    pub scenarios: HashMap<String, RawScenarioPreset>,
}

#[test]
#[allow(clippy::bool_assert_comparison)]
fn test_parse_scenario_presets() {
    let toml_str = r#"
[scenarios.test_preset]
world_size = [128.0, 128.0]
initial_resources = [5.0]
decay_rate = 0.05
cell_position = [64.0, 64.0]
cell_radius = 2.0
initial_energy = 40.0
initial_cell_resources = 20.0
energy_capacity = 80.0
mandatory_cost_per_tick = 1.5
passive_energy_income = 0.1
capacity_limit = 25.0
stress_energy_threshold = 8.0
dormancy_allowed = true
dormant_mandatory_cost_modifier = 0.15
critical_capacity_overrun = 4.0
heat_dissipation_rate = 0.25
heat_warning_threshold = 45.0
heat_death_threshold = 75.0
waste_sink_rate = 0.15
waste_warning_threshold = 12.0
waste_death_threshold = 22.0
max_uptake_per_tick = 0.25
metabolism_resource_per_tick = 0.25
growth_enabled = true
division_energy_cost = 2.0
"#;
    let config: TestConfig = toml::from_str(toml_str).unwrap();
    let preset = config.scenarios.get("test_preset").unwrap();
    assert_eq!(preset.decay_rate, 0.05);
    assert_eq!(preset.initial_cell_resources, Some(20.0));
    assert_eq!(preset.max_uptake_per_tick, Some(0.25));
    assert_eq!(preset.metabolism_resource_per_tick, Some(0.25));
    assert_eq!(preset.growth_enabled, true);
    assert_eq!(preset.division_energy_cost, Some(2.0));
}

#[test]
fn test_sweep_scenario_reference_mapping() {
    let toml_str = r#"
[run]
output_dir = "outputs"
seed = 100
ticks = 100

[cell]
radius = 1.0
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 1.0
passive_energy_income = 0.0
capacity_limit = 10.0
initial_metabolic_material = 1.0
initial_transport_material = 1.0
initial_boundary_material = 1.0
initial_structural_material = 1.0

[lifecycle]
stress_energy_threshold = 5.0
dormancy_allowed = true
dormant_mandatory_cost_modifier = 0.1
critical_capacity_overrun = 2.0

[resource_interaction]
energy_per_resource = 1.0
heat_per_resource = 0.0
waste_per_resource = 0.0
decay_rate = 0.0
default_resource_density = 1.0
default_max_uptake_per_tick = 1.0
default_metabolism_resource_per_tick = 1.0

[environment]
heat_dissipation_rate = 0.1
heat_warning_threshold = 10.0
heat_death_threshold = 20.0
waste_sink_rate = 0.1
waste_warning_threshold = 10.0
waste_death_threshold = 20.0

[scenarios.test_preset]
world_size = [128.0, 128.0]
initial_resources = [5.0]
decay_rate = 0.05
cell_position = [64.0, 64.0]
cell_radius = 2.0
initial_energy = 40.0
initial_cell_resources = 20.0
energy_capacity = 80.0
mandatory_cost_per_tick = 1.5
passive_energy_income = 0.1
capacity_limit = 25.0
stress_energy_threshold = 8.0
dormancy_allowed = true
dormant_mandatory_cost_modifier = 0.15
critical_capacity_overrun = 4.0
heat_dissipation_rate = 0.25
heat_warning_threshold = 45.0
heat_death_threshold = 75.0
waste_sink_rate = 0.15
waste_warning_threshold = 12.0
waste_death_threshold = 22.0
max_uptake_per_tick = 0.25
metabolism_resource_per_tick = 0.25
growth_enabled = true
division_energy_cost = 2.0

[[sweep]]
name = "test_sweep"
param = "resource_density"
from = 1.0
to = 10.0
steps = 5
scenario = "test_preset"
"#;

    // Deserialization check
    let config: alife::bin::sweep_analyzer::AnalyzerConfig = toml::from_str(toml_str).unwrap();
    let sweep = &config.sweep.as_ref().unwrap()[0];
    assert_eq!(sweep.scenario.as_deref(), Some("test_preset"));
    let preset = config
        .scenarios
        .as_ref()
        .unwrap()
        .get("test_preset")
        .unwrap();
    assert_eq!(preset.initial_cell_resources, Some(20.0));
    assert_eq!(preset.division_energy_cost, Some(2.0));
    assert_eq!(preset.max_uptake_per_tick, Some(0.25));
    assert_eq!(preset.metabolism_resource_per_tick, Some(0.25));
}
