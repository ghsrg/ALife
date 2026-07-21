---
tags:
  - alife
  - worklog/plan
  - plan/AL-003-S05
  - area/genome
  - area/lineage
---

# PLAN: AL-003-S05 Lineage Event Log And Replay

## TDD_PLAN_PROPOSAL

Plan ID: `AL-003-S05`

Selected slice: Lineage Event Log And Replay

Current roadmap status: `planned`

Goal: add a deterministic, read-only lineage event log and replay summary so births, divisions, deaths, Genome copying, mutation, and inheritance can be reconstructed from committed Core evidence without letting lineage data affect simulation behavior.

Architecture: introduce a small World-owned append-only lineage log with typed IDs and compact event payloads. Core processes append lineage events at deterministic commit points; Observer/replay code reads those committed events to reconstruct parent/daughter/genome relationships. The log is evidence/projection input, not a behavior input, and must not enter Genome Runtime, Feasibility, Scheduler decisions, Process selection, or stable behavior hashing unless explicitly tested as observer-only.

Tech stack: Rust core modules, existing `TickExecutor`, `WorldState`, `CellStore`, `GenomeState`, `EventBuffer`, `StableStateHasher`, and integration tests under `tests/`.

## Source-Of-Truth Hierarchy

1. `docs/PRINCIPLES.md`
2. `docs/GLOSSARY.md`
3. `docs/biology/genome.md`
4. `docs/biology/division-partition.md`
5. `docs/genetics/inheritance.md`
6. `docs/genetics/heredity.md`
7. `docs/genetics/mutation.md`
8. `docs/mechanics/division-inheritance.md`
9. `docs/mechanics/tick-transaction.md`
10. `docs/mechanics/deterministic-execution.md`
11. `docs/mechanics/snapshot-replay.md`
12. `docs/observer/observer-layer.md`
13. `docs/observer/projection-contract.md`
14. `docs/implementation/implementation-phases.md`

Existing code and tests are implementation evidence only. Worklogs are historical/execution evidence, not source of truth.

## Files Read

- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/source-map.md`
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
- `src/core/events.rs`
- `src/core/ids.rs`
- `src/core/genome.rs`
- `src/core/world.rs`
- `src/core/tick.rs`
- `src/core/cell_store.rs`
- `src/core/stable_state_hash.rs`
- `src/core/summary.rs`
- `tests/phase3c_genome_copying.rs`
- `tests/phase2_division_smoke.rs`
- `outputs/worklogs/2026-07-20-1244-REPORT-al-003-s02-genome-runtime-contract-output-coverage.md`
- `outputs/worklogs/2026-07-20-1513-REPORT-al-003-s03-scheduled-genome-runtime-cadence.md`
- `outputs/worklogs/2026-07-20-1741-REPORT-al-003-s04-genome-copying-mutation-repair.md`
- `outputs/worklogs/2026-07-21-1014-REPORT-al-004-s01-observer-contract-closure.md`

## Domain Modeling Decisions

| Question | Decision |
| --- | --- |
| Stable identity | Add typed `LineageEventId` and use existing `CellId`, `GenomeId`, `Tick`. |
| State owner | `WorldState` owns the append-only `LineageEventLog`; Observer/replay borrows it read-only. |
| Mutation authority | Core commit paths append events; no code may read lineage events to decide behavior. |
| Hot path category | Event data, not persistent Cell state and not a derived index. |
| Replay category | Observer/read-model reconstruction over committed lineage events. |
| Storage | In-memory `Vec<LineageEvent>` for this slice; durable storage/SQLite remains `AL-005-S01`. |
| Projection | Provide replay summary/read model only; versioned projection envelope remains `AL-004-S02`. |

## Assumptions

- `LineageEventLog` should be separate from the current lightweight `EventBuffer`, because lineage replay needs parent/daughter/genome payloads that `Event { kind, cell_id }` does not carry.
- `LineageEventLog` may live in `src/core/lineage.rs` because events are committed by Core, but every consumer-facing interpretation remains observer-only.
- Initial lineage events should cover currently implemented mechanisms only: initial Genome assignment, Genome copy completion, mutation during copy, division inheritance, birth, and death.
- `Needs Review`: whether `CellDead` lineage events should include genome id at death when a dead cell still retains a Genome carrier. The plan includes this as evidence payload if present, but does not add Genome fragment/decomposition inheritance.
- `Needs Review`: whether initial cells without Genome should receive founder lineage events. This plan records founder cell events for all initial Cells and optional founder Genome data when present.

## Forbidden Scope

- Do not implement SQLite, Parquet, snapshots, migrations, or durable run metadata.
- Do not implement `AL-004-S02` projection envelope or UI projection compatibility.
- Do not implement selection/drift interpretation, species-like clusters, fitness, or evolution analytics.
- Do not implement HGT, recombination, Genome fragment integration, or damaged-carrier repair semantics.
- Do not feed lineage ids, generation, parentage, replay summaries, observer labels, or lineage metrics into Genome Runtime, Feasibility, Scheduler, Process selection, Cell lifecycle, or mutation probability.
- Do not change current division/copying mechanics except to append deterministic evidence after existing commits.

## BDD Agent Scenario Cards

### `AL-003-S05-AC01`: Founder And Genome Origin Events

Source links: `docs/biology/genome.md`, `docs/genetics/heredity.md`, `docs/observer/observer-layer.md`, `src/core/world.rs`, `src/core/genome_bootstrap.rs`.

Priority: P1

Intent: replay needs a stable origin for initial Cells and initial Genomes without inventing species or organism concepts.

Given a `WorldState` is created from a config with initial Cells and optional Genome templates  
When the committed initial world is inspected through the lineage log  
Then every initial Cell has a founder lineage event with `tick=0`, `cell_id`, no parent, and optional `genome_id`/`template_id` evidence when a Genome exists.

TDD obligation: add a failing test before adding the lineage module or World wiring.

Evidence IDs: `AL-003-S05-EV01`, `AL-003-S05-EV02`.

### `AL-003-S05-AC02`: Genome Copy And Mutation Events

Source links: `docs/biology/genome.md`, `docs/genetics/mutation.md`, `docs/biology/division-partition.md`, `src/core/world.rs`, `tests/phase3c_genome_copying.rs`.

Priority: P1

Intent: replay must know which Genome copy came from which parent Genome and whether outputs changed during deterministic copying.

Given Genome copying completes for a Cell  
When `execute_genome_copying` creates the copied World-owned Genome  
Then the lineage log records parent genome id, copied genome id, carrier material/amount/integrity, and a bounded deterministic mutation delta list for changed outputs.

TDD obligation: add RED tests for no-mutation and forced-mutation copy events.

Evidence IDs: `AL-003-S05-EV03`, `AL-003-S05-EV04`.

### `AL-003-S05-AC03`: Division Inheritance Events

Source links: `docs/biology/division-partition.md`, `docs/genetics/inheritance.md`, `docs/mechanics/division-inheritance.md`, `src/core/world.rs`, `src/core/tick.rs`.

Priority: P1

Intent: replay must reconstruct parent-to-daughter relationships, generation, inherited Genome ids, and partition outcome without reading mutable `CellStore` history.

Given a Genome-bearing or Genome-free Cell divides  
When division commits and daughter B is inserted  
Then the lineage log records parent cell id, daughter A id, daughter B id, parent genome id, daughter genome ids, split ratio, partition loss fraction, and commit tick in deterministic order.

TDD obligation: add RED test that current `EventBuffer` is insufficient because it cannot reconstruct both daughters and genome inheritance.

Evidence IDs: `AL-003-S05-EV05`, `AL-003-S05-EV06`.

### `AL-003-S05-AC04`: Death Events And Replay Summary

Source links: `docs/biology/lifecycle.md`, `docs/genetics/heredity.md`, `docs/observer/observer-layer.md`, `src/core/tick.rs`, `src/core/events.rs`.

Priority: P1

Intent: replay must account for lineage termination without treating death as selection command or behavior feedback.

Given a Cell enters `Dead` lifecycle state  
When the lifecycle commit emits `CellDead`  
Then the lineage log records death tick, cell id, and optional genome id at death; replay summary marks that Cell as not alive from that tick onward.

TDD obligation: add RED test for death lineage event and replay status.

Evidence IDs: `AL-003-S05-EV07`, `AL-003-S05-EV08`.

### `AL-003-S05-AC05`: Replay Determinism And Observer Boundary

Source links: `docs/mechanics/deterministic-execution.md`, `docs/mechanics/snapshot-replay.md`, `docs/observer/observer-layer.md`, `src/core/stable_state_hash.rs`, `src/core/genome.rs`.

Priority: P1

Intent: lineage replay must be deterministic and read-only; it must never become behavior input.

Given two runs use the same seed/config/binary  
When lineage replay summaries are built after identical Tick sequences  
Then lineage events and replay summaries match exactly, stable behavior hash remains unchanged by observer replay, and source scanning/compile boundaries prove Genome Runtime and Feasibility do not depend on lineage replay output.

TDD obligation: add RED/characterization tests for deterministic replay, no behavior-hash coupling, and no Genome Runtime dependency on lineage read models.

Evidence IDs: `AL-003-S05-EV09`, `AL-003-S05-EV10`.

### `AL-003-S05-AC06`: Delivery Closure And Handoff

Source links: `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `docs/delivery/worklog-ledger.md`.

