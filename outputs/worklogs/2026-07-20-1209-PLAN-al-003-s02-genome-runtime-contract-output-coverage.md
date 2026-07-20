# AL-003-S02 Genome Runtime Contract And Registered Output Coverage Plan

Plan ID: `AL-003-S02`
Status: `planned`
Date: 2026-07-20

## Goal

Define and test the Genome Runtime boundary before scheduler/cadence work:
normalized local inputs, registered output coverage, priority normalization,
capability/material masks, minimal runtime trace, and explicit unsupported or
deferred output dispositions.

## Source Hierarchy

1. `docs/PRINCIPLES.md`
2. Canon:
   - `docs/genetics/genome-runtime.md`
   - `docs/genetics/regulatory-interface.md`
   - `docs/genetics/regulatory-network.md`
   - `docs/biology/action-process-registry.md`
   - `docs/biology/process-capabilities.md`
   - `docs/biology/feasibility.md`
   - `docs/world/tick-semantics.md`
3. Implementation plan: `docs/implementation/implementation-phases.md`
4. Existing implementation evidence:
   - `src/core/genome.rs`
   - `src/core/action_plan.rs`
   - `src/core/process.rs`
   - `src/core/world.rs`
   - `src/core/tick.rs`
5. Existing tests:
   - `tests/phase3a_genome_bootstrap.rs`
   - `tests/phase3a_genome_config.rs`
   - `tests/phase3a_action_plan.rs`
   - `tests/phase3a_tick_integration.rs`
   - `tests/scheduler_genome_cadence.rs`
6. Historical evidence only:
   - `outputs/worklogs/2026-07-13-2351-PLAN-phase-3A-genome-bootstrap.md`
   - `outputs/worklogs/2026-07-14-0955-REPORT-phase-3A-genome-bootstrap.md`

## Current Evidence Summary

- Phase 3A is implemented as a narrow Genome bootstrap slice with constant
  priority outputs and Feasibility remaining authoritative.
- Current `GenomeOutputId` supports six outputs:
  `resource_uptake_priority`, `energy_conversion_priority`,
  `material_synthesis_priority`, `repair_priority`, `movement_priority`, and
  `division_preparation_priority`.
- Canon `biology/action-process-registry.md` lists additional `status: now`
  outputs that current implementation does not support yet:
  `resource_export_priority`, `signal_emit_priority`,
  `genome_copying_priority`, `division_partition_priority`, `dormancy_bias`,
  and `internal_rebalance_priority`.
- `Needs Review`: scheduler cadence tests already exist, but `AL-003-S03`
  remains the owner for scheduled runtime cadence closure. `AL-003-S02` must
  preserve those regressions without claiming cadence completion.

## Decision

Do not silently enable every canon `status: now` output in `AL-003-S02`.

This slice must first create an explicit output disposition model and tests:

| Output group | Disposition |
| --- | --- |
| Existing Phase 3A six outputs | `enabled_now` if already backed by process execution and Feasibility. |
| Additional canon `status: now` outputs | `deferred` or `unsupported_until_registry_change` until each has defensible execution, trace, and Feasibility coverage. |
| Future registry entries | rejected by parser/runtime and marked non-executable. |

## BDD Agent Scenario Cards

### `AL-003-S02-AC01` Runtime Boundary

Given a Cell has Genome state and committed local state, when Genome Runtime is
evaluated, then it reads only normalized local snapshot inputs and emits bounded
priorities, not world mutations.

TDD obligation: add failing tests for normalized local input shape and absence
of forbidden global/observer inputs.

### `AL-003-S02-AC02` Registered Output Coverage

Given `docs/biology/action-process-registry.md`, when Genome outputs are parsed
or evaluated, then every accepted output maps to a current registered process,
and unsupported or future outputs are explicitly rejected or documented as
deferred.

TDD obligation: add failing coverage tests for every canon output name and its
disposition.

### `AL-003-S02-AC03` Capability Masking

Given a Cell lacks required material capability or sensing basis, when runtime
builds priorities, then masked inputs/outputs cannot create executable
capability and Feasibility remains final authority.

TDD obligation: add failing tests where high priority cannot bypass missing
capability/material basis.

### `AL-003-S02-AC04` Runtime Trace

