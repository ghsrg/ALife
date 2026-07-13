use alife::core::ids::ResourceTypeId;
use alife::core::units::ResourceAmount;
use alife::runner::config_parser::{ParseError, RawScenarioConfig};
use std::collections::{HashMap, HashSet};

const PHASE2G_SCENARIOS: &[(&str, &str)] = &[
    ("resource_type_decay_diffusion", "nutrient_decay_rate"),
    ("material_type_degradation", "material_decay_rate"),
    ("passive_reaction_viability", "passive_reaction_rate"),
    (
        "controlled_reaction_feasibility",
        "controlled_reaction_rate",
    ),
    (
        "fragment_decomposition_conversion",
        "decomposition_materials_per_tick",
    ),
    ("local_heat_degradation", "reaction_heat_output"),
    ("boundary_retention_leakage", "contact_exchange_rate"),
    ("repair_viability", "repair_amount_per_tick"),
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

fn fixture() -> String {
    r#"
scenario_id = "phase2g"
seed = 7
tick_count = 10

[world]
size = [32.0, 32.0]
[space]
spatial_grid_size = 8.0
[resources]
resource_type_ids = ["nutrient_A", "waste_A"]
initial_distribution = [10.0, 0.0]
optional_decay_rate = 0.0
[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = { nutrient_A = 1.0 }
initial_materials = { boundary = 1.0, structural = 1.0 }
initial_energy = 5.0
energy_capacity = 10.0
mandatory_cost_per_tick = 1.0
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

[chemistry.resources.nutrient_A]
volume = 1.0
diffusion_rate = 0.2
energy_value = 2.0
decay_rate = 0.01
reactivity_profile = "reactive"
permeability = "passive"
tags = ["energy_source", "dissolved"]
[chemistry.resources.waste_A]
volume = 1.0
diffusion_rate = 0.1
energy_value = 0.0
decay_rate = 0.02
reactivity_profile = "stable"
permeability = "blocked"
tags = ["waste"]

[chemistry.materials.boundary_polymer_A]
volume = 1.0
stability = 0.8
strength = 0.7
permeability = 0.5
energy_capacity = 0.0
decay_rate = 0.01
repair_resource = "nutrient_A"
repair_amount = 0.25
[chemistry.materials.structural_polymer_A]
volume = 1.0
stability = 0.9
strength = 0.8
permeability = 0.0
energy_capacity = 0.0
decay_rate = 0.02
repair_resource = "nutrient_A"
repair_amount = 0.5

[chemistry.reactions.passive_decay]
mode = "passive"
inputs = { nutrient_A = 1.0 }
outputs = { waste_A = 1.0 }
configured_sink_amount = 0.0
energy_output = 0.0
heat_output = 0.0
rate = 0.1
probability = 1.0
accounting_destination = "waste_A"
[chemistry.reactions.controlled_conversion]
mode = "controlled"
process_id = "energy_conversion"
inputs = { nutrient_A = 1.0 }
required_materials = { boundary_polymer_A = 0.2 }
outputs = { waste_A = 0.5 }
configured_sink_amount = 0.5
energy_output = 0.8
heat_output = 0.1
rate = 0.2
probability = 0.5
accounting_destination = "waste_A"

[chemistry.heat]
capacity = 10.0
dissipation_rate = 0.2
warning_threshold = 8.0
death_threshold = 10.0
[chemistry.boundary]
default_permeability = "blocked"
retention_rate = 0.9
[chemistry.repair]
enabled = true
energy_cost = 0.5
max_amount_per_tick = 1.0
"#
    .to_string()
}

#[test]
fn parses_typed_chemistry_and_normalizes_declarations() {
    let config = RawScenarioConfig::parse(&fixture()).unwrap();
    assert_eq!(config.chemistry.resources.len(), 2);
    assert_eq!(config.chemistry.materials.len(), 2);
    assert_eq!(config.chemistry.reactions.len(), 2);
    assert_eq!(
        config.chemistry.reactions[0].accounting_destination,
        "waste_A"
    );
    assert_eq!(config.chemistry.heat.warning_threshold, 8.0);
    assert!(config.chemistry.boundary.retention_rate >= 0.0);
    assert!(config.chemistry.repair.enabled);
    assert_eq!(
        config.initial_typed_resources[0],
        vec![(
            ResourceTypeId::from_raw(0),
            ResourceAmount::new(1.0).unwrap()
        )]
    );
}

#[test]
fn rejects_unknown_ids_and_catalysts() {
    for (needle, replacement) in [
        ("nutrient_A = 1.0 }\noutputs", "unknown = 1.0 }\noutputs"),
        ("boundary_polymer_A = 0.2", "unknown_material = 0.2"),
    ] {
        let error =
            RawScenarioConfig::parse(&fixture().replacen(needle, replacement, 1)).unwrap_err();
        assert!(matches!(error, ParseError::ValidationError(_)));
    }
}

#[test]
fn rejects_duplicate_ids_and_invalid_numeric_values() {
    let duplicate = fixture().replace(
        "resource_type_ids = [\"nutrient_A\", \"waste_A\"]",
        "resource_type_ids = [\"nutrient_A\", \"nutrient_A\"]",
    );
    assert!(matches!(
        RawScenarioConfig::parse(&duplicate),
        Err(ParseError::ValidationError(_))
    ));

    for value in ["-0.1", "nan"] {
        let invalid = fixture().replacen("rate = 0.1", &format!("rate = {value}"), 1);
        assert!(matches!(
            RawScenarioConfig::parse(&invalid),
            Err(ParseError::ValidationError(_))
        ));
    }
    let probability = fixture().replace("probability = 0.5", "probability = 1.1");
    assert!(matches!(
        RawScenarioConfig::parse(&probability),
        Err(ParseError::ValidationError(_))
    ));
}

#[test]
fn rejects_reactions_without_accounting_destination() {
    let invalid = fixture().replace(
        "accounting_destination = \"waste_A\"",
        "accounting_destination = \"\"",
    );
    assert!(matches!(
        RawScenarioConfig::parse(&invalid),
        Err(ParseError::ValidationError(_))
    ));
}

#[test]
fn rejects_controlled_reaction_without_registered_process_binding() {
    let error =
        RawScenarioConfig::parse(&fixture().replace("process_id = \"energy_conversion\"\n", ""))
            .unwrap_err();

    assert!(matches!(error, ParseError::ValidationError(_)));
}

#[test]
fn rejects_reaction_with_unaccounted_input_matter() {
    let error = RawScenarioConfig::parse(&fixture().replace(
        "configured_sink_amount = 0.5",
        "configured_sink_amount = 0.0",
    ))
    .unwrap_err();

    assert!(matches!(error, ParseError::ValidationError(_)));
}

#[test]
fn phase2g_analyzer_configs_cover_full_and_smoke_scenarios() {
    let full = analyzer_sweeps("config/analyzer/sweep_analyzer.toml");
    let smoke = analyzer_sweeps("config/analyzer/sweep_analyzer_smoke.toml");
    let expected = PHASE2G_SCENARIOS
        .iter()
        .map(|(scenario, _)| (*scenario).to_string())
        .collect::<HashSet<_>>();

    assert!(expected.is_subset(&full.keys().cloned().collect()));
    assert!(expected.is_subset(&smoke.keys().cloned().collect()));

    for (scenario, param) in PHASE2G_SCENARIOS {
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
