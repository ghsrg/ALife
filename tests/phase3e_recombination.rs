use alife::core::cell_store::CellIndex;
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, GenomeCopyingConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::genome::{
    GenomeCarrierState, GenomeId, GenomeOutputId, GenomeOutputValue, GenomeState, GenomeTemplateId,
};
use alife::core::process::{ActionCandidate, FeasibilityResult, ProcessId, RejectionReason};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount,
};

fn base_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(16.0, 16.0),
        radius: Radius::new(1.5).unwrap(),
        initial_energy: EnergyAmount::new(20.0).unwrap(),
        energy_capacity: EnergyAmount::new(30.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        initial_resource_amount: ResourceAmount::new(20.0).unwrap(),
        capacity_limit: CapacityAmount::new(40.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: alife::core::units::Tick::from_raw(1),
            seed: alife::core::units::Seed::from_raw(1),
            size: alife::core::units::WorldSize::new(100.0, 100.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: true,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
            energy_per_resource: 1.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
        cell,
        EnvironmentConfig {
            heat_current: alife::core::units::HeatAmount::zero(),
            heat_generated_per_tick: alife::core::units::HeatAmount::zero(),
            heat_dissipation_rate: alife::core::units::HeatAmount::new(0.1).unwrap(),
            heat_warning_threshold: alife::core::units::HeatAmount::new(20.0).unwrap(),
            heat_death_threshold: alife::core::units::HeatAmount::new(40.0).unwrap(),
            waste_current: alife::core::units::WasteAmount::zero(),
            waste_generated_per_tick: alife::core::units::WasteAmount::zero(),
            waste_sink_rate: alife::core::units::WasteAmount::new(0.1).unwrap(),
            waste_warning_threshold: alife::core::units::WasteAmount::new(20.0).unwrap(),
            waste_death_threshold: alife::core::units::WasteAmount::new(40.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(2.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 0.5,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap();
    config.genome_copying = GenomeCopyingConfig {
        enabled: true,
        energy_cost_per_step: EnergyAmount::new(0.5).unwrap(),
        carrier_resource_cost_per_step: ResourceAmount::new(0.5).unwrap(),
        progress_per_step: 0.2,
        mutation_rate: 0.05,
        mutation_step: 0.1,
    };
    config
}

#[test]
fn test_genome_recombination_crossover() {
    let carrier = GenomeCarrierState::new("structural_material".to_string(), 1.0, 1.0).unwrap();
    let template = GenomeTemplateId::new("template_1").unwrap();

    let parent_a = GenomeState {
        id: GenomeId::from_raw(1),
        template_id: template.clone(),
        carrier: carrier.clone(),
        outputs: vec![
            (
                GenomeOutputId::ResourceUptakePriority,
                GenomeOutputValue::new(0.8),
            ),
            (
                GenomeOutputId::EnergyConversionPriority,
                GenomeOutputValue::new(0.2),
            ),
            (
                GenomeOutputId::GenomeRecombinationPriority,
                GenomeOutputValue::new(0.9),
            ),
        ],
    };

    let parent_b = GenomeState {
        id: GenomeId::from_raw(2),
        template_id: template,
        carrier,
        outputs: vec![
            (
                GenomeOutputId::ResourceUptakePriority,
                GenomeOutputValue::new(0.1),
            ),
            (
                GenomeOutputId::EnergyConversionPriority,
                GenomeOutputValue::new(0.9),
            ),
            (
                GenomeOutputId::GenomeRecombinationPriority,
                GenomeOutputValue::new(0.3),
            ),
        ],
    };

    // Mask 0b00000001: bit 0 is 1 (use partner_b for ResourceUptakePriority)
    let recombined = parent_a.recombine(&parent_b, GenomeId::from_raw(3), 0b00000001);

    assert_eq!(recombined.id.raw(), 3);
    assert_eq!(
        recombined
            .output(GenomeOutputId::ResourceUptakePriority)
            .unwrap()
            .raw(),
        0.1
    );
    assert_eq!(
        recombined
            .output(GenomeOutputId::EnergyConversionPriority)
            .unwrap()
            .raw(),
        0.2
    );
}

#[test]
fn test_recombination_feasibility_contact_requirement() {
    let executor = TickExecutor::new(base_config()).unwrap();
    let cell_1 = CellIndex::from_raw(0);

    let action = ActionCandidate {
        process_id: ProcessId::GenomeRecombination,
        requested_amount: 1.0,
    };

    // Single isolated cell without partner in contact -> MissingContactOrJoint
    let result = executor.world().validate_feasibility(cell_1, &action);
    assert_eq!(
        result,
        FeasibilityResult::Rejected(RejectionReason::MissingContactOrJoint)
    );
}
