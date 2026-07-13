use alife::core::deltas::{
    AccountingRejection, AccountingReport, ReactionDestination, ReactionInventory,
    ReactionLocation, ReactionSource,
};
use alife::core::ids::{CellId, MaterialTypeId, ResourceTypeId};
use alife::core::reactions::{ReactionDelta, ReactionId, ReactionMode, ReactionTerm};
use alife::core::resource_types::{
    PermeabilityConstraint, ReactivityProfile, ResourceProperties, ResourceRegistry, ResourceTags,
    ResourceType,
};
use alife::core::resources::ResourceGrid;
use alife::core::units::{
    DecayRate, DiffusionRate, EnergyValue, GridCoord, MaterialAmount, ResourceAmount, Volume,
    WorldSize,
};

fn resource_term(id: u32, amount: f32) -> ReactionTerm {
    ReactionTerm::resource(
        ResourceTypeId::from_raw(id),
        ResourceAmount::new(amount).unwrap(),
    )
}

fn material_term(id: u32, amount: f32) -> ReactionTerm {
    ReactionTerm::material(
        MaterialTypeId::from_raw(id),
        MaterialAmount::new(amount).unwrap(),
    )
}

fn location(x: usize) -> ReactionLocation {
    ReactionLocation::Grid(GridCoord::new(x, 0))
}

fn source(id: u32) -> ReactionSource {
    ReactionSource::Cell(CellId::from_raw(id))
}

fn registry() -> ResourceRegistry {
    ResourceRegistry::new(vec![ResourceType::new(
        ResourceTypeId::from_raw(1),
        ResourceProperties::new(
            Volume::new(1.0).unwrap(),
            DiffusionRate::new(0.5).unwrap(),
            EnergyValue::new(1.0).unwrap(),
            DecayRate::new(0.0).unwrap(),
            ReactivityProfile::Stable,
            PermeabilityConstraint::Passive,
            ResourceTags::empty(),
        ),
    )])
    .unwrap()
}

#[test]
fn typed_diffusion_conserves_resource_amount_between_local_cells() {
    let mut grid = ResourceGrid::new_typed(
        WorldSize::new(2.0, 1.0).unwrap(),
        1.0,
        vec![(
            ResourceTypeId::from_raw(1),
            ResourceAmount::new(10.0).unwrap(),
        )],
    )
    .unwrap();
    let left = GridCoord::new(0, 0);
    let right = GridCoord::new(1, 0);
    grid.set_amount_at_type(
        ResourceTypeId::from_raw(1),
        left,
        ResourceAmount::new(10.0).unwrap(),
    )
    .unwrap();
    grid.set_amount_at_type(ResourceTypeId::from_raw(1), right, ResourceAmount::zero())
        .unwrap();

    grid.diffuse_resource_type(ResourceTypeId::from_raw(1), left, right, &registry())
        .unwrap();

    let total = grid
        .amount_at_type(ResourceTypeId::from_raw(1), left)
        .unwrap()
        .raw()
        + grid
            .amount_at_type(ResourceTypeId::from_raw(1), right)
            .unwrap()
            .raw();
    assert!((total - 10.0).abs() < f32::EPSILON);
    assert!(
        grid.amount_at_type(ResourceTypeId::from_raw(1), right)
            .unwrap()
            .raw()
            > 0.0
    );
}

#[test]
fn reaction_delta_consumes_inputs_and_produces_outputs_atomically() {
    let mut inventory = ReactionInventory::new();
    inventory.set_resource(
        location(0),
        ResourceTypeId::from_raw(1),
        ResourceAmount::new(2.0).unwrap(),
    );
    let delta = ReactionDelta::builder(
        location(0),
        source(3),
        ReactionId::from_raw(7),
        ReactionMode::Conversion,
    )
    .inputs(vec![resource_term(1, 2.0)])
    .destinations(vec![
        ReactionDestination::Product(resource_term(2, 1.25)),
        ReactionDestination::Residual(resource_term(3, 0.75)),
    ])
    .build();

    let report = AccountingReport::validate_and_commit(inventory, vec![delta]);

    assert_eq!(report.accepted_reaction_ids(), &[ReactionId::from_raw(7)]);
    assert_eq!(
        report
            .inventory()
            .resource(location(0), ResourceTypeId::from_raw(1)),
        ResourceAmount::zero()
    );
    assert_eq!(
        report
            .inventory()
            .resource(location(0), ResourceTypeId::from_raw(2)),
        ResourceAmount::new(1.25).unwrap()
    );
    assert_eq!(
        report
            .inventory()
            .resource(location(0), ResourceTypeId::from_raw(3)),
        ResourceAmount::new(0.75).unwrap()
    );
}

