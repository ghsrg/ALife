use std::collections::BTreeSet;
use std::fs;

use alife::observer::contract::{ObserverConsumerSurface, observer_field_by_id};
use alife::observer::projection_envelope::{
    ProjectionBuildContext, ProjectionCompleteness, ProjectionCompletenessState,
    ProjectionEnvelope, ProjectionKind, ProjectionSchemaVersion, ProjectionSource,
    SchemaExportDisposition, schema_export_disposition,
};
use alife::runner::projections::{ProjectedCell, WorldFrameProjection};
use alife::viewer_server::frame_encoder::{decode_frame, encode_world_frame};

#[test]
fn projection_envelope_declares_required_metadata_vocabulary() {
    let envelope = ProjectionEnvelope::new(
        ProjectionSchemaVersion::new("FixtureProjection", 1, 0),
        ProjectionKind::Metrics,
        Some(12),
        ProjectionBuildContext::fixture(ProjectionCompleteness::full(), 1_725_000_000_000),
    );

    assert_eq!(envelope.schema_version.family(), "FixtureProjection");
    assert_eq!(envelope.schema_version.major(), 1);
    assert_eq!(envelope.schema_version.minor(), 0);
    assert_eq!(envelope.projection_kind, ProjectionKind::Metrics);
    assert_eq!(envelope.run_id, None);
    assert_eq!(envelope.tick, Some(12));
    assert_eq!(envelope.config_hash, None);
    assert_eq!(envelope.engine_version, None);
    assert_eq!(envelope.source, ProjectionSource::Fixture);
    assert_eq!(
        envelope.completeness.state(),
        ProjectionCompletenessState::Full
    );
    assert_eq!(envelope.generated_at_unix_ms, 1_725_000_000_000);
}

#[test]
fn world_frame_projection_wraps_with_runner_envelope_metadata() {
    let frame = sample_world_frame();
    let context = ProjectionBuildContext::runner_live(
        "run-alpha",
        0xA11F_E004,
        "alife-test/004",
        ProjectionCompleteness::bounded("viewport frame"),
        1_725_000_001_234,
    );

    let enveloped = frame.as_enveloped(context);

    assert_eq!(enveloped.envelope.projection_kind, ProjectionKind::Frame);
    assert_eq!(
        enveloped.envelope.schema_version.family(),
        "WorldFrameProjection"
    );
    assert_eq!(
        enveloped.envelope.schema_version.major(),
        u16::from(WorldFrameProjection::SCHEMA_VERSION)
    );
    assert_eq!(enveloped.envelope.tick, Some(frame.committed_tick));
    assert_eq!(enveloped.envelope.run_id.as_deref(), Some("run-alpha"));
    assert_eq!(enveloped.envelope.config_hash, Some(0xA11F_E004));
    assert_eq!(
        enveloped.envelope.engine_version.as_deref(),
        Some("alife-test/004")
    );
    assert_eq!(enveloped.envelope.source, ProjectionSource::Live);
    assert_eq!(
        enveloped.envelope.completeness.state(),
        ProjectionCompletenessState::Bounded
    );
    assert_eq!(enveloped.payload, frame);
}

#[test]
fn world_frame_envelope_does_not_change_alif_v2_binary_body() {
    let frame = sample_world_frame();
    let direct_body = encode_world_frame(&frame);

    let enveloped = frame.as_enveloped(ProjectionBuildContext::runner_live(
        "run-alpha",
        0xA11F_E004,
        "alife-test/004",
        ProjectionCompleteness::full(),
        1_725_000_001_234,
    ));
    let enveloped_body = encode_world_frame(&enveloped.payload);

    assert_eq!(enveloped_body, direct_body);
    assert_eq!(decode_frame(&enveloped_body).unwrap(), frame);
}

