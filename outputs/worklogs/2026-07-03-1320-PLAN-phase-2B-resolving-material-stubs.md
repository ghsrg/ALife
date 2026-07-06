# Phase 2B Resolving Material Stubs Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Eliminate material and capability stubs/placeholders by implementing 9 distinct material types in `CellStore` and mapping them 1-to-1 to 11 capabilities while maintaining full backwards compatibility with legacy scenarios.

**Architecture:**
- **Materials**: Define 9 separate SoA arrays inside `CellStore` representing the recommended material inventory (boundary, transport, metabolic, storage, synthesis, structural, repair, contractile, sensory).
- **Capabilities**: Update `CellStore::has_capability` to check the corresponding material amount $> 0.0$ (e.g. `Metabolism` requires metabolic material, `ResourceUptake` requires transport material, etc.).
- **Legacy Compatibility**: Update the parser in `config_parser.rs`. If a legacy TOML scenario defines general materials (like `cell_wall` or sums) but lacks specific ones, distribute the sum equally among all 9 materials so that all capabilities are preserved, keeping Phase 1/2 tests completely stable.

**Tech Stack:** Rust 2024, Cargo integration tests.

---

## File Structure

Modify:
- [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs): Replace `initial_material_amount` in `CellInitialConfig` with 9 specific material amounts. Update builders and `config_hash()`.
- [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs): Map parsed TOML materials to the 9 specific fields. Distribute legacy/unspecified material amounts.
- [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs): Upgrade SoA vectors to store all 9 material types separately. Expose getters/mutators. Implement capability mapping in `has_capability`.
- [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs): Update `execute_growth_for_test` to increment structural material and scale radius by total material mass.
- [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs): Add all 11 capabilities to `MaterialCapability` and `MaterialCapabilityFlags`.

Create:
- [phase2_materials_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_materials_smoke.rs): Integration test suite verifying multi-material parser mapping, capability checks, and legacy scenario compatibility.

---

## Task 1: Extend Capabilities Registry

**Files:**
- Modify: [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs)

