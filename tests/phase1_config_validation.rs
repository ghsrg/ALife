use alife::core::cell_store::{CellIndex, LifecycleState};
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::summary::{CollapseReason, RunSummary, SurvivalResult};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

#[derive(Clone, Copy, Debug)]
struct ScenarioExpectation {
    scenario_id: &'static str,
    expected_result: SurvivalResult,
    expected_reason: CollapseReason,
    expected_tick: u64,
    expected_lifecycle: LifecycleState,
}

#[derive(Clone, Copy, Debug)]
struct ScenarioRun {
    summary: RunSummary,
    lifecycle: LifecycleState,
}

#[allow(clippy::too_many_arguments)]
fn runtime_config(
    scenario_id: &str,
    tick_count: u64,
    passive_income: f32,
    initial_energy: f32,
    energy_capacity: f32,
    mandatory_cost: f32,
    dormant_modifier: f32,
    capacity_limit: f32,
    initial_resources_total: f32,
    initial_materials_total: f32,
    heat_current: f32,
    heat_generated: f32,
    heat_dissipation: f32,
    heat_warning: f32,
    heat_death: f32,
    waste_current: f32,
    waste_generated: f32,
    waste_sink: f32,
    waste_warning: f32,
    waste_death: f32,
    stress_energy_threshold: f32,
    dormancy_allowed: bool,
    critical_capacity_overrun: f32,
) -> RuntimeConfig {
    let _scenario_id_is_documentation_only = scenario_id;

    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(tick_count),
            seed: Seed::from_raw(42),
            size: WorldSize::new(512.0, 512.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(
            vec![
                ResourceAmount::new(10.0).unwrap(),
                ResourceAmount::new(5.0).unwrap(),
            ],
            0.01,
        )
        .unwrap(),
        ResourceInteractionConfig::disabled(),
        CellInitialConfig {
            position: Position::new(256.0, 256.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(initial_energy).unwrap(),
            energy_capacity: EnergyAmount::new(energy_capacity).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(mandatory_cost).unwrap(),
            passive_energy_income: EnergyAmount::new(passive_income).unwrap(),
            capacity_limit: CapacityAmount::new(capacity_limit).unwrap(),
            initial_resource_amount: ResourceAmount::new(initial_resources_total).unwrap(),
            initial_material_amount: MaterialAmount::new(initial_materials_total).unwrap(),
        },
        EnvironmentConfig {
            heat_current: HeatAmount::new(heat_current).unwrap(),
            heat_generated_per_tick: HeatAmount::new(heat_generated).unwrap(),
            heat_dissipation_rate: HeatAmount::new(heat_dissipation).unwrap(),
            heat_warning_threshold: HeatAmount::new(heat_warning).unwrap(),
            heat_death_threshold: HeatAmount::new(heat_death).unwrap(),
            waste_current: WasteAmount::new(waste_current).unwrap(),
            waste_generated_per_tick: WasteAmount::new(waste_generated).unwrap(),
            waste_sink_rate: WasteAmount::new(waste_sink).unwrap(),
            waste_warning_threshold: WasteAmount::new(waste_warning).unwrap(),
            waste_death_threshold: WasteAmount::new(waste_death).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(stress_energy_threshold).unwrap(),
            dormancy_allowed,
            dormant_mandatory_cost_modifier: dormant_modifier,
            critical_capacity_overrun: CapacityAmount::new(critical_capacity_overrun).unwrap(),
        },
    )
    .unwrap()
}

fn single_cell_survival_config() -> RuntimeConfig {
    runtime_config(
        "single_cell_survival",
        100,
        2.0,
        50.0,
        100.0,
        2.0,
        0.1,
        30.0,
        3.0,
        5.0,
        0.0,
        0.1,
        0.2,
        50.0,
        80.0,
        0.0,
        0.05,
        0.1,
        10.0,
        20.0,
        10.0,
        true,
        5.0,
    )
}

fn single_cell_starvation_config() -> RuntimeConfig {
    runtime_config(
        "single_cell_starvation",
        100,
        0.0,
        1.0,
        100.0,
        5.0,
        0.5,
        50.0,
        3.0,
        5.0,
        0.0,
        0.1,
        0.2,
        50.0,
        80.0,
        0.0,
        0.05,
        0.1,
        10.0,
        20.0,
        10.0,
        false,
        5.0,
    )
}

fn single_cell_dormancy_config() -> RuntimeConfig {
    runtime_config(
        "single_cell_dormancy",
        10,
        0.0,
        1.0,
        100.0,
        5.0,
        0.1,
        30.0,
        3.0,
        5.0,
        0.0,
        0.1,
        0.2,
        50.0,
        80.0,
        0.0,
        0.05,
        0.1,
        10.0,
        20.0,
        10.0,
        true,
        5.0,
    )
}

fn single_cell_heat_death_config() -> RuntimeConfig {
    runtime_config(
        "single_cell_heat_death",
        20,
        2.0,
        50.0,
        100.0,
        2.0,
        0.1,
        30.0,
        3.0,
        5.0,
        0.0,
        5.0,
        0.0,
        10.0,
        12.0,
        0.0,
        0.05,
        0.1,
        10.0,
        20.0,
        10.0,
        true,
        5.0,
    )
}

fn single_cell_waste_death_config() -> RuntimeConfig {
    runtime_config(
        "single_cell_waste_death",
        20,
        2.0,
        50.0,
        100.0,
        2.0,
        0.1,
        30.0,
        3.0,
        5.0,
        0.0,
        0.1,
        0.2,
        50.0,
        80.0,
        0.0,
        5.0,
        0.0,
        10.0,
        12.0,
        10.0,
        true,
        5.0,
    )
}

fn single_cell_over_capacity_config() -> RuntimeConfig {
    runtime_config(
        "single_cell_over_capacity",
        100,
        5.0,
        50.0,
        100.0,
        2.0,
        0.1,
        15.0,
        20.0,
        5.0,
        0.0,
        0.1,
        0.2,
        50.0,
        80.0,
        0.0,
        0.05,
        0.1,
        10.0,
        20.0,
        10.0,
        true,
        5.0,
    )
}

fn run(config: RuntimeConfig) -> ScenarioRun {
    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();
    let lifecycle = executor
        .world()
        .cells()
        .lifecycle_state(CellIndex::from_raw(0));
    ScenarioRun { summary, lifecycle }
}

fn assert_scenario(config: RuntimeConfig, expected: ScenarioExpectation) {
    let run = run(config);

    assert_eq!(
        run.summary.survival_result, expected.expected_result,
        "{} survival_result",
        expected.scenario_id
    );
    assert_eq!(
        run.summary.collapse_reason, expected.expected_reason,
        "{} collapse_reason",
        expected.scenario_id
    );
    assert_eq!(
        run.summary.tick.raw(),
        expected.expected_tick,
        "{} tick",
        expected.scenario_id
    );
    assert_eq!(
        run.lifecycle, expected.expected_lifecycle,
        "{} lifecycle",
        expected.scenario_id
    );
}

#[test]
fn current_survival_config_is_stable_in_rust() {
    assert_scenario(
        single_cell_survival_config(),
        ScenarioExpectation {
            scenario_id: "single_cell_survival",
            expected_result: SurvivalResult::Stable,
            expected_reason: CollapseReason::None,
            expected_tick: 100,
            expected_lifecycle: LifecycleState::Alive,
        },
    );
}

#[test]
fn current_starvation_config_collapses_in_rust() {
    assert_scenario(
        single_cell_starvation_config(),
        ScenarioExpectation {
            scenario_id: "single_cell_starvation",
            expected_result: SurvivalResult::Collapse,
            expected_reason: CollapseReason::MandatoryCostUnpaid,
            expected_tick: 1,
            expected_lifecycle: LifecycleState::Dead,
        },
    );
}

#[test]
fn current_dormancy_config_reaches_dormancy_then_depletes_energy_in_rust() {
    let mut executor = TickExecutor::new(single_cell_dormancy_config()).unwrap();

    let first = executor.step().unwrap();
    assert_eq!(first.survival_result, SurvivalResult::Fragile);
    assert_eq!(
        executor
            .world()
            .cells()
            .lifecycle_state(CellIndex::from_raw(0)),
        LifecycleState::Dormant
    );

    let second = executor.step().unwrap();
    assert_eq!(second.survival_result, SurvivalResult::Collapse);
    assert_eq!(second.collapse_reason, CollapseReason::EnergyDepleted);
}

#[test]
fn current_heat_death_config_collapses_in_rust() {
    assert_scenario(
        single_cell_heat_death_config(),
        ScenarioExpectation {
            scenario_id: "single_cell_heat_death",
            expected_result: SurvivalResult::Collapse,
            expected_reason: CollapseReason::HeatLimitExceeded,
            expected_tick: 3,
            expected_lifecycle: LifecycleState::Dead,
        },
    );
}

#[test]
fn current_waste_death_config_collapses_in_rust() {
    assert_scenario(
        single_cell_waste_death_config(),
        ScenarioExpectation {
            scenario_id: "single_cell_waste_death",
            expected_result: SurvivalResult::Collapse,
            expected_reason: CollapseReason::WasteLimitExceeded,
            expected_tick: 3,
            expected_lifecycle: LifecycleState::Dead,
        },
    );
}

#[test]
fn current_over_capacity_config_collapses_in_rust() {
    assert_scenario(
        single_cell_over_capacity_config(),
        ScenarioExpectation {
            scenario_id: "single_cell_over_capacity",
            expected_result: SurvivalResult::Collapse,
            expected_reason: CollapseReason::CapacityExceeded,
            expected_tick: 1,
            expected_lifecycle: LifecycleState::Dead,
        },
    );
}

#[test]
#[ignore = "manual result dump for worklog report"]
fn dump_current_phase1_rust_config_results() {
    let scenarios: [(&str, RuntimeConfig); 6] = [
        ("single_cell_survival", single_cell_survival_config()),
        ("single_cell_starvation", single_cell_starvation_config()),
        ("single_cell_dormancy", single_cell_dormancy_config()),
        ("single_cell_heat_death", single_cell_heat_death_config()),
        ("single_cell_waste_death", single_cell_waste_death_config()),
        (
            "single_cell_over_capacity",
            single_cell_over_capacity_config(),
        ),
    ];

    println!("scenario_id,survival_result,collapse_reason,tick,final_energy,heat,waste,lifecycle");
    for (scenario_id, config) in scenarios {
        let run = run(config);
        println!(
            "{},{:?},{:?},{},{:.3},{:.3},{:.3},{:?}",
            scenario_id,
            run.summary.survival_result,
            run.summary.collapse_reason,
            run.summary.tick.raw(),
            run.summary.metrics.final_energy,
            run.summary.metrics.heat,
            run.summary.metrics.waste,
            run.lifecycle
        );
    }
}

#[test]
fn native_toml_parser_loads_valid_scenarios() {
    use alife::runner::config_parser::RawScenarioConfig;
    use std::fs;

    let scenarios = [
        "single_cell_survival.toml",
        "single_cell_starvation.toml",
        "single_cell_dormancy.toml",
        "single_cell_heat_death.toml",
        "single_cell_waste_death.toml",
    ];

    for name in scenarios {
        let path = format!("tools/early-stability/scenarios/{}", name);
        let content =
            fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {}", path));
        let config_res = RawScenarioConfig::parse(&content);
        assert!(
            config_res.is_ok(),
            "Failed to parse {}: {:?}",
            name,
            config_res.err()
        );
    }
}

#[test]
fn native_toml_parser_loads_over_capacity_scenario() {
    use alife::runner::config_parser::RawScenarioConfig;
    use std::fs;

    let path = "tools/early-stability/scenarios/single_cell_over_capacity.toml";
    let content = fs::read_to_string(path).expect("Failed to read over-capacity scenario");
    let config_res = RawScenarioConfig::parse(&content);
    assert!(
        config_res.is_ok(),
        "Expected success for loading over-capacity configuration, but got error: {:?}",
        config_res.err()
    );
}

#[test]
fn parsed_over_capacity_toml_collapses_in_runtime() {
    use alife::runner::config_parser::RawScenarioConfig;
    use std::fs;

    let path = "tools/early-stability/scenarios/single_cell_over_capacity.toml";
    let content = fs::read_to_string(path).expect("Failed to read over-capacity scenario");
    let config =
        RawScenarioConfig::parse(&content).expect("Failed to parse over-capacity scenario");

    assert_scenario(
        config,
        ScenarioExpectation {
            scenario_id: "single_cell_over_capacity",
            expected_result: SurvivalResult::Collapse,
            expected_reason: CollapseReason::CapacityExceeded,
            expected_tick: 1,
            expected_lifecycle: LifecycleState::Dead,
        },
    );
}
