# Phase 2C Growth and Division Prep Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement cell growth as physical material synthesis, scaling cell radius and capacity dynamically, and implement local pressure/contact tracking to gate division readiness and feasibility.

**Architecture:** 
- **Growth Loop**: A new registered process `GrowthResourceAllocation` converts internal resources and energy into materials, which scales the cell's mass (`materials`), physical `radius`, and `capacity_limit`.
- **Pressure Sensing**: Rebuilding the `SpatialIndex` compiles local contact pressure (the sum of active overlap distances).
- **Division Prep**: A cell becomes `division_ready` when its radius reaches a configured `growth_target_radius`. Division feasibility checks if the cell is ready, has enough energy, and local pressure is below a configured limit.

**Tech Stack:** Rust 2024, Cargo integration tests.

---

## File Structure

Modify:
- [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs): Add config fields for growth costs, target radius, and maximum division pressure.
- [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs): Parse growth and division parameters.
- [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs): Register the new `Division` process and add related feasibility rejection reasons.
- [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs): Add `set_radius()`, `set_capacity_limit()`, and track dynamic pressure and materials.
- [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs): Implement pressure accumulation and division feasibility validation.
- [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs): Integrate growth process execution and update pressure during the physics solver.

Create:
- [phase2_growth_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_growth_smoke.rs): Integration test suite verifying material growth, radius/capacity scaling, pressure sensing, and division checks.

---

## Task 1: Add Growth & Division Config

**Files:**
- Modify: [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs)
- Modify: [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs)

- [ ] **Step 1: Write failing config parsing test**
Add a test in [tests/phase2_growth_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_growth_smoke.rs) that parses a scenario with growth and division parameters.

```rust
use alife::runner::config_parser::RawScenarioConfig;

fn base_scenario_toml() -> &'static str {
    r#"
scenario_id = "growth_test"
seed = 42
tick_count = 100

[world]
size = [16.0, 16.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = {}
initial_materials = { structural = 5.0 }
initial_energy = 5.0
energy_capacity = 10.0
mandatory_cost_per_tick = 2.0
capacity_limit = 20.0

[environment]
ambient_temperature = 25.0
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.0
heat_warning_threshold = 10.0
heat_death_threshold = 20.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.0
waste_warning_threshold = 10.0
waste_death_threshold = 20.0

[lifecycle]
stress_energy_threshold = 2.0
dormancy_allowed = true
critical_capacity_overrun = 5.0

[growth]
growth_cost_resource = 2.0
growth_cost_energy = 1.5
growth_target_radius = 2.0
max_division_pressure = 0.5
"#
}

#[test]
fn parser_loads_growth_and_division_config() {
    let toml = base_scenario_toml();
    let config = RawScenarioConfig::parse(toml).unwrap();
    assert_eq!(config.growth.growth_cost_resource, 2.0);
    assert_eq!(config.growth.growth_target_radius, 2.0);
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_growth_smoke parser_loads_growth_and_division_config`
Expected: Compilation failure because `growth` section is missing in `RawScenarioConfig`.

- [ ] **Step 3: Implement GrowthConfig fields**
In [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs), add `GrowthConfig`:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GrowthConfig {
    pub growth_cost_resource: ResourceAmount,
    pub growth_cost_energy: EnergyAmount,
    pub growth_target_radius: Radius,
    pub max_division_pressure: f32,
}

impl Default for GrowthConfig {
    fn default() -> Self {
        Self {
            growth_cost_resource: ResourceAmount::new(2.0).unwrap(),
            growth_cost_energy: EnergyAmount::new(1.0).unwrap(),
            growth_target_radius: Radius::new(2.0).unwrap(),
            max_division_pressure: 0.5,
        }
    }
}
```
Add `pub growth: GrowthConfig` to `RuntimeConfig`. Update `RuntimeConfig::new()` to initialize it with default values.

In [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs), add `RawGrowth` and parse it:
```rust
#[derive(Deserialize, Debug)]
pub struct RawGrowth {
    pub growth_cost_resource: f32,
    pub growth_cost_energy: f32,
    pub growth_target_radius: f32,
    pub max_division_pressure: f32,
}

// Add pub growth: Option<RawGrowth> to RawScenarioConfig.
```
Map it in `to_runtime_config()` to `runtime_config.growth`.

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_growth_smoke parser_loads_growth_and_division_config`
Expected: PASS

---

## Task 2: Implement Pressure Accumulation in Physics Solver

**Files:**
- Modify: [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs)
- Modify: [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs)

- [ ] **Step 1: Write failing pressure sensing test**
Add a test in [tests/phase2_growth_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_growth_smoke.rs) verifying that contact pressure is accumulated on overlapping cells.