Given runtime evaluation happens, when diagnostics are enabled, then trace
records tick, cell id, normalized inputs, output priorities, action plan, and
feasibility result without becoming behavior input.

TDD obligation: add failing tests for trace presence and read-only behavior.

### `AL-003-S02-AC05` Phase Boundary

Given `AL-003-S03` owns cadence and `AL-003-S04` owns copying/mutation, when
`AL-003-S02` is executed, then it must not implement mutation, inheritance,
lineage replay, or new scheduler semantics.

TDD obligation: preserve existing scheduler genome cadence tests as regression
only and do not mark `AL-003-S03` complete.

## TDD Task Plan

### `AL-003-S02-T01` RED: Registry Coverage And Disposition Tests

- Add tests that enumerate all canon output names from the current registry
  contract and assert each has an explicit runtime disposition.
- Expected initial failure: additional canon outputs have no explicit
  implementation disposition.
- Evidence: `AL-003-S02-EV01`.

### `AL-003-S02-T02` GREEN: Output Disposition Model

- Add a small internal model for Genome output disposition:
  `enabled_now`, `deferred`, or `unsupported_until_registry_change`.
- Keep existing six Phase 3A outputs enabled only where Process execution and
  Feasibility already support them.
- Do not silently make parser/runtime accept newly listed outputs as executable.
- Evidence: `AL-003-S02-EV02`.

### `AL-003-S02-T03` RED/GREEN: Normalized Local Input Snapshot

- Add a minimal immutable runtime input snapshot type.
- Include only local, normalized fields supported by current state.
- Exclude forbidden inputs by API shape: global population, species id, observer
  metrics, organism id, target coordinates, neighbor genome.
- Evidence: `AL-003-S02-EV03`.

### `AL-003-S02-T04` RED/GREEN: Capability And Material Masks

- Add tests proving high priority does not enable unavailable process
  capability.
- Implement minimal masks so missing material/capability makes the candidate
  unavailable or leaves Feasibility rejection authoritative.
- Evidence: `AL-003-S02-EV04`.

### `AL-003-S02-T05` RED/GREEN: ActionPlan Output Coverage

- Ensure `ActionPlan` is built from registered enabled outputs only.
- Add explicit tests for deferred outputs such as `resource_export_priority`,
  `signal_emit_priority`, `genome_copying_priority`,
  `division_partition_priority`, `dormancy_bias`, and
  `internal_rebalance_priority`.
- Evidence: `AL-003-S02-EV05`.

### `AL-003-S02-T06` RED/GREEN: Minimal Runtime Trace

- Add runtime trace fields needed for Observer/UI later:
  tick, cell id, normalized inputs, regulatory outputs, action plan, and
  feasibility result summary.
- Keep trace read-only and sampled/optional if needed for performance.
- Evidence: `AL-003-S02-EV06`.

### `AL-003-S02-T07` REFACTOR: Remove Phase 3A Naming From Generic Runtime Path

- Rename or isolate internal `phase3a` concepts only where they now represent
  general runtime contract behavior.
- Preserve existing Phase 3A behavior and tests.
- Evidence: `AL-003-S02-EV07`.

### `AL-003-S02-T08` Docs, Status, Report, And Candidate Next Work

- Update `docs/delivery/roadmap.md` and `docs/delivery/status.md` after
  verified implementation.
- Keep `AL-003-S03` as planned unless separately verified.
- Review `Candidate Next Work` in the same pass.
- Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-003-s02-genome-runtime-contract-output-coverage.md`.
- Evidence: `AL-003-S02-EV08`.

## Verification Commands

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --test phase3a_genome_bootstrap
cargo test --test phase3a_genome_config
cargo test --test phase3a_action_plan
cargo test --test phase3a_tick_integration
cargo test --test phase2_process_registry
cargo test --test phase2_process_smoke
cargo test --test scheduler_genome_cadence
```

## Forbidden Scope

- No mutation implementation.
- No inheritance implementation.
- No lineage replay implementation.
- No organism-level inputs.
- No observer metrics as behavior input.
- No hardcoded species, cell roles, or biological shortcuts.
- No scheduler cadence closure claim for `AL-003-S03`.
- No silent enabling of additional canon outputs without explicit disposition and
  tests.

## Approval Gate

Reply `OK EXECUTE AL-003-S02` to authorize execution of this TDD plan.

