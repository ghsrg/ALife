use alife::observer::monitor_payloads::build_monitor_data_panel_projection;
use alife::runner::engine::{RunEngine, RunEngineConfig};
use alife::runner::scenario::{load_scenario_document, scan_scenarios};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

fn classify_cell_role(materials: &[f32; 9]) -> &'static str {
    let roles = [
        "Boundary",
        "Transport",
        "Metabolic",
        "Storage",
        "Synthesis",
        "Structural",
        "Repair",
        "Contractile",
        "Sensory",
    ];
    let mut max_val = -1.0f32;
    let mut max_idx = 0usize;
    for (i, &amt) in materials.iter().enumerate() {
        if amt > max_val {
            max_val = amt;
            max_idx = i;
        }
    }
    if max_val <= 0.0 {
        "Unclassified"
    } else {
        roles[max_idx]
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===================================================================================");
    println!(" ALife Verification & Balance Suite: canonical_living_world.toml (10,000 Ticks)");
    println!("===================================================================================");

    let scenarios_dir = PathBuf::from("config/scenarios");
    let scenarios = scan_scenarios(&scenarios_dir)?;
    let meta = scenarios
        .into_iter()
        .find(|s| s.id == "canonical_living_world")
        .ok_or("Scenario 'canonical_living_world' not found in config/scenarios")?;

    println!("[verify] Loading scenario: {} ({})", meta.id, meta.path.display());
    let document = load_scenario_document(&meta)?;
    let engine_config = RunEngineConfig::default();
    let mut engine = RunEngine::prepare_from_document(&document, engine_config)?;

    let target_ticks = 10000;
    let sample_interval = 1000;
    println!("[verify] Running simulation for {} ticks with role & division tracking...", target_ticks);
    engine.start()?;

    let start_time = Instant::now();
    let initial_cell_count = engine.latest_committed_snapshot().cells.len() as u32;

    struct SampleData {
        tick: u64,
        alive: u32,
        dead: u32,
        role_counts: HashMap<&'static str, usize>,
        generations: u32,
        carriers: u32,
        env_res: f32,
        utilization: f32,
    }

    let mut samples: Vec<SampleData> = Vec::new();

    let sample_engine = |eng: &mut RunEngine| -> SampleData {
        let snapshot = eng.latest_committed_snapshot();
        let monitor = build_monitor_data_panel_projection(&snapshot, "verify-canonical-run");
        let mut role_counts = HashMap::new();
        for c in &snapshot.cells {
            if c.lifecycle_state == alife::core::cell_store::LifecycleState::Alive {
                let role = classify_cell_role(&c.materials);
                *role_counts.entry(role).or_insert(0) += 1;
            }
        }
        SampleData {
            tick: eng.current_tick(),
            alive: monitor.payload.world.population_lifecycle.alive,
            dead: monitor.payload.world.population_lifecycle.dead,
            role_counts,
            generations: monitor.payload.evolution.total_generations,
            carriers: monitor.payload.evolution.active_carriers_count,
            env_res: monitor.payload.world.resource_cycle.locations.environment,
            utilization: monitor.payload.world.energy_flow.utilization_rate,
        }
    };

    samples.push(sample_engine(&mut engine));

    while engine.current_tick() < target_ticks {
        let current = engine.current_tick();
        let step_count = sample_interval.min(target_ticks - current);
        for _ in 0..step_count {
            engine.run_one_tick()?;
        }
        samples.push(sample_engine(&mut engine));
    }

    let elapsed = start_time.elapsed().as_secs_f32();
    let tps = target_ticks as f32 / elapsed;

    println!("\n[verify] Execution finished in {:.2}s ({:.0} TPS)\n", elapsed, tps);
    println!("------------------------------------------------------------------------------------------------------------------");
    println!("| Tick  | Alive | Dead | Gen | Carriers | Bnd | Trn | Met | Strg | Syn | Strc | Rpr | Cnt | Sns | Env Res  | Util % |");
    println!("------------------------------------------------------------------------------------------------------------------");

    let mut min_alive = u32::MAX;
    let mut max_alive = 0u32;
    let mut reproduction_occurred = false;

    for s in &samples {
        if s.alive < min_alive {
            min_alive = s.alive;
        }
        if s.alive > max_alive {
            max_alive = s.alive;
        }
        if s.alive > initial_cell_count || s.generations > 1 || s.carriers > initial_cell_count {
            reproduction_occurred = true;
        }

        let get_r = |k: &'static str| *s.role_counts.get(k).unwrap_or(&0);
        println!(
            "| {:5} | {:5} | {:4} | {:3} | {:8} | {:3} | {:3} | {:3} | {:4} | {:3} | {:4} | {:3} | {:3} | {:3} | {:8.1} | {:5.1}% |",
            s.tick,
            s.alive,
            s.dead,
            s.generations,
            s.carriers,
            get_r("Boundary"),
            get_r("Transport"),
            get_r("Metabolic"),
            get_r("Storage"),
            get_r("Synthesis"),
            get_r("Structural"),
            get_r("Repair"),
            get_r("Contractile"),
            get_r("Sensory"),
            s.env_res,
            s.utilization * 100.0
        );
    }
    println!("------------------------------------------------------------------------------------------------------------------");

    let final_sample = samples.last().unwrap();
    let final_alive = final_sample.alive;

    println!("\n=== CELL ROLE BALANCING AUDIT (TICK {}) ===", target_ticks);
    let roles_list = [
        "Boundary",
        "Transport",
        "Metabolic",
        "Storage",
        "Synthesis",
        "Structural",
        "Repair",
        "Contractile",
        "Sensory",
    ];
    let mut active_roles_count = 0;
    for role in roles_list {
        let cnt = *final_sample.role_counts.get(role).unwrap_or(&0);
        if cnt > 0 {
            active_roles_count += 1;
        }
        println!("  {:12}: {:2} cells ({:.1}%)", role, cnt, if final_alive > 0 { (cnt as f32 / final_alive as f32) * 100.0 } else { 0.0 });
    }

    println!("\n=== REPRODUCTION & POPULATION DYNAMICS ===");
    println!("  Initial cells:     {}", initial_cell_count);
    println!("  Max living cells:  {}", max_alive);
    println!("  Final living cells:{}", final_alive);
    println!("  Active Carriers:   {}", final_sample.carriers);
    println!("  Reproduction:      {}", if reproduction_occurred { "YES (REPRODUCTION OBSERVED)" } else { "NO" });

    // VERIFICATION ASSERTIONS
    assert!(min_alive > 0, "EXTINCTION ERROR: Population dropped to 0!");
    assert!(final_alive > 0, "EXTINCTION ERROR: Final population is 0!");
    assert!(active_roles_count >= 5, "DIVERSITY ERROR: Cell roles are collapsed into fewer than 5 active types!");

    println!("\n===================================================================================");
    println!(" VERIFICATION RESULT: SUCCESS");
    println!(" Population: min={}, max={}, final={}, active_roles={}/9, reproduction={}",
        min_alive, max_alive, final_alive, active_roles_count, reproduction_occurred
    );
    println!("===================================================================================");

    let report_json = json!({
        "scenario_id": "canonical_living_world",
        "ticks_verified": target_ticks,
        "execution_seconds": elapsed,
        "ticks_per_second": tps,
        "reproduction_observed": reproduction_occurred,
        "initial_cell_count": initial_cell_count,
        "population_min_alive": min_alive,
        "population_max_alive": max_alive,
        "final_alive": final_alive,
        "active_roles_count": active_roles_count,
        "role_breakdown_final": final_sample.role_counts,
    });

    let outputs_dir = PathBuf::from("outputs");
    fs::create_dir_all(&outputs_dir)?;
    let report_path = outputs_dir.join("canonical_living_world_report.json");
    fs::write(&report_path, serde_json::to_string_pretty(&report_json)?)?;
    println!("[verify] Report saved to: {}", report_path.display());

    Ok(())
}
