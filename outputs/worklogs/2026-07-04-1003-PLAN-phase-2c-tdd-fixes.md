---
tags:
  - alife
  - worklog/plan
  - tdd
  - phase/2c
---

# Phase 2C Review Fixes — TDD Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development`
> (recommended) or `superpowers:executing-plans` to implement this plan task-by-task.
> Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Bring Phase 2B/2C implementation in line with the review findings from
`2026-07-03-2136-PLAN-phase-2c-review-fixes.md`, using strict Red-Green-Refactor TDD
with Rust domain modeling constraints.

**Architecture:** Every fix is driven by a failing test that proves the broken invariant,
then minimal production code makes it pass. No production code is written without a red
test first. ALife Canon rules apply: no biological shortcuts, material capabilities
derive only from material amounts, tick phases are deterministic.

**Tech Stack:** Rust 2024 edition, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt`.

---

## File Map

| File | Role | Changes |
|---|---|---|
| `src/core/process.rs` | ProcessSpec registry, FeasibilityResult enrichment | Major additions |
| `src/core/cell_store.rs` | Remove `disabled_capabilities`, add `division_ready` to `RuntimeFlags` | Modify |
| `src/core/world.rs` | Pass accepted amounts through execution, fix growth material targeting | Modify |
| `src/core/tick.rs` | Fix tick phase order, aggregate multi-cell metrics | Modify |
| `src/core/summary.rs` | Add `ProcessDiagnostics` to `RunSummary` | Modify |
| `src/core/config.rs` | Extend `config_hash()` | Modify |
| `src/runner/config_parser.rs` | Reject unknown material names, add legacy flag | Modify |
| `tests/phase2_process_registry.rs` | Registry, FeasibilityResult, diagnostics | **New** |
| `tests/phase2_tick_phases.rs` | Tick order, pressure-before-reflex, division_ready | **New** |
| `tests/phase2_growth_accounting.rs` | Growth targets only structural, no capability leaks | **New** |
| `tests/phase2_multi_cell_fairness.rs` | Aggregate metrics, sequential bias doc | **New** |
| `tests/phase2_config_hash.rs` | config_hash coverage | **New** |
| `tests/phase2_materials_smoke.rs` | Replace `strip_capability_for_test` with material zeroing | Modify |

---

## Task 1 — Format Fix

- [ ] `cargo fmt`
- [ ] `cargo fmt --check` — exit 0
- [ ] `cargo test --workspace --all-targets` — all pass
- [ ] `git commit -m "style: cargo fmt"`

---

## Task 2 — ProcessSpec Registry

**Files:** `src/core/process.rs`, `tests/phase2_process_registry.rs` [NEW]

**Problem:** No static contract per `ProcessId`. New processes can be added without accounting rules.

- [ ] **RED** — create `tests/phase2_process_registry.rs`:

  ```rust
  use alife::core::process::{ProcessId, ProcessSpec, ProcessStatus, MaterialCapability};

  #[test]
  fn test_every_process_id_has_registry_entry() {
      for id in [
          ProcessId::MandatoryUpkeep,
          ProcessId::LocalResourceUptake,
          ProcessId::MetabolismEnergyConversion,
          ProcessId::MaterialSynthesis,
          ProcessId::GrowthResourceAllocation,
          ProcessId::Division,
          ProcessId::ContractileDisplacement,
      ] {
          let spec = ProcessSpec::for_id(id);
          assert_eq!(spec.process_id, id, "Missing registry entry for {:?}", id);
      }
  }

  #[test]
  fn test_division_is_future_status() {
      assert_eq!(ProcessSpec::for_id(ProcessId::Division).status, ProcessStatus::Future);
  }

  #[test]
  fn test_uptake_requires_resource_uptake_capability() {
      assert!(ProcessSpec::for_id(ProcessId::LocalResourceUptake)
          .required_capabilities.contains(&MaterialCapability::ResourceUptake));
  }
  ```

- [ ] **RED verify:** `cargo test --test phase2_process_registry` — FAIL (ProcessSpec not found)

- [ ] **GREEN** — append to `src/core/process.rs`:

  ```rust
  #[derive(Clone, Copy, Debug, PartialEq, Eq)]
  pub enum ProcessStatus { Now, Future }

  #[derive(Clone, Debug)]
  pub struct ProcessSpec {
      pub process_id: ProcessId,
      pub status: ProcessStatus,
      pub required_capabilities: &'static [MaterialCapability],
      pub description: &'static str,
  }

  impl ProcessSpec {
      pub fn for_id(id: ProcessId) -> &'static ProcessSpec {
          PROCESS_REGISTRY.iter().find(|s| s.process_id == id)
              .expect("every ProcessId must have a registry entry")
      }
  }

  static PROCESS_REGISTRY: &[ProcessSpec] = &[
      ProcessSpec { process_id: ProcessId::MandatoryUpkeep, status: ProcessStatus::Now,
          required_capabilities: &[],
          description: "Deducts mandatory energy cost every tick." },
      ProcessSpec { process_id: ProcessId::LocalResourceUptake, status: ProcessStatus::Now,
          required_capabilities: &[MaterialCapability::ResourceUptake],
          description: "Absorbs external resources from local grid cell." },
      ProcessSpec { process_id: ProcessId::MetabolismEnergyConversion, status: ProcessStatus::Now,
          required_capabilities: &[MaterialCapability::Metabolism],
          description: "Converts internal resources to energy." },
      ProcessSpec { process_id: ProcessId::MaterialSynthesis, status: ProcessStatus::Now,
          required_capabilities: &[MaterialCapability::MaterialSynthesis],
          description: "Converts resource+energy into structural material." },
      ProcessSpec { process_id: ProcessId::GrowthResourceAllocation, status: ProcessStatus::Now,
          required_capabilities: &[MaterialCapability::StructuralGrowth],
          description: "Grows cell radius using resource+energy budget." },
      ProcessSpec { process_id: ProcessId::ContractileDisplacement, status: ProcessStatus::Now,
          required_capabilities: &[MaterialCapability::Contractility],
          description: "Displaces cell away from collision neighbors when contact_pressure > 0." },
      ProcessSpec { process_id: ProcessId::Division, status: ProcessStatus::Future,
          required_capabilities: &[],
          description: "Reserved for Phase 2D: splits cell into two daughters." },
  ];
  ```

- [ ] **GREEN verify:** `cargo test --test phase2_process_registry` — 3 PASS
- [ ] `cargo test --workspace --all-targets && cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `git commit -m "feat: add ProcessSpec registry with ProcessStatus"`

