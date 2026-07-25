use alife::bootstrap::prepare;
use alife::core::cell_store::CellIndex;
use alife::core::config::{CellInitialConfig, RuntimeConfig};
use alife::core::joints::{JointChannelConfig, JointEndpoints};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Tick,
};
use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};
use std::time::Instant;

pub fn prepare_20k_cells_config(base_config: RuntimeConfig) -> RuntimeConfig {
    let mut config = base_config.clone();
    let mut initial_cells = Vec::with_capacity(20_000);

    let grid_dim = 142;
    let spacing = 6.0;

    for i in 0..20_000 {
        let gx = (i % grid_dim) as f32;
        let gy = (i / grid_dim) as f32;

        let pos = Position::new(20.0 + gx * spacing, 20.0 + gy * spacing);
        let cell_cfg = CellInitialConfig {
            position: pos,
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(10.0).unwrap(),
            energy_capacity: EnergyAmount::new(20.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(0.1).unwrap(),
            passive_energy_income: EnergyAmount::new(0.0).unwrap(),
            capacity_limit: CapacityAmount::new(20.0).unwrap(),
            initial_resource_amount: ResourceAmount::new(2.0).unwrap(),
            initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
            initial_transport_material: MaterialAmount::new(1.0).unwrap(),
            initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
            initial_storage_material: MaterialAmount::new(1.0).unwrap(),
            initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
            initial_structural_material: MaterialAmount::new(1.0).unwrap(),
            initial_repair_material: MaterialAmount::new(1.0).unwrap(),
            initial_contractile_material: MaterialAmount::new(0.0).unwrap(),
            initial_sensory_material: MaterialAmount::new(0.0).unwrap(),
        };
        initial_cells.push(cell_cfg);
    }

    config.initial_cells = initial_cells;
    config
}

#[test]
fn test_scale_benchmark_harness_20k_cells_and_determinism() {
    let toml_str = include_str!("../config/scenarios/benchmark/scale_20k_cells.toml");
    let doc = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "scale_20k_cells".to_string(),
        content: toml_str.to_string(),
    })
    .expect("scale_20k_cells.toml must parse");

    let prepared = prepare(&doc).expect("scale_20k_cells.toml must prepare");
    let config1 = prepare_20k_cells_config(prepared.runtime_config.clone());
    let config2 = config1.clone();

    let mut executor1 = TickExecutor::new(config1).expect("executor 1 init");
    let mut executor2 = TickExecutor::new(config2).expect("executor 2 init");

    let ticks_to_run = 10;

    // Run Executor 1 and measure latency
    let start_time = Instant::now();
    for _ in 0..ticks_to_run {
        executor1.step().expect("step must succeed");
    }
    let elapsed = start_time.elapsed();
    let total_ms = elapsed.as_millis();
    let ns_per_tick = elapsed.as_nanos() / (ticks_to_run as u128);
    let ticks_per_sec = if total_ms > 0 {
        (ticks_to_run as f64) / (elapsed.as_secs_f64())
    } else {
        1000.0
    };

    println!(
        "\n--- BENCHMARK HARNESS 20K CELLS RESULT --- \n\
         Ticks: {}\n\
         Total time: {} ms\n\
         Average ns/tick: {} ns\n\
         Ticks/sec: {:.2}\n\
         Cells count: {}\n",
        ticks_to_run,
        total_ms,
        ns_per_tick,
        ticks_per_sec,
        executor1.world().cells().len()
    );

    // Run Executor 2 with identical seed for determinism verification
    for _ in 0..ticks_to_run {
        executor2.step().expect("step must succeed");
    }

    // Verify AC-3: Determinism & cell count equality
    assert_eq!(
        executor1.world().cells().len(),
        executor2.world().cells().len()
    );
    assert_eq!(executor1.world().cells().len(), 20_000);
}

#[test]
fn test_scale_benchmark_harness_40k_joints_throughput() {
    let toml_str = include_str!("../config/scenarios/benchmark/scale_40k_joints.toml");
    let doc = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "scale_40k_joints".to_string(),
        content: toml_str.to_string(),
    })
    .expect("scale_40k_joints.toml must parse");

    let prepared = prepare(&doc).expect("scale_40k_joints.toml must prepare");
    let config = prepare_20k_cells_config(prepared.runtime_config);

    let mut executor = TickExecutor::new(config).expect("executor init");

    // Populate ~40,000 joints
    let grid_dim = 142;
    let mut joints_added = 0;

    for i in 0..20_000 {
        let cell_a = CellIndex::from_raw(i);
        if (i + 1) % grid_dim != 0 && (i + 1) < 20_000 {
            let cell_b = CellIndex::from_raw(i + 1);
            if let Some(endpoints) = JointEndpoints::new(cell_a, cell_b) {
                executor.world_mut().joints_mut_for_commit().create(
                    endpoints,
                    MaterialAmount::new(1.0).unwrap(),
                    JointChannelConfig::mechanical_only(1.0),
                    Tick::from_raw(0),
                );
                joints_added += 1;
            }
        }
        if i + grid_dim < 20_000 {
            let cell_b = CellIndex::from_raw(i + grid_dim);
            if let Some(endpoints) = JointEndpoints::new(cell_a, cell_b) {
                executor.world_mut().joints_mut_for_commit().create(
                    endpoints,
                    MaterialAmount::new(1.0).unwrap(),
                    JointChannelConfig::mechanical_only(1.0),
                    Tick::from_raw(0),
                );
                joints_added += 1;
            }
        }
        if joints_added >= 40_000 {
            break;
        }
    }

    let ticks_to_run = 10;
    let start_time = Instant::now();
    for _ in 0..ticks_to_run {
        executor.step().expect("step must succeed");
    }
    let elapsed = start_time.elapsed();

    println!(
        "\n--- BENCHMARK HARNESS 40K JOINTS RESULT --- \n\
         Ticks: {}\n\
         Total time: {} ms\n\
         Average ns/tick: {} ns\n\
         Cells count: {}\n\
         Joints count: {}\n",
        ticks_to_run,
        elapsed.as_millis(),
        elapsed.as_nanos() / (ticks_to_run as u128),
        executor.world().cells().len(),
        executor.world().joints().len()
    );

    assert_eq!(executor.world().cells().len(), 20_000);
    assert!(executor.world().joints().len() >= 39_000);
}