Priority: P1

Intent: closure should unblock downstream storage, observer projection, and UI evolution planning while keeping unresolved features owned by later Plan IDs.

Given lineage tests pass  
When delivery closure runs  
Then roadmap/status/acceptance/ledger/report mark `AL-003-S05` according to evidence, and durable storage remains `AL-005-S01`, projection envelope remains `AL-004-S02`, and UI lineage views remain `AL-007-S16`.

TDD obligation: no production behavior; delivery-lint and closure-verification after implementation tests.

Evidence IDs: `AL-003-S05-EV11`, `AL-003-S05-EV12`.

## Proposed File Plan

Create:

- `src/core/lineage.rs`: typed lineage event ids, event payload enums/structs, append-only log, replay summary/read model.
- `tests/phase3d_lineage_replay.rs`: TDD tests for founder events, copy/mutation events, division inheritance, death replay, determinism, and observer boundary.

Modify:

- `src/core/ids.rs`: add `LineageEventId` typed id.
- `src/core/mod.rs`: expose `lineage`.
- `src/core/world.rs`: own `LineageEventLog`, append founder/copy/division events, expose read-only accessor.
- `src/core/tick.rs`: append death lineage events at lifecycle commit and ensure division event ordering remains deterministic.
- `src/core/stable_state_hash.rs`: keep lineage replay observer-only; if lineages are included anywhere, test and document why behavior hash remains stable. Preferred: do not include lineage events in `StableStateHasher::hash_world`.
- `tests/phase3c_genome_copying.rs`, `tests/phase2_division_smoke.rs`: only adjust if new lineage assertions fit existing coverage; prefer the dedicated `phase3d_lineage_replay.rs`.
- Delivery closure after execution: `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `docs/delivery/worklog-ledger.md`, `outputs/worklogs/index.md`.

Do not create a new Canon doc in this slice. If implementation discovers that lineage entity semantics are under-specified, stop and record `Needs Review` instead of inventing new behavior.

## Numbered TDD Tasks

### `AL-003-S05-T01`: RED for `AL-003-S05-AC01`

- Add `tests/phase3d_lineage_replay.rs`.
- Write failing test `initial_cells_emit_founder_lineage_events`.
- Expected RED: `alife::core::lineage` module or `WorldState::lineage_events()` does not exist.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test --test phase3d_lineage_replay initial_cells_emit_founder_lineage_events
```

- Capture result as `AL-003-S05-EV01`.

### `AL-003-S05-T02`: GREEN for `AL-003-S05-AC01`

