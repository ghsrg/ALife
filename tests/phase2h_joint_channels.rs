use alife::core::cell_store::CellIndex;
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
fn joint_mechanical_constraint_limits_endpoint_separation() {
    let mut config = two_touching_cells_config();
    config.joints.mechanical_strength = 1.0;
    config.joints.creation_material_cost = MaterialAmount::new(0.1).unwrap();
    let mut exec = TickExecutor::new(config).unwrap();

    exec.step().unwrap();
    exec.world_mut()
        .cells_mut_for_commit()
        .set_position(CellIndex::from_raw(1), Position::new(45.0, 32.0));

    let summary = exec.step().unwrap();

    let a = exec.world().cells().position(CellIndex::from_raw(0));
    let b = exec.world().cells().position(CellIndex::from_raw(1));
    let dx = a.x() - b.x();
    let dy = a.y() - b.y();
    let distance = (dx * dx + dy * dy).sqrt();
    assert!(distance < 15.0);
    assert!(summary.metrics.joint_mechanical_correction_amount > 0.0);
}

#[test]
fn joint_resource_channel_moves_resource_down_gradient_and_conserves_total() {
    let mut config = two_touching_cells_config();
    config.joints.resource_transfer_rate = 0.5;
    config.joints.max_resource_transfer_per_tick = ResourceAmount::new(2.0).unwrap();
    config.initial_cells[0].initial_resource_amount = ResourceAmount::new(10.0).unwrap();
    config.initial_cells[1].initial_resource_amount = ResourceAmount::zero();
    config.initial_cells[1].capacity_limit = CapacityAmount::new(50.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    exec.step().unwrap();
    let before_total = exec
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(0))
        .raw()
        + exec
            .world()
            .cells()
            .resource_amount(CellIndex::from_raw(1))
            .raw();

    let summary = exec.step().unwrap();
    let after_a = exec
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(0))
        .raw();
    let after_b = exec
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(1))
        .raw();

    assert!(after_a < 10.0);
    assert!(after_b > 0.0);
    assert!(((after_a + after_b) - before_total).abs() < 0.0001);
    assert!(summary.metrics.joint_resource_transfer_amount > 0.0);
}

#[test]
fn joint_resource_channel_does_not_transfer_energy_buffer() {
    let mut config = two_touching_cells_config();
    config.joints.resource_transfer_rate = 0.5;
    config.initial_cells[0].initial_energy = EnergyAmount::new(20.0).unwrap();
    config.initial_cells[1].initial_energy = EnergyAmount::zero();

    let mut exec = TickExecutor::new(config).unwrap();
    exec.step().unwrap();
    exec.step().unwrap();

    assert_eq!(
        exec.world()
            .cells()
            .energy(CellIndex::from_raw(1))
            .current()
            .raw(),
        0.0
    );
}

#[test]
fn joint_signal_written_in_tick_n_is_readable_in_tick_n_plus_one() {
    let mut config = two_touching_cells_config();
    config.joints.signal_conductivity = 1.0;
    config.joints.signal_decay = 0.0;
    config.local_interaction.contact_stimulus_per_overlap = 1.0;
    config.initial_cells[0].initial_sensory_material = MaterialAmount::new(1.0).unwrap();
    config.initial_cells[1].initial_sensory_material = MaterialAmount::new(1.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    let first = exec.step().unwrap();
    assert_eq!(first.metrics.joint_signal_readable_total, 0.0);
    assert!(first.metrics.joint_signal_generated_total > 0.0);

    let second = exec.step().unwrap();
    assert!(second.metrics.joint_signal_readable_total > 0.0);
}

#[test]
fn joint_heat_channel_moves_temperature_without_energy_transfer() {
    let mut config = two_touching_cells_config();
    config.joints.heat_conductivity = 0.5;
    config.chemistry.heat.capacity = 1.0;
    config.initial_cells[0].initial_energy = EnergyAmount::new(10.0).unwrap();
    config.initial_cells[1].initial_energy = EnergyAmount::new(1.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    exec.step().unwrap();
    exec.world_mut().cells_mut_for_commit().set_temperature(
        CellIndex::from_raw(0),
        alife::core::units::Temperature::new(40.0),
    );
    exec.world_mut().cells_mut_for_commit().set_temperature(
        CellIndex::from_raw(1),
        alife::core::units::Temperature::new(20.0),
    );

    let before_energy_b = exec
        .world()
        .cells()
        .energy(CellIndex::from_raw(1))
        .current()
        .raw();
    let summary = exec.step().unwrap();

    assert!(
        exec.world()
            .cells()
            .temperature(CellIndex::from_raw(0))
            .raw()
            < 40.0
    );
    assert!(
        exec.world()
            .cells()
            .temperature(CellIndex::from_raw(1))
            .raw()
            > 20.0
    );
    assert_eq!(
        exec.world()
            .cells()
            .energy(CellIndex::from_raw(1))
            .current()
            .raw(),
        before_energy_b
    );
    assert!(summary.metrics.joint_heat_transfer_amount > 0.0);
}
