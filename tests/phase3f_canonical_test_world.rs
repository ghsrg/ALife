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
    assert!(
        config
            .chemistry
            .resources
            .iter()
            .any(|resource| resource.id == "nucleotide_precursor")
    );
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
    let copying = config
        .genome_physical_accounting
        .copying
        .as_ref()
        .expect("canonical copying precursor accounting");
    assert_eq!(copying.carrier_material_id, "genome_carrier_matrix");
    assert_eq!(copying.precursor_requirements.len(), 4);
    assert!(
        config
            .genome_physical_accounting
            .recombination
            .as_ref()
            .is_some_and(|rule| rule.precursor_requirements.len() == 4)
    );
    let temperature = config
        .fields
        .iter()
        .find(|field| field.id == "temperature")
        .expect("canonical local temperature field");
    assert_eq!(temperature.initial_value.raw(), 25.0);
    assert_eq!(temperature.min_value.raw(), 0.0);
    assert_eq!(temperature.max_value.raw(), 100.0);
    assert!(
        document
            .canonical_source
            .contains("[canonical_manifests.genome_precursors]")
    );
    assert!(document.canonical_source.contains("[fields.temperature]"));
    assert_ne!(document.scenario_hash.raw(), 0);
}
