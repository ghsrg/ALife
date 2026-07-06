---
tags:
  - alife
  - worklog/plan
  - review
  - phase/2c
---

# PLAN: Phase 2B/2C Review Fixes

## Goal

Bring the completed Phase 2B/2C implementation back in line with the Phase 2 strategy before moving to Phase 2D.

Phase 2B/2C is broadly useful and most mechanics exist, but the review found several issues where tests currently prove local functions rather than the intended world behavior. Fix these before adding real division/decomposition in Phase 2D.

## Review Scope

Reviewed:

- `outputs/worklogs/2026-07-02-1855-PLAN-phase-2-global-roadmap.md`
- `outputs/worklogs/2026-07-03-1002-REPORT-phase-2b-process-registry.md`
- `outputs/worklogs/2026-07-03-1035-REPORT-phase-2b-reachability-validation.md`
- `outputs/worklogs/2026-07-03-1350-REPORT-phase-2b-material-stubs-resolution.md`
- `outputs/worklogs/2026-07-03-1105-PLAN-phase-2C-growth-and-division-prep.md`
- `outputs/worklogs/2026-07-03-1650-PLAN-phase-2c-reflexive-actions-growth.md`
- `outputs/worklogs/2026-07-03-1740-REPORT-phase-2c-reflexive-actions-growth.md`
- `outputs/worklogs/2026-07-03-1850-REPORT-phase2c-stability-analysis.md`
- `src/core/tick.rs`
- `src/core/world.rs`
- `src/core/cell_store.rs`
- `src/core/config.rs`
- `src/core/process.rs`
- `tests/phase2_process_smoke.rs`
- `tests/phase2_materials_smoke.rs`
- `tests/phase2_reachability.rs`
- `tests/phase2_growth_smoke.rs`
- `tests/phase2_reflex_smoke.rs`
- `tests/phase2_stability_analysis.rs`

Verification run during review:

```powershell
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --check
```

Observed:

- `cargo test --workspace --all-targets` passes.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes.
- `cargo fmt --check` fails on `tests/phase2_stability_analysis.rs`.

## Findings

### P0. Phase 2B Process Registry is still a match-based feasibility helper, not a registry contract

**Problem**

Phase 2B reports say Process Registry was introduced, but the current model is mostly:

- `ProcessId` enum in `src/core/process.rs:50`;
- `ActionCandidate` with `requested_amount` in `src/core/process.rs:61`;
- one large `WorldState::validate_feasibility()` match in `src/core/world.rs:164`;
- execution logic split across `TickExecutor::step()` and ad hoc `WorldState::execute_*` methods.

There is no single registry entry per process that declares:

```text
process_id
required_capabilities
required_inputs
energy_cost
material/resource cost
output/effect
failure modes
status
```

This is weaker than the documentation expectation from the process registry discussion and the Phase 2 global roadmap.

**Impact**

New processes can be added by editing match branches without updating a canonical registry entry. Feasibility, execution, reporting and tests can drift. This becomes risky before Phase 2D because Division and Decomposition need explicit failure modes and accounting contracts.

**Proposal**

Introduce a minimal in-code `ProcessSpec` registry in `src/core/process.rs`:

```text
ProcessSpec
  process_id
  status: Now | Future
  required_capabilities
  reads
  writes
  cost_policy
  failure_modes
```

Keep execution functions in `WorldState` / systems, but make feasibility derive from the spec where possible. If a rule is process-specific, the spec should name that predicate explicitly.

Add tests:

- every `ProcessId` has a registry entry;
- every `ProcessId::Now` has at least one feasibility test;
- `Future` process entries cannot execute;
- registry-required capability matches actual `validate_feasibility()` behavior.

### P0. `ActionCandidate.requested_amount` is not the execution source of truth

**Problem**

`ActionCandidate` carries `requested_amount`, but several process paths ignore it and use config constants instead:

- `src/core/world.rs:227-228` synthesis reads `config.synthesis.*`;
- `src/core/world.rs:271-272` growth reads `config.growth.*`;
- `src/core/world.rs:303-310` `execute_growth_for_test()` ignores `_action`;
- `src/core/world.rs:360-362` `execute_synthesis()` accepts no action/candidate.

Metabolism checks `requested_amount` in feasibility (`src/core/world.rs:212`), but actual execution in `src/core/tick.rs` consumes `config.resource_interaction.metabolism_resource_per_tick`.

**Impact**

Feasibility can approve one requested amount while execution performs another amount. This violates the ActionPlan/Feasibility contract and can produce hidden accounting bugs when Genome Runtime later modulates priorities/amounts.