```rust
use alife::core::cell_store::CellIndex;
use alife::core::tick::TickExecutor;

#[test]
fn cells_accumulate_contact_pressure_during_collisions() {
    let base = base_scenario_config(); // helper returning valid RuntimeConfig
    // Set cell_1 and cell_2 overlapping
    let cell_1 = CellInitialConfig { position: Position::new(4.0, 4.0), radius: Radius::new(2.0).unwrap(), ..base.cell };
    let cell_2 = CellInitialConfig { position: Position::new(5.0, 4.0), radius: Radius::new(2.0).unwrap(), ..base.cell };
    let config = base.with_cells(vec![cell_1, cell_2]);

    let mut exec = TickExecutor::new(config).unwrap();
    let _summary = exec.step().unwrap();

    let pressure_0 = exec.world().cells().contact_pressure(CellIndex::from_raw(0));
    assert!(pressure_0 > 0.0);
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_growth_smoke cells_accumulate_contact_pressure_during_collisions`
Expected: Compilation failure because `contact_pressure` does not exist on `CellStore`.

- [ ] **Step 3: Add pressure field to CellStore and update during tick**
In [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs), add `pressures: Vec<f32>` to `CellStore`.
Expose:
```rust
    pub fn contact_pressure(&self, index: CellIndex) -> f32 {
        self.pressures[index.raw()] = self.pressures[index.raw()];
    }

    pub fn set_contact_pressure(&mut self, index: CellIndex, pressure: f32) {
        self.pressures[index.raw()] = pressure;
    }
```
Initialize with `0.0` in `insert_initial`.

In [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs), at the start of the physics solver loop inside `step()`, reset cell pressures to `0.0`.
When resolving cell-cell overlaps:
```rust
                    if dist_sq < target_dist * target_dist {
                        let dist = dist_sq.sqrt();
                        let overlap = target_dist - dist;
                        overlap_resolved += overlap;

                        // Increment local pressure on both cells
                        let cells = self.world.cells_mut_for_commit();
                        let p_i = cells.contact_pressure(idx_i) + overlap;
                        let p_j = cells.contact_pressure(idx_j) + overlap;
                        cells.set_contact_pressure(idx_i, p_i);
                        cells.set_contact_pressure(idx_j, p_j);
                        
                        // ... (existing separation push) ...
                    }
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_growth_smoke cells_accumulate_contact_pressure_during_collisions`
Expected: PASS

---

## Task 3: Implement Material Growth & Radius Scaling

**Files:**
- Modify: [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs)
- Modify: [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs)
- Modify: [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs)

- [ ] **Step 1: Write failing growth test**
Add a test in [tests/phase2_growth_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_growth_smoke.rs) that triggers `GrowthResourceAllocation` and verifies that the cell's radius and capacity scale up.

```rust
use alife::core::process::{ActionCandidate, ProcessId};

#[test]
fn structural_growth_increases_cell_radius_and_capacity() {
    let mut config = base_scenario_config();
    config.resource_interaction.enabled = true;
    config.cell.initial_resource_amount = ResourceAmount::new(10.0).unwrap();
    config.cell.initial_energy = EnergyAmount::new(10.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);

    let initial_radius = exec.world().cells().radius(idx).raw();
    let initial_cap = exec.world().cells().capacity_limit(idx).raw();

    // Trigger growth process directly
    let candidate_growth = ActionCandidate {
        process_id: ProcessId::GrowthResourceAllocation,
        requested_amount: 1.0,
    };
    let res = exec.world_mut().execute_growth_for_test(idx, &candidate_growth);
    assert!(res.is_ok());

    let final_radius = exec.world().cells().radius(idx).raw();
    let final_cap = exec.world().cells().capacity_limit(idx).raw();

    assert!(final_radius > initial_radius);
    assert!(final_cap > initial_cap);
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_growth_smoke structural_growth_increases_cell_radius_and_capacity`
Expected: Compilation failure because `execute_growth_for_test` and `GrowthResourceAllocation` process logic do not exist.

- [ ] **Step 3: Register Growth Process & Implement Updates**
In [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs), add `GrowthResourceAllocation` to `ProcessId`.

In [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs), add mutators:
```rust
    pub fn set_radius(&mut self, index: CellIndex, radius: Radius) {
        self.radii[index.raw()] = radius;
    }
    
    pub fn set_capacity_limit(&mut self, index: CellIndex, limit: CapacityAmount) {
        self.capacity_limits[index.raw()] = limit;
    }

    pub fn set_materials(&mut self, index: CellIndex, amount: MaterialAmount) {
        self.materials[index.raw()] = amount;
    }
```