- Add `LineageEventId` to `src/core/ids.rs`.
- Add `src/core/lineage.rs` with:
  - `LineageEventId`
  - `LineageEventKind`
  - `FounderCellLineage`
  - `GenomeOrigin`
  - `LineageEvent`
  - `LineageEventLog`
  - deterministic `push_*` methods and `iter_ordered()`
- Add `lineage` to `src/core/mod.rs`.
- Add `lineage_events: LineageEventLog` to `WorldState`.
- Append founder events during `WorldState::from_config` after initial Cells and Genomes are assigned.
- Run the same test and capture pass as `AL-003-S05-EV02`.

### `AL-003-S05-T03`: RED for `AL-003-S05-AC02`

- Add failing tests:
  - `genome_copy_completion_records_parent_and_child_genome`
  - `forced_mutation_records_bounded_output_delta`
- Expected RED: copied Genome exists, but no lineage event records parent/copy/mutation details.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test --test phase3d_lineage_replay genome_copy_completion_records_parent_and_child_genome forced_mutation_records_bounded_output_delta
```

- Capture result as `AL-003-S05-EV03`.

### `AL-003-S05-T04`: GREEN for `AL-003-S05-AC02`

- Extend `src/core/lineage.rs` with:
  - `GenomeCopyLineage`
  - `GenomeMutationDelta`
  - stable output ids and before/after values for changed outputs only.
- In `WorldState::execute_genome_copying`, after pushing the copied Genome and assigning `copied_genome_id`, append one Genome copy lineage event.
- Ensure no-mutation copies record an empty mutation delta list, not missing copy evidence.
- Run the same tests and capture pass as `AL-003-S05-EV04`.

### `AL-003-S05-T05`: RED for `AL-003-S05-AC03`

- Add failing tests:
  - `division_lineage_event_reconstructs_parent_and_daughters`
  - `division_lineage_event_records_genome_inheritance`
- Expected RED: current `EventBuffer` has `CellDivided` and `CellBorn`, but cannot reconstruct both daughters and Genome inheritance.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test --test phase3d_lineage_replay division_lineage_event_reconstructs_parent_and_daughters division_lineage_event_records_genome_inheritance
```

- Capture result as `AL-003-S05-EV05`.

### `AL-003-S05-T06`: GREEN for `AL-003-S05-AC03`

- Extend `src/core/lineage.rs` with `DivisionLineage`.
- In `WorldState::execute_division`, append one division lineage event after daughter B is inserted, typed resources are partitioned, and daughter ActionPlans are reset.
- Include:
  - `tick`
  - `parent_cell_id`
  - `daughter_a_cell_id`
  - `daughter_b_cell_id`
  - `parent_genome_id`
  - `daughter_a_genome_id`
  - `daughter_b_genome_id`
  - `split_ratio`
  - `partition_loss_fraction`
- Keep existing `EventBuffer` emissions in `TickExecutor` for backward compatibility.
- Run the same tests and capture pass as `AL-003-S05-EV06`.

### `AL-003-S05-T07`: RED for `AL-003-S05-AC04`

- Add failing tests:
  - `death_lineage_event_records_cell_and_genome_at_death`
  - `lineage_replay_marks_dead_cells_as_not_alive`
- Expected RED: `CellDead` exists in `EventBuffer`, but no lineage replay summary tracks death status.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test --test phase3d_lineage_replay death_lineage_event_records_cell_and_genome_at_death lineage_replay_marks_dead_cells_as_not_alive
```

- Capture result as `AL-003-S05-EV07`.

### `AL-003-S05-T08`: GREEN for `AL-003-S05-AC04`

- Extend `src/core/lineage.rs` with:
  - `CellDeathLineage`
  - `LineageReplaySummary`
  - `CellLineageRecord`
  - `GenomeLineageRecord`
  - `build_lineage_replay_summary(events: &LineageEventLog)`.
- In `TickExecutor::step`, when `next_state == LifecycleState::Dead`, append death lineage event with cell id and optional current Genome id.
- Replay should use event order, not current `CellStore`, to compute parentage/generation/alive status.
- Run the same tests and capture pass as `AL-003-S05-EV08`.

### `AL-003-S05-T09`: RED for `AL-003-S05-AC05`

- Add failing/characterization tests:
  - `lineage_replay_is_deterministic_for_same_seed_and_config`
  - `lineage_replay_does_not_change_stable_behavior_hash`
  - `genome_runtime_and_feasibility_do_not_depend_on_lineage_replay`
- Expected RED may be missing replay API or missing dependency guard.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test --test phase3d_lineage_replay lineage_replay_is_deterministic_for_same_seed_and_config lineage_replay_does_not_change_stable_behavior_hash genome_runtime_and_feasibility_do_not_depend_on_lineage_replay
```

