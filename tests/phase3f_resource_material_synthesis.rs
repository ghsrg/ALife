use alife::core::material_instance::{
    MaterialCapabilityProfile, MaterialFragmentConversionRecipe, MaterialInstance, MaterialProfile,
    MaterialRecipeInput, MaterialSynthesisInventory, MaterialSynthesisRecipe,
    MaterialSynthesisRejection,
};
use alife::core::process::MaterialCapability;
use alife::core::units::{
    EnergyAmount, HeatAmount, MaterialAmount, Position, ResourceAmount, Tick,
};

fn profile(
    volume: f32,
    stability: f32,
    strength: f32,
    energy_capacity: f32,
    permeability: f32,
    durability: f32,
) -> MaterialProfile {
    MaterialProfile::new(
        volume,
        stability,
        strength,
        energy_capacity,
        permeability,
        durability,
    )
    .unwrap()
}

#[test]
fn material_instance_derives_profile_and_capabilities_from_volume_weighted_precursors() {
    let amino = MaterialRecipeInput::new(
        "amino_acid",
        MaterialAmount::new(1.0).unwrap(),
        profile(0.4, 0.5, 0.3, 0.2, 0.7, 0.4),
        MaterialCapabilityProfile::empty().with(MaterialCapability::MaterialSynthesis, 0.8),
    );
    let phospholipid = MaterialRecipeInput::new(
        "phospholipid",
        MaterialAmount::new(3.0).unwrap(),
        profile(0.8, 0.7, 0.6, 0.1, 0.3, 0.9),
        MaterialCapabilityProfile::empty().with(MaterialCapability::BoundaryPermeability, 0.6),
    );

    let material = MaterialInstance::from_precursors(
        MaterialAmount::new(2.0).unwrap(),
        vec![amino, phospholipid],
    )
    .unwrap();

    assert_eq!(material.amount().raw(), 2.0);
    assert!((material.profile().strength() - 0.525).abs() < 0.000_001);
    assert!((material.profile().durability() - 0.775).abs() < 0.000_001);
    assert!(
        (material
            .capabilities()
            .value(MaterialCapability::MaterialSynthesis)
            - 0.2)
            .abs()
            < 0.000_001
    );
    assert!(
        (material
            .capabilities()
            .value(MaterialCapability::BoundaryPermeability)
            - 0.45)
            .abs()
            < 0.000_001
    );
}

#[test]
fn material_instance_rejects_empty_or_zero_volume_precursor_sets() {
    assert!(
        MaterialInstance::from_precursors(MaterialAmount::new(1.0).unwrap(), Vec::new()).is_err()
    );
}

#[test]
fn material_instance_fingerprint_is_independent_from_precursor_declaration_order() {
    let amino = MaterialRecipeInput::new(
        "amino_acid",
        MaterialAmount::new(1.0).unwrap(),
        profile(0.4, 0.5, 0.3, 0.2, 0.7, 0.4),
        MaterialCapabilityProfile::empty().with(MaterialCapability::MaterialSynthesis, 0.8),
    );
    let phospholipid = MaterialRecipeInput::new(
        "phospholipid",
        MaterialAmount::new(3.0).unwrap(),
        profile(0.8, 0.7, 0.6, 0.1, 0.3, 0.9),
        MaterialCapabilityProfile::empty().with(MaterialCapability::BoundaryPermeability, 0.6),
    );

    let declared = MaterialInstance::from_precursors(
        MaterialAmount::new(2.0).unwrap(),
        vec![amino.clone(), phospholipid.clone()],
    )
    .unwrap();
    let reversed = MaterialInstance::from_precursors(
        MaterialAmount::new(2.0).unwrap(),
        vec![phospholipid, amino],
    )
    .unwrap();

    assert_eq!(declared.profile(), reversed.profile());
    assert_eq!(declared.capabilities(), reversed.capabilities());
    assert_eq!(declared.stable_fingerprint(), reversed.stable_fingerprint());
}

fn membrane_recipe() -> MaterialSynthesisRecipe {
    MaterialSynthesisRecipe::new(
        "flexible_membrane",
        MaterialAmount::new(2.0).unwrap(),
        EnergyAmount::new(4.0).unwrap(),
        HeatAmount::new(0.2).unwrap(),
        vec![
            MaterialRecipeInput::new(
                "amino_acid",
                MaterialAmount::new(1.0).unwrap(),
                profile(0.4, 0.5, 0.3, 0.2, 0.7, 0.4),
                MaterialCapabilityProfile::empty().with(MaterialCapability::MaterialSynthesis, 0.8),
            ),
            MaterialRecipeInput::new(
                "phospholipid",
                MaterialAmount::new(3.0).unwrap(),
                profile(0.8, 0.7, 0.6, 0.1, 0.3, 0.9),
                MaterialCapabilityProfile::empty()
                    .with(MaterialCapability::BoundaryPermeability, 0.6),
            ),
        ],
        vec![("inert_waste", ResourceAmount::new(0.5).unwrap())],
    )
}

