use serde::Serialize;
use std::collections::HashMap;

use crate::core::snapshot::CommittedSnapshot;
use crate::core::summary::MetricsSummary;
use crate::observer::balance::{BalanceFinding, BalanceOutcome};
use crate::observer::classifiers::ClassificationResult;
use crate::observer::contract::{coverage_status_specs, warning_code_specs};
use crate::observer::payloads::{
    BalanceFindingProjectionPayload, ClassificationEvidencePayload,
    ClassificationProjectionPayload, CoverageMechanismPayload, CoverageProjectionPayload,
    FieldSummaryPayload, ObserverProjectionPayloadError, ProjectionSourceMetricRef,
    ResourceAmountPayload, ResourceLayerCellPayload, ResourceLayerSummaryPayload,
    VisualCellPayload, VisualWorldProjection, WarningProjectionPayload,
};
use crate::observer::projection_envelope::ProjectionCompleteness;

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

pub fn build_visual_world_projection(snapshot: &CommittedSnapshot) -> VisualWorldProjection {
    let completeness = ProjectionCompleteness::bounded(
        "CommittedSnapshot exposes exact current resource grid cells and bounded per-cell material/resource summaries for Control Center visualization.",
    );

    let cells = snapshot
        .cells
        .iter()
        .map(|cell| VisualCellPayload {
            id: cell.id.raw(),
            x: cell.position.x(),
            y: cell.position.y(),
            radius: cell.radius.raw(),
            energy: cell.energy.raw(),
            energy_capacity: cell.energy_capacity.raw(),
            lifecycle_state: cell.lifecycle_state,
            materials: cell
                .materials
                .iter()
                .enumerate()
                .map(
                    |(index, amount)| crate::observer::payloads::MaterialAmountPayload {
                        material_type_id: index as u32,
                        amount: *amount,
                    },
                )
                .collect(),
            internal_resources: cell
                .internal_resources
                .iter()
                .enumerate()
                .map(|(index, amount)| ResourceAmountPayload {
                    resource_type_id: index as u32,
                    amount: amount.raw(),
                })
                .collect(),
            local_external_resources: cell
                .local_external_resources
                .iter()
                .enumerate()
                .map(|(index, amount)| ResourceAmountPayload {
                    resource_type_id: index as u32,
                    amount: amount.raw(),
                })
                .collect(),
        })
        .collect();

    let resource_layers = snapshot
        .resource_layers
        .iter()
        .map(|layer| ResourceLayerSummaryPayload {
            layer_index: layer.layer_index,
            width: layer.width,
            height: layer.height,
            total_amount: layer.total_amount.raw(),
            cells: layer
                .cells
                .iter()
                .map(|cell| ResourceLayerCellPayload {
                    x: cell.x,
                    y: cell.y,
                    amount: cell.amount.raw(),
                })
                .collect(),
            completeness: ProjectionCompleteness::bounded(
                "CommittedSnapshot exposes exact current resource grid cells for this bounded world.",
            ),
        })
        .collect();

    let fields = vec![
        FieldSummaryPayload {
            field_id: "heat".to_string(),
            value: snapshot.heat,
            source_metric: source_metric("heat", "CommittedSnapshot.heat"),
        },
        FieldSummaryPayload {
            field_id: "waste".to_string(),
            value: snapshot.waste,
            source_metric: source_metric("waste", "CommittedSnapshot.waste"),
        },
    ];

    let source_metrics = vec![
        source_metric("tick", "CommittedSnapshot.tick"),
        source_metric("cells.id", "CommittedSnapshot.cells[].id"),
        source_metric("cells.position", "CommittedSnapshot.cells[].position"),
        source_metric("cells.radius", "CommittedSnapshot.cells[].radius"),
        source_metric("cells.energy", "CommittedSnapshot.cells[].energy"),
        source_metric(
            "cells.energy_capacity",
            "CommittedSnapshot.cells[].energy_capacity",
        ),
        source_metric("cells.materials", "CommittedSnapshot.cells[].materials"),
        source_metric(
            "cells.internal_resources",
            "CommittedSnapshot.cells[].internal_resources",
        ),
        source_metric(
            "cells.local_external_resources",
            "CommittedSnapshot.cells[].local_external_resources",
        ),
        source_metric(
            "cells.lifecycle",
            "CommittedSnapshot.cells[].lifecycle_state",
        ),
        source_metric(
            "resource_layer_totals",
            "CommittedSnapshot.resource_layer_totals",
        ),
        source_metric("heat", "CommittedSnapshot.heat"),
        source_metric("waste", "CommittedSnapshot.waste"),
    ];

    VisualWorldProjection {
        tick: snapshot.tick.raw(),
        cells,
        resource_layers,
        fields,
        completeness,
        source_metrics,
    }
}

