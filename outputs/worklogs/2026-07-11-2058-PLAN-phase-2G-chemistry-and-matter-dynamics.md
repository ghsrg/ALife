# Phase 2G Chemistry And Matter Dynamics Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace Phase 2's generic resource layers and direct material decomposition with deterministic typed chemistry, material-specific degradation, explicit MaterialFragments, local Heat, Boundary retention/leakage and baseline repair before Genome Runtime.

**Architecture:** Keep `WorldState` as the owner of persistent typed Resource, MaterialFragment and local Heat state. Implement reactions as validated registry entries that read committed snapshots, emit typed deltas, pass accounting validation, and commit in deterministic order. Extend the existing shared Observer contract so Core summaries and `sweep_analyzer` consume the same mechanism metrics.

**Tech Stack:** Rust 2024, existing `alife-core` modules, `serde`/`toml`, typed unit wrappers, deterministic Tick pipeline, Cargo integration tests, TOML scenario/analyzer configs.

---

## Scope And Non-Goals

Phase 2G must implement the following minimum domain surface:

```text
ResourceTypeId registry:
  volume, diffusion_rate, energy_value, decay_rate,
  reactivity_profile, permeability_constraints, tags

MaterialTypeId registry:
  volume, stability, strength, permeability, energy_capacity,
  decay_rate, repair_requirements, reaction_profile,
  signal sensitivity/storage/conductivity fields

ReactionRegistry:
  passive, controlled, degradation, decay, synthesis, conversion
  inputs, conditions, catalyst/material requirements, rate, probability,
  locality, products, heat release, permitted energy output, accounting

MaterialFragmentStore:
  stable identity, MaterialTypeId, amount, location, stability,
  damage, decay state and explicit conversion path

Local Heat:
  temperature, capacity, generated heat, transfer/dissipation,
  material tolerance and degradation threshold

Boundary baseline:
  material-derived permeability, retention, leakage and damage modifiers

Repair baseline:
  explicit input consumption, Feasibility gate, bounded restoration and failure
```

Do not implement Genome regulation, HGT, full thermodynamics, global chemical fields, organism control, automatic fragment uptake or persistent Joints in this plan. Controlled reactions and repair must expose registered process hooks so Phase 3 can regulate them later.

Required invariants:

```text
Energy is not matter and passive reactions never directly credit Energy Buffer.
No reaction creates products without typed input matter.
Every input has a product, retained state, residual/waste or explicit configured sink.
Material leaving a Cell becomes MaterialFragment before any Resource conversion.
MaterialFragment has no active Cell capability outside Cell/Joint context.
Resource movement and reactions are local and deterministic.
Feasibility is read-only; rejected controlled reactions consume nothing.
All committed deltas use stable ordering and typed non-negative amounts.
Observer metrics are read-only and identical for Core consumers and sweep_analyzer.
```

## File Map

Create:

```text
src/core/resource_types.rs
  ResourceTypeId, ResourceType, ResourceRegistry, typed resource property validation.

src/core/material_types.rs
  MaterialTypeId, MaterialType, MaterialRegistry, MaterialState and capability/reaction profiles.

src/core/reactions.rs
  ReactionId, ReactionMode, ReactionDefinition, conditions, input/output terms,
  accounting destinations, ReactionRegistry validation and deterministic matching.

src/core/fragments.rs
  MaterialFragmentId, MaterialFragment, FragmentStore and fragment conversion/degradation state.

src/core/heat.rs
  local Heat state, generated/transfer/dissipation deltas and material tolerance checks.

tests/phase2g_resource_types.rs
tests/phase2g_material_types.rs
tests/phase2g_reactions.rs
tests/phase2g_accounting.rs
tests/phase2g_fragments.rs
tests/phase2g_heat_boundary_repair.rs
tests/phase2g_tick_integration.rs
tests/phase2g_observer_outputs.rs
tests/phase2g_sweep_parser.rs
tests/phase2g_determinism.rs

config/chemistry/phase2g.toml
config/scenarios/chemistry/phase2g.toml
```

Modify:

