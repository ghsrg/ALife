use alife::observer::projection::{metrics_summary_features, organism_view_features};

#[test]
fn metrics_projection_exposes_joint_features_without_behavior_authority() {
    let metrics = alife::core::summary::MetricsSummary {
        joint_count: 2,
        joint_resource_transfer_amount: 1.5,
        joint_signal_readable_total: 0.25,
        joint_heat_transfer_amount: 0.75,
        ..Default::default()
    };

    let features = metrics_summary_features(&metrics);

    assert_eq!(features["joint_count"], 2.0);
    assert_eq!(features["joint_resource_transfer_amount"], 1.5);
    assert_eq!(features["joint_signal_readable_total"], 0.25);
    assert_eq!(features["joint_heat_transfer_amount"], 0.75);
}

#[test]
fn organism_view_is_connected_component_projection_only() {
    let components = organism_view_features(4, &[(0, 1), (1, 2)]);

    assert_eq!(components.component_count, 2);
    assert_eq!(components.largest_component_size, 3);
    assert_eq!(components.isolated_cell_count, 1);
}
