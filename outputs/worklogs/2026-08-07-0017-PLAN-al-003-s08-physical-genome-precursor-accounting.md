---
plan_id: AL-003-S08
status: proposed
date: 2026-08-07
scope: Physical Genome Precursor Accounting
---

# AL-003-S08 Physical Genome Precursor Accounting

## Delivery-Control Routing

Mode: `TDD_PLAN_REQUEST`

Route:

```text
delivery-control -> roadmap-control -> writing-plans
```

Decision: create a TDD handoff plan only. Do not execute implementation in this slice-planning pass.

Approval gate:

```text
OK EXECUTE AL-003-S08
```

## Slice Status

Roadmap status: `planned`.

Operational status: `blocked-dependency` on `AL-003-S07`.

Reason: `AL-003-S08` should consume the Resource-derived material/accounting model established by `AL-003-S07`. The plan can be prepared now, but implementation should start only after `AL-003-S07` is accepted, merged, or explicitly chosen as the base branch.

## Goal

Make Genome copying and Genome recombination physically accounted operations. They must consume configured local nucleotide-like precursor Resources and Energy only after feasibility succeeds, keep Genome carrier state explicit, and preserve all state on rejection.

This slice replaces current generic Genome-carrier accounting and hardcoded recombination cost. It does not change mutation/recombination algorithms except where accounting and validation require stable carrier provenance.

## Source Hierarchy

Authority order used for this plan:

1. `docs/PRINCIPLES.md`
2. `docs/biology/genome.md`
3. `docs/genetics/inheritance.md`
4. `docs/genetics/recombination.md`
5. `docs/genetics/mutation.md`
6. `docs/genetics/genome-runtime.md`
7. `docs/biology/action-process-registry.md`
8. `docs/biology/feasibility.md`
9. `docs/biology/process-capabilities.md`
10. `docs/world/resources.md`
11. `docs/world/materials.md`
12. `docs/world/reactions.md`
13. `docs/config/reactions_config.md`
14. `docs/mechanics/genome-action-pipeline.md`
15. `docs/mechanics/action-feasibility.md`
16. `docs/mechanics/division-inheritance.md`
17. `docs/mechanics/matter-accounting.md`
18. `docs/mechanics/resource-material.md`
19. `docs/mechanics/material-decomposition.md`
20. `docs/delivery/roadmap.md`
21. `docs/delivery/status.md`
22. `docs/delivery/acceptance.md`
23. `outputs/worklogs/2026-08-06-2246-PLAN-al-003-s07-resource-derived-material-synthesis.md`

Implementation files inspected:

- `src/core/config.rs`
- `src/core/world.rs`
- `src/core/process.rs`
- `src/core/genome.rs`
- `src/runner/config_parser.rs`
- `tests/phase3c_genome_copying.rs`
- `tests/phase3e_recombination.rs`

Fixture created for downstream tests:

- `config/scenarios/demo/canonical_test_world.toml`

## Current Code Facts

- `GenomeCopyingConfig` currently stores `carrier_resource_cost_per_step` as a generic `ResourceAmount`.
- `WorldState::validate_feasibility` for `ProcessId::GenomeCopying` checks `generic_resource_amount`, not typed nucleotide-like resources.
- `WorldState::execute_genome_copying` debits generic resources and increments `copied_genome_carrier_amount`.
- `WorldState::validate_feasibility` for `ProcessId::GenomeRecombination` uses a hardcoded energy cost of `4.0` and no precursor requirements.
- `WorldState::execute_genome_recombination` performs its own direct energy check/debit and does not use a configured physical precursor transaction.
- `MaterialCapabilityFlags::has(MaterialCapability::GenomeCopying)` currently derives Genome copying capability from `material_synthesis && repair`, which is a shortcut rather than explicit material-derived capability.
- `GenomeCarrierState` already records `material_id`, `amount`, and `integrity`, but the carrier amount is not tied to a typed precursor transaction.

## Assumptions

1. `AL-003-S07` will provide or has provided `MaterialInstance`, material capability profiles, resource-derived material synthesis, and explicit material fragment accounting.
2. `canonical_test_world.toml` is a forward-contract fixture. On current `main`, parser/runtime may ignore or reject some S07/S08 fields until the planned slices are implemented.
3. `nucleotide_precursor`, `phosphate`, `short_peptide`, and `catalyst_mineral` are scenario Resources, not special biological shortcuts. Their usefulness comes only from configured processes/reactions.
4. Genome carrier matter is modeled as physical carrier state attached to Genome state, not as a behavior-defining Material role.
5. Field runtime effects remain `AL-003-S09` scope and must stay manifest-only here.