---

## Task 3 — Enrich FeasibilityResult with `accepted_amount`

**Files:** `src/core/process.rs`, `src/core/world.rs`, `src/core/tick.rs`,
           extend `tests/phase2_process_registry.rs`

**Problem (`world.rs:212` vs `tick.rs:129`):** Feasibility checks `requested_amount` but execution
independently re-reads `config.*_per_tick` constants. These can diverge.
`execute_growth_for_test` ignores `_action` entirely (`world.rs:303-306`).

- [ ] **RED** — add to `tests/phase2_process_registry.rs`:

  ```rust
  fn minimal_config(density: f32) -> alife::core::config::RuntimeConfig { /* ... */ }

  #[test]
  fn test_feasibility_allowed_carries_accepted_amount() {
      let executor = TickExecutor::new(minimal_config(5.0)).unwrap();
      let candidate = ActionCandidate { process_id: ProcessId::LocalResourceUptake,
          requested_amount: 0.3 };
      match executor.world().validate_feasibility(CellIndex::from_raw(0), &candidate) {
          FeasibilityResult::Allowed { accepted_amount, .. } => {
              assert!(accepted_amount <= 0.3 + f32::EPSILON);
              assert!(accepted_amount > 0.0);
          }
          r => panic!("Expected Allowed, got {:?}", r),
      }
  }

  #[test]
  fn test_feasibility_clamps_to_available_resource() {
      let executor = TickExecutor::new(minimal_config(0.1)).unwrap();
      let candidate = ActionCandidate { process_id: ProcessId::LocalResourceUptake,
          requested_amount: 1.0 };
      match executor.world().validate_feasibility(CellIndex::from_raw(0), &candidate) {
          FeasibilityResult::Allowed { accepted_amount, .. } =>
              assert!(accepted_amount <= 0.1 + f32::EPSILON),
          r => panic!("Expected Allowed, got {:?}", r),
      }
  }
  ```

- [ ] **RED verify:** `cargo test --test phase2_process_registry test_feasibility` — FAIL