#[test]
fn projection_completeness_records_partial_missing_fields() {
    let completeness = ProjectionCompleteness::partial(
        vec!["resources", "cells.materials", "resources"],
        "ALIF v2 frame body does not include resource or material payloads yet",
    );

    assert_eq!(completeness.state(), ProjectionCompletenessState::Partial);
    assert_eq!(
        completeness.missing_fields(),
        &["cells.materials", "resources"]
    );
    assert_eq!(
        completeness.reason(),
        Some("ALIF v2 frame body does not include resource or material payloads yet")
    );
}

#[test]
fn projection_completeness_records_stale_and_unavailable_reasons() {
    let stale = ProjectionCompleteness::stale("stream reconnecting");
    let unavailable = ProjectionCompleteness::unavailable("historical ticks are not stored yet");

    assert_eq!(stale.state(), ProjectionCompletenessState::Stale);
    assert_eq!(stale.reason(), Some("stream reconnecting"));
    assert!(stale.missing_fields().is_empty());

    assert_eq!(
        unavailable.state(),
        ProjectionCompletenessState::Unavailable
    );
    assert_eq!(
        unavailable.reason(),
        Some("historical ticks are not stored yet")
    );
    assert!(unavailable.missing_fields().is_empty());
}

#[test]
fn projection_kind_vocabulary_covers_planned_observer_projection_kinds() {
    let kinds = ProjectionKind::all_canonical()
        .iter()
        .map(|kind| kind.as_str())
        .collect::<BTreeSet<_>>();

    for expected in [
        "FrameProjection",
        "EntityProjection",
        "InspectorProjection",
        "MetricsProjection",
        "OrganismViewProjection",
        "LineageProjection",
        "CoverageProjection",
        "BehaviorProfileProjection",
        "BalanceFindingProjection",
        "ClassificationProjection",
        "DebugTraceProjection",
    ] {
        assert!(
            kinds.contains(expected),
            "missing projection kind {expected}"
        );
    }
}

#[test]
fn observer_contract_maps_projection_envelope_fields_to_al_004_s02() {
    for field_id in [
        "projection_envelope.schema_version",
        "projection_envelope.projection_kind",
        "projection_envelope.run_id",
        "projection_envelope.tick",
        "projection_envelope.config_hash",
        "projection_envelope.engine_version",
        "projection_envelope.source",
        "projection_envelope.completeness",
        "projection_envelope.generated_at_unix_ms",
    ] {
        let spec = observer_field_by_id(field_id).expect("missing envelope field spec");
        assert_eq!(
            spec.consumer_surface,
            ObserverConsumerSurface::ProjectionEnvelope
        );
        assert_eq!(spec.follow_up_plan_id, Some("AL-004-S02"));
        assert!(!spec.mutable);
    }
}

#[test]
fn projection_envelope_declares_rust_typed_contract_only_schema_disposition() {
    assert_eq!(
        schema_export_disposition(),
        SchemaExportDisposition::RustTypedContractOnly
    );
}

#[test]
fn projection_envelope_does_not_enter_core_behavior_inputs() {
    for path in [
        "src/core/genome.rs",
        "src/core/tick.rs",
        "src/core/world.rs",
        "src/core/process.rs",
        "src/core/stable_state_hash.rs",
    ] {
        let source = fs::read_to_string(path).expect("core source should be readable");
        assert!(
            !source.contains("projection_envelope") && !source.contains("ProjectionEnvelope"),
            "projection envelope leaked into core behavior source {path}"
        );
    }
}

fn sample_world_frame() -> WorldFrameProjection {
    WorldFrameProjection {
        schema_version: WorldFrameProjection::SCHEMA_VERSION,
        committed_tick: 77,
        projection_sequence: 12,
        wall_clock_generated_at_ms: 1_725_000_001_000,
        previous_committed_tick: Some(76),
        heat: 3.5,
        waste: 1.25,
        cells: vec![ProjectedCell {
            id: 42,
            x: 9.0,
            y: 4.5,
            radius: 1.25,
            energy: 8.0,
            lifecycle: 0,
        }],
    }
}
