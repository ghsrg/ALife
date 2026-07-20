---
tags:
  - alife
  - worklog/plan
  - delivery/plan
  - plan/al-003-s03
---

# PLAN: AL-003-S03 Scheduled Genome Runtime Cadence

Plan ID: `AL-003-S03`
Status: `planned`
Confidence: `medium`
Route: `delivery-control` -> `roadmap-control`
Request type: `TDD_PLAN_REQUEST`

## Selected Slice

`AL-003-S03` closes scheduled Genome Runtime cadence on top of the closed
`AL-003-S02` runtime contract.

The slice must prove that Genome Runtime refreshes committed `ActionPlan` state
on deterministic cadence, not every Tick by default. It must preserve the cached
plan between refreshes, expose cadence/refresh diagnostics, and keep Feasibility
as the only execution authority.

## Files Read

- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `docs/implementation/implementation-phases.md`
- `docs/engine/scheduler.md`
- `docs/engine/performance.md`
- `docs/genetics/genome-runtime.md`
- `docs/genetics/regulatory-interface.md`
- `docs/world/tick-semantics.md`
- `docs/mechanics/genome-action-pipeline.md`
- `src/core/config.rs`
- `src/core/world.rs`
- `src/core/tick.rs`
- `src/core/cell_store.rs`
- `src/core/summary.rs`
- `tests/scheduler_genome_cadence.rs`
- `tests/phase3b_runtime_contract.rs`
- `tests/phase3a_action_plan.rs`
- `tests/phase3a_tick_integration.rs`

## Source-Of-Truth Hierarchy

1. `docs/implementation/implementation-phases.md` Phase 3 gate.
2. `docs/engine/scheduler.md` Genome Runtime cadence rules.
3. `docs/engine/performance.md` scheduled Genome Runtime requirement.
4. `docs/genetics/genome-runtime.md` and
   `docs/genetics/regulatory-interface.md` runtime boundary.
5. `docs/world/tick-semantics.md` and
   `docs/mechanics/genome-action-pipeline.md` Tick/ActionPlan/Feasibility
   ordering.
6. `docs/delivery/roadmap.md` slice scope and dependencies.
7. Worklogs only as historical evidence, not canonical requirements.

## Evidence Summary

Existing implementation evidence already includes:

- scheduler config fields for Genome Runtime base cadence and per-layer cost;
- template-level runtime interval and regulatory depth effective cadence;
- deterministic initial Genome decision offsets;
- cached `ActionPlan` state in `CellStore`;
- `next_genome_decision_due_tick` state;
- refresh-count metric in `TickSummary`;
- minimal `GenomeRuntimeTrace` diagnostics from `AL-003-S02`;
- `tests/scheduler_genome_cadence.rs` covering cached plan reuse, effective
  cadence, and deterministic staggering.

Needs Review:

- The current implementation may already satisfy most of `AL-003-S03`; this
  plan should treat the slice as closure/hardening unless a RED test exposes a
  real gap.
- `docs/engine/performance.md` mentions a runtime-state dirty flag. The current
  code uses cached `ActionPlan` plus due tick state instead. Do not add a dirty
  flag unless a source-of-truth behavior requires it for this slice.

## BDD Agent Scenario Cards

### AL-003-S03-AC01: Cached ActionPlan Between Refreshes

Sources: `docs/engine/scheduler.md`, `docs/engine/performance.md`,
`docs/mechanics/genome-action-pipeline.md`, `docs/delivery/roadmap.md`.

Given a living Cell with a Genome and effective Genome Runtime cadence greater
than one Tick,
when simulation advances before `next_genome_decision_due_tick`,
then the last committed `ActionPlan` is reused, process attempts may still run
through Feasibility, `genome_decision_refresh_count` remains zero, and no hidden
every-Tick Genome Runtime trace is emitted.

TDD obligation: add or strengthen a characterization test that proves no refresh
count and no runtime trace before the due Tick while the cached process order is
still used.

### AL-003-S03-AC02: Due Tick Refresh And Next Due Tick

Sources: `docs/engine/scheduler.md`, `docs/genetics/genome-runtime.md`,
`docs/world/tick-semantics.md`.

Given a Cell whose Genome Runtime due Tick has arrived,
when the Tick executes,
then the runtime refreshes the committed `ActionPlan` once, records a minimal
runtime trace, increments `genome_decision_refresh_count`, and schedules the
next due Tick by the effective cadence.

TDD obligation: add a focused test for refresh-on-due, trace-on-due, and no
second refresh until the next due Tick.

### AL-003-S03-AC03: Deterministic Cadence Configuration

Sources: `docs/engine/scheduler.md`, `docs/engine/performance.md`,
`docs/implementation/implementation-phases.md`.

Given scheduler defaults, Genome template runtime interval, and regulatory
depth,
when effective cadence is resolved,
then the value is deterministic, never zero, stable for same config, and visible
through configuration/diagnostics evidence.

TDD obligation: keep or extend config tests for template override, regulatory
depth cost, validation rejection of zero cadence, and same-config stability.

### AL-003-S03-AC04: Deterministic Staggering