**Proposal**

Decide the Phase 2 contract:

```text
ActionCandidate.requested_amount is the requested physical amount.
Feasibility returns accepted_amount or rejection.
Execution consumes exactly accepted_amount and the declared process cost policy.
```

Replace `FeasibilityResult::Feasible` with a richer result:

```text
Allowed {
  accepted_amount,
  resource_cost,
  energy_cost
}
Rejected(reason)
```

Then pass that accepted execution payload into `execute_*`.

Add tests:

- requesting less than config max executes only requested amount;
- requesting more than available is clamped or rejected by the declared policy;
- feasibility and execution agree for uptake, metabolism, synthesis and growth.

### P1. Legacy material fallback grants all capabilities from one generic material

**Problem**

Phase 2B material-stub resolution kept legacy compatibility by distributing general/legacy material across all 9 material vectors. The test asserts that a legacy config with `initial_materials = { cell_wall = 9.0 }` grants all 11 capabilities:

- `tests/phase2_materials_smoke.rs` `test_legacy_config_backward_compatibility`.

**Impact**

This is acceptable as a temporary parser compatibility shim, but it is dangerous as a domain default. A structural/boundary-like legacy material can silently become transport, metabolism, synthesis, contractility and sensing. This weakens mechanism reachability because many processes become reachable through parser fallback rather than actual material composition.

**Proposal**

Keep compatibility only behind an explicit legacy path:

```text
legacy_material_distribution = true
```

For current/future configs:

```text
unknown or generic material names must fail validation,
or map only through an explicit alias table to one material type.
```

Update tests:

- legacy scenario with explicit compatibility flag may distribute materials;
- current scenario with `cell_wall` maps only to structural/boundary, not all capabilities;
- current scenario with unknown material name fails parse with a clear error.

### P1. Capability disabling is test-only but lives in hot CellStore state

**Problem**

`CellStore` contains `disabled_capabilities: Vec<u16>` and exposes `strip_capability_for_test()`:

- `src/core/cell_store.rs:97`;
- `src/core/cell_store.rs:299-305`.

The method is named test-only, but the state is part of production `CellStore` and `has_capability()` checks it on every query.

**Impact**

This adds a non-physical override layer to behavior. It can become a hidden shortcut that disables material-derived capabilities without any material damage/degradation model. That contradicts the material-first strategy unless clearly marked as debug/test-only and impossible to set from configs or runtime behavior.

**Proposal**

Choose one:

Option A, preferred:

```text
remove disabled_capabilities from production CellStore;
tests should remove the underlying material amount instead.
```

Option B, acceptable temporarily:

```text
rename to debug_disabled_capabilities;
compile behind #[cfg(test)] if feasible;
assert parser/config cannot set it;
document it as non-Canon test harness state.
```

Add tests:

- capability disappears when corresponding material amount is zero;
- reachability tests use material mutation, not `strip_capability_for_test()`;
- no public non-test config can disable capability directly.

### P1. Feasibility diagnostics are aggregate counters only

**Problem**

Phase 2B added `process_attempts` and `process_rejections`, but the summary does not preserve per-process or per-reason diagnostics. Tests only assert coarse totals:

- `tests/phase2_reachability.rs` checks `process_attempts` / `process_rejections`.

**Impact**

For Phase 2D and later Genome work, aggregate rejection counts will not explain why a cell stalled. We need at least observer-only last feasibility result or per-process counters to debug material/reflex behavior.

**Proposal**

Add observer-only diagnostics without affecting behavior:

```text
ProcessDiagnostics
  attempts_by_process
  rejections_by_process
  rejections_by_reason
```

Keep it in run summary or debug snapshot, not as a Cell input.

Add tests:

- missing metabolism increments `MetabolismEnergyConversion / MissingCapability`;
- missing uptake increments `LocalResourceUptake / MissingCapability`;
- insufficient resources increments the correct process/reason pair.

### P0. Contractile reflex uses stale or manually injected pressure

**Problem**

`TickExecutor::step()` executes the reflex loop before the physics solver computes contact pressure:

- `src/core/tick.rs:53` starts `Phase A: Uptake, Metabolism, Synthesis, Growth, and Displacement Reflex Loop`.
- `src/core/tick.rs:194` attempts `ContractileDisplacement`.
- `src/core/tick.rs:211` starts the positional overlap solver.
- `src/core/tick.rs:214-281` resets and writes `contact_pressure`.

The tests that claim autonomous displacement manually inject pressure first:

