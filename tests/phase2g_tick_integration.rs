use alife::core::config::{
    CellInitialConfig, ChemistryBoundaryConfig, ChemistryConfig, ChemistryHeatConfig,
    ChemistryMaterialConfig, ChemistryReactionConfig, ChemistryRepairConfig,
    ChemistryResourceConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::resources::ResourceLayerIndex;
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn passive_chemistry_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(4.0, 4.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(5.0).unwrap(),
        energy_capacity: EnergyAmount::new(10.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::zero(),
        initial_transport_material: MaterialAmount::zero(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::zero(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(2),
            seed: Seed::from_raw(7),
            size: WorldSize::new(8.0, 8.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 0,
        },
        ResourceConfig::new(
            vec![ResourceAmount::new(10.0).unwrap(), ResourceAmount::zero()],
            0.0,
        )
        .unwrap(),
        ResourceInteractionConfig::disabled(),
        cell,
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(10.0).unwrap(),
            heat_death_threshold: HeatAmount::new(20.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(10.0).unwrap(),
            waste_death_threshold: WasteAmount::new(20.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(1.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(10.0).unwrap(),
        },
    )
    .unwrap();
    config.chemistry = ChemistryConfig {
        resources: vec![
            ChemistryResourceConfig {
                id: "nutrient_A".to_string(),
                volume: 1.0,
                diffusion_rate: 0.0,
                energy_value: 0.0,
                decay_rate: 0.0,
                reactivity_profile: "reactive".to_string(),
                permeability: "passive".to_string(),
                tags: Vec::new(),
            },
            ChemistryResourceConfig {
                id: "waste_A".to_string(),
                volume: 1.0,
                diffusion_rate: 0.0,
                energy_value: 0.0,
                decay_rate: 0.0,
                reactivity_profile: "stable".to_string(),
                permeability: "blocked".to_string(),
                tags: Vec::new(),
            },
        ],
        materials: Vec::<ChemistryMaterialConfig>::new(),
        reactions: vec![ChemistryReactionConfig {
            id: "passive_decay".to_string(),
            mode: "passive".to_string(),
            process_id: None,
            inputs: vec![("nutrient_A".to_string(), 1.0)],
            required_materials: Vec::new(),
            outputs: vec![("waste_A".to_string(), 1.0)],
            configured_sink_amount: 0.0,
            energy_output: 0.0,
            heat_output: 0.0,
            rate: 0.1,
            probability: 1.0,
            accounting_destination: "waste_A".to_string(),
        }],
        heat: ChemistryHeatConfig {
            capacity: 10.0,
            dissipation_rate: 0.0,
            warning_threshold: 8.0,
            death_threshold: 10.0,
        },
        boundary: ChemistryBoundaryConfig {
            default_permeability: "blocked".to_string(),
            retention_rate: 1.0,
        },
        repair: ChemistryRepairConfig {
            enabled: false,
            energy_cost: 0.0,
            max_amount_per_tick: 0.0,
        },
    };
    config
}

#[test]
fn passive_reaction_runs_without_genome_and_commits_local_products() {
    let mut executor = TickExecutor::new(passive_chemistry_config()).unwrap();

    executor.step().unwrap();

    let coord = executor
        .world()
        .resources()
        .coord_for_position(Position::new(4.0, 4.0));
    assert_eq!(
        executor
            .world()
            .resources()
            .amount_at(ResourceLayerIndex::from_raw(0), coord)
            .unwrap(),
        ResourceAmount::new(9.9).unwrap()
    );
    assert_eq!(
        executor
            .world()
            .resources()
            .amount_at(ResourceLayerIndex::from_raw(1), coord)
            .unwrap(),
        ResourceAmount::new(0.1).unwrap()
    );
}

#[test]
fn zero_rate_reaction_is_not_executed_or_counted() {
    let mut config = passive_chemistry_config();
    config.chemistry.reactions[0].rate = 0.0;
    let mut executor = TickExecutor::new(config).unwrap();

    let summary = executor.step().unwrap();

    let coord = executor
        .world()
        .resources()
        .coord_for_position(Position::new(4.0, 4.0));
    assert_eq!(
        executor
            .world()
            .resources()
            .amount_at(ResourceLayerIndex::from_raw(0), coord)
            .unwrap(),
        ResourceAmount::new(10.0).unwrap()
    );
    assert_eq!(summary.metrics.reaction_executed_count, 0);
    assert_eq!(summary.metrics.reaction_input_amount, 0.0);
    assert_eq!(summary.metrics.reaction_output_amount, 0.0);
}

#[test]
fn material_type_decay_degrades_matching_material_slot() {
    let mut config = passive_chemistry_config();
    config.cell.initial_boundary_material = MaterialAmount::new(2.0).unwrap();
    config.initial_cells[0].initial_boundary_material = MaterialAmount::new(2.0).unwrap();
    config.chemistry.heat.warning_threshold = 100.0;
    config.chemistry.heat.death_threshold = 200.0;
    config.chemistry.reactions.clear();
    config.chemistry.materials = vec![ChemistryMaterialConfig {
        id: "boundary_polymer_A".to_string(),
        volume: 1.0,
        stability: 0.8,
        strength: 0.7,
        permeability: 0.1,
        energy_capacity: 0.0,
        decay_rate: 0.25,
        repair_resource: "nutrient_A".to_string(),
        repair_amount: 0.5,
    }];
    let mut executor = TickExecutor::new(config).unwrap();

    let summary = executor.step().unwrap();

    let index = alife::core::cell_store::CellIndex::from_raw(0);
    assert_eq!(
        executor.world().cells().boundary_material(index),
        MaterialAmount::new(1.5).unwrap()
    );
    assert_eq!(summary.metrics.material_degradation_amount, 0.5);
}

#[test]
fn controlled_reaction_requires_metabolic_capability_and_consumes_typed_input() {
    let mut config = passive_chemistry_config();
    config.cell.initial_metabolic_material = MaterialAmount::new(1.0).unwrap();
    config.chemistry.reactions.push(ChemistryReactionConfig {
        id: "controlled_conversion".to_string(),
        mode: "controlled".to_string(),
        process_id: Some("energy_conversion".to_string()),
        inputs: vec![("nutrient_A".to_string(), 1.0)],
        required_materials: Vec::new(),
        outputs: vec![("waste_A".to_string(), 1.0)],
        configured_sink_amount: 0.0,
        energy_output: 0.5,
        heat_output: 0.0,
        rate: 1.0,
        probability: 1.0,
        accounting_destination: "waste_A".to_string(),
    });
    let mut executor = TickExecutor::new(config).unwrap();
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_typed_resource_amount(
            alife::core::cell_store::CellIndex::from_raw(0),
            alife::core::ids::ResourceTypeId::from_raw(0),
            ResourceAmount::new(1.0).unwrap(),
        )
        .unwrap();

    executor.step().unwrap();

    let cell = executor.world().cells();
    let index = alife::core::cell_store::CellIndex::from_raw(0);
    assert_eq!(cell.resource_amount(index), ResourceAmount::zero());
    assert_eq!(
        cell.energy(index).current(),
        EnergyAmount::new(5.5).unwrap()
    );
}

#[test]
fn missing_catalyst_blocks_controlled_reaction() {
    let mut config = passive_chemistry_config();
    config.cell.initial_metabolic_material = MaterialAmount::new(1.0).unwrap();
    config.initial_cells[0].initial_metabolic_material = MaterialAmount::new(1.0).unwrap();
    config.chemistry.reactions.clear();
    config.chemistry.reactions.push(ChemistryReactionConfig {
        id: "controlled_conversion".to_string(),
        mode: "controlled".to_string(),
        process_id: Some("energy_conversion".to_string()),
        inputs: vec![("nutrient_A".to_string(), 1.0)],
        required_materials: vec![("boundary_polymer_A".to_string(), 1.0)],
        outputs: vec![("waste_A".to_string(), 1.0)],
        configured_sink_amount: 0.0,
        energy_output: 0.5,
        heat_output: 0.0,
        rate: 1.0,
        probability: 1.0,
        accounting_destination: "waste_A".to_string(),
    });
    let mut executor = TickExecutor::new(config).unwrap();
    let index = alife::core::cell_store::CellIndex::from_raw(0);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_typed_resource_amount(
            index,
            alife::core::ids::ResourceTypeId::from_raw(0),
            ResourceAmount::new(1.0).unwrap(),
        )
        .unwrap();

    let summary = executor.step().unwrap();

    assert_eq!(summary.metrics.reaction_executed_count, 0);
    assert_eq!(summary.metrics.reaction_rejected_count, 1);
    assert_eq!(
        executor
            .world()
            .cells()
            .typed_resource_amount(index, alife::core::ids::ResourceTypeId::from_raw(0))
            .unwrap(),
        ResourceAmount::new(1.0).unwrap()
    );
}

#[test]
fn controlled_reaction_heat_changes_only_local_cell_temperature() {
    let mut config = passive_chemistry_config();
    config.cell.initial_metabolic_material = MaterialAmount::new(1.0).unwrap();
    config.chemistry.reactions.push(ChemistryReactionConfig {
        id: "heated_conversion".to_string(),
        mode: "controlled".to_string(),
        process_id: Some("energy_conversion".to_string()),
        inputs: vec![("nutrient_A".to_string(), 1.0)],
        required_materials: Vec::new(),
        outputs: vec![("waste_A".to_string(), 1.0)],
        configured_sink_amount: 0.0,
        energy_output: 0.0,
        heat_output: 1.0,
        rate: 1.0,
        probability: 1.0,
        accounting_destination: "waste_A".to_string(),
    });
    let mut executor = TickExecutor::new(config).unwrap();
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_typed_resource_amount(
            alife::core::cell_store::CellIndex::from_raw(0),
            alife::core::ids::ResourceTypeId::from_raw(0),
            ResourceAmount::new(1.0).unwrap(),
        )
        .unwrap();

    executor.step().unwrap();

    assert_eq!(
        executor
            .world()
            .cells()
            .temperature(alife::core::cell_store::CellIndex::from_raw(0)),
        alife::core::units::Temperature::new(25.1)
    );
}

#[test]
fn local_reaction_heat_degrades_material_over_tolerance_in_tick() {
    let mut config = passive_chemistry_config();
    config.cell.initial_boundary_material = MaterialAmount::new(1.0).unwrap();
    config.cell.initial_metabolic_material = MaterialAmount::new(1.0).unwrap();
    config.chemistry.heat.warning_threshold = 25.05;
    config.chemistry.heat.death_threshold = 25.15;
    config.chemistry.reactions.push(ChemistryReactionConfig {
        id: "heated_conversion".to_string(),
        mode: "controlled".to_string(),
        process_id: Some("energy_conversion".to_string()),
        inputs: vec![("nutrient_A".to_string(), 1.0)],
        required_materials: Vec::new(),
        outputs: vec![("waste_A".to_string(), 1.0)],
        configured_sink_amount: 0.0,
        energy_output: 0.0,
        heat_output: 1.0,
        rate: 1.0,
        probability: 1.0,
        accounting_destination: "waste_A".to_string(),
    });
    let mut executor = TickExecutor::new(config).unwrap();
    let index = alife::core::cell_store::CellIndex::from_raw(0);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_typed_resource_amount(
            index,
            alife::core::ids::ResourceTypeId::from_raw(0),
            ResourceAmount::new(1.0).unwrap(),
        )
        .unwrap();

    executor.step().unwrap();

    let cells = executor.world().cells();
    assert!(
        (cells.material_damage(index, alife::core::materials::MaterialSlot::Boundary) - 0.5).abs()
            < 1e-5
    );
    assert!((cells.boundary_material(index).raw() - 0.5).abs() < 1e-5);
}

#[test]
fn passive_reaction_heat_changes_local_cell_temperature() {
    let mut config = passive_chemistry_config();
    config.chemistry.heat.capacity = 10.0;
    config.chemistry.heat.warning_threshold = 100.0;
    config.chemistry.heat.death_threshold = 200.0;
    config.chemistry.reactions[0].heat_output = 2.0;
    config.chemistry.reactions[0].rate = 1.0;
    let mut executor = TickExecutor::new(config).unwrap();

    executor.step().unwrap();

    let index = alife::core::cell_store::CellIndex::from_raw(0);
    assert!(
        executor.world().cells().temperature(index).raw() > 25.0,
        "passive reaction heat must be committed to local cell temperature"
    );
}

#[test]
fn material_fragments_convert_to_resources_after_identity_retention_tick() {
    let mut config = passive_chemistry_config();
    config.chemistry.reactions.clear();
    config.chemistry.heat.warning_threshold = 100.0;
    config.chemistry.heat.death_threshold = 200.0;
    config.decomposition.enabled = true;
    config.decomposition.resource_layer_index = 0;
    config.decomposition.resources_per_tick = ResourceAmount::zero();
    config.decomposition.materials_per_tick = MaterialAmount::new(1.0).unwrap();
    config.cell.initial_energy = EnergyAmount::zero();
    config.initial_cells[0].initial_energy = EnergyAmount::zero();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(1.0).unwrap();
    config.initial_cells[0].mandatory_cost_per_tick = EnergyAmount::new(1.0).unwrap();
    config.cell.initial_boundary_material = MaterialAmount::new(2.0).unwrap();
    config.initial_cells[0].initial_boundary_material = MaterialAmount::new(2.0).unwrap();
    let mut executor = TickExecutor::new(config).unwrap();

    let first_decomposition = executor.step().unwrap();
    assert!(first_decomposition.metrics.fragment_created_amount > 0.0);
    assert_eq!(first_decomposition.metrics.fragment_converted_amount, 0.0);
    assert!(executor.world().fragments().total_amount().raw() > 0.0);

    let second_decomposition = executor.step().unwrap();

    assert!(
        second_decomposition.metrics.fragment_converted_amount > 0.0,
        "fragment conversion should be an explicit later step, not direct material-to-resource release"
    );
}
