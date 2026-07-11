use std::path::Path;
use std::process::Command;

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
}
