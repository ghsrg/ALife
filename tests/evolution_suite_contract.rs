use alife::observer::evolution_suite::{
    EvolutionRunOutcome, LineageFrequencyWindow, evaluate_evolution_suite,
};
use std::collections::HashMap;

#[test]
fn test_evolution_suite_outcome_classification() {
    let mut lineage1 = HashMap::new();
    lineage1.insert("lin-A".to_string(), 15);
    lineage1.insert("lin-B".to_string(), 5);

    let window = LineageFrequencyWindow {
        tick_start: 0,
        tick_end: 1000,
        lineage_counts: lineage1,
        observed_frequency_shift: 0.25,
    };

    // Test Stable outcome
    let pop_history_stable = vec![(0, 20), (5000, 25), (10000, 30)];
    let res_stable = evaluate_evolution_suite(
        "long_run_stable",
        42,
        10000,
        10,
        &pop_history_stable,
        vec![window.clone()],
    );
    assert_eq!(res_stable.outcome, EvolutionRunOutcome::Stable);
    assert_eq!(res_stable.final_population, 30);
    assert_eq!(res_stable.windows.len(), 1);

    // Test Collapse outcome
    let pop_history_collapse = vec![(0, 20), (2000, 0)];
    let res_collapse = evaluate_evolution_suite(
        "long_run_stress",
        42,
        5000,
        10,
        &pop_history_collapse,
        vec![window],
    );
    assert_eq!(res_collapse.outcome, EvolutionRunOutcome::Collapse);
    assert_eq!(res_collapse.final_population, 0);

    // Test Fragile outcome
    let pop_history_fragile = vec![(0, 20), (5000, 5)];
    let res_fragile = evaluate_evolution_suite(
        "long_run_stress",
        42,
        10000,
        10,
        &pop_history_fragile,
        vec![],
    );
    assert_eq!(res_fragile.outcome, EvolutionRunOutcome::Fragile);
}
