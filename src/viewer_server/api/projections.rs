use crate::core::snapshot::CommittedSnapshot;
use crate::observer::monitor_payloads::build_monitor_data_panel_projection;
use crate::observer::projection::{
    build_coverage_projection, build_visual_world_projection_sampled, build_warning_projection,
};
use crate::observer::projection_envelope::{ProjectionCompleteness, ProjectionCompletenessState};
use crate::viewer_server::state::AppState;
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde_json::{Value, json};

use axum::extract::Query;

#[derive(serde::Deserialize, Debug)]
pub struct ProjectionParams {
    pub stride: Option<usize>,
}

fn completeness_state_label(state: ProjectionCompletenessState) -> &'static str {
    match state {
        ProjectionCompletenessState::Full => "full",
        ProjectionCompletenessState::Bounded => "bounded",
        ProjectionCompletenessState::Sampled => "sampled",
        ProjectionCompletenessState::Partial => "partial",
        ProjectionCompletenessState::DebugSelected => "debug_selected",
        ProjectionCompletenessState::Stale => "stale",
        ProjectionCompletenessState::Unavailable => "unavailable",
    }
}

fn completeness_json(completeness: &ProjectionCompleteness) -> Value {
    json!({
        "state": completeness_state_label(completeness.state()),
        "missing_fields": completeness.missing_fields(),
        "reason": completeness.reason(),
    })
}

fn visual_world_json(
    state: &crate::viewer_server::state::SharedState,
    snapshot: &CommittedSnapshot,
    stride: usize,
) -> Option<Value> {
    let projection = build_visual_world_projection_sampled(&snapshot, stride);

    Some(json!({
        "schema_version": "VisualWorldProjection/v1",
        "projection_kind": "VisualWorldProjection",
        "run_id": state.run_id,
        "tick": projection.tick,
        "source": "live",
        "completeness": completeness_json(&projection.completeness),
        "payload": {
            "cells": projection.cells.iter().map(|cell| json!({
                "id": cell.id,
                "x": cell.x,
                "y": cell.y,
                "radius": cell.radius,
                "energy": cell.energy,
                "energy_capacity": cell.energy_capacity,
                "lifecycle_state": format!("{:?}", cell.lifecycle_state),
                "materials": cell.materials.iter().map(|material| json!({
                    "material_type_id": material.material_type_id,
                    "amount": material.amount,
                })).collect::<Vec<_>>(),
                "internal_resources": cell.internal_resources.iter().map(|resource| json!({
                    "resource_type_id": resource.resource_type_id,
                    "amount": resource.amount,
                })).collect::<Vec<_>>(),
                "local_external_resources": cell.local_external_resources.iter().map(|resource| json!({
                    "resource_type_id": resource.resource_type_id,
                    "amount": resource.amount,
                })).collect::<Vec<_>>(),
                "phenotype_traits": json!({
                    "flagella_count": cell.phenotype_traits.flagella_count,
                    "spike_count": cell.phenotype_traits.spike_count,
                    "receptor_halo_intensity": cell.phenotype_traits.receptor_halo_intensity,
                    "lineage_hue": cell.phenotype_traits.lineage_hue,
                    "division_flash_intensity": cell.phenotype_traits.division_flash_intensity,
                }),
            })).collect::<Vec<_>>(),
            "joints": projection.joints.iter().map(|joint| json!({
                "id": joint.id,
                "cell1_id": joint.cell1_id,
                "cell2_id": joint.cell2_id,
                "rest_length": joint.rest_length,
                "pulse_intensity": joint.pulse_intensity,
                "signal_speed": joint.signal_speed,
            })).collect::<Vec<_>>(),
            "organisms": projection.organisms.iter().map(|organism| json!({
                "id": organism.id,
                "cell_ids": organism.cell_ids,
                "hull_color_hue": organism.hull_color_hue,
                "organic_membrane_tension": organism.organic_membrane_tension,
            })).collect::<Vec<_>>(),
            "resource_layers": projection.resource_layers.iter().map(|layer| json!({
                "layer_index": layer.layer_index,
                "resource_type_id": layer.resource_type_id,
                "resource_id": layer.resource_id,
                "width": layer.width,
                "height": layer.height,
                "total_amount": layer.total_amount,
                "cells": layer.cells.iter().map(|cell| json!({
                    "x": cell.x,
                    "y": cell.y,
                    "amount": cell.amount,
                })).collect::<Vec<_>>(),
                "completeness": completeness_json(&layer.completeness),
            })).collect::<Vec<_>>(),
            "fields": projection.fields.iter().map(|field| json!({
                "field_id": field.field_id,
                "value": field.value,
                "source_metric": {
                    "field_id": field.source_metric.field_id,
                    "source_owner": field.source_metric.source_owner,
                    "source_path": field.source_metric.source_path,
                },
            })).collect::<Vec<_>>(),
            "source_metrics": projection.source_metrics.iter().map(|source| json!({
                "field_id": source.field_id,
                "source_owner": source.source_owner,
                "source_path": source.source_path,
            })).collect::<Vec<_>>(),
        }
    }))
}

