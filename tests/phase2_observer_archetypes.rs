use alife::observer::classifiers::classify_organism_archetypes;
use alife::observer::config::load_organism_archetype_classifier;
use alife::observer::projection::{EntityType, extract_features};
use std::collections::HashMap;

#[test]
fn test_classify_organism_archetype_stable_colony() {
    let config = load_organism_archetype_classifier(
        "docs/config/observer/organism-archetype-classifier.toml",
    )
    .unwrap();
    let mut raw_data = HashMap::new();
    raw_data.insert("cell_count".to_string(), 4.0);
    raw_data.insert("joint_count".to_string(), 3.0);
    raw_data.insert("connectedness".to_string(), 1.0);
    raw_data.insert("lifetime_ticks".to_string(), 120.0);
    raw_data.insert("joint_persistence".to_string(), 0.85);

    let window = extract_features(
        "run-123",
        EntityType::Organism,
        "colony-0",
        0,
        100,
        &raw_data,
    );
    let result = classify_organism_archetypes(&window, &config);

    assert!(result.primary_label.is_some());
    assert_eq!(result.primary_label.unwrap(), "stable-colony");
}

#[test]
fn test_classify_organism_archetype_transient_cluster() {
    let config = load_organism_archetype_classifier(
        "docs/config/observer/organism-archetype-classifier.toml",
    )
    .unwrap();
    let mut raw_data = HashMap::new();
    raw_data.insert("cell_count".to_string(), 2.0);
    raw_data.insert("joint_count".to_string(), 1.0);
    raw_data.insert("connectedness".to_string(), 0.5);
    raw_data.insert("lifetime_ticks".to_string(), 30.0);
    raw_data.insert("joint_persistence".to_string(), 0.15);

    let window = extract_features(
        "run-123",
        EntityType::Organism,
        "colony-1",
        0,
        30,
        &raw_data,
    );
    let result = classify_organism_archetypes(&window, &config);

    assert!(result.primary_label.is_some());
    assert_eq!(result.primary_label.unwrap(), "transient-cluster");
}

#[test]
fn test_classify_organism_archetype_resource_sharing_colony() {
    let config = load_organism_archetype_classifier(
        "docs/config/observer/organism-archetype-classifier.toml",
    )
    .unwrap();
    let mut raw_data = HashMap::new();
    raw_data.insert("cell_count".to_string(), 3.0);
    raw_data.insert("joint_count".to_string(), 2.0);
    raw_data.insert("connectedness".to_string(), 1.0);
    raw_data.insert("lifetime_ticks".to_string(), 110.0);
    raw_data.insert("joint_persistence".to_string(), 0.80);
    raw_data.insert("requires_resource_transfer".to_string(), 1.0);

    let window = extract_features(
        "run-123",
        EntityType::Organism,
        "colony-2",
        0,
        100,
        &raw_data,
    );
    let result = classify_organism_archetypes(&window, &config);

    assert!(result.primary_label.is_some());
    assert_eq!(result.primary_label.unwrap(), "resource-sharing-colony");
}
