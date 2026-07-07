use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct RawScenarioPreset {
    pub world_size: Vec<f32>,
    pub initial_resources: Vec<f32>,
    pub decay_rate: f32,
    pub cell_position: Vec<f32>,
    pub cell_radius: f32,
    pub initial_energy: f32,
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
    pub growth_enabled: bool,
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
growth_enabled = true
"#;
    let config: TestConfig = toml::from_str(toml_str).unwrap();
    let preset = config.scenarios.get("test_preset").unwrap();
    assert_eq!(preset.decay_rate, 0.05);
    assert_eq!(preset.growth_enabled, true);
}