fn monitor_json(
    state: &crate::viewer_server::state::SharedState,
    snapshot: &CommittedSnapshot,
) -> Value {
    json!(build_monitor_data_panel_projection(
        snapshot,
        state.run_id.as_deref().unwrap_or("unavailable"),
    ))
}

fn empty_section_json(projection_kind: &str, payload_key: &str, payload: Value) -> Value {
    json!({
        "schema_version": format!("{projection_kind}/v1"),
        "projection_kind": projection_kind,
        "source": "live",
        "completeness": completeness_json(&ProjectionCompleteness::bounded(
            "Projection category is exposed by the gateway; no current live payload rows are available.",
        )),
        "payload": {
            payload_key: payload,
        }
    })
}

async fn handle_latest_projections(
    Query(params): Query<ProjectionParams>,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let mut locked = state.lock().unwrap();
    let stride = params.stride.unwrap_or(1);
    let Some(snapshot) = locked
        .engine
        .as_mut()
        .map(|engine| engine.latest_committed_snapshot())
    else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "ok": false,
                "category": "projection_unavailable",
                "projection_status": "unavailable",
                "message": "No active committed snapshot is available for Observer projections.",
            })),
        ));
    };
    let Some(visual_world) = visual_world_json(&locked, &snapshot, stride) else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "ok": false,
                "category": "projection_unavailable",
                "projection_status": "unavailable",
                "message": "No active committed snapshot is available for Observer projections.",
            })),
        ));
    };
    let monitor = monitor_json(&locked, &snapshot);

    let coverage = build_coverage_projection(Vec::new()).unwrap();
    let warnings = build_warning_projection(
        "CONFIG_TUNING_RECOMMENDED",
        "latest_live_projection",
        "observer_live_gateway",
        Vec::new(),
    )
    .map(|warning| vec![warning])
    .unwrap_or_default();

    let cell_classifications: Vec<Value> = snapshot
        .cells
        .iter()
        .map(|cell| {
            let material_names = [
                "boundary",
                "transport",
                "metabolic",
                "storage",
                "synthesis",
                "structural",
                "repair",
                "contractile",
                "sensory",
            ];
            let mut max_amount = 0.0f32;
            let mut max_index = 0usize;
            for (i, amount) in cell.materials.iter().enumerate() {
                if *amount > max_amount {
                    max_amount = *amount;
                    max_index = i;
                }
            }
            let total: f32 = cell.materials.iter().sum();
            let confidence = if total > 0.0 { max_amount / total } else { 0.0 };
            let role = if max_amount > 0.0 && max_index < material_names.len() {
                Some(material_names[max_index])
            } else {
                None
            };
            json!({
                "classification_id": format!("live:{}:cell-functional-role", cell.id.raw()),
                "dimension_id": "cell-functional-role",
                "entity_type": "Cell",
                "entity_id": format!("{}", cell.id.raw()),
                "mode": "Potential",
                "primary_label": role,
                "secondary_labels": [],
                "status": if role.is_some() { "Classified" } else { "Unknown" },
                "confidence": confidence,
                "tick_start": snapshot.tick.raw(),
                "tick_end": snapshot.tick.raw(),
                "classifier_version": "runner-inline-v1",
                "registry_version": "runner-inline-v1",
                "source_projection": "CommittedSnapshot.cells[].materials",
                "completeness": { "state": "bounded", "missing_fields": [], "reason": "Derived from snapshot material fractions." },
                "evidence": [],
                "limitations": ["Inline runner classification from snapshot material fractions only."]
            })
        })
        .collect();

    Ok(Json(json!({
        "schema_version": "ControlCenterProjectionBundle/v1",
        "projection_kind": "DebugProjectionBundle",
        "source": "live",
        "run_id": locked.run_id,
        "tick": locked.committed_tick,
        "config_hash": locked.scenario_hash,
        "engine_version": env!("CARGO_PKG_VERSION"),
        "visual_world": visual_world,
        "monitor": monitor,
        "coverage": empty_section_json(
            "CoverageProjection",
            "mechanisms",
            json!(coverage.mechanisms.iter().map(|mechanism| json!({
                "mechanism_id": mechanism.mechanism_id,
                "status_id": mechanism.status_id,
                "source_report": mechanism.source_report,
            })).collect::<Vec<_>>()),
        ),
        "warnings": empty_section_json(
            "WarningProjection",
            "warnings",
            json!(warnings.iter().map(|warning| json!({
                "code": warning.code,
                "disposition": format!("{:?}", warning.disposition),
                "affected_scope": warning.affected_scope,
                "source_report": warning.source_report,
                "recommended_reruns": warning.recommended_reruns,
            })).collect::<Vec<_>>()),
        ),
        "classifications": json!({
            "schema_version": "ClassificationProjection/v1",
            "projection_kind": "ClassificationProjection",
            "source": "live",
            "completeness": completeness_json(&ProjectionCompleteness::bounded(
                "Cell role classifications derived from snapshot material fractions.",
            )),
            "payload": {
                "classifications": cell_classifications,
            }
        }),
        "balance_findings": empty_section_json(
            "BalanceFindingProjection",
            "findings",
            json!([]),
        ),
    })))
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/projections/latest", get(handle_latest_projections))
        .with_state(state)
}
