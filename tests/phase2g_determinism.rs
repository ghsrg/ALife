use alife::core::config::{
    CellInitialConfig, ChemistryBoundaryConfig, ChemistryConfig, ChemistryHeatConfig,
    ChemistryReactionConfig, ChemistryRepairConfig, ChemistryResourceConfig, EnvironmentConfig,
    LifecycleConfig, ResourceConfig, ResourceInteractionConfig, RuntimeConfig, SpaceConfig,
    WorldConfig,
};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn probabilistic_passive_config(seed: u64) -> RuntimeConfig {
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
            seed: Seed::from_raw(seed),
            size: WorldSize::new(8.0, 8.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 1.0,
            physics_solver_iterations: 0,
        },
        ResourceConfig::new(
            vec![ResourceAmount::new(64.0).unwrap(), ResourceAmount::zero()],
            0.0,
        )
        .unwrap(),
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
                material_profile: None,
                material_capabilities:
                    alife::core::material_instance::MaterialCapabilityProfile::empty(),
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
                material_profile: None,
                material_capabilities:
                    alife::core::material_instance::MaterialCapabilityProfile::empty(),
            },
        ],
        materials: Vec::new(),
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
            rate: 1.0,
            probability: 0.5,
            accounting_destination: "waste_A".to_string(),
            material_output: None,
        }],
        heat: ChemistryHeatConfig {
            capacity: 10.0,
            dissipation_rate: 0.0,
            warning_threshold: 100.0,
            death_threshold: 200.0,
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

fn run_metrics(seed: u64) -> (u32, u32, f32, f32, f32) {
    let mut executor = TickExecutor::new(probabilistic_passive_config(seed)).unwrap();
    let summary = executor.step().unwrap();
    (
        summary.metrics.reaction_matched_count,
        summary.metrics.reaction_executed_count,
        summary.metrics.reaction_input_amount,
        summary.metrics.reaction_output_amount,
        summary.metrics.reaction_accounting_error,
    )
}

#[test]
fn same_seed_config_and_ticks_reproduce_all_phase2g_metrics() {
    assert_eq!(run_metrics(7), run_metrics(7));
}

#[test]
fn different_seed_changes_only_probability_sampling_not_accounting_rules() {
    let a = run_metrics(7);
    let b = run_metrics(8);

    assert_ne!(a.1, b.1);
    assert_eq!(a.4, 0.0);
    assert_eq!(b.4, 0.0);
    assert_eq!(a.2, a.3);
    assert_eq!(b.2, b.3);
}
