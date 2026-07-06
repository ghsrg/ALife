# Phase 2B Process Registry and Feasibility Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace Phase 1 hardcoded simulation updates with a flexible, material-capability-driven process registry and explicit feasibility validation checks.

**Architecture:** Cells store their materials in a structured inventory. Each material type has associated capability flags (uptake, metabolism, growth, etc.). Actions (uptake, metabolism, upkeep) are generated as `ActionCandidate`s, verified against material capabilities and resources in a `Feasibility` system, and executed deterministically if approved.

**Tech Stack:** Rust 2024, Cargo integration tests.

---

## File Structure

Modify:
- [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs): Add material types registry config and default mappings.
- [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs): Upgrade cell material inventory to map multiple material types and query capabilities.
- [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs): Refactor `TickExecutor::step` to drive the tick via candidate generation, feasibility validation, and process execution.

Create:
- [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs): Define `MaterialCapability`, `ProcessId`, `ProcessRegistry`, `ActionCandidate`, `Feasibility`, and `RejectionReason`.
- [phase2_process_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_process_smoke.rs): Integration test suite verifying process execution, feasibility rejection, and compatibility.

---

## Task 1: Add Material Registry & Capability Definitions

**Files:**
- Modify: [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs)
- Create: [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs)

- [ ] **Step 1: Write failing config test**
Write a test in [tests/phase2_process_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_process_smoke.rs) verifying that we can define material types with specific capability flags.

```rust
use alife::core::process::{MaterialCapability, MaterialCapabilityFlags};

#[test]
fn material_capabilities_flags_work() {
    let flags = MaterialCapabilityFlags {
        boundary_permeability: true,
        resource_uptake: true,
        metabolism: false,
        structural_growth: false,
        storage_capacity: true,
        repair: false,
    };
    assert!(flags.has(MaterialCapability::BoundaryPermeability));
    assert!(!flags.has(MaterialCapability::Metabolism));
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_process_smoke material_capabilities_flags_work`
Expected: Compilation failure because `process.rs` does not exist.

- [ ] **Step 3: Implement MaterialCapability and flags**
Create [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs):

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaterialCapability {
    BoundaryPermeability,
    ResourceUptake,
    Metabolism,
    StructuralGrowth,
    StorageCapacity,
    Repair,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MaterialCapabilityFlags {
    pub boundary_permeability: bool,
    pub resource_uptake: bool,
    pub metabolism: bool,
    pub structural_growth: bool,
    pub storage_capacity: bool,
    pub repair: bool,
}

impl MaterialCapabilityFlags {
    pub const fn has(&self, capability: MaterialCapability) -> bool {
        match capability {
            MaterialCapability::BoundaryPermeability => self.boundary_permeability,
            MaterialCapability::ResourceUptake => self.resource_uptake,
            MaterialCapability::Metabolism => self.metabolism,
            MaterialCapability::StructuralGrowth => self.structural_growth,
            MaterialCapability::StorageCapacity => self.storage_capacity,
            MaterialCapability::Repair => self.repair,
        }
    }
}
```

In [src/core/lib.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/lib.rs), register the `process` module:
```rust
pub mod process;
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_process_smoke material_capabilities_flags_work`
Expected: PASS

---

## Task 2: Upgrade Cell Inventory to Support Multi-Material Capabilities

**Files:**
- Modify: [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs)
- Modify: [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs)

- [ ] **Step 1: Write failing inventory capability test**
Add a test in [tests/phase2_process_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_process_smoke.rs) that updates a cell's material inventory and asserts whether it has specific capabilities.

```rust
use alife::core::cell_store::{CellStore, InitialCellState, EnergyBuffer};
use alife::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
};

#[test]
fn cell_inventory_queries_capabilities_based_on_material_amounts() {
    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(EnergyAmount::new(5.0).unwrap(), EnergyAmount::new(10.0).unwrap()),
        resources: ResourceAmount::zero(),
        materials: MaterialAmount::new(5.0).unwrap(),
        capacity_limit: CapacityAmount::new(10.0).unwrap(),
        temperature: Temperature::new(25.0),
    });

    // Expect the default single-cell initialization to assign a material with all capabilities
    let idx = alife::core::cell_store::CellIndex::from_raw(0);
    assert!(cells.has_capability(idx, MaterialCapability::Metabolism));
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_process_smoke cell_inventory_queries_capabilities_based_on_material_amounts`
Expected: Compilation failure (missing `has_capability` on `CellStore`).

- [ ] **Step 3: Modify CellStore and config default material properties**
Update [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs):
Add `has_capability` method. Since Phase 2 keeps a flat `MaterialAmount` in `CellStore` representing the sum of cell materials, for backward compatibility we can treat this flat amount as a default material type that has all capabilities (or read them from the config).

```rust
use crate::core::process::{MaterialCapability, MaterialCapabilityFlags};

