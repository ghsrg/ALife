use alife::bootstrap::starter_state::{assign_initial_genomes, starter_energy_range};
use alife::core::config::{
    CellInitialConfig, ResourceConfig, ResourceInteractionConfig, RuntimeConfig, SpaceConfig,
    WorldConfig,
};
use alife::core::genome::{
    GenomeCarrierState, GenomeOutputId, GenomeOutputValue, GenomeTemplate, GenomeTemplateId,
};
use alife::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Seed, Tick,
    WasteAmount, WorldSize,
};

fn cell(energy: f32) -> CellInitialConfig {
    CellInitialConfig {
        position: Position::new(5.0, 5.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(energy).unwrap(),
        energy_capacity: EnergyAmount::new(20.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(0.1).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(1.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    }
}

fn template() -> GenomeTemplate {
    GenomeTemplate::new(
        GenomeTemplateId::new("balanced").unwrap(),
        0.1,
        1,
        GenomeCarrierState::new("genome_carrier_A".to_string(), 1.0, 1.0).unwrap(),
        vec![(
            GenomeOutputId::ResourceUptakePriority,
            GenomeOutputValue::new(0.5),
        )],
    )
    .unwrap()
}

fn runtime_with_genome() -> RuntimeConfig {
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(3),
            seed: Seed::from_raw(42),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 4.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        cell(8.0),
        alife::core::config::EnvironmentConfig {
            heat_current: alife::core::units::HeatAmount::zero(),
            heat_generated_per_tick: alife::core::units::HeatAmount::zero(),
            heat_dissipation_rate: alife::core::units::HeatAmount::zero(),
            heat_warning_threshold: alife::core::units::HeatAmount::new(20.0).unwrap(),
            heat_death_threshold: alife::core::units::HeatAmount::new(40.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(20.0).unwrap(),
            waste_death_threshold: WasteAmount::new(40.0).unwrap(),
        },
        alife::core::config::LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(1.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 0.1,
            critical_capacity_overrun: CapacityAmount::new(30.0).unwrap(),
        },
    )
    .unwrap();
    config.genome_templates = vec![template()];
    config.initial_cell_genome_templates = vec![Some(GenomeTemplateId::new("balanced").unwrap())];
    config
}

#[test]
fn starter_energy_range_reports_min_and_max() {
    let cells = vec![cell(5.0), cell(9.0)];

    assert_eq!(starter_energy_range(&cells), Some((5.0, 9.0)));
}

#[test]
fn initial_genome_assignment_uses_existing_bootstrap_logic() {
    let genomes = assign_initial_genomes(&runtime_with_genome()).unwrap();

    assert_eq!(genomes.len(), 1);
    assert_eq!(genomes[0].id.raw(), 1);
    assert_eq!(genomes[0].template_id.as_str(), "balanced");
}

#[test]
fn missing_genome_template_returns_stable_error() {
    let mut config = runtime_with_genome();
    config.genome_templates.clear();

    let err = assign_initial_genomes(&config).unwrap_err();
    assert_eq!(err.code(), "BOOTSTRAP_UNKNOWN_GENOME_TEMPLATE");
}