```text
src/core/mod.rs
  Export the new domain modules.

src/core/units.rs
  Add only typed wrappers required by the new registries/deltas; do not expose raw f32 amounts from public chemistry APIs.

src/core/resources.rs
  Preserve grid locality while replacing the single optional decay behavior with per-ResourceType properties and deterministic diffusion/decay operations.

src/core/materials.rs
  Keep the current composition slots as a compatibility adapter while adding MaterialType identity, MaterialState and registry-backed properties.

src/core/config.rs
  Add validated Phase 2G runtime configuration, registry definitions, reaction definitions, Heat, Boundary and repair settings to config hashing.

src/runner/config_parser.rs
  Parse and validate Phase 2G TOML ids, references, bounds and accounting destinations.

src/core/deltas.rs
  Add typed chemistry, fragment, Heat, Boundary and repair delta variants plus deterministic conflict validation.

src/core/world.rs
  Own registries, FragmentStore and local Heat state; expose read-only snapshots and commit-only mutation APIs.

src/core/tick.rs
  Insert passive locality/diffusion/decay, reaction matching/execution, Heat, degradation, repair and fragment commit phases.

src/core/summary.rs
  Add chemistry, fragment, Heat, Boundary, repair and accounting counters to committed summaries.

src/observer/projection.rs
  Normalize Phase 2G metrics into the shared Observer projection contract.

src/bin/sweep_analyzer.rs
  Consume Core/Observer metrics and export Phase 2G CSV fields without duplicating chemistry formulas.

config/analyzer/sweep_analyzer.toml
config/analyzer/sweep_analyzer_smoke.toml
  Add full and minimal Phase 2G scenario coverage.

outputs/worklogs/index.md
  Add the completed Phase 2G report after implementation.
```

## Domain Decisions To Keep Stable

1. `ResourceLayerIndex` remains a storage/index adapter during migration. New behavior resolves a validated `ResourceTypeId` through `ResourceRegistry`; no caller may assume layer number equals semantic resource identity.
2. The first chemistry fixture contains two Resources with different decay/diffusion/permeability and two Materials with different stability/decay. Use `nutrient_A`, `waste_A`, `boundary_polymer_A` and `structural_polymer_A` as config ids, not hardcoded behavior ids.
3. Reaction probability uses a deterministic sampler derived from `Seed`, `Tick`, location and `ReactionId`; it must not use wall-clock state, unordered map iteration or a global mutable RNG.
4. Passive reactions run without Genome. Controlled reactions are only executable through a registered `ProcessId` and a passed Feasibility result.
5. Initial material slots remain available to existing Phase 2 tests. Their registry-backed `MaterialTypeId` and `MaterialState` are the new source for Phase 2G degradation/Boundary/repair behavior.
6. Decomposition emits fragments first. A separate registered degradation/conversion reaction consumes a fragment and produces typed Resources; no direct `MaterialAmount -> ResourceGrid` shortcut remains.

## TDD Tasks

### Task 1: Add Typed Resource And Material Identity Registries

**Files:**
- Create: `src/core/resource_types.rs`
- Create: `src/core/material_types.rs`
- Modify: `src/core/units.rs`, `src/core/materials.rs`, `src/core/mod.rs`
- Test: `tests/phase2g_resource_types.rs`, `tests/phase2g_material_types.rs`

- [ ] **Step 1: Write failing Resource registry tests**

Add tests with these exact behaviors:

```rust
#[test]
fn resource_registry_resolves_ids_and_preserves_properties() {}

#[test]
fn resource_registry_rejects_duplicate_ids_and_invalid_properties() {}

#[test]
fn resource_properties_are_not_shared_by_unrelated_types() {}
```

The first test registers `nutrient_A` and `waste_A` with different decay/diffusion/energy values and asserts lookup by `ResourceTypeId`. The second covers duplicate id, negative rate, non-finite value and unknown permeability reference. The third proves changing one type does not change the other.

- [ ] **Step 2: Run the focused tests and verify red**

Run:

```text
cargo test --test phase2g_resource_types -- --nocapture
```

Expected: compilation failure because `ResourceRegistry` and `ResourceTypeId` do not exist.

- [ ] **Step 3: Write failing Material registry tests**

Add tests with these exact behaviors:

```rust
#[test]
fn material_registry_resolves_physical_and_reaction_properties() {}

#[test]
fn material_registry_rejects_invalid_stability_decay_and_repair_requirements() {}

#[test]
fn material_state_tracks_damage_fatigue_and_stored_signal_without_cell_role() {}
```

- [ ] **Step 4: Implement minimal typed registries**

Implement private validated fields and public read-only accessors. Use typed ids, `ResourceAmount`/`MaterialAmount` wrappers and `Result<_, RegistryError>`. Add `MaterialState { damage, fatigue, stored_signal, conductivity_modifier }` as a value object with bounded constructors. Keep `MaterialComposition` unchanged except for an explicit registry lookup adapter.

- [ ] **Step 5: Run both focused test files and verify green**

Run:

```text
cargo test --test phase2g_resource_types -- --nocapture
cargo test --test phase2g_material_types -- --nocapture
```

Expected: all tests pass.

- [ ] **Step 6: Commit the registry boundary**