## Open Questions

| ID | Question | Default for implementation unless changed |
| --- | --- | --- |
| `AL-003-S08-Q01` | Should copy/recombination requirements live directly under `[genome_copying]` / `[genome_recombination]`, or under a shared `[genome_physical_accounting]` config? | Use a shared `genome_physical_accounting` config with per-process requirement sets to avoid duplicating resource validation logic. |
| `AL-003-S08-Q02` | Should recombination consume precursors from only the acting cell or from both participating cells? | Consume from the acting cell first; partner only supplies Genome information/contact unless a later explicit exchange rule is added. |
| `AL-003-S08-Q03` | Should Genome carrier degradation immediately return Resources? | No. It creates carrier fragments/remains first; Resource recovery needs explicit configured conversion/degradation output. |

## BDD Scenario Cards

### `AL-003-S08-SC01` Typed Genome Copying Precursors

Given a living cell with a Genome, explicit Genome-copying capability, Energy, free capacity, and configured local precursor Resources.

When `genome_copying` feasibility and execution run.

Then the action consumes only the configured typed precursor Resources and Energy, increments copy progress, adds copied carrier amount/provenance, and does not debit generic resource pools.

Acceptance: `AL-003-S08-AC01`.

### `AL-003-S08-SC02` Copy Rejection Is Atomic

Given a Genome-copying action where Energy, capability, capacity, Genome state, or one configured precursor is missing.

When feasibility rejects the action.

Then no Resource, Energy, Genome copy progress, copied carrier amount, copied Genome id, mutation delta, or lineage event changes.

Acceptance: `AL-003-S08-AC01`.

### `AL-003-S08-SC03` Copy Completion Produces Physical Carrier Evidence

Given a cell completes Genome-copy progress.

When the copied Genome is created.

Then copied `GenomeCarrierState` records material id, amount, integrity, source requirements, and deterministic lineage evidence without treating carrier damage as mutation.

Acceptance: `AL-003-S08-AC01`.

### `AL-003-S08-SC04` Recombination Has Physical Cost And Contact

Given two cells with local contact or Joint path, valid Genome states, recombination-capable materials, Energy, and configured precursor Resources.

When `genome_recombination` executes.

Then recombination consumes configured Energy and precursor Resources atomically, creates a recombined Genome through the existing deterministic recombination operator, and records lineage/provenance evidence.

Acceptance: `AL-003-S08-AC01`.

### `AL-003-S08-SC05` Recombination Rejection Preserves State

Given recombination lacks contact/Joint, capability, Energy, Genome, or one required precursor.

When feasibility rejects the action.

Then neither participant loses Resources or Energy, and no Genome output, Genome id, lineage event, recombination trace, or carrier state changes.

Acceptance: `AL-003-S08-AC01`.

### `AL-003-S08-SC06` Genome Carrier Degradation Is Explicit Matter Accounting

Given Genome carrier material is damaged, dead, or decomposing.

When degradation/decomposition processes run.

Then carrier matter becomes explicit fragments/remains or configured Resource outputs only through declared degradation/conversion rules, never silent deletion or Energy substitution.

Acceptance: `AL-003-S08-AC01`.

### `AL-003-S08-SC07` Canonical Test World Exercises The Surface

Given `config/scenarios/demo/canonical_test_world.toml`.

When scenario resolution and validation run after S07/S08 support exists.

Then the scenario declares Resource-derived material synthesis, nucleotide-like precursor requirements, metabolism by-products, natural decay, material degradation, explicit fragment conversion, and manifest-only local Field declarations without special species/cell roles.

Acceptance: `AL-003-S07-AC03`, `AL-003-S08-AC01`, downstream `AL-003-S09-AC01` manifest boundary only.

## Acceptance Mapping

| Acceptance ID | Scenario cards | Primary evidence |
| --- | --- | --- |
| `AL-003-S08-AC01` | `SC01`, `SC02`, `SC03`, `SC04`, `SC05`, `SC06`, `SC07` | `tests/phase3g_genome_precursors.rs`, `tests/phase3e_recombination.rs`, `tests/phase3c_genome_copying.rs`, `tests/phase3f_canonical_test_world.rs` |

## TDD Tasks

### `AL-003-S08-T01` RED: Parser rejects missing or invalid physical Genome precursor config

Add failing tests in `tests/phase3g_genome_precursors.rs` for:

