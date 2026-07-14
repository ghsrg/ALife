use alife::bootstrap::viability::{ViabilityStatus, validate_prepared_config};
use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};

const SCENARIO: &str = include_str!("../config/scenarios/genome/phase3a_genome_bootstrap.toml");

#[test]
fn viable_minimal_scenario_passes() {
    let document = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "viable".to_string(),
        content: SCENARIO.to_string(),
    })
    .unwrap();

    let report = validate_prepared_config(&document.runtime_config).unwrap();
    assert_eq!(report.status, ViabilityStatus::Pass);
    assert!(report.warnings.is_empty());
}

#[test]
fn out_of_bounds_cell_fails_with_stable_code() {
    let document = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "bad".to_string(),
        content: SCENARIO.replace(
            "initial_position = [16.0, 16.0]",
            "initial_position = [99.0, 99.0]",
        ),
    })
    .unwrap();

    let err = validate_prepared_config(&document.runtime_config).unwrap_err();
    assert_eq!(err.code, "CELLS_WITHIN_WORLD_BOUNDS");
}

#[test]
fn low_starter_energy_is_warning_not_error() {
    let document = ScenarioDocument::resolve(ScenarioSource::Inline {
        id: "low_energy".to_string(),
        content: SCENARIO.replace("initial_energy = 8.0", "initial_energy = 0.1"),
    })
    .unwrap();

    let report = validate_prepared_config(&document.runtime_config).unwrap();
    assert_eq!(report.status, ViabilityStatus::Warn);
    assert_eq!(report.warnings, ["BOOTSTRAP_LOW_START_ENERGY"]);
}
