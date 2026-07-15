use alife::core::stable_state_hash::StableStateHasher;
use alife::core::tick::TickExecutor;
use alife::runner::scenario::{load_scenario_document, scan_scenarios};

fn run_state_hash(id: &str, ticks: usize) -> u64 {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios.iter().find(|scenario| scenario.id == id).unwrap();
    let document = load_scenario_document(meta).unwrap();
    let mut executor = TickExecutor::new(document.runtime_config).unwrap();
    for _ in 0..ticks {
        executor.step().unwrap();
    }
    StableStateHasher::hash_world(executor.world())
}

#[test]
fn scheduler_cadence_is_deterministic_for_same_seed_and_config() {
    assert_eq!(
        run_state_hash("demo_living_world", 200),
        run_state_hash("demo_living_world", 200)
    );
}
