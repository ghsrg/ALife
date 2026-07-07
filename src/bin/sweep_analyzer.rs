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
    pub world_size: Vec<f32>,
    pub initial_resources: Vec<f32>,
    pub decay_rate: f32,
    pub cell_position: Vec<f32>,
    pub cell_radius: f32,
    pub initial_energy: f32,
    pub energy_capacity: f32,
    pub mandatory_cost_per_tick: f32,
    pub passive_energy_income: f32,
    pub capacity_limit: f32,
    pub stress_energy_threshold: f32,
    pub dormancy_allowed: bool,
    pub dormant_mandatory_cost_modifier: f32,
    pub critical_capacity_overrun: f32,
    pub heat_dissipation_rate: f32,
    pub heat_warning_threshold: f32,
    pub heat_death_threshold: f32,
    pub waste_sink_rate: f32,
    pub waste_warning_threshold: f32,
    pub waste_death_threshold: f32,
    pub growth_enabled: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// Single simulation run result
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct SimResult {
    collapsed: bool,
    collapse_tick: Option<u32>,
    dormant_ticks: u32,
    active_ticks: u32,
    stressed_ticks: u32,
    min_energy: f32,
    max_energy: f32,
    final_energy: f32,
    total_resource_consumed: f32,
    metabolism_count: u32,
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

    let (world_w, world_h) = if let Some(p) = preset {
        (p.world_size[0], p.world_size[1])
    } else {
        (512.0, 512.0)
    };

    let (cell_x, cell_y) = if let Some(p) = preset {
        (p.cell_position[0], p.cell_position[1])
    } else {
        (256.0, 256.0)
    };

    let cell_radius_base = if let Some(p) = preset {
        p.cell_radius
    } else {
        cell_cfg.radius
    };
    let cell_radius = overrides
        .get("cell_radius")
        .copied()
        .unwrap_or(cell_radius_base);

    let initial_energy_base = if let Some(p) = preset {
        p.initial_energy
    } else {
        cell_cfg.initial_energy
    };
    let initial_energy = overrides
        .get("initial_energy")
        .copied()
        .unwrap_or(initial_energy_base);

    let energy_capacity_base = if let Some(p) = preset {
        p.energy_capacity
    } else {
        cell_cfg.energy_capacity
    };
    let energy_capacity = overrides
        .get("energy_capacity")
        .copied()
        .unwrap_or(energy_capacity_base);

    let capacity_limit_base = if let Some(p) = preset {
        p.capacity_limit
    } else {
        cell_cfg.capacity_limit
    };
    let capacity_limit = overrides
        .get("capacity_limit")
        .copied()
        .unwrap_or(capacity_limit_base);

    let decay_rate_base = if let Some(p) = preset {
        p.decay_rate
    } else {
        ri.decay_rate
    };
    let decay_rate = overrides
        .get("decay_rate")
        .copied()
        .unwrap_or(decay_rate_base);

    let stress_energy_threshold_base = if let Some(p) = preset {
        p.stress_energy_threshold
    } else {
        lc.stress_energy_threshold
    };
    let stress_energy_threshold = overrides
        .get("stress_energy_threshold")
        .copied()
        .unwrap_or(stress_energy_threshold_base);

    let critical_capacity_overrun_base = if let Some(p) = preset {
        p.critical_capacity_overrun
    } else {
        lc.critical_capacity_overrun
    };
    let critical_capacity_overrun = overrides
        .get("critical_capacity_overrun")
        .copied()
        .unwrap_or(critical_capacity_overrun_base);

    let heat_dissipation_rate_base = if let Some(p) = preset {
        p.heat_dissipation_rate
    } else {
        env.heat_dissipation_rate
    };
    let heat_dissipation_rate = overrides
        .get("heat_dissipation_rate")
        .copied()
        .unwrap_or(heat_dissipation_rate_base);

    let heat_warning_threshold_base = if let Some(p) = preset {
        p.heat_warning_threshold
    } else {
        env.heat_warning_threshold
    };
    let heat_warning_threshold = overrides
        .get("heat_warning_threshold")
        .copied()
        .unwrap_or(heat_warning_threshold_base);

    let heat_death_threshold_base = if let Some(p) = preset {
        p.heat_death_threshold
    } else {
        env.heat_death_threshold
    };
    let heat_death_threshold = overrides
        .get("heat_death_threshold")
        .copied()
        .unwrap_or(heat_death_threshold_base);

    let waste_sink_rate_base = if let Some(p) = preset {
        p.waste_sink_rate
    } else {
        env.waste_sink_rate
    };
    let waste_sink_rate = overrides
        .get("waste_sink_rate")
        .copied()
        .unwrap_or(waste_sink_rate_base);

    let waste_warning_threshold_base = if let Some(p) = preset {
        p.waste_warning_threshold
    } else {
        env.waste_warning_threshold
    };
    let waste_warning_threshold = overrides
        .get("waste_warning_threshold")
        .copied()
        .unwrap_or(waste_warning_threshold_base);

    let waste_death_threshold_base = if let Some(p) = preset {
        p.waste_death_threshold
    } else {
        env.waste_death_threshold
    };
    let waste_death_threshold = overrides
        .get("waste_death_threshold")
        .copied()
        .unwrap_or(waste_death_threshold_base);

    let resource_density_base = if let Some(p) = preset {
        p.initial_resources
            .first()
            .copied()
            .unwrap_or(ri.default_resource_density)
    } else {
        ri.default_resource_density
    };
    let resource_density = overrides
        .get("resource_density")
        .copied()
        .unwrap_or(resource_density_base);

    let passive_income_base = if let Some(p) = preset {
        p.passive_energy_income
    } else {
        cell_cfg.passive_energy_income
    };
    let passive_income = overrides
        .get("passive_energy_income")
        .copied()
        .unwrap_or(passive_income_base);

    let upkeep_base = if let Some(p) = preset {
        p.mandatory_cost_per_tick
    } else {
        cell_cfg.mandatory_cost_per_tick
    };
    let upkeep = overrides
        .get("mandatory_cost_per_tick")
        .copied()
        .unwrap_or(upkeep_base);

    let dormant_mod_base = if let Some(p) = preset {
        p.dormant_mandatory_cost_modifier
    } else {
        lc.dormant_mandatory_cost_modifier
    };
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

    let dormancy_allowed = if let Some(p) = preset {
        p.dormancy_allowed
    } else {
        lc.dormancy_allowed
    };

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
    rt.growth_enabled = preset.map(|p| p.growth_enabled).unwrap_or(false);
    rt
}