- `tests/phase2_reflex_smoke.rs:116-120`
- `tests/phase2_stability_analysis.rs:208-209`
- `tests/phase2_stability_analysis.rs:227-228`

**Impact**

Phase 2C currently proves that displacement works if pressure was already present, but not that world-generated collision/growth pressure can trigger a reflex in the intended tick model. This weakens the acceptance gate `contractile_displacement_reachable_if_enabled` and can hide phase-order bugs before Phase 2D/2E.

**Proposal**

Refactor the tick into explicit deterministic subphases:

```text
rebuild spatial index
clear contact_pressure
contact sensing pass: detect overlaps and write pressure only
material reflex pass: uptake/metabolism/synthesis/growth/displacement
physics solve pass: resolve overlaps and walls
lifecycle/accounting commit
```

Alternatively, if pressure is intended to be previous-tick sensory state, rename it to `last_contact_pressure`, keep current order, and update docs/tests to assert one-tick delayed reflex. The preferred Phase 2C fix is same-tick contact sensing before reflex, because current tests and reports describe pressure as immediate collision input.

Add tests:

- overlapping contractile cells move without manually setting `contact_pressure`;
- non-overlapping contractile cells do not move;
- growth-created overlap produces pressure and then physical push/displacement deterministically.

### P0. Structural growth can accidentally create all material capabilities

**Problem**

`WorldState::execute_growth_for_test()` adds generic material mass and then calls `CellStore::set_materials()`:

- `src/core/world.rs:303` defines `execute_growth_for_test`.
- `src/core/world.rs:335` calls `set_materials`.
- `src/core/cell_store.rs:328` distributes the supplied amount evenly across all 9 material vectors.

This means structural growth can increase boundary, transport, metabolic, storage, synthesis, repair, contractile and sensory material too.

**Impact**

This contradicts Phase 2 material capability strategy. A structural growth process should not silently unlock uptake, metabolism, synthesis, contractility or sensing. It can create shortcuts where a cell gains capabilities through generic mass instead of explicit material synthesis and Feasibility.

**Proposal**

Rename and narrow the method:

```text
execute_growth_for_test -> execute_growth
growth consumes resource/energy budget
growth increases structural material or growth_progress only
radius/capacity may derive from physical mass
growth must not call set_materials()
```

Keep `set_materials()` only as a cold test/config helper or remove it from production growth paths.

Add tests:

- growth increases `structural_material`;
- growth increases radius/capacity;
- growth does not increase transport/metabolic/synthesis/contractile/sensory material;
- a cell without contractile material cannot become contractile through growth.

### P1. `division_ready` is not committed as runtime state

**Problem**

Phase 2 global roadmap names `division_ready flag as behavior state`, but current implementation only exposes division readiness through a direct `validate_feasibility(ProcessId::Division)` query:

- `src/core/world.rs:285-298`
- `src/core/cell_store.rs:32-38` `RuntimeFlags` has no `division_ready`.

**Impact**

Phase 2C acceptance says `division_ready` should be reached deterministically, without creating daughter cells. Phase 2D needs a committed source-of-truth flag/event to start real division. If readiness is only an ad hoc feasibility query, the next phase can accidentally recompute it at a different phase boundary.

**Proposal**

Add `division_ready: bool` to `RuntimeFlags`.

Compute it after growth and contact-pressure sensing are available:

```text
division_ready =
  radius >= growth_target_radius
  and contact_pressure <= max_division_pressure
  and cell is not Dead
```

Do not create daughters in this fix.

Add tests:

- below target radius -> `division_ready == false`;
- target radius and low pressure -> `division_ready == true`;
- target radius and high pressure -> `division_ready == false`;
- flag is stable in deterministic replay.

### P1. Multi-cell resource allocation has deterministic but behavior-distorting index bias

**Problem**

The stability analysis explicitly documents that the first cell takes scarce resources first:

- `tests/phase2_stability_analysis.rs:581` `test_multi_cell_resource_order_independence`.
- `tests/phase2_stability_analysis.rs:636-638` only prints the bias; it has no assertions.

The current reflex loop processes cells sequentially by dense index:

- `src/core/tick.rs:57` loops `for i in 0..len`.

**Impact**

For multi-cell worlds, fixed index order can become artificial selection pressure. It is deterministic, but not physically meaningful. This matters before Phase 2D because division will create new indices and could bias descendants or earlier cells.

**Proposal**

Decide and implement one explicit policy:

Option A, recommended for Phase 2C hardening:

