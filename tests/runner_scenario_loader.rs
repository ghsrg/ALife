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

#[test]
fn demo_living_world_is_available_for_manual_runner_usage() {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios
        .iter()
        .find(|scenario| scenario.id == "demo_living_world")
        .expect("demo_living_world should be listed for manual use");
    let document = load_scenario_document(meta).unwrap();
    let config = &document.runtime_config;

    assert_eq!(document.id, "demo_living_world");
    assert_eq!(config.world.size.width(), 1024.0);
    assert_eq!(config.world.size.height(), 768.0);
    assert_eq!(config.world.tick_count.raw(), 50_000);
    assert_eq!(config.initial_cells.len(), 24);
    assert_eq!(config.initial_cell_genome_templates.len(), 24);
    assert!(config.resource_interaction.enabled);
    assert!(config.division.enabled);
    assert!(config.decomposition.enabled);
    assert!(config.local_interaction.enabled);
    assert!(config.joints.enabled);
    assert_eq!(config.simulation_time.tick_duration_ms, 100);
    assert_eq!(config.scheduler.cell.genome_runtime_base_ticks, 10);
    assert_eq!(config.scheduler.cell.genome_runtime_ticks_per_layer, 10);
    assert_eq!(config.scheduler.world.resource_diffusion_ticks, 2);
    assert_eq!(config.scheduler.world.resource_decay_ticks, 5);
    assert_eq!(config.scheduler.observer.resource_totals_ticks, 10);
    assert!(config.cell.initial_energy.raw() <= config.cell.energy_capacity.raw());
}

#[test]
fn demo_living_world_uses_non_every_tick_genome_cadence() {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios
        .iter()
        .find(|scenario| scenario.id == "demo_living_world")
        .unwrap();
    let document = load_scenario_document(meta).unwrap();
    let config = document.runtime_config;
    let template = config
        .genome_templates
        .iter()
        .find(|template| template.id().as_str() == "balanced")
        .unwrap();

    assert!(
        config.effective_genome_runtime_cadence_ticks_for_template(template) > 1,
        "demo_living_world must not override Genome Runtime back to every Tick"
    );
}
