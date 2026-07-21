---
tags:
  - alife
  - worklog/report
  - plan/AL-004-S02
  - area/observer
  - area/projection
---

# REPORT: AL-004-S02 Versioned Projection Envelope

## Purpose

Close `AL-004-S02` by implementing a shared Rust-only typed projection envelope for Observer/Runner/UI/storage/report consumers without changing Core behavior or ALIF v2 binary transport semantics.

Worklogs are evidence only, not source of truth.

## Source Documents Read

- `docs/PRINCIPLES.md`
- `docs/observer/observer-layer.md`
- `docs/observer/projection-contract.md`
- `docs/runner/projections.md`
- `outputs/worklogs/2026-07-21-1248-PLAN-al-004-s02-versioned-projection-envelope.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`

## Changed Files

- `src/observer/projection_envelope.rs`: adds typed schema version, projection kind/entity/source/completeness vocabulary, build context, envelope wrapper, generic enveloped payload, and Rust-only schema export disposition.
- `src/observer/mod.rs`: exposes the projection envelope module.
- `src/observer/contract.rs`: adds envelope-level field inventory mapped to `AL-004-S02`.
- `src/runner/projections.rs`: adds non-breaking `WorldFrameProjection::as_enveloped(context)` helper.
- `tests/projection_envelope_contract.rs`: covers envelope vocabulary, Runner frame mapping, ALIF v2 body compatibility, completeness states, projection-kind coverage, Observer inventory mapping, schema disposition, and Core boundary guard.
- `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `docs/delivery/worklog-ledger.md`, `outputs/worklogs/index.md`: delivery closure and Candidate Next Work update.

## Verification

| Evidence ID | Command | Result |
| --- | --- | --- |
| `AL-004-S02-EV01` | `$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo test --test projection_envelope_contract projection_envelope_declares_required_metadata_vocabulary` | RED passed by failing on missing `observer::projection_envelope`, `as_enveloped`, and `ProjectionEnvelope` contract surface. First compile attempt timed out before RED; repeat reached expected failure. |
| `AL-004-S02-EV02`-`AL-004-S02-EV10` | `$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo test --test projection_envelope_contract` | PASS: 9 passed, 0 failed. |
| `AL-004-S02-EV11` | `$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo test --test projection_envelope_contract --test observer_contract_closure --test runner_projection_world_frame --test runner_frame_encoder --test runner_projection_sampler --test runner_tick_broadcast --test runner_ws_stream --test phase3d_lineage_replay` | PASS: 52 passed, 0 failed across the focused regression set. |
| `AL-004-S02-EV12` | `$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo fmt --check`; `git diff --check`; deterministic path/ID traceability scan | PASS: format check and whitespace check exited 0; all new AL-004-S02 report/code/test/doc paths resolved. `git diff --check` reported line-ending warnings only. |

Full workspace `cargo test` was intentionally not run to avoid unnecessary disk growth. The focused regression uses `target\codex-al004s02`.

## Coverage Matrix

| Plan ID | Requirement | Scenario ID | Task ID(s) | Evidence ID(s) | Test/Evidence | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `AL-004-S02` | Canonical projection envelope vocabulary | `AL-004-S02-AC01` | `AL-004-S02-T01`, `AL-004-S02-T02` | `AL-004-S02-EV01`, `AL-004-S02-EV02` | `projection_envelope_declares_required_metadata_vocabulary` | covered |
| `AL-004-S02` | Current Runner frame maps into envelope without ALIF rewrite | `AL-004-S02-AC02` | `AL-004-S02-T03`, `AL-004-S02-T04` | `AL-004-S02-EV03`, `AL-004-S02-EV04` | `world_frame_projection_wraps_with_runner_envelope_metadata`, `world_frame_envelope_does_not_change_alif_v2_binary_body`, `runner_frame_encoder` regression | covered |
| `AL-004-S02` | Completeness, source, unavailable, and stale semantics | `AL-004-S02-AC03` | `AL-004-S02-T05`, `AL-004-S02-T06` | `AL-004-S02-EV05`, `AL-004-S02-EV06` | completeness constructor tests | covered |
| `AL-004-S02` | Projection-kind coverage for downstream slices | `AL-004-S02-AC04` | `AL-004-S02-T07`, `AL-004-S02-T08` | `AL-004-S02-EV07`, `AL-004-S02-EV08` | projection-kind vocabulary and Observer inventory mapping tests | covered |
| `AL-004-S02` | Observer boundary and Rust-only schema disposition | `AL-004-S02-AC05` | `AL-004-S02-T09`, `AL-004-S02-T10` | `AL-004-S02-EV09`, `AL-004-S02-EV10` | schema disposition test and Core source boundary guard | covered |
| `AL-004-S02` | Delivery closure and handoff | `AL-004-S02-AC06` | `AL-004-S02-T11`, `AL-004-S02-T12` | `AL-004-S02-EV11`, `AL-004-S02-EV12` | roadmap/status/acceptance/ledger updates and final verification | covered |

## Status Update

- `AL-004-S02`: `done`, `high`.
- `Current Focus`: none selected after closure.
- `Candidate Next Work`: reviewed and updated. `AL-002-S16` remains first executable next item; `AL-005-S01` is now ready to plan because the typed envelope exists; `AL-007-S09` remains blocked by active Runner hardening and storage/index dependencies.

## Follow-Up Slices

- `AL-005-S01`: run metadata, storage index, keyframe/history ownership, and durable artifact references.
- `AL-007-S09`: UI version compatibility, stale/unavailable Tick presentation, and keyframe/history UI after active dependencies are staged.
- `AL-004-S04`/`AL-004-S05`: concrete OrganismView, coverage, warning, and balance payload projections.

## Notes

- `Needs Review`: `docs/observer/projection-contract.md` still names `generated_at`, while the Rust envelope uses `generated_at_unix_ms` to match existing frame timestamp semantics and avoid a new time dependency.
- `Needs Review`: generated JSON/TOML/TypeScript schema export remains deferred until a consumer slice proves it is needed.
- `Needs Review`: `run_id`, `config_hash`, and `engine_version` remain optional in fixture/in-process envelopes but are required by the `runner_live` constructor.
