use alife::core::cell_store::CellIndex;
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, GrowthConfig, LifecycleConfig, MaterialEffectConfig,
    ResourceConfig, ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::process::{ActionCandidate, ProcessId};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};
use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn material_effects_parse_from_scenario_config() {
    let config = RawScenarioConfig::parse(MATERIAL_EFFECTS_TOML).unwrap();

    assert_eq!(config.material_effects.transport_uptake_per_unit, 2.0);
    assert_eq!(config.material_effects.metabolic_conversion_per_unit, 3.0);
    assert_eq!(config.material_effects.storage_capacity_per_unit, 4.0);
    assert_eq!(config.material_effects.structural_growth_per_unit, 5.0);
    assert_eq!(config.material_effects.contractile_force_per_unit, 6.0);
    assert_eq!(config.material_effects.sensory_input_per_unit, 7.0);
    assert_eq!(config.material_effects.boundary_retention_per_unit, 8.0);
    assert_eq!(config.material_effects.repair_stress_buffer_per_unit, 9.0);
}

#[test]
fn transport_material_scales_local_resource_uptake() {
    let low = run_one_tick_with_materials_and_capacity(
        MaterialProfile {
            transport: 1.0,
            metabolic: 0.0,
            storage: 0.0,
            structural: 0.0,
            contractile: 0.0,
        },
        10.0,
    );
    let high = run_one_tick_with_materials_and_capacity(
        MaterialProfile {
            transport: 3.0,
            metabolic: 0.0,
            storage: 0.0,
            structural: 0.0,
            contractile: 0.0,
        },
        10.0,
    );

    assert!(high.internal_resources > low.internal_resources);
    assert_eq!(low.internal_resources, 1.0);
    assert_eq!(high.internal_resources, 3.0);
}

#[test]
fn metabolic_material_scales_energy_conversion_rate() {
    let low = run_one_tick_with_materials(MaterialProfile {
        transport: 0.0,
        metabolic: 1.0,
        storage: 0.0,
        structural: 0.0,
        contractile: 0.0,
    });
    let high = run_one_tick_with_materials(MaterialProfile {
        transport: 0.0,
        metabolic: 3.0,
        storage: 0.0,
        structural: 0.0,
        contractile: 0.0,
    });

    assert!(high.energy > low.energy);
    assert_eq!(low.energy, 30.0);
    assert_eq!(high.energy, 50.0);
}

#[test]
fn storage_material_increases_effective_free_capacity() {
    let no_storage = run_one_tick_with_materials(MaterialProfile {
        transport: 3.0,
        metabolic: 0.0,
        storage: 0.0,
        structural: 0.0,
        contractile: 0.0,
    });
    let with_storage = run_one_tick_with_materials(MaterialProfile {
        transport: 3.0,
        metabolic: 0.0,
        storage: 2.0,
        structural: 0.0,
        contractile: 0.0,
    });

    assert!(with_storage.internal_resources > no_storage.internal_resources);
    assert_eq!(no_storage.internal_resources, 1.0);
    assert_eq!(with_storage.internal_resources, 3.0);
}

#[test]
fn structural_material_scales_growth_output() {
    let low = grow_once(1.0);
    let high = grow_once(3.0);

    assert!(high.radius > low.radius);
    assert_eq!(low.structural_material, 2.0);
    assert_eq!(high.structural_material, 6.0);
}

#[test]
fn contractile_material_scales_displacement_output() {
    let low = displacement_after_one_tick(1.0);
    let high = displacement_after_one_tick(3.0);

    assert!(high > low);
}

#[test]
fn sensory_material_changes_sensed_input_metric_without_command_behavior() {
    let low = run_sensory_profile(1.0);
    let high = run_sensory_profile(3.0);

    assert!(high > low);
}

#[test]
fn repair_material_has_explicit_placeholder_status() {
    let mut config = base_config(MaterialProfile {
        transport: 0.0,
        metabolic: 0.0,
        storage: 0.0,
        structural: 0.0,
        contractile: 0.0,
    });
    config.cell.initial_repair_material = MaterialAmount::new(1.0).unwrap();
    config.initial_cells[0].initial_repair_material = MaterialAmount::new(1.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    let summary = exec.run_until_configured_tick().unwrap();

    assert!(summary.metrics.repair_placeholder_available);
    assert!(
        summary
            .diagnostics
            .tool_limited_mechanisms
            .contains(&"repair".to_string())
    );
}

#[test]
fn material_profile_baseline_is_deterministic_for_10_plus_cells() {
    let text = std::fs::read_to_string(
        "config/scenarios/material_profiles/material_profile_baseline.toml",
    )
    .unwrap();
    let cfg_a = RawScenarioConfig::parse(&text).unwrap();
    let cfg_b = RawScenarioConfig::parse(&text).unwrap();

    assert!(cfg_a.initial_cells.len() >= 10);

    let mut a = TickExecutor::new(cfg_a).unwrap();
    let mut b = TickExecutor::new(cfg_b).unwrap();

    let summary_a = a.run_until_configured_tick().unwrap();
    let summary_b = b.run_until_configured_tick().unwrap();

    assert_eq!(summary_a.config_hash, summary_b.config_hash);
    assert_eq!(summary_a.survival_result, summary_b.survival_result);
    assert_eq!(summary_a.collapse_reason, summary_b.collapse_reason);
    assert_eq!(
        summary_a.metrics.alive_cells_count,
        summary_b.metrics.alive_cells_count
    );
    assert_eq!(
        summary_a.metrics.final_energy,
        summary_b.metrics.final_energy
    );
}

struct MaterialProfile {
    transport: f32,
    metabolic: f32,
    storage: f32,
    structural: f32,
    contractile: f32,
}

struct OneTickResult {
    internal_resources: f32,
    energy: f32,
}

fn run_one_tick_with_materials(profile: MaterialProfile) -> OneTickResult {
    run_one_tick_with_materials_and_capacity(profile, 4.0)
}

fn run_one_tick_with_materials_and_capacity(
    profile: MaterialProfile,
    capacity_limit: f32,
) -> OneTickResult {
    let mut config = base_config(profile);
    config.cell.capacity_limit = CapacityAmount::new(capacity_limit).unwrap();
    config.initial_cells[0].capacity_limit = CapacityAmount::new(capacity_limit).unwrap();
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.energy_per_resource = 10.0;
    config.material_effects.storage_capacity_per_unit = 2.0;

    let mut exec = TickExecutor::new(config).unwrap();
    exec.step().unwrap();
    let idx = CellIndex::from_raw(0);
    OneTickResult {
        internal_resources: exec.world().cells().resource_amount(idx).raw(),
        energy: exec.world().cells().energy(idx).current().raw(),
    }
}

fn grow_once(structural: f32) -> GrowthResult {
    let mut config = base_config(MaterialProfile {
        transport: 0.0,
        metabolic: 0.0,
        storage: 0.0,
        structural,
        contractile: 0.0,
    });
    config.growth_enabled = true;
    config.growth = GrowthConfig {
        growth_cost_resource: ResourceAmount::new(1.0).unwrap(),
        growth_cost_energy: EnergyAmount::new(1.0).unwrap(),
        growth_target_radius: Radius::new(10.0).unwrap(),
        max_division_pressure: 100.0,
    };

    let mut exec = TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);
    exec.world_mut()
        .execute_growth(
            idx,
            &ActionCandidate {
                process_id: ProcessId::GrowthResourceAllocation,
                requested_amount: 1.0,
            },
        )
        .unwrap();

    GrowthResult {
        radius: exec.world().cells().radius(idx).raw(),
        structural_material: exec.world().cells().structural_material(idx).raw(),
    }
}

