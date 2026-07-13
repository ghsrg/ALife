use alife::core::tick::TickExecutor;
use alife::runner::config_parser::RawScenarioConfig;

fn load(path: &str) -> alife::core::config::RuntimeConfig {
    let text = std::fs::read_to_string(path).unwrap();
    RawScenarioConfig::parse(&text).unwrap()
}

fn run_for_ticks(path: &str, ticks: u64) -> alife::core::summary::RunSummary {
    let mut config = load(path);
    config.world.tick_count = alife::core::units::Tick::from_raw(ticks);
    let mut executor = TickExecutor::new(config).unwrap();
    let mut summary = executor.step().unwrap();
    let mut resource_diffused_amount = summary.metrics.resource_diffused_amount;
    let mut repair_success_count = summary.metrics.repair_success_count;
    let mut fragment_created_amount = summary.metrics.fragment_created_amount;
    let mut fragment_converted_amount = summary.metrics.fragment_converted_amount;
    let mut joint_created_count = summary.metrics.joint_created_count;
    let mut joint_broken_count = summary.metrics.joint_broken_count;
    let mut joint_degradation_amount = summary.metrics.joint_degradation_amount;
    for _ in 1..ticks {
        summary = executor.step().unwrap();
        resource_diffused_amount += summary.metrics.resource_diffused_amount;
        repair_success_count += summary.metrics.repair_success_count;
        fragment_created_amount += summary.metrics.fragment_created_amount;
        fragment_converted_amount += summary.metrics.fragment_converted_amount;
        joint_created_count += summary.metrics.joint_created_count;
        joint_broken_count += summary.metrics.joint_broken_count;
        joint_degradation_amount += summary.metrics.joint_degradation_amount;
    }
    summary.metrics.resource_diffused_amount = resource_diffused_amount;
    summary.metrics.repair_success_count = repair_success_count;
    summary.metrics.fragment_created_amount = fragment_created_amount;
    summary.metrics.fragment_converted_amount = fragment_converted_amount;
    summary.metrics.joint_created_count = joint_created_count;
    summary.metrics.joint_broken_count = joint_broken_count;
    summary.metrics.joint_degradation_amount = joint_degradation_amount;
    summary
}

const ACCOUNTING_TOLERANCE: f32 = 0.05;

#[test]
fn baseline_world_runs_10k_ticks_with_clean_accounting_and_bounded_population() {
    let summary = run_for_ticks("config/scenarios/world/world_baseline_stable.toml", 10_000);

    assert!(
        summary.metrics.alive_cells_count >= 1,
        "alive={} dead={} collapse={:?} tick={} energy={} loss={} gain={} fragments_created={} fragments_converted={} repair={} joints={} joint_deg={}",
        summary.metrics.alive_cells_count,
        summary.metrics.dead_cells_count,
        summary.collapse_reason,
        summary.tick.raw(),
        summary.metrics.final_energy,
        summary.metrics.integrated_matter_unclassified_loss,
        summary.metrics.integrated_matter_unclassified_gain,
        summary.metrics.fragment_created_amount,
        summary.metrics.fragment_converted_amount,
        summary.metrics.repair_success_count,
        summary.metrics.joint_created_count,
        summary.metrics.joint_degradation_amount
    );
    assert!(summary.metrics.alive_cells_count <= 64);
    assert!(
        summary.metrics.integrated_matter_unclassified_loss <= ACCOUNTING_TOLERANCE,
        "loss={} gain={} alive={} dead={} fragments_created={} fragments_converted={} repair={} joints={} joint_deg={}",
        summary.metrics.integrated_matter_unclassified_loss,
        summary.metrics.integrated_matter_unclassified_gain,
        summary.metrics.alive_cells_count,
        summary.metrics.dead_cells_count,
        summary.metrics.fragment_created_amount,
        summary.metrics.fragment_converted_amount,
        summary.metrics.repair_success_count,
        summary.metrics.joint_created_count,
        summary.metrics.joint_degradation_amount
    );
    assert!(
        summary.metrics.integrated_matter_unclassified_gain <= ACCOUNTING_TOLERANCE,
        "loss={} gain={} before={} after={} internal_resource={} external_resource={} final_energy={}",
        summary.metrics.integrated_matter_unclassified_loss,
        summary.metrics.integrated_matter_unclassified_gain,
        summary.metrics.integrated_matter_before,
        summary.metrics.integrated_matter_after,
        summary.metrics.final_internal_resources,
        summary.metrics.final_external_resources,
        summary.metrics.final_energy
    );
    assert!(summary.metrics.resource_diffused_amount > 0.0);
    assert!(summary.metrics.repair_success_count > 0);
    assert!(summary.metrics.fragment_created_amount > 0.0);
    assert!(summary.metrics.fragment_converted_amount > 0.0);
    assert!(summary.metrics.joint_created_count > 0);
    assert!(
        summary.metrics.joint_broken_count > 0 || summary.metrics.joint_degradation_amount > 0.0
    );
}

#[test]
fn baseline_world_replay_is_deterministic() {
    let first = run_for_ticks("config/scenarios/world/world_baseline_stable.toml", 2_000);
    let second = run_for_ticks("config/scenarios/world/world_baseline_stable.toml", 2_000);

    assert_eq!(first.config_hash, second.config_hash);
    assert_eq!(
        first.metrics.alive_cells_count,
        second.metrics.alive_cells_count
    );
    assert_eq!(
        first.metrics.dead_cells_count,
        second.metrics.dead_cells_count
    );
    assert_eq!(
        first.metrics.divisions_count,
        second.metrics.divisions_count
    );
    assert_eq!(
        first.metrics.joint_created_count,
        second.metrics.joint_created_count
    );
    assert!((first.metrics.final_energy - second.metrics.final_energy).abs() < 0.0001);
}

#[test]
fn stress_world_fails_deterministically_without_accounting_leak() {
    let first = run_for_ticks("config/scenarios/world/world_stress_regression.toml", 1_000);
    let second = run_for_ticks("config/scenarios/world/world_stress_regression.toml", 1_000);

    assert_eq!(first.collapse_reason, second.collapse_reason);
    assert_eq!(
        first.metrics.dead_cells_count,
        second.metrics.dead_cells_count
    );
    assert!(first.metrics.integrated_matter_unclassified_loss <= ACCOUNTING_TOLERANCE);
    assert!(
        first.metrics.integrated_matter_unclassified_gain <= ACCOUNTING_TOLERANCE,
        "collapse={:?} dead={} tick={} loss={} gain={}",
        first.collapse_reason,
        first.metrics.dead_cells_count,
        first.tick.raw(),
        first.metrics.integrated_matter_unclassified_loss,
        first.metrics.integrated_matter_unclassified_gain
    );
}
