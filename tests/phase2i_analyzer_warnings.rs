#[test]
fn flat_sweeps_have_non_degenerate_calibration_ranges() {
    let full = std::fs::read_to_string("config/analyzer/sweep_analyzer.toml").unwrap();

    assert!(full.contains("name = \"material_type_degradation\""));
    assert!(full.contains("param = \"material_decay_rate\""));
    assert!(full.contains("from = 0.0"));
    assert!(full.contains("to = 0.5"));

    assert!(full.contains("name = \"joint_creation_viability\""));
    assert!(full.contains("param = \"joint_mechanical_strength\""));
    assert!(full.contains("from = 0.05"));
    assert!(full.contains("to = 1.25"));

    assert!(full.contains("name = \"boundary_retention_leakage\""));
    assert!(full.contains("param = \"contact_exchange_rate\""));
    assert!(full.contains("to = 1.5"));
}