```text
For each resource grid cell:
  collect uptake requests from Cells mapped to that grid cell
  distribute available resource proportionally or equally by deterministic sorted CellIndex
  commit accepted uptake after allocation
```

Option B, acceptable as temporary:

```text
Keep sequential allocation, but mark it as known temporary engine bias,
add a failing/ignored test and require replacement before Phase 2D division scale tests.
```

Add tests:

- two identical cells in same resource cell receive equal or explicitly bounded allocations;
- swapping initial order does not change aggregate survival outcome for identical cells.

### P1. Multi-cell summaries still mix aggregate and first-cell-only metrics

**Problem**

`TickExecutor::build_metrics_summary()` sums final energy across all cells but reads resources/capacity/growth readiness only from the first cell:

- `src/core/tick.rs:623` uses `CellIndex::from_raw(0)`.

**Impact**

Stability/reachability reports for Phase 2C can be misleading. A run with one healthy first cell and several collapsing cells may look better than it is, or vice versa. This will get worse in Phase 2D when daughter cells appear.

**Proposal**

Replace first-cell metrics with aggregate metrics:

```text
final_internal_resources = sum over living/non-dead cells
final_used_capacity = sum over living/non-dead cells
final_free_capacity = sum over living/non-dead cells
growth_readiness = any or count-based observer metric, not first-cell-only
```

If per-cell detail is needed, keep it in snapshots or a separate debug projection, not in run-level summary.

Add tests:

- two-cell run reports summed internal resources;
- first dead / second alive run reports correct aggregate survival and metrics;
- growth readiness is not based only on cell index 0.

### P2. Config hash omits behavior-critical fields

**Problem**

`RuntimeConfig::config_hash()` omits multiple fields that affect behavior:

- world size;
- spatial grid size;
- physics solver iterations;
- cell positions;
- cell radii;
- lifecycle thresholds/modifiers;
- environment heat/waste thresholds and sinks;
- `growth_enabled`.

Relevant location:

- `src/core/config.rs:269-326`.

**Impact**

The deterministic replay contract depends on config identity. Two behaviorally different configs can produce the same reported `config_hash`, which weakens run comparison and debugging.

**Proposal**

Extend `config_hash()` to include every behavior-critical scalar and all per-cell position/radius fields.

Add tests:

- changing position changes hash;
- changing radius changes hash;
- changing physics iterations changes hash;
- changing lifecycle/environment thresholds changes hash;
- changing `growth_enabled` changes hash.

### P2. Formatting is not clean

**Problem**

`cargo fmt --check` fails on `tests/phase2_stability_analysis.rs`.

**Impact**

The current branch is not fully verification-clean even though tests and clippy pass.

**Proposal**

Run:

```powershell
cargo fmt
cargo fmt --check
```

Then rerun:

```powershell
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Implementation Order

1. Fix formatting first, because it is mechanical and removes noise.
2. Turn Phase 2B process definitions into a minimal registry contract.
3. Make feasibility/execution use the same accepted action amount.
4. Fix structural growth material accounting.
5. Fix contact-pressure/reflex tick ordering or explicitly convert it to previous-tick pressure.
6. Add committed `division_ready` runtime flag.
7. Replace first-cell-only run metrics with multi-cell aggregates.
8. Decide and implement resource allocation fairness policy.
9. Tighten legacy material fallback and remove or isolate test-only capability disabling.
10. Expand Process diagnostics.
11. Expand `config_hash()` coverage.
12. Rerun all Phase 1/2 tests and clippy.

## Acceptance Gates

The fix is complete when:

- every `ProcessId` has a minimal registry entry;
- feasibility and execution use the same accepted action amount;
- legacy/generic material parsing cannot silently grant all capabilities except through an explicit compatibility mode;
- reachability tests manipulate real material amounts instead of hidden capability overrides, or the override is strictly test/debug-only;
- diagnostics can identify process and rejection reason, not only total rejection count;
- no test manually sets `contact_pressure` to prove autonomous displacement;
- growth cannot create unrelated material capabilities;
- `RuntimeFlags` exposes committed `division_ready`;
- two-cell/multi-cell summaries are aggregate-safe;
- scarce local resource allocation has an explicit tested policy;
- behavior-critical config changes alter `config_hash`;
- `cargo fmt --check` passes;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes;
- `cargo test --workspace --all-targets` passes.

## Out Of Scope

- No real daughter-cell creation.
- No death decomposition.
- No Joints.
- No Genome Runtime.
- No viewer changes.

These remain Phase 2D, Phase 2E, Phase 3, or Phase View work.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