- [ ] **GREEN** — in `src/core/process.rs`, replace `FeasibilityResult`:

  ```rust
  #[derive(Clone, Copy, Debug, PartialEq)]
  pub enum FeasibilityResult {
      /// Execution MUST use `accepted_amount`, never re-read config constants.
      Allowed { accepted_amount: f32, energy_cost: f32, resource_cost: f32 },
      Rejected(RejectionReason),
  }
  impl FeasibilityResult {
      pub fn is_feasible(&self) -> bool { matches!(self, Self::Allowed { .. }) }
  }
  ```

- [ ] **GREEN** — in `src/core/world.rs`, update `validate_feasibility` to return `Allowed {..}`:
  - `MandatoryUpkeep`: `Allowed { accepted_amount: 0.0, energy_cost: 0.0, resource_cost: 0.0 }`
  - `LocalResourceUptake`: compute `accepted = min(requested, available_in_grid, free_capacity)` via private helper; return `Allowed { accepted_amount: accepted, .. }`
  - `MetabolismEnergyConversion`: `accepted = min(requested, internal_res)`
  - `MaterialSynthesis`, `GrowthResourceAllocation`, `ContractileDisplacement`: `Allowed { accepted_amount: 1.0, energy_cost: cost_eng, resource_cost: cost_res }`
  - `Division` (Future): `Allowed { accepted_amount: 0.0, .. }`

- [ ] **GREEN** — in `src/core/tick.rs`, change callers to destructure `Allowed`:

  ```rust
  if let FeasibilityResult::Allowed { accepted_amount, energy_cost, resource_cost } =
      self.world.validate_feasibility(index, &candidate_uptake)
  { /* use accepted_amount, not config.* */ }
  ```

- [ ] `cargo test --workspace --all-targets`
- [ ] `git commit -m "feat: FeasibilityResult::Allowed carries accepted_amount; execution uses accepted payload"`

---

## Task 4 — Remove `disabled_capabilities` from CellStore

**Files:** `src/core/cell_store.rs`, test files that call `strip_capability_for_test`

**Problem:** `disabled_capabilities` bitmask + `strip_capability_for_test()` bypass the
material-derived capability rule. Capability must come only from material amounts.

- [ ] **Characterization RED-then-PASS** — add to `tests/phase2_process_registry.rs`:

  ```rust
  #[test]
  fn test_capability_lost_when_material_zeroed() {
      let mut executor = TickExecutor::new(minimal_config(5.0)).unwrap();
      let idx = CellIndex::from_raw(0);
      assert!(executor.world().cells().has_capability(idx, MaterialCapability::ResourceUptake));
      executor.world_mut().cells_mut_for_commit()
          .set_transport_material(idx, MaterialAmount::zero());
      assert!(!executor.world().cells().has_capability(idx, MaterialCapability::ResourceUptake));
  }
  ```

  Run — expect PASS (material zeroing already works, confirms good path).

- [ ] **Find all usages:**

  ```powershell
  grep -rn "strip_capability_for_test" tests/ src/
  ```

- [ ] **Replace each** `strip_capability_for_test(idx, Cap::X)` with the corresponding setter:
  - `Metabolism` → `set_metabolic_material(idx, MaterialAmount::zero())`
  - `ResourceUptake` → `set_transport_material(idx, MaterialAmount::zero())`
  - `MaterialSynthesis` → `set_synthesis_material(idx, MaterialAmount::zero())`
  - `StructuralGrowth` → `set_structural_material(idx, MaterialAmount::zero())`
  - `Contractility` → `set_contractile_material(idx, MaterialAmount::zero())`

- [ ] **Remove from `src/core/cell_store.rs`:**
  - Field `disabled_capabilities: Vec<u16>`
  - `disabled_capabilities: Vec::with_capacity(capacity)` in `with_capacity()`
  - `self.disabled_capabilities.push(0)` in `insert_initial()`
  - Bitmask check block (4 lines) in `has_capability()`
  - `pub fn strip_capability_for_test(...)` method
  - `const fn capability_bit(...)` private function

- [ ] `cargo test --workspace --all-targets && cargo clippy ... -- -D warnings`
- [ ] `git commit -m "refactor: remove disabled_capabilities; capability derives from materials only"`

---

