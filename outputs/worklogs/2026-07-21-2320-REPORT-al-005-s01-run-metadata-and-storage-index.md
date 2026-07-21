---
tags:
  - alife
  - worklog/report
  - report/AL-005-S01
  - area/storage
  - area/runner
---

# REPORT: AL-005-S01 Run Metadata And Storage Index

Plan ID: `AL-005-S01`

Outcome: `PASS`

Selected slice: Run Metadata And Storage Index

Purpose: implement a minimal file-backed SQLite run metadata/index boundary for completed or failed runs, with artifact reference rows and no storage authority over Core behavior.

Worklogs are evidence, not source of truth.

## Source Documents Read

- `docs/PRINCIPLES.md`
- `docs/GLOSSARY.md`
- `docs/engine/storage.md`
- `docs/engine/serialization.md`
- `docs/mechanics/snapshot-replay.md`
- `docs/observer/projection-contract.md`
- `docs/runner/runner.md`
- `docs/runner/run-lifecycle.md`
- `docs/runner/projections.md`
- `docs/implementation/implementation-phases.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `src/core/snapshot.rs`
- `src/core/lineage.rs`
- `src/observer/projection_envelope.rs`
- `src/runner/engine.rs`
- `src/viewer_server/state.rs`

## Changed Files

- `Cargo.toml`, `Cargo.lock`: add `rusqlite` with bundled SQLite for a minimal file-backed DB.
- `src/storage/mod.rs`: add storage-owned run metadata/domain types, artifact reference records, and `SqliteRunIndex`.
- `src/lib.rs`: expose `storage`.
- `tests/storage_run_metadata.rs`: cover reproducibility metadata, validated run identity, and tick ranges.
- `tests/storage_sqlite_index.rs`: cover SQLite metadata rows, artifact references without blobs, file-delete reset, unavailable keyframe references, and Core boundary guard.
- Delivery artifacts: roadmap, status, acceptance matrix, worklog ledger, worklog index.

## Coverage Matrix

| Scenario ID | Task IDs | Evidence IDs | Requirement | Evidence | Status |
| --- | --- | --- | --- | --- | --- |
| `AL-005-S01-AC01` | `AL-005-S01-T01`, `AL-005-S01-T02` | `AL-005-S01-EV01`, `AL-005-S01-EV02` | Run metadata records run id, scenario id/hash, seed, engine version, schema version, lifecycle status, tick range, and timestamps. | `tests/storage_run_metadata.rs` | covered |
| `AL-005-S01-AC02` | `AL-005-S01-T03`, `AL-005-S01-T04` | `AL-005-S01-EV03`, `AL-005-S01-EV04` | File-backed SQLite index persists run metadata rows and can be reset by deleting the DB file. | `tests/storage_sqlite_index.rs` | covered |
| `AL-005-S01-AC03` | `AL-005-S01-T05`, `AL-005-S01-T06` | `AL-005-S01-EV05`, `AL-005-S01-EV06` | Artifact rows store references, kind, tick range, completeness, and notes without storing payload blobs. | `tests/storage_sqlite_index.rs` | covered |
| `AL-005-S01-AC04` | `AL-005-S01-T07`, `AL-005-S01-T08` | `AL-005-S01-EV07`, `AL-005-S01-EV08` | Lineage/keyframe handoff is represented as explicit artifact references; unavailable keyframes are not silently substituted. | `tests/storage_sqlite_index.rs` | covered |
| `AL-005-S01-AC05` | `AL-005-S01-T09`, `AL-005-S01-T10` | `AL-005-S01-EV09`, `AL-005-S01-EV10` | Storage remains outside Core and Tick hot path. | source guard in `tests/storage_sqlite_index.rs` | covered |
| `AL-005-S01-AC06` | `AL-005-S01-T11`, `AL-005-S01-T12`, `AL-005-S01-T13` | `AL-005-S01-EV11`, `AL-005-S01-EV12` | Existing lineage/projection/runner closure behavior remains intact. | focused regression suite and delivery updates | covered |

## Verification

RED evidence:

- `$env:CARGO_TARGET_DIR='target\codex-al005s01'; cargo test --test storage_run_metadata`
  - Result: failed as expected with `could not find storage in alife`.
- `$env:CARGO_TARGET_DIR='target\codex-al005s01'; cargo test --test storage_sqlite_index`
  - Result: failed as expected with missing `ArtifactCompleteness`, `ArtifactKind`, `ArtifactRecord`, and `SqliteRunIndex`.

GREEN evidence:

- `$env:CARGO_TARGET_DIR='target\codex-al005s01'; cargo test --test storage_run_metadata`
  - Result: 2 passed, 0 failed.
- `$env:CARGO_TARGET_DIR='target\codex-al005s01'; cargo test --test storage_sqlite_index`
  - Result: 5 passed, 0 failed.
- `$env:CARGO_TARGET_DIR='target\codex-al005s01'; cargo test --test storage_run_metadata --test storage_sqlite_index`
  - Result: 7 passed, 0 failed.
- `$env:CARGO_TARGET_DIR='target\codex-al005s01'; cargo test --test projection_envelope_contract --test phase3d_lineage_replay --test runner_graceful_shutdown`
  - Result: 21 passed, 0 failed.
- `$env:CARGO_TARGET_DIR='target\codex-al005s01'; cargo fmt --check`
  - Result: PASS after `cargo fmt`.
- `$env:CARGO_TARGET_DIR='target\codex-al005s01'; cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - Result: FAIL on pre-existing out-of-scope warnings in `src/core/world.rs` (`too_many_arguments`) and `src/viewer_server/api/stream.rs` (`collapsible_if`). The AL-005-S01 storage warning found during the first clippy run was fixed.

Notes:

- The first SQLite test run required network access to fetch `rusqlite` and `libsqlite3-sys`; after that, focused tests used `target\codex-al005s01`.
- Full workspace `cargo test` was not run to avoid unnecessary target growth and disk pressure.

## Status Update Recommendation

- Set `AL-005-S01` to `done` / `high`.
- Set parent `AL-005` to `in-progress` / `medium`, because `AL-005-S02` through `AL-005-S04` remain planned.
- Clear `Current Focus`.
- Move `AL-007-S09` from blocked to ready-to-plan because durable run metadata/artifact ownership is now explicit.

## Follow-Up Scope

- `AL-007-S09`: UI/history can consume explicit metadata/artifact references and unavailable keyframe semantics.
- `AL-005-S02`: analytics export foundation remains separate and depends on Observer balance/warning projections.
- `AL-005-S03`: long-run scenario suite remains separate.
- `AL-005-S04`: calibration/comparison tooling remains separate.

## Notes

- SQLite rows contain references and metadata only; no full snapshot, lineage tree, or projection blob is stored by default.
- Deleting the DB file is the intended lightweight test reset path.
- `Needs Review`: actual binary snapshot/event serialization remains future storage/replay work; this slice only owns the index and references.
