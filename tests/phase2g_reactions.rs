use alife::core::deltas::{
    AccountingRejection, AccountingReport, ReactionDestination, ReactionInventory,
    ReactionLocation, ReactionSource,
};
use alife::core::ids::CellId;
use alife::core::ids::{MaterialTypeId, ResourceTypeId};
use alife::core::reactions::{
    CatalystRequirement, Locality, Reaction, ReactionConditions, ReactionContext, ReactionDelta,
    ReactionId, ReactionMode, ReactionRegistry, ReactionRegistryError, ReactionTerm,
};
use alife::core::units::{
    EnergyAmount, GridCoord, HeatAmount, MaterialAmount, Radius, ResourceAmount, Temperature,
};

fn resource(id: u32, amount: f32) -> ReactionTerm {
    ReactionTerm::resource(
        ResourceTypeId::from_raw(id),
        ResourceAmount::new(amount).unwrap(),
    )
}

fn material(id: u32, amount: f32) -> ReactionTerm {
    ReactionTerm::material(
        MaterialTypeId::from_raw(id),
        MaterialAmount::new(amount).unwrap(),
    )
}

fn context() -> ReactionContext {
    ReactionContext::new(
        vec![resource(1, 2.0), resource(2, 1.0)],
        vec![material(7, 0.5)],
        Temperature::new(0.5),
        Radius::new(0.25).unwrap(),
    )
}

#[test]
fn reaction_modes_cover_the_six_supported_families() {
    assert_eq!(
        [
            ReactionMode::Passive,
            ReactionMode::Controlled,
            ReactionMode::Degradation,
            ReactionMode::Decay,
            ReactionMode::Synthesis,
            ReactionMode::Conversion,
        ]
        .len(),
        6
    );
}

#[test]
fn reaction_matches_terms_conditions_catalyst_rate_probability_and_locality() {
    let reaction = Reaction::builder(ReactionId::from_raw(4), ReactionMode::Controlled)
        .inputs(vec![resource(1, 1.0), resource(2, 0.5)])
        .outputs(vec![resource(3, 0.5)])
        .conditions(ReactionConditions::temperature_between(0.2, 0.8).unwrap())
        .catalyst(CatalystRequirement::new(
            MaterialTypeId::from_raw(7),
            MaterialAmount::new(0.2).unwrap(),
        ))
        .rate(0.25)
        .probability(0.8)
        .locality(Locality::new(Radius::new(0.5).unwrap()))
        .build()
        .unwrap();

    assert!(reaction.matches(&context()));
    assert!(!reaction.matches(&ReactionContext::new(
        vec![resource(1, 0.5), resource(2, 1.0)],
        vec![material(7, 0.5)],
        Temperature::new(0.5),
        Radius::new(0.25).unwrap(),
    )));
}

#[test]
fn reaction_rejects_invalid_rate_probability_and_empty_accounting() {
    assert!(
        Reaction::builder(ReactionId::from_raw(1), ReactionMode::Decay)
            .inputs(Vec::new())
            .outputs(vec![resource(1, 1.0)])
            .build()
            .is_err()
    );
    assert!(
        Reaction::builder(ReactionId::from_raw(2), ReactionMode::Decay)
            .inputs(vec![resource(1, 1.0)])
            .rate(-1.0)
            .build()
            .is_err()
    );
    assert!(
        Reaction::builder(ReactionId::from_raw(3), ReactionMode::Decay)
            .inputs(vec![resource(1, 1.0)])
            .probability(1.1)
            .build()
            .is_err()
    );
}

