use alife::core::genome::{
    GenomeCarrierState, GenomeOutputId, GenomeOutputValue, GenomeTemplate, GenomeTemplateId,
};
use alife::core::{
    config::{
        CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
        ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
    },
    units::{
        CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius,
        ResourceAmount, Seed, Tick, WasteAmount, WorldSize,
    },
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

fn base_cell() -> CellInitialConfig {
    CellInitialConfig {
        position: Position::new(8.0, 8.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(10.0).unwrap(),
        energy_capacity: EnergyAmount::new(20.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(1.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(2.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::new(1.0).unwrap(),
        initial_sensory_material: MaterialAmount::new(1.0).unwrap(),
    }
}

fn base_runtime_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(5),
            seed: Seed::from_raw(42),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(4.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        base_cell(),
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(20.0).unwrap(),
            heat_death_threshold: HeatAmount::new(40.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(20.0).unwrap(),
            waste_death_threshold: WasteAmount::new(40.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(2.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap()
}

#[test]
fn runtime_config_defaults_to_no_genome_templates() {
    let config = base_runtime_config();

    assert!(config.genome_templates.is_empty());
    assert_eq!(config.initial_cell_genome_templates, vec![None]);
}

#[test]
fn runtime_config_hash_changes_when_genome_template_changes() {
    let config_a = base_runtime_config();
    let mut config_b = base_runtime_config();
    config_b.genome_templates.push(
        GenomeTemplate::new(
            GenomeTemplateId::new("balanced").unwrap(),
            0.08,
            1,
            GenomeCarrierState::new("genome_carrier_A".to_string(), 1.0, 1.0).unwrap(),
            vec![(
                GenomeOutputId::ResourceUptakePriority,
                GenomeOutputValue::new(0.7),
            )],
        )
        .unwrap(),
    );
    config_b.initial_cell_genome_templates =
        vec![Some(GenomeTemplateId::new("balanced").unwrap())];

    assert_ne!(config_a.config_hash(), config_b.config_hash());
}
