---
tags:
  - alife
  - delivery/acceptance
---

# Delivery Acceptance

## Current/Next Acceptance Matrix

Acceptance rows in this section use executable slice Plan IDs. Add or change rows
here only for the current delivery slice and Candidate Next Work.

| Acceptance ID | Plan ID | Acceptance outcome | Required evidence |
| --- | --- | --- | --- |
| `AL-001-S04-AC01` | `AL-001-S04` | Roadmap, status, worklog ledger, source map, control vocabulary, and acceptance matrix have separate responsibilities. | `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/worklog-ledger.md`, `docs/delivery/source-map.md`, `docs/delivery/control.md`, `docs/delivery/acceptance.md` |
| `AL-001-S04-AC02` | `AL-001-S04` | Historical worklog-derived roadmap rows are preserved in the ledger and removed from active roadmap scope. | `docs/delivery/worklog-ledger.md`, `docs/delivery/roadmap.md` |
| `AL-001-S04-AC03` | `AL-001-S04` | Status is an operational dashboard for current, active, blocked, ready-next, and recently closed work. | `docs/delivery/status.md` |
| `AL-001-S04-AC04` | `AL-001-S04` | Current and next acceptance rows use slice-level `AL-###-S##-AC##` IDs. | `docs/delivery/acceptance.md` |
| `AL-001-S04-AC05` | `AL-001-S04` | Candidate Next Work is reviewed during roadmap changes. | `docs/delivery/roadmap.md`, `AGENTS.MD` |
| `AL-003-S02-AC01` | `AL-003-S02` | Genome Runtime contract and registered output coverage can be planned before scheduler/cadence work. | `docs/delivery/roadmap.md` |
| `AL-003-S03-AC01` | `AL-003-S03` | Scheduled Genome Runtime cadence is verified on top of the closed `AL-003-S02` contract without every-Tick recomputation. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-20-1513-REPORT-al-003-s03-scheduled-genome-runtime-cadence.md`, `tests/scheduler_genome_cadence.rs` |
| `AL-003-S04-AC01` | `AL-003-S04` | Genome copying, mutation, and repair are implemented as material-backed, deterministic Core behavior after scheduled runtime cadence is closed. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-20-1607-PLAN-al-003-s04-genome-copying-mutation-repair.md`, `outputs/worklogs/2026-07-20-1741-REPORT-al-003-s04-genome-copying-mutation-repair.md`, `tests/phase3c_genome_copying.rs`, `config/scenarios/genome/phase3c_genome_copying_conservative.toml` |
| `AL-003-S05-AC01` | `AL-003-S05` | Lineage event log and replay reconstruct founder Cells, Genome copying/mutation, division inheritance, and death from read-only Core evidence without behavior feedback. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-21-1105-PLAN-al-003-s05-lineage-event-log-and-replay.md`, `outputs/worklogs/2026-07-21-1159-REPORT-al-003-s05-lineage-event-log-and-replay.md`, `src/core/lineage.rs`, `tests/phase3d_lineage_replay.rs` |
| `AL-004-S01-AC01` | `AL-004-S01` | Observer vocabulary, source, and ownership matrix is closed before versioned projection expansion. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-20-2111-PLAN-al-004-s01-observer-contract-closure.md`, `outputs/worklogs/2026-07-21-1014-REPORT-al-004-s01-observer-contract-closure.md`, `src/observer/contract.rs`, `tests/observer_contract_closure.rs` |
| `AL-004-S02-AC01` | `AL-004-S02` | Versioned projection envelope is implemented as a Rust-only typed Observer contract with top-level projection/entity/source/completeness vocabulary, non-breaking `WorldFrameProjection v2` wrapping, and explicit storage/schema-generation deferrals. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-21-1248-PLAN-al-004-s02-versioned-projection-envelope.md`, `outputs/worklogs/2026-07-21-1312-REPORT-al-004-s02-versioned-projection-envelope.md`, `src/observer/projection_envelope.rs`, `src/observer/contract.rs`, `src/runner/projections.rs`, `tests/projection_envelope_contract.rs` |
| `AL-004-S03-AC01` | `AL-004-S03` | Classification registry/provenance baseline is closed for implemented early Observer classifiers while consumer payload projection and full provenance fields remain downstream work. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-22-1256-REPORT-al-004-s03-classification-registry-and-provenance.md`, `config/observer/classification-registry.toml`, `config/observer/cell-functional-role-classifier.toml`, `config/observer/behavior-profile-classifier.toml`, `config/observer/organism-archetype-classifier.toml`, `src/observer/config.rs`, `src/observer/classifiers.rs`, `src/observer/projection.rs`, `tests/phase2_observer_config.rs`, `tests/phase2_observer_role_classifier.rs`, `tests/phase2_observer_behavior_classifier.rs`, `tests/phase2_observer_archetypes.rs`, `tests/observer_contract_closure.rs`, `tests/projection_envelope_contract.rs` |
| `AL-004-S05-AC01` | `AL-004-S05` | Visual world projection payload is bounded, source-backed, and explicit about Cell draw data, lifecycle, energy, resource layer summaries, field summaries, and completeness without exposing mutable `WorldState`. | `outputs/worklogs/2026-07-22-1304-PLAN-al-004-s05-visual-balance-coverage-warning-projections.md`, `outputs/worklogs/2026-07-22-1331-REPORT-al-004-s05-visual-balance-coverage-warning-projections.md`, `docs/observer/projection-contract.md`, `docs/observer/observer-layer.md`, `src/core/snapshot.rs`, `src/core/summary.rs`, `src/observer/payloads.rs`, `tests/observer_projection_payloads.rs` |
| `AL-004-S05-AC02` | `AL-004-S05` | Classification projection preserves deterministic classification id, interval, mode, labels, confidence, status, evidence summary, registry/classifier versions, source references, data completeness, and limitation text. | `outputs/worklogs/2026-07-22-1304-PLAN-al-004-s05-visual-balance-coverage-warning-projections.md`, `outputs/worklogs/2026-07-22-1331-REPORT-al-004-s05-visual-balance-coverage-warning-projections.md`, `docs/observer/classification-contract.md`, `docs/observer/classification-registry.md`, `outputs/worklogs/2026-07-22-1256-REPORT-al-004-s03-classification-registry-and-provenance.md`, `src/observer/payloads.rs`, `tests/observer_projection_payloads.rs` |
| `AL-004-S05-AC03` | `AL-004-S05` | Coverage and warning projections use canonical Observer statuses/codes, preserve legacy warning disposition, and reject unknown warning codes instead of silently treating them as truth. | `outputs/worklogs/2026-07-22-1304-PLAN-al-004-s05-visual-balance-coverage-warning-projections.md`, `outputs/worklogs/2026-07-22-1331-REPORT-al-004-s05-visual-balance-coverage-warning-projections.md`, `docs/observer/mechanism-coverage.md`, `docs/observer/observer-layer.md`, `src/observer/contract.rs`, `src/observer/payloads.rs`, `tests/observer_projection_payloads.rs` |
| `AL-004-S05-AC04` | `AL-004-S05` | Balance finding projection preserves compared profiles, equal-requirements context, result, evidence metrics, dominance rate, source scenario/report, recommendations, reruns, confidence, and explicit incompleteness. | `outputs/worklogs/2026-07-22-1304-PLAN-al-004-s05-visual-balance-coverage-warning-projections.md`, `outputs/worklogs/2026-07-22-1331-REPORT-al-004-s05-visual-balance-coverage-warning-projections.md`, `docs/observer/behavior-profile-balance.md`, `src/observer/balance.rs`, `src/observer/payloads.rs`, `tests/phase2_observer_balance.rs`, `tests/observer_projection_payloads.rs` |
| `AL-004-S05-AC05` | `AL-004-S05` | Observer projection payloads remain read-only and cannot enter Core Tick, Genome Runtime, Feasibility, Process selection, `WorldState`, or stable state hash behavior. | `outputs/worklogs/2026-07-22-1304-PLAN-al-004-s05-visual-balance-coverage-warning-projections.md`, `outputs/worklogs/2026-07-22-1331-REPORT-al-004-s05-visual-balance-coverage-warning-projections.md`, `docs/mechanics/observer-projection.md`, `docs/observer/observer-layer.md`, `tests/observer_projection_payloads.rs`, `tests/observer_contract_closure.rs` |
| `AL-005-S01-AC01` | `AL-005-S01` | Run metadata and storage index are implemented as minimal file-backed SQLite rows with run reproducibility metadata, artifact references, explicit unavailable keyframes, reset-by-delete test behavior, and no Core behavior authority. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-21-2320-REPORT-al-005-s01-run-metadata-and-storage-index.md`, `src/storage/mod.rs`, `tests/storage_run_metadata.rs`, `tests/storage_sqlite_index.rs` |
| `AL-002-S16-AC01` | `AL-002-S16` | Runner-4 hardening is closed for remote viewer opt-in/CORS, stable HTTP errors, graceful shutdown state, reconnect latest-frame behavior, and status metadata without Core behavior changes. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-20-2134-PLAN-al-002-s16-runner-4-remote-viewer-acceptance-hardening.md`, `outputs/worklogs/2026-07-21-2221-REPORT-al-002-s16-runner-4-remote-viewer-acceptance-hardening.md`, `tests/runner_server_config.rs`, `tests/runner_http_info.rs`, `tests/runner_http_run_control.rs`, `tests/runner_ws_reconnect.rs`, `tests/runner_graceful_shutdown.rs` |
| `AL-007-S20-AC01` | `AL-007-S20` | Start residual visual gaps are closed or deliberately routed: disabled future workspaces remain visible with unavailable reasons, Start full-screen works, simulation rate and visualization FPS are visible, and unavailable projection fields remain explicit without Core behavior changes. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-22-0003-REPORT-al-007-s20-start-track-residual-visual-gap-disposition.md`, `ui/control-center/src/App.test.tsx`, `ui/control-center/src/components/MonitorWorkspace.test.tsx`, `ui/control-center/src/runner/apiClient.test.ts`, `ui/control-center/tests/e2e/monitor.spec.ts`, `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts` |
| `AL-007-S09-AC01` | `AL-007-S09` | Projection context shows source, version, completeness, run, and tick. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-22-1059-REPORT-al-007-s09-versioned-projections-keyframes-history.md`, `ui/control-center/src/projection/projectionContext.test.ts`, `ui/control-center/src/components/MonitorWorkspace.test.tsx` |
| `AL-007-S09-AC02` | `AL-007-S09` | Bounded client live history can freeze a frame while live state advances and then jump back to latest live. | `outputs/worklogs/2026-07-22-1059-REPORT-al-007-s09-versioned-projections-keyframes-history.md`, `ui/control-center/src/app/appState.test.ts`, `ui/control-center/src/components/MonitorWorkspace.test.tsx` |
| `AL-007-S09-AC03` | `AL-007-S09` | Unavailable historical ticks do not substitute a nearby frame. | `outputs/worklogs/2026-07-22-1059-REPORT-al-007-s09-versioned-projections-keyframes-history.md`, `ui/control-center/src/projection/projectionContext.test.ts`, `ui/control-center/src/app/appState.test.ts`, `ui/control-center/src/components/MonitorWorkspace.test.tsx` |
| `AL-007-S09-AC04` | `AL-007-S09` | Stale live context is explicit and read-only after disconnect, then restores on reconnect. | `outputs/worklogs/2026-07-22-1059-REPORT-al-007-s09-versioned-projections-keyframes-history.md`, `ui/control-center/src/app/appState.test.ts` |
| `AL-007-S09-AC05` | `AL-007-S09` | Start/Monitor acceptance remains usable after the new context UI is added. | `outputs/worklogs/2026-07-22-1059-REPORT-al-007-s09-versioned-projections-keyframes-history.md`, `ui/control-center/tests/e2e/monitor.spec.ts`, `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts` |

