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
    features.insert("joint_count".to_string(), metrics.joint_count as f32);
    features.insert(
        "joint_created_count".to_string(),
        metrics.joint_created_count as f32,
    );
    features.insert(
        "joint_creation_rejected_count".to_string(),
        metrics.joint_creation_rejected_count as f32,
    );
    features.insert(
        "joint_broken_count".to_string(),
        metrics.joint_broken_count as f32,
    );
    features.insert(
        "joint_resource_transfer_amount".to_string(),
        metrics.joint_resource_transfer_amount,
    );
    features.insert(
        "joint_resource_transfer_gross_amount".to_string(),
        metrics.joint_resource_transfer_gross_amount,
    );
    features.insert(
        "joint_resource_transfer_net_amount".to_string(),
        metrics.joint_resource_transfer_net_amount,
    );
    features.insert(
        "joint_resource_source_final_amount".to_string(),
        metrics.joint_resource_source_final_amount,
    );
    features.insert(
        "joint_resource_target_final_amount".to_string(),
        metrics.joint_resource_target_final_amount,
    );
    features.insert(
        "joint_resource_backflow_amount".to_string(),
        metrics.joint_resource_backflow_amount,
    );
    features.insert(
        "joint_signal_generated_total".to_string(),
        metrics.joint_signal_generated_total,
    );
    features.insert(
        "joint_signal_readable_total".to_string(),
        metrics.joint_signal_readable_total,
    );
    features.insert(
        "joint_heat_transfer_amount".to_string(),
        metrics.joint_heat_transfer_amount,
    );
    features.insert(
        "joint_degradation_amount".to_string(),
        metrics.joint_degradation_amount,
    );
    features.insert(
        "joint_mechanical_correction_amount".to_string(),
        metrics.joint_mechanical_correction_amount,
    );
    features
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrganismViewFeatures {
    pub component_count: u32,
    pub largest_component_size: u32,
    pub isolated_cell_count: u32,
}

pub fn organism_view_features(
    cell_count: usize,
    active_edges: &[(usize, usize)],
) -> OrganismViewFeatures {
    let mut parent: Vec<usize> = (0..cell_count).collect();

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    for &(a, b) in active_edges {
        if a >= cell_count || b >= cell_count || a == b {
            continue;
        }
        let root_a = find(&mut parent, a);
        let root_b = find(&mut parent, b);
        if root_a != root_b {
            parent[root_b] = root_a;
        }
    }

    let mut sizes = std::collections::BTreeMap::<usize, u32>::new();
    for cell in 0..cell_count {
        let root = find(&mut parent, cell);
        *sizes.entry(root).or_insert(0) += 1;
    }

    OrganismViewFeatures {
        component_count: sizes.len() as u32,
        largest_component_size: sizes.values().copied().max().unwrap_or(0),
        isolated_cell_count: sizes.values().filter(|&&size| size == 1).count() as u32,
    }
}