- [ ] **Step 1: Update MaterialCapability & MaterialCapabilityFlags**
Add all 11 capabilities to [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs):

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaterialCapability {
    BoundaryPermeability,
    ResourceUptake,
    Metabolism,
    StorageCapacity,
    MaterialSynthesis,
    StructuralGrowth,
    Repair,
    Contractility,
    ResourceSensing,
    PressureSensing,
    DamageSensing,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MaterialCapabilityFlags {
    pub boundary_permeability: bool,
    pub resource_uptake: bool,
    pub metabolism: bool,
    pub storage_capacity: bool,
    pub material_synthesis: bool,
    pub structural_growth: bool,
    pub repair: bool,
    pub contractility: bool,
    pub resource_sensing: bool,
    pub pressure_sensing: bool,
    pub damage_sensing: bool,
}
```

Update `has()` method mapping on `MaterialCapabilityFlags` to support all 11 variants.

- [ ] **Step 2: Update capability bit helper in CellStore**
Modify [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs) `capability_bit` mapping:
```rust
const fn capability_bit(capability: crate::core::process::MaterialCapability) -> u16 {
    match capability {
        crate::core::process::MaterialCapability::BoundaryPermeability => 1 << 0,
        crate::core::process::MaterialCapability::ResourceUptake => 1 << 1,
        crate::core::process::MaterialCapability::Metabolism => 1 << 2,
        crate::core::process::MaterialCapability::StorageCapacity => 1 << 3,
        crate::core::process::MaterialCapability::MaterialSynthesis => 1 << 4,
        crate::core::process::MaterialCapability::StructuralGrowth => 1 << 5,
        crate::core::process::MaterialCapability::Repair => 1 << 6,
        crate::core::process::MaterialCapability::Contractility => 1 << 7,
        crate::core::process::MaterialCapability::ResourceSensing => 1 << 8,
        crate::core::process::MaterialCapability::PressureSensing => 1 << 9,
        crate::core::process::MaterialCapability::DamageSensing => 1 << 10,
    }
}
```
Update `disabled_capabilities: Vec<u16>` in `CellStore` to support 16-bit masks.

- [ ] **Step 3: Run cargo test to verify it compiles**
Run: `cargo test`
Ensure everything compiles successfully with updated flags.

---

## Task 2: Implement 9-Material Inventory in CellStore

**Files:**
- Modify: [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs)

- [ ] **Step 1: Replace general materials vector with 9 distinct vectors**
In [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs):
Replace `materials: Vec<MaterialAmount>` with:
```rust
    boundary_materials: Vec<MaterialAmount>,
    transport_materials: Vec<MaterialAmount>,
    metabolic_materials: Vec<MaterialAmount>,
    storage_materials: Vec<MaterialAmount>,
    synthesis_materials: Vec<MaterialAmount>,
    structural_materials: Vec<MaterialAmount>,
    repair_materials: Vec<MaterialAmount>,
    contractile_materials: Vec<MaterialAmount>,
    sensory_materials: Vec<MaterialAmount>,
```
Initialize all 9 vectors in `with_capacity()` and `insert_initial()`.

- [ ] **Step 2: Add getters and mutators**
Implement getters and mutators for all 9 materials:
- `boundary_material(index) -> MaterialAmount`, `set_boundary_material(index, amount)`
- (Repeat for all other 8 types: transport, metabolic, storage, synthesis, structural, repair, contractile, sensory).

Implement `total_materials(index) -> MaterialAmount` summing all 9 materials. Update `used_capacity(index)` to use `self.total_materials(index)`.

- [ ] **Step 3: Implement exact capability mapping in has_capability**
Update `has_capability`:
```rust
    pub fn has_capability(
        &self,
        index: CellIndex,
        capability: crate::core::process::MaterialCapability,
    ) -> bool {
        if self.lifecycle_state(index) == LifecycleState::Dead {
            return false;
        }
        let disabled = self.disabled_capabilities[index.raw()];
        let bit = capability_bit(capability);
        if (disabled & bit) != 0 {
            return false;
        }

        use crate::core::process::MaterialCapability;
        match capability {
            MaterialCapability::BoundaryPermeability => self.boundary_materials[index.raw()].raw() > 0.0,
            MaterialCapability::ResourceUptake => self.transport_materials[index.raw()].raw() > 0.0,
            MaterialCapability::Metabolism => self.metabolic_materials[index.raw()].raw() > 0.0,
            MaterialCapability::StorageCapacity => self.storage_materials[index.raw()].raw() > 0.0,
            MaterialCapability::MaterialSynthesis => self.synthesis_materials[index.raw()].raw() > 0.0,
            MaterialCapability::StructuralGrowth => self.structural_materials[index.raw()].raw() > 0.0,
            MaterialCapability::Repair => self.repair_materials[index.raw()].raw() > 0.0,
            MaterialCapability::Contractility => self.contractile_materials[index.raw()].raw() > 0.0,
            MaterialCapability::ResourceSensing
            | MaterialCapability::PressureSensing
            | MaterialCapability::DamageSensing => self.sensory_materials[index.raw()].raw() > 0.0,
        }
    }
```

---

## Task 3: Update Config & TOML Parsing with Legacy Compatibility

**Files:**
- Modify: [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs)
- Modify: [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs)

- [ ] **Step 1: Replace material amount in CellInitialConfig**
In [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs):
Replace `initial_material_amount` in `CellInitialConfig` with the 9 specific material amounts. Update builder methods and `config_hash()`.

- [ ] **Step 2: Map parsed materials and distribute legacy values**
In [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs):
Parse named materials. If a legacy name (like `cell_wall`) is used, or if the config lacks specific metabolic/transport/boundary material types but has non-zero total materials, distribute the sum equally among all 9 materials so that all capabilities are enabled.

```rust
        let mut boundary = 0.0;
        let mut transport = 0.0;
        let mut metabolic = 0.0;
        let mut storage = 0.0;
        let mut synthesis = 0.0;
        let mut structural = 0.0;
        let mut repair = 0.0;
        let mut contractile = 0.0;
        let mut sensory = 0.0;

        let has_specific = raw_cell.initial_materials.keys().any(|k| {
            matches!(k.as_str(), "boundary" | "transport" | "metabolic" | "storage" | "synthesis" | "structural" | "repair" | "contractile" | "sensory")
        });

        if !has_specific && cell_initial_materials_sum > 0.0 {
            // Legacy / backwards-compatible mode: distribute equally
            let share = cell_initial_materials_sum / 9.0;
            boundary = share;
            transport = share;
            metabolic = share;
            storage = share;
            synthesis = share;
            structural = share;
            repair = share;
            contractile = share;
            sensory = share;
        } else {
            for (k, &v) in &raw_cell.initial_materials {
                match k.as_str() {
                    "boundary" | "membrane" | "envelope" => boundary = v,
                    "transport" | "pump" => transport = v,
                    "metabolic" | "metabolism" | "converter" => metabolic = v,
                    "storage" | "vacuolar" => storage = v,
                    "synthesis" | "producer" => synthesis = v,
                    "structural" | "skeleton" | "wall" | "cell_wall" => structural = v,
                    "repair" => repair = v,
                    "contractile" | "motor" => contractile = v,
                    "sensory" | "receptor" => sensory = v,
                    _ => structural = v,
                }
            }
        }
```

---

## Task 4: Integration Verification & Tests

**Files:**
- Create: [phase2_materials_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_materials_smoke.rs)

- [ ] **Step 1: Write integration tests**
Add a test verifying:
1. Parse new named materials from TOML.
2. Capability checks return true only if corresponding material $> 0.0$.
3. Legacy scenarios map successfully and run stable.

- [ ] **Step 2: Run complete test suite**
Run: `cargo test`
Ensure all 75+ tests pass cleanly.

- [ ] **Step 3: Linter & Formatter validation**
Run:
```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
Expected: PASS

---

## Acceptance Check
- All 9 materials are separate properties of the cell.
- All 11 capabilities are resolved from specific material amounts.
- Legacy scenario compatibility is preserved.
- All workspace tests pass.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
