use std::process::Command;

fn csv_rows(path: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let csv = std::fs::read_to_string(path).unwrap_or_else(|err| panic!("{path}: {err}"));
    let mut lines = csv.lines();
    let header = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::to_string)
        .collect::<Vec<_>>();
    let rows = lines
        .map(|line| line.split(',').map(str::to_string).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (header, rows)
}

fn any_positive(path: &str, column: &str) -> bool {
    let (header, rows) = csv_rows(path);
    let index = header
        .iter()
        .position(|field| field == column)
        .unwrap_or_else(|| panic!("{path} missing {column}"));
    rows.iter()
        .any(|row| row[index].parse::<f32>().unwrap() > 0.0)
}

#[test]
fn phase2g_smoke_keeps_core_matter_dynamics_reachable() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--bin",
            "sweep_analyzer",
            "--",
            "config/analyzer/sweep_analyzer_smoke.toml",
        ])
        .output()
        .expect("sweep analyzer should run");

    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(any_positive(
        "outputs/raw_data/smoke/resource_type_decay_diffusion.csv",
        "resource_diffused_amount"
    ));
    assert!(any_positive(
        "outputs/raw_data/smoke/fragment_decomposition_conversion.csv",
        "fragment_converted_amount"
    ));
    assert!(any_positive(
        "outputs/raw_data/smoke/local_heat_degradation.csv",
        "heat_peak_temperature"
    ));
    assert!(any_positive(
        "outputs/raw_data/smoke/repair_viability.csv",
        "repair_success_count"
    ));
}
