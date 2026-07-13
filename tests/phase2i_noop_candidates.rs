use alife::core::cell_store::CellIndex;
use alife::core::config::*;
use alife::core::materials::MaterialSlot;
use alife::core::process::ProcessId;
use alife::core::tick::TickExecutor;
use alife::core::units::*;

fn base_cell(position: Position) -> CellInitialConfig {
    CellInitialConfig {
        position,
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(30.0).unwrap(),
        energy_capacity: EnergyAmount::new(60.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(10.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(2.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    }
}

fn two_cell_config() -> RuntimeConfig {
    let cell = base_cell(Position::new(8.0, 8.0));
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(5),
            seed: Seed::from_raw(31),
            size: WorldSize::new(32.0, 32.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(50.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        cell,
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(100.0).unwrap(),
            heat_death_threshold: HeatAmount::new(200.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(100.0).unwrap(),
            waste_death_threshold: WasteAmount::new(200.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(1.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(100.0).unwrap(),
        },
    )
    .unwrap();
    config.initial_cells = vec![
        base_cell(Position::new(8.0, 8.0)),
        base_cell(Position::new(9.9, 8.0)),
    ];
    config.local_interaction.enabled = true;
    config.joints.enabled = true;
    config.joints.creation_material_cost = MaterialAmount::new(0.5).unwrap();
    config
}

#[test]
fn joint_create_is_not_attempted_again_after_active_joint_exists() {
    let mut executor = TickExecutor::new(two_cell_config()).unwrap();

    let first = executor.step().unwrap();
    let second = executor.step().unwrap();

    assert_eq!(first.metrics.joint_created_count, 1);
    assert_eq!(second.metrics.joint_created_count, 0);
    assert_eq!(second.metrics.joint_creation_rejected_count, 0);
}

#[test]
fn repair_candidate_is_not_emitted_without_boundary_damage() {
    let mut config = two_cell_config();
    config.chemistry.repair.enabled = true;
    config.chemistry.repair.energy_cost = 0.1;
    config.chemistry.repair.max_amount_per_tick = 0.25;
    let mut executor = TickExecutor::new(config).unwrap();

    let summary = executor.step().unwrap();

    assert_eq!(
        executor
            .world()
            .cells()
            .material_damage(CellIndex::from_raw(0), MaterialSlot::Boundary),
        0.0
    );
    assert_eq!(
        summary
            .diagnostics
            .attempts_by_process
            .get(&ProcessId::RepairBoundary)
            .copied()
            .unwrap_or(0),
        0
    );
}
