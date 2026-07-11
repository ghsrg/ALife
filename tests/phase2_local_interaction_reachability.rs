use std::process::Command;

#[test]
fn phase2f_reachability_sweep_has_contact_exchange_and_no_low_information_warning() {
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

    let csv = std::fs::read_to_string("outputs/raw_data/smoke/local_interaction_viability.csv")
        .expect("local interaction raw csv should exist");

    let header: Vec<&str> = csv.lines().next().unwrap().split(',').collect();
    let idx = |name: &str| header.iter().position(|field| *field == name).unwrap();
    let rows: Vec<Vec<&str>> = csv
        .lines()
        .skip(1)
        .map(|line| line.split(',').collect())
        .collect();

    assert!(
        rows.iter()
            .any(|row| row[idx("contact_pairs_count")].parse::<u32>().unwrap() > 0)
    );
    assert!(
        rows.iter()
            .any(|row| row[idx("contact_exchange_amount")].parse::<f32>().unwrap() > 0.0)
    );
    assert!(rows.iter().any(|row| {
        row[idx("contact_stimulus_readable_total")]
            .parse::<f32>()
            .unwrap()
            > 0.0
    }));
    assert!(
        rows.iter()
            .all(|row| !row[idx("warning_codes")].contains("LOW_INFORMATION_SWEEP"))
    );
}
