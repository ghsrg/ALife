use alife::observer::balance::{
    evaluate_balance, BalanceOutcome, ControlledConditions, ProfileVariables,
};

#[test]
fn test_evaluate_balance_tradeoff() {
    let cond = ControlledConditions {
        scenario_id: "test-scen".to_string(),
        scenario_version: "1.0".to_string(),
        ticks_requested: 1000,
        seed: 42,
        world_size: [64.0, 64.0],
    };

    // p1 has high survival but low division
    let v1 = ProfileVariables {
        survival_ticks: 900,
        divisions_count: 2,
    };
    // p2 has lower survival but high division
    let v2 = ProfileVariables {
        survival_ticks: 600,
        divisions_count: 5,
    };

    let finding = evaluate_balance("dormancy-oriented", "opportunistic-growth", &cond, &v1, &v2);
    assert_eq!(finding.result, BalanceOutcome::TradeoffObserved);
    assert!(finding.equal_requirements);
}
