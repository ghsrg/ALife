use serde::Serialize;
use std::collections::HashMap;

use crate::core::summary::MetricsSummary;

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

pub fn metrics_summary_features(metrics: &MetricsSummary) -> HashMap<String, f32> {
    let mut features = HashMap::new();
    features.insert(
        "reaction_matched_count".to_string(),
        metrics.reaction_matched_count as f32,
    );
    features.insert(
        "reaction_executed_count".to_string(),
        metrics.reaction_executed_count as f32,
    );
    features.insert(
        "reaction_rejected_count".to_string(),
        metrics.reaction_rejected_count as f32,
    );
    features.insert(
        "reaction_input_amount".to_string(),
        metrics.reaction_input_amount,
    );
    features.insert(
        "reaction_output_amount".to_string(),
        metrics.reaction_output_amount,
    );
    features.insert(
        "reaction_heat_generated".to_string(),
        metrics.reaction_heat_generated,
    );
    features.insert(
        "reaction_energy_output".to_string(),
        metrics.reaction_energy_output,
    );
    features.insert(
        "reaction_accounting_error".to_string(),
        metrics.reaction_accounting_error,
    );
    features.insert(
        "resource_diffused_amount".to_string(),
        metrics.resource_diffused_amount,
    );
    features.insert(
        "resource_decay_amount".to_string(),
        metrics.resource_decay_amount,
    );
    features.insert(
        "fragment_created_amount".to_string(),
        metrics.fragment_created_amount,
    );
    features.insert(
        "fragment_converted_amount".to_string(),
        metrics.fragment_converted_amount,
    );
    features.insert("heat_peak_temperature".to_string(), metrics.heat);
    features.insert(
        "material_degradation_amount".to_string(),
        metrics.material_degradation_amount,
    );
    features.insert(
        "boundary_leakage_amount".to_string(),
        metrics.boundary_leakage_amount,
    );
    features.insert(
        "repair_success_count".to_string(),
        metrics.repair_success_count as f32,
    );
    features.insert(
        "repair_rejection_count".to_string(),
        metrics.repair_rejection_count as f32,
    );
    features
}
