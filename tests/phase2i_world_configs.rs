use alife::runner::config_parser::RawScenarioConfig;

fn parse(path: &str) -> alife::core::config::RuntimeConfig {
    let text = std::fs::read_to_string(path).unwrap();
    RawScenarioConfig::parse(&text).unwrap()
}

#[test]
fn integrated_world_configs_parse_and_have_distinct_hashes() {
    let baseline = parse("config/scenarios/world/world_baseline_stable.toml");
    let showcase = parse("config/scenarios/world/world_mechanism_showcase.toml");
    let stress = parse("config/scenarios/world/world_stress_regression.toml");

    assert_ne!(baseline.config_hash(), showcase.config_hash());
    assert_ne!(baseline.config_hash(), stress.config_hash());
    assert_ne!(showcase.config_hash(), stress.config_hash());
}

#[test]
fn baseline_world_enables_required_phase2_mechanisms_without_genome() {
    let config = parse("config/scenarios/world/world_baseline_stable.toml");

    assert!(config.resource_interaction.enabled);
    assert!(config.growth_enabled);
    assert!(config.division.enabled);
    assert!(config.decomposition.enabled);
    assert!(config.local_interaction.enabled);
    assert!(config.joints.enabled);
    assert!(config.chemistry.repair.enabled);
    assert!(!config.chemistry.reactions.is_empty());
}
