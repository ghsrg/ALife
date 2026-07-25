use crate::storage::analytics_export::AnalyticsDataset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RunCalibrationComparison {
    pub baseline_run_id: String,
    pub baseline_scenario_id: String,
    pub candidate_run_id: String,
    pub candidate_scenario_id: String,
    pub pop_growth_delta: f64,
    pub mass_balance_drift_delta: f64,
    pub stability_classification: String,
    pub rerun_recommendation: String,
    pub provenance_links: Vec<String>,
}

pub fn compare_run_datasets(
    baseline: &AnalyticsDataset,
    candidate: &AnalyticsDataset,
) -> RunCalibrationComparison {
    let baseline_pop_end = baseline
        .population
        .last()
        .map(|r| r.alive_cells as f64)
        .unwrap_or(0.0);
    let candidate_pop_end = candidate
        .population
        .last()
        .map(|r| r.alive_cells as f64)
        .unwrap_or(0.0);

    let pop_growth_delta = candidate_pop_end - baseline_pop_end;

    let baseline_drift = baseline
        .balance
        .iter()
        .map(|r| r.unaccounted_difference.abs())
        .fold(0.0, f64::max);
    let candidate_drift = candidate
        .balance
        .iter()
        .map(|r| r.unaccounted_difference.abs())
        .fold(0.0, f64::max);

    let mass_balance_drift_delta = candidate_drift - baseline_drift;

    let stability_classification = if candidate_pop_end == 0.0 && baseline_pop_end > 0.0 {
        "collapse".to_string()
    } else if pop_growth_delta > 0.0 && mass_balance_drift_delta <= 0.001 {
        "improved_stability".to_string()
    } else if mass_balance_drift_delta > 0.1 || pop_growth_delta < 0.0 {
        "increased_fragility".to_string()
    } else {
        "equivalent_stability".to_string()
    };

    let rerun_recommendation = match stability_classification.as_str() {
        "improved_stability" => {
            "Candidate parameters show improved population growth and mass conservation. Recommended for baseline update."
                .to_string()
        }
        "collapse" | "increased_fragility" => {
            "Candidate run exhibited fragility or collapse. Recommend rerunning with conservative parameter boundaries."
                .to_string()
        }
        _ => "Candidate stability is equivalent to baseline. No configuration mutation required."
            .to_string(),
    };

    let provenance_links = vec![
        format!("baseline://{}", baseline.manifest.run_id),
        format!("candidate://{}", candidate.manifest.run_id),
    ];

    RunCalibrationComparison {
        baseline_run_id: baseline.manifest.run_id.clone(),
        baseline_scenario_id: baseline.manifest.scenario_id.clone(),
        candidate_run_id: candidate.manifest.run_id.clone(),
        candidate_scenario_id: candidate.manifest.scenario_id.clone(),
        pop_growth_delta,
        mass_balance_drift_delta,
        stability_classification,
        rerun_recommendation,
        provenance_links,
    }
}

pub fn generate_calibration_report(comparison: &RunCalibrationComparison) -> String {
    format!(
        "# ALife Evolution Calibration & Run Comparison Report\n\n\
         ## Metadata & Provenance\n\
         - **Baseline Run:** {} (Scenario: {})\n\
         - **Candidate Run:** {} (Scenario: {})\n\
         - **Provenance Links:** {}\n\n\
         ## Calibration Metrics\n\
         - **Population Growth Delta:** {:.2}\n\
         - **Mass Balance Drift Delta:** {:.4}\n\
         - **Stability Classification:** {}\n\n\
         ## Recommendation\n\
         {}\n",
        comparison.baseline_run_id,
        comparison.baseline_scenario_id,
        comparison.candidate_run_id,
        comparison.candidate_scenario_id,
        comparison.provenance_links.join(", "),
        comparison.pop_growth_delta,
        comparison.mass_balance_drift_delta,
        comparison.stability_classification,
        comparison.rerun_recommendation
    )
}