- valid `[genome_physical_accounting.copying]` typed resource requirements;
- valid `[genome_physical_accounting.recombination]` typed resource requirements;
- unknown resource id rejection;
- negative or zero requirement rejection where a requirement is declared;
- unknown output resource rejection;
- config hash changes when precursor requirements change.

Expected initial failure: config structs and parser fields do not exist.

Evidence ID: `AL-003-S08-E01`.

### `AL-003-S08-T02` GREEN: Add config model and parser validation

Implement minimal config types:

- `GenomePhysicalAccountingConfig`
- `GenomePrecursorRequirement`
- `GenomeProcessAccountingRule`

Parser behavior:

- validate typed Resource ids against declared `resources.resource_type_ids`;
- validate non-negative costs and outputs;
- preserve declaration-order deterministic hashing;
- keep backwards compatibility only when the new accounting section is absent in legacy scenarios;
- make `canonical_test_world.toml` resolve after S07 material-synthesis parser support is present.

Evidence ID: `AL-003-S08-E02`.

### `AL-003-S08-T03` RED: Copying feasibility uses typed precursors and is read-only

Add tests proving:

- missing `nucleotide_precursor` rejects with `InsufficientResources`;
- missing `phosphate` rejects with `InsufficientResources`;
- insufficient Energy rejects before mutation/progress;
- insufficient free capacity rejects before debit;
- rejection preserves all typed Resources, Energy, copy progress, copied carrier amount, copied Genome id, and lineage event count.

Expected initial failure: current code checks `generic_resource_amount` and returns allowed/rejected based on the generic pool.

Evidence ID: `AL-003-S08-E03`.

### `AL-003-S08-T04` GREEN: Implement atomic typed Genome-copying transaction

Change Genome-copying feasibility/execution to:

- compute accepted progress from configured progress and remaining copy progress;
- compute per-step typed Resource requirements;
- verify each typed Resource amount before debit;
- verify Energy and capacity before debit;
- debit all typed Resources and Energy together only after all checks pass;
- remove use of `carrier_resource_cost_per_step` for new configs;
- keep legacy scenarios stable when no physical accounting section is configured.

Evidence ID: `AL-003-S08-E04`.

### `AL-003-S08-T05` RED: Copy completion records physical carrier provenance

Add tests proving completed copied Genome records:

- carrier material id from config/template;
- carrier amount equal to configured physical-accounting output, not generic resource consumed;
- carrier integrity bounded by source/template rule;
- deterministic lineage event includes carrier amount and material id;
- forced mutation still only changes Genome outputs through existing mutation operator.

Expected initial failure: copied carrier amount is derived from generic resource consumption and lacks typed-source provenance.

Evidence ID: `AL-003-S08-E05`.

### `AL-003-S08-T06` GREEN: Bind copied Genome carrier state to precursor transaction

Implement minimal carrier accounting:

- derive copied carrier amount from configured output amount/progress;
- store copied carrier material id and amount in Cell/Genome state;
- keep existing mutation operator deterministic;
- update stable-state/config hash if carrier accounting state is included in committed state.

Evidence ID: `AL-003-S08-E06`.

### `AL-003-S08-T07` RED: Recombination feasibility requires contact plus physical precursors

Extend `tests/phase3e_recombination.rs` or add `tests/phase3g_genome_precursors.rs` cases for:

- no contact/Joint rejects with no debit;
- missing recombination precursor rejects with no debit;
- insufficient Energy rejects with no debit;
- missing recombination capability rejects with no debit;
- successful feasibility reports configured typed requirements.

Expected initial failure: recombination uses hardcoded Energy cost and no precursor requirements.

Evidence ID: `AL-003-S08-E07`.

### `AL-003-S08-T08` GREEN: Implement atomic recombination precursor debit

Change recombination execution to use the same feasibility transaction model:

- no direct hardcoded `4.0` energy check in execution;
- no execution-side feasibility bypass;
- debit configured typed Resources and Energy only after contact/Joint, capability, Genome, and resource checks pass;
- keep existing deterministic crossover/recombine semantics.

Evidence ID: `AL-003-S08-E08`.

### `AL-003-S08-T09` RED: Genome carrier degradation/death accounting is explicit

Add tests for:

- carrier degradation does not mutate Genome information directly;
- dead-cell decomposition keeps carrier matter accounted;
- Resource recovery from carrier/fragments requires explicit configured output/conversion;
- no silent conversion of Genome carrier matter to generic Resource.

Expected initial failure: decomposition and carrier accounting are not connected enough to prove this.

Evidence ID: `AL-003-S08-E09`.

### `AL-003-S08-T10` GREEN: Implement minimal carrier fragment/remains accounting

