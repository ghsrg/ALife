use alife::core::process::ProcessId;
use alife::core::tick::TickExecutor;
use alife::runner::config_parser::RawScenarioConfig;

fn config_with_genome_cadence(cadence: u64, seed: u64) -> alife::core::config::RuntimeConfig {
    RawScenarioConfig::parse(&format!(
        r#"
scenario_id = "scheduler_genome_cadence"
seed = {seed}
tick_count = 30
legacy_material_distribution = false

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [50.0]
optional_decay_rate = 0.0

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 1.0
metabolism_resource_per_tick = 0.5
energy_per_resource = 1.0
heat_per_resource = 0.0
waste_per_resource = 0.0

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = {{ nutrient_A = 5.0 }}
initial_materials = {{ boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 0.0, contractile = 0.0, sensory = 0.0 }}
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.01
capacity_limit = 40.0

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
critical_capacity_overrun = 50.0

[scheduler.cell]
genome_runtime_base_ticks = {cadence}
genome_runtime_ticks_per_layer = 1

[genome_templates.balanced]
variation_amplitude = 0.0
runtime_interval_ticks = {cadence}
regulatory_depth = 1

[genome_templates.balanced.carrier]
material_id = "genome_carrier_A"
amount = 1.0
integrity = 1.0

[genome_templates.balanced.outputs]
resource_uptake_priority = 0.8
energy_conversion_priority = 0.7
material_synthesis_priority = 0.1
division_preparation_priority = 0.0
repair_priority = 0.0
"#
    ))
    .unwrap()
}

#[test]
fn cached_action_plan_runs_between_genome_runtime_refreshes() {
    let config = config_with_genome_cadence(10, 0);
    let first_due_tick = 10 + config.initial_genome_runtime_offsets(1, 10)[0];
    let mut executor = TickExecutor::new(config).unwrap();

    for _ in 1..first_due_tick {
        let summary = executor.step().unwrap();
        assert_eq!(summary.metrics.genome_decision_refresh_count, 0);
        assert!(
            summary
                .diagnostics
                .attempt_order_by_process
                .contains(&ProcessId::LocalResourceUptake)
        );
    }

    let due = executor.step().unwrap();
    assert_eq!(due.metrics.genome_decision_refresh_count, 1);
    assert_eq!(due.diagnostics.genome_runtime_traces.len(), 1);
}

#[test]
fn genome_runtime_trace_is_not_emitted_before_due_tick() {
    let config = config_with_genome_cadence(10, 0);
    let first_due_tick = 10 + config.initial_genome_runtime_offsets(1, 10)[0];
    let mut executor = TickExecutor::new(config).unwrap();

    for _ in 1..first_due_tick {
        let summary = executor.step().unwrap();

        assert_eq!(summary.metrics.genome_decision_refresh_count, 0);
        assert!(summary.diagnostics.genome_runtime_traces.is_empty());
        assert!(
            summary
                .diagnostics
                .attempt_order_by_process
                .contains(&ProcessId::LocalResourceUptake)
        );
    }
}

#[test]
fn genome_runtime_refresh_waits_for_next_due_tick_after_refresh() {
    let cadence = 10;
    let config = config_with_genome_cadence(cadence, 0);
    let first_due_tick = cadence + config.initial_genome_runtime_offsets(1, cadence)[0];
    let mut executor = TickExecutor::new(config).unwrap();

    for _ in 1..first_due_tick {
        executor.step().unwrap();
    }

    let due = executor.step().unwrap();
    assert_eq!(due.metrics.genome_decision_refresh_count, 1);
    assert_eq!(due.diagnostics.genome_runtime_traces.len(), 1);

    for _ in 1..cadence {
        let summary = executor.step().unwrap();

        assert_eq!(summary.metrics.genome_decision_refresh_count, 0);
        assert!(summary.diagnostics.genome_runtime_traces.is_empty());
    }

    let next_due = executor.step().unwrap();
    assert_eq!(next_due.metrics.genome_decision_refresh_count, 1);
    assert_eq!(next_due.diagnostics.genome_runtime_traces.len(), 1);
}

#[test]
fn effective_genome_cadence_uses_template_override_and_regulatory_depth() {
    let config = RawScenarioConfig::parse(
        r#"
scenario_id = "scheduler_genome_effective_cadence"
seed = 1
tick_count = 1

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = { nutrient_A = 1.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 0.0, contractile = 0.0, sensory = 0.0 }
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.01
capacity_limit = 40.0

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
critical_capacity_overrun = 50.0

[scheduler.cell]
genome_runtime_base_ticks = 5
genome_runtime_ticks_per_layer = 3

[genome_templates.balanced]
variation_amplitude = 0.0
runtime_interval_ticks = 7
regulatory_depth = 3

[genome_templates.balanced.carrier]
material_id = "genome_carrier_A"
amount = 1.0
integrity = 1.0
"#,
    )
    .unwrap();

    assert_eq!(
        config
            .effective_genome_runtime_cadence_ticks("balanced")
            .unwrap(),
        13
    );
}

#[test]
fn genome_runtime_refresh_is_deterministically_staggered() {
    let config = config_with_genome_cadence(10, 0);
    let offsets = config.initial_genome_runtime_offsets(20, 10);

    assert_eq!(offsets, config.initial_genome_runtime_offsets(20, 10));
    assert!(offsets.iter().any(|offset| *offset != offsets[0]));
    assert!(offsets.iter().all(|offset| *offset < 10));
}