```text
git add src/core/resource_types.rs src/core/material_types.rs src/core/units.rs src/core/materials.rs src/core/mod.rs tests/phase2g_resource_types.rs tests/phase2g_material_types.rs
git commit -m "feat: add typed resource and material registries"
```

### Task 2: Parse And Validate Phase 2G Configuration

**Files:**
- Create: `config/chemistry/phase2g.toml`, `config/scenarios/chemistry/phase2g.toml`
- Modify: `src/core/config.rs`, `src/runner/config_parser.rs`
- Test: `tests/phase2g_sweep_parser.rs`, `tests/phase2_config_hash.rs`

- [ ] **Step 1: Add failing parser tests**

Add tests that parse a minimal TOML fixture containing two ResourceTypes, two MaterialTypes, one passive decay reaction and one controlled conversion reaction. Assert that references resolve, numeric bounds are normalized and the config hash changes when a registry property or reaction coefficient changes. Add negative cases for unknown ids, duplicate ids, negative rates, probability outside `0..=1`, missing accounting destination and unknown catalyst.

- [ ] **Step 2: Run parser tests and verify red**

```text
cargo test --test phase2g_sweep_parser phase2g -- --nocapture
cargo test --test phase2_config_hash -- --nocapture
```

Expected: compile or assertion failures because Phase 2G raw config structures are absent.

- [ ] **Step 3: Implement raw and runtime config structures**

Add explicit parser structs for `[chemistry.resources.*]`, `[chemistry.materials.*]`, `[chemistry.reactions.*]`, `[chemistry.heat]`, `[chemistry.boundary]` and `[chemistry.repair]`. Convert them into validated registries and reaction definitions before `RuntimeConfig` is returned. Reject unknown references before world construction. Include normalized registry and reaction values in `RuntimeConfig::config_hash` in stable declaration order.

- [ ] **Step 4: Run parser, config-hash and existing config tests**

```text
cargo test --test phase2g_sweep_parser -- --nocapture
cargo test --test phase2_config_hash -- --nocapture
cargo test --test phase1_config_validation -- --nocapture
```

Expected: Phase 2G tests and existing config tests pass.

- [ ] **Step 5: Commit validated configuration**

```text
git add config/chemistry/phase2g.toml config/scenarios/chemistry/phase2g.toml src/core/config.rs src/runner/config_parser.rs tests/phase2g_sweep_parser.rs tests/phase2_config_hash.rs
git commit -m "feat: add phase 2g chemistry configuration"
```

### Task 3: Upgrade ResourceGrid To Typed Per-Resource Dynamics

**Files:**
- Modify: `src/core/resources.rs`, `src/core/world.rs`, `src/core/config.rs`
- Test: `tests/phase2g_resource_types.rs`, `tests/phase2g_accounting.rs`

- [ ] **Step 1: Write failing ResourceGrid behavior tests**

Add tests for:

```rust
#[test]
fn different_resource_types_decay_at_different_rates() {}

#[test]
fn diffusion_moves_only_local_amount_and_conserves_each_resource_type() {}

#[test]
fn resource_permeability_is_resolved_by_type_not_layer_position() {}

#[test]
fn diffusion_and_decay_are_deterministic_for_same_snapshot_and_config() {}
```

Use two adjacent cells/patches and assert no cross-type mixing, no negative amounts and per-type conservation except explicitly configured decay sink.

- [ ] **Step 2: Run tests and verify red**

```text
cargo test --test phase2g_resource_types -- --nocapture
cargo test --test phase2g_accounting -- --nocapture
```

- [ ] **Step 3: Implement typed lookup and delta-based updates**

Add `ResourceGrid::amount_at_type`, `set_amount_at_type`, `apply_diffusion_delta` and `apply_decay_delta`. Keep old layer methods as checked adapters for existing Phase 1/2 callers. Use deterministic coordinate/type ordering and a double-buffer or preallocated delta buffer so one update cannot read its own partially committed output.

- [ ] **Step 4: Run focused and regression tests**

```text
cargo test --test phase2g_resource_types -- --nocapture
cargo test --test phase2g_accounting -- --nocapture
cargo test --test phase1_resource_grid -- --nocapture
cargo test --test phase1_resource_interaction -- --nocapture
```

- [ ] **Step 5: Commit typed Resource dynamics**

```text
git add src/core/resources.rs src/core/world.rs src/core/config.rs tests/phase2g_resource_types.rs tests/phase2g_accounting.rs
git commit -m "feat: add typed resource diffusion and decay"
```

### Task 4: Build And Validate ReactionRegistry

**Files:**
- Create: `src/core/reactions.rs`
- Modify: `src/core/config.rs`, `src/core/mod.rs`
- Test: `tests/phase2g_reactions.rs`