## Task 5 — Fix growth: only structural material increases

**Files:** `src/core/world.rs`, `src/core/tick.rs`, `tests/phase2_growth_accounting.rs` [NEW]

**Problem (`cell_store.rs:328-339`):** `set_materials()` distributes new mass evenly across
all 9 material types, silently granting all capabilities to growing cells.

- [ ] **RED** — create `tests/phase2_growth_accounting.rs`:

  ```rust
  // Cell with only structural_material, no transport/metabolic/contractile

  #[test]
  fn test_growth_increases_only_structural_material() {
      // before/after execute_growth — only structural_material increases
  }

  #[test]
  fn test_growth_increases_radius() { /* ... */ }

  #[test]
  fn test_growth_does_not_grant_contractility() {
      // 5x execute_growth on cell with contractile_material=0 → still no Contractility cap
  }
  ```

- [ ] **RED verify:** `cargo test --test phase2_growth_accounting` — FAIL
  (method named `execute_growth` not found; `set_materials` distributes to all)

- [ ] **GREEN** — in `src/core/world.rs`:
  - Rename `execute_growth_for_test` → `execute_growth`
  - Replace `set_materials(cell_idx, MaterialAmount::new(new_materials).unwrap())` with:

    ```rust
    let old_structural = self.cells.structural_material(cell_idx).raw();
    let new_structural = old_structural + 1.0;
    self.cells.set_structural_material(
        cell_idx, MaterialAmount::new(new_structural).expect("positive"),
    );
    // Use structural as mass proxy for radius scaling:
    let old_mass = old_structural.max(f32::EPSILON);
    let new_mass = new_structural;
    let new_radius_val = old_radius * (new_mass / old_mass).sqrt();
    ```

- [ ] Update call site in `src/core/tick.rs`: `execute_growth_for_test` → `execute_growth`
- [ ] `cargo test --workspace --all-targets`
- [ ] `git commit -m "fix: growth targets only structural_material; rename execute_growth_for_test -> execute_growth"`

---

## Task 6 — Fix tick phase order: contact sensing before reflex

**Files:** `src/core/tick.rs`, `tests/phase2_tick_phases.rs` [NEW]

**Root cause:** Reflex loop (including `ContractileDisplacement`) runs at line ~53.
`contact_pressure` is written by the physics solver at line ~211+.
Reflex always sees zero/stale pressure → displacement never triggers autonomously.

**Required phase order in `step()`:**

```
Phase 0: rebuild_spatial_index()
Phase 1: Contact Sensing — reset pressures, detect overlaps, write contact_pressure
         + Compute division_ready flags (Task 7)
Phase 2: Material Reflex Loop — uptake/metabolism/synthesis/growth/displacement
         (reads pressure from Phase 1)
Phase 3: Physics Solve Pass — Verlet solver resolves remaining overlaps
Phase 4: Lifecycle/accounting commit
```

- [ ] **RED** — create `tests/phase2_tick_phases.rs`:

  ```rust
  #[test]
  fn test_overlapping_contractile_cells_move_without_manual_pressure_injection() {
      // Two cells: position_a=(32,32), position_b=(33,32), radius=2 each → overlap=3
      // NO set_contact_pressure call
      // After 2 ticks: at least one cell must have moved
  }

  #[test]
  fn test_non_overlapping_cells_do_not_move_via_displacement() {
      // Two cells 40 units apart, radius=1 → no overlap → no pressure → no displacement
  }
  ```

- [ ] **RED verify:** `cargo test --test phase2_tick_phases test_overlapping` — FAIL
- [ ] **GREEN** — refactor `step()` in `src/core/tick.rs`:

  Extract contact sensing into a block before the reflex loop:

  ```rust
  // Phase 1: Contact Sensing
  { for i in 0..len { cells.set_contact_pressure(CellIndex::from_raw(i), 0.0); } }
  let mut sense_pairs = Vec::new();
  self.world.spatial_index().generate_candidate_pairs(cells, &mut sense_pairs);
  for (a, b) in &sense_pairs {
      let overlap = compute_overlap(pos_a, pos_b, r_a, r_b);
      if overlap > 0.0 {
          cells.set_contact_pressure(a, cells.contact_pressure(a) + overlap);
          cells.set_contact_pressure(b, cells.contact_pressure(b) + overlap);
      }
  }
  ```

  Remove the `contact_pressure` reset that currently lives inside the physics solver
  (around the current line 215-220 — it will be redundant once Phase 1 owns the reset).

