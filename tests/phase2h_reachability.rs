use std::process::Command;

#[test]
fn smoke_sweep_outputs_phase2h_joint_reachability_csvs() {
    let status = Command::new(env!("CARGO_BIN_EXE_sweep_analyzer"))
        .arg("config/analyzer/sweep_analyzer_smoke.toml")
        .status()
        .expect("sweep analyzer runs");
    assert!(status.success());

    for file in [
        "outputs/raw_data/smoke/joint_creation_viability.csv",
        "outputs/raw_data/smoke/joint_resource_channel.csv",
        "outputs/raw_data/smoke/joint_signal_delay.csv",
        "outputs/raw_data/smoke/joint_heat_channel.csv",
        "outputs/raw_data/smoke/joint_degradation_break.csv",
        "outputs/raw_data/smoke/joint_lifecycle_division.csv",
    ] {
        let csv = std::fs::read_to_string(file).unwrap_or_else(|err| panic!("{file}: {err}"));
        assert!(csv.contains("joint_count"), "{file} missing joint_count");
        assert!(
            csv.contains("joint_created_count"),
            "{file} missing joint_created_count"
        );
        assert!(
            csv.contains("joint_resource_transfer_amount"),
            "{file} missing resource metric"
        );
        assert!(
            csv.contains("joint_resource_transfer_gross_amount"),
            "{file} missing gross resource transfer audit metric"
        );
        assert!(
            csv.contains("joint_resource_transfer_net_amount"),
            "{file} missing net resource transfer audit metric"
        );
        assert!(
            csv.contains("joint_resource_source_final_amount"),
            "{file} missing source endpoint resource audit metric"
        );
        assert!(
            csv.contains("joint_resource_target_final_amount"),
            "{file} missing target endpoint resource audit metric"
        );
        assert!(
            csv.contains("joint_resource_backflow_amount"),
            "{file} missing backflow resource audit metric"
        );
        assert!(
            csv.contains("joint_signal_readable_total"),
            "{file} missing signal metric"
        );
        assert!(
            csv.contains("joint_heat_transfer_amount"),
            "{file} missing heat metric"
        );
        assert!(
            csv.contains("joint_degradation_amount"),
            "{file} missing degradation metric"
        );
    }
}
