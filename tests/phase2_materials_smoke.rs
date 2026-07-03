use alife::core::process::{MaterialCapability, MaterialCapabilityFlags};

#[test]
fn test_all_11_capabilities_exist() {
    let capabilities = [
        MaterialCapability::BoundaryPermeability,
        MaterialCapability::ResourceUptake,
        MaterialCapability::Metabolism,
        MaterialCapability::StorageCapacity,
        MaterialCapability::MaterialSynthesis,
        MaterialCapability::StructuralGrowth,
        MaterialCapability::Repair,
        MaterialCapability::Contractility,
        MaterialCapability::ResourceSensing,
        MaterialCapability::PressureSensing,
        MaterialCapability::DamageSensing,
    ];

    assert_eq!(capabilities.len(), 11);
}

#[test]
fn test_material_capability_flags_support_all_11() {
    let flags = MaterialCapabilityFlags {
        boundary_permeability: true,
        resource_uptake: true,
        metabolism: true,
        storage_capacity: true,
        material_synthesis: true,
        structural_growth: true,
        repair: true,
        contractility: true,
        resource_sensing: true,
        pressure_sensing: true,
        damage_sensing: true,
    };

    assert!(flags.has(MaterialCapability::MaterialSynthesis));
    assert!(flags.has(MaterialCapability::Contractility));
    assert!(flags.has(MaterialCapability::ResourceSensing));
    assert!(flags.has(MaterialCapability::PressureSensing));
    assert!(flags.has(MaterialCapability::DamageSensing));
}

#[test]
fn capability_depends_on_specific_material_amount() {
    use alife::core::cell_store::{CellStore, EnergyBuffer, InitialCellState};
    use alife::core::units::{
        CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
    };

    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(5.0).unwrap(),
            EnergyAmount::new(10.0).unwrap(),
        ),
        resources: ResourceAmount::zero(),
        boundary_material: MaterialAmount::zero(),
        transport_material: MaterialAmount::zero(),
        metabolic_material: MaterialAmount::zero(),
        storage_material: MaterialAmount::zero(),
        synthesis_material: MaterialAmount::zero(),
        structural_material: MaterialAmount::zero(),
        repair_material: MaterialAmount::zero(),
        contractile_material: MaterialAmount::zero(),
        sensory_material: MaterialAmount::zero(),
        capacity_limit: CapacityAmount::new(10.0).unwrap(),
        temperature: Temperature::new(25.0),
    });

    let idx = alife::core::cell_store::CellIndex::from_raw(0);

    // Initial state: no materials -> no capabilities.
    assert!(!cells.has_capability(idx, MaterialCapability::ResourceUptake));
    assert!(!cells.has_capability(idx, MaterialCapability::Metabolism));

    // Set transport material -> ResourceUptake becomes true
    cells.set_transport_material(idx, MaterialAmount::new(1.5).unwrap());
    assert_eq!(cells.transport_material(idx).raw(), 1.5);
    assert!(cells.has_capability(idx, MaterialCapability::ResourceUptake));
    assert!(!cells.has_capability(idx, MaterialCapability::Metabolism));

    // Set metabolic material -> Metabolism becomes true
    cells.set_metabolic_material(idx, MaterialAmount::new(2.0).unwrap());
    assert_eq!(cells.metabolic_material(idx).raw(), 2.0);
    assert!(cells.has_capability(idx, MaterialCapability::Metabolism));

    // Verify total_materials() sums them
    // transport (1.5) + metabolic (2.0) = 3.5
    assert_eq!(cells.total_materials(idx).raw(), 3.5);
}

#[test]
fn test_legacy_config_backward_compatibility() {
    use alife::core::cell_store::CellIndex;
    use alife::core::process::MaterialCapability;
    use alife::core::tick::TickExecutor;
    use alife::runner::config_parser::RawScenarioConfig;

    let toml = r#"
scenario_id = "legacy_test"
seed = 42
tick_count = 10

[world]
size = [16.0, 16.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [10.0]

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = {}
initial_materials = { cell_wall = 9.0 }
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
"#;

    let config = RawScenarioConfig::parse(toml).unwrap();

    // Check that config fields have equally distributed materials (9.0 / 9.0 = 1.0 each)
    assert_eq!(config.cell.initial_boundary_material.raw(), 1.0);
    assert_eq!(config.cell.initial_transport_material.raw(), 1.0);
    assert_eq!(config.cell.initial_metabolic_material.raw(), 1.0);
    assert_eq!(config.cell.initial_structural_material.raw(), 1.0);

    let exec = TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);

    // Legacy scenario has non-zero total materials -> all 11 capabilities should be true!
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::BoundaryPermeability)
    );
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::ResourceUptake)
    );
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::Metabolism)
    );
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::StorageCapacity)
    );
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::MaterialSynthesis)
    );
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::StructuralGrowth)
    );
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::Repair)
    );
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::Contractility)
    );
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::ResourceSensing)
    );
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::PressureSensing)
    );
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::DamageSensing)
    );
}

#[test]
fn test_specific_named_materials_parsing() {
    use alife::core::cell_store::CellIndex;
    use alife::core::process::MaterialCapability;
    use alife::core::tick::TickExecutor;
    use alife::runner::config_parser::RawScenarioConfig;

    let toml = r#"
scenario_id = "specific_materials_test"
seed = 42
tick_count = 10

[world]
size = [16.0, 16.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [10.0]

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = {}
initial_materials = { boundary = 3.5, transport = 2.0, metabolic = 1.0 }
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
"#;

    let config = RawScenarioConfig::parse(toml).unwrap();

    // Check that config fields parsed specific materials correctly
    assert_eq!(config.cell.initial_boundary_material.raw(), 3.5);
    assert_eq!(config.cell.initial_transport_material.raw(), 2.0);
    assert_eq!(config.cell.initial_metabolic_material.raw(), 1.0);

    // Unspecified specific materials should remain zero
    assert_eq!(config.cell.initial_structural_material.raw(), 0.0);

    let exec = TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);

    // Capabilities corresponding to specified materials should be true
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::BoundaryPermeability)
    );
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::ResourceUptake)
    );
    assert!(
        exec.world()
            .cells()
            .has_capability(idx, MaterialCapability::Metabolism)
    );

    // Capabilities corresponding to unspecified materials should be false
    assert!(
        !exec
            .world()
            .cells()
            .has_capability(idx, MaterialCapability::StructuralGrowth)
    );
}