In [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs), implement growth feasibility check and execution:
```rust
    pub fn execute_growth_for_test(&mut self, cell_idx: CellIndex, action: &ActionCandidate) -> Result<(), String> {
        let config = self.config.clone();
        
        let cost_res = config.growth.growth_cost_resource.raw();
        let cost_eng = config.growth.growth_cost_energy.raw();

        let current_res = self.cells.resource_amount(cell_idx).raw();
        let current_eng = self.cells.energy(cell_idx).current().raw();

        if current_res < cost_res || current_eng < cost_eng {
            return Err("Insufficient resources or energy".to_string());
        }

        // Deduct cost
        self.cells.set_resources(cell_idx, ResourceAmount::new(current_res - cost_res).unwrap());
        let next_energy = EnergyAmount::new(current_eng - cost_eng).unwrap();
        self.cells.set_energy(cell_idx, EnergyBuffer::new(next_energy, self.cells.energy(cell_idx).capacity()));

        // Synthesize materials (mass)
        let old_materials = self.cells.material_amount(cell_idx).raw();
        let new_materials = old_materials + 1.0;
        self.cells.set_materials(cell_idx, MaterialAmount::new(new_materials).unwrap());

        // Update radius based on mass scaling: radius = base_radius * sqrt(new_mass / old_mass)
        let old_radius = self.cells.radius(cell_idx).raw();
        let new_radius_val = old_radius * (new_materials / old_materials).sqrt();
        let new_radius = Radius::new(new_radius_val).unwrap();
        self.cells.set_radius(cell_idx, new_radius);

        // Update capacity limit scaling with radius area increase: capacity = old_cap * (new_radius / old_radius)^2
        let old_cap = self.cells.capacity_limit(cell_idx).raw();
        let new_cap_val = old_cap * (new_radius_val / old_radius).powi(2);
        self.cells.set_capacity_limit(cell_idx, CapacityAmount::new(new_cap_val).unwrap());

        Ok(())
    }
```

In [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs), inside `step()`, generate and execute the growth candidate in the process loop if the cell has excess resources:
```rust
                // 3. Growth Process
                let candidate_growth = ActionCandidate {
                    process_id: ProcessId::GrowthResourceAllocation,
                    requested_amount: 1.0,
                };
                if self.world.validate_feasibility(idx, &candidate_growth).is_feasible() {
                    // Execute growth
                    let _ = self.world.execute_growth_for_test(idx, &candidate_growth);
                }
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_growth_smoke structural_growth_increases_cell_radius_and_capacity`
Expected: PASS

---

## Task 4: Implement Division Readiness & Feasibility Gated by Pressure

**Files:**
- Modify: [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs)
- Modify: [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs)

- [ ] **Step 1: Write failing division feasibility test**
Add a test in [tests/phase2_growth_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_growth_smoke.rs) verifying division readiness and pressure-gated feasibility.

```rust
#[test]
fn division_readiness_and_pressure_gating_work() {
    let mut config = base_scenario_config();
    config.growth.growth_target_radius = 2.0;
    config.growth.max_division_pressure = 0.5;

    let mut exec = TickExecutor::new(config).unwrap();
    let idx = CellIndex::from_raw(0);

    let candidate_division = ActionCandidate {
        process_id: ProcessId::Division,
        requested_amount: 0.0,
    };

    // 1. Division should be rejected because cell radius (1.0) is below target (2.0)
    let res = exec.world().validate_feasibility(idx, &candidate_division);
    assert!(matches!(res, FeasibilityResult::Rejected(RejectionReason::RadiusBelowTarget)));

    // Set radius to target
    exec.world_mut().cells_mut_for_commit().set_radius(idx, Radius::new(2.0).unwrap());

    // 2. Division should now be feasible
    let res = exec.world().validate_feasibility(idx, &candidate_division);
    assert!(matches!(res, FeasibilityResult::Feasible));

    // Set high contact pressure
    exec.world_mut().cells_mut_for_commit().set_contact_pressure(idx, 1.0);

    // 3. Division should be rejected due to high pressure
    let res = exec.world().validate_feasibility(idx, &candidate_division);
    assert!(matches!(res, FeasibilityResult::Rejected(RejectionReason::PressureTooHigh)));
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_growth_smoke division_readiness_and_pressure_gating_work`
Expected: Compilation failure (missing `ProcessId::Division`, `RejectionReason` variants, etc.).

- [ ] **Step 3: Define Division Rejections & Validate Feasibility**
In [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs), add:
```rust
// In ProcessId:
Division,

// In RejectionReason:
RadiusBelowTarget,
PressureTooHigh,
```

In [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs), inside `validate_feasibility`:
```rust
            ProcessId::Division => {
                let radius = self.cells.radius(cell_idx).raw();
                let target = self.config.growth.growth_target_radius.raw();
                if radius < target {
                    return FeasibilityResult::Rejected(RejectionReason::RadiusBelowTarget);
                }

                let pressure = self.cells.contact_pressure(cell_idx);
                let max_pressure = self.config.growth.max_division_pressure;
                if pressure > max_pressure {
                    return FeasibilityResult::Rejected(RejectionReason::PressureTooHigh);
                }

                FeasibilityResult::Feasible
            }
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_growth_smoke division_readiness_and_pressure_gating_work`
Expected: PASS

---

## Task 5: Final Review & Integration Verification

- [ ] **Step 1: Run complete test suite**
Run: `cargo test`
Expected: PASS (All 71 existing + new tests pass).

- [ ] **Step 2: Linter & Formatter validation**
Run:
```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
Expected: PASS

- [ ] **Step 3: Document progress worklog**
Generate the walkthrough and report summarizing the findings.

---

## Acceptance Check
- Cells grow by consuming resources/energy and adding materials.
- Overlap contact pressure is calculated correctly.
- Cell radius and capacity scale with materials.
- Division readiness and feasibility are correctly gated by target radius and contact pressure.
- All tests pass cleanly.