- [ ] **Step 1: Write failing reaction definition tests**

Cover:

```rust
#[test]
fn reaction_matches_inputs_conditions_catalyst_rate_and_locality() {}

#[test]
fn passive_reaction_does_not_require_genome_or_action_plan() {}

#[test]
fn controlled_reaction_requires_registered_process_and_feasibility() {}

#[test]
fn reaction_registry_rejects_unknown_types_duplicate_ids_and_invalid_coefficients() {}

#[test]
fn reaction_modes_cover_passive_controlled_degradation_decay_synthesis_and_conversion() {}
```

- [ ] **Step 2: Run the focused reaction tests and verify red**

```text
cargo test --test phase2g_reactions -- --nocapture
```

- [ ] **Step 3: Implement validated reaction value objects**

Define `ReactionMode`, `ReactionId`, `ReactionInput`, `ReactionOutput`, `ReactionCondition`, `CatalystRequirement`, `LocalityRule`, `AccountingDestination` and `ReactionDefinition`. Define `ReactionRegistry::validate` to check ids, typed references, non-negative coefficients, allowed modes, explicit output destinations and configured sink declarations. Keep controlled reaction permission separate from pure definition matching.

- [ ] **Step 4: Implement deterministic reaction candidate matching**

Implement `ReactionRegistry::matching_candidates(snapshot, locality, tick, seed)` with stable reaction-id order. Rate limits amount; probability uses the deterministic sampler defined above. Matching must return a candidate/delta description and never mutate WorldState.

- [ ] **Step 5: Run tests and commit**

```text
cargo test --test phase2g_reactions -- --nocapture
git add src/core/reactions.rs src/core/config.rs src/core/mod.rs tests/phase2g_reactions.rs
git commit -m "feat: add validated reaction registry"
```

### Task 5: Implement Reaction Execution And Matter Accounting

**Files:**
- Modify: `src/core/reactions.rs`, `src/core/deltas.rs`, `src/core/world.rs`
- Test: `tests/phase2g_accounting.rs`, `tests/phase2g_reactions.rs`

- [ ] **Step 1: Write failing execution tests**

Add exact cases:

```rust
#[test]
fn reaction_consumes_inputs_and_produces_bounded_outputs() {}

#[test]
fn reaction_heat_is_emitted_without_direct_energy_buffer_credit() {}

#[test]
fn controlled_conversion_can_credit_energy_only_after_feasibility() {}

#[test]
fn reaction_with_unaccounted_input_is_rejected_before_commit() {}

#[test]
fn reaction_with_product_without_input_is_rejected() {}

#[test]
fn rejected_reaction_has_no_partial_consumption_or_output() {}
```

- [ ] **Step 2: Run tests and verify red**

```text
cargo test --test phase2g_accounting -- --nocapture
cargo test --test phase2g_reactions -- --nocapture
```

- [ ] **Step 3: Implement typed reaction deltas**

Add `ReactionDelta` containing source location, reaction id, typed input consumption, typed outputs, retained material, explicit residual/sink, Heat output and optional controlled Energy output. Add `AccountingReport` with input, accounted destinations and imbalance. Reject the whole delta before commit when any amount is negative, any capacity bound is exceeded or any input is unaccounted.

- [ ] **Step 4: Add deterministic conflict resolution**

Sort deltas by locality, source entity id, reaction id and resource/material id. Resolve competing consumption against the committed available amount; later deltas are rejected with an explicit conflict reason. Never apply partial input/output pairs.

- [ ] **Step 5: Run accounting and full core regressions**

```text
cargo test --test phase2g_accounting -- --nocapture
cargo test --test phase2g_reactions -- --nocapture
cargo test --workspace --all-targets
```

- [ ] **Step 6: Commit reaction execution**

```text
git add src/core/reactions.rs src/core/deltas.rs src/core/world.rs tests/phase2g_accounting.rs tests/phase2g_reactions.rs
git commit -m "feat: execute reactions with explicit matter accounting"
```

### Task 6: Integrate Passive And Controlled Reactions Into Tick

**Files:**
- Modify: `src/core/tick.rs`, `src/core/world.rs`, `src/core/process.rs`, `src/core/summary.rs`
- Test: `tests/phase2g_tick_integration.rs`, `tests/phase2_tick_phases.rs`

- [ ] **Step 1: Write failing phase-order and gating tests**

Add tests that assert:

```rust
#[test]
fn passive_reaction_runs_without_genome_and_is_visible_next_tick() {}

#[test]
fn controlled_reaction_runs_only_after_process_and_feasibility() {}

#[test]
fn passive_reaction_does_not_credit_energy_buffer() {}

#[test]
fn controlled_reaction_failure_leaves_inputs_unchanged() {}

#[test]
fn reaction_commit_order_is_stable_for_multiple_cells_and_reactions() {}
```

