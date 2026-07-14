use alife::runner::scenario::{load_scenario_document, scan_scenarios};

#[test]
fn scan_scenarios_finds_nested_toml_files_with_ids() {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let ids = scenarios
        .iter()
        .map(|scenario| scenario.id.as_str())
        .collect::<Vec<_>>();

    assert!(ids.contains(&"bootstrap_minimal_viable_world"));
    assert!(ids.contains(&"phase3a_genome_bootstrap"));
}

#[test]
fn scan_scenarios_returns_stable_sorted_ids() {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let ids = scenarios
        .iter()
        .map(|scenario| scenario.id.clone())
        .collect::<Vec<_>>();
    let mut sorted = ids.clone();
    sorted.sort();

    assert_eq!(ids, sorted);
}

#[test]
fn load_scenario_document_resolves_path_through_scenario_document() {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios
        .iter()
        .find(|scenario| scenario.id == "bootstrap_minimal_viable_world")
        .unwrap();
    let document = load_scenario_document(meta).unwrap();

    assert_eq!(document.id, "bootstrap_minimal_viable_world");
    assert_eq!(document.runtime_config.world.seed.raw(), 42);
}
