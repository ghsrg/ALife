---
tags:
  - alife
  - worklog/plan
  - delivery/plan
  - plan/al-003-s04
---

# PLAN: AL-003-S04 Genome Copying, Mutation, And Repair

Plan ID: `AL-003-S04`
Status: `planned`
Confidence: `medium`
Route: `delivery-control` -> `roadmap-control` + `rust-domain-modeling`
Request type: `TDD_PLAN_REQUEST`

## Selected Slice

`AL-003-S04` closes the first executable Genome copying, mutation, and repair
slice after scheduled Genome Runtime cadence.

The slice must make Genome copying a registered, feasible, material-backed
process. Mutation may occur only through explicit copying/repair mechanisms, must
be deterministic for the same seed/config/state, must stay bounded and
validated, and must never create direct world actions, Resources, Materials, or
species-like shortcuts.

Lineage event log and replay remain `AL-003-S05`.

## Files Read

- `AGENTS.MD`
- `docs/PRINCIPLES.md`
- `docs/GLOSSARY.md`
- `docs/ROADMAP.md`
- `docs/INDEX.md`
- `docs/mechanics/INDEX.md`
- `docs/mechanics/genome-action-pipeline.md`
- `docs/mechanics/action-feasibility.md`
- `docs/mechanics/tick-transaction.md`
- `docs/mechanics/deterministic-execution.md`
- `docs/mechanics/config-to-runtime.md`
- `docs/mechanics/division-inheritance.md`
- `docs/mechanics/long-running-process.md`
- `docs/mechanics/matter-accounting.md`
- `docs/mechanics/capacity-accounting.md`
- `docs/biology/genome.md`
- `docs/biology/action-process-registry.md`
- `docs/biology/process-capabilities.md`
- `docs/biology/process-progress.md`
- `docs/biology/feasibility.md`
- `docs/biology/lifecycle.md`
- `docs/biology/division-partition.md`
- `docs/genetics/genome-representation.md`
- `docs/genetics/genome-runtime.md`
- `docs/genetics/regulatory-interface.md`
- `docs/genetics/regulatory-network.md`
- `docs/genetics/mutation.md`
- `docs/genetics/inheritance.md`
- `docs/implementation/implementation-phases.md`
- `docs/engine/scheduler.md`
- `docs/engine/performance.md`
- `docs/decisions/ADR-0001-tech-stack.md`
- `docs/delivery/source-map.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `outputs/worklogs/2026-07-20-1513-REPORT-al-003-s03-scheduled-genome-runtime-cadence.md`
- `src/core/genome.rs`
- `src/core/process.rs`
- `src/core/action_plan.rs`
- `src/core/cell_store.rs`
- `src/core/world.rs`
- `src/core/tick.rs`
- `src/core/config.rs`
- `src/core/stable_state_hash.rs`
- `src/runner/config_parser.rs`
- `tests/phase3b_runtime_contract.rs`
- `tests/phase3a_action_plan.rs`
- `tests/phase3a_tick_integration.rs`
- `tests/phase2_process_registry.rs`
- `tests/phase2_division_smoke.rs`
- `tests/phase2_process_smoke.rs`

## Source-Of-Truth Hierarchy

1. `docs/PRINCIPLES.md`
2. Canon: `docs/biology/genome.md`,
   `docs/biology/action-process-registry.md`,
   `docs/biology/division-partition.md`,
   `docs/genetics/mutation.md`,
   `docs/genetics/inheritance.md`
3. Mechanics cards: Genome -> ActionPlan, ActionPlan -> Feasibility,
   Division -> Partition -> Inheritance, Long-Running Process, Matter and
   Capacity Accounting.
4. Accepted ADR: `docs/decisions/ADR-0001-tech-stack.md`
5. Implementation plan: `docs/implementation/implementation-phases.md`
6. Delivery roadmap/status/acceptance for Plan ID scope.
7. Code/tests as implementation evidence only.
8. Worklogs as historical evidence only, not source of truth.

## Current Evidence Summary

Existing implementation evidence:

- `GenomeState` has `GenomeId`, `template_id`, carrier state, and outputs.
- `WorldState` owns genomes in a central vector and Cells store `GenomeId`.
- `CellStore` tracks Genome carrier capacity contribution through
  `genome_carrier_amounts`.
- `genome_copying_priority` is intentionally deferred in
  `tests/phase3b_runtime_contract.rs` and `src/core/genome.rs`.
- `ProcessId` does not yet include `GenomeCopying`.
- `ActionPlan` baseline does not include Genome copying.
- Division currently does not require completed Genome copying and does not
  assign a new copied Genome to the inserted daughter.
- No mutation operators or mutation seed domain are implemented.

Needs Review:

- Exact default costs/rates for Genome copying and mutation are not specified in
  Canon. The implementation should introduce conservative explicit config with
  safe defaults and tests, then leave numeric balancing to later sweep/calibration
  work.
- `repair` in this slice must mean Genome carrier repair/copy-error handling,
  not reopening Phase 2 boundary repair unless a test proves integration is
  required.

## Rust Domain Modeling Decision

Recommended model for this slice:

- Keep `GenomeId` as stable identity.
- Keep Genome data World-owned, not stored by value in Cells.
- Add hot Cell-side copy/repair progress as SoA fields in `CellStore`.
- Add persistent copied Genome state to World-owned genome storage only at
  deterministic commit boundaries.
- Use explicit typed value objects or validated constructors for copy progress,
  mutation chance, carrier amount/integrity, costs, and bounded output values.
- Use deterministic seed domains for copy mutation; do not rely on unordered
  iteration or ambient RNG.

Rejected alternatives:

| Approach | Pros | Cons | Decision |
| --- | --- | --- | --- |
| Clone parent `GenomeState` directly during division | Smallest code diff; easy tests | Violates no-hidden-copy; bypasses physical carrier/cost/progress; hides mutation boundary | Reject |
| Store copied Genome object directly inside `CellStore` | Local to Cell; simple lookup | Duplicates World-owned Genome state; risks clone-heavy hot path and identity confusion | Reject |
| World-owned Genome storage + CellStore progress/carrier fields | Aligns with typed IDs and SoA; deterministic; keeps Cells light | Requires more explicit plumbing and tests | Choose |

Confidence: `medium`.

## BDD Agent Scenario Cards

### AL-003-S04-AC01: Genome Copying Output Becomes Registered Runtime Intent

Sources: `docs/biology/action-process-registry.md`,
`docs/genetics/regulatory-interface.md`, `docs/delivery/roadmap.md`.

Given a Genome template with `genome_copying_priority`,
when config parsing and Genome Runtime output disposition run,
then `genome_copying_priority` is accepted only after `ProcessId::GenomeCopying`
is registered as a `status: now` planned long-running process, and it becomes an
`ActionPlan` candidate without bypassing Feasibility.

TDD obligation: update `phase3b_runtime_contract` and ActionPlan tests before
adding production support.

### AL-003-S04-AC02: Genome Copying Is Physical, Feasible, And Accounted

Sources: `docs/biology/genome.md`,
`docs/biology/division-partition.md`, `docs/mechanics/matter-accounting.md`,
`docs/mechanics/capacity-accounting.md`.

Given a Cell with a functional Genome, configured carrier recipe/cost, enough
Energy, enough physical carrier matter or precursor Resource, and enough
capacity,
when `GenomeCopying` is attempted,
then Feasibility allows bounded copy progress, consumes explicit Energy and
matter inputs, increases copied carrier/copy progress, and does not create matter
from Energy.

TDD obligation: add feasibility and execution tests proving paid progress and
rejection without material/energy/capacity.

### AL-003-S04-AC03: Genome Copying Is Long-Running And Deterministic

Sources: `docs/biology/process-progress.md`, `docs/engine/scheduler.md`,
`docs/mechanics/long-running-process.md`, `docs/mechanics/deterministic-execution.md`.

Given the same seed, config, and starting state,
when Genome copying progresses over multiple scheduled attempts,
then progress, final copy completion, and stable state hash are reproducible.

TDD obligation: add replay tests and include copy progress/copy identity in
stable state hashing.

### AL-003-S04-AC04: Mutation Is Explicit, Bounded, Validated, And Seeded

Sources: `docs/genetics/mutation.md`,
`docs/genetics/regulatory-network.md`, `docs/genetics/genome-representation.md`.

Given a completed Genome copy and configured mutation rate/bounds,
when deterministic copy finalization runs,
then mutation may alter only registered bounded Genome outputs or explicitly
modeled graph fields, invalid mutation is rejected or clamped by validation, and
the same seed/config/state yields the same copied Genome.

TDD obligation: add tests for no mutation at zero rate, deterministic mutation at
forced rate, bounds clamping, and invalid output rejection.

### AL-003-S04-AC05: Division Requires A Physical Genome Copy

Sources: `docs/biology/division-partition.md`, `docs/genetics/inheritance.md`,
`docs/biology/lifecycle.md`.

Given a division-ready Cell with a Genome but no complete copied Genome carrier,
when `Division` Feasibility runs,
then division is rejected before partition and no daughter is created. Given a
complete copied Genome carrier, division may proceed and the daughter receives a
valid `GenomeId` with physical carrier state.

TDD obligation: update division tests so hidden Genome cloning during division is
impossible.

### AL-003-S04-AC06: Genome Repair Does Not Become Automatic Rescue

Sources: `docs/biology/genome.md`, `docs/genetics/inheritance.md`,
`docs/genetics/mutation.md`.

Given damaged/incomplete Genome carrier state,
when Genome repair/copy-error handling is configured and feasible,
then repair consumes explicit resources/energy and can improve carrier integrity
or reject invalid copied Genome state, but it must not silently make offspring
viable or repair mutation damage without a process.

TDD obligation: add minimal tests for explicit paid repair or explicit rejection;
do not implement automatic repair as validation side effect.

## TDD Tasks

AL-003-S04-T01: RED for `AL-003-S04-AC01`

- Update `tests/phase3b_runtime_contract.rs` to expect
  `genome_copying_priority` as enabled now and mapped to
  `ProcessId::GenomeCopying`.
- Add `ProcessId::GenomeCopying` expectations in process registry tests.
- Run:
  - `cargo test --test phase3b_runtime_contract`
  - `cargo test --test phase2_process_registry`
- Capture expected failures as `AL-003-S04-EV01`.

AL-003-S04-T02: GREEN for `AL-003-S04-AC01`

- Add `ProcessId::GenomeCopying`, registry entry, stable ordering/hash mapping,
  and `GenomeOutputId::GenomeCopyingPriority`.
- Add Genome copying to `ActionPlan` only as a priority candidate.
- Do not execute copying from Genome Runtime directly.
- Capture pass as `AL-003-S04-EV02`.

AL-003-S04-T03: RED for `AL-003-S04-AC02`

- Add focused feasibility tests for `GenomeCopying`:
  - allowed when Energy, carrier/precursor matter, capability, and capacity are
    present;
  - rejected when each required input is missing;
  - rejected or disabled when config disables copying.
- Prefer a new `tests/phase3c_genome_copying.rs`.
- Capture expected failure as `AL-003-S04-EV03`.

AL-003-S04-T04: GREEN for `AL-003-S04-AC02`

- Add explicit config for Genome copying cost/progress with safe defaults and
  parser validation.
- Add `CellStore` SoA fields for copied carrier amount/integrity/progress, or a
  small equivalent state that remains Cell-owned until final copy commit.
- Implement `WorldState::validate_feasibility` and minimal execution for
  `GenomeCopying`.
- Ensure Energy cannot create carrier matter; tests must observe resource or
  material consumption.
- Capture pass as `AL-003-S04-EV04`.

AL-003-S04-T05: RED for `AL-003-S04-AC03`

- Add deterministic multi-Tick copy progress and stable hash replay tests.
- Verify process attempt cadence does not create hidden every-Tick copying unless
  configured.
- Capture expected failure as `AL-003-S04-EV05`.

AL-003-S04-T06: GREEN for `AL-003-S04-AC03`

- Add scheduled `GenomeCopying` attempt handling in `TickExecutor`.
- Include copy progress and completed copied Genome references in stable state
  hash.
- Keep iteration and commit order deterministic.
- Capture pass as `AL-003-S04-EV06`.

AL-003-S04-T07: RED for `AL-003-S04-AC04`

- Add mutation tests:
  - mutation rate `0.0` copies exactly;
  - forced mutation is deterministic for same seed/config/state;
  - mutated outputs remain bounded;
  - invalid output binding cannot be produced.
- Capture expected failure as `AL-003-S04-EV07`.

AL-003-S04-T08: GREEN for `AL-003-S04-AC04`

- Add a minimal deterministic mutation operator set suitable for current direct
  regulatory outputs, such as bounded output value shifts.
- Add a dedicated seed domain for Genome copy mutation.
- Validate copied Genome before commit.
- Do not add future operators such as HGT, recombination, fragment insertion, or
  topology growth unless already necessary for current tests.
- Capture pass as `AL-003-S04-EV08`.

AL-003-S04-T09: RED for `AL-003-S04-AC05`

- Update division tests:
  - division rejects when Genome copy is incomplete;
  - division with complete copied Genome assigns daughter a valid `GenomeId`;
  - parent and daughter carrier/capacity accounting stays non-negative.
- Capture expected failure as `AL-003-S04-EV09`.

AL-003-S04-T10: GREEN for `AL-003-S04-AC05`

- Gate division Feasibility on copied Genome availability only for Genome-bearing
  Cells.
- On division commit, assign one valid Genome to each daughter according to the
  copied-carrier rule.
- Reset or schedule Genome Runtime cadence for affected daughters
  deterministically.
- Capture pass as `AL-003-S04-EV10`.

AL-003-S04-T11: RED/GREEN for `AL-003-S04-AC06`

- Add minimal explicit Genome repair/copy-error tests or, if the code model does
  not yet expose damaged copy states, add a documented rejection path.
- Implement only the minimal paid repair/rejection behavior needed to prevent
  automatic rescue.
- Capture result as `AL-003-S04-EV11`.

AL-003-S04-T12: Regression Fence

- Run:
  - `cargo test --test phase3c_genome_copying`
  - `cargo test --test phase3b_runtime_contract`
  - `cargo test --test phase3a_action_plan`
  - `cargo test --test phase3a_tick_integration`
  - `cargo test --test phase2_division_smoke`
  - `cargo test --test phase2_process_registry`
  - `cargo test --test phase2_process_smoke`
  - `cargo test --test scheduler_genome_cadence`
  - `cargo test --test scheduler_config`
- Capture result as `AL-003-S04-EV12`.

AL-003-S04-T13: REFACTOR/Docs/Closure

- Refactor only after tests are green.
- Run:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `git diff --check`
- Update approved delivery-control artifacts:
  - `docs/delivery/roadmap.md`
  - `docs/delivery/status.md`
  - `docs/delivery/acceptance.md`
  - `docs/delivery/worklog-ledger.md`
  - `outputs/worklogs/index.md`
- Review `Candidate Next Work` during any roadmap change.
- Create closure report:
  - `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-003-s04-genome-copying-mutation-repair.md`
- Capture result as `AL-003-S04-EV13`.

## Verification Commands

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --test phase3c_genome_copying
cargo test --test phase3b_runtime_contract
cargo test --test phase3a_action_plan
cargo test --test phase3a_tick_integration
cargo test --test phase2_division_smoke
cargo test --test phase2_process_registry
cargo test --test phase2_process_smoke
cargo test --test scheduler_genome_cadence
cargo test --test scheduler_config
git diff --check
```

