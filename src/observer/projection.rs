use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum EntityType {
    Cell,
    Organism,
    Population,
}

#[derive(Debug, Serialize, Clone)]
pub struct ObservationWindow {
    pub run_id: String,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub tick_start: u64,
    pub tick_end: u64,
    pub features: HashMap<String, f32>,
    pub data_completeness: f32,
    pub projection_version: String,
}

pub fn extract_features(
    run_id: &str,
    entity_type: EntityType,
    entity_id: &str,
    tick_start: u64,
    tick_end: u64,
    raw_data: &HashMap<String, f32>,
) -> ObservationWindow {
    let mut features = HashMap::new();

    // Extract dormant fraction
    if let (Some(&d), Some(&t)) = (
        raw_data.get("dormant_ticks"),
        raw_data.get("ticks_executed"),
    ) {
        features.insert(
            "dormant_fraction".to_string(),
            if t > 0.0 { d / t } else { 0.0 },
        );
    }

    // Extract material fractions
    let total_mat = raw_data.get("total_materials").copied().unwrap_or(0.0);
    if total_mat > 0.0 {
        for mat_name in &[
            "boundary_material",
            "transport_material",
            "metabolic_material",
            "storage_material",
            "synthesis_material",
            "structural_material",
            "repair_material",
            "contractile_material",
            "sensory_material",
        ] {
            if let Some(&val) = raw_data.get(*mat_name) {
                features.insert(format!("{}_fraction", mat_name), val / total_mat);
            }
        }
    }

    // Copy other features directly
    for (k, &v) in raw_data {
        if !features.contains_key(k) {
            features.insert(k.clone(), v);
        }
    }

    ObservationWindow {
        run_id: run_id.to_string(),
        entity_type,
        entity_id: entity_id.to_string(),
        tick_start,
        tick_end,
        features,
        data_completeness: 1.0,
        projection_version: "1.0.0".to_string(),
    }
}
