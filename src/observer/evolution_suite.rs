use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum EvolutionRunOutcome {
    Collapse,
    Stable,
    Fragile,
    Invalid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LongRunScenarioConfig {
    pub scenario_id: String,
    pub seed_matrix: Vec<u64>,
    pub max_ticks: u64,
    pub min_population_threshold: usize,
    pub sampling_cadence: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LineageFrequencyWindow {
    pub tick_start: u64,
    pub tick_end: u64,
    pub lineage_counts: HashMap<String, usize>,
    pub observed_frequency_shift: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvolutionSuiteResult {
    pub scenario_id: String,
    pub seed: u64,
    pub final_tick: u64,
    pub final_population: usize,
    pub outcome: EvolutionRunOutcome,
    pub windows: Vec<LineageFrequencyWindow>,
}

pub fn evaluate_evolution_suite(
    scenario_id: &str,
    seed: u64,
    max_ticks: u64,
    min_population_threshold: usize,
    pop_history: &[(u64, usize)],
    windows: Vec<LineageFrequencyWindow>,
) -> EvolutionSuiteResult {
    if pop_history.is_empty() {
        return EvolutionSuiteResult {
            scenario_id: scenario_id.to_string(),
            seed,
            final_tick: 0,
            final_population: 0,
            outcome: EvolutionRunOutcome::Invalid,
            windows,
        };
    }

    let (final_tick, final_population) = *pop_history.last().unwrap();

    let outcome = if final_population == 0 {
        EvolutionRunOutcome::Collapse
    } else if final_population < min_population_threshold {
        EvolutionRunOutcome::Fragile
    } else if final_tick >= max_ticks {
        EvolutionRunOutcome::Stable
    } else {
        EvolutionRunOutcome::Fragile
    };

    EvolutionSuiteResult {
        scenario_id: scenario_id.to_string(),
        seed,
        final_tick,
        final_population,
        outcome,
        windows,
    }
}
