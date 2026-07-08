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
        load_cell_role_classifier("docs/config/observer/cell-functional-role-classifier.toml")
            .unwrap();
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
