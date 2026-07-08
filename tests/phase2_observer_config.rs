use alife::observer::config::{
    load_classification_registry, load_cell_role_classifier,
    load_behavior_profile_classifier, load_organism_archetype_classifier
};

#[test]
fn test_load_all_observer_configs() {
    let reg = load_classification_registry("docs/config/observer/classification-registry.toml").unwrap();
    assert_eq!(reg.registry.id, "observer-classification-registry");
    assert!(reg.dimensions.contains_key("cell-functional-role"));

    let role_cfg = load_cell_role_classifier("docs/config/observer/cell-functional-role-classifier.toml").unwrap();
    assert_eq!(role_cfg.rules.get("boundary-supporting-like").unwrap().min_fraction, 0.20);

    let behavior_cfg = load_behavior_profile_classifier("docs/config/observer/behavior-profile-classifier.toml").unwrap();
    let dormancy_profile = behavior_cfg.profiles.get("dormancy-oriented-like").unwrap();
    assert_eq!(dormancy_profile.clauses[0].feature, "dormant_fraction");
    assert_eq!(dormancy_profile.clauses[0].operator, ">=");
    assert_eq!(dormancy_profile.clauses[0].value, 0.80);
}
