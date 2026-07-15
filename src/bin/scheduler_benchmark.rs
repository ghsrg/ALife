use alife::core::tick::TickExecutor;
use alife::runner::scenario::{load_scenario_document, scan_scenarios};
use std::time::Instant;

fn main() {
    let scenario_id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "demo_living_world".to_string());
    let ticks: usize = std::env::args()
        .nth(2)
        .and_then(|value| value.parse().ok())
        .unwrap_or(1000);
    let repeats: usize = std::env::args()
        .nth(3)
        .and_then(|value| value.parse().ok())
        .unwrap_or(5);

    let scenarios = scan_scenarios("config/scenarios").expect("scenario scan should succeed");
    let meta = scenarios
        .iter()
        .find(|scenario| scenario.id == scenario_id)
        .unwrap_or_else(|| panic!("unknown scenario_id: {scenario_id}"));
    let document = load_scenario_document(meta).expect("scenario should load");

    let mut results = Vec::new();
    for _ in 0..repeats {
        let mut executor =
            TickExecutor::new(document.runtime_config.clone()).expect("executor should start");
        for _ in 0..100 {
            executor.step().expect("warmup tick should commit");
        }
        let start = Instant::now();
        for _ in 0..ticks {
            executor.step().expect("benchmark tick should commit");
        }
        let elapsed = start.elapsed().as_secs_f64();
        results.push(ticks as f64 / elapsed);
    }
    results.sort_by(|a, b| a.partial_cmp(b).expect("TPS values should be finite"));
    let median = results[results.len() / 2];
    println!("scenario={scenario_id} ticks={ticks} repeats={repeats} median_tps={median:.2}");
}
