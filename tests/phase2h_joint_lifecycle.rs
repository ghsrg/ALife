use alife::core::cell_store::{CellIndex, LifecycleState, RuntimeFlags};
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn cell_at(x: f32) -> CellInitialConfig {
    CellInitialConfig {
        position: Position::new(x, 32.0),
        radius: Radius::new(2.0).unwrap(),
        initial_energy: EnergyAmount::new(20.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(5.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(2.0).unwrap(),
        initial_transport_material: MaterialAmount::new(2.0).unwrap(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(2.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    }
}

fn two_touching_cells_config() -> RuntimeConfig {
    let world = WorldConfig {
        tick_count: Tick::from_raw(10),
        seed: Seed::from_raw(7),
        size: WorldSize::new(64.0, 64.0).unwrap(),
    };
    let space = SpaceConfig {
        spatial_grid_size: 8.0,
        physics_solver_iterations: 4,
    };
    let resources = ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap();
    let cell = cell_at(30.0);
    let environment = EnvironmentConfig {
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
    };
    let lifecycle = LifecycleConfig {
        stress_energy_threshold: EnergyAmount::new(1.0).unwrap(),
        dormancy_allowed: false,
        dormant_mandatory_cost_modifier: 0.1,
        critical_capacity_overrun: CapacityAmount::new(50.0).unwrap(),
    };
    let mut config = RuntimeConfig::new(
        world,
        space,
        resources,
        ResourceInteractionConfig::disabled(),
        cell,
        environment,
        lifecycle,
    )
    .unwrap()
    .with_cells(vec![cell_at(30.0), cell_at(33.9)]);
    config.joints.enabled = true;
    config.joints.creation_material_cost = MaterialAmount::new(0.5).unwrap();
    config.local_interaction.enabled = true;
    config
}

#[test]
fn joint_degrades_and_breaks_deterministically() {
    let mut config = two_touching_cells_config();
    config.joints.creation_material_cost = MaterialAmount::new(1.5).unwrap();
    config.joints.upkeep_material_decay_per_tick = 0.6;
    config.joints.break_damage_threshold = 1.0;
    let mut exec = TickExecutor::new(config).unwrap();

    exec.step().unwrap();
    let first_joint = exec.world().joints().active_ids().next().unwrap();
    let first_material = exec
        .world()
        .joints()
        .material_amount(first_joint)
        .unwrap()
        .raw();

    let summary = exec.step().unwrap();

    assert!(summary.metrics.joint_degradation_amount > 0.0);
    assert!(
        exec.world()
            .joints()
            .material_amount(first_joint)
            .unwrap()
            .raw()
            < first_material
    );
    assert!(exec.world().joints().is_broken(first_joint).unwrap());
    assert_eq!(summary.metrics.joint_broken_count, 1);
}

#[test]
fn endpoint_death_disables_living_joint_channels_without_instant_material_loss() {
    let mut config = two_touching_cells_config();
    config.joints.resource_transfer_rate = 1.0;
    let mut exec = TickExecutor::new(config).unwrap();

    exec.step().unwrap();
    let joint = exec.world().joints().active_ids().next().unwrap();
    let material = exec.world().joints().material_amount(joint).unwrap().raw();
    exec.world_mut()
        .cells_mut_for_commit()
        .set_lifecycle_state(CellIndex::from_raw(0), LifecycleState::Dead);

    let summary = exec.step().unwrap();

    assert!(!exec.world().joints().is_active(joint).unwrap());
    assert_eq!(
        exec.world().joints().material_amount(joint).unwrap().raw(),
        material
    );
    assert_eq!(summary.metrics.joint_resource_transfer_amount, 0.0);
}

#[test]
fn division_breaks_external_joints_without_duplication() {
    let mut config = two_touching_cells_config();
    config.division.enabled = true;
    config.growth_enabled = true;
    config.growth.growth_target_radius = Radius::new(3.0).unwrap();
    config.growth.max_division_pressure = 100.0;
    config.division.energy_cost = EnergyAmount::zero();
    let mut exec = TickExecutor::new(config).unwrap();

    exec.step().unwrap();
    exec.world_mut()
        .cells_mut_for_commit()
        .set_radius(CellIndex::from_raw(0), Radius::new(3.0).unwrap());
    exec.world_mut().cells_mut_for_commit().set_runtime_flags(
        CellIndex::from_raw(0),
        RuntimeFlags {
            division_ready: true,
            ..Default::default()
        },
    );

    exec.step().unwrap();

    assert_eq!(exec.world().joints().len(), 1);
    assert_eq!(exec.world().joints().active_ids().count(), 0);
}
