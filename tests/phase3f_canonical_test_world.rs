use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};

#[test]
fn canonical_test_world_resolves_resource_derived_material_synthesis_surface() {
    let document = ScenarioDocument::resolve(ScenarioSource::Path(
        "config/scenarios/demo/canonical_test_world.toml".into(),
    ))
    .unwrap();

    let config = &document.runtime_config;
    assert_eq!(document.id, "canonical_test_world");
    assert!(config.chemistry.resources.len() >= 18);
    assert!(config
        .chemistry
        .resources
        .iter()
        .any(|resource| resource.id == "nucleotide_precursor"));
    assert_eq!(
        config
            .chemistry
            .reactions
            .iter()
            .filter(
                |reaction| reaction.process_id.as_deref() == Some("material_synthesis")
                    && reaction.material_output.is_some()
            )
            .count(),
        7
    );
    assert!(config.chemistry.reactions.iter().any(|reaction| {
        reaction.id == "spent_material_fragment_conversion"
            && reaction.mode == "passive"
            && reaction.outputs.iter().any(|(id, _)| id == "phosphate")
    }));
    assert_ne!(document.scenario_hash.raw(), 0);
}
