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
    assert!(
        document
            .canonical_source
            .contains("runtime_kind=\"scalar_only\"")
    );
    assert!(document.canonical_source.contains(
        "supported_scalar_profiles=[\"temperature\", \"light\", \"pressure\", \"radiation\", \"chemical_gradient\", \"flow\"]"
    ));
    assert!(document.canonical_source.contains(
        "unsupported_direct_effects=[\"field_to_energy_buffer\", \"field_to_cell_movement\", \"field_to_genome_mutation\", \"field_to_material_damage\", \"field_to_resource_transport\", \"field_to_genome_behavior\"]"
    ));
    assert_ne!(document.scenario_hash.raw(), 0);
}

#[test]
fn canonical_test_world_snapshot_preserves_resource_layer_identity() {
    let document = ScenarioDocument::resolve(ScenarioSource::Path(
        "config/scenarios/demo/canonical_test_world.toml".into(),
    ))
    .unwrap();
    let world =
        alife::core::world::WorldState::from_config(document.runtime_config.clone()).unwrap();
    let snapshot = alife::core::snapshot::CommittedSnapshot::from_world(&world);

    let expected_ids = &document.resource_type_ids;
    assert_eq!(expected_ids.len(), 19);
    assert_eq!(snapshot.resource_layers.len(), expected_ids.len());

    for (index, expected_id) in expected_ids.iter().enumerate() {
        let layer = &snapshot.resource_layers[index];
        assert_eq!(layer.layer_index, index as u32);
        assert_eq!(layer.resource_type_id, index as u32);
        assert_eq!(layer.resource_id, *expected_id);
        assert!(layer.width > 0);
        assert!(layer.height > 0);
        assert_eq!(layer.cells.len(), (layer.width * layer.height) as usize);
    }

    assert_eq!(snapshot.resource_layers[0].resource_id, "amino_acid");
    assert_eq!(
        snapshot.resource_layers[18].resource_id,
        "nucleotide_precursor"
    );
}
