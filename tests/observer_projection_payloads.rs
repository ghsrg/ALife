use alife::core::cell_store::LifecycleState;
use alife::core::ids::CellId;
use alife::core::snapshot::{
    CellSnapshot, CommittedSnapshot, ResourceLayerCellSnapshot, ResourceLayerSnapshot,
};
use alife::core::units::{EnergyAmount, Position, Radius, ResourceAmount, Tick};
use alife::observer::balance::{BalanceFinding, BalanceOutcome};
use alife::observer::classifiers::{
    ClassificationMode, ClassificationResult, ClassificationStatus, EvidenceRecord,
};
use alife::observer::contract::WarningDisposition;
use alife::observer::projection::{
    EntityType, build_classification_projection, build_coverage_projection,
    build_visual_world_projection, build_warning_projection, project_balance_finding,
};
use alife::observer::projection_envelope::ProjectionCompletenessState;

#[test]
fn visual_world_projection_is_bounded_and_source_backed() {
    let snapshot = CommittedSnapshot {
        tick: Tick::from_raw(42),
        cells: vec![CellSnapshot {
            id: CellId::from_raw(7),
            position: Position::new(3.0, 5.0),
            radius: Radius::new(1.5).unwrap(),
            energy: EnergyAmount::new(12.0).unwrap(),
            energy_capacity: EnergyAmount::new(20.0).unwrap(),
            lifecycle_state: LifecycleState::Alive,
            materials: [1.0, 0.5, 2.0, 0.0, 0.0, 1.5, 0.0, 0.25, 0.75],
            internal_resources: vec![ResourceAmount::new(3.0).unwrap()],
            local_external_resources: vec![
                ResourceAmount::new(1.0).unwrap(),
                ResourceAmount::new(2.0).unwrap(),
            ],
        }],
        heat: 2.5,
        waste: 0.75,
        resource_layer_totals: vec![
            ResourceAmount::new(4.0).unwrap(),
            ResourceAmount::new(8.0).unwrap(),
        ],
        resource_layers: vec![
            ResourceLayerSnapshot {
                layer_index: 0,
                width: 1,
                height: 1,
                total_amount: ResourceAmount::new(4.0).unwrap(),
                cells: vec![ResourceLayerCellSnapshot {
                    x: 0,
                    y: 0,
                    amount: ResourceAmount::new(4.0).unwrap(),
                }],
            },
            ResourceLayerSnapshot {
                layer_index: 1,
                width: 1,
                height: 1,
                total_amount: ResourceAmount::new(8.0).unwrap(),
                cells: vec![ResourceLayerCellSnapshot {
                    x: 0,
                    y: 0,
                    amount: ResourceAmount::new(8.0).unwrap(),
                }],
            },
        ],
        joints: vec![],
        organisms: vec![],
    };

    let payload = build_visual_world_projection(&snapshot);

    assert_eq!(payload.tick, 42);
    assert_eq!(
        payload.completeness.state(),
        ProjectionCompletenessState::Bounded
    );
    assert!(payload.completeness.missing_fields().is_empty());

    assert_eq!(payload.cells.len(), 1);
    assert_eq!(payload.cells[0].id, 7);
    assert_eq!(payload.cells[0].x, 3.0);
    assert_eq!(payload.cells[0].y, 5.0);
    assert_eq!(payload.cells[0].radius, 1.5);
    assert_eq!(payload.cells[0].energy, 12.0);
    assert_eq!(payload.cells[0].energy_capacity, 20.0);
    assert_eq!(payload.cells[0].lifecycle_state, LifecycleState::Alive);
    assert_eq!(payload.cells[0].materials.len(), 9);
    assert_eq!(payload.cells[0].internal_resources[0].amount, 3.0);
    assert_eq!(payload.cells[0].local_external_resources.len(), 2);

    assert_eq!(payload.resource_layers.len(), 2);
    assert_eq!(payload.resource_layers[0].layer_index, 0);
    assert_eq!(payload.resource_layers[0].total_amount, 4.0);
    assert_eq!(payload.resource_layers[0].cells[0].amount, 4.0);
    assert_eq!(payload.resource_layers[1].layer_index, 1);
    assert_eq!(payload.resource_layers[1].total_amount, 8.0);

    assert_eq!(payload.fields.len(), 2);
    assert_eq!(payload.fields[0].field_id, "heat");
    assert_eq!(payload.fields[0].value, 2.5);
    assert_eq!(payload.fields[1].field_id, "waste");
    assert_eq!(payload.fields[1].value, 0.75);

    let source_fields: Vec<&str> = payload
        .source_metrics
        .iter()
        .map(|source| source.field_id.as_str())
        .collect();
    assert_eq!(
        source_fields,
        vec![
            "tick",
            "cells.id",
            "cells.position",
            "cells.radius",
            "cells.energy",
            "cells.energy_capacity",
            "cells.materials",
            "cells.internal_resources",
            "cells.local_external_resources",
            "cells.lifecycle",
            "resource_layer_totals",
            "heat",
            "waste"
        ]
    );
}

