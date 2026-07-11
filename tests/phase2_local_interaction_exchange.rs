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
use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn parser_loads_local_interaction_config() {
    let toml = r#"
scenario_id = "local_interaction_parser"
seed = 7
tick_count = 10

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 1

[resources]
resource_type_ids = ["r"]
initial_distribution = [0.0]
optional_decay_rate = 0.0
passive_energy_income_placeholder = 0.0

[cell]
initial_position = [10.0, 10.0]
radius = 1.0
initial_resources = {"r" = 0.0}
initial_materials = {boundary = 1.0, transport = 1.0, metabolic = 0.0, storage = 0.0, synthesis = 0.0, structural = 1.0, repair = 0.0, contractile = 0.0, sensory = 1.0}
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.0
capacity_limit = 20.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.0
heat_warning_threshold = 100.0
heat_death_threshold = 200.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.0
waste_warning_threshold = 100.0
waste_death_threshold = 200.0

[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
dormant_mandatory_cost_modifier = 1.0
critical_capacity_overrun = 100.0

[resource_interaction]
enabled = false
uptake_layer_index = 0
max_uptake_per_tick = 0.0
metabolism_resource_per_tick = 0.0
energy_per_resource = 0.0
heat_per_resource = 0.0
waste_per_resource = 0.0

[local_interaction]
enabled = true
contact_exchange_rate = 0.5
max_exchange_per_pair = 2.0
min_boundary_capability = 0.1
min_transport_capability = 0.1
contact_stimulus_per_overlap = 0.25
stimulus_decay_per_tick = 0.5
"#;

    let config = RawScenarioConfig::parse(toml).unwrap();
    assert!(config.local_interaction.enabled);
    assert_eq!(config.local_interaction.contact_exchange_rate, 0.5);
    assert_eq!(config.local_interaction.max_exchange_per_pair.raw(), 2.0);
    assert_eq!(config.local_interaction.contact_stimulus_per_overlap, 0.25);
}

fn exchange_cell(x: f32, resources: f32, boundary: f32, transport: f32) -> CellInitialConfig {
    CellInitialConfig {
        position: Position::new(x, 10.0),
        radius: Radius::new(2.0).unwrap(),
        initial_energy: EnergyAmount::new(20.0).unwrap(),
        energy_capacity: EnergyAmount::new(20.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(resources).unwrap(),
        initial_boundary_material: MaterialAmount::new(boundary).unwrap(),
        initial_transport_material: MaterialAmount::new(transport).unwrap(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    }
}

fn contact_exchange_config(
    boundary_a: f32,
    transport_a: f32,
    boundary_b: f32,
    transport_b: f32,
) -> RuntimeConfig {
    let a = exchange_cell(10.0, 10.0, boundary_a, transport_a);
    let b = exchange_cell(13.0, 0.0, boundary_b, transport_b);
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(1),
            seed: Seed::from_raw(2),
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
    config.local_interaction.contact_exchange_rate = 0.5;
    config.local_interaction.max_exchange_per_pair = ResourceAmount::new(2.0).unwrap();
    config.local_interaction.min_boundary_capability = 0.1;
    config.local_interaction.min_transport_capability = 0.1;
    config
}

#[test]
fn passive_contact_exchange_moves_resources_down_gradient_and_conserves_total() {
    let mut exec = TickExecutor::new(contact_exchange_config(1.0, 1.0, 1.0, 1.0)).unwrap();
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

    let a_after = exec
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(0))
        .raw();
    let b_after = exec
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(1))
        .raw();
    assert!(a_after < 10.0);
    assert!(b_after > 0.0);
    assert!(((a_after + b_after) - before_total).abs() < 0.0001);
    assert!(summary.metrics.contact_exchange_amount > 0.0);
    assert_eq!(summary.metrics.contact_exchange_pairs_count, 1);
}

#[test]
fn passive_contact_exchange_removes_only_what_target_capacity_accepts() {
    let mut config = contact_exchange_config(1.0, 1.0, 1.0, 1.0);
    config.initial_cells[1].capacity_limit = CapacityAmount::new(4.5).unwrap();
    let mut exec = TickExecutor::new(config).unwrap();

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

    let a_after = exec
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(0))
        .raw();
    let b_after = exec
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(1))
        .raw();
    assert!(summary.metrics.contact_exchange_amount > 0.0);
    assert!(summary.metrics.contact_exchange_amount < 2.0);
    assert!(((a_after + b_after) - before_total).abs() < 0.0001);
}

#[test]
fn local_interaction_metrics_are_deterministic_for_same_seed_and_config() {
    let mut config = contact_exchange_config(1.0, 1.0, 1.0, 1.0);
    config.local_interaction.contact_stimulus_per_overlap = 0.5;

    let mut first = TickExecutor::new(config.clone()).unwrap();
    let mut second = TickExecutor::new(config).unwrap();

    let first_tick_1 = first.step().unwrap();
    let second_tick_1 = second.step().unwrap();
    let first_tick_2 = first.step().unwrap();
    let second_tick_2 = second.step().unwrap();

    let metric_tuple = |summary: &alife::core::summary::RunSummary| {
        (
            summary.metrics.contact_pairs_count,
            summary.metrics.contact_pressure_pre_total.to_bits(),
            summary.metrics.contact_pressure_post_total.to_bits(),
            summary.metrics.contact_pressure_max_over_tick.to_bits(),
            summary.metrics.contact_exchange_amount.to_bits(),
            summary.metrics.contact_exchange_pairs_count,
        )
    };

    assert_eq!(metric_tuple(&first_tick_1), metric_tuple(&second_tick_1));
    assert_eq!(metric_tuple(&first_tick_2), metric_tuple(&second_tick_2));
}

#[test]
fn passive_contact_exchange_rejects_when_boundary_material_missing() {
    let mut exec = TickExecutor::new(contact_exchange_config(0.0, 1.0, 1.0, 1.0)).unwrap();
    let summary = exec.step().unwrap();

    let a_after = exec
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(0))
        .raw();
    let b_after = exec
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(1))
        .raw();

    assert_eq!(a_after, 10.0);
    assert_eq!(b_after, 0.0);
    assert_eq!(summary.metrics.contact_exchange_amount, 0.0);
    assert_eq!(summary.metrics.contact_exchange_rejections_no_capability, 1);
}

#[test]
fn passive_contact_exchange_rejects_when_transport_material_missing() {
    let mut exec = TickExecutor::new(contact_exchange_config(1.0, 0.0, 1.0, 1.0)).unwrap();
    let summary = exec.step().unwrap();

    assert_eq!(
        exec.world()
            .cells()
            .resource_amount(CellIndex::from_raw(0))
            .raw(),
        10.0
    );
    assert_eq!(
        exec.world()
            .cells()
            .resource_amount(CellIndex::from_raw(1))
            .raw(),
        0.0
    );
    assert_eq!(summary.metrics.contact_exchange_amount, 0.0);
    assert_eq!(summary.metrics.contact_exchange_rejections_no_capability, 1);
}