- [ ] Remove manual `set_contact_pressure` calls from existing tests:

  ```powershell
  grep -rn "set_contact_pressure" tests/
  ```

- [ ] `cargo test --workspace --all-targets`
- [ ] `git commit -m "fix: contact sensing pass before material reflex loop; remove manual pressure injection from tests"`

---

## Task 7 — Add `division_ready` to RuntimeFlags

**Files:** `src/core/cell_store.rs`, `src/core/tick.rs`, extend `tests/phase2_tick_phases.rs`

- [ ] **RED** — add to `tests/phase2_tick_phases.rs`:

  ```rust
  #[test]
  fn test_division_ready_false_below_target_radius() {
      // radius=1.0, growth_target_radius=2.0 → after step: division_ready=false
  }

  #[test]
  fn test_division_ready_true_at_target_radius_low_pressure() {
      // radius=3.0, growth_target_radius=2.0, no neighbors → pressure=0 → ready=true
  }
  ```

- [ ] **RED verify:** FAIL — `division_ready` field not on `RuntimeFlags`

- [ ] **GREEN** — in `src/core/cell_store.rs`:

  ```rust
  pub struct RuntimeFlags {
      pub mandatory_paid: bool,
      pub stalled: bool,
      pub over_capacity: bool,
      pub inert: bool,
      pub division_ready: bool,   // NEW
  }
  ```

- [ ] **GREEN** — in `src/core/tick.rs`, after Phase 1 contact sensing:

  ```rust
  for i in 0..len {
      let index = CellIndex::from_raw(i);
      if cells.lifecycle_state(index) == LifecycleState::Dead { continue; }
      let ready = cells.radius(index).raw() >= config.growth.growth_target_radius.raw()
          && cells.contact_pressure(index) <= config.growth.max_division_pressure;
      let mut flags = cells.runtime_flags(index);
      flags.division_ready = ready;
      cells.set_runtime_flags(index, flags);
  }
  ```

- [ ] `cargo test --workspace --all-targets`
- [ ] `git commit -m "feat: add division_ready to RuntimeFlags; computed after contact sensing pass"`

---

## Task 8 — Aggregate multi-cell metrics

**Files:** `src/core/tick.rs`, `tests/phase2_multi_cell_fairness.rs` [NEW]

**Problem (`tick.rs:623-629`):** `build_metrics_summary` reads `resource_amount`,
`used_capacity`, `free_capacity` from `CellIndex::from_raw(0)` only.

- [ ] **RED** — create `tests/phase2_multi_cell_fairness.rs`:

  ```rust
  #[test]
  fn test_two_cell_summary_reports_summed_internal_resources() {
      // cell_a internal_resource=3.0, cell_b=7.0, no uptake
      // summary.metrics.final_internal_resources must be ~10.0
  }
  ```

- [ ] **RED verify:** `cargo test --test phase2_multi_cell_fairness` — FAIL (reports 3.0)

- [ ] **GREEN** — replace cell-0-only block in `build_metrics_summary`:

  ```rust
  let mut final_internal_resources = 0.0_f32;
  let mut final_used_capacity      = 0.0_f32;
  let mut final_free_capacity      = 0.0_f32;
  let mut growth_readiness         = false;
  for i in 0..cells.len() {
      let idx = CellIndex::from_raw(i);
      if cells.lifecycle_state(idx) == LifecycleState::Dead { continue; }
      final_internal_resources += cells.resource_amount(idx).raw();
      final_used_capacity      += cells.used_capacity(idx).raw();
      final_free_capacity      += cells.free_capacity(idx).raw();
      if cells.runtime_flags(idx).division_ready { growth_readiness = true; }
  }
  ```

- [ ] `cargo test --workspace --all-targets`
- [ ] `git commit -m "fix: aggregate multi-cell metrics in RunSummary"`

---

## Task 9 — ProcessDiagnostics

**Files:** `src/core/summary.rs`, `src/core/process.rs`, `src/core/tick.rs`,
           extend `tests/phase2_process_registry.rs`

