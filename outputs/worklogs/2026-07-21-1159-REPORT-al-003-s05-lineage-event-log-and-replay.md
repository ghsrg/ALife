---
tags:
  - alife
  - worklog/report
  - report/AL-003-S05
  - area/genome
  - area/lineage
---

# REPORT: AL-003-S05 Lineage Event Log And Replay

Plan ID: `AL-003-S05`

Outcome: `PASS`

Selected slice: Lineage Event Log And Replay

Purpose: implement a deterministic, read-only lineage event log and replay summary for founder Cells, Genome copying/mutation, division inheritance, and Cell death without making lineage data a behavior input.

Worklogs are evidence, not source of truth.

## Source Documents Read

- `docs/PRINCIPLES.md`
- `docs/GLOSSARY.md`
- `docs/biology/genome.md`
- `docs/biology/division-partition.md`
- `docs/genetics/inheritance.md`
- `docs/genetics/heredity.md`
- `docs/genetics/mutation.md`
- `docs/mechanics/division-inheritance.md`
- `docs/mechanics/tick-transaction.md`
- `docs/mechanics/deterministic-execution.md`
- `docs/mechanics/snapshot-replay.md`
- `docs/observer/observer-layer.md`
- `docs/observer/projection-contract.md`
- `docs/implementation/implementation-phases.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `outputs/worklogs/2026-07-21-1105-PLAN-al-003-s05-lineage-event-log-and-replay.md`

## Changed Files

- `src/core/lineage.rs`: added typed lineage event payloads, append-only log, replay summary records, and deterministic replay builder.
- `src/core/ids.rs`: added `LineageEventId`.
- `src/core/mod.rs`: exposed `core::lineage`.
- `src/core/world.rs`: World-owned `LineageEventLog`; founder, Genome copy/mutation, and division lineage emission.
- `src/core/tick.rs`: Cell death lineage emission beside existing `CellDead` event.
- `tests/phase3d_lineage_replay.rs`: TDD coverage for founder, copy/mutation, division inheritance, death replay, deterministic replay, stable hash boundary, and no runtime/feasibility replay dependency.
- Delivery artifacts: roadmap, status, acceptance matrix, worklog ledger, worklog index.

## Coverage Matrix

| Scenario ID | Task IDs | Evidence IDs | Requirement | Evidence | Status |
| --- | --- | --- | --- | --- | --- |
| `AL-003-S05-AC01` | `AL-003-S05-T01`, `AL-003-S05-T02` | `AL-003-S05-EV01`, `AL-003-S05-EV02` | Initial Cells emit founder lineage events with optional Genome/template origin evidence. | RED compile failure for missing `core::lineage`; GREEN `initial_cells_emit_founder_lineage_events`. | covered |
| `AL-003-S05-AC02` | `AL-003-S05-T03`, `AL-003-S05-T04` | `AL-003-S05-EV03`, `AL-003-S05-EV04` | Genome copy completion records parent/copy identity, carrier evidence, and mutation deltas. | `genome_copy_completion_records_parent_and_child_genome`, `forced_mutation_records_bounded_output_delta`. | covered |
| `AL-003-S05-AC03` | `AL-003-S05-T05`, `AL-003-S05-T06` | `AL-003-S05-EV05`, `AL-003-S05-EV06` | Division lineage reconstructs parent/daughters and Genome inheritance. | `division_lineage_event_reconstructs_parent_and_daughters`, `division_lineage_event_records_genome_inheritance`. | covered |
| `AL-003-S05-AC04` | `AL-003-S05-T07`, `AL-003-S05-T08` | `AL-003-S05-EV07`, `AL-003-S05-EV08` | Death events mark replay records as not alive. | `death_lineage_event_records_cell_and_genome_at_death`, `lineage_replay_marks_dead_cells_as_not_alive`. | covered |
| `AL-003-S05-AC05` | `AL-003-S05-T09`, `AL-003-S05-T10` | `AL-003-S05-EV09`, `AL-003-S05-EV10` | Replay is deterministic, observer-only, and does not alter stable behavior hash. | `lineage_replay_is_deterministic_for_same_seed_and_config`, `lineage_replay_does_not_change_stable_behavior_hash`, source boundary guard. | covered |
| `AL-003-S05-AC06` | `AL-003-S05-T11`, `AL-003-S05-T12` | `AL-003-S05-EV11`, `AL-003-S05-EV12` | Closure evidence updates delivery-control artifacts and keeps downstream storage/projection/UI scope deferred. | Focused regression suite, delivery artifact updates, `git diff --check`. | covered |

## Verification

RED evidence:

- `$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test --test phase3d_lineage_replay initial_cells_emit_founder_lineage_events`
  - Result: failed as expected with missing `alife::core::lineage` and `WorldState::lineage_events()`.
- `$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test --test phase3d_lineage_replay`
  - Result after adding later tests: failed as expected with missing replay and payload accessors.

GREEN evidence:

- `$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test --test phase3d_lineage_replay`
  - Result: 10 passed, 0 failed.
- `$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test --test phase3d_lineage_replay --test phase3c_genome_copying --test scheduler_genome_cadence --test phase3b_runtime_contract --test phase3a_action_plan --test phase2_division_smoke --test phase2_decomposition_smoke`
  - Result: 41 passed, 0 failed.

Full `cargo test` was intentionally not run in this closure pass to avoid unnecessary target growth after the user explicitly warned about disk usage. Verification reused `target\codex-al003s05`.

## Status Update Recommendation

- Set `AL-003-S05` to `done` / `high`.
- Set parent `AL-003` to `done` / `high` because `AL-003-S01` through `AL-003-S05` now have closure evidence.
- Clear `Current Focus` until the next Plan ID is selected.
- Move `AL-004-S02` to first `Next`; it now has stronger input from closed lineage evidence.
- Keep durable storage/indexing in `AL-005-S01`, versioned projection envelope in `AL-004-S02`, and UI lineage views in `AL-007-S16`.

## Follow-Up Scope

- `Needs Review`: death lineage currently records optional current Genome id at death. If future Canon changes carrier decomposition semantics, replay payload may need a versioned extension.
- `Needs Review`: lineage replay is in-memory only. Durable storage remains `AL-005-S01`.
- `Needs Review`: projection envelope for lineage consumers remains `AL-004-S02`; UI lineage visualization remains `AL-007-S16`.
