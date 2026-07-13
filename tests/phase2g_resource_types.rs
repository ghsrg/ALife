use alife::core::ids::ResourceTypeId;
use alife::core::resource_types::{
    PermeabilityConstraint, ReactivityProfile, ResourceProperties, ResourceTag, ResourceTags,
    ResourceType,
};
use alife::core::units::{DecayRate, DiffusionRate, EnergyValue, Volume};

#[test]
fn resource_type_keeps_typed_properties_and_tags() {
    let tags = ResourceTags::from([ResourceTag::EnergySource, ResourceTag::Dissolved]);
    let properties = ResourceProperties::new(
        Volume::new(1.0).unwrap(),
        DiffusionRate::new(0.2).unwrap(),
        EnergyValue::new(4.0).unwrap(),
        DecayRate::new(0.01).unwrap(),
        ReactivityProfile::Stable,
        PermeabilityConstraint::Passive,
        tags,
    );
    let resource = ResourceType::new(ResourceTypeId::from_raw(7), properties);

    assert_eq!(resource.id(), ResourceTypeId::from_raw(7));
    assert_eq!(resource.properties().volume().raw(), 1.0);
    assert_eq!(resource.properties().diffusion_rate().raw(), 0.2);
    assert_eq!(resource.properties().energy_value().raw(), 4.0);
    assert!(
        resource
            .properties()
            .tags()
            .contains(ResourceTag::Dissolved)
    );
}

#[test]
fn resource_properties_reject_non_finite_typed_values() {
    assert!(Volume::new(f32::NAN).is_err());
    assert!(DiffusionRate::new(-0.1).is_err());
    assert!(EnergyValue::new(f32::INFINITY).is_err());
}