- [ ] **RED** — add test:

  ```rust
  #[test]
  fn test_diagnostics_records_metabolism_rejection_when_metabolic_material_zero() {
      // set_metabolic_material(idx, zero); step() →
      // summary.diagnostics.rejections_by_process[MetabolismEnergyConversion] > 0
  }
  ```

- [ ] **RED verify:** FAIL — `diagnostics` field not on `RunSummary`

- [ ] **GREEN** — add `Hash` to `RejectionReason` in `src/core/process.rs`
- [ ] **GREEN** — add to `src/core/summary.rs`:

  ```rust
  #[derive(Clone, Debug, Default)]
  pub struct ProcessDiagnostics {
      pub attempts_by_process: HashMap<ProcessId, u32>,
      pub rejections_by_process: HashMap<ProcessId, u32>,
      pub rejections_by_reason: HashMap<RejectionReason, u32>,
  }
  // Add `pub diagnostics: ProcessDiagnostics` to RunSummary
  ```

- [ ] **GREEN** — collect in `tick.rs`: `*diag.attempts_by_process.entry(id).or_insert(0) += 1` on each attempt, `rejections_by_process` and `rejections_by_reason` on each rejection
- [ ] `cargo test --workspace --all-targets`
- [ ] `git commit -m "feat: add ProcessDiagnostics with per-process rejection tracking"`

---

## Task 10 — Tighten material parser

**Files:** `src/runner/config_parser.rs`, `tests/phase2_materials_smoke.rs`

- [ ] **RED** — add:

  ```rust
  #[test]
  fn test_unknown_material_name_fails_parse() { /* unknown_goop → error */ }

  #[test]
  fn test_legacy_flag_allows_generic_material() { /* legacy_material_distribution=true → ok */ }
  ```

- [ ] **RED verify:** FAIL — unknown name currently triggers silent fallback

- [ ] **GREEN** — add explicit allowlist to `config_parser.rs`:

  ```rust
  fn resolve_material_name(name: &str, legacy: bool) -> Result<MaterialTarget, ParseError> {
      match name {
          "boundary" | "cell_wall"       => Ok(MaterialTarget::Boundary),
          "transport"                    => Ok(MaterialTarget::Transport),
          "metabolic" | "metabolism"     => Ok(MaterialTarget::Metabolic),
          "storage"                      => Ok(MaterialTarget::Storage),
          "synthesis"                    => Ok(MaterialTarget::Synthesis),
          "structural"                   => Ok(MaterialTarget::Structural),
          "repair"                       => Ok(MaterialTarget::Repair),
          "contractile" | "contractility"=> Ok(MaterialTarget::Contractile),
          "sensory" | "sensory_material" => Ok(MaterialTarget::Sensory),
          other if legacy                => Ok(MaterialTarget::LegacyAll(other.to_string())),
          other => Err(ParseError::UnknownMaterialName(other.to_string())),
      }
  }
  ```

  Read `legacy_material_distribution` flag from TOML before parsing materials.

- [ ] `cargo test --workspace --all-targets`
- [ ] `git commit -m "fix: unknown material names fail parse; require legacy_material_distribution = true for generic distribution"`

---

## Task 11 — Expand `config_hash` coverage

**Files:** `src/core/config.rs`, `tests/phase2_config_hash.rs` [NEW]

**Missing from current hash (`config.rs:269-349`):** world size, `spatial_grid_size`,
`physics_solver_iterations`, cell position, cell radius, lifecycle thresholds,
environment thresholds, `growth_enabled`.

- [ ] **RED** — create `tests/phase2_config_hash.rs` with assertions:
  - `changing_cell_position_changes_hash`
  - `changing_cell_radius_changes_hash`
  - `changing_physics_iterations_changes_hash`
  - `changing_lifecycle_threshold_changes_hash`
  - `changing_growth_enabled_changes_hash`
  - `changing_world_size_changes_hash`

- [ ] **RED verify:** several FAIL

