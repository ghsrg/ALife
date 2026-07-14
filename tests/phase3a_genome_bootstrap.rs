use alife::core::genome::{
    GenomeCarrierState, GenomeOutputId, GenomeOutputValue, GenomeTemplate, GenomeTemplateId,
};

#[test]
fn genome_output_value_clamps_to_canonical_range() {
    assert_eq!(GenomeOutputValue::new(-1.5).raw(), -1.0);
    assert_eq!(GenomeOutputValue::new(1.5).raw(), 1.0);
    assert_eq!(GenomeOutputValue::new(0.25).raw(), 0.25);
}

#[test]
fn genome_output_id_accepts_registered_phase3a_outputs() {
    assert_eq!(
        GenomeOutputId::parse("resource_uptake_priority").unwrap(),
        GenomeOutputId::ResourceUptakePriority
    );
    assert_eq!(
        GenomeOutputId::parse("energy_conversion_priority").unwrap(),
        GenomeOutputId::EnergyConversionPriority
    );
    assert_eq!(
        GenomeOutputId::parse("material_synthesis_priority").unwrap(),
        GenomeOutputId::MaterialSynthesisPriority
    );
    assert_eq!(
        GenomeOutputId::parse("repair_priority").unwrap(),
        GenomeOutputId::RepairPriority
    );
    assert_eq!(
        GenomeOutputId::parse("movement_priority").unwrap(),
        GenomeOutputId::MovementPriority
    );
    assert_eq!(
        GenomeOutputId::parse("division_preparation_priority").unwrap(),
        GenomeOutputId::DivisionPreparationPriority
    );
}

#[test]
fn genome_output_id_rejects_unregistered_phase3a_outputs() {
    assert!(GenomeOutputId::parse("growth_priority").is_err());
    assert!(GenomeOutputId::parse("joint_create_priority").is_err());
    assert!(GenomeOutputId::parse("observer_fitness").is_err());
}

#[test]
fn genome_template_requires_non_negative_variation_and_carrier() {
    let carrier = GenomeCarrierState::new("genome_carrier_A".to_string(), 1.0, 1.0).unwrap();
    let template = GenomeTemplate::new(
        GenomeTemplateId::new("balanced").unwrap(),
        0.08,
        1,
        carrier,
        vec![(
            GenomeOutputId::ResourceUptakePriority,
            GenomeOutputValue::new(0.7),
        )],
    )
    .unwrap();

    assert_eq!(template.id().as_str(), "balanced");
    assert_eq!(template.variation_amplitude(), 0.08);
    assert_eq!(template.runtime_interval_ticks(), 1);
}
