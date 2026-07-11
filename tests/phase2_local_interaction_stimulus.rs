use alife::core::{
    cell_store::CellIndex,
    config::{
        CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
        ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
    },
    tick::TickExecutor,
    units::{
        CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius,
        ResourceAmount, Seed, Tick, WasteAmount, WorldSize,
    },
};

fn stimulus_cell(x: f32, sensory: f32) -> CellInitialConfig {
    CellInitialConfig {
        position: Position::new(x, 10.0),
        radius: Radius::new(2.0).unwrap(),
        initial_energy: EnergyAmount::new(20.0).unwrap(),
        energy_capacity: EnergyAmount::new(20.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::zero(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::new(sensory).unwrap(),
    }
}

fn contact_stimulus_config(receiver_sensory: f32) -> RuntimeConfig {
    let a = stimulus_cell(10.0, receiver_sensory);
    let b = stimulus_cell(13.0, receiver_sensory);
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(2),
            seed: Seed::from_raw(3),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::zero()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        a,
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
    .unwrap()
    .with_cells(vec![a, b]);
    config.local_interaction.enabled = true;
    config.local_interaction.contact_stimulus_per_overlap = 0.5;
    config.local_interaction.stimulus_decay_per_tick = 0.0;
    config
}

#[test]
fn contact_stimulus_created_in_tick_n_is_readable_in_tick_n_plus_1() {
    let mut exec = TickExecutor::new(contact_stimulus_config(1.0)).unwrap();

    assert_eq!(
        exec.world().cells().contact_stimulus(CellIndex::from_raw(0)),
        0.0
    );
    let first = exec.step().unwrap();
    assert_eq!(first.metrics.contact_stimulus_readable_total, 0.0);
    assert!(first.metrics.contact_stimulus_generated_total > 0.0);

    let second = exec.step().unwrap();
    assert!(
        exec.world().cells().contact_stimulus(CellIndex::from_raw(0)) > 0.0
    );
    assert!(
        exec.world().cells().contact_stimulus(CellIndex::from_raw(1)) > 0.0
    );
    assert!(second.metrics.contact_stimulus_readable_total > 0.0);
}

#[test]
fn contact_stimulus_requires_sensory_material() {
    let mut exec = TickExecutor::new(contact_stimulus_config(0.0)).unwrap();
    exec.step().unwrap();
    let second = exec.step().unwrap();

    assert_eq!(
        exec.world().cells().contact_stimulus(CellIndex::from_raw(0)),
        0.0
    );
    assert_eq!(
        exec.world().cells().contact_stimulus(CellIndex::from_raw(1)),
        0.0
    );
    assert_eq!(second.metrics.contact_stimulus_readable_total, 0.0);
}
