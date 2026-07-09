use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn base_phase2d_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(32.0, 32.0),
        radius: Radius::new(2.0).unwrap(),
        initial_energy: EnergyAmount::new(80.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(0.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(100.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(20.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(2.0).unwrap(),
        initial_transport_material: MaterialAmount::new(2.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(2.0).unwrap(),
        initial_storage_material: MaterialAmount::new(2.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(2.0).unwrap(),
        initial_structural_material: MaterialAmount::new(12.0).unwrap(),
        initial_repair_material: MaterialAmount::new(2.0).unwrap(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::new(2.0).unwrap(),
    };

    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(20),
            seed: Seed::from_raw(7),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(vec![ResourceAmount::zero()], 0.0).unwrap(),
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
            stress_energy_threshold: EnergyAmount::new(5.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(20.0).unwrap(),
        },
    )
    .unwrap();
    config.growth_enabled = true;
    config.growth.growth_target_radius = Radius::new(2.0).unwrap();
    config.growth.max_division_pressure = 0.5;
    config
}

#[test]
fn division_and_decomposition_configs_have_safe_defaults() {
    let config = base_phase2d_config();
    assert!(!config.division.enabled, "division must be explicit");
    assert!(
        !config.decomposition.enabled,
        "decomposition must be explicit"
    );
    assert_eq!(config.division.split_ratio, 0.5);
    assert_eq!(config.division.partition_loss_fraction, 0.0);
}

#[test]
fn division_and_decomposition_config_changes_affect_config_hash() {
    let base = base_phase2d_config();

    let mut division_changed = base.clone();
    division_changed.division.enabled = true;
    assert_ne!(base.config_hash(), division_changed.config_hash());

    let mut decomposition_changed = base.clone();
    decomposition_changed.decomposition.enabled = true;
    assert_ne!(base.config_hash(), decomposition_changed.config_hash());
}

#[test]
fn parser_loads_phase2d_division_and_decomposition_blocks() {
    let toml = r#"
scenario_id = "phase2d_parser"
seed = 7
tick_count = 10

[world]
size = [64.0, 64.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 4

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [0.0]
optional_decay_rate = 0.0

[cell]
initial_position = [32.0, 32.0]
radius = 2.0
initial_resources = { nutrient = 20.0 }
initial_materials = { boundary = 2.0, transport = 2.0, metabolic = 2.0, storage = 2.0, synthesis = 2.0, structural = 12.0, repair = 2.0, sensory = 2.0 }
initial_energy = 80.0
energy_capacity = 100.0
mandatory_cost_per_tick = 0.0
capacity_limit = 100.0

[environment]
ambient_temperature = 25.0
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
stress_energy_threshold = 5.0
dormancy_allowed = false
critical_capacity_overrun = 20.0

[growth]
growth_cost_resource = 2.0
growth_cost_energy = 1.0
growth_target_radius = 2.0
max_division_pressure = 0.5

[division]
enabled = true
energy_cost = 2.0
split_ratio = 0.5
daughter_spacing = 0.25
min_daughter_radius = 0.5
partition_loss_fraction = 0.0

[decomposition]
enabled = true
resource_layer_index = 0
resources_per_tick = 1.0
materials_per_tick = 1.0
remove_when_empty = false
"#;

    let config = alife::runner::config_parser::RawScenarioConfig::parse(toml).unwrap();
    assert!(config.division.enabled);
    assert!(config.decomposition.enabled);
    assert_eq!(config.division.energy_cost.raw(), 2.0);
    assert_eq!(config.decomposition.resources_per_tick.raw(), 1.0);
}

#[test]
fn cell_store_can_insert_partitioned_daughter_without_copying_runtime_flags() {
    use alife::core::cell_store::{CellIndex, InitialCellState, RuntimeFlags};
    use alife::core::tick::TickExecutor;

    let mut exec = TickExecutor::new(base_phase2d_config()).unwrap();
    let parent = CellIndex::from_raw(0);

    exec.world_mut().cells_mut_for_commit().set_runtime_flags(
        parent,
        RuntimeFlags {
            mandatory_paid: true,
            stalled: true,
            over_capacity: true,
            inert: false,
            division_ready: true,
        },
    );

    let daughter_state = InitialCellState {
        position: Position::new(34.0, 32.0),
        radius: Radius::new(1.0).unwrap(),
        energy: exec.world().cells().energy(parent),
        resources: ResourceAmount::new(5.0).unwrap(),
        boundary_material: MaterialAmount::new(1.0).unwrap(),
        transport_material: MaterialAmount::new(1.0).unwrap(),
        metabolic_material: MaterialAmount::new(1.0).unwrap(),
        storage_material: MaterialAmount::new(1.0).unwrap(),
        synthesis_material: MaterialAmount::new(1.0).unwrap(),
        structural_material: MaterialAmount::new(6.0).unwrap(),
        repair_material: MaterialAmount::new(1.0).unwrap(),
        contractile_material: MaterialAmount::zero(),
        sensory_material: MaterialAmount::new(1.0).unwrap(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        temperature: alife::core::units::Temperature::new(25.0),
    };

    let id = exec
        .world_mut()
        .cells_mut_for_commit()
        .insert_partitioned_daughter(daughter_state);
    let idx = exec.world().cells().resolve_id_cold(id).unwrap();
    assert_eq!(idx.raw(), 1);
    assert_eq!(
        exec.world().cells().runtime_flags(idx),
        RuntimeFlags::default()
    );
    assert_eq!(
        exec.world().cells().lifecycle_state(idx),
        alife::core::cell_store::LifecycleState::Alive
    );
}

#[test]
fn division_rejects_when_disabled_or_energy_cost_unpaid() {
    use alife::core::cell_store::CellIndex;
    use alife::core::process::{ActionCandidate, FeasibilityResult, ProcessId, RejectionReason};
    use alife::core::tick::TickExecutor;

    let mut disabled = base_phase2d_config();
    disabled.division.enabled = false;
    let exec = TickExecutor::new(disabled).unwrap();
    let candidate = ActionCandidate {
        process_id: ProcessId::Division,
        requested_amount: 0.0,
    };
    assert!(matches!(
        exec.world()
            .validate_feasibility(CellIndex::from_raw(0), &candidate),
        FeasibilityResult::Rejected(RejectionReason::ProcessDisabled)
    ));

    let mut low_energy = base_phase2d_config();
    low_energy.division.enabled = true;
    low_energy.division.energy_cost = EnergyAmount::new(90.0).unwrap();
    let exec = TickExecutor::new(low_energy).unwrap();
    assert!(matches!(
        exec.world()
            .validate_feasibility(CellIndex::from_raw(0), &candidate),
        FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
    ));
}

#[test]
fn division_allowed_when_ready_enabled_low_pressure_and_energy_available() {
    use alife::core::cell_store::CellIndex;
    use alife::core::process::{ActionCandidate, FeasibilityResult, ProcessId};
    use alife::core::tick::TickExecutor;

    let mut config = base_phase2d_config();
    config.division.enabled = true;
    config.division.energy_cost = EnergyAmount::new(2.0).unwrap();
    let exec = TickExecutor::new(config).unwrap();
    let candidate = ActionCandidate {
        process_id: ProcessId::Division,
        requested_amount: 0.0,
    };

    assert!(matches!(
        exec.world()
            .validate_feasibility(CellIndex::from_raw(0), &candidate),
        FeasibilityResult::Allowed {
            energy_cost: 2.0,
            ..
        }
    ));
}

#[test]
fn division_creates_two_daughters_and_partitions_accounted_state() {
    use alife::core::cell_store::CellIndex;
    use alife::core::process::{ActionCandidate, ProcessId};
    use alife::core::tick::TickExecutor;

    let mut config = base_phase2d_config();
    config.division.enabled = true;
    config.division.energy_cost = EnergyAmount::new(2.0).unwrap();
    config.division.split_ratio = 0.5;
    config.division.partition_loss_fraction = 0.0;

    let mut exec = TickExecutor::new(config).unwrap();
    let parent = CellIndex::from_raw(0);
    let before_energy = exec.world().cells().energy(parent).current().raw();
    let before_resources = exec.world().cells().resource_amount(parent).raw();
    let before_materials = exec.world().cells().total_materials(parent).raw();

    let outcome = exec
        .world_mut()
        .execute_division(
            parent,
            &ActionCandidate {
                process_id: ProcessId::Division,
                requested_amount: 1.0,
            },
        )
        .expect("division should execute");

    assert_eq!(exec.world().cells().len(), 2);
    assert_ne!(outcome.daughter_a_id, outcome.daughter_b_id);

    let a = outcome.daughter_a_index;
    let b = outcome.daughter_b_index;
    let after_energy = exec.world().cells().energy(a).current().raw()
        + exec.world().cells().energy(b).current().raw();
    let after_resources = exec.world().cells().resource_amount(a).raw()
        + exec.world().cells().resource_amount(b).raw();
    let after_materials = exec.world().cells().total_materials(a).raw()
        + exec.world().cells().total_materials(b).raw();

    assert!((after_energy - (before_energy - 2.0)).abs() < 0.001);
    assert!((after_resources - before_resources).abs() < 0.001);
    assert!((after_materials - before_materials).abs() < 0.001);
    assert_eq!(exec.world().cells().runtime_flags(a), Default::default());
    assert_eq!(exec.world().cells().runtime_flags(b), Default::default());
}

#[test]
fn tick_executor_divides_ready_cell_once_and_emits_events() {
    use alife::core::events::EventKind;
    use alife::core::tick::TickExecutor;

    let mut config = base_phase2d_config();
    config.division.enabled = true;
    config.division.energy_cost = EnergyAmount::new(2.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();

    // Make cell ready to divide by setting division_ready flag.
    let parent = alife::core::cell_store::CellIndex::from_raw(0);
    let mut flags = exec.world().cells().runtime_flags(parent);
    flags.division_ready = true;
    exec.world_mut()
        .cells_mut_for_commit()
        .set_runtime_flags(parent, flags);

    let summary = exec.step().unwrap();

    assert_eq!(exec.world().cells().len(), 2);
    assert_eq!(summary.metrics.divisions_count, 1);
    assert_eq!(summary.metrics.births_count, 1);
    assert!(
        exec.world()
            .events()
            .iter_ordered()
            .any(|event| event.kind == EventKind::CellDivided)
    );
    assert!(
        exec.world()
            .events()
            .iter_ordered()
            .any(|event| event.kind == EventKind::CellBorn)
    );
}

#[test]
fn daughters_do_not_divide_again_in_same_tick() {
    let mut config = base_phase2d_config();
    config.division.enabled = true;
    config.division.energy_cost = EnergyAmount::zero();
    config.division.min_daughter_radius = Radius::new(0.5).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();

    // Set parent cell ready to divide.
    let parent = alife::core::cell_store::CellIndex::from_raw(0);
    let mut flags = exec.world().cells().runtime_flags(parent);
    flags.division_ready = true;
    exec.world_mut()
        .cells_mut_for_commit()
        .set_runtime_flags(parent, flags);

    let _ = exec.step().unwrap();

    assert_eq!(
        exec.world().cells().len(),
        2,
        "division must use candidates collected before daughter insertion"
    );
}

#[test]
fn division_replay_is_deterministic_for_same_seed_and_config() {
    let mut config = base_phase2d_config();
    config.division.enabled = true;
    config.division.energy_cost = EnergyAmount::zero();

    let mut run_a = TickExecutor::new(config.clone()).unwrap();
    let mut run_b = TickExecutor::new(config).unwrap();

    let parent_a = alife::core::cell_store::CellIndex::from_raw(0);
    let mut flags_a = run_a.world().cells().runtime_flags(parent_a);
    flags_a.division_ready = true;
    run_a
        .world_mut()
        .cells_mut_for_commit()
        .set_runtime_flags(parent_a, flags_a);

    let parent_b = alife::core::cell_store::CellIndex::from_raw(0);
    let mut flags_b = run_b.world().cells().runtime_flags(parent_b);
    flags_b.division_ready = true;
    run_b
        .world_mut()
        .cells_mut_for_commit()
        .set_runtime_flags(parent_b, flags_b);

    let summary_a = run_a.step().unwrap();
    let summary_b = run_b.step().unwrap();

    assert_eq!(
        summary_a.metrics.divisions_count,
        summary_b.metrics.divisions_count
    );
    assert_eq!(
        summary_a.metrics.births_count,
        summary_b.metrics.births_count
    );
    assert_eq!(run_a.world().cells().len(), run_b.world().cells().len());
}