struct GrowthResult {
    radius: f32,
    structural_material: f32,
}

fn displacement_after_one_tick(contractile: f32) -> f32 {
    let mut config = base_config(MaterialProfile {
        transport: 0.0,
        metabolic: 0.0,
        storage: 0.0,
        structural: 0.0,
        contractile,
    });
    config.contractility.force_factor = 0.1;
    config.material_effects.contractile_force_per_unit = 1.0;

    let mut cell_b = config.cell;
    cell_b.position = Position::new(2.5, 2.0);
    let cell_a = config.cell;
    config = config.with_cells(vec![cell_a, cell_b]);

    let mut exec = TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);
    exec.world_mut()
        .cells_mut_for_commit()
        .set_contact_pressure(idx, 1.0);
    let before = exec.world().cells().position(idx).x();
    exec.world_mut()
        .execute_displacement(idx)
        .expect("contractile displacement should be feasible");
    let after = exec.world().cells().position(idx).x();
    (after - before).abs()
}

fn run_sensory_profile(sensory: f32) -> f32 {
    let mut config = base_config(MaterialProfile {
        transport: 0.0,
        metabolic: 0.0,
        storage: 0.0,
        structural: 0.0,
        contractile: 0.0,
    });
    config.cell.initial_sensory_material = MaterialAmount::new(sensory).unwrap();
    config.initial_cells[0].initial_sensory_material = MaterialAmount::new(sensory).unwrap();
    config.material_effects.sensory_input_per_unit = 1.0;

    let mut exec = TickExecutor::new(config).unwrap();
    exec.run_until_configured_tick()
        .unwrap()
        .metrics
        .sensory_input_accumulated
}

fn base_config(profile: MaterialProfile) -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(2.0, 2.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(20.0).unwrap(),
        energy_capacity: EnergyAmount::new(50.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(4.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(
            if profile.metabolic > 0.0 || profile.structural > 0.0 {
                3.0
            } else {
                0.0
            },
        )
        .unwrap(),
        initial_boundary_material: MaterialAmount::zero(),
        initial_transport_material: MaterialAmount::new(profile.transport).unwrap(),
        initial_metabolic_material: MaterialAmount::new(profile.metabolic).unwrap(),
        initial_storage_material: MaterialAmount::new(profile.storage).unwrap(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(profile.structural).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::new(profile.contractile).unwrap(),
        initial_sensory_material: MaterialAmount::zero(),
    };
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(5),
            seed: Seed::from_raw(1),
            size: WorldSize::new(20.0, 20.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 5.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(100.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: true,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
            energy_per_resource: 0.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
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
            stress_energy_threshold: EnergyAmount::zero(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 0.0,
            critical_capacity_overrun: CapacityAmount::new(100.0).unwrap(),
        },
    )
    .unwrap();
    config.material_effects = MaterialEffectConfig::default();
    config
}

const MATERIAL_EFFECTS_TOML: &str = r#"
scenario_id = "material_effects_parse"
seed = 1
tick_count = 1

[world]
size = [16.0, 16.0]

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [10.0]

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = {}
initial_materials = { transport = 1.0, metabolic = 1.0 }
initial_energy = 5.0
energy_capacity = 10.0
mandatory_cost_per_tick = 0.0
capacity_limit = 20.0

[environment]
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
stress_energy_threshold = 0.0
dormancy_allowed = false
critical_capacity_overrun = 10.0

[material_effects]
transport_uptake_per_unit = 2.0
metabolic_conversion_per_unit = 3.0
storage_capacity_per_unit = 4.0
structural_growth_per_unit = 5.0
contractile_force_per_unit = 6.0
sensory_input_per_unit = 7.0
boundary_retention_per_unit = 8.0
repair_stress_buffer_per_unit = 9.0
"#;