- [ ] **Step 2: Run integration tests and verify red**

```text
cargo test --test phase2g_tick_integration -- --nocapture
cargo test --test phase2_tick_phases -- --nocapture
```

- [ ] **Step 3: Add explicit Tick stages**

Use this order in `TickExecutor::step`:

```text
1. rebuild spatial/contact derived state
2. mandatory costs and committed pre-action snapshot
3. passive Resource diffusion/decay and passive reactions -> deltas
4. material/process reflexes and controlled reaction feasibility
5. controlled reaction execution -> deltas
6. local Heat transfer/dissipation and Material degradation -> deltas
7. baseline repair actions -> deltas
8. death/decomposition -> MaterialFragments and explicit conversion reactions
9. physics/lifecycle commit and summary projection
10. advance Tick and expose only committed outputs
```

Route all Phase 2G mutation through World commit methods. Preserve existing Phase 2 local interaction ordering and ensure same-tick generated outputs are not readable by later systems unless the existing Tick contract explicitly permits it.

- [ ] **Step 4: Add diagnostics and summary counters**

Record matched, executed, rejected, conflict-rejected, input, output, Heat, Energy-output, fragment-created and accounting-warning totals. Keep counters observer-only.

- [ ] **Step 5: Run phase-order and regression tests**

```text
cargo test --test phase2g_tick_integration -- --nocapture
cargo test --test phase2_tick_phases -- --nocapture
cargo test --workspace --all-targets
```

- [ ] **Step 6: Commit Tick integration**

```text
git add src/core/tick.rs src/core/world.rs src/core/process.rs src/core/summary.rs tests/phase2g_tick_integration.rs tests/phase2_tick_phases.rs
git commit -m "feat: integrate passive and controlled chemistry into ticks"
```

### Task 7: Add MaterialFragmentStore And Explicit Decomposition

**Files:**
- Create: `src/core/fragments.rs`
- Modify: `src/core/world.rs`, `src/core/tick.rs`, `src/core/events.rs`, `src/core/mod.rs`
- Test: `tests/phase2g_fragments.rs`, `tests/phase2_decomposition_smoke.rs`

- [ ] **Step 1: Write failing fragment tests**

Cover:

```rust
#[test]
fn dead_cell_material_becomes_identity_preserving_fragment() {}

#[test]
fn fragment_does_not_grant_active_cell_capability() {}

#[test]
fn fragment_decay_depends_on_material_type_and_local_heat() {}

#[test]
fn fragment_becomes_resource_only_through_registered_conversion() {}

#[test]
fn decomposition_never_silently_increases_generic_resource_layer() {}

#[test]
fn fragment_creation_and_conversion_are_replayable() {}
```

- [ ] **Step 2: Run tests and verify red**

```text
cargo test --test phase2g_fragments -- --nocapture
cargo test --test phase2_decomposition_smoke -- --nocapture
```

- [ ] **Step 3: Implement FragmentStore**

Add typed `MaterialFragmentId`, dense World-owned storage, deterministic iteration and commit APIs. A fragment stores `material_type_id`, amount, position, stability, damage, decay state and creation tick. Emit explicit fragment-created, fragment-degraded and fragment-converted events.

- [ ] **Step 4: Replace direct dead-material Resource conversion**

Change `WorldState::execute_decomposition_for_dead_cells` so internal Resources follow explicit Resource rules and each externalized Material slot creates a MaterialFragment. Remove the direct `decomposed_mat_sum -> ResourceGrid` path. Add a registered conversion reaction fixture that converts one selected fragment type into `mineral_A` only after degradation conditions match.

- [ ] **Step 5: Run old and new decomposition tests**

```text
cargo test --test phase2g_fragments -- --nocapture
cargo test --test phase2_decomposition_smoke -- --nocapture
cargo test --test phase2_sweep_conservation -- --nocapture
```

- [ ] **Step 6: Commit fragments and decomposition**

```text
git add src/core/fragments.rs src/core/world.rs src/core/tick.rs src/core/events.rs src/core/mod.rs tests/phase2g_fragments.rs tests/phase2_decomposition_smoke.rs
git commit -m "feat: preserve material identity through fragments"
```

### Task 8: Implement Local Heat, Material Degradation And Boundary Baseline

**Files:**
- Create: `src/core/heat.rs`
- Modify: `src/core/cell_store.rs`, `src/core/material_types.rs`, `src/core/tick.rs`, `src/core/world.rs`, `src/core/config.rs`
- Test: `tests/phase2g_heat_boundary_repair.rs`

