use alife::core::genome::{
    GenomeOutputDisposition, GenomeOutputId, GenomeRuntimeInputs, GenomeRuntimeTrace,
    UnsupportedGenomeOutput,
};
use alife::core::ids::CellId;
use alife::core::process::{MaterialCapability, ProcessId};
use alife::core::units::Tick;
use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn canon_outputs_have_explicit_runtime_disposition() {
    let expected = [
        (
            "resource_uptake_priority",
            GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::LocalResourceUptake,
            },
        ),
        (
            "resource_export_priority",
            GenomeOutputDisposition::Deferred {
                reason: "resource export execution is not yet integrated into ActionPlan",
            },
        ),
        (
            "energy_conversion_priority",
            GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::MetabolismEnergyConversion,
            },
        ),
        (
            "material_synthesis_priority",
            GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::MaterialSynthesis,
            },
        ),
        (
            "repair_priority",
            GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::RepairBoundary,
            },
        ),
        (
            "signal_emit_priority",
            GenomeOutputDisposition::Deferred {
                reason: "signal emit execution is not yet integrated into ActionPlan",
            },
        ),
        (
            "movement_priority",
            GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::ContractileDisplacement,
            },
        ),
        (
            "division_preparation_priority",
            GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::GrowthResourceAllocation,
            },
        ),
        (
            "genome_copying_priority",
            GenomeOutputDisposition::EnabledNow {
                process_id: ProcessId::GenomeCopying,
            },
        ),
        (
            "division_partition_priority",
            GenomeOutputDisposition::Deferred {
                reason: "division partition execution is not yet integrated into ActionPlan",
            },
        ),
        (
            "dormancy_bias",
            GenomeOutputDisposition::Deferred {
                reason: "dormancy bias execution is not yet integrated into ActionPlan",
            },
        ),
        (
            "internal_rebalance_priority",
            GenomeOutputDisposition::Deferred {
                reason: "internal rebalance execution is not yet integrated into ActionPlan",
            },
        ),
    ];

    for (name, disposition) in expected {
        assert_eq!(GenomeOutputId::disposition_for(name), disposition, "{name}");
    }

    assert_eq!(
        GenomeOutputId::disposition_for("observer_fitness"),
        GenomeOutputDisposition::UnsupportedUntilRegistryChange(UnsupportedGenomeOutput {
            output_name: "observer_fitness",
        })
    );
}

#[test]
fn parser_accepts_only_enabled_runtime_outputs() {
    assert_eq!(
        GenomeOutputId::parse("resource_uptake_priority").unwrap(),
        GenomeOutputId::ResourceUptakePriority
    );

    let err = GenomeOutputId::parse("resource_export_priority").unwrap_err();
    assert!(
        err.to_string().contains("deferred"),
        "deferred canon outputs must be rejected until execution is implemented"
    );
}

#[test]
fn runtime_inputs_are_local_normalized_and_capability_masked() {
    let inputs = GenomeRuntimeInputs::new(
        0.25,
        2.0,
        8.0,
        [
            (MaterialCapability::ResourceUptake, true),
            (MaterialCapability::Metabolism, false),
        ],
    );

    assert_eq!(inputs.local_energy_level(), 0.25);
    assert_eq!(inputs.crowding_pressure(), 1.0);
    assert!(inputs.capability_available(MaterialCapability::ResourceUptake));
    assert!(!inputs.capability_available(MaterialCapability::Metabolism));
    assert!(!inputs.can_emit_to(ProcessId::MetabolismEnergyConversion));
}

#[test]
fn runtime_trace_is_debug_state_not_behavior_input() {
    let trace = GenomeRuntimeTrace::new(
        Tick::from_raw(7),
        CellId::from_raw(3),
        GenomeRuntimeInputs::new(0.5, 0.0, 4.0, []),
        [
            ("resource_uptake_priority", 0.8),
            ("energy_conversion_priority", -1.2),
        ],
        [
            ProcessId::LocalResourceUptake,
            ProcessId::MetabolismEnergyConversion,
        ],
        "metabolism rejected: missing capability",
    );

    assert_eq!(trace.tick(), Tick::from_raw(7));
    assert_eq!(trace.cell_id(), CellId::from_raw(3));
    assert_eq!(trace.outputs()[1].value(), -1.0);
    assert_eq!(
        trace.action_plan(),
        &[
            ProcessId::LocalResourceUptake,
            ProcessId::MetabolismEnergyConversion,
        ]
    );
    assert!(trace.feasibility_result().contains("missing capability"));
}

#[test]
fn tick_summary_includes_minimal_genome_runtime_trace_on_refresh() {
    let mut executor = alife::core::tick::TickExecutor::new(
        RawScenarioConfig::parse(
            r#"
scenario_id = "phase3b_runtime_trace"
seed = 9
tick_count = 3
legacy_material_distribution = false

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 1

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 1.0
metabolism_resource_per_tick = 1.0
energy_per_resource = 1.0
heat_per_resource = 0.0
waste_per_resource = 0.0

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = { nutrient_A = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 0.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 0.0, contractile = 0.0, sensory = 0.0 }
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.1
capacity_limit = 20.0

[cell.genome]
template = "balanced"

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.0
heat_warning_threshold = 20.0
heat_death_threshold = 40.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.0
waste_warning_threshold = 20.0
waste_death_threshold = 40.0

[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 5.0

[genome_templates.balanced]
variation_amplitude = 0.0
runtime_interval_ticks = 1

[genome_templates.balanced.carrier]
material_id = "genome_carrier_A"
amount = 1.0
integrity = 1.0

[genome_templates.balanced.outputs]
resource_uptake_priority = 0.8
energy_conversion_priority = 0.7
"#,
        )
        .unwrap(),
    )
    .unwrap();

    let summary = executor.step().unwrap();
    let trace = summary
        .diagnostics
        .genome_runtime_traces
        .first()
        .expect("genome refresh should emit a debug trace");

    assert_eq!(trace.tick(), Tick::from_raw(0));
    assert!(
        !trace
            .inputs()
            .can_emit_to(ProcessId::MetabolismEnergyConversion)
    );
    assert!(
        trace
            .outputs()
            .iter()
            .any(|output| output.output_id() == "resource_uptake_priority")
    );
    assert_eq!(
        trace.action_plan().first(),
        Some(&ProcessId::LocalResourceUptake)
    );
    assert_eq!(trace.feasibility_result(), "not_evaluated");
}
