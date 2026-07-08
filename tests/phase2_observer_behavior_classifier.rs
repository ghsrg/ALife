use alife::observer::{
    classifiers::{ClassificationStatus, classify_behavior_profiles},
    config::load_behavior_profile_classifier,
    projection::{EntityType, extract_features},
};
use std::collections::HashMap;

#[test]
fn test_classify_behavior_profiles() {
    let config =
        load_behavior_profile_classifier("docs/config/observer/behavior-profile-classifier.toml")
            .unwrap();
    let mut raw_data = HashMap::new();
    // Simulate dormancy-oriented behavior
    raw_data.insert("dormant_fraction".to_string(), 0.85);
    raw_data.insert("dormancy_entries".to_string(), 2.0);

    let window = extract_features("run-123", EntityType::Cell, "cell-0", 0, 100, &raw_data);

    let res = classify_behavior_profiles(&window, &config);
    assert_eq!(res.primary_label.unwrap(), "dormancy-oriented");
    assert_eq!(res.status, ClassificationStatus::Classified);
}
