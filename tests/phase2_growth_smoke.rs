use alife::runner::config_parser::RawScenarioConfig;

fn base_scenario_toml() -> &'static str {
    r#"
scenario_id = "growth_test"
seed = 42
tick_count = 100

[world]
size = [16.0, 16.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = {}
initial_materials = { structural = 5.0 }
initial_energy = 5.0
energy_capacity = 10.0
mandatory_cost_per_tick = 2.0
capacity_limit = 20.0

[environment]
ambient_temperature = 25.0
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.0
heat_warning_threshold = 10.0
heat_death_threshold = 20.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.0
waste_warning_threshold = 10.0
waste_death_threshold = 20.0

[lifecycle]
stress_energy_threshold = 2.0
dormancy_allowed = true
critical_capacity_overrun = 5.0

[growth]
growth_cost_resource = 2.0
growth_cost_energy = 1.5
growth_target_radius = 2.0
max_division_pressure = 0.5
"#
}

#[test]
fn parser_loads_growth_and_division_config() {
    let toml = base_scenario_toml();
    let config = RawScenarioConfig::parse(toml).unwrap();
    assert_eq!(config.growth.growth_cost_resource.raw(), 2.0);
    assert_eq!(config.growth.growth_target_radius.raw(), 2.0);
}

#[test]
fn cells_accumulate_contact_pressure_during_collisions() {
    use alife::core::cell_store::CellIndex;
    use alife::core::config::CellInitialConfig;
    use alife::core::tick::TickExecutor;
    use alife::core::units::{Position, Radius};

    let base = RawScenarioConfig::parse(base_scenario_toml()).unwrap();
    // Set cell_1 and cell_2 overlapping
    let cell_1 = CellInitialConfig {
        position: Position::new(4.0, 4.0),
        radius: Radius::new(2.0).unwrap(),
        ..base.cell
    };
    let cell_2 = CellInitialConfig {
        position: Position::new(5.0, 4.0),
        radius: Radius::new(2.0).unwrap(),
        ..base.cell
    };
    let config = base.with_cells(vec![cell_1, cell_2]);

    let mut exec = TickExecutor::new(config).unwrap();
    let _summary = exec.step().unwrap();

    let pressure_0 = exec
        .world()
        .cells()
        .contact_pressure(CellIndex::from_raw(0));
    assert!(pressure_0 > 0.0);
}

#[test]
fn structural_growth_increases_cell_radius_and_capacity() {
    use alife::core::cell_store::CellIndex;
    use alife::core::process::{ActionCandidate, ProcessId};
    use alife::core::tick::TickExecutor;
    use alife::core::units::{EnergyAmount, ResourceAmount};

    let mut config = RawScenarioConfig::parse(base_scenario_toml()).unwrap();
    config.resource_interaction.enabled = true;
    config.cell.initial_resource_amount = ResourceAmount::new(10.0).unwrap();
    config.cell.initial_energy = EnergyAmount::new(10.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);

    let initial_radius = exec.world().cells().radius(idx).raw();
    let initial_cap = exec.world().cells().capacity_limit(idx).raw();

    // Trigger growth process directly
    let candidate_growth = ActionCandidate {
        process_id: ProcessId::GrowthResourceAllocation,
        requested_amount: 1.0,
    };
    let res = exec
        .world_mut()
        .execute_growth_for_test(idx, &candidate_growth);
    assert!(res.is_ok());

    let final_radius = exec.world().cells().radius(idx).raw();
    let final_cap = exec.world().cells().capacity_limit(idx).raw();

    assert!(final_radius > initial_radius);
    assert!(final_cap > initial_cap);
}

#[test]
fn division_readiness_and_pressure_gating_work() {
    use alife::core::cell_store::CellIndex;
    use alife::core::process::{ActionCandidate, FeasibilityResult, ProcessId, RejectionReason};
    use alife::core::tick::TickExecutor;
    use alife::core::units::Radius;

    let mut config = RawScenarioConfig::parse(base_scenario_toml()).unwrap();
    config.growth.growth_target_radius = Radius::new(2.0).unwrap();
    config.growth.max_division_pressure = 0.5;

    let mut exec = TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);

    let candidate_division = ActionCandidate {
        process_id: ProcessId::Division,
        requested_amount: 0.0,
    };

    // 1. Division should be rejected because cell radius (1.0) is below target (2.0)
    let res = exec.world().validate_feasibility(idx, &candidate_division);
    assert!(matches!(
        res,
        FeasibilityResult::Rejected(RejectionReason::RadiusBelowTarget)
    ));

    // Set radius to target
    exec.world_mut()
        .cells_mut_for_commit()
        .set_radius(idx, Radius::new(2.0).unwrap());

    // 2. Division should now be feasible
    let res = exec.world().validate_feasibility(idx, &candidate_division);
    assert!(matches!(res, FeasibilityResult::Allowed { .. }));

    // Set high contact pressure
    exec.world_mut()
        .cells_mut_for_commit()
        .set_contact_pressure(idx, 1.0);

    // 3. Division should be rejected due to high pressure
    let res = exec.world().validate_feasibility(idx, &candidate_division);
    assert!(matches!(
        res,
        FeasibilityResult::Rejected(RejectionReason::PressureTooHigh)
    ));
}
