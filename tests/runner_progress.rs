use alife::runner::progress::{ProgressInterval, ProgressSnapshot, format_progress_table};

#[test]
fn progress_interval_defaults_to_2000_ms_and_rejects_zero() {
    assert_eq!(ProgressInterval::default().as_millis(), 2000);
    assert!(ProgressInterval::from_millis(0).is_err());
    assert_eq!(ProgressInterval::from_millis(250).unwrap().as_millis(), 250);
}

#[test]
fn progress_table_contains_required_status_fields() {
    let rendered = format_progress_table(&ProgressSnapshot {
        elapsed_ms: 2500,
        tick: 10,
        max_ticks: 20,
        ticks_per_second: 40.0,
        cells: 3,
        alive_cells: Some(2),
        dead_cells: Some(1),
        heat: 1.5,
        waste: 2.5,
        state: "Running".to_string(),
        collapse_reason: None,
        snapshot_builds: 1,
        genome_refreshes: 0,
        resource_decay_elapsed_ticks: 0,
    });

    assert!(rendered.contains("elapsed_s"));
    assert!(rendered.contains("tick"));
    assert!(rendered.contains("tps"));
    assert!(rendered.contains("cells"));
    assert!(rendered.contains("2.50"));
    assert!(rendered.contains("10/20"));
    assert!(rendered.contains("Running"));
}

#[test]
fn progress_table_contains_scheduler_diagnostics() {
    let rendered = format_progress_table(&ProgressSnapshot {
        elapsed_ms: 2500,
        tick: 10,
        max_ticks: 20,
        ticks_per_second: 40.0,
        cells: 3,
        alive_cells: Some(2),
        dead_cells: Some(1),
        heat: 1.5,
        waste: 2.5,
        state: "Running".to_string(),
        collapse_reason: None,
        snapshot_builds: 2,
        genome_refreshes: 0,
        resource_decay_elapsed_ticks: 5,
    });

    assert!(rendered.contains("snapshots"));
    assert!(rendered.contains("genome"));
    assert!(rendered.contains("decay_dt"));
    assert!(rendered.contains("2"));
    assert!(rendered.contains("5"));
}
