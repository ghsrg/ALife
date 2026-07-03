# Phase 2B Mechanism Reachability Verification Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend Rust-side Mechanism Reachability verification to cover Phase 2 material capabilities, process registries, and feasibility rejection diagnostics, ensuring the stability and usability of the new process-driven structure.

**Architecture:** Integrate process attempt/rejection count metrics inside `MetricsSummary` to expose feasibility diagnostics. Add a dedicated reachability integration test suite verifying that enabling/disabling specific material capabilities (like `Metabolism` or `ResourceUptake`) correctly drives cell survival or deterministic collapse.

**Tech Stack:** Rust 2024, Cargo integration tests.

---

## File Structure

Modify:
- [summary.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/summary.rs): Add process attempts and rejections metrics to `MetricsSummary`.
- [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs): Increment process metrics during candidate generation and feasibility validation.

Create:
- [phase2_reachability.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_reachability.rs): Integration test suite verifying survival and collapse reachability under gated material capabilities.

---

## Task 1: Add Feasibility Diagnostic Metrics to MetricsSummary

**Files:**
- Modify: [summary.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/summary.rs)
- Modify: [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs)

- [ ] **Step 1: Write failing metrics test**
Add a test in [tests/phase2_reachability.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_reachability.rs) verifying that tick execution records process attempts and rejections.

```rust
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Temperature, Tick, WasteAmount, WorldSize,
};
use alife::core::tick::TickExecutor;

fn base_test_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(100),
            seed: Seed::from_raw(42),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        CellInitialConfig {
            position: Position::new(1.0, 1.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(5.0).unwrap(),
            energy_capacity: EnergyAmount::new(10.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::zero(),
            capacity_limit: CapacityAmount::new(20.0).unwrap(),
            initial_resource_amount: ResourceAmount::zero(),
            initial_material_amount: MaterialAmount::new(4.0).unwrap(),
        },
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(10.0).unwrap(),
            heat_death_threshold: HeatAmount::new(20.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(10.0).unwrap(),
            waste_death_threshold: WasteAmount::new(20.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(2.0).unwrap(),
            dormancy_allowed: true,
            dormant_mandatory_cost_modifier: 0.25,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap()
}

#[test]
fn tick_executor_records_process_attempts_and_rejections() {
    let mut config = base_test_config();
    config.resource_interaction.enabled = true;
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(2.0).unwrap();
    // Initial cell has 0 resources, so metabolism conversion should fail validation.
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    let summary = exec.step().unwrap();

    assert!(summary.metrics.process_attempts > 0);
    assert!(summary.metrics.process_rejections > 0);
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_reachability tick_executor_records_process_attempts_and_rejections`
Expected: Compilation failure because `process_attempts` does not exist on `MetricsSummary`.

- [ ] **Step 3: Add metrics to summary.rs**
In [summary.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/summary.rs), add fields:
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MetricsSummary {
    // ...
    pub process_attempts: u32,
    pub process_rejections: u32,
}
```
Update all mock instantiations of `MetricsSummary` in existing tests to include `process_attempts: 0` and `process_rejections: 0`.

- [ ] **Step 4: Accumulate metrics in tick.rs**
In [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs), update the uptake and metabolism process validation to increment attempts and rejections.
```rust
        let mut process_attempts = 0;
        let mut process_rejections = 0;

        if config.resource_interaction.enabled {
            for i in 0..len {
                let idx = CellIndex::from_raw(i);
                if self.world.cells().lifecycle_state(idx) == LifecycleState::Dead {
                    continue;
                }

                // Uptake
                let candidate_uptake = ActionCandidate {
                    process_id: ProcessId::LocalResourceUptake,
                    requested_amount: config.resource_interaction.max_uptake_per_tick.raw(),
                };
                process_attempts += 1;
                let uptake_res = self.world.validate_feasibility(idx, &candidate_uptake);
                if uptake_res.is_feasible() {
                    // ... (existing execution) ...
                } else {
                    process_rejections += 1;
                }

                // Metabolism
                let candidate_metabolism = ActionCandidate {
                    process_id: ProcessId::MetabolismEnergyConversion,
                    requested_amount: config.resource_interaction.metabolism_resource_per_tick.raw(),
                };
                process_attempts += 1;
                let metab_res = self.world.validate_feasibility(idx, &candidate_metabolism);
                if metab_res.is_feasible() {
                    // ... (existing execution) ...
                } else {
                    process_rejections += 1;
                }
            }
        }