- [ ] **GREEN** — add to the scalar loop in `config_hash()`:

  ```rust
  // World geometry
  self.world.size.x().to_bits() as u64,
  self.world.size.y().to_bits() as u64,
  self.space.spatial_grid_size.to_bits() as u64,
  self.space.physics_solver_iterations as u64,
  // Primary cell
  self.cell.position.x.to_bits() as u64,
  self.cell.position.y.to_bits() as u64,
  self.cell.radius.raw().to_bits() as u64,
  // Lifecycle
  self.lifecycle.stress_energy_threshold.raw().to_bits() as u64,
  self.lifecycle.dormancy_allowed as u64,
  self.lifecycle.dormant_mandatory_cost_modifier.to_bits() as u64,
  self.lifecycle.critical_capacity_overrun.raw().to_bits() as u64,
  // Environment
  self.environment.heat_dissipation_rate.raw().to_bits() as u64,
  self.environment.heat_warning_threshold.raw().to_bits() as u64,
  self.environment.heat_death_threshold.raw().to_bits() as u64,
  self.environment.waste_sink_rate.raw().to_bits() as u64,
  self.environment.waste_warning_threshold.raw().to_bits() as u64,
  self.environment.waste_death_threshold.raw().to_bits() as u64,
  // Flags
  self.growth_enabled as u64,
  ```

  Also fold `position` and `radius` from each entry in `initial_cells`.

- [ ] `cargo test --workspace --all-targets`
- [ ] `git commit -m "fix: extend config_hash to cover position, radius, physics, lifecycle, environment, growth_enabled"`

---

## Task 12 — Document sequential resource allocation bias

**Files:** `tests/phase2_multi_cell_fairness.rs`

Per the review, keep sequential allocation for Phase 2C (Option B). Assert bounded bias
and add an `#[ignore]` gate required before Phase 2D division scale tests.

- [ ] **Add to `tests/phase2_multi_cell_fairness.rs`:**

  ```rust
  #[test]
  fn test_sequential_bias_is_bounded_and_documented() {
      // Two identical cells, same grid tile, 1.5 units available, max_uptake=1.0 each
      // Sequential: cell 0 takes 1.0, cell 1 takes 0.5
      // Assert: res_a + res_b ≈ 1.5 (conservation)
      // Assert: res_a >= res_b - 0.01 (known sequential bias)
  }

  #[test]
  #[ignore = "requires proportional resource allocation — must be implemented before Phase 2D division scale tests"]
  fn test_equal_uptake_with_fair_allocation() {
      // When proportional allocation is implemented, identical cells must receive equal shares.
      // Implementation guide: collect all requests per grid tile, distribute proportionally,
      // commit after all shares computed.
      todo!()
  }
  ```

- [ ] Run — `test_sequential_bias_is_bounded_and_documented` PASS, ignored test SKIPPED
- [ ] `git commit -m "test: document sequential resource allocation bias; add ignored Phase 2D gate"`

---

## Task 13 — Final Verification

- [ ] `cargo test --workspace --all-targets 2>&1`
  - Expected: all tests PASS; 1 IGNORED (`test_equal_uptake_with_fair_allocation`)
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings 2>&1` — exit 0
- [ ] `cargo fmt --check 2>&1` — exit 0
- [ ] `git commit -m "chore: Phase 2C review fixes complete — all acceptance gates satisfied"`

---

## Acceptance Gates

- [ ] Every `ProcessId` has a `ProcessSpec` registry entry with `ProcessStatus`
- [ ] `FeasibilityResult::Allowed` carries `accepted_amount`; tick.rs uses it, not config constants
- [ ] `disabled_capabilities` removed; `strip_capability_for_test` removed; tests use material zeroing
- [ ] Growth increases only `structural_material`; non-structural capabilities unchanged after 5× growth
- [ ] Contact sensing pass runs **before** material reflex loop; no test manually sets `contact_pressure`
- [ ] `RuntimeFlags.division_ready` committed after contact sensing; tested below/at/above target radius
- [ ] `RunSummary.metrics` aggregates across all living cells
- [ ] `ProcessDiagnostics` in `RunSummary` with `rejections_by_process` populated
- [ ] Unknown material names fail parse; `legacy_material_distribution = true` allows legacy
- [ ] `config_hash()` changes on: position, radius, physics iterations, lifecycle thresholds, `growth_enabled`, world size
- [ ] Sequential bias documented; `#[ignore]` gate added for Phase 2D
- [ ] `cargo fmt --check`, `cargo clippy ... -D warnings`, `cargo test --workspace` all pass

## Out of Scope

- Daughter-cell creation (Phase 2D)
- Death/decomposition (Phase 2E)
- Joints, Genome Runtime, viewer changes

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