- [ ] **Step 1: Write failing Heat and Boundary tests**

Add tests for:

```rust
#[test]
fn reaction_heat_changes_temperature_by_heat_capacity() {}

#[test]
fn local_heat_dissipates_toward_ambient_at_configured_rate() {}

#[test]
fn material_over_tolerance_degrades_without_hp_shortcut() {}

#[test]
fn boundary_default_permeability_is_blocked() {}

#[test]
fn boundary_damage_increases_only_physically_compatible_leakage() {}

#[test]
fn missing_boundary_rule_does_not_allow_resource_exchange() {}
```

- [ ] **Step 2: Run tests and verify red**

```text
cargo test --test phase2g_heat_boundary_repair -- --nocapture
```

- [ ] **Step 3: Implement Heat state and commit rules**

Add typed local Heat state to `CellStore` or a dedicated World-owned SoA, with `heat_capacity`, `temperature`, `generated_this_tick` and `dissipated_this_tick`. Apply reaction Heat through capacity; transfer only through contact/local environment mechanisms present in Phase 2G; dissipate only through configured explicit sink. Heat is never written to Energy Buffer.

- [ ] **Step 4: Implement Material degradation and Boundary derivation**

Derive Boundary properties from registry-backed material composition and `MaterialState`. Compute permeability by `ResourceTypeId`, integrity, damage and physical compatibility. Apply leakage only for allowed passive rules or explicit damage-compatible leakage. Do not add a Boundary entity or hardcoded Cell category.

- [ ] **Step 5: Run Heat, Boundary and regression tests**

```text
cargo test --test phase2g_heat_boundary_repair -- --nocapture
cargo test --test phase2_material_profile_gating -- --nocapture
cargo test --test phase2_material_profile_effects -- --nocapture
cargo test --workspace --all-targets
```

- [ ] **Step 6: Commit Heat and Boundary baseline**

```text
git add src/core/heat.rs src/core/cell_store.rs src/core/material_types.rs src/core/tick.rs src/core/world.rs src/core/config.rs tests/phase2g_heat_boundary_repair.rs
git commit -m "feat: add local heat and material boundary degradation"
```

### Task 9: Add Feasibility-Gated Baseline Repair

**Files:**
- Modify: `src/core/process.rs`, `src/core/world.rs`, `src/core/tick.rs`, `src/core/summary.rs`, `src/core/config.rs`
- Test: `tests/phase2g_heat_boundary_repair.rs`, `tests/phase2_process_registry.rs`

- [ ] **Step 1: Write failing repair tests**

Cover:

```rust
#[test]
fn repair_consumes_declared_resource_material_and_energy_inputs() {}

#[test]
fn repair_reduces_material_damage_when_feasibility_passes() {}

#[test]
fn repair_rejects_without_capability_or_required_inputs() {}

#[test]
fn rejected_repair_has_no_partial_consumption() {}

#[test]
fn repair_cannot_restore_material_above_configured_amount_or_capacity() {}
```

- [ ] **Step 2: Run tests and verify red**

```text
cargo test --test phase2g_heat_boundary_repair -- --nocapture
cargo test --test phase2_process_registry -- --nocapture
```

- [ ] **Step 3: Register the repair process**

Add a `RepairBoundary` or equivalent registered `ProcessId` with explicit costs and `MaterialCapability::Repair`. Extend `FeasibilityResult` diagnostics for missing capability, insufficient Resource, insufficient Energy, missing target damage and Boundary-blocked repair.

- [ ] **Step 4: Execute repair through the same delta/commit path**

Do not mutate `MaterialState` during Feasibility. Generate a repair delta only after acceptance; commit inputs and damage reduction atomically. Record rejected and successful repair counts in `MetricsSummary`.

- [ ] **Step 5: Run repair and full core tests**

```text
cargo test --test phase2g_heat_boundary_repair -- --nocapture
cargo test --test phase2_process_registry -- --nocapture
cargo test --workspace --all-targets
```

- [ ] **Step 6: Commit repair baseline**

```text
git add src/core/process.rs src/core/world.rs src/core/tick.rs src/core/summary.rs src/core/config.rs tests/phase2g_heat_boundary_repair.rs tests/phase2_process_registry.rs
git commit -m "feat: add feasibility-gated material repair"
```

### Task 10: Expose Phase 2G Metrics Through The Shared Observer

**Files:**
- Modify: `src/core/summary.rs`, `src/observer/projection.rs`, `src/bin/sweep_analyzer.rs`
- Test: `tests/phase2g_observer_outputs.rs`

- [ ] **Step 1: Write failing shared Observer tests**

