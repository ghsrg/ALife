use alife::core::process::ProcessId;
use alife::core::tick::TickExecutor;
use alife::core::{cell_store::CellIndex, materials::MaterialSlot};
use alife::runner::config_parser::RawScenarioConfig;

fn process_cadence_config() -> alife::core::config::RuntimeConfig {
    RawScenarioConfig::parse(
        r#"
scenario_id = "scheduler_process_cadence"
seed = 17
tick_count = 30
legacy_material_distribution = false

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [100.0]
optional_decay_rate = 0.0

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 1.0
metabolism_resource_per_tick = 0.2
energy_per_resource = 1.0
heat_per_resource = 0.0
waste_per_resource = 0.0

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = { nutrient_A = 10.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 0.0, sensory = 1.0 }
initial_energy = 20.0
energy_capacity = 40.0
mandatory_cost_per_tick = 0.01
capacity_limit = 60.0

[cell.genome]
template = "balanced"

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.1
heat_warning_threshold = 50.0
heat_death_threshold = 100.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.1
waste_warning_threshold = 50.0
waste_death_threshold = 100.0

[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 60.0

[scheduler.cell]
genome_runtime_base_ticks = 1
genome_runtime_ticks_per_layer = 1
simple_synthesis_ticks = 5
basic_repair_ticks = 10

[genome_templates.balanced]
variation_amplitude = 0.0
runtime_interval_ticks = 1
regulatory_depth = 1

[genome_templates.balanced.carrier]
material_id = "genome_carrier_A"
amount = 1.0
integrity = 1.0

[genome_templates.balanced.outputs]
resource_uptake_priority = 0.0
energy_conversion_priority = 0.0
material_synthesis_priority = 1.0
repair_priority = 1.0
"#,
    )
    .unwrap()
}

#[test]
fn material_synthesis_attempt_cadence_gates_atomic_attempts() {
    let mut executor = TickExecutor::new(process_cadence_config()).unwrap();

    for _ in 0..4 {
        let summary = executor.step().unwrap();
        assert!(
            !summary
                .diagnostics
                .attempt_order_by_process
                .contains(&ProcessId::MaterialSynthesis)
        );
    }

    let tick_5 = executor.step().unwrap();
    assert!(
        tick_5
            .diagnostics
            .attempt_order_by_process
            .contains(&ProcessId::MaterialSynthesis)
    );
}

#[test]
fn repair_attempt_cadence_gates_atomic_attempts() {
    let mut config = process_cadence_config();
    config.chemistry.repair.enabled = true;
    config.chemistry.repair.energy_cost = 0.0;
    config.chemistry.repair.max_amount_per_tick = 0.1;
    let mut executor = TickExecutor::new(config).unwrap();
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_material_damage(CellIndex::from_raw(0), MaterialSlot::Boundary, 0.5);

    for _ in 0..9 {
        let summary = executor.step().unwrap();
        assert!(
            !summary
                .diagnostics
                .attempt_order_by_process
                .contains(&ProcessId::RepairBoundary)
        );
    }

    let tick_10 = executor.step().unwrap();
    assert!(
        tick_10
            .diagnostics
            .attempt_order_by_process
            .contains(&ProcessId::RepairBoundary)
    );
}
