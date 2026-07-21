use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use alife::storage::{
    ArtifactCompleteness, ArtifactKind, ArtifactRecord, RunId, RunMetadata, RunStatus,
    SqliteRunIndex, StorageSchemaVersion, TickRange,
};

#[test]
fn sqlite_index_persists_run_metadata_in_file_rows() {
    let db_path = temp_db_path("metadata");
    let index = SqliteRunIndex::open(&db_path).unwrap();
    let metadata = sample_metadata("run-alpha", RunStatus::Completed);

    index.upsert_run(&metadata).unwrap();
    let loaded = index.get_run(metadata.run_id()).unwrap().unwrap();

    assert_eq!(loaded, metadata);
    assert!(db_path.exists());

    let _ = fs::remove_file(db_path);
}

#[test]
fn sqlite_index_persists_artifact_references_without_blobs() {
    let db_path = temp_db_path("artifacts");
    let index = SqliteRunIndex::open(&db_path).unwrap();
    let run_id = RunId::new("run-artifacts").unwrap();
    index
        .upsert_run(&sample_metadata(run_id.as_str(), RunStatus::Completed))
        .unwrap();

    let artifact = ArtifactRecord::new(
        run_id.clone(),
        ArtifactKind::LineageEvents,
        "outputs/runs/run-artifacts/lineage.events",
        TickRange::closed(0, 128).unwrap(),
        ArtifactCompleteness::Bounded,
        Some("lineage event log reference only"),
    )
    .unwrap();

    index.insert_artifact(&artifact).unwrap();
    let loaded = index.artifacts_for_run(&run_id).unwrap();

    assert_eq!(loaded, vec![artifact]);

    let _ = fs::remove_file(db_path);
}

#[test]
fn deleting_database_file_resets_the_index_between_tests() {
    let db_path = temp_db_path("reset");
    {
        let index = SqliteRunIndex::open(&db_path).unwrap();
        index
            .upsert_run(&sample_metadata("run-reset", RunStatus::Completed))
            .unwrap();
        assert!(
            index
                .get_run(&RunId::new("run-reset").unwrap())
                .unwrap()
                .is_some()
        );
    }

    fs::remove_file(&db_path).unwrap();

    let index = SqliteRunIndex::open(&db_path).unwrap();
    assert!(
        index
            .get_run(&RunId::new("run-reset").unwrap())
            .unwrap()
            .is_none()
    );

    let _ = fs::remove_file(db_path);
}

#[test]
fn unavailable_keyframe_is_indexed_explicitly_without_substitution() {
    let db_path = temp_db_path("unavailable-keyframe");
    let index = SqliteRunIndex::open(&db_path).unwrap();
    let run_id = RunId::new("run-keyframe").unwrap();
    index
        .upsert_run(&sample_metadata(run_id.as_str(), RunStatus::Completed))
        .unwrap();

    let artifact = ArtifactRecord::new(
        run_id.clone(),
        ArtifactKind::ProjectionKeyframe,
        "outputs/runs/run-keyframe/keyframes/unavailable.tick-64",
        TickRange::closed(64, 64).unwrap(),
        ArtifactCompleteness::Unavailable,
        Some("keyframe not recorded at this tick"),
    )
    .unwrap();

    index.insert_artifact(&artifact).unwrap();

    let loaded = index.artifacts_for_run(&run_id).unwrap();
    assert_eq!(loaded[0].kind(), ArtifactKind::ProjectionKeyframe);
    assert_eq!(loaded[0].completeness(), ArtifactCompleteness::Unavailable);
    assert_eq!(loaded[0].tick_range().start(), 64);
    assert_eq!(loaded[0].tick_range().end(), Some(64));
    assert_eq!(
        loaded[0].notes(),
        Some("keyframe not recorded at this tick")
    );

    let _ = fs::remove_file(db_path);
}

#[test]
fn core_sources_do_not_import_storage_or_sqlite() {
    for path in [
        "src/core/world.rs",
        "src/core/tick.rs",
        "src/core/snapshot.rs",
        "src/core/lineage.rs",
    ] {
        let source = fs::read_to_string(path).unwrap();
        assert!(!source.contains("crate::storage"));
        assert!(!source.contains("rusqlite"));
        assert!(!source.contains("SqliteRunIndex"));
    }
}

fn sample_metadata(run_id: &str, status: RunStatus) -> RunMetadata {
    RunMetadata::new(
        RunId::new(run_id).unwrap(),
        "demo_living_world",
        "scenario_hash_v1:0000000000000042",
        42,
        "alife-test/005",
        1,
        StorageSchemaVersion::current(),
        status,
        TickRange::closed(0, 128).unwrap(),
        1_725_000_000_000,
        Some(1_725_000_010_000),
    )
    .unwrap()
}

fn temp_db_path(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("alife-{label}-{nonce}.sqlite"))
}
