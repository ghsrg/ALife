use alife::storage::{RunId, RunMetadata, RunStatus, StorageSchemaVersion, TickRange};

#[test]
fn run_metadata_records_reproducibility_identity_and_status() {
    let metadata = RunMetadata::new(
        RunId::new("run-alpha").unwrap(),
        "demo_living_world",
        "scenario_hash_v1:0000000000000042",
        42,
        "alife-test/005",
        1,
        StorageSchemaVersion::current(),
        RunStatus::Completed,
        TickRange::closed(0, 128).unwrap(),
        1_725_000_000_000,
        Some(1_725_000_010_000),
    )
    .unwrap();

    assert_eq!(metadata.run_id().as_str(), "run-alpha");
    assert_eq!(metadata.scenario_id(), "demo_living_world");
    assert_eq!(
        metadata.scenario_hash(),
        "scenario_hash_v1:0000000000000042"
    );
    assert_eq!(metadata.effective_seed(), 42);
    assert_eq!(metadata.engine_version(), "alife-test/005");
    assert_eq!(metadata.scenario_schema_version(), 1);
    assert_eq!(
        metadata.storage_schema_version(),
        StorageSchemaVersion::current()
    );
    assert_eq!(metadata.status(), RunStatus::Completed);
    assert_eq!(metadata.tick_range().start(), 0);
    assert_eq!(metadata.tick_range().end(), Some(128));
    assert_eq!(metadata.started_at_unix_ms(), 1_725_000_000_000);
    assert_eq!(metadata.ended_at_unix_ms(), Some(1_725_000_010_000));
}

#[test]
fn run_metadata_rejects_invalid_identity_and_tick_ranges() {
    assert!(RunId::new("").is_err());
    assert!(RunId::new("run/alpha").is_err());
    assert!(TickRange::closed(10, 9).is_err());
}