## Legacy Acceptance Matrix

These rows preserve the initial top-level `AL-###` delivery-control acceptance
mapping for backward compatibility. Do not expand or rewrite non-next rows until
their Plan ID becomes current or Candidate Next Work.

| Acceptance ID | Plan ID | Acceptance outcome | Required evidence |
| --- | --- | --- | --- |
| `AL-001-AC01` | `AL-001` | Every historical worklog has one ledger row. | `docs/delivery/worklog-ledger.md` |
| `AL-001-AC02` | `AL-001` | Delivery streams have explicit owners and status classes. | `docs/delivery/status.md` |
| `AL-001-AC03` | `AL-001` | Legacy labels remain backward-compatible aliases. | `docs/delivery/control.md` |
| `AL-002-AC01` | `AL-002` | Runner phases have source, test, and worklog evidence mapped. | Runner docs, `src/runner/`, `src/viewer_server/`, `tests/runner_*.rs` |
| `AL-003-AC01` | `AL-003` | Core Phase 2 and integrated world status is separated from later Genome work. | Phase 2 docs, `src/core/`, Phase 2 tests |
| `AL-004-AC01` | `AL-004` | Genome Phase 3A evidence is mapped without implying full Phase 3 completion. | Genetics docs, genome source/tests, Phase 3A worklogs |
| `AL-005-AC01` | `AL-005` | Observer contracts, analyzer evidence, and UI projection needs are reconciled. | Observer docs, observer source/tests, worklogs |
| `AL-006-AC01` | `AL-006` | Bootstrap foundation status is mapped to Runner startup constraints. | Bootstrap docs/source/tests/worklogs |
| `AL-007-AC01` | `AL-007` | UI-1D dependency pre-check respects Runner, Observer, and projection boundaries. | UI plan, Runner plan, Observer projection contract |
| `AL-007-AC02` | `AL-007` | Start demo path is coherent and does not infer unavailable projection data. | UI tests and e2e evidence |
| `AL-007-AC03` | `AL-007` | Screenshot export works within Start scope. | UI tests and manual/e2e evidence |
| `AL-007-AC04` | `AL-007` | Start acceptance hardening preserves UI-1C behavior and defers Debug/Research scope. | UI tests, build, e2e evidence |
| `AL-008-AC01` | `AL-008` | Stability tools and reachability evidence are indexed without entering the simulation hot path. | Early stability docs/tool/tests/worklogs |

## Evidence Rules

- Worklogs can support an evidence row, but cannot be the only reason to mark a
  delivery item `done-evidenced`.
- A completed report with verification commands but no delivery coverage mapping
  starts as `done-weak-evidence`.
- A stream with code and tests but unclear Canon or ownership starts as
  `Needs Review`.
