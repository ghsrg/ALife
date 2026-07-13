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

fn cell_at(x: f32, structural: f32) -> CellInitialConfig {
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
        initial_structural_material: MaterialAmount::new(structural).unwrap(),
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
    let resource_interaction = ResourceInteractionConfig::disabled();
    let cell = cell_at(30.0, 2.0);
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
        resource_interaction,
        cell,
        environment,
        lifecycle,
    )
    .unwrap()
    .with_cells(vec![cell_at(30.0, 2.0), cell_at(33.9, 2.0)]);
    config.joints.enabled = true;
    config.joints.creation_material_cost = MaterialAmount::new(1.0).unwrap();
    config.joints.creation_resource_cost = ResourceAmount::zero();
    config.joints.creation_energy_cost = EnergyAmount::zero();
    config.joints.resource_transfer_rate = 0.0;
    config.local_interaction.enabled = true;
    config
}

#[test]
fn joint_creation_requires_local_contact_and_consumes_material_cost() {
    let mut exec = TickExecutor::new(two_touching_cells_config()).unwrap();
    let before_a = exec
        .world()
        .cells()
        .structural_material(CellIndex::from_raw(0))
        .raw();
    let before_b = exec
        .world()
        .cells()
        .structural_material(CellIndex::from_raw(1))
        .raw();

    let summary = exec.step().unwrap();

    assert_eq!(exec.world().joints().len(), 1);
    assert_eq!(summary.metrics.joint_created_count, 1);
    assert!(
        exec.world()
            .cells()
            .structural_material(CellIndex::from_raw(0))
            .raw()
            < before_a
    );
    assert!(
        exec.world()
            .cells()
            .structural_material(CellIndex::from_raw(1))
            .raw()
            < before_b
    );
}

#[test]
fn joint_creation_rejects_distant_or_material_free_cells() {
    let mut distant = two_touching_cells_config();
    distant.initial_cells[1].position = Position::new(50.0, 32.0);
    let mut exec = TickExecutor::new(distant).unwrap();
    let summary = exec.step().unwrap();
    assert_eq!(exec.world().joints().len(), 0);
    assert_eq!(summary.metrics.joint_creation_rejected_count, 0);

    let mut material_free = two_touching_cells_config();
    material_free.initial_cells[0].initial_structural_material = MaterialAmount::zero();
    let mut exec = TickExecutor::new(material_free).unwrap();
    let summary = exec.step().unwrap();
    assert_eq!(exec.world().joints().len(), 0);
    assert_eq!(summary.metrics.joint_creation_rejected_count, 1);
}