Implement only the smallest behavior required by `T09`:

- produce explicit Genome carrier fragment/remains accounting on death/degradation;
- route Resource recovery through configured degradation/conversion outputs;
- keep external fragments inactive outside living-cell context;
- avoid adding active Field behavior or new biological roles.

Evidence ID: `AL-003-S08-E10`.

### `AL-003-S08-T11` RED/GREEN: Canonical test world validation

Add or update scenario tests:

- `tests/phase3f_canonical_test_world.rs` verifies `canonical_test_world.toml` resolves;
- asserts `nucleotide_precursor` exists;
- asserts genome precursor manifest/config is present;
- asserts copy and recombination requirements use typed Resources;
- asserts material synthesis and fragment conversion surfaces remain present from S07;
- asserts field declarations are manifest-only and do not execute S09 behavior.

Evidence ID: `AL-003-S08-E11`.

### `AL-003-S08-T12` REFACTOR: Remove shortcuts and centralize accounting helpers

Clean up after tests pass:

- remove or deprecate `carrier_resource_cost_per_step` where new config is present;
- replace `MaterialCapability::GenomeCopying => material_synthesis && repair` with explicit material-derived capability from S07 capability profiles;
- centralize typed Resource debit preflight to avoid separate copy/recombination implementations;
- keep public API changes minimal and documented in tests.

Evidence ID: `AL-003-S08-E12`.

### `AL-003-S08-T13` REGRESSION: Determinism, replay, and legacy safety

Run regression checks:

- copied Genome deterministic under same seed/tick/state;
- recombination deterministic under same contact/partner/crossover state;
- stable config hash changes when precursor requirements change;
- existing Phase 3C copying tests still pass or are intentionally migrated;
- existing Phase 3E recombination tests still pass with configured physical costs;
- legacy scenarios without physical accounting keep existing behavior until migrated.

Evidence ID: `AL-003-S08-E13`.

## Verification Commands

Run focused checks first:

```powershell
cargo test --test phase3g_genome_precursors
cargo test --test phase3c_genome_copying
cargo test --test phase3e_recombination
cargo test --test phase3f_canonical_test_world
```

Then run broader Core/parser checks:

```powershell
cargo test --test phase3a_genome_bootstrap --test phase3a_tick_integration --test phase3a_action_plan
cargo test --test runner_scenario_loader
cargo test --workspace --lib
```

Final verification if the Windows linker/disk environment allows:

```powershell
cargo test --workspace --all-targets
git diff --check
```

If `cargo test --workspace --all-targets` fails with Windows PDB linker limits or disk exhaustion, record the exact toolchain error and preserve focused Rust test evidence. Do not mark `AL-003-S08` done from focused tests alone; route to closure verification.

## Forbidden Scope

- Do not implement `AL-003-S09` local Field runtime.
- Do not add species IDs, cell classes, organs, predators, brains, or scripted behavior.
- Do not let Resources directly provide function; function must come through Materials/capability/process rules.
- Do not let Energy create Genome carrier matter.
- Do not mutate Genome information due only to carrier damage.
- Do not add directed beneficial mutation/recombination.
- Do not silently consume MaterialFragments as ordinary Resources.
- Do not refactor UI or Observer projections beyond what is required to expose source-backed accounting evidence.
- Do not change `canonical_living_world.toml` in this slice unless explicitly approved.

## Deterministic Delivery-Lint Result

Scope checked:

- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `outputs/worklogs/2026-08-06-2246-PLAN-al-003-s07-resource-derived-material-synthesis.md`

| Severity | Rule | ID/Path | Finding | Required action |
| --- | --- | --- | --- | --- |
| `WARN` | `DL007` | `AL-003-S07` | `AL-003-S07` is still `planned` / dependency for `AL-003-S08`; current repository also has uncommitted S07 roadmap/status/acceptance edits. | Do not execute S08 until S07 is accepted/merged or user explicitly chooses the S07 worktree/base. |
| `WARN` | `DL008` | `canonical_test_world.toml` | The new canonical test scenario is a forward-contract fixture while current main parser/runtime does not fully support S07/S08 fields. | Treat as RED fixture for S07/S08 tests, not proof of current runtime support. |

Decision: `PASS_WITH_WARNINGS` for plan creation; `BLOCKED_DEPENDENCY` for implementation execution.

## Handoff

After approval, start with `AL-003-S08-T01` and do not skip RED tests. The first implementation task must make currently implicit/generic Genome carrier accounting visible as typed Resource requirements before changing recombination behavior.