## Forbidden Scope

- Do not implement lineage event log or replay; that is `AL-003-S05`.
- Do not add Observer/UI projections; those belong to `AL-004`/`AL-007`.
- Do not create Resources, Materials, reactions, or Joint channels from Genome.
- Do not allow Energy to create physical Genome carrier matter.
- Do not introduce species IDs, cell classes, organs, brains, predators, or
  scripted behavior.
- Do not use observer metrics, lineage metrics, or fitness as behavior input.
- Do not use `Rc<RefCell<_>>`, `Arc<Mutex<_>>`, long-lived Cell references, or
  clone-heavy Cell-owned Genome state to work around ownership issues.
- Do not mark `AL-003-S04` done without closure verification.

## Open Questions

- Needs Review: exact copy cost, mutation rate defaults, and carrier precursor
  source should be conservative config values in implementation and may require
  later sweep calibration. They should not block the TDD plan.
- Needs Review: if damaged Genome carrier state is too thin for meaningful repair
  in this slice, close repair as explicit rejection/copy-error validation and
  defer richer damage-repair mechanics to a future slice rather than inventing a
  large repair subsystem.

## Approval Gate

Reply `OK EXECUTE AL-003-S04` to authorize execution of this TDD plan.

Reply `CHANGE AL-003-S04` with corrections to revise the plan.