Add tests that run the same deterministic fixture through Core summary extraction and `sweep_analyzer` result mapping. Assert identical names, units, tick windows and values for:

```text
reaction_matched_count
reaction_executed_count
reaction_rejected_count
reaction_input_amount
reaction_output_amount
reaction_heat_generated
reaction_energy_output
reaction_accounting_error
resource_diffused_amount
resource_decay_amount
fragment_created_amount
fragment_converted_amount
heat_peak_temperature
material_degradation_amount
boundary_leakage_amount
repair_success_count
repair_rejection_count
```

- [ ] **Step 2: Run tests and verify red**

```text
cargo test --test phase2g_observer_outputs -- --nocapture
```

- [ ] **Step 3: Add typed summary fields and Observer projection mapping**

Extend `RunSummary`/`MetricsSummary` with the Phase 2G fields. Add projection keys with stable snake_case names and source provenance. Keep all fields observer-only; no Core process may read them.

- [ ] **Step 4: Refactor analyzer mapping to consume the shared metrics**

Change `SimResult`, CSV headers and row serialization so Phase 2G values come from Core/Observer summary data. Do not calculate reaction, Heat, fragment or repair values independently in `sweep_analyzer.rs`.

- [ ] **Step 5: Run Observer and existing analyzer tests**

```text
cargo test --test phase2g_observer_outputs -- --nocapture
cargo test --test phase2_sweep_observer_outputs -- --nocapture
cargo test --test phase2_sweep_outputs -- --nocapture
cargo test --workspace --all-targets
```

- [ ] **Step 6: Commit the shared Observer integration**

```text
git add src/core/summary.rs src/observer/projection.rs src/bin/sweep_analyzer.rs tests/phase2g_observer_outputs.rs
git commit -m "feat: expose phase 2g metrics through shared observer"
```

### Task 11: Add Full And Smoke Chemistry Scenarios

**Files:**
- Modify: `config/analyzer/sweep_analyzer.toml`, `config/analyzer/sweep_analyzer_smoke.toml`
- Modify: `config/chemistry/phase2g.toml`, `config/scenarios/chemistry/phase2g.toml`
- Test: `tests/phase2g_sweep_parser.rs`, `tests/phase2g_observer_outputs.rs`

- [ ] **Step 1: Add failing scenario coverage tests**

Assert that both analyzer configs contain these scenarios with all required parameter sweeps:

```text
resource_type_decay_diffusion
material_type_degradation
passive_reaction_viability
controlled_reaction_feasibility
fragment_decomposition_conversion
local_heat_degradation
boundary_retention_leakage
repair_viability
```

The smoke config must use fewer ticks and steps but preserve every scenario and every negative control.

- [ ] **Step 2: Run parser coverage tests and verify red**

```text
cargo test --test phase2g_sweep_parser phase2g -- --nocapture
```

- [ ] **Step 3: Add canonical configs and scenario fixtures**

Configure at least:

```text
nutrient_A: low decay, faster diffusion, nonzero energy_value
waste_A: higher decay, slower diffusion, blocked/default permeability
boundary_polymer_A: higher stability, passive tiny nutrient permeability
structural_polymer_A: lower or different stability and no nutrient permeability
```

Add passive decay, passive conversion, controlled conversion, fragment degradation, Heat stress and repair negative-control fixtures. Every scenario records expected activation and rejection outcomes in comments or adjacent test assertions.

- [ ] **Step 4: Run smoke analyzer and verify artifacts**

```text
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer_smoke.toml
```

Expected files include:

```text
outputs/raw_data/smoke/resource_type_decay_diffusion.csv
outputs/raw_data/smoke/material_type_degradation.csv
outputs/raw_data/smoke/passive_reaction_viability.csv
outputs/raw_data/smoke/controlled_reaction_feasibility.csv
outputs/raw_data/smoke/fragment_decomposition_conversion.csv
outputs/raw_data/smoke/local_heat_degradation.csv
outputs/raw_data/smoke/boundary_retention_leakage.csv
outputs/raw_data/smoke/repair_viability.csv
```

- [ ] **Step 5: Commit scenario coverage**

```text
git add config/chemistry/phase2g.toml config/scenarios/chemistry/phase2g.toml config/analyzer/sweep_analyzer.toml config/analyzer/sweep_analyzer_smoke.toml tests/phase2g_sweep_parser.rs tests/phase2g_observer_outputs.rs
git commit -m "test: add phase 2g chemistry analyzer scenarios"
```

### Task 12: Determinism, Negative Controls, Performance And Gate Report

