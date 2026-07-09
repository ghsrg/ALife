use alife::observer::config::{
    load_behavior_profile_classifier, load_cell_role_classifier, load_classification_registry,
    load_organism_archetype_classifier,
};

#[test]
fn test_load_all_observer_configs() {
    let reg = load_classification_registry("config/observer/classification-registry.toml").unwrap();
    assert_eq!(reg.registry.id, "observer-classification-registry");
    assert!(reg.dimensions.contains_key("cell-functional-role"));

    let role_cfg =
        load_cell_role_classifier("config/observer/cell-functional-role-classifier.toml").unwrap();
    assert_eq!(
        role_cfg
            .rules
            .get("boundary-supporting")
            .unwrap()
            .min_fraction,
        0.20
    );

    let behavior_cfg =
        load_behavior_profile_classifier("config/observer/behavior-profile-classifier.toml")
            .unwrap();
    let dormancy_profile = behavior_cfg.profiles.get("dormancy-oriented").unwrap();
    assert_eq!(dormancy_profile.clauses[0].feature, "dormant_fraction");
    assert_eq!(dormancy_profile.clauses[0].operator, ">=");
    assert_eq!(dormancy_profile.clauses[0].value, 0.80);

    let archetype_cfg =
        load_organism_archetype_classifier("config/observer/organism-archetype-classifier.toml")
            .unwrap();
    assert!(archetype_cfg.archetypes.contains_key("transient-cluster"));
}