```
Pass `process_attempts` and `process_rejections` into `build_metrics_summary()`.

- [ ] **Step 5: Run test to verify it passes**
Run: `cargo test --test phase2_reachability tick_executor_records_process_attempts_and_rejections`
Expected: PASS

---

## Task 2: Implement Reachability Tests for Gated Material Capabilities

**Files:**
- Modify: [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs)
- Modify: [phase2_reachability.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_reachability.rs)

- [ ] **Step 1: Write failing capability gating tests**
Add tests verifying that survival is reachable if capabilities exist, and cell collapse occurs if capabilities are missing.

```rust
use alife::core::cell_store::CellIndex;
use alife::core::summary::SurvivalResult;
use alife::core::process::MaterialCapability;

#[test]
fn cell_collapses_if_metabolism_capability_is_missing() {
    let mut config = base_test_config();
    config.resource_interaction.enabled = true;
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(2.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.cell.initial_resource_amount = ResourceAmount::new(10.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    
    // Explicitly strip Metabolism capability from Cell 0 in the test
    exec.world_mut().cells_mut_for_commit().strip_capability_for_test(
        CellIndex::from_raw(0), 
        MaterialCapability::Metabolism
    );

    let summary = exec.run_until_configured_tick().unwrap();
    // Cell must collapse because it cannot perform metabolism without the capability
    assert_eq!(summary.survival_result, SurvivalResult::Collapsed);
}

#[test]
fn cell_collapses_if_resource_uptake_capability_is_missing() {
    let mut config = base_test_config();
    config.resource_interaction.enabled = true;
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(2.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    
    // Populate resource grid cell of Cell 0
    let cell_pos = exec.world().cells().position(CellIndex::from_raw(0));
    exec.world_mut().resources_mut().add_resource(0, cell_pos, ResourceAmount::new(20.0).unwrap());

    // Strip ResourceUptake capability
    exec.world_mut().cells_mut_for_commit().strip_capability_for_test(
        CellIndex::from_raw(0), 
        MaterialCapability::ResourceUptake
    );

    let summary = exec.run_until_configured_tick().unwrap();
    assert_eq!(summary.survival_result, SurvivalResult::Collapsed);
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_reachability`
Expected: Compilation failure because `strip_capability_for_test` does not exist on `CellStore`.

- [ ] **Step 3: Implement strip_capability_for_test on CellStore**
Modify [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs) to store specific disabled capabilities for testing purposes, or configure them using a small flag mask.
We can add a `disabled_capabilities: Vec<Vec<MaterialCapability>>` (or a bitmask `Vec<u16>`) field in `CellStore`.

In `CellStore`:
```rust
#[derive(Clone, Debug, Default)]
pub struct CellStore {
    // ... (existing fields) ...
    disabled_capabilities: Vec<u16>,
}
```
In `with_capacity`:
```rust
        Self {
            // ... (existing fields) ...
            disabled_capabilities: Vec::with_capacity(capacity),
        }
```
In `insert_initial`:
```rust
        self.disabled_capabilities.push(0);
```

Implement bitmask mapping for `MaterialCapability`:
```rust
impl MaterialCapability {
    pub const fn to_mask(self) -> u16 {
        match self {
            Self::BoundaryPermeability => 1 << 0,
            Self::ResourceUptake => 1 << 1,
            Self::Metabolism => 1 << 2,
            Self::StructuralGrowth => 1 << 3,
            Self::StorageCapacity => 1 << 4,
            Self::Repair => 1 << 5,
        }
    }
}
```

Update `has_capability`:
```rust
    pub fn has_capability(&self, index: CellIndex, capability: MaterialCapability) -> bool {
        if self.lifecycle_state(index) == LifecycleState::Dead {
            return false;
        }
        let mask = self.disabled_capabilities[index.raw()];
        if (mask & capability.to_mask()) != 0 {
            return false;
        }

        let amount = self.materials[index.raw()];
        if amount.raw() > 0.0 {
            let default_flags = MaterialCapabilityFlags {
                boundary_permeability: true,
                resource_uptake: true,
                metabolism: true,
                structural_growth: true,
                storage_capacity: true,
                repair: true,
            };
            default_flags.has(capability)
        } else {
            false
        }
    }

    pub fn strip_capability_for_test(&mut self, index: CellIndex, capability: MaterialCapability) {
        self.disabled_capabilities[index.raw()] |= capability.to_mask();
    }
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_reachability`
Expected: PASS

---

## Task 3: Final Verification & Reachability Report

- [ ] **Step 1: Run complete test suite**
Run: `cargo test`
Expected: PASS

- [ ] **Step 2: Linter & Formatter validation**
Run:
```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
Expected: PASS

- [ ] **Step 3: Document reachability worklog**
Generate the walkthrough and report summarizing the findings.

---

## Acceptance Check
- Capability gating successfully drives survival or collapse.
- Process metrics (attempts and rejections) are tracked.
- All integration tests pass cleanly.