#[test]
fn unaccounted_input_rejects_whole_delta_without_partial_effects() {
    let mut inventory = ReactionInventory::new();
    inventory.set_resource(
        location(0),
        ResourceTypeId::from_raw(1),
        ResourceAmount::new(2.0).unwrap(),
    );
    let delta = ReactionDelta::builder(
        location(0),
        source(1),
        ReactionId::from_raw(8),
        ReactionMode::Degradation,
    )
    .inputs(vec![resource_term(1, 2.0)])
    .destinations(vec![ReactionDestination::Sink(resource_term(1, 1.5))])
    .build();

    let report = AccountingReport::validate_and_commit(inventory, vec![delta]);

    assert_eq!(
        report.rejection(ReactionId::from_raw(8)),
        Some(&AccountingRejection::UnaccountedInput)
    );
    assert_eq!(
        report
            .inventory()
            .resource(location(0), ResourceTypeId::from_raw(1)),
        ResourceAmount::new(2.0).unwrap()
    );
}

#[test]
fn products_without_input_are_rejected_without_creating_matter() {
    let inventory = ReactionInventory::new();
    let delta = ReactionDelta::builder(
        location(0),
        ReactionSource::Environment,
        ReactionId::from_raw(9),
        ReactionMode::Passive,
    )
    .destinations(vec![ReactionDestination::Product(material_term(4, 1.0))])
    .build();

    let report = AccountingReport::validate_and_commit(inventory, vec![delta]);

    assert_eq!(
        report.rejection(ReactionId::from_raw(9)),
        Some(&AccountingRejection::ProductsRequireInputs)
    );
    assert_eq!(
        report
            .inventory()
            .material(location(0), MaterialTypeId::from_raw(4)),
        MaterialAmount::zero()
    );
}

#[test]
fn competing_consumption_uses_deterministic_location_source_reaction_type_order() {
    let mut inventory = ReactionInventory::new();
    inventory.set_resource(
        location(0),
        ResourceTypeId::from_raw(1),
        ResourceAmount::new(1.0).unwrap(),
    );
    inventory.set_resource(
        location(1),
        ResourceTypeId::from_raw(1),
        ResourceAmount::new(1.0).unwrap(),
    );
    let make_delta = |at, source_id, reaction_id, output_id| {
        ReactionDelta::builder(
            location(at),
            source(source_id),
            ReactionId::from_raw(reaction_id),
            ReactionMode::Conversion,
        )
        .inputs(vec![resource_term(1, 1.0)])
        .destinations(vec![ReactionDestination::Product(resource_term(
            output_id, 1.0,
        ))])
        .build()
    };
    let deltas = vec![
        make_delta(1, 1, 1, 5),
        make_delta(0, 2, 1, 4),
        make_delta(0, 1, 9, 3),
        make_delta(0, 1, 2, 2),
    ];

    let report = AccountingReport::validate_and_commit(inventory, deltas);

    assert_eq!(
        report.accepted_reaction_ids(),
        &[ReactionId::from_raw(2), ReactionId::from_raw(1)]
    );
    assert_eq!(
        report.rejection(ReactionId::from_raw(9)),
        Some(&AccountingRejection::InsufficientInput)
    );
    assert_eq!(
        report.rejection(ReactionId::from_raw(1)),
        None,
        "the same reaction id at a later location is independently accepted"
    );
    assert_eq!(
        report
            .inventory()
            .resource(location(0), ResourceTypeId::from_raw(4)),
        ResourceAmount::zero(),
        "later source must not receive a partial product"
    );
}

#[test]
fn reaction_type_is_the_final_deterministic_conflict_tiebreaker() {
    let mut inventory = ReactionInventory::new();
    for id in 1..=3 {
        inventory.set_resource(
            location(0),
            ResourceTypeId::from_raw(id),
            ResourceAmount::new(1.0).unwrap(),
        );
    }
    let make_delta = |primary_id, output_id| {
        ReactionDelta::builder(
            location(0),
            source(1),
            ReactionId::from_raw(20),
            ReactionMode::Conversion,
        )
        .inputs(vec![resource_term(primary_id, 1.0), resource_term(3, 1.0)])
        .destinations(vec![ReactionDestination::Product(resource_term(
            output_id, 2.0,
        ))])
        .build()
    };

    let report =
        AccountingReport::validate_and_commit(inventory, vec![make_delta(2, 9), make_delta(1, 8)]);

    assert_eq!(
        report
            .inventory()
            .resource(location(0), ResourceTypeId::from_raw(8)),
        ResourceAmount::new(2.0).unwrap()
    );
    assert_eq!(
        report
            .inventory()
            .resource(location(0), ResourceTypeId::from_raw(9)),
        ResourceAmount::zero()
    );
}
