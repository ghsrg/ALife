use crate::observer::projection::{
    build_coverage_projection, build_visual_world_projection, build_warning_projection,
};
use crate::observer::projection_envelope::{ProjectionCompleteness, ProjectionCompletenessState};
use crate::viewer_server::state::AppState;
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde_json::{Value, json};

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

fn visual_world_json(state: &mut crate::viewer_server::state::SharedState) -> Option<Value> {
    let snapshot = state.engine.as_mut()?.latest_committed_snapshot();
    let projection = build_visual_world_projection(&snapshot);

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
                "lifecycle_state": format!("{:?}", cell.lifecycle_state),
                "materials": cell.materials.iter().map(|material| json!({
                    "material_type_id": material.material_type_id,
                    "amount": material.amount,
                })).collect::<Vec<_>>(),
                "internal_resources": cell.internal_resources.iter().map(|resource| json!({
                    "resource_type_id": resource.resource_type_id,
                    "amount": resource.amount,
                })).collect::<Vec<_>>(),
            })).collect::<Vec<_>>(),
            "resource_layers": projection.resource_layers.iter().map(|layer| json!({
                "layer_index": layer.layer_index,
                "total_amount": layer.total_amount,
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
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let mut locked = state.lock().unwrap();
    let Some(visual_world) = visual_world_json(&mut locked) else {
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

    let coverage = build_coverage_projection(Vec::new()).unwrap();
    let warnings = build_warning_projection(
        "CONFIG_TUNING_RECOMMENDED",
        "latest_live_projection",
        "observer_live_gateway",
        Vec::new(),
    )
    .map(|warning| vec![warning])
    .unwrap_or_default();

    Ok(Json(json!({
        "schema_version": "ControlCenterProjectionBundle/v1",
        "projection_kind": "DebugProjectionBundle",
        "source": "live",
        "run_id": locked.run_id,
        "tick": locked.committed_tick,
        "config_hash": locked.scenario_hash,
        "engine_version": env!("CARGO_PKG_VERSION"),
        "visual_world": visual_world,
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
        "classifications": empty_section_json(
            "ClassificationProjection",
            "classifications",
            json!([]),
        ),
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