**Files:**
- Modify: `tests/phase2g_determinism.rs`, `tests/phase2g_accounting.rs`, `tests/phase2g_tick_integration.rs`
- Create: `outputs/worklogs/2026-07-11-2058-REPORT-phase-2G-chemistry-and-matter-dynamics.md`
- Modify: `outputs/worklogs/index.md`

- [ ] **Step 1: Add determinism and negative-control tests**

Add tests for:

```rust
#[test]
fn same_seed_config_and_ticks_reproduce_all_phase2g_metrics() {}

#[test]
fn different_seed_changes_only_probability_sampling_not_accounting_rules() {}

#[test]
fn missing_catalyst_blocks_controlled_reaction() {}

#[test]
fn missing_material_capability_blocks_repair_or_conversion() {}

#[test]
fn passive_reaction_stays_active_when_genome_runtime_is_disabled() {}

#[test]
fn material_fragment_is_not_silently_absorbed_as_resource() {}

#[test]
fn heat_is_not_reported_as_energy_buffer_transfer() {}
```

- [ ] **Step 2: Run the deterministic and negative-control tests**

```text
cargo test --test phase2g_determinism -- --nocapture
cargo test --test phase2g_accounting -- --nocapture
cargo test --test phase2g_tick_integration -- --nocapture
```

- [ ] **Step 3: Run formatting, lint and all tests**

```text
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
```

Expected: all commands exit with code 0.

- [ ] **Step 4: Run smoke and full analyzer checks**

```text
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer_smoke.toml
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml
```

Verify that both runs produce Phase 2G CSVs, observer metric names match Core summaries, accounting warnings are explicit, and no `LOW_INFORMATION_SWEEP` hides a configured negative control. The full run may take several minutes; allow the process to complete before inspecting artifacts.

- [ ] **Step 5: Write the phase report**

Record:

```text
implemented modules and commits
tests and exact commands
full/smoke artifact paths
Resource and Material differential evidence
reaction accounting evidence
fragment identity/conversion evidence
Heat/Boundary/repair evidence
known tool-limited behavior
Phase 2G acceptance gate status
follow-up constraints for Phase 2H and Phase 3
```

- [ ] **Step 6: Update worklog index and commit the report**

```text
git add tests/phase2g_determinism.rs tests/phase2g_accounting.rs tests/phase2g_tick_integration.rs outputs/worklogs/2026-07-11-2058-REPORT-phase-2G-chemistry-and-matter-dynamics.md outputs/worklogs/index.md
git commit -m "docs: report phase 2g chemistry verification"
```

## Acceptance Checklist

The implementation is not complete until every item below is evidenced in tests or generated artifacts:

```text
[ ] two ResourceTypes differ in decay, diffusion or permeability
[ ] two MaterialTypes differ in stability/degradation
[ ] all six ReactionModes are represented in validated definitions
[ ] passive reaction executes without Genome
[ ] controlled reaction requires Process/ActionPlan/Feasibility
[ ] reaction inputs, products, residuals and sinks are explicit
[ ] products without inputs and unaccounted inputs are rejected
[ ] reaction Heat is local and does not directly transfer Energy Buffer
[ ] per-resource diffusion/decay is local and deterministic
[ ] dead Cell Materials become identity-preserving MaterialFragments
[ ] fragments convert to Resources only through explicit reaction/conversion
[ ] local Heat can trigger material degradation by tolerance
[ ] Boundary default is blocked and damage only expands compatible leakage
[ ] repair consumes explicit inputs and can fail
[ ] Core and sweep_analyzer use identical Phase 2G Observer metrics
[ ] full and smoke analyzer configs cover all Phase 2G scenarios
[ ] same seed/config produces identical Phase 2G state and metrics
[ ] cargo fmt, clippy and workspace tests pass
[ ] Phase 1 and Phase 2A-F tests still pass
[ ] Phase 2G report and worklog index entry exist
```

## Self-Review

Coverage mapping:

```text
typed Resource/Material properties       -> Tasks 1-3
Reaction inputs/conditions/catalysts     -> Task 4
rates/probability/locality               -> Tasks 4-5
products/Heat/Energy/accounting          -> Tasks 5-6
passive/controlled/degradation/decay     -> Tasks 4-7
synthesis/conversion/repair              -> Tasks 4, 7 and 9
MaterialFragment identity                -> Task 7
Heat and Material degradation             -> Task 8
Boundary retention/permeability          -> Task 8
shared Observer and analyzer              -> Task 10
smoke/full scenarios                      -> Task 11
determinism and Phase 2 gates            -> Task 12
```

No implementation step relies on an undefined test target or silently moves Chemistry authority into `sweep_analyzer`. The plan intentionally leaves persistent Joints, Genome regulation and organism-level behavior to Phase 2H/3.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