fn source_metric(field_id: &str, source_path: &str) -> ProjectionSourceMetricRef {
    ProjectionSourceMetricRef::new(field_id, "CoreCommittedSnapshot", source_path)
}

pub fn build_classification_projection(
    result: &ClassificationResult,
    entity_type: EntityType,
    run_id: &str,
    registry_version: &str,
    source_projection: &str,
    limitations: Vec<String>,
) -> ClassificationProjectionPayload {
    let classification_id = format!(
        "{}:{:?}:{}:{}:{}-{}:{:?}:{}:{}",
        run_id,
        entity_type,
        result.entity_id,
        result.dimension_id,
        result.tick_start,
        result.tick_end,
        result.mode,
        result.classifier_version,
        registry_version
    );

    let completeness = if result.data_completeness >= 1.0 {
        ProjectionCompleteness::full()
    } else {
        ProjectionCompleteness::partial(
            vec!["classification.source_window"],
            "Classification is based on a bounded derived feature window.",
        )
    };

    let evidence = result
        .evidence
        .iter()
        .map(|record| ClassificationEvidencePayload {
            feature: record.feature.clone(),
            expected: record.expected.clone(),
            actual: record.actual,
            matched: record.matched,
            source_metric: ProjectionSourceMetricRef::new(
                record.feature.clone(),
                "ObserverDerivedFeature",
                format!("{}::{}", source_projection, record.feature),
            ),
        })
        .collect();

    ClassificationProjectionPayload {
        classification_id,
        dimension_id: result.dimension_id.clone(),
        entity_type,
        entity_id: result.entity_id.clone(),
        mode: result.mode.clone(),
        primary_label: result.primary_label.clone(),
        secondary_labels: result.secondary_labels.clone(),
        status: result.status.clone(),
        confidence: result.confidence,
        tick_start: result.tick_start,
        tick_end: result.tick_end,
        classifier_version: result.classifier_version.clone(),
        registry_version: registry_version.to_string(),
        source_projection: source_projection.to_string(),
        completeness,
        evidence,
        limitations,
    }
}

pub fn build_coverage_projection(
    mechanisms: Vec<(String, String, String)>,
) -> Result<CoverageProjectionPayload, ObserverProjectionPayloadError> {
    let mut payload_mechanisms = Vec::with_capacity(mechanisms.len());

    for (mechanism_id, status_id, source_report) in mechanisms {
        if !coverage_status_specs()
            .iter()
            .any(|spec| spec.status_id == status_id)
        {
            return Err(ObserverProjectionPayloadError::UnknownCoverageStatus(
                status_id,
            ));
        }

        payload_mechanisms.push(CoverageMechanismPayload {
            mechanism_id,
            status_id,
            source_report,
        });
    }

    Ok(CoverageProjectionPayload {
        mechanisms: payload_mechanisms,
    })
}

pub fn build_warning_projection(
    code: &str,
    affected_scope: &str,
    source_report: &str,
    recommended_reruns: Vec<String>,
) -> Result<WarningProjectionPayload, ObserverProjectionPayloadError> {
    let spec = warning_code_specs()
        .iter()
        .find(|spec| spec.code == code)
        .ok_or_else(|| ObserverProjectionPayloadError::UnknownWarningCode(code.to_string()))?;

    Ok(WarningProjectionPayload {
        code: code.to_string(),
        disposition: spec.disposition,
        affected_scope: affected_scope.to_string(),
        source_report: source_report.to_string(),
        recommended_reruns,
    })
}

pub fn project_balance_finding(
    finding: &BalanceFinding,
    source_report: &str,
) -> BalanceFindingProjectionPayload {
    let (claimed_result, limitations) = if finding.equal_requirements {
        (finding.result, Vec::new())
    } else {
        (
            BalanceOutcome::Inconclusive,
            vec![
                "Balance claim suppressed because compared profiles do not have equal requirements."
                    .to_string(),
            ],
        )
    };

    BalanceFindingProjectionPayload {
        finding_id: finding.finding_id.clone(),
        compared_profiles: finding.compared_profiles.clone(),
        equal_requirements: finding.equal_requirements,
        reported_result: finding.result,
        claimed_result,
        evidence_metrics: finding.evidence_metrics.clone(),
        dominance_rate: finding.dominance_rate,
        affected_scenarios: finding.affected_scenarios.clone(),
        suspected_cause: finding.suspected_cause.clone(),
        recommendation: finding.recommendation.clone(),
        recommended_reruns: finding.recommended_reruns.clone(),
        confidence: finding.confidence,
        source_report: source_report.to_string(),
        limitations,
    }
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