// ─────────────────────────────────────────────────────────────────────────────
// Run simulation for `ticks` steps, collect metrics
// ─────────────────────────────────────────────────────────────────────────────

fn run_simulation(rt_config: RuntimeConfig, ticks: u32) -> SimResult {
    let mut executor = match TickExecutor::new(rt_config) {
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
                total_resource_consumed: 0.0,
                metabolism_count: 0,
            };
        }
    };

    let initial_grid = executor
        .world()
        .resources()
        .total_amount_for_layer(alife::core::resources::ResourceLayerIndex::from_raw(0))
        .map(|a| a.raw())
        .unwrap_or(0.0);

    let mut min_energy = f32::MAX;
    let mut max_energy = f32::MIN;
    let mut final_energy = 0.0;
    let mut dormant_ticks = 0u32;
    let mut active_ticks = 0u32;
    let mut stressed_ticks = 0u32;
    let mut metabolism_count = 0u32;
    let mut collapsed = false;
    let mut collapse_tick = None;

    for t in 0..ticks {
        let summary = match executor.step() {
            Ok(s) => s,
            Err(_) => {
                collapsed = true;
                collapse_tick = Some(t);
                break;
            }
        };

        if summary.survival_result == SurvivalResult::Collapse {
            collapsed = true;
            collapse_tick = Some(t);
            break;
        }

        let e = summary.metrics.final_energy;
        min_energy = min_energy.min(e);
        max_energy = max_energy.max(e);
        final_energy = e;

        if summary.metrics.process_attempts > 0 {
            metabolism_count += 1;
        }

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

    let final_grid = executor
        .world()
        .resources()
        .total_amount_for_layer(alife::core::resources::ResourceLayerIndex::from_raw(0))
        .map(|a| a.raw())
        .unwrap_or(0.0);

    let total_resource_consumed = (initial_grid - final_grid).max(0.0);

    if min_energy == f32::MAX {
        min_energy = 0.0;
    }
    if max_energy == f32::MIN {
        max_energy = 0.0;
    }

    SimResult {
        collapsed,
        collapse_tick,
        dormant_ticks,
        active_ticks,
        stressed_ticks,
        min_energy,
        max_energy,
        final_energy,
        total_resource_consumed,
        metabolism_count,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Classify an outcome into a zone label
// ─────────────────────────────────────────────────────────────────────────────

fn classify(res: &SimResult, ticks: u32) -> &'static str {
    if res.collapsed {
        return "collapse";
    }
    let dormant_pct = res.dormant_ticks as f32 / ticks as f32;
    let active_pct = res.active_ticks as f32 / ticks as f32;
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

// ─────────────────────────────────────────────────────────────────────────────
// Run a 1-D sweep and write results
// ─────────────────────────────────────────────────────────────────────────────

fn run_sweep(
    cfg: &AnalyzerConfig,
    sweep: &SweepDef,
    preset: Option<&RawScenarioPreset>,
    out_dir: &str,
) {
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
        "param_value,zone,collapsed,collapse_tick,\
         dormant_ticks,active_ticks,stressed_ticks,\
         min_energy,max_energy,final_energy,mean_energy,\
         resource_consumed,metabolism_count"
    )
    .unwrap();

    let mut final_energies: Vec<f32> = Vec::new();
    let mut zone_counts: HashMap<&str, usize> = HashMap::new();

    for i in 0..sweep.steps {
        let val = sweep.from + step_size * i as f32;
        let mut overrides = HashMap::new();
        overrides.insert(sweep.param.as_str(), val);

        let rt = build_config(cfg, preset, &overrides);
        let res = run_simulation(rt, cfg.run.ticks);
        let zone = classify(&res, cfg.run.ticks);
        *zone_counts.entry(zone).or_insert(0) += 1;

        let mean_e = if res.active_ticks + res.dormant_ticks + res.stressed_ticks > 0 {
            // approximation from available data
            (res.min_energy + res.max_energy) / 2.0
        } else {
            0.0
        };

        final_energies.push(res.final_energy);

        writeln!(
            csv,
            "{:.4},{},{},{},{},{},{},{:.3},{:.3},{:.3},{:.3},{:.3},{}",
            val,
            zone,
            res.collapsed,
            res.collapse_tick
                .map(|t| t.to_string())
                .unwrap_or_else(|| "-".to_string()),
            res.dormant_ticks,
            res.active_ticks,
            res.stressed_ticks,
            res.min_energy,
            res.max_energy,
            res.final_energy,
            mean_e,
            res.total_resource_consumed,
            res.metabolism_count,
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

    // append summary block at the end of CSV
    writeln!(csv).unwrap();
    writeln!(csv, "# SUMMARY").unwrap();
    writeln!(csv, "# min_final_energy,{:.3}", stats.min).unwrap();
    writeln!(csv, "# max_final_energy,{:.3}", stats.max).unwrap();
    writeln!(csv, "# mean_final_energy,{:.3}", stats.mean).unwrap();
    writeln!(
        csv,
        "# ideal_range,{:.3},{:.3}",
        stats.ideal_range_lo, stats.ideal_range_hi
    )
    .unwrap();
    for (zone, count) in &zone_counts {
        writeln!(csv, "# zone_{},{}", zone, count).unwrap();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Run a 2-D matrix sweep and write results
// ─────────────────────────────────────────────────────────────────────────────

fn run_matrix(
    cfg: &AnalyzerConfig,
    mat: &MatrixDef,
    preset: Option<&RawScenarioPreset>,
    out_dir: &str,
) {
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
        "{},{},zone,collapsed,dormant_pct,active_pct,min_energy,max_energy,final_energy",
        mat.param_x, mat.param_y
    )
    .unwrap();

    let mut all_finals: Vec<f32> = Vec::new();
    let mut zone_counts: HashMap<&str, usize> = HashMap::new();

    for ix in 0..mat.steps_x {
        let vx = mat.from_x + step_x * ix as f32;
        for iy in 0..mat.steps_y {
            let vy = mat.from_y + step_y * iy as f32;

            let mut overrides = HashMap::new();
            overrides.insert(mat.param_x.as_str(), vx);
            overrides.insert(mat.param_y.as_str(), vy);

            let rt = build_config(cfg, preset, &overrides);
            let res = run_simulation(rt, cfg.run.ticks);
            let zone = classify(&res, cfg.run.ticks);
            *zone_counts.entry(zone).or_insert(0) += 1;

            let total_alive = res.dormant_ticks + res.active_ticks + res.stressed_ticks;
            let dormant_pct = if total_alive > 0 {
                res.dormant_ticks as f32 / total_alive as f32
            } else {
                0.0
            };
            let active_pct = if total_alive > 0 {
                res.active_ticks as f32 / total_alive as f32
            } else {
                0.0
            };

            all_finals.push(res.final_energy);

            writeln!(
                csv,
                "{:.4},{:.4},{},{},{:.3},{:.3},{:.3},{:.3},{:.3}",
                vx,
                vy,
                zone,
                res.collapsed,
                dormant_pct,
                active_pct,
                res.min_energy,
                res.max_energy,
                res.final_energy,
            )
            .unwrap();
        }
        print!(".");
        let _ = std::io::stdout().flush();
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

    writeln!(csv).unwrap();
    writeln!(csv, "# SUMMARY").unwrap();
    writeln!(csv, "# min_final_energy,{:.3}", stats.min).unwrap();
    writeln!(csv, "# max_final_energy,{:.3}", stats.max).unwrap();
    writeln!(csv, "# mean_final_energy,{:.3}", stats.mean).unwrap();
    writeln!(
        csv,
        "# ideal_range,{:.3},{:.3}",
        stats.ideal_range_lo, stats.ideal_range_hi
    )
    .unwrap();
    for (zone, count) in &zone_counts {
        writeln!(csv, "# zone_{},{}", zone, count).unwrap();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Write the Markdown report that links all generated CSV files
// ─────────────────────────────────────────────────────────────────────────────

fn write_report(cfg: &AnalyzerConfig, out_dir: &str) {
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
    writeln!(f, "> Summary rows in each CSV are prefixed with `#` and contain `min`, `max`, `mean`, and `ideal_range`.").unwrap();

    println!("\n  Report → {}", report_path);
}

// ─────────────────────────────────────────────────────────────────────────────
// main
// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    // allow override: cargo run --bin sweep_analyzer -- path/to/other.toml
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "sweep_analyzer.toml".to_string());

    println!("ALife Sweep Analyzer");
    println!("Config: {}", config_path);

    let raw = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|e| panic!("Cannot read {}: {}", config_path, e));

    let cfg: AnalyzerConfig =
        toml::from_str(&raw).unwrap_or_else(|e| panic!("Cannot parse {}: {}", config_path, e));

    std::fs::create_dir_all(&cfg.run.output_dir).expect("cannot create output_dir");

    println!(
        "ticks={} seed={} output={}",
        cfg.run.ticks, cfg.run.seed, cfg.run.output_dir
    );

    if let Some(sweeps) = &cfg.sweep {
        let out = cfg.run.output_dir.clone();
        for sweep in sweeps {
            let preset = sweep.scenario.as_ref().and_then(|name| {
                cfg.scenarios
                    .as_ref()
                    .and_then(|scenarios| scenarios.get(name))
            });
            run_sweep(&cfg, sweep, preset, &out);
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
            run_matrix(&cfg, mat, preset, &out);
        }
    }

    write_report(&cfg, &cfg.run.output_dir.clone());

    println!(
        "\n✓ Sweep Analyzer finished. Results in: {}/",
        cfg.run.output_dir
    );
}
