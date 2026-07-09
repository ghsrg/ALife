use alife::observer::{
    classifiers::{
        ClassificationStatus, classify_cell_roles_observed, classify_cell_roles_potential,
    },
    config::load_cell_role_classifier,
    projection::{EntityType, extract_features},
};
use std::collections::HashMap;

#[test]
fn test_classify_cell_roles_potential_and_observed() {
    let config =
        load_cell_role_classifier("config/observer/cell-functional-role-classifier.toml").unwrap();
    let mut raw_data = HashMap::new();
    raw_data.insert("boundary_material".to_string(), 30.0);
    raw_data.insert("total_materials".to_string(), 100.0);
    raw_data.insert("PassiveUptake_executed".to_string(), 0.0); // no action executed

    let window = extract_features("run-123", EntityType::Cell, "cell-0", 0, 100, &raw_data);

    let pot_res = classify_cell_roles_potential(&window, &config);
    assert_eq!(pot_res.primary_label.unwrap(), "boundary-supporting");
    assert_eq!(pot_res.status, ClassificationStatus::Classified);

    let obs_res = classify_cell_roles_observed(&window, &config);
    assert_eq!(obs_res.status, ClassificationStatus::Unknown);
    assert!(obs_res.primary_label.is_none());
}

#[test]
fn test_observed_role_execution_and_tie_breaking() {
    let config =
        load_cell_role_classifier("config/observer/cell-functional-role-classifier.toml").unwrap();

    // Test case 1: Single action executed
    let mut raw_data1 = HashMap::new();
    raw_data1.insert("PassiveUptake_executed".to_string(), 5.0);
    let window1 = extract_features("run-1", EntityType::Cell, "cell-0", 0, 100, &raw_data1);
    let obs_res1 = classify_cell_roles_observed(&window1, &config);
    assert_eq!(obs_res1.primary_label.unwrap(), "boundary-supporting");
    assert_eq!(obs_res1.status, ClassificationStatus::Classified);

    // Test case 2: Tie-breaking where boundary-supporting (PassiveUptake) vs transport-like (ActiveUptake) both executed with same count (5.0).
    // "boundary-supporting" is alphabetically before "transport-like".
    let mut raw_data2 = HashMap::new();
    raw_data2.insert("PassiveUptake_executed".to_string(), 5.0);
    raw_data2.insert("ActiveUptake_executed".to_string(), 5.0);
    let window2 = extract_features("run-2", EntityType::Cell, "cell-0", 0, 100, &raw_data2);
    let obs_res2 = classify_cell_roles_observed(&window2, &config);
    assert_eq!(obs_res2.primary_label.unwrap(), "boundary-supporting");

    // Test case 3: transport-like (ActiveUptake) executed with higher count (10.0) vs boundary-supporting (PassiveUptake) (5.0)
    let mut raw_data3 = HashMap::new();
    raw_data3.insert("PassiveUptake_executed".to_string(), 5.0);
    raw_data3.insert("ActiveUptake_executed".to_string(), 10.0);
    let window3 = extract_features("run-3", EntityType::Cell, "cell-0", 0, 100, &raw_data3);
    let obs_res3 = classify_cell_roles_observed(&window3, &config);
    assert_eq!(obs_res3.primary_label.unwrap(), "transport-like");
}
