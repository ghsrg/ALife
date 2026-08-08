use alife::core::snapshot::{CommittedSnapshot, ResourceLayerCellSnapshot, ResourceLayerSnapshot};
use alife::core::units::{ResourceAmount, Tick};
use alife::observer::monitor_payloads::build_monitor_data_panel_projection;
use serde_json::json;

fn resource_snapshot(total: f32) -> CommittedSnapshot {
    CommittedSnapshot {
        tick: Tick::from_raw(7),
        cells: Vec::new(),
        joints: Vec::new(),
        organisms: Vec::new(),
        heat: 0.0,
        waste: 0.0,
        resource_layer_totals: vec![ResourceAmount::new(total).unwrap()],
        resource_layers: vec![ResourceLayerSnapshot {
            layer_index: 0,
            resource_type_id: 0,
            resource_id: "amino_acid".to_string(),
            width: 1,
            height: 1,
            total_amount: ResourceAmount::new(total).unwrap(),
            cells: vec![ResourceLayerCellSnapshot {
                x: 0,
                y: 0,
                amount: ResourceAmount::new(total).unwrap(),
            }],
        }],
        scalar_field_layers: vec![],
    }
}

#[test]
fn world_resource_cycle_exposes_conservation_accounting_buckets() {
    let monitor = build_monitor_data_panel_projection(&resource_snapshot(90.0), "run-accounting");
    let json = json!(monitor);
    let resource_cycle = &json["payload"]["world"]["resource_cycle"];

    assert_eq!(resource_cycle["state"], "available");
    assert_eq!(
        resource_cycle["source"],
        "MonitorAccountingProjection.resource"
    );
    assert_eq!(resource_cycle["total_amount"], 90.0);
    assert!(resource_cycle["locations"]["environment"].as_f64().unwrap() >= 0.0);
    assert!(
        resource_cycle["accounting"]["explicit_decay_or_sink"]
            .as_f64()
            .unwrap()
            >= 0.0
    );
    assert!(
        resource_cycle["accounting"]["metabolism_or_cell_uptake"]
            .as_f64()
            .unwrap()
            >= 0.0
    );
    assert!(
        resource_cycle["accounting"]["material_conversion"]
            .as_f64()
            .unwrap()
            >= 0.0
    );
    assert!(
        resource_cycle["accounting"]["unclassified_loss"]
            .as_f64()
            .unwrap()
            >= 0.0
    );
}

#[test]
fn monitor_completeness_does_not_mark_available_resource_cycle_missing() {
    let monitor = build_monitor_data_panel_projection(&resource_snapshot(90.0), "run-accounting");

    assert!(
        !monitor
            .completeness
            .missing_fields
            .contains(&"world.resource_cycle"),
        "world.resource_cycle is available and must not be reported as missing"
    );
}
