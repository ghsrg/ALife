use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};

#[test]
fn runner_resolves_canonical_scenario_document_with_hash() {
    let document = ScenarioDocument::resolve(ScenarioSource::Path(
        "config/scenarios/bootstrap/minimal_viable_world.toml".into(),
    ))
    .unwrap();

    assert_eq!(document.id, "bootstrap_minimal_viable_world");
    assert_eq!(document.schema_version, 1);
    assert_ne!(document.scenario_hash.raw(), 0);
    assert_eq!(document.runtime_config.world.seed.raw(), 42);
}
