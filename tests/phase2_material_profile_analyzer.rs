use std::path::Path;
use std::process::Command;

fn csv_rows(csv: &str) -> Vec<Vec<&str>> {
    csv.lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split(',').collect())
        .collect()
}

fn column_index(csv: &str, column: &str) -> usize {
    csv.lines()
        .next()
        .expect("csv should have a header")
        .split(',')
        .position(|name| name == column)
        .unwrap_or_else(|| panic!("csv should include column {column}"))
}

fn metric_for(csv: &str, scenario_id: &str, profile_id: &str, column: &str) -> f32 {
    let scenario_idx = column_index(csv, "scenario_id");
    let profile_idx = column_index(csv, "profile_id");
    let metric_idx = column_index(csv, column);
    csv_rows(csv)
        .into_iter()
        .find(|row| row[scenario_idx] == scenario_id && row[profile_idx] == profile_id)
        .unwrap_or_else(|| panic!("missing row {scenario_id}/{profile_id}"))[metric_idx]
        .parse::<f32>()
        .unwrap_or_else(|_| panic!("{column} should be numeric"))
}

#[test]
fn material_profile_files_exist_under_config_root() {
    assert!(Path::new("config/material_profiles/phase2_profiles.toml").exists());
    assert!(
        Path::new("config/scenarios/material_profiles/material_profile_baseline.toml").exists()
    );
    assert!(
        Path::new("config/scenarios/material_profiles/material_profile_negative_controls.toml")
            .exists()
    );
    assert!(Path::new("config/analyzer/material_profile_sweeps.toml").exists());
}

#[test]
fn sweep_analyzer_writes_material_profile_outputs() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--bin",
            "sweep_analyzer",
            "--",
            "config/analyzer/material_profile_sweeps.toml",
        ])
        .output()
        .expect("sweep analyzer should run");

    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(Path::new("outputs/raw_data/material_profile_summary.csv").exists());
    assert!(Path::new("outputs/raw_data/material_profile_coverage.csv").exists());

    let reports = std::fs::read_dir("outputs/reports").unwrap();
    assert!(reports.filter_map(Result::ok).any(|entry| {
        entry
            .file_name()
            .to_string_lossy()
            .starts_with("material-profile-coverage-")
    }));

    let summary = std::fs::read_to_string("outputs/raw_data/material_profile_summary.csv")
        .expect("summary csv should be readable");
    let coverage = std::fs::read_to_string("outputs/raw_data/material_profile_coverage.csv")
        .expect("coverage csv should be readable");

    for profile_id in [
        "transport_low",
        "no_transport",
        "metabolic_low",
        "no_metabolic",
        "storage_low",
        "no_storage",
        "structural_low",
        "no_structural",
        "contractile_low",
        "no_contractile",
        "sensory_low",
        "no_sensory",
    ] {
        assert!(
            summary.contains(&format!("material_profile_negative_controls,{profile_id},")),
            "missing negative-control row for {profile_id}"
        );
    }

    for profile_id in [
        "weak_cell",
        "balanced_baseline",
        "transport_rich",
        "metabolic_rich",
        "storage_rich",
        "structural_rich",
        "contractile_rich",
        "sensory_rich",
    ] {
        assert!(
            summary.contains(&format!(",{profile_id},")),
            "missing weak/balanced/specialized scale row for {profile_id}"
        );
    }

    for profile_id in ["metabolic_rich", "storage_rich", "structural_rich"] {
        assert!(
            summary.contains(&format!("material_profile_tradeoff_probe,{profile_id},")),
            "missing tradeoff probe row for {profile_id}"
        );
    }

    let baseline_energy = metric_for(
        &summary,
        "material_profile_baseline",
        "balanced_baseline",
        "energy_produced",
    );
    let tradeoff_metabolic_energy = metric_for(
        &summary,
        "material_profile_tradeoff_probe",
        "metabolic_rich",
        "energy_produced",
    );
    let tradeoff_metabolic_heat = metric_for(
        &summary,
        "material_profile_tradeoff_probe",
        "metabolic_rich",
        "heat_generated",
    );
    let baseline_heat = metric_for(
        &summary,
        "material_profile_baseline",
        "balanced_baseline",
        "heat_generated",
    );
    assert!(tradeoff_metabolic_energy > baseline_energy);
    assert!(tradeoff_metabolic_heat > baseline_heat);

    let tradeoff_storage_capacity = metric_for(
        &summary,
        "material_profile_tradeoff_probe",
        "storage_rich",
        "capacity_free",
    );
    let tradeoff_storage_energy = metric_for(
        &summary,
        "material_profile_tradeoff_probe",
        "storage_rich",
        "energy_produced",
    );
    assert!(
        tradeoff_storage_capacity
            > metric_for(
                &summary,
                "material_profile_baseline",
                "balanced_baseline",
                "capacity_free"
            )
    );
    assert!(tradeoff_storage_energy < baseline_energy);

    let tradeoff_structural_growth = metric_for(
        &summary,
        "material_profile_tradeoff_probe",
        "structural_rich",
        "growth_executed",
    );
    let tradeoff_structural_capacity_used = metric_for(
        &summary,
        "material_profile_tradeoff_probe",
        "structural_rich",
        "capacity_used",
    );
    assert!(
        tradeoff_structural_growth
            > metric_for(
                &summary,
                "material_profile_baseline",
                "balanced_baseline",
                "growth_executed"
            )
    );
    assert!(
        tradeoff_structural_capacity_used
            > metric_for(
                &summary,
                "material_profile_baseline",
                "balanced_baseline",
                "capacity_used"
            )
    );

    assert!(summary.contains("PROFILE_EFFECT_FLAT"));
    assert!(summary.contains("PROFILE_EFFECT_TOO_SMALL"));
    assert!(summary.contains("SCENARIO_TOO_EASY"));
    assert!(summary.contains("SCENARIO_TOO_HARD"));
    assert!(
        !summary.contains("PROFILE_DOMINATES_ALL_CONTEXTS"),
        "minimal tradeoff probe should prevent a fake dominant-profile pass"
    );

    for material_id in ["boundary", "repair"] {
        let line = coverage
            .lines()
            .find(|line| line.starts_with(&format!("{material_id},")))
            .unwrap_or_else(|| panic!("missing coverage row for {material_id}"));
        assert!(line.contains("covered_as_placeholder"));
        assert!(line.contains("tool_limited"));
        assert!(line.contains("not_full_mechanism"));
    }
}