#[test]
fn resource_registry_looks_up_deterministically_and_preserves_differential_properties() {
    use alife::core::resource_types::ResourceRegistry;

    let first = ResourceType::new(
        ResourceTypeId::from_raw(2),
        ResourceProperties::new(
            Volume::new(1.0).unwrap(),
            DiffusionRate::new(0.1).unwrap(),
            EnergyValue::new(2.0).unwrap(),
            DecayRate::new(0.0).unwrap(),
            ReactivityProfile::Stable,
            PermeabilityConstraint::Blocked,
            ResourceTags::from([ResourceTag::Dissolved]),
        ),
    );
    let second = ResourceType::new(
        ResourceTypeId::from_raw(1),
        ResourceProperties::new(
            Volume::new(3.0).unwrap(),
            DiffusionRate::new(0.8).unwrap(),
            EnergyValue::new(9.0).unwrap(),
            DecayRate::new(0.2).unwrap(),
            ReactivityProfile::Reactive,
            PermeabilityConstraint::Passive,
            ResourceTags::from([ResourceTag::Waste, ResourceTag::EnergySource]),
        ),
    );

    let registry = ResourceRegistry::new(vec![first, second]).unwrap();
    assert_eq!(
        registry
            .get(ResourceTypeId::from_raw(1))
            .unwrap()
            .properties()
            .volume()
            .raw(),
        3.0
    );
    assert_eq!(
        registry
            .get(ResourceTypeId::from_raw(2))
            .unwrap()
            .properties()
            .energy_value()
            .raw(),
        2.0
    );
    assert!(registry.get(ResourceTypeId::from_raw(99)).is_none());
    assert_eq!(
        registry
            .iter()
            .map(|resource| resource.id().raw())
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
}

#[test]
fn resource_registry_rejects_duplicate_ids() {
    use alife::core::resource_types::{ResourceRegistry, ResourceRegistryError};

    let resource = ResourceType::new(
        ResourceTypeId::from_raw(1),
        ResourceProperties::new(
            Volume::new(1.0).unwrap(),
            DiffusionRate::new(0.0).unwrap(),
            EnergyValue::new(0.0).unwrap(),
            DecayRate::new(0.0).unwrap(),
            ReactivityProfile::Stable,
            PermeabilityConstraint::Blocked,
            ResourceTags::empty(),
        ),
    );
    let error = ResourceRegistry::new(vec![resource, resource]).unwrap_err();
    assert_eq!(
        error,
        ResourceRegistryError::DuplicateId(ResourceTypeId::from_raw(1))
    );
}

#[test]
fn resource_registry_reports_unknown_ids_as_reachable_errors() {
    use alife::core::resource_types::{ResourceRegistry, ResourceRegistryError};

    let registry = ResourceRegistry::new(Vec::new()).unwrap();
    assert_eq!(
        registry.lookup(ResourceTypeId::from_raw(99)),
        Err(ResourceRegistryError::UnknownId(ResourceTypeId::from_raw(
            99
        )))
    );
}

#[test]
fn resource_tags_accept_variable_sized_deterministic_collections() {
    let tags = ResourceTags::from([
        ResourceTag::EnergySource,
        ResourceTag::Dissolved,
        ResourceTag::StructuralPrecursor,
        ResourceTag::Waste,
    ]);
    assert!(tags.contains(ResourceTag::StructuralPrecursor));
    assert_eq!(tags.bits(), 0b1111);

    let collected: ResourceTags = vec![ResourceTag::Waste, ResourceTag::EnergySource]
        .into_iter()
        .collect();
    assert_eq!(collected.bits(), 0b1001);
}

#[test]
fn resource_grid_applies_different_decay_rates_by_resource_type() {
    use alife::core::resources::ResourceGrid;
    use alife::core::units::{GridCoord, WorldSize};

    let nutrient = ResourceType::new(
        ResourceTypeId::from_raw(1),
        ResourceProperties::new(
            Volume::new(1.0).unwrap(),
            DiffusionRate::new(0.0).unwrap(),
            EnergyValue::new(1.0).unwrap(),
            DecayRate::new(0.1).unwrap(),
            ReactivityProfile::Stable,
            PermeabilityConstraint::Passive,
            ResourceTags::empty(),
        ),
    );
    let waste = ResourceType::new(
        ResourceTypeId::from_raw(2),
        ResourceProperties::new(
            Volume::new(1.0).unwrap(),
            DiffusionRate::new(0.0).unwrap(),
            EnergyValue::new(0.0).unwrap(),
            DecayRate::new(0.5).unwrap(),
            ReactivityProfile::Reactive,
            PermeabilityConstraint::Blocked,
            ResourceTags::empty(),
        ),
    );
    let registry =
        alife::core::resource_types::ResourceRegistry::new(vec![nutrient, waste]).unwrap();
    let mut grid = ResourceGrid::new_typed(
        WorldSize::new(2.0, 1.0).unwrap(),
        1.0,
        vec![
            (
                ResourceTypeId::from_raw(1),
                alife::core::units::ResourceAmount::new(10.0).unwrap(),
            ),
            (
                ResourceTypeId::from_raw(2),
                alife::core::units::ResourceAmount::new(10.0).unwrap(),
            ),
        ],
    )
    .unwrap();

    grid.decay_with_registry(&registry);

    let coord = GridCoord::new(0, 0);
    assert_eq!(
        grid.amount_at_type(ResourceTypeId::from_raw(1), coord)
            .unwrap()
            .raw(),
        9.0
    );
    assert_eq!(
        grid.amount_at_type(ResourceTypeId::from_raw(2), coord)
            .unwrap()
            .raw(),
        5.0
    );
}

#[test]
fn resource_tag_discriminants_are_stable() {
    assert_eq!(ResourceTag::EnergySource as u8, 0);
    assert_eq!(ResourceTag::Dissolved as u8, 1);
    assert_eq!(ResourceTag::StructuralPrecursor as u8, 2);
    assert_eq!(ResourceTag::Waste as u8, 3);
}