#[test]
fn registry_validates_ids_and_returns_sorted_deterministic_candidates() {
    let first = Reaction::builder(ReactionId::from_raw(8), ReactionMode::Passive)
        .inputs(vec![resource(1, 1.0)])
        .outputs(vec![resource(2, 1.0)])
        .build()
        .unwrap();
    let second = Reaction::builder(ReactionId::from_raw(2), ReactionMode::Decay)
        .inputs(vec![resource(1, 1.0)])
        .outputs(vec![resource(2, 1.0)])
        .build()
        .unwrap();
    let registry = ReactionRegistry::new(vec![first, second]).unwrap();

    assert_eq!(
        registry
            .matching_candidates(&context())
            .map(|reaction| reaction.id())
            .collect::<Vec<_>>(),
        vec![ReactionId::from_raw(2), ReactionId::from_raw(8)]
    );
    assert_eq!(
        ReactionRegistry::new(vec![
            Reaction::builder(ReactionId::from_raw(2), ReactionMode::Decay)
                .inputs(vec![resource(1, 1.0)])
                .outputs(vec![resource(2, 1.0)])
                .build()
                .unwrap(),
            Reaction::builder(ReactionId::from_raw(2), ReactionMode::Decay)
                .inputs(vec![resource(1, 1.0)])
                .outputs(vec![resource(2, 1.0)])
                .build()
                .unwrap(),
        ])
        .unwrap_err(),
        ReactionRegistryError::DuplicateId(ReactionId::from_raw(2))
    );
}

#[test]
fn passive_reaction_releases_heat_without_crediting_energy_buffer() {
    let location = ReactionLocation::Grid(GridCoord::new(0, 0));
    let mut inventory = ReactionInventory::new();
    inventory.set_resource(
        location,
        ResourceTypeId::from_raw(1),
        ResourceAmount::new(1.0).unwrap(),
    );
    let delta = ReactionDelta::builder(
        location,
        ReactionSource::Environment,
        ReactionId::from_raw(10),
        ReactionMode::Passive,
    )
    .inputs(vec![resource(1, 1.0)])
    .destinations(vec![ReactionDestination::Residual(resource(2, 1.0))])
    .heat_output(HeatAmount::new(0.4).unwrap())
    .build();

    let report = AccountingReport::validate_and_commit(inventory, vec![delta]);

    assert_eq!(report.heat_at(location), HeatAmount::new(0.4).unwrap());
    assert_eq!(report.energy_at(location), EnergyAmount::zero());
}

#[test]
fn passive_reaction_cannot_credit_energy_buffer() {
    let location = ReactionLocation::Grid(GridCoord::new(0, 0));
    let mut inventory = ReactionInventory::new();
    inventory.set_resource(
        location,
        ResourceTypeId::from_raw(1),
        ResourceAmount::new(1.0).unwrap(),
    );
    let delta = ReactionDelta::builder(
        location,
        ReactionSource::Environment,
        ReactionId::from_raw(11),
        ReactionMode::Passive,
    )
    .inputs(vec![resource(1, 1.0)])
    .destinations(vec![ReactionDestination::Sink(resource(1, 1.0))])
    .energy_output(EnergyAmount::new(0.5).unwrap())
    .build();

    let report = AccountingReport::validate_and_commit(inventory, vec![delta]);

    assert_eq!(
        report.rejection(ReactionId::from_raw(11)),
        Some(&AccountingRejection::PassiveEnergyCreditForbidden)
    );
    assert_eq!(report.energy_at(location), EnergyAmount::zero());
}

#[test]
fn controlled_energy_requires_explicit_allowed_feasibility_token() {
    let location = ReactionLocation::Grid(GridCoord::new(0, 0));
    let source = ReactionSource::Cell(CellId::from_raw(4));
    let make_inventory = || {
        let mut inventory = ReactionInventory::new();
        inventory.set_resource(
            location,
            ResourceTypeId::from_raw(1),
            ResourceAmount::new(1.0).unwrap(),
        );
        inventory
    };
    let base = || {
        ReactionDelta::builder(
            location,
            source,
            ReactionId::from_raw(12),
            ReactionMode::Controlled,
        )
        .inputs(vec![resource(1, 1.0)])
        .destinations(vec![ReactionDestination::Sink(resource(1, 1.0))])
        .energy_output(EnergyAmount::new(0.8).unwrap())
    };

    let rejected = AccountingReport::validate_and_commit(make_inventory(), vec![base().build()]);
    assert_eq!(
        rejected.rejection(ReactionId::from_raw(12)),
        Some(&AccountingRejection::ControlledEnergyNotAllowed)
    );
    assert_eq!(rejected.energy_at(location), EnergyAmount::zero());
}
