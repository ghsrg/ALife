use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceOutcome {
    TradeoffObserved,
    Balanced,
    NotBalanced,
    Inconclusive,
    InsufficientCoverage,
    DominanceObserved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlledConditions {
    pub scenario_id: String,
    pub scenario_version: String,
    pub ticks_requested: u64,
    pub seed: u64,
    pub world_size: [f64; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileVariables {
    pub survival_ticks: u64,
    pub divisions_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceFinding {
    pub finding_id: String,
    pub compared_profiles: (String, String),
    pub equal_requirements: bool,
    pub result: BalanceOutcome,
    pub evidence_metrics: Vec<String>,
    pub dominance_rate: f32,
    pub affected_scenarios: Vec<String>,
    pub suspected_cause: Option<String>,
    pub recommendation: Option<String>,
    pub recommended_reruns: Vec<String>,
    pub confidence: f32,
}

pub fn evaluate_balance(
    profile_a: &str,
    profile_b: &str,
    cond: &ControlledConditions,
    v1: &ProfileVariables,
    v2: &ProfileVariables,
) -> BalanceFinding {
    // Determine the outcome.
    // If one profile has higher survival but lower divisions, or vice versa, it's a tradeoff.
    // If they are identical, it's balanced.
    // If one dominates (higher or equal on both, with at least one strict inequality), it's not balanced.
    let (outcome, dominance_rate) = if (v1.survival_ticks > v2.survival_ticks
        && v1.divisions_count < v2.divisions_count)
        || (v1.survival_ticks < v2.survival_ticks && v1.divisions_count > v2.divisions_count)
    {
        (BalanceOutcome::TradeoffObserved, 0.0)
    } else if v1.survival_ticks == v2.survival_ticks && v1.divisions_count == v2.divisions_count {
        (BalanceOutcome::Balanced, 0.0)
    } else {
        // Dominance calculation: compute relative difference
        let s_diff = (v1.survival_ticks as f64 - v2.survival_ticks as f64).abs();
        let s_max = v1.survival_ticks.max(v2.survival_ticks) as f64;
        let s_rel = if s_max > 0.0 { s_diff / s_max } else { 0.0 };

        let d_diff = (v1.divisions_count as f64 - v2.divisions_count as f64).abs();
        let d_max = v1.divisions_count.max(v2.divisions_count) as f64;
        let d_rel = if d_max > 0.0 { d_diff / d_max } else { 0.0 };

        let rate = (s_rel + d_rel) / 2.0;
        (BalanceOutcome::NotBalanced, rate as f32)
    };

    let finding_id = format!("{}-{}-{}", cond.scenario_id, profile_a, profile_b);

    let evidence_metrics = vec![
        format!(
            "{}: survival_ticks={}, divisions_count={}",
            profile_a, v1.survival_ticks, v1.divisions_count
        ),
        format!(
            "{}: survival_ticks={}, divisions_count={}",
            profile_b, v2.survival_ticks, v2.divisions_count
        ),
    ];

    let suspected_cause = if outcome == BalanceOutcome::NotBalanced {
        Some("One profile dominates the other in both survival and reproduction, indicating a lack of trade-off.".to_string())
    } else {
        None
    };

    let recommendation = if outcome == BalanceOutcome::NotBalanced {
        Some(
            "Adjust parameters to introduce higher upkeep cost or division requirements."
                .to_string(),
        )
    } else {
        None
    };

    let recommended_reruns = if outcome == BalanceOutcome::NotBalanced {
        vec!["sensitivity_sweep".to_string()]
    } else {
        vec![]
    };

    BalanceFinding {
        finding_id,
        compared_profiles: (profile_a.to_string(), profile_b.to_string()),
        equal_requirements: true,
        result: outcome,
        evidence_metrics,
        dominance_rate,
        affected_scenarios: vec![cond.scenario_id.clone()],
        suspected_cause,
        recommendation,
        recommended_reruns,
        confidence: 1.0,
    }
}
