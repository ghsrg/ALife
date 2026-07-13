use alife::core::ids::MaterialTypeId;
use alife::core::material_types::{
    MaterialProperties, MaterialState, ReactionProfile, RepairRequirements, SignalProperties,
};
use alife::core::units::{AmountError, DecayRate, EnergyCapacity, SignalAmount, Strength, Volume};

#[test]
fn material_properties_capture_chemistry_and_signal_contract() {
    let properties = MaterialProperties::new(
        Volume::new(2.0).unwrap(),
        Strength::new(0.8).unwrap(),
        Strength::new(0.9).unwrap(),
        Strength::new(0.4).unwrap(),
        EnergyCapacity::new(5.0).unwrap(),
        DecayRate::new(0.01).unwrap(),
        RepairRequirements::new(Volume::new(0.2).unwrap()),
        ReactionProfile::Passive,
        SignalProperties::new(
            Strength::new(0.3).unwrap(),
            SignalAmount::new(1.0).unwrap(),
            Strength::new(0.7).unwrap(),
        ),
    );

    assert_eq!(properties.volume().raw(), 2.0);
    assert_eq!(properties.energy_capacity().raw(), 5.0);
    assert_eq!(properties.reaction_profile(), ReactionProfile::Passive);
    assert_eq!(properties.signal().signal_storage().raw(), 1.0);
}

#[test]
fn material_state_is_bounded_by_typed_state_units() {
    let state = MaterialState::new(
        Strength::new(0.25).unwrap(),
        Strength::new(0.5).unwrap(),
        SignalAmount::new(0.75).unwrap(),
        Strength::new(0.8).unwrap(),
    );

    assert_eq!(state.damage().raw(), 0.25);
    assert_eq!(state.stored_signal().raw(), 0.75);
    assert!(Strength::new(1.1).is_err());
    assert!(SignalAmount::new(1.1).is_err());
}

#[test]
fn interval_units_report_out_of_range_values_precisely() {
    assert_eq!(Strength::new(-0.1), Err(AmountError::OutOfRange));
    assert_eq!(Strength::new(1.1), Err(AmountError::OutOfRange));
    assert_eq!(SignalAmount::new(-0.1), Err(AmountError::OutOfRange));
    assert_eq!(SignalAmount::new(1.1), Err(AmountError::OutOfRange));
}

#[test]
fn material_registry_looks_up_deterministically_and_preserves_differential_properties() {
    use alife::core::material_types::{MaterialRegistry, MaterialType};

    let make_properties = |volume, stability| {
        MaterialProperties::new(
            Volume::new(volume).unwrap(),
            Strength::new(stability).unwrap(),
            Strength::new(0.5).unwrap(),
            Strength::new(0.2).unwrap(),
            EnergyCapacity::new(1.0).unwrap(),
            DecayRate::new(0.01).unwrap(),
            RepairRequirements::new(Volume::new(0.1).unwrap()),
            ReactionProfile::Passive,
            SignalProperties::new(
                Strength::new(0.1).unwrap(),
                SignalAmount::new(0.2).unwrap(),
                Strength::new(0.3).unwrap(),
            ),
        )
    };
    let first = MaterialType::new(MaterialTypeId::from_raw(2), make_properties(4.0, 0.8));
    let second = MaterialType::new(MaterialTypeId::from_raw(1), make_properties(2.0, 0.3));

    let registry = MaterialRegistry::new(vec![first, second]).unwrap();
    assert_eq!(
        registry
            .get(MaterialTypeId::from_raw(1))
            .unwrap()
            .properties()
            .volume()
            .raw(),
        2.0
    );
    assert_eq!(
        registry
            .get(MaterialTypeId::from_raw(2))
            .unwrap()
            .properties()
            .stability()
            .raw(),
        0.8
    );
    assert!(registry.get(MaterialTypeId::from_raw(99)).is_none());
    assert_eq!(
        registry
            .iter()
            .map(|material| material.id().raw())
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
}

#[test]
fn material_registry_rejects_duplicate_ids() {
    use alife::core::material_types::{MaterialRegistry, MaterialRegistryError, MaterialType};
    let properties = MaterialProperties::new(
        Volume::new(1.0).unwrap(),
        Strength::new(0.5).unwrap(),
        Strength::new(0.5).unwrap(),
        Strength::new(0.5).unwrap(),
        EnergyCapacity::new(0.0).unwrap(),
        DecayRate::new(0.0).unwrap(),
        RepairRequirements::new(Volume::new(0.0).unwrap()),
        ReactionProfile::Passive,
        SignalProperties::new(
            Strength::new(0.0).unwrap(),
            SignalAmount::new(0.0).unwrap(),
            Strength::new(0.0).unwrap(),
        ),
    );
    let material = MaterialType::new(MaterialTypeId::from_raw(1), properties);
    let error = MaterialRegistry::new(vec![material, material]).unwrap_err();
    assert_eq!(
        error,
        MaterialRegistryError::DuplicateId(MaterialTypeId::from_raw(1))
    );
}

#[test]
fn material_registry_reports_unknown_ids_as_reachable_errors() {
    use alife::core::material_types::{MaterialRegistry, MaterialRegistryError};

    let registry = MaterialRegistry::new(Vec::new()).unwrap();
    assert_eq!(
        registry.lookup(MaterialTypeId::from_raw(99)),
        Err(MaterialRegistryError::UnknownId(MaterialTypeId::from_raw(
            99
        )))
    );
}
