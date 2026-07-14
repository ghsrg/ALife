use alife::bootstrap::field_layers::constant_field_layer;
use alife::bootstrap::resource_layers::{patches_resource_layer, uniform_resource_layer};
use alife::bootstrap::seed_domains::SplitMix64;

#[test]
fn uniform_resource_layer_reports_exact_total_and_bounds() {
    let summary = uniform_resource_layer(0, 4, 2.5).unwrap();

    assert_eq!(summary.layer_index, 0);
    assert_eq!(summary.total, 10.0);
    assert_eq!(summary.min, 2.5);
    assert_eq!(summary.max, 2.5);
}

#[test]
fn patches_resource_layer_respects_amount_bounds() {
    let mut rng = SplitMix64::new(123);
    let summary = patches_resource_layer(1, 8, 1.0, 3.0, &mut rng).unwrap();

    assert_eq!(summary.layer_index, 1);
    assert!(summary.total >= 8.0);
    assert!(summary.total <= 24.0);
    assert!(summary.min >= 1.0);
    assert!(summary.max <= 3.0);
}

#[test]
fn invalid_resource_layer_parameters_return_stable_error_code() {
    let err = uniform_resource_layer(0, 4, -1.0).unwrap_err();

    assert_eq!(err.code(), "BOOTSTRAP_INVALID_RESOURCE_LAYER");
}

#[test]
fn constant_field_layer_reports_min_equal_max() {
    let summary = constant_field_layer("temperature", 25.0).unwrap();

    assert_eq!(summary.field_id, "temperature");
    assert_eq!(summary.min, 25.0);
    assert_eq!(summary.max, 25.0);
}
