use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::genome::{
    GenomeCarrierState, GenomeOutputId, GenomeOutputValue, GenomeTemplate, GenomeTemplateId,
};
use alife::core::process::ProcessId;
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn config_with_genome(outputs: Vec<(GenomeOutputId, f32)>) -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(16.0, 16.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(10.0).unwrap(),
        energy_capacity: EnergyAmount::new(20.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(1.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(3),
            seed: Seed::from_raw(42),
            size: WorldSize::new(32.0, 32.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: true,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(2.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
            energy_per_resource: 2.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
        cell,
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
    .unwrap();
    config.genome_templates.push(
        GenomeTemplate::new(
            GenomeTemplateId::new("balanced").unwrap(),
            0.0,
            1,
            GenomeCarrierState::new("genome_carrier_A".to_string(), 1.0, 1.0).unwrap(),
            outputs
                .into_iter()
                .map(|(id, value)| (id, GenomeOutputValue::new(value)))
                .collect(),
        )
        .unwrap(),
    );
    config.initial_cell_genome_templates =
        vec![Some(GenomeTemplateId::new("balanced").unwrap())];
    config
}

#[test]
fn genome_priority_changes_attempt_order_visible_in_diagnostics_trace() {
    let mut executor = TickExecutor::new(config_with_genome(vec![
        (GenomeOutputId::MaterialSynthesisPriority, 0.9),
        (GenomeOutputId::ResourceUptakePriority, 0.1),
    ]))
    .unwrap();

    let summary = executor.step().unwrap();

    assert_eq!(
        summary.diagnostics.attempt_order_by_process.get(0),
        Some(&ProcessId::MaterialSynthesis)
    );
    assert!(
        summary
            .diagnostics
            .attempt_order_by_process
            .contains(&ProcessId::LocalResourceUptake)
    );
}

#[test]
fn high_priority_missing_capability_is_still_rejected_by_feasibility() {
    let mut executor = TickExecutor::new(config_with_genome(vec![
        (GenomeOutputId::EnergyConversionPriority, 1.0),
        (GenomeOutputId::ResourceUptakePriority, 0.1),
    ]))
    .unwrap();

    let summary = executor.step().unwrap();
    let metabolism_rejections = summary
        .diagnostics
        .rejections_by_process
        .get(&ProcessId::MetabolismEnergyConversion)
        .copied()
        .unwrap_or(0);

    assert!(metabolism_rejections > 0);
}

#[test]
fn repair_priority_is_present_in_action_plan_trace_when_damage_exists() {
    let mut config = config_with_genome(vec![
        (GenomeOutputId::RepairPriority, 1.0),
        (GenomeOutputId::ResourceUptakePriority, 0.1),
    ]);
    config.chemistry.repair.enabled = true;
    config.chemistry.repair.energy_cost = 0.1;
    config.chemistry.repair.max_amount_per_tick = 0.5;
    config.cell.initial_repair_material = MaterialAmount::new(1.0).unwrap();
    config.cell.initial_boundary_material = MaterialAmount::new(1.0).unwrap();
    config.cell.initial_resource_amount = ResourceAmount::new(2.0).unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    let cell = alife::core::cell_store::CellIndex::from_raw(0);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_material_damage(cell, alife::core::materials::MaterialSlot::Boundary, 0.5);

    let summary = executor.step().unwrap();

    assert_eq!(
        summary.diagnostics.attempt_order_by_process.get(0),
        Some(&ProcessId::RepairBoundary)
    );
}