impl CellStore {
    pub fn has_capability(&self, index: CellIndex, capability: MaterialCapability) -> bool {
        // If the cell is dead, it has no capabilities.
        if self.lifecycle_state(index) == LifecycleState::Dead {
            return false;
        }
        // In Phase 2, if the cell has material amount > 0, it has all default capabilities.
        let amount = self.materials[index.raw()];
        if amount.raw() > 0.0 {
            // Default material has all capabilities
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
}
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_process_smoke cell_inventory_queries_capabilities_based_on_material_amounts`
Expected: PASS

---

## Task 3: Implement Process Registry & Feasibility Validation

**Files:**
- Modify: [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs)

- [ ] **Step 1: Write failing feasibility test**
Add a test in [tests/phase2_process_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_process_smoke.rs) that builds feasibility inputs and validates different processes.

```rust
use alife::core::process::{ActionCandidate, FeasibilityInput, FeasibilityResult, ProcessId, RejectionReason};

#[test]
fn feasibility_validates_uptake_and_metabolism() {
    // Setup minimal configuration
    let base = base_test_config();
    let mut exec = alife::core::tick::TickExecutor::new(base).unwrap();
    let idx = alife::core::cell_store::CellIndex::from_raw(0);

    // 1. Uptake candidate when free capacity exists
    let candidate_uptake = ActionCandidate {
        process_id: ProcessId::LocalResourceUptake,
        requested_amount: 1.0,
    };
    let result = exec.world().validate_feasibility(idx, &candidate_uptake);
    assert!(matches!(result, FeasibilityResult::Feasible));

    // 2. Metabolism candidate when internal resources exist
    let candidate_metabolism = ActionCandidate {
        process_id: ProcessId::MetabolismEnergyConversion,
        requested_amount: 1.0,
    };
    let result = exec.world().validate_feasibility(idx, &candidate_metabolism);
    // Should be rejected since initial cell has 0 resources
    assert!(matches!(result, FeasibilityResult::Rejected(RejectionReason::InsufficientResources)));
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_process_smoke feasibility_validates_uptake_and_metabolism`
Expected: Compilation failure due to missing `ProcessId`, `ActionCandidate`, `validate_feasibility`, etc.

- [ ] **Step 3: Define Process IDs and Feasibility logic**
In [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs):

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProcessId {
    MandatoryUpkeep,
    LocalResourceUptake,
    MetabolismEnergyConversion,
    MaterialSynthesis,
    GrowthResourceAllocation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActionCandidate {
    pub process_id: ProcessId,
    pub requested_amount: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RejectionReason {
    MissingCapability(MaterialCapability),
    InsufficientResources,
    InsufficientEnergy,
    InsufficientCapacity,
    LifecycleStateDead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeasibilityResult {
    Feasible,
    Rejected(RejectionReason),
}

pub struct FeasibilityInput<'a> {
    pub cell_idx: crate::core::cell_store::CellIndex,
    pub cells: &'a crate::core::cell_store::CellStore,
    pub resource_interaction: &'a crate::core::config::ResourceInteractionConfig,
}

impl FeasibilityResult {
    pub fn is_feasible(&self) -> bool {
        matches!(self, Self::Feasible)
    }
}
```

In [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs), implement `validate_feasibility`:
```rust
use crate::core::process::{ActionCandidate, FeasibilityResult, ProcessId, MaterialCapability, RejectionReason};

impl WorldState {
    pub fn validate_feasibility(&self, cell_idx: CellIndex, action: &ActionCandidate) -> FeasibilityResult {
        if self.cells.lifecycle_state(cell_idx) == LifecycleState::Dead {
            return FeasibilityResult::Rejected(RejectionReason::LifecycleStateDead);
        }

        match action.process_id {
            ProcessId::MandatoryUpkeep => {
                // Must have energy to pay
                let current_energy = self.cells.energy_amount(cell_idx);
                if current_energy.raw() <= 0.0 {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
                } else {
                    FeasibilityResult::Feasible
                }
            }
            ProcessId::LocalResourceUptake => {
                if !self.cells.has_capability(cell_idx, MaterialCapability::ResourceUptake) {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(MaterialCapability::ResourceUptake));
                }
                // Check capacity
                let free_capacity = self.cells.free_capacity(cell_idx);
                if free_capacity.raw() <= 0.0 {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientCapacity)
                } else {
                    FeasibilityResult::Feasible
                }
            }
            ProcessId::MetabolismEnergyConversion => {
                if !self.cells.has_capability(cell_idx, MaterialCapability::Metabolism) {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(MaterialCapability::Metabolism));
                }
                // Check internal resource amount
                let internal_res = self.cells.resource_amount(cell_idx);
                if internal_res.raw() < action.requested_amount {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientResources)
                } else {
                    FeasibilityResult::Feasible
                }
            }
            ProcessId::MaterialSynthesis | ProcessId::GrowthResourceAllocation => {
                FeasibilityResult::Feasible
            }
        }
    }
}
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_process_smoke feasibility_validates_uptake_and_metabolism`
Expected: PASS

---

## Task 4: Integrate Process Registry in TickExecutor

**Files:**
- Modify: [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs)

- [ ] **Step 1: Write failing simulation test**
Add a test verifying that processes are driven and executed during `step()`.

```rust
#[test]
fn tick_executor_drives_uptake_and_metabolism_via_processes() {
    let mut config = base_test_config();
    config.resource_interaction.enabled = true;
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(2.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.energy_per_resource = 3.0;

    let mut exec = TickExecutor::new(config).unwrap();
    // Pre-populate resources on grid cell of Cell 0
    let cell_pos = exec.world().cells().position(CellIndex::from_raw(0));
    exec.world_mut().resources_mut().add_resource(0, cell_pos, ResourceAmount::new(10.0).unwrap());

    // Step 1: Uptake local resource
    let summary = exec.step().unwrap();
    assert_eq!(summary.survival_result, SurvivalResult::Stable);

    let cell_res = exec.world().cells().resource_amount(CellIndex::from_raw(0));
    assert!(cell_res.raw() > 0.0); // Uptake executed.
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_process_smoke tick_executor_drives_uptake_and_metabolism_via_processes`
Expected: FAIL (either doesn't compile or does not execute the uptake).

- [ ] **Step 3: Refactor TickExecutor step to use processes**
In [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs), refactor the uptake and metabolism phase to generate `ActionCandidate`s, validate them using `validate_feasibility`, and execute them if valid.

```rust
        // Rebuild Spatial Index at the start of tick
        self.world.rebuild_spatial_index();

        let mut metabolism_heat_total = 0.0_f32;
        let mut metabolism_waste_total = 0.0_f32;

        // Phase A: Uptake and Metabolism via Process Registry
        if config.resource_interaction.enabled {
            for i in 0..len {
                let idx = CellIndex::from_raw(i);
                if self.world.cells().lifecycle_state(idx) == LifecycleState::Dead {
                    continue;
                }

                // 1. Local Resource Uptake
                let candidate_uptake = ActionCandidate {
                    process_id: ProcessId::LocalResourceUptake,
                    requested_amount: config.resource_interaction.max_uptake_per_tick.raw(),
                };
                if self.world.validate_feasibility(idx, &candidate_uptake).is_feasible() {
                    let pos = self.world.cells().position(idx);
                    let layer = config.resource_interaction.uptake_layer_index;
                    let free_cap = self.world.cells().free_capacity(idx).raw();
                    let max_uptake = config.resource_interaction.max_uptake_per_tick.raw().min(free_cap);

                    let taken = self.world.resources_mut().extract_resource(layer, pos, ResourceAmount::new(max_uptake).unwrap());
                    if taken.raw() > 0.0 {
                        self.world.cells_mut_for_commit().add_resources(idx, taken);
                    }
                }

                // 2. Metabolism Energy Conversion
                let candidate_metabolism = ActionCandidate {
                    process_id: ProcessId::MetabolismEnergyConversion,
                    requested_amount: config.resource_interaction.metabolism_resource_per_tick.raw(),
                };
                if self.world.validate_feasibility(idx, &candidate_metabolism).is_feasible() {
                    let rate = config.resource_interaction.metabolism_resource_per_tick;
                    let consumed = self.world.cells_mut_for_commit().consume_resources(idx, rate);
                    if consumed.raw() > 0.0 {
                        let energy_gen = consumed.raw() * config.resource_interaction.energy_per_resource;
                        let heat_gen = consumed.raw() * config.resource_interaction.heat_per_resource;
                        let waste_gen = consumed.raw() * config.resource_interaction.waste_per_resource;

                        let current_buf = self.world.cells().energy_buffer(idx);
                        let next_energy = EnergyAmount::new(current_buf.current().raw() + energy_gen).unwrap();
                        self.world.cells_mut_for_commit().set_energy(idx, EnergyBuffer::new(next_energy, current_buf.capacity()));

                        metabolism_heat_total += heat_gen;
                        metabolism_waste_total += waste_gen;
                    }
                }
            }
        }
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_process_smoke tick_executor_drives_uptake_and_metabolism_via_processes`
Expected: PASS

---

## Task 5: Final Review & Integration Verification

**Files:**
- Modify: [phase2_process_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_process_smoke.rs)

- [ ] **Step 1: Run full test suite**
Run: `cargo test`
Expected: PASS (All 61 existing + new tests pass).

- [ ] **Step 2: Validate code quality and lints**
Run:
```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
Expected: PASS

- [ ] **Step 3: Document worklog**
Document changes in worklog. Do not commit.

---

## Acceptance Check

Phase 2B is complete when:
- Material capabilities are modeled as flags.
- Processes (uptake, metabolism) are registered and validated via `FeasibilityResult`.
- All legacy Phase 1 survival/reachability integration tests pass.
- Linter and formatter checks are green.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
