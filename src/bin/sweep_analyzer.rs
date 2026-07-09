#![allow(dead_code)]

use alife::core::cell_store::{CellIndex, LifecycleState};
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::summary::SurvivalResult;
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};
use alife::observer::{
    classifiers::{
        classify_behavior_profiles, classify_cell_roles_observed, classify_cell_roles_potential,
        ClassificationResult,
    },
    config::{
        BehaviorClassifierConfig, CellRoleClassifierConfig, load_behavior_profile_classifier,
        load_cell_role_classifier,
    },
    projection::{EntityType, extract_features},
};
use std::collections::HashMap;
use std::io::Write;

// ─────────────────────────────────────────────────────────────────────────────
// TOML Config Structs (deserialized from sweep_analyzer.toml)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize)]
pub struct AnalyzerConfig {
    pub run: RunConfig,
    pub cell: CellConfig,
    pub lifecycle: LifecycleRaw,
    pub resource_interaction: ResourceInteractionRaw,
    pub environment: EnvironmentRaw,
    pub sweep: Option<Vec<SweepDef>>,
    pub matrix: Option<Vec<MatrixDef>>,
    #[allow(dead_code)]
    pub scenarios: Option<std::collections::HashMap<String, RawScenarioPreset>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RunConfig {
    pub output_dir: String,
    pub seed: u64,
    pub ticks: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct CellConfig {
    pub radius: f32,
    pub initial_energy: f32,
    pub energy_capacity: f32,
    pub mandatory_cost_per_tick: f32,
    pub passive_energy_income: f32,
    pub capacity_limit: f32,
    pub initial_metabolic_material: f32,
    pub initial_transport_material: f32,
    pub initial_boundary_material: f32,
    pub initial_structural_material: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct LifecycleRaw {
    pub stress_energy_threshold: f32,
    pub dormancy_allowed: bool,
    pub dormant_mandatory_cost_modifier: f32,
    pub critical_capacity_overrun: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct ResourceInteractionRaw {
    pub energy_per_resource: f32,
    pub heat_per_resource: f32,
    pub waste_per_resource: f32,
    pub decay_rate: f32,
    pub default_resource_density: f32,
    pub default_max_uptake_per_tick: f32,
    pub default_metabolism_resource_per_tick: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct EnvironmentRaw {
    pub heat_dissipation_rate: f32,
    pub heat_warning_threshold: f32,
    pub heat_death_threshold: f32,
    pub waste_sink_rate: f32,
    pub waste_warning_threshold: f32,
    pub waste_death_threshold: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct SweepDef {
    pub name: String,
    pub param: String,
    pub from: f32,
    pub to: f32,
    pub steps: usize,
    pub scenario: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MatrixDef {
    pub name: String,
    pub param_x: String,
    pub from_x: f32,
    pub to_x: f32,
    pub steps_x: usize,
    pub param_y: String,
    pub from_y: f32,
    pub to_y: f32,
    pub steps_y: usize,
    pub scenario: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, Clone)]
pub struct RawScenarioPreset {
    pub world_size: Option<Vec<f32>>,
    pub initial_resources: Option<Vec<f32>>,
    pub decay_rate: Option<f32>,
    pub cell_position: Option<Vec<f32>>,
    pub cell_radius: Option<f32>,
    pub initial_energy: Option<f32>,
    pub energy_capacity: Option<f32>,
    pub mandatory_cost_per_tick: Option<f32>,
    pub passive_energy_income: Option<f32>,
    pub capacity_limit: Option<f32>,
    pub stress_energy_threshold: Option<f32>,
    pub dormancy_allowed: Option<bool>,
    pub dormant_mandatory_cost_modifier: Option<f32>,
    pub critical_capacity_overrun: Option<f32>,
    pub heat_dissipation_rate: Option<f32>,
    pub heat_warning_threshold: Option<f32>,
    pub heat_death_threshold: Option<f32>,
    pub waste_sink_rate: Option<f32>,
    pub waste_warning_threshold: Option<f32>,
    pub waste_death_threshold: Option<f32>,
    pub growth_enabled: Option<bool>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Single simulation run result
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SimResult {
    pub collapsed: bool,
    pub collapse_tick: Option<u32>,
    pub dormant_ticks: u32,
    pub active_ticks: u32,
    pub stressed_ticks: u32,
    pub min_energy: f32,
    pub max_energy: f32,
    pub final_energy: f32,
    pub mean_energy: f32,
    pub initial_energy: f32,
    pub death_reason: String,
    pub energy_produced: f32,
    pub passive_energy_received: f32,
    pub energy_spent_upkeep: f32,
    pub energy_spent_dormant_upkeep: f32,
    pub energy_spent_movement: f32,
    pub energy_spent_growth: f32,
    pub energy_spent_repair: f32,
    pub energy_spent_division: f32,
    pub initial_world_resource: f32,
    pub final_world_resource: f32,
    pub resource_regenerated: f32,
    pub resource_absorbed: f32,
    pub resource_metabolized: f32,
    pub internal_resource_final: f32,
    pub resource_released: f32,
    pub resource_explicit_sink: f32,
    pub resource_balance_error: f32,
    pub energy_balance_error: f32,
    pub dormancy_enter_count: u32,
    pub dormancy_exit_count: u32,
    pub ticks_executed: u32,
    pub total_resource_consumed: f32,
    pub metabolism_count: u32,
    pub potential_role: String,
    pub observed_role: String,
    pub behavior_profile: String,
    pub ticks_per_second: f32,
    pub bhv_res: Option<ClassificationResult>,
    pub explicit_energy_loss: f32,
    pub death_cleanup_loss_energy: f32,
    pub death_cleanup_loss_resources: f32,
    pub clamping_loss: f32,
    pub unpaid_mandatory_cost: f32,
    pub resource_decay: f32,
    pub resource_sink: f32,
    pub numerical_error_energy: f32,
    pub numerical_error_resources: f32,
    pub unclassified_loss_energy: f32,
    pub unclassified_loss_resources: f32,
}

// ─────────────────────────────────────────────────────────────────────────────
// Build RuntimeConfig, patching one or two parameters
// ─────────────────────────────────────────────────────────────────────────────

pub fn build_config(
    cfg: &AnalyzerConfig,
    preset: Option<&RawScenarioPreset>,
    overrides: &std::collections::HashMap<&str, f32>,
) -> RuntimeConfig {
    let cell_cfg = &cfg.cell;
    let lc = &cfg.lifecycle;
    let ri = &cfg.resource_interaction;
    let env = &cfg.environment;

    let (world_w, world_h) = preset
        .and_then(|p| p.world_size.as_ref())
        .map(|ws| (ws[0], ws[1]))
        .unwrap_or((512.0, 512.0));

    let (cell_x, cell_y) = preset
        .and_then(|p| p.cell_position.as_ref())
        .map(|cp| (cp[0], cp[1]))
        .unwrap_or((256.0, 256.0));

    let cell_radius_base = preset
        .and_then(|p| p.cell_radius)
        .unwrap_or(cell_cfg.radius);
    let cell_radius = overrides
        .get("cell_radius")
        .copied()
        .unwrap_or(cell_radius_base);

    let initial_energy_base = preset
        .and_then(|p| p.initial_energy)
        .unwrap_or(cell_cfg.initial_energy);
    let initial_energy = overrides
        .get("initial_energy")
        .copied()
        .unwrap_or(initial_energy_base);

    let energy_capacity_base = preset
        .and_then(|p| p.energy_capacity)
        .unwrap_or(cell_cfg.energy_capacity);
    let energy_capacity = overrides
        .get("energy_capacity")
        .copied()
        .unwrap_or(energy_capacity_base);

    let capacity_limit_base = preset
        .and_then(|p| p.capacity_limit)
        .unwrap_or(cell_cfg.capacity_limit);
    let capacity_limit = overrides
        .get("capacity_limit")
        .copied()
        .unwrap_or(capacity_limit_base);

    let decay_rate_base = preset
        .and_then(|p| p.decay_rate)
        .unwrap_or(ri.decay_rate);
    let decay_rate = overrides
        .get("decay_rate")
        .copied()
        .unwrap_or(decay_rate_base);

    let stress_energy_threshold_base = preset
        .and_then(|p| p.stress_energy_threshold)
        .unwrap_or(lc.stress_energy_threshold);
    let stress_energy_threshold = overrides
        .get("stress_energy_threshold")
        .copied()
        .unwrap_or(stress_energy_threshold_base);

    let critical_capacity_overrun_base = preset
        .and_then(|p| p.critical_capacity_overrun)
        .unwrap_or(lc.critical_capacity_overrun);
    let critical_capacity_overrun = overrides
        .get("critical_capacity_overrun")
        .copied()
        .unwrap_or(critical_capacity_overrun_base);

    let heat_dissipation_rate_base = preset
        .and_then(|p| p.heat_dissipation_rate)
        .unwrap_or(env.heat_dissipation_rate);
    let heat_dissipation_rate = overrides
        .get("heat_dissipation_rate")
        .copied()
        .unwrap_or(heat_dissipation_rate_base);

    let heat_warning_threshold_base = preset
        .and_then(|p| p.heat_warning_threshold)
        .unwrap_or(env.heat_warning_threshold);
    let heat_warning_threshold = overrides
        .get("heat_warning_threshold")
        .copied()
        .unwrap_or(heat_warning_threshold_base);

    let heat_death_threshold_base = preset
        .and_then(|p| p.heat_death_threshold)
        .unwrap_or(env.heat_death_threshold);
    let heat_death_threshold = overrides
        .get("heat_death_threshold")
        .copied()
        .unwrap_or(heat_death_threshold_base);

    let waste_sink_rate_base = preset
        .and_then(|p| p.waste_sink_rate)
        .unwrap_or(env.waste_sink_rate);
    let waste_sink_rate = overrides
        .get("waste_sink_rate")
        .copied()
        .unwrap_or(waste_sink_rate_base);

    let waste_warning_threshold_base = preset
        .and_then(|p| p.waste_warning_threshold)
        .unwrap_or(env.waste_warning_threshold);
    let waste_warning_threshold = overrides
        .get("waste_warning_threshold")
        .copied()
        .unwrap_or(waste_warning_threshold_base);

    let waste_death_threshold_base = preset
        .and_then(|p| p.waste_death_threshold)
        .unwrap_or(env.waste_death_threshold);
    let waste_death_threshold = overrides
        .get("waste_death_threshold")
        .copied()
        .unwrap_or(waste_death_threshold_base);

    let resource_density_base = preset
        .and_then(|p| p.initial_resources.as_ref())
        .and_then(|r| r.first().copied())
        .unwrap_or(ri.default_resource_density);
    let resource_density = overrides
        .get("resource_density")
        .copied()
        .unwrap_or(resource_density_base);

    let passive_income_base = preset
        .and_then(|p| p.passive_energy_income)
        .unwrap_or(cell_cfg.passive_energy_income);
    let passive_income = overrides
        .get("passive_energy_income")
        .copied()
        .unwrap_or(passive_income_base);

    let upkeep_base = preset
        .and_then(|p| p.mandatory_cost_per_tick)
        .unwrap_or(cell_cfg.mandatory_cost_per_tick);
    let upkeep = overrides
        .get("mandatory_cost_per_tick")
        .copied()
        .unwrap_or(upkeep_base);

    let dormant_mod_base = preset
        .and_then(|p| p.dormant_mandatory_cost_modifier)
        .unwrap_or(lc.dormant_mandatory_cost_modifier);
    let dormant_mod = overrides
        .get("dormant_mandatory_cost_modifier")
        .copied()
        .unwrap_or(dormant_mod_base);

    let uptake = overrides
        .get("max_uptake_per_tick")
        .copied()
        .unwrap_or(ri.default_max_uptake_per_tick);
    let metabolism = overrides
        .get("metabolism_resource_per_tick")
        .copied()
        .unwrap_or(ri.default_metabolism_resource_per_tick);

    let resources = ResourceConfig::new(
        vec![ResourceAmount::new(resource_density.max(0.001)).unwrap()],
        decay_rate.clamp(0.0, 1.0),
    )
    .unwrap();

    let resource_interaction = ResourceInteractionConfig {
        enabled: true,
        uptake_layer_index: 0,
        max_uptake_per_tick: ResourceAmount::new(uptake.max(0.001)).unwrap(),
        metabolism_resource_per_tick: ResourceAmount::new(metabolism.max(0.0)).unwrap(),
        energy_per_resource: ri.energy_per_resource,
        heat_per_resource: ri.heat_per_resource,
        waste_per_resource: ri.waste_per_resource,
    };

    let cell = CellInitialConfig {
        position: Position::new(cell_x, cell_y),
        radius: Radius::new(cell_radius.max(0.1)).unwrap(),
        initial_energy: EnergyAmount::new(initial_energy).unwrap(),
        energy_capacity: EnergyAmount::new(energy_capacity).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(upkeep.max(0.0)).unwrap(),
        passive_energy_income: EnergyAmount::new(passive_income.max(0.0)).unwrap(),
        capacity_limit: CapacityAmount::new(capacity_limit.max(1.0)).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::new(cell_cfg.initial_boundary_material).unwrap(),
        initial_transport_material: MaterialAmount::new(cell_cfg.initial_transport_material)
            .unwrap(),
        initial_metabolic_material: MaterialAmount::new(cell_cfg.initial_metabolic_material)
            .unwrap(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(cell_cfg.initial_structural_material)
            .unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };

    let environment = EnvironmentConfig {
        heat_current: HeatAmount::zero(),
        heat_generated_per_tick: HeatAmount::zero(),
        heat_dissipation_rate: HeatAmount::new(heat_dissipation_rate).unwrap(),
        heat_warning_threshold: HeatAmount::new(heat_warning_threshold).unwrap(),
        heat_death_threshold: HeatAmount::new(heat_death_threshold).unwrap(),
        waste_current: WasteAmount::zero(),
        waste_generated_per_tick: WasteAmount::zero(),
        waste_sink_rate: WasteAmount::new(waste_sink_rate).unwrap(),
        waste_warning_threshold: WasteAmount::new(waste_warning_threshold).unwrap(),
        waste_death_threshold: WasteAmount::new(waste_death_threshold).unwrap(),
    };

    let dormancy_allowed = preset
        .and_then(|p| p.dormancy_allowed)
        .unwrap_or(lc.dormancy_allowed);

    let lifecycle = LifecycleConfig {
        stress_energy_threshold: EnergyAmount::new(stress_energy_threshold).unwrap(),
        dormancy_allowed,
        dormant_mandatory_cost_modifier: dormant_mod.clamp(0.0, 1.0),
        critical_capacity_overrun: CapacityAmount::new(critical_capacity_overrun.max(0.1)).unwrap(),
    };

    let mut rt = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(cfg.run.ticks.into()),
            seed: Seed::from_raw(cfg.run.seed),
            size: WorldSize::new(world_w, world_h).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        resources,
        resource_interaction,
        cell,
        environment,
        lifecycle,
    )
    .unwrap();

    rt.growth = alife::core::config::GrowthConfig::default();
    rt.synthesis = alife::core::config::SynthesisConfig::default();
    rt.contractility = alife::core::config::ContractilityConfig::default();
    rt.growth_enabled = preset.and_then(|p| p.growth_enabled).unwrap_or(false);
    rt
}

// ─────────────────────────────────────────────────────────────────────────────
// Run simulation for `ticks` steps, collect metrics
// ─────────────────────────────────────────────────────────────────────────────

use std::sync::OnceLock;

static ROLE_CLASSIFIER: OnceLock<CellRoleClassifierConfig> = OnceLock::new();
static BEHAVIOR_CLASSIFIER: OnceLock<BehaviorClassifierConfig> = OnceLock::new();

fn get_role_classifier() -> &'static CellRoleClassifierConfig {
    ROLE_CLASSIFIER.get_or_init(|| {
        load_cell_role_classifier("config/observer/cell-functional-role-classifier.toml")
            .expect("Failed to load cell functional role classifier config")
    })
}

fn get_behavior_classifier() -> &'static BehaviorClassifierConfig {
    BEHAVIOR_CLASSIFIER.get_or_init(|| {
        load_behavior_profile_classifier("config/observer/behavior-profile-classifier.toml")
            .expect("Failed to load behavior profile classifier config")
    })
}

fn run_simulation(rt_config: RuntimeConfig, ticks: u32) -> SimResult {
    let role_classifier = get_role_classifier();
    let behavior_classifier = get_behavior_classifier();
    let metabolism_cost = rt_config
        .resource_interaction
        .metabolism_resource_per_tick
        .raw();
    let energy_per_resource = rt_config.resource_interaction.energy_per_resource;
    let synthesis_cost_res = rt_config.synthesis.cost_resource.raw();
    let synthesis_cost_energy = rt_config.synthesis.cost_energy.raw();
    let growth_cost_res = rt_config.growth.growth_cost_resource.raw();
    let growth_cost_energy = rt_config.growth.growth_cost_energy.raw();
    let displacement_cost_energy = rt_config.contractility.energy_cost.raw();
    let mandatory_cost_per_tick = rt_config.cell.mandatory_cost_per_tick.raw();
    let dormant_cost =
        mandatory_cost_per_tick * rt_config.lifecycle.dormant_mandatory_cost_modifier;
    let passive_energy_income = rt_config.cell.passive_energy_income.raw();

    let mut executor = match TickExecutor::new(rt_config.clone()) {
        Ok(e) => e,
        Err(_) => {
            return SimResult {
                collapsed: true,
                collapse_tick: Some(0),
                dormant_ticks: 0,
                active_ticks: 0,
                stressed_ticks: 0,
                min_energy: 0.0,
                max_energy: 0.0,
                final_energy: 0.0,
                mean_energy: 0.0,
                initial_energy: 0.0,
                death_reason: "config_invalid".to_string(),
                energy_produced: 0.0,
                passive_energy_received: 0.0,
                energy_spent_upkeep: 0.0,
                energy_spent_dormant_upkeep: 0.0,
                energy_spent_movement: 0.0,
                energy_spent_growth: 0.0,
                energy_spent_repair: 0.0,
                energy_spent_division: 0.0,
                initial_world_resource: 0.0,
                final_world_resource: 0.0,
                resource_regenerated: 0.0,
                resource_absorbed: 0.0,
                resource_metabolized: 0.0,
                internal_resource_final: 0.0,
                resource_released: 0.0,
                resource_explicit_sink: 0.0,
                resource_balance_error: 0.0,
                energy_balance_error: 0.0,
                dormancy_enter_count: 0,
                dormancy_exit_count: 0,
                ticks_executed: 0,
                total_resource_consumed: 0.0,
                metabolism_count: 0,
                potential_role: "unknown".to_string(),
                observed_role: "unknown".to_string(),
                behavior_profile: "unknown".to_string(),
                ticks_per_second: 0.0,
                bhv_res: None,
                explicit_energy_loss: 0.0,
                death_cleanup_loss_energy: 0.0,
                death_cleanup_loss_resources: 0.0,
                clamping_loss: 0.0,
                unpaid_mandatory_cost: 0.0,
                resource_decay: 0.0,
                resource_sink: 0.0,
                numerical_error_energy: 0.0,
                numerical_error_resources: 0.0,
                unclassified_loss_energy: 0.0,
                unclassified_loss_resources: 0.0,
            };
        }
    };

    let len = executor.world().cells().len();

    let initial_grid = executor
        .world()
        .resources()
        .total_amount_for_layer(alife::core::resources::ResourceLayerIndex::from_raw(0))
        .map(|a| a.raw())
        .unwrap_or(0.0);
    let initial_cell_res = (0..len)
        .map(|i| {
            executor
                .world()
                .cells()
                .resource_amount(CellIndex::from_raw(i))
                .raw()
        })
        .sum::<f32>();
    let initial_cell_mat = (0..len)
        .map(|i| {
            executor
                .world()
                .cells()
                .total_materials(CellIndex::from_raw(i))
                .raw()
        })
        .sum::<f32>();

    let initial_energy = (0..len)
        .map(|i| {
            executor
                .world()
                .cells()
                .energy(CellIndex::from_raw(i))
                .current()
                .raw()
        })
        .sum::<f32>();

    let mut min_energy = f32::MAX;
    let mut max_energy = f32::MIN;
    let mut dormant_ticks = 0u32;
    let mut active_ticks = 0u32;
    let mut stressed_ticks = 0u32;
    let mut metabolism_count = 0u32;
    let mut collapsed = false;
    let mut collapse_tick = None;
    let mut death_reason = "none".to_string();

    let mut metabolized_cumulative = 0.0_f32;
    let mut decay_cumulative = 0.0_f32;
    let mut resource_absorbed_cumulative = 0.0_f32;
    let mut energy_produced = 0.0_f32;
    let mut passive_energy_received = 0.0_f32;
    let mut energy_spent_upkeep = 0.0_f32;
    let mut energy_spent_dormant_upkeep = 0.0_f32;
    let mut energy_spent_synthesis = 0.0_f32;
    let mut energy_spent_growth = 0.0_f32;
    let mut energy_spent_movement = 0.0_f32;
    let mut dormancy_enter_count = 0_u32;
    let mut dormancy_exit_count = 0_u32;
    let mut ticks_executed = 0_u32;
    let mut energy_sum = 0.0_f32;

    let mut clamping_cumulative = 0.0_f32;
    let mut unpaid_mandatory_cost_cumulative = 0.0_f32;
    let mut death_cleanup_loss_energy = 0.0_f32;
    let mut death_cleanup_loss_resources = 0.0_f32;

    let mut total_synthesis_successes = 0u32;
    let mut total_growth_successes = 0u32;
    let mut total_displacement_successes = 0u32;
    let mut growth_attempts = 0u32;
    let mut movement_ticks = 0u32;
    let mut scarcity_ticks = 0u32;
    let mut initial_radius = 0.0_f32;
    let mut final_radius = 0.0_f32;
    let mut initial_pos = Position::new(0.0, 0.0);
    let mut final_pos = Position::new(0.0, 0.0);

    if len > 0 {
        initial_radius = executor
            .world()
            .cells()
            .radius(CellIndex::from_raw(0))
            .raw();
        final_radius = initial_radius;
        initial_pos = executor.world().cells().position(CellIndex::from_raw(0));
        final_pos = initial_pos;
    }

    let loop_start = std::time::Instant::now();

    for t in 0..ticks {
        let mut cell_states_before = Vec::with_capacity(len);
        let mut cell_res_before = 0.0_f32;
        for i in 0..len {
            let index = CellIndex::from_raw(i);
            cell_states_before.push(executor.world().cells().lifecycle_state(index));
            cell_res_before += executor.world().cells().resource_amount(index).raw();
        }
        let grid_before = executor
            .world()
            .resources()
            .total_amount_for_layer(alife::core::resources::ResourceLayerIndex::from_raw(0))
            .map(|a| a.raw())
            .unwrap_or(0.0);

        let energy_before_sum = (0..len)
            .map(|i| {
                executor
                    .world()
                    .cells()
                    .energy(CellIndex::from_raw(i))
                    .current()
                    .raw()
            })
            .sum::<f32>();

        let summary = match executor.step() {
            Ok(s) => s,
            Err(_) => {
                collapsed = true;
                collapse_tick = Some(t);
                death_reason = "engine_error".to_string();
                break;
            }
        };

        ticks_executed += 1;

        if summary.survival_result == SurvivalResult::Collapse {
            collapsed = true;
            collapse_tick = Some(t);
            death_reason = format!("{:?}", summary.collapse_reason);
            break;
        }

        let e = summary.metrics.final_energy;
        min_energy = min_energy.min(e);
        max_energy = max_energy.max(e);
        energy_sum += e;

        if summary.metrics.process_attempts > 0 {
            metabolism_count += 1;
        }

        let grid_after = executor
            .world()
            .resources()
            .total_amount_for_layer(alife::core::resources::ResourceLayerIndex::from_raw(0))
            .map(|a| a.raw())
            .unwrap_or(0.0);
        let mut cell_res_after = 0.0_f32;
        for i in 0..len {
            let index = CellIndex::from_raw(i);
            cell_res_after += executor.world().cells().resource_amount(index).raw();
        }

        // Get process successes in this tick
        let get_successes = |proc_id: alife::core::process::ProcessId,
                             diag: &alife::core::summary::ProcessDiagnostics|
         -> u32 {
            let att = diag.attempts_by_process.get(&proc_id).copied().unwrap_or(0);
            let rej = diag
                .rejections_by_process
                .get(&proc_id)
                .copied()
                .unwrap_or(0);
            att.saturating_sub(rej)
        };

        let metabolism_successes = get_successes(
            alife::core::process::ProcessId::MetabolismEnergyConversion,
            &summary.diagnostics,
        );
        let synthesis_successes = get_successes(
            alife::core::process::ProcessId::MaterialSynthesis,
            &summary.diagnostics,
        );
        let growth_successes = get_successes(
            alife::core::process::ProcessId::GrowthResourceAllocation,
            &summary.diagnostics,
        );
        let displacement_successes = get_successes(
            alife::core::process::ProcessId::ContractileDisplacement,
            &summary.diagnostics,
        );

        let metabolized_tick = metabolism_successes as f32 * metabolism_cost;
        metabolized_cumulative += metabolized_tick;
        energy_produced += metabolized_tick * energy_per_resource;

        let synthesis_tick_res = synthesis_successes as f32 * synthesis_cost_res;
        energy_spent_synthesis += synthesis_successes as f32 * synthesis_cost_energy;

        let growth_tick_res = growth_successes as f32 * growth_cost_res;
        energy_spent_growth += growth_successes as f32 * growth_cost_energy;

        energy_spent_movement += displacement_successes as f32 * displacement_cost_energy;

        let uptake_tick = (cell_res_after - cell_res_before)
            + metabolized_tick
            + synthesis_tick_res
            + growth_tick_res;
        resource_absorbed_cumulative += uptake_tick;

        let decay_tick = (grid_before - uptake_tick - grid_after).max(0.0);
        decay_cumulative += decay_tick;

        // Energy spent and passive income
        for (i, &state_before) in cell_states_before.iter().enumerate() {
            let index = CellIndex::from_raw(i);
            let state_after = executor.world().cells().lifecycle_state(index);
            if state_before != alife::core::cell_store::LifecycleState::Dead {
                passive_energy_received += passive_energy_income;
                if state_before == alife::core::cell_store::LifecycleState::Dormant {
                    energy_spent_dormant_upkeep += dormant_cost;
                } else {
                    energy_spent_upkeep += mandatory_cost_per_tick;
                }

                // Check transitions
                if state_before != alife::core::cell_store::LifecycleState::Dormant
                    && state_after == alife::core::cell_store::LifecycleState::Dormant
                {
                    dormancy_enter_count += 1;
                }
                if state_before == alife::core::cell_store::LifecycleState::Dormant
                    && state_after != alife::core::cell_store::LifecycleState::Dormant
                    && state_after != alife::core::cell_store::LifecycleState::Dead
                {
                    dormancy_exit_count += 1;
                }
            }
        }

        // New accounting category calculations
        let mut upkeep_tick_sum = 0.0_f32;
        for (i, &state_before) in cell_states_before.iter().enumerate() {
            if state_before != LifecycleState::Dead {
                let flags = executor.world().cells().runtime_flags(CellIndex::from_raw(i));
                if flags.mandatory_paid {
                    if state_before == LifecycleState::Dormant {
                        upkeep_tick_sum += dormant_cost;
                    } else {
                        upkeep_tick_sum += mandatory_cost_per_tick;
                    }
                }
            }
        }

        let spent_tick_sum = (synthesis_successes as f32 * synthesis_cost_energy)
            + (growth_successes as f32 * growth_cost_energy)
            + (displacement_successes as f32 * displacement_cost_energy);

        let mut passive_income_tick_sum = 0.0_f32;
        for &state_before in &cell_states_before {
            if state_before != LifecycleState::Dead {
                passive_income_tick_sum += passive_energy_income;
            }
        }
        let produced_tick_sum = metabolism_successes as f32 * energy_per_resource;

        let expected_after_tick = energy_before_sum + passive_income_tick_sum + produced_tick_sum - upkeep_tick_sum - spent_tick_sum;
        let energy_after_sum = (0..len)
            .map(|i| {
                executor
                    .world()
                    .cells()
                    .energy(CellIndex::from_raw(i))
                    .current()
                    .raw()
            })
            .sum::<f32>();

        let clamping_tick = (expected_after_tick - energy_after_sum).max(0.0);
        clamping_cumulative += clamping_tick;

        let mut unpaid_mandatory_cost_tick = 0.0_f32;
        for (i, &state_before) in cell_states_before.iter().enumerate() {
            if state_before != LifecycleState::Dead {
                let flags = executor.world().cells().runtime_flags(CellIndex::from_raw(i));
                if !flags.mandatory_paid {
                    let cost = if state_before == LifecycleState::Dormant {
                        dormant_cost
                    } else {
                        mandatory_cost_per_tick
                    };
                    unpaid_mandatory_cost_tick += cost;
                }
            }
        }
        unpaid_mandatory_cost_cumulative += unpaid_mandatory_cost_tick;

        for (i, &state_before) in cell_states_before.iter().enumerate() {
            let index = CellIndex::from_raw(i);
            let state_after = executor.world().cells().lifecycle_state(index);
            if state_before != LifecycleState::Dead && state_after == LifecycleState::Dead {
                death_cleanup_loss_energy += executor.world().cells().energy(index).current().raw();
                death_cleanup_loss_resources += executor.world().cells().resource_amount(index).raw()
                    + executor.world().cells().total_materials(index).raw();
            }
        }

        if len > 0 {
            let state = executor
                .world()
                .cells()
                .lifecycle_state(CellIndex::from_raw(0));
            match state {
                LifecycleState::Dormant => dormant_ticks += 1,
                LifecycleState::Stressed => stressed_ticks += 1,
                LifecycleState::Alive => active_ticks += 1,
                LifecycleState::Dead => {}
            }
        }

        if !collapsed {
            total_synthesis_successes += synthesis_successes;
            total_growth_successes += growth_successes;
            total_displacement_successes += displacement_successes;
            growth_attempts += summary
                .diagnostics
                .attempts_by_process
                .get(&alife::core::process::ProcessId::GrowthResourceAllocation)
                .copied()
                .unwrap_or(0);
            if displacement_successes > 0 {
                movement_ticks += 1;
            }
            if uptake_tick <= 0.001 {
                scarcity_ticks += 1;
            }
            if len > 0 {
                final_radius = executor
                    .world()
                    .cells()
                    .radius(CellIndex::from_raw(0))
                    .raw();
                final_pos = executor.world().cells().position(CellIndex::from_raw(0));
            }
        }
    }

    let final_grid = executor
        .world()
        .resources()
        .total_amount_for_layer(alife::core::resources::ResourceLayerIndex::from_raw(0))
        .map(|a| a.raw())
        .unwrap_or(0.0);
    let final_cell_res = (0..len)
        .map(|i| {
            executor
                .world()
                .cells()
                .resource_amount(CellIndex::from_raw(i))
                .raw()
        })
        .sum::<f32>();

    let final_cell_res_alive = (0..len)
        .filter(|&i| executor.world().cells().lifecycle_state(CellIndex::from_raw(i)) != LifecycleState::Dead)
        .map(|i| {
            executor
                .world()
                .cells()
                .resource_amount(CellIndex::from_raw(i))
                .raw()
        })
        .sum::<f32>();

    let final_cell_mat_alive = (0..len)
        .filter(|&i| executor.world().cells().lifecycle_state(CellIndex::from_raw(i)) != LifecycleState::Dead)
        .map(|i| {
            executor
                .world()
                .cells()
                .total_materials(CellIndex::from_raw(i))
                .raw()
        })
        .sum::<f32>();

    let final_energy = (0..len)
        .map(|i| {
            executor
                .world()
                .cells()
                .energy(CellIndex::from_raw(i))
                .current()
                .raw()
        })
        .sum::<f32>();

    let final_energy_alive = (0..len)
        .filter(|&i| executor.world().cells().lifecycle_state(CellIndex::from_raw(i)) != LifecycleState::Dead)
        .map(|i| {
            executor
                .world()
                .cells()
                .energy(CellIndex::from_raw(i))
                .current()
                .raw()
        })
        .sum::<f32>();

    let energy_total_input = initial_energy + energy_produced + passive_energy_received;
    let energy_total_output = final_energy_alive
        + energy_spent_upkeep
        + energy_spent_dormant_upkeep
        + energy_spent_synthesis
        + energy_spent_growth
        + energy_spent_movement
        + death_cleanup_loss_energy
        + clamping_cumulative;
    let energy_balance_error = (energy_total_input - energy_total_output).abs();

    let numerical_error_energy;
    let unclassified_loss_energy;
    if energy_balance_error < 0.01 {
        numerical_error_energy = energy_balance_error;
        unclassified_loss_energy = 0.0;
    } else {
        numerical_error_energy = 0.0;
        unclassified_loss_energy = energy_balance_error;
    }

    let resource_total_input = initial_grid + initial_cell_res + initial_cell_mat;
    let resource_total_output = final_grid
        + final_cell_res_alive
        + final_cell_mat_alive
        + metabolized_cumulative
        + decay_cumulative
        + death_cleanup_loss_resources;
    let resource_balance_error = (resource_total_input - resource_total_output).abs();

    let numerical_error_resources;
    let unclassified_loss_resources;
    if resource_balance_error < 0.01 {
        numerical_error_resources = resource_balance_error;
        unclassified_loss_resources = 0.0;
    } else {
        numerical_error_resources = 0.0;
        unclassified_loss_resources = resource_balance_error;
    }

    let mean_energy = if ticks_executed > 0 {
        energy_sum / ticks_executed as f32
    } else {
        0.0
    };

    let total_resource_consumed = (initial_grid - final_grid).max(0.0);

    if min_energy == f32::MAX {
        min_energy = 0.0;
    }
    if max_energy == f32::MIN {
        max_energy = 0.0;
    }

    let elapsed = loop_start.elapsed();
    let ticks_per_second = if elapsed.as_secs_f32() > 1e-6 {
        ticks_executed as f32 / elapsed.as_secs_f32()
    } else {
        0.0
    };

    let mut raw_data = HashMap::new();
    raw_data.insert("ticks_executed".to_string(), ticks_executed as f32);
    raw_data.insert("dormant_ticks".to_string(), dormant_ticks as f32);
    raw_data.insert("dormancy_entries".to_string(), dormancy_enter_count as f32);
    raw_data.insert(
        "growth_attempts_fraction".to_string(),
        if ticks_executed > 0 {
            growth_attempts as f32 / ticks_executed as f32
        } else {
            0.0
        },
    );
    let radius_increase_factor = if initial_radius > 0.0 {
        final_radius / initial_radius
    } else {
        1.0
    };
    raw_data.insert("radius_increase_factor".to_string(), radius_increase_factor);
    raw_data.insert(
        "metabolism_executions_fraction".to_string(),
        if ticks_executed > 0 {
            metabolism_count as f32 / ticks_executed as f32
        } else {
            0.0
        },
    );
    raw_data.insert(
        "heat_produced_per_resource".to_string(),
        rt_config.resource_interaction.heat_per_resource,
    );
    raw_data.insert(
        "contractile_executions_fraction".to_string(),
        if ticks_executed > 0 {
            movement_ticks as f32 / ticks_executed as f32
        } else {
            0.0
        },
    );

    let dx = final_pos.x() - initial_pos.x();
    let dy = final_pos.y() - initial_pos.y();
    let displacement_distance = (dx * dx + dy * dy).sqrt();
    raw_data.insert("displacement_distance".to_string(), displacement_distance);
    raw_data.insert("scarcity_ticks".to_string(), scarcity_ticks as f32);
    let energy_loss_rate = if ticks_executed > 0 {
        ((initial_energy - final_energy) / ticks_executed as f32).max(0.0)
    } else {
        0.0
    };
    raw_data.insert("energy_loss_rate".to_string(), energy_loss_rate);

    if len > 0 {
        let cell_idx = CellIndex::from_raw(0);
        let cells = executor.world().cells();
        raw_data.insert(
            "boundary_material".to_string(),
            cells.boundary_material(cell_idx).raw(),
        );
        raw_data.insert(
            "transport_material".to_string(),
            cells.transport_material(cell_idx).raw(),
        );
        raw_data.insert(
            "metabolic_material".to_string(),
            cells.metabolic_material(cell_idx).raw(),
        );
        raw_data.insert(
            "storage_material".to_string(),
            cells.storage_material(cell_idx).raw(),
        );
        raw_data.insert(
            "synthesis_material".to_string(),
            cells.synthesis_material(cell_idx).raw(),
        );
        raw_data.insert(
            "structural_material".to_string(),
            cells.structural_material(cell_idx).raw(),
        );
        raw_data.insert(
            "repair_material".to_string(),
            cells.repair_material(cell_idx).raw(),
        );
        raw_data.insert(
            "contractile_material".to_string(),
            cells.contractile_material(cell_idx).raw(),
        );
        raw_data.insert(
            "sensory_material".to_string(),
            cells.sensory_material(cell_idx).raw(),
        );
        raw_data.insert(
            "total_materials".to_string(),
            cells.total_materials(cell_idx).raw(),
        );
    } else {
        raw_data.insert("boundary_material".to_string(), 0.0);
        raw_data.insert("transport_material".to_string(), 0.0);
        raw_data.insert("metabolic_material".to_string(), 0.0);
        raw_data.insert("storage_material".to_string(), 0.0);
        raw_data.insert("synthesis_material".to_string(), 0.0);
        raw_data.insert("structural_material".to_string(), 0.0);
        raw_data.insert("repair_material".to_string(), 0.0);
        raw_data.insert("contractile_material".to_string(), 0.0);
        raw_data.insert("sensory_material".to_string(), 0.0);
        raw_data.insert("total_materials".to_string(), 0.0);
    }

    let passive_uptake = if raw_data.get("boundary_material").copied().unwrap_or(0.0) > 0.0 {
        resource_absorbed_cumulative
    } else {
        0.0
    };
    raw_data.insert("PassiveUptake_executed".to_string(), passive_uptake);

    let active_uptake = if raw_data.get("transport_material").copied().unwrap_or(0.0) > 0.0 {
        resource_absorbed_cumulative
    } else {
        0.0
    };
    raw_data.insert("ActiveUptake_executed".to_string(), active_uptake);

    raw_data.insert("Metabolism_executed".to_string(), metabolized_cumulative);
    raw_data.insert("Storage_executed".to_string(), final_cell_res);
    raw_data.insert(
        "MaterialSynthesis_executed".to_string(),
        total_synthesis_successes as f32,
    );
    raw_data.insert("Growth_executed".to_string(), total_growth_successes as f32);
    raw_data.insert(
        "ContractileDisplacement_executed".to_string(),
        total_displacement_successes as f32,
    );

    let window = extract_features(
        "run-id",
        EntityType::Cell,
        "cell-0",
        0,
        ticks_executed as u64,
        &raw_data,
    );

    let pot_res = classify_cell_roles_potential(&window, role_classifier);
    let obs_res = classify_cell_roles_observed(&window, role_classifier);
    let bhv_res = classify_behavior_profiles(&window, behavior_classifier);

    let potential_role = pot_res
        .primary_label
        .clone()
        .unwrap_or_else(|| "unknown".to_string());
    let observed_role = obs_res
        .primary_label
        .clone()
        .unwrap_or_else(|| "unknown".to_string());
    let behavior_profile = bhv_res
        .primary_label
        .clone()
        .unwrap_or_else(|| "unknown".to_string());

    SimResult {
        collapsed,
        collapse_tick,
        dormant_ticks,
        active_ticks,
        stressed_ticks,
        min_energy,
        max_energy,
        final_energy,
        mean_energy,
        initial_energy,
        death_reason,
        energy_produced,
        passive_energy_received,
        energy_spent_upkeep,
        energy_spent_dormant_upkeep,
        energy_spent_movement,
        energy_spent_growth,
        energy_spent_repair: 0.0,
        energy_spent_division: 0.0,
        initial_world_resource: initial_grid,
        final_world_resource: final_grid,
        resource_regenerated: 0.0,
        resource_absorbed: resource_absorbed_cumulative,
        resource_metabolized: metabolized_cumulative,
        internal_resource_final: final_cell_res,
        resource_released: 0.0,
        resource_explicit_sink: decay_cumulative,
        resource_balance_error,
        energy_balance_error,
        dormancy_enter_count,
        dormancy_exit_count,
        ticks_executed,
        total_resource_consumed,
        metabolism_count,
        potential_role,
        observed_role,
        behavior_profile,
        ticks_per_second,
        bhv_res: Some(bhv_res),
        explicit_energy_loss: 0.0,
        death_cleanup_loss_energy,
        death_cleanup_loss_resources,
        clamping_loss: clamping_cumulative,
        unpaid_mandatory_cost: unpaid_mandatory_cost_cumulative,
        resource_decay: decay_cumulative,
        resource_sink: 0.0,
        numerical_error_energy,
        numerical_error_resources,
        unclassified_loss_energy,
        unclassified_loss_resources,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Classify an outcome into a zone label
// ─────────────────────────────────────────────────────────────────────────────

fn classify(res: &SimResult, ticks: u32) -> &'static str {
    if res.collapsed {
        return "collapse";
    }
    let ticks_f = ticks.max(1) as f32;
    let dormant_pct = res.dormant_ticks as f32 / ticks_f;
    let active_pct = res.active_ticks as f32 / ticks_f;
    if dormant_pct > 0.80 {
        "dormancy"
    } else if dormant_pct > 0.20 {
        "dormancy_survival"
    } else if active_pct > 0.70 && res.final_energy > res.min_energy + 10.0 {
        "accumulates"
    } else {
        "stable"
    }
}

pub fn detect_warnings(results: &[SimResult], scenario_id: &str) -> Vec<String> {
    let mut warnings = Vec::new();
    if results.is_empty() {
        return warnings;
    }

    let scenario_lower = scenario_id.to_lowercase();
    if (scenario_lower.contains("dormant") || scenario_lower.contains("dormancy"))
        && results.iter().all(|r| r.dormant_ticks == 0)
    {
        warnings.push("SCENARIO_MECHANISM_NOT_ACTIVATED".to_string());
    }

    let first_energy = results[0].final_energy;
    let all_energies_same = results
        .iter()
        .all(|r| (r.final_energy - first_energy).abs() < 1e-4);

    let ticks = results[0].ticks_executed.max(1);
    let first_zone = classify(&results[0], ticks);
    let all_zones_same = results.iter().all(|r| classify(r, ticks) == first_zone);

    if all_energies_same || all_zones_same {
        warnings.push("LOW_INFORMATION_SWEEP".to_string());
    }

    warnings
}

// ─────────────────────────────────────────────────────────────────────────────
// Aggregate stats over a slice of SimResults
// ─────────────────────────────────────────────────────────────────────────────

struct Stats {
    min: f32,
    max: f32,
    mean: f32,
    ideal_range_lo: f32,
    ideal_range_hi: f32,
}

fn compute_stats(values: &[f32]) -> Stats {
    if values.is_empty() {
        return Stats {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            ideal_range_lo: 0.0,
            ideal_range_hi: 0.0,
        };
    }
    let min = values.iter().copied().fold(f32::MAX, f32::min);
    let max = values.iter().copied().fold(f32::MIN, f32::max);
    let mean = values.iter().copied().sum::<f32>() / values.len() as f32;
    // "Ideal range" = mean ± 10% (as a viability guideline)
    let spread = (max - min) * 0.1;
    Stats {
        min,
        max,
        mean,
        ideal_range_lo: (mean - spread).max(min),
        ideal_range_hi: (mean + spread).min(max),
    }
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct ClassificationRecord {
    pub sweep_name: String,
    pub scenario_id: String,
    pub parameter_name: String,
    pub parameter_value: f32,
    pub secondary_parameter_name: String,
    pub secondary_parameter_value: f32,
    pub potential_role: String,
    pub observed_role: String,
    pub behavior_profile: String,
    pub ticks_per_second: f32,
    pub classification_mode: String,
    pub status: String,
    pub primary_label: String,
    pub secondary_labels: String,
    pub score: f32,
    pub confidence: f32,
    pub evidence_summary: String,
    pub classifier_version: String,
    pub tick_start: u64,
    pub tick_end: u64,
    pub data_completeness: f32,
}

fn write_summary_row(
    out_dir: &str,
    name: &str,
    scenario_id: &str,
    stats: &Stats,
    zone_counts: &HashMap<&str, usize>,
) {
    let summary_path = format!("{}/sweep_scenario_summary.csv", out_dir);
    let is_new = !std::path::Path::new(&summary_path).exists();

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&summary_path)
        .expect("cannot open sweep_scenario_summary.csv");

    if is_new {
        writeln!(
            file,
            "sweep_name,scenario_id,min_final_energy,max_final_energy,mean_final_energy,\
             ideal_range_lo,ideal_range_hi,stable_count,collapse_count,dormancy_count,\
             dormancy_survival_count,accumulates_count"
        )
        .unwrap();
    }

    let stable = zone_counts.get("stable").copied().unwrap_or(0);
    let collapse = zone_counts.get("collapse").copied().unwrap_or(0);
    let dormancy = zone_counts.get("dormancy").copied().unwrap_or(0);
    let dormancy_survival = zone_counts.get("dormancy_survival").copied().unwrap_or(0);
    let accumulates = zone_counts.get("accumulates").copied().unwrap_or(0);

    writeln!(
        file,
        "{},{},{:.3},{:.3},{:.3},{:.3},{:.3},{},{},{},{},{}",
        name,
        scenario_id,
        stats.min,
        stats.max,
        stats.mean,
        stats.ideal_range_lo,
        stats.ideal_range_hi,
        stable,
        collapse,
        dormancy,
        dormancy_survival,
        accumulates
    )
    .unwrap();
}

// ─────────────────────────────────────────────────────────────────────────────
// Run a 1-D sweep and write results
// ─────────────────────────────────────────────────────────────────────────────

pub fn run_sweep(
    cfg: &AnalyzerConfig,
    sweep: &SweepDef,
    preset: Option<&RawScenarioPreset>,
    out_dir: &str,
) -> Vec<ClassificationRecord> {
    println!(
        "\n▶ Sweep «{}» — {} from {:.3} to {:.3} in {} steps ({} ticks each)",
        sweep.name, sweep.param, sweep.from, sweep.to, sweep.steps, cfg.run.ticks
    );

    let step_size = if sweep.steps <= 1 {
        0.0
    } else {
        (sweep.to - sweep.from) / (sweep.steps - 1) as f32
    };

    let csv_path = format!("{}/{}.csv", out_dir, sweep.name);
    let mut csv = std::fs::File::create(&csv_path).expect("cannot create CSV");
    writeln!(
        csv,
        "scenario_id,scenario_version,config_hash,seed,ticks_requested,ticks_executed,\
         parameter_name,parameter_value,secondary_parameter_name,secondary_parameter_value,\
         zone,scenario_status,warning_codes,survived_to_end,survival_ticks,death_tick,death_reason,\
         active_ticks,active_fraction,dormant_ticks,dormant_fraction,dormancy_enter_count,dormancy_exit_count,\
         initial_energy,final_energy,min_energy,max_energy,mean_energy,\
         energy_produced,passive_energy_received,\
         energy_spent_upkeep,energy_spent_dormant_upkeep,energy_spent_movement,energy_spent_growth,energy_spent_repair,energy_spent_division,\
         initial_world_resource,final_world_resource,resource_regenerated,resource_absorbed,resource_metabolized,internal_resource_final,resource_released,resource_explicit_sink,\
         resource_balance_error,energy_balance_error,ticks_per_second,\
         explicit_energy_loss,death_cleanup_loss_energy,death_cleanup_loss_resources,clamping_loss,unpaid_mandatory_cost,resource_decay,resource_sink,numerical_error_energy,numerical_error_resources,unclassified_loss_energy,unclassified_loss_resources"
    )
    .unwrap();

    let mut final_energies: Vec<f32> = Vec::new();
    let mut zone_counts: HashMap<&str, usize> = HashMap::new();
    let mut run_results = Vec::new();
    let mut records = Vec::new();

    let scenario_id = sweep.scenario.as_deref().unwrap_or("none");

    for i in 0..sweep.steps {
        let val = sweep.from + step_size * i as f32;
        let mut overrides = HashMap::new();
        overrides.insert(sweep.param.as_str(), val);

        let rt = build_config(cfg, preset, &overrides);
        let config_hash = rt.config_hash();
        let res = run_simulation(rt, cfg.run.ticks);
        let zone = classify(&res, cfg.run.ticks);
        *zone_counts.entry(zone).or_insert(0) += 1;

        final_energies.push(res.final_energy);

        let bhv_res = res.bhv_res.as_ref();
        records.push(ClassificationRecord {
            sweep_name: sweep.name.clone(),
            scenario_id: scenario_id.to_string(),
            parameter_name: sweep.param.clone(),
            parameter_value: val,
            secondary_parameter_name: "none".to_string(),
            secondary_parameter_value: 0.0,
            potential_role: res.potential_role.clone(),
            observed_role: res.observed_role.clone(),
            behavior_profile: res.behavior_profile.clone(),
            ticks_per_second: res.ticks_per_second,
            classification_mode: bhv_res.map(|r| format!("{:?}", r.mode)).unwrap_or_else(|| "unknown".to_string()),
            status: bhv_res.map(|r| format!("{:?}", r.status)).unwrap_or_else(|| "unknown".to_string()),
            primary_label: bhv_res.map(|r| r.primary_label.clone().unwrap_or_else(|| "unknown".to_string())).unwrap_or_else(|| "unknown".to_string()),
            secondary_labels: bhv_res.map(|r| r.secondary_labels.iter().map(|lr| lr.label.as_str()).collect::<Vec<_>>().join(",")).unwrap_or_else(|| "".to_string()),
            score: bhv_res.map(|r| r.confidence).unwrap_or(0.0),
            confidence: bhv_res.map(|r| r.confidence).unwrap_or(0.0),
            evidence_summary: bhv_res.map(|r| {
                r.evidence.iter().map(|e| format!("{} {}: {}", e.feature, e.expected, e.matched)).collect::<Vec<_>>().join(", ")
            }).unwrap_or_else(|| "".to_string()),
            classifier_version: bhv_res.map(|r| r.classifier_version.clone()).unwrap_or_else(|| "".to_string()),
            tick_start: bhv_res.map(|r| r.tick_start).unwrap_or(0),
            tick_end: bhv_res.map(|r| r.tick_end).unwrap_or(0),
            data_completeness: bhv_res.map(|r| r.data_completeness).unwrap_or(0.0),
        });

        run_results.push((val, config_hash, res, zone));
    }

    let sweep_warnings = detect_warnings(
        &run_results
            .iter()
            .map(|(_, _, r, _)| r.clone())
            .collect::<Vec<_>>(),
        scenario_id,
    );

    for (val, config_hash, res, zone) in run_results {
        let scenario_status = if res.collapsed { "collapsed" } else { "stable" };
        let mut row_warnings = Vec::new();
        if res.resource_balance_error > 0.01 || res.energy_balance_error > 0.01 {
            row_warnings.push("BALANCE_ERROR".to_string());
        }
        for w in &sweep_warnings {
            row_warnings.push(w.clone());
        }
        let warning_codes = if row_warnings.is_empty() {
            "none".to_string()
        } else {
            row_warnings.join("|")
        };

        let active_fraction = if res.ticks_executed > 0 {
            res.active_ticks as f32 / res.ticks_executed as f32
        } else {
            0.0
        };
        let dormant_fraction = if res.ticks_executed > 0 {
            res.dormant_ticks as f32 / res.ticks_executed as f32
        } else {
            0.0
        };

        writeln!(
            csv,
            "{},1.0,{},{},{},{},{},{:.4},none,0.0,{},{},{},{},{},{},{},{},{:.4},{},{:.4},{},{},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4}",
            scenario_id,
            config_hash,
            cfg.run.seed,
            cfg.run.ticks,
            res.ticks_executed,
            sweep.param,
            val,
            zone,
            scenario_status,
            warning_codes,
            !res.collapsed,
            res.ticks_executed,
            res.collapse_tick.unwrap_or(0),
            res.death_reason,
            res.active_ticks,
            active_fraction,
            res.dormant_ticks,
            dormant_fraction,
            res.dormancy_enter_count,
            res.dormancy_exit_count,
            res.initial_energy,
            res.final_energy,
            res.min_energy,
            res.max_energy,
            res.mean_energy,
            res.energy_produced,
            res.passive_energy_received,
            res.energy_spent_upkeep,
            res.energy_spent_dormant_upkeep,
            res.energy_spent_movement,
            res.energy_spent_growth,
            res.energy_spent_repair,
            res.energy_spent_division,
            res.initial_world_resource,
            res.final_world_resource,
            res.resource_regenerated,
            res.resource_absorbed,
            res.resource_metabolized,
            res.internal_resource_final,
            res.resource_released,
            res.resource_explicit_sink,
            res.resource_balance_error,
            res.energy_balance_error,
            res.ticks_per_second,
            res.explicit_energy_loss,
            res.death_cleanup_loss_energy,
            res.death_cleanup_loss_resources,
            res.clamping_loss,
            res.unpaid_mandatory_cost,
            res.resource_decay,
            res.resource_sink,
            res.numerical_error_energy,
            res.numerical_error_resources,
            res.unclassified_loss_energy,
            res.unclassified_loss_resources
        )
        .unwrap();

        // console progress
        let collapse_str = res
            .collapse_tick
            .map(|t| format!("tick {}", t))
            .unwrap_or_else(|| "—".to_string());
        println!(
            "  {:>8.4} │ {:20} │ energy [{:.1}..{:.1}] final={:.1} | collapsed={} @{}",
            val,
            zone,
            res.min_energy,
            res.max_energy,
            res.final_energy,
            res.collapsed,
            collapse_str
        );
    }

    // aggregate statistics
    let stats = compute_stats(&final_energies);
    println!("\n  ── Aggregate (final_energy) ──────────────────────────");
    println!(
        "  min={:.2}  max={:.2}  mean={:.2}  ideal=[{:.2}..{:.2}]",
        stats.min, stats.max, stats.mean, stats.ideal_range_lo, stats.ideal_range_hi
    );
    println!("  Zone distribution: {:?}", zone_counts);
    println!("  CSV → {}", csv_path);

    write_summary_row(out_dir, &sweep.name, scenario_id, &stats, &zone_counts);
    records
}

// ─────────────────────────────────────────────────────────────────────────────
// Run a 2-D matrix sweep and write results
// ─────────────────────────────────────────────────────────────────────────────

pub fn run_matrix(
    cfg: &AnalyzerConfig,
    mat: &MatrixDef,
    preset: Option<&RawScenarioPreset>,
    out_dir: &str,
) -> Vec<ClassificationRecord> {
    println!(
        "\n▶ Matrix «{}» — {} × {} ({} × {} = {} runs, {} ticks each)",
        mat.name,
        mat.param_x,
        mat.param_y,
        mat.steps_x,
        mat.steps_y,
        mat.steps_x * mat.steps_y,
        cfg.run.ticks
    );

    let step_x = if mat.steps_x <= 1 {
        0.0
    } else {
        (mat.to_x - mat.from_x) / (mat.steps_x - 1) as f32
    };
    let step_y = if mat.steps_y <= 1 {
        0.0
    } else {
        (mat.to_y - mat.from_y) / (mat.steps_y - 1) as f32
    };

    let csv_path = format!("{}/{}_matrix.csv", out_dir, mat.name);
    let mut csv = std::fs::File::create(&csv_path).expect("cannot create matrix CSV");
    writeln!(
        csv,
        "scenario_id,scenario_version,config_hash,seed,ticks_requested,ticks_executed,\
         parameter_name,parameter_value,secondary_parameter_name,secondary_parameter_value,\
         zone,scenario_status,warning_codes,survived_to_end,survival_ticks,death_tick,death_reason,\
         active_ticks,active_fraction,dormant_ticks,dormant_fraction,dormancy_enter_count,dormancy_exit_count,\
         initial_energy,final_energy,min_energy,max_energy,mean_energy,\
         energy_produced,passive_energy_received,\
         energy_spent_upkeep,energy_spent_dormant_upkeep,energy_spent_movement,energy_spent_growth,energy_spent_repair,energy_spent_division,\
         initial_world_resource,final_world_resource,resource_regenerated,resource_absorbed,resource_metabolized,internal_resource_final,resource_released,resource_explicit_sink,\
         resource_balance_error,energy_balance_error,ticks_per_second,\
         explicit_energy_loss,death_cleanup_loss_energy,death_cleanup_loss_resources,clamping_loss,unpaid_mandatory_cost,resource_decay,resource_sink,numerical_error_energy,numerical_error_resources,unclassified_loss_energy,unclassified_loss_resources"
    )
    .unwrap();

    let mut all_finals: Vec<f32> = Vec::new();
    let mut zone_counts: HashMap<&str, usize> = HashMap::new();
    let mut run_results = Vec::new();
    let mut records = Vec::new();

    let scenario_id = mat.scenario.as_deref().unwrap_or("none");

    for ix in 0..mat.steps_x {
        let vx = mat.from_x + step_x * ix as f32;
        for iy in 0..mat.steps_y {
            let vy = mat.from_y + step_y * iy as f32;

            let mut overrides = HashMap::new();
            overrides.insert(mat.param_x.as_str(), vx);
            overrides.insert(mat.param_y.as_str(), vy);

            let rt = build_config(cfg, preset, &overrides);
            let config_hash = rt.config_hash();
            let res = run_simulation(rt, cfg.run.ticks);
            let zone = classify(&res, cfg.run.ticks);
            *zone_counts.entry(zone).or_insert(0) += 1;

            all_finals.push(res.final_energy);

            let bhv_res = res.bhv_res.as_ref();
            records.push(ClassificationRecord {
                sweep_name: mat.name.clone(),
                scenario_id: scenario_id.to_string(),
                parameter_name: mat.param_x.clone(),
                parameter_value: vx,
                secondary_parameter_name: mat.param_y.clone(),
                secondary_parameter_value: vy,
                potential_role: res.potential_role.clone(),
                observed_role: res.observed_role.clone(),
                behavior_profile: res.behavior_profile.clone(),
                ticks_per_second: res.ticks_per_second,
                classification_mode: bhv_res.map(|r| format!("{:?}", r.mode)).unwrap_or_else(|| "unknown".to_string()),
                status: bhv_res.map(|r| format!("{:?}", r.status)).unwrap_or_else(|| "unknown".to_string()),
                primary_label: bhv_res.map(|r| r.primary_label.clone().unwrap_or_else(|| "unknown".to_string())).unwrap_or_else(|| "unknown".to_string()),
                secondary_labels: bhv_res.map(|r| r.secondary_labels.iter().map(|lr| lr.label.as_str()).collect::<Vec<_>>().join(",")).unwrap_or_else(|| "".to_string()),
                score: bhv_res.map(|r| r.confidence).unwrap_or(0.0),
                confidence: bhv_res.map(|r| r.confidence).unwrap_or(0.0),
                evidence_summary: bhv_res.map(|r| {
                    r.evidence.iter().map(|e| format!("{} {}: {}", e.feature, e.expected, e.matched)).collect::<Vec<_>>().join(", ")
                }).unwrap_or_else(|| "".to_string()),
                classifier_version: bhv_res.map(|r| r.classifier_version.clone()).unwrap_or_else(|| "".to_string()),
                tick_start: bhv_res.map(|r| r.tick_start).unwrap_or(0),
                tick_end: bhv_res.map(|r| r.tick_end).unwrap_or(0),
                data_completeness: bhv_res.map(|r| r.data_completeness).unwrap_or(0.0),
            });

            run_results.push((vx, vy, config_hash, res, zone));
        }
        print!(".");
        let _ = std::io::stdout().flush();
    }

    let matrix_warnings = detect_warnings(
        &run_results
            .iter()
            .map(|(_, _, _, r, _)| r.clone())
            .collect::<Vec<_>>(),
        scenario_id,
    );

    for (vx, vy, config_hash, res, zone) in run_results {
        let scenario_status = if res.collapsed { "collapsed" } else { "stable" };
        let mut row_warnings = Vec::new();
        if res.resource_balance_error > 0.01 || res.energy_balance_error > 0.01 {
            row_warnings.push("BALANCE_ERROR".to_string());
        }
        for w in &matrix_warnings {
            row_warnings.push(w.clone());
        }
        let warning_codes = if row_warnings.is_empty() {
            "none".to_string()
        } else {
            row_warnings.join("|")
        };

        let active_fraction = if res.ticks_executed > 0 {
            res.active_ticks as f32 / res.ticks_executed as f32
        } else {
            0.0
        };
        let dormant_fraction = if res.ticks_executed > 0 {
            res.dormant_ticks as f32 / res.ticks_executed as f32
        } else {
            0.0
        };

        writeln!(
            csv,
            "{},1.0,{},{},{},{},{},{:.4},{},{:.4},{},{},{},{},{},{},{},{},{:.4},{},{:.4},{},{},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4}",
            scenario_id,
            config_hash,
            cfg.run.seed,
            cfg.run.ticks,
            res.ticks_executed,
            mat.param_x,
            vx,
            mat.param_y,
            vy,
            zone,
            scenario_status,
            warning_codes,
            !res.collapsed,
            res.ticks_executed,
            res.collapse_tick.unwrap_or(0),
            res.death_reason,
            res.active_ticks,
            active_fraction,
            res.dormant_ticks,
            dormant_fraction,
            res.dormancy_enter_count,
            res.dormancy_exit_count,
            res.initial_energy,
            res.final_energy,
            res.min_energy,
            res.max_energy,
            res.mean_energy,
            res.energy_produced,
            res.passive_energy_received,
            res.energy_spent_upkeep,
            res.energy_spent_dormant_upkeep,
            res.energy_spent_movement,
            res.energy_spent_growth,
            res.energy_spent_repair,
            res.energy_spent_division,
            res.initial_world_resource,
            res.final_world_resource,
            res.resource_regenerated,
            res.resource_absorbed,
            res.resource_metabolized,
            res.internal_resource_final,
            res.resource_released,
            res.resource_explicit_sink,
            res.resource_balance_error,
            res.energy_balance_error,
            res.ticks_per_second,
            res.explicit_energy_loss,
            res.death_cleanup_loss_energy,
            res.death_cleanup_loss_resources,
            res.clamping_loss,
            res.unpaid_mandatory_cost,
            res.resource_decay,
            res.resource_sink,
            res.numerical_error_energy,
            res.numerical_error_resources,
            res.unclassified_loss_energy,
            res.unclassified_loss_resources
        )
        .unwrap();
    }

    println!();
    let stats = compute_stats(&all_finals);
    println!("  ── Aggregate (final_energy across matrix) ────────────");
    println!(
        "  min={:.2}  max={:.2}  mean={:.2}  ideal=[{:.2}..{:.2}]",
        stats.min, stats.max, stats.mean, stats.ideal_range_lo, stats.ideal_range_hi
    );
    println!("  Zone distribution: {:?}", zone_counts);
    println!("  CSV → {}", csv_path);

    write_summary_row(out_dir, &mat.name, scenario_id, &stats, &zone_counts);
    records
}

// ─────────────────────────────────────────────────────────────────────────────
// Write the Markdown report that links all generated CSV files
// ─────────────────────────────────────────────────────────────────────────────

fn write_report(cfg: &AnalyzerConfig, out_dir: &str, records: &[ClassificationRecord]) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let report_path = format!("{}/sweep_report_{}.md", out_dir, ts);
    let mut f = std::fs::File::create(&report_path).expect("cannot create report");

    writeln!(f, "# Sweep Analyzer Report").unwrap();
    writeln!(f).unwrap();
    writeln!(f, "- **seed**: {}", cfg.run.seed).unwrap();
    writeln!(f, "- **ticks per run**: {}", cfg.run.ticks).unwrap();
    writeln!(f, "- **output_dir**: `{}`", cfg.run.output_dir).unwrap();
    writeln!(f).unwrap();
    writeln!(f, "## Detailed Accounting Categories").unwrap();
    writeln!(f, "The following new accounting category fields are tracked in the simulation results:").unwrap();
    writeln!(f, "- **explicit_energy_loss**: Energy explicitly removed/sunk.").unwrap();
    writeln!(f, "- **death_cleanup_loss_energy**: Remaining energy of cells upon death.").unwrap();
    writeln!(f, "- **death_cleanup_loss_resources**: Remaining resource inventory and materials of cells upon death.").unwrap();
    writeln!(f, "- **clamping_loss**: Energy lost due to maximum capacity limits and clamping.").unwrap();
    writeln!(f, "- **unpaid_mandatory_cost**: Mandatory upkeep cost that could not be paid due to energy depletion.").unwrap();
    writeln!(f, "- **resource_decay**: Environmental resource decay over time.").unwrap();
    writeln!(f, "- **resource_sink**: Resources explicitly removed/sunk.").unwrap();
    writeln!(f, "- **numerical_error_energy**: Energy balance error below the tolerance threshold (< 0.01).").unwrap();
    writeln!(f, "- **numerical_error_resources**: Resource balance error below the tolerance threshold (< 0.01).").unwrap();
    writeln!(f, "- **unclassified_loss_energy**: Energy balance error exceeding the tolerance threshold.").unwrap();
    writeln!(f, "- **unclassified_loss_resources**: Resource balance error exceeding the tolerance threshold.").unwrap();
    writeln!(f).unwrap();
    writeln!(f, "## Zone Legend").unwrap();
    writeln!(f, "| Zone | Meaning |").unwrap();
    writeln!(f, "|---|---|").unwrap();
    writeln!(f, "| `collapse` | Cell died before end of run |").unwrap();
    writeln!(
        f,
        "| `dormancy` | > 80 % ticks in dormancy, survival only |"
    )
    .unwrap();
    writeln!(
        f,
        "| `dormancy_survival` | 20–80 % dormancy, fragile active periods |"
    )
    .unwrap();
    writeln!(f, "| `stable` | Active with steady energy |").unwrap();
    writeln!(f, "| `accumulates` | Active and growing energy buffer |").unwrap();
    writeln!(f).unwrap();
    writeln!(f, "## Sweeps").unwrap();

    if let Some(sweeps) = &cfg.sweep {
        for s in sweeps {
            writeln!(
                f,
                "- **{}**: param `{}` from `{:.3}` to `{:.3}` in {} steps → `{}/{}.csv`",
                s.name, s.param, s.from, s.to, s.steps, out_dir, s.name
            )
            .unwrap();
        }
    }

    writeln!(f).unwrap();
    writeln!(f, "## Matrices").unwrap();

    if let Some(matrices) = &cfg.matrix {
        for m in matrices {
            writeln!(
                f,
                "- **{}**: `{}` × `{}`, grid {}×{} → `{}/{}_matrix.csv`",
                m.name, m.param_x, m.param_y, m.steps_x, m.steps_y, out_dir, m.name
            )
            .unwrap();
        }
    }

    writeln!(f).unwrap();
    writeln!(f, "## Performance analysis").unwrap();
    writeln!(f).unwrap();
    writeln!(f, "| Sweep Name | Avg Ticks/Sec |").unwrap();
    writeln!(f, "|---|---|").unwrap();
    let mut sweep_perf: HashMap<String, (f32, usize)> = HashMap::new();
    for r in records {
        let entry = sweep_perf.entry(r.sweep_name.clone()).or_insert((0.0, 0));
        entry.0 += r.ticks_per_second;
        entry.1 += 1;
    }
    let mut sorted_sweeps: Vec<_> = sweep_perf.into_iter().collect();
    sorted_sweeps.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, (sum_tps, count)) in sorted_sweeps {
        let avg = if count > 0 {
            sum_tps / count as f32
        } else {
            0.0
        };
        writeln!(f, "| {} | {:.2} |", name, avg).unwrap();
    }

    writeln!(f).unwrap();
    writeln!(f, "> Aggregated statistics are saved in `sweep_scenario_summary.csv` containing `min`, `max`, `mean`, and `ideal_range`.").unwrap();

    println!("\n  Report → {}", report_path);
}

// ─────────────────────────────────────────────────────────────────────────────
// main
// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    // allow override: cargo run --bin sweep_analyzer -- path/to/other.toml
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config/analyzer/sweep_analyzer.toml".to_string());

    println!("ALife Sweep Analyzer");
    println!("Config: {}", config_path);

    let raw = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|e| panic!("Cannot read {}: {}", config_path, e));

    let cfg: AnalyzerConfig =
        toml::from_str(&raw).unwrap_or_else(|e| panic!("Cannot parse {}: {}", config_path, e));

    let allowed_scenarios = [
        "finite_resource_viability",
        "passive_income_survival",
        "steady_resource_flow",
        "dormancy_survival",
        "resource_abundance",
    ];

    if let Some(sweeps) = &cfg.sweep {
        for sweep in sweeps {
            let sc = sweep.scenario.as_deref().unwrap_or("");
            if sc.is_empty() || sc == "none" || !allowed_scenarios.contains(&sc) {
                eprintln!(
                    "Validation error: sweep '{}' has invalid, empty, or missing scenario '{}'. Allowed scenarios: {:?}",
                    sweep.name, sc, allowed_scenarios
                );
                std::process::exit(1);
            }
            if let Some(scenarios) = &cfg.scenarios {
                if !scenarios.contains_key(sc) {
                    eprintln!(
                        "Validation error: scenario preset '{}' specified in sweep '{}' does not exist under scenarios presets in configuration.",
                        sc, sweep.name
                    );
                    std::process::exit(1);
                }
            }
        }
    }

    if let Some(matrices) = &cfg.matrix {
        for mat in matrices {
            let sc = mat.scenario.as_deref().unwrap_or("");
            if sc.is_empty() || sc == "none" || !allowed_scenarios.contains(&sc) {
                eprintln!(
                    "Validation error: matrix '{}' has invalid, empty, or missing scenario '{}'. Allowed scenarios: {:?}",
                    mat.name, sc, allowed_scenarios
                );
                std::process::exit(1);
            }
            if let Some(scenarios) = &cfg.scenarios {
                if !scenarios.contains_key(sc) {
                    eprintln!(
                        "Validation error: scenario preset '{}' specified in matrix '{}' does not exist under scenarios presets in configuration.",
                        sc, mat.name
                    );
                    std::process::exit(1);
                }
            }
        }
    }

    std::fs::create_dir_all(&cfg.run.output_dir).expect("cannot create output_dir");

    println!(
        "ticks={} seed={} output={}",
        cfg.run.ticks, cfg.run.seed, cfg.run.output_dir
    );

    // Load classifiers
    let _registry = alife::observer::config::load_classification_registry(
        "config/observer/classification-registry.toml",
    )
    .expect("Failed to load classification registry");
    let _role_classifier = get_role_classifier();
    let _behavior_classifier = get_behavior_classifier();

    let mut all_records = Vec::new();

    if let Some(sweeps) = &cfg.sweep {
        let out = cfg.run.output_dir.clone();
        for sweep in sweeps {
            let preset = sweep.scenario.as_ref().and_then(|name| {
                cfg.scenarios
                    .as_ref()
                    .and_then(|scenarios| scenarios.get(name))
            });
            let recs = run_sweep(&cfg, sweep, preset, &out);
            all_records.extend(recs);
        }
    }

    if let Some(matrices) = &cfg.matrix {
        let out = cfg.run.output_dir.clone();
        for mat in matrices {
            let preset = mat.scenario.as_ref().and_then(|name| {
                cfg.scenarios
                    .as_ref()
                    .and_then(|scenarios| scenarios.get(name))
            });
            let recs = run_matrix(&cfg, mat, preset, &out);
            all_records.extend(recs);
        }
    }

    // Now write reports/behavior_profiles.csv
    std::fs::create_dir_all("outputs/raw_data").ok();
    std::fs::create_dir_all("outputs/reports").ok();

    let behavior_profiles_csv_path = "outputs/raw_data/behavior_profiles.csv";
    let mut bp_csv = std::fs::File::create(behavior_profiles_csv_path)
        .expect("cannot create behavior_profiles.csv");
    writeln!(
        bp_csv,
        "sweep_name,scenario_id,parameter_name,parameter_value,secondary_parameter_name,secondary_parameter_value,potential_role,observed_role,behavior_profile,ticks_per_second,\
         classification_mode,status,primary_label,secondary_labels,score,confidence,evidence_summary,classifier_version,tick_start,tick_end,data_completeness"
    )
    .unwrap();

    for r in &all_records {
        writeln!(
            bp_csv,
            "{},{},{},{:.4},{},{:.4},{},{},{},{:.4},{},{},{},\"{}\",\
             {:.4},{:.4},\"{}\",{},{},{},{:.4}",
            r.sweep_name,
            r.scenario_id,
            r.parameter_name,
            r.parameter_value,
            r.secondary_parameter_name,
            r.secondary_parameter_value,
            r.potential_role,
            r.observed_role,
            r.behavior_profile,
            r.ticks_per_second,
            r.classification_mode,
            r.status,
            r.primary_label,
            r.secondary_labels,
            r.score,
            r.confidence,
            r.evidence_summary.replace("\"", "\"\""),
            r.classifier_version,
            r.tick_start,
            r.tick_end,
            r.data_completeness
        )
        .unwrap();
    }

    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // CSV in reports
    let bp_reports_csv_path = format!("outputs/reports/behavior-profiles-{}.csv", ts);
    std::fs::copy(behavior_profiles_csv_path, &bp_reports_csv_path).ok();

    // JSON in reports
    let bp_reports_json_path = format!("outputs/reports/behavior-profiles-{}.json", ts);
    let json_content =
        serde_json::to_string_pretty(&all_records).unwrap_or_else(|_| "[]".to_string());
    std::fs::write(&bp_reports_json_path, json_content).ok();

    // Markdown report in reports
    let bp_reports_md_path = format!("outputs/reports/behavior-profiles-{}.md", ts);
    let mut bp_md = std::fs::File::create(&bp_reports_md_path)
        .expect("cannot create behavior-profiles md report");
    writeln!(bp_md, "# Behavior Profiles Report").unwrap();
    writeln!(bp_md).unwrap();
    writeln!(bp_md, "| Sweep Name | Scenario | Parameter | Value | Sec Param | Sec Value | Potential Role | Observed Role | Behavior Profile | Speed (t/s) | Mode | Status | Primary | Secondary | Score | Conf | Evidence | Ver | Start | End | Complete |").unwrap();
    writeln!(bp_md, "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    for r in &all_records {
        writeln!(
            bp_md,
            "| {} | {} | {} | {:.4} | {} | {:.4} | {} | {} | {} | {:.2} | {} | {} | {} | {} | {:.4} | {:.4} | {} | {} | {} | {} | {:.4} |",
            r.sweep_name,
            r.scenario_id,
            r.parameter_name,
            r.parameter_value,
            r.secondary_parameter_name,
            r.secondary_parameter_value,
            r.potential_role,
            r.observed_role,
            r.behavior_profile,
            r.ticks_per_second,
            r.classification_mode,
            r.status,
            r.primary_label,
            r.secondary_labels,
            r.score,
            r.confidence,
            r.evidence_summary,
            r.classifier_version,
            r.tick_start,
            r.tick_end,
            r.data_completeness
        )
        .unwrap();
    }

    write_report(&cfg, &cfg.run.output_dir.clone(), &all_records);

    println!(
        "\n✓ Sweep Analyzer finished. Results in: {}/",
        cfg.run.output_dir
    );
}