- Capture result as `AL-003-S05-EV09`.

### `AL-003-S05-T10`: GREEN for `AL-003-S05-AC05`

- Ensure `LineageEvent` derives deterministic equality/debug traits and event order is append-only by commit order.
- Keep `StableStateHasher::hash_world` behavior-focused; do not hash lineage replay output.
- Implement source-scan or API-boundary guard in tests to ensure `src/core/genome.rs`, `validate_feasibility`, and process selection do not import/use lineage replay summaries.
- Run the same tests and capture pass as `AL-003-S05-EV10`.

### `AL-003-S05-T11`: REFACTOR and Regression

- Refactor only after all `phase3d_lineage_replay` tests pass.
- Keep `EventBuffer` unchanged unless adding optional references is unavoidable; existing `phase2_division_smoke` tests must remain valid.
- Run focused regression:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test --test phase3d_lineage_replay --test phase3c_genome_copying --test scheduler_genome_cadence --test phase3b_runtime_contract --test phase3a_action_plan --test phase2_division_smoke --test phase2_decomposition_smoke
```

- Capture result as `AL-003-S05-EV11`.

### `AL-003-S05-T12`: Delivery Closure

- Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-003-s05-lineage-event-log-and-replay.md`.
- Update:
  - `docs/delivery/roadmap.md`
  - `docs/delivery/status.md`
  - `docs/delivery/acceptance.md`
  - `docs/delivery/worklog-ledger.md`
  - `outputs/worklogs/index.md`
- Run deterministic delivery-lint and closure-verification.
- Run:

```powershell
git diff --check
```

- Attempt broader verification if practical:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test
```

- If full `cargo test` times out, record timeout explicitly and rely only on targeted evidence.
- Capture closure as `AL-003-S05-EV12`.

## Verification Commands

Primary:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test --test phase3d_lineage_replay
```

Focused regression:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al003s05'; cargo test --test phase3d_lineage_replay --test phase3c_genome_copying --test scheduler_genome_cadence --test phase3b_runtime_contract --test phase3a_action_plan --test phase2_division_smoke --test phase2_decomposition_smoke
```

Delivery:

```powershell
git diff --check
```

## Open Questions

1. `Needs Review`: should death lineage events include current Genome id if the dead Cell still has a carrier, or should death only reference Cell id and let replay resolve last known Genome assignment?
2. `Needs Review`: should founder events be emitted for Genome-free initial Cells? This plan says yes because replay needs complete Cell origin evidence.
3. `Needs Review`: should lineage events be excluded from `StableStateHasher` permanently as observer-only evidence, or should a separate `LineageStateHasher` be introduced for replay equality? This plan prefers separate replay equality and no behavior-hash coupling.
4. `Needs Review`: should mutation deltas record all output before/after values or only changed outputs? This plan records changed outputs only and validates copy parent/child identity separately.

## Status Update Recommendation

- Set `docs/delivery/status.md` `Current Focus` to `AL-003-S05` with status `planned`.
- Mark `AL-003-S05` in `Next` as `planned-ready`.
- Keep `docs/delivery/roadmap.md` status for `AL-003-S05` as `planned` until execution/closure.
- Keep `AL-004-S02` and `AL-002-S16` in `Next`.

## Approval Gate

Reply `OK EXECUTE AL-003-S05` to authorize execution of this TDD plan.

Reply `CHANGE AL-003-S05` with corrections to revise the plan.

Generic `OK` approves the plan content only. It does not authorize execution.