Sources: `docs/engine/scheduler.md`, `docs/implementation/implementation-phases.md`.

Given multiple Genome-bearing Cells with the same seed and cadence,
when initial decision offsets are computed,
then offsets are deterministic, bounded by cadence, and do not collapse every
cell onto the same refresh Tick unless the cadence makes that unavoidable.

TDD obligation: keep or extend the existing staggering test and verify same
seed/config replay.

### AL-003-S03-AC05: No Scope Leakage Into Mutation Or Authority

Sources: `docs/genetics/regulatory-interface.md`,
`docs/mechanics/genome-action-pipeline.md`, `docs/delivery/roadmap.md`.

Given a Genome Runtime refresh,
when outputs become process priorities,
then they remain bounded `ActionPlan` candidates only; Feasibility remains
authoritative, and this slice does not implement Genome copying, mutation,
repair, lineage replay, Observer projection contracts, or UI behavior.

TDD obligation: preserve Phase 3A/3B and Phase 2 process feasibility regression
tests.

## TDD Tasks

AL-003-S03-T01: RED for `AL-003-S03-AC01`

- Add/strengthen `tests/scheduler_genome_cadence.rs` to assert that before the
  due Tick:
  - `genome_decision_refresh_count == 0`;
  - `diagnostics.genome_runtime_traces` is empty;
  - cached `ActionPlan` process ordering still drives process attempts.
- Run `cargo test --test scheduler_genome_cadence` and capture expected failure
  as `AL-003-S03-EV01` if current coverage is insufficient.

AL-003-S03-T02: GREEN for `AL-003-S03-AC01`

- If the RED test fails, make the minimal scheduler/cache fix in `src/core/tick.rs`
  or `src/core/cell_store.rs`.
- Do not change process feasibility or Genome output semantics.
- Capture pass as `AL-003-S03-EV02`.

AL-003-S03-T03: RED for `AL-003-S03-AC02`

- Add a due-Tick test proving exactly one refresh at the due Tick, runtime trace
  emission only on refresh, and no immediate repeated refresh after the due Tick.
- Prefer direct observable summary/diagnostics assertions over private-state
  coupling.
- Capture expected failure or characterization result as `AL-003-S03-EV03`.

AL-003-S03-T04: GREEN for `AL-003-S03-AC02`

- If needed, fix due Tick progression so the next due Tick is advanced by the
  effective cadence and diagnostics are emitted only for real refreshes.
- Capture pass as `AL-003-S03-EV04`.

AL-003-S03-T05: RED/GREEN for `AL-003-S03-AC03`

- Add missing tests for zero cadence rejection or clamping behavior, template
  override, and regulatory-depth cadence calculation if existing tests do not
  fully cover them.
- Keep validation behavior consistent with existing config parser contracts.
- Capture result as `AL-003-S03-EV05`.

AL-003-S03-T06: RED/GREEN for `AL-003-S03-AC04`

- Strengthen deterministic staggering/replay evidence if current tests are too
  narrow.
- Verify offsets are stable for same seed/config and bounded by cadence.
- Capture result as `AL-003-S03-EV06`.

AL-003-S03-T07: Regression Fence for `AL-003-S03-AC05`

- Run existing Genome Runtime and process feasibility regression tests:
  - `cargo test --test phase3b_runtime_contract`
  - `cargo test --test phase3a_action_plan`
  - `cargo test --test phase3a_tick_integration`
  - `cargo test --test phase2_process_registry`
  - `cargo test --test phase2_process_smoke`
- Capture result as `AL-003-S03-EV07`.

AL-003-S03-T08: REFACTOR/Docs/Closure

- Run formatting and lint:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
- Update only approved delivery-control artifacts after verification:
  - `docs/delivery/roadmap.md`
  - `docs/delivery/status.md`
  - `docs/delivery/acceptance.md`
  - `docs/delivery/worklog-ledger.md`
  - `outputs/worklogs/index.md`
- Review `Candidate Next Work` during any roadmap change.
- Create closure report:
  - `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-003-s03-scheduled-genome-runtime-cadence.md`
- Capture final evidence as `AL-003-S03-EV08`.

## Verification Commands

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --test scheduler_genome_cadence
cargo test --test phase3b_runtime_contract
cargo test --test phase3a_action_plan
cargo test --test phase3a_tick_integration
cargo test --test phase2_process_registry
cargo test --test phase2_process_smoke
git diff --check
```

## Forbidden Scope

- Do not implement Genome copying, mutation, repair, or lineage replay.
- Do not add Observer projection contracts or UI behavior.
- Do not make Genome Runtime outputs mutate world state directly.
- Do not bypass process Feasibility.
- Do not treat worklogs as canonical requirements.
- Do not mark `AL-003-S03` done without closure verification.

## Open Questions

- Needs Review: whether the current cached `ActionPlan` plus due Tick state is
  accepted as the planned runtime-state mechanism for `AL-003-S03`, leaving any
  richer dirty-flag model out of scope unless a failing requirement demands it.

## Approval Gate

Reply `OK EXECUTE AL-003-S03` to authorize execution of this TDD plan.

Reply `CHANGE AL-003-S03` with corrections to revise the plan.