#[test]
fn balance_projection_does_not_claim_balance_without_equal_requirements() {
    let finding = BalanceFinding {
        finding_id: "demo-a-b".to_string(),
        compared_profiles: ("a".to_string(), "b".to_string()),
        equal_requirements: false,
        result: BalanceOutcome::Balanced,
        evidence_metrics: vec!["a: survival_ticks=100, divisions_count=1".to_string()],
        dominance_rate: 0.0,
        affected_scenarios: vec!["demo_living_world".to_string()],
        suspected_cause: None,
        recommendation: None,
        recommended_reruns: vec!["equal-requirements-rerun".to_string()],
        confidence: 0.9,
    };

    let payload = project_balance_finding(&finding, "balance-report.json");

    assert_eq!(payload.finding_id, "demo-a-b");
    assert_eq!(payload.reported_result, BalanceOutcome::Balanced);
    assert_eq!(payload.claimed_result, BalanceOutcome::Inconclusive);
    assert!(!payload.equal_requirements);
    assert_eq!(payload.source_report, "balance-report.json");
    assert_eq!(
        payload.limitations,
        vec!["Balance claim suppressed because compared profiles do not have equal requirements."]
    );
}

#[test]
fn observer_payloads_do_not_enter_core_behavior() {
    let core_files = [
        "src/core/world.rs",
        "src/core/tick.rs",
        "src/core/cell_store.rs",
        "src/core/resources.rs",
    ];

    for file in core_files {
        let content = std::fs::read_to_string(file).unwrap();
        assert!(
            !content.contains("observer::payloads"),
            "{} must not depend on observer payloads",
            file
        );
        assert!(
            !content.contains("build_visual_world_projection"),
            "{} must not build observer projections",
            file
        );
    }
}

#[test]
fn coverage_projection_rejects_unknown_statuses() {
    let payload = build_coverage_projection(vec![(
        "PassiveUptake".to_string(),
        "covered".to_string(),
        "runner-summary.json".to_string(),
    )])
    .unwrap();

    assert_eq!(payload.mechanisms.len(), 1);
    assert_eq!(payload.mechanisms[0].mechanism_id, "PassiveUptake");
    assert_eq!(payload.mechanisms[0].status_id, "covered");
    assert_eq!(payload.mechanisms[0].source_report, "runner-summary.json");

    let err = build_coverage_projection(vec![(
        "UnknownMechanism".to_string(),
        "maybe_covered".to_string(),
        "runner-summary.json".to_string(),
    )])
    .unwrap_err();
    assert_eq!(err.to_string(), "unknown coverage status: maybe_covered");
}

#[test]
fn warning_projection_preserves_canonical_and_legacy_dispositions() {
    let canonical = build_warning_projection(
        "CONFIG_TUNING_RECOMMENDED",
        "demo_living_world",
        "balance-report.json",
        vec!["AL-004-S05-rich-world".to_string()],
    )
    .unwrap();
    assert_eq!(canonical.code, "CONFIG_TUNING_RECOMMENDED");
    assert_eq!(
        canonical.disposition,
        WarningDisposition::CanonicalObserverWarning
    );

    let legacy = build_warning_projection(
        "LOW_INFORMATION_SWEEP",
        "demo_living_world",
        "legacy-sweep.json",
        Vec::new(),
    )
    .unwrap();
    assert_eq!(
        legacy.disposition,
        WarningDisposition::LegacyAnalyzerWarning
    );

    let err = build_warning_projection("MADE_UP_WARNING", "world", "report.json", Vec::new())
        .unwrap_err();
    assert_eq!(err.to_string(), "unknown warning code: MADE_UP_WARNING");
}

#[test]
fn classification_projection_keeps_deterministic_provenance() {
    let result = ClassificationResult {
        dimension_id: "cell-functional-role".to_string(),
        entity_id: "cell-7".to_string(),
        mode: ClassificationMode::Observed,
        primary_label: Some("transport-like".to_string()),
        secondary_labels: vec![],
        status: ClassificationStatus::Classified,
        confidence: 0.82,
        tick_start: 10,
        tick_end: 20,
        classifier_version: "cell-role/v3".to_string(),
        evidence: vec![EvidenceRecord {
            feature: "ActiveUptake_executed".to_string(),
            expected: "> 0".to_string(),
            actual: 3.0,
            matched: true,
        }],
        data_completeness: 0.6,
    };

    let payload = build_classification_projection(
        &result,
        EntityType::Cell,
        "run-a",
        "registry/v2",
        "BehaviorProfileProjection",
        vec!["Observed role is based on process counters, not material ownership.".to_string()],
    );

    assert_eq!(
        payload.classification_id,
        "run-a:Cell:cell-7:cell-functional-role:10-20:Observed:cell-role/v3:registry/v2"
    );
    assert_eq!(payload.entity_type, EntityType::Cell);
    assert_eq!(payload.primary_label.as_deref(), Some("transport-like"));
    assert_eq!(payload.status, ClassificationStatus::Classified);
    assert_eq!(payload.confidence, 0.82);
    assert_eq!(payload.classifier_version, "cell-role/v3");
    assert_eq!(payload.registry_version, "registry/v2");
    assert_eq!(payload.source_projection, "BehaviorProfileProjection");
    assert_eq!(
        payload.completeness.state(),
        ProjectionCompletenessState::Partial
    );
    assert_eq!(
        payload.limitations,
        vec!["Observed role is based on process counters, not material ownership."]
    );
    assert_eq!(payload.evidence.len(), 1);
    assert_eq!(payload.evidence[0].feature, "ActiveUptake_executed");
    assert_eq!(
        payload.evidence[0].source_metric.field_id,
        "ActiveUptake_executed"
    );
}
