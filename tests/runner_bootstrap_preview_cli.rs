use serde_json::Value;
use std::process::Command;

#[test]
fn runner_bootstrap_preview_prints_json_without_running_ticks() {
    let output = Command::new(env!("CARGO_BIN_EXE_runner"))
        .args(["--bootstrap-preview", "demo_world_resource"])
        .output()
        .expect("runner --bootstrap-preview should execute");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: Value =
        serde_json::from_slice(&output.stdout).expect("bootstrap preview stdout should be JSON");
    assert_eq!(json["scenario_id"], "demo_world_resource");
    assert_eq!(json["tick_executed"], false);
    assert_eq!(json["resource_layers"].as_array().unwrap().len(), 27);
    assert_eq!(json["field_layers"].as_array().unwrap().len(), 5);
    assert!(
        json["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|warning| { warning["code"] == "BOOTSTRAP_FIELD_LAYER_NOT_CORE_INTEGRATED" })
    );
}
