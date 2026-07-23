use alife::bootstrap::preview::{
    BootstrapPreviewOptions, SeedSweepOptions, build_bootstrap_preview, run_bootstrap_seed_sweep,
};
use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};

fn demo_document() -> ScenarioDocument {
    ScenarioDocument::resolve(ScenarioSource::Path(
        "config/scenarios/demo/demo_world_resource.toml".into(),
    ))
    .expect("demo_world_resource should resolve")
}

#[test]
fn bootstrap_preview_exports_stable_manifest_and_bounded_resource_preview_without_ticks() {
    let document = demo_document();
    let preview_a = build_bootstrap_preview(
        &document,
        BootstrapPreviewOptions {
            max_resource_cells_per_layer: 4,
        },
    )
    .expect("preview should build");
    let preview_b = build_bootstrap_preview(
        &document,
        BootstrapPreviewOptions {
            max_resource_cells_per_layer: 4,
        },
    )
    .expect("preview should be deterministic");

    assert_eq!(preview_a.scenario_id, "demo_world_resource");
    assert_eq!(preview_a.tick_executed, false);
    assert_eq!(preview_a.scenario_hash, preview_b.scenario_hash);
    assert_eq!(preview_a.prepared_state_hash, preview_b.prepared_state_hash);
    assert_eq!(preview_a.generator_versions, preview_b.generator_versions);
    assert_eq!(preview_a.resource_layers.len(), 27);
    assert_eq!(preview_a.resource_layers[0].sampled_cells.len(), 4);
    assert!(preview_a.resource_layers[0].total > 0.0);
    assert!(preview_a.resource_layers[0].max >= preview_a.resource_layers[0].min);
    assert!(preview_a.cell_summary.initial_cells >= 1);
}

#[test]
fn bootstrap_preview_keeps_fields_manifest_only_with_explicit_warning() {
    let document = demo_document();
    let preview = build_bootstrap_preview(&document, BootstrapPreviewOptions::default())
        .expect("preview should build");

    assert_eq!(preview.field_layers.len(), 5);
    assert!(
        preview
            .field_layers
            .iter()
            .any(|field| field.field_id == "heat")
    );
    assert!(
        preview
            .field_layers
            .iter()
            .any(|field| field.field_id == "waste")
    );
    assert!(
        preview
            .field_layers
            .iter()
            .any(|field| field.field_id == "temperature")
    );
    assert!(
        preview
            .field_layers
            .iter()
            .all(|field| field.spatial_grid_available == false)
    );
    assert!(preview.warnings.iter().any(|warning| {
        warning.code == "BOOTSTRAP_FIELD_LAYER_NOT_CORE_INTEGRATED"
            && warning.message.contains("manifest-only")
    }));
}

#[test]
fn seed_sweep_report_is_deterministic_and_compact() {
    let document = demo_document();
    let report_a = run_bootstrap_seed_sweep(
        &document,
        SeedSweepOptions {
            first_seed: 10,
            seed_count: 4,
            max_resource_cells_per_layer: 2,
        },
    )
    .expect("seed sweep should build");
    let report_b = run_bootstrap_seed_sweep(
        &document,
        SeedSweepOptions {
            first_seed: 10,
            seed_count: 4,
            max_resource_cells_per_layer: 2,
        },
    )
    .expect("seed sweep should be deterministic");

    assert_eq!(report_a.rows, report_b.rows);
    assert_eq!(report_a.rows.len(), 4);
    assert!(report_a.rows.iter().all(|row| row.tick_executed == false));
    assert!(
        report_a
            .rows
            .iter()
            .all(|row| row.resource_layer_count == 27)
    );
    assert!(report_a.rows.iter().any(|row| !row.warnings.is_empty()));
}