#[test]
fn material_synthesis_transaction_debits_inputs_and_records_outputs_atomically() {
    let recipe = membrane_recipe();
    let mut inventory = MaterialSynthesisInventory::new(
        EnergyAmount::new(6.0).unwrap(),
        MaterialAmount::new(5.0).unwrap(),
    );
    inventory.set_resource("amino_acid", ResourceAmount::new(1.0).unwrap());
    inventory.set_resource("phospholipid", ResourceAmount::new(3.5).unwrap());

    let outcome = recipe.apply(&mut inventory).unwrap();

    assert_eq!(inventory.energy().raw(), 2.0);
    assert_eq!(inventory.resource_amount("amino_acid").raw(), 0.0);
    assert_eq!(inventory.resource_amount("phospholipid").raw(), 0.5);
    assert_eq!(inventory.materials().len(), 1);
    assert_eq!(
        outcome.material().stable_fingerprint(),
        inventory.materials()[0].stable_fingerprint()
    );
    assert!((outcome.material().profile().strength() - 0.525).abs() < 0.000_001);
    assert_eq!(outcome.waste_outputs()[0].0, "inert_waste");
    assert_eq!(outcome.waste_outputs()[0].1.raw(), 0.5);
    assert_eq!(outcome.heat_output().raw(), 0.2);
}

#[test]
fn material_synthesis_rejections_do_not_mutate_inventory() {
    let recipe = membrane_recipe();
    let mut inventory = MaterialSynthesisInventory::new(
        EnergyAmount::new(3.9).unwrap(),
        MaterialAmount::new(5.0).unwrap(),
    );
    inventory.set_resource("amino_acid", ResourceAmount::new(1.0).unwrap());
    inventory.set_resource("phospholipid", ResourceAmount::new(3.0).unwrap());
    let before = inventory.snapshot();

    let rejection = recipe.apply(&mut inventory).unwrap_err();

    assert_eq!(rejection, MaterialSynthesisRejection::InsufficientEnergy);
    assert_eq!(inventory.snapshot(), before);
}

#[test]
fn material_degradation_creates_profile_preserving_inactive_fragment() {
    let material = membrane_recipe()
        .derive_material_instance()
        .expect("recipe is valid");

    let fragment = material
        .degrade_to_fragment(
            MaterialAmount::new(0.75).unwrap(),
            Position::new(3.0, 4.0),
            Tick::from_raw(8),
        )
        .unwrap();

    assert_eq!(fragment.amount().raw(), 0.75);
    assert_eq!(fragment.position(), Position::new(3.0, 4.0));
    assert_eq!(fragment.created_tick(), Tick::from_raw(8));
    assert_eq!(fragment.profile(), material.profile());
    assert_eq!(fragment.source_fingerprint(), material.stable_fingerprint());
    assert_eq!(
        fragment.active_cell_capability(MaterialCapability::BoundaryPermeability),
        0.0
    );
    assert!(fragment.resource_outputs_without_conversion().is_empty());
}

#[test]
fn material_fragment_becomes_resources_only_through_explicit_conversion_recipe() {
    let material = membrane_recipe()
        .derive_material_instance()
        .expect("recipe is valid");
    let fragment = material
        .degrade_to_fragment(
            MaterialAmount::new(1.0).unwrap(),
            Position::new(0.0, 0.0),
            Tick::from_raw(1),
        )
        .unwrap();
    let conversion = MaterialFragmentConversionRecipe::new(
        "fragment_to_precursors",
        vec![
            ("amino_acid", ResourceAmount::new(0.2).unwrap()),
            ("inert_waste", ResourceAmount::new(0.6).unwrap()),
        ],
    );

    let outputs = conversion.convert(&fragment);

    assert_eq!(outputs[0].0, "amino_acid");
    assert_eq!(outputs[0].1.raw(), 0.2);
    assert_eq!(outputs[1].0, "inert_waste");
    assert_eq!(outputs[1].1.raw(), 0.6);
}
