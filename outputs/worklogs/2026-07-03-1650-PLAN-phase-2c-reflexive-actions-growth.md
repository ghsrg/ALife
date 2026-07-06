# Phase 2C Reflexive Actions, Growth, and Contractility Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement Phase 2C reflexive material actions, dynamic structural material synthesis, and contractile physical displacement under deterministic accounting, replacing manual process triggers with an autonomous Reflex Policy.

**Architecture:**
- **Material Synthesis**: Implement `ProcessId::MaterialSynthesis` converting internal resources and energy into specific materials (e.g. `structural_material` for growth).
- **Contractile Displacement**: Implement `ProcessId::ContractileDisplacement` using `Contractility` capability to actively push the cell away from high-pressure collisions.
- **Reflex Policy**: Replace hardcoded tick execution branches with a deterministic priority-ordered reflex action loop: Uptake -> Metabolism -> Synthesis -> Growth -> Displacement.
- **Config & TOML**: Add `synthesis` and `contractility` blocks to config files and the TOML parser with default presets for backward compatibility.

**Tech Stack:** Rust 2024, Cargo integration tests.

---

## File Structure

Modify:
- [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs): Add `SynthesisConfig` and `ContractilityConfig` to `RuntimeConfig`.
- [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs): Parse `[synthesis]` and `[contractility]` blocks.
- [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs): Add `ContractileDisplacement` to `ProcessId`. Update `RejectionReason`.
- [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs): Implement feasibility checks and executions for `MaterialSynthesis` and `ContractileDisplacement`.
- [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs): Integrate the priority-ordered Reflex Policy loop inside `step()`.

Create:
- [phase2_reflex_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_reflex_smoke.rs): Integration test suite verifying autonomous material synthesis, growth, and contractile movement.

---

## Task 1: Implement Material Synthesis Process

**Files:**
- Modify: [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs)
- Modify: [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs)
- Modify: [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs)

- [ ] **Step 1: Write SynthesisConfig structure and parser**
In [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs), define `SynthesisConfig`:
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct SynthesisConfig {
    pub cost_resource: ResourceAmount,
    pub cost_energy: EnergyAmount,
}
```
Add it to `RuntimeConfig`. In [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs), parse `[synthesis]` block, defaulting to `cost_resource = 1.0`, `cost_energy = 5.0`.

- [ ] **Step 2: Implement feasibility validation for MaterialSynthesis**
In [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs) `validate_feasibility`:
```rust
            ProcessId::MaterialSynthesis => {
                if !self.cells.has_capability(cell_idx, MaterialCapability::MaterialSynthesis) {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(
                        MaterialCapability::MaterialSynthesis,
                    ));
                }
                let cost_res = self.config.synthesis.cost_resource.raw();
                let cost_eng = self.config.synthesis.cost_energy.raw();
                let current_res = self.cells.resource_amount(cell_idx).raw();
                let current_eng = self.cells.energy(cell_idx).current().raw();

                if current_res < cost_res {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientResources)
                } else if current_eng < cost_eng {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
                } else {
                    FeasibilityResult::Feasible
                }
            }
```

- [ ] **Step 3: Implement MaterialSynthesis execution**
In [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs), implement `execute_synthesis`:
```rust
    pub fn execute_synthesis(&mut self, cell_idx: CellIndex) -> Result<(), String> {
        let cost_res = self.config.synthesis.cost_resource.raw();
        let cost_eng = self.config.synthesis.cost_energy.raw();
        let current_res = self.cells.resource_amount(cell_idx).raw();
        let current_eng = self.cells.energy(cell_idx).current().raw();

        if current_res < cost_res || current_eng < cost_eng {
            return Err("Insufficient resources or energy".to_string());
        }

        self.cells.set_resources(cell_idx, ResourceAmount::new(current_res - cost_res).unwrap());
        let next_energy = EnergyAmount::new(current_eng - cost_eng).unwrap();
        self.cells.set_energy(cell_idx, EnergyBuffer::new(next_energy, self.cells.energy(cell_idx).capacity()));

        // Synthesize structural material by default
        let old_structural = self.cells.structural_material(cell_idx).raw();
        self.cells.set_structural_material(cell_idx, MaterialAmount::new(old_structural + 1.0).unwrap());

        Ok(())
    }
```

- [ ] **Step 4: Run cargo test to verify it compiles**
Run: `cargo test`
Expected: PASS

---

## Task 2: Implement Contractile Displacement (Movement)

**Files:**
- Modify: [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs)
- Modify: [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs)
- Modify: [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs)
- Modify: [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs)

- [ ] **Step 1: Update ProcessId and RejectionReason**
In [process.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/process.rs):
Add `ContractileDisplacement` to `ProcessId` enum.
Add `NoPressure` to `RejectionReason` enum.

- [ ] **Step 2: Add ContractilityConfig**
In [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs):
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct ContractilityConfig {
    pub energy_cost: EnergyAmount,
    pub force_factor: f32,
}
```
Add to `RuntimeConfig`. Update TOML parser defaulting to `energy_cost = 1.0`, `force_factor = 0.1`.

- [ ] **Step 3: Implement feasibility validation for ContractileDisplacement**
In [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs) `validate_feasibility`:
```rust
            ProcessId::ContractileDisplacement => {
                if !self.cells.has_capability(cell_idx, MaterialCapability::Contractility) {
                    return FeasibilityResult::Rejected(RejectionReason::MissingCapability(
                        MaterialCapability::Contractility,
                    ));
                }
                let pressure = self.cells.contact_pressure(cell_idx);
                if pressure <= 0.0 {
                    return FeasibilityResult::Rejected(RejectionReason::NoPressure);
                }
                let cost_eng = self.config.contractility.energy_cost.raw();
                let current_eng = self.cells.energy(cell_idx).current().raw();

                if current_eng < cost_eng {
                    FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
                } else {
                    FeasibilityResult::Feasible
                }
            }
```

- [ ] **Step 4: Implement ContractileDisplacement execution**
In [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs), implement `execute_displacement`:
```rust
    pub fn execute_displacement(&mut self, cell_idx: CellIndex) -> Result<(), String> {
        let cost_eng = self.config.contractility.energy_cost.raw();
        let current_eng = self.cells.energy(cell_idx).current().raw();
        if current_eng < cost_eng {
            return Err("Insufficient energy".to_string());
        }

        let pressure = self.cells.contact_pressure(cell_idx);
        if pressure <= 0.0 {
            return Err("No pressure present".to_string());
        }

        // Deduct energy
        let next_energy = EnergyAmount::new(current_eng - cost_eng).unwrap();
        self.cells.set_energy(cell_idx, EnergyBuffer::new(next_energy, self.cells.energy(cell_idx).capacity()));

        // Calculate push vector away from colliding neighbors
        let cell_pos = self.cells.position(cell_idx);
        let cell_rad = self.cells.radius(cell_idx).raw();
        let mut push_x = 0.0;
        let mut push_y = 0.0;

        for i in 0..self.cells.len() {
            let other_idx = CellIndex::from_raw(i);
            if other_idx == cell_idx || self.cells.lifecycle_state(other_idx) == LifecycleState::Dead {
                continue;
            }
            let other_pos = self.cells.position(other_idx);
            let other_rad = self.cells.radius(other_idx).raw();
            let dx = cell_pos.x() - other_pos.x();
            let dy = cell_pos.y() - other_pos.y();
            let dist = (dx * dx + dy * dy).sqrt();
            let sum_rad = cell_rad + other_rad;
            if dist < sum_rad && dist > 0.001 {
                let overlap = sum_rad - dist;
                push_x += (dx / dist) * overlap;
                push_y += (dy / dist) * overlap;
            }
        }

        // Scale by contractile capability (mass) and force factor config
        let contractility_mass = self.cells.contractile_material(cell_idx).raw();
        let shift_factor = contractility_mass * self.config.contractility.force_factor;
        let final_x = cell_pos.x() + push_x * shift_factor;
        let final_y = cell_pos.y() + push_y * shift_factor;

        // Clamp to world boundaries
        let max_w = self.config.world.size.width();
        let max_h = self.config.world.size.height();
        let clamped_x = final_x.clamp(cell_rad, max_w - cell_rad);
        let clamped_y = final_y.clamp(cell_rad, max_h - cell_rad);

        self.cells.set_position(cell_idx, Position::new(clamped_x, clamped_y));
        Ok(())
    }
```

---

## Task 3: Implement Material-Driven Reflex Policy

**Files:**
- Modify: [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs)

- [ ] **Step 1: Replace TickExecutor::step process triggers with Reflex Loop**
In [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs), update cell process loop inside `step()`:
```rust
        // Reflexive Action Selection Loop
        for i in 0..cells_len {
            let index = CellIndex::from_raw(i);
            if self.world.cells().lifecycle_state(index) == LifecycleState::Dead {
                continue;
            }

            // 1. Local Resource Uptake
            if config.resource_interaction.enabled {
                let candidate_uptake = ActionCandidate {
                    process_id: ProcessId::LocalResourceUptake,
                    requested_amount: config.resource_interaction.uptake_rate_per_tick,
                };
                process_attempts += 1;
                if self.world.validate_feasibility(index, &candidate_uptake).is_feasible() {
                    let layer = 0;
                    let pos = self.world.cells().position(index);
                    let coord = self.world.resources().position_to_coord(pos);
                    let external_available = self.world.resources().amount_at(layer, coord).unwrap();
                    let requested = config.resource_interaction.uptake_rate_per_tick;
                    let accepted = {
                        let cells = self.world.cells_mut_for_commit();
                        cells.add_resources_limited_by_capacity(index, requested)
                    };
                    let remaining_external = external_available.saturating_sub(accepted);
                    self.world.resources_mut_for_commit().set_amount_at(layer, coord, remaining_external).unwrap();
                } else {
                    process_rejections += 1;
                }
            }

            // 2. Metabolism Energy Conversion
            if config.resource_interaction.enabled {
                let candidate_metabolism = ActionCandidate {
                    process_id: ProcessId::MetabolismEnergyConversion,
                    requested_amount: config.resource_interaction.metabolism_resource_per_tick.raw(),
                };
                process_attempts += 1;
                if self.world.validate_feasibility(index, &candidate_metabolism).is_feasible() {
                    let consumed = {
                        let cells = self.world.cells_mut_for_commit();
                        cells.consume_resources(index, config.resource_interaction.metabolism_resource_per_tick)
                    };
                    let metabolism_energy = EnergyAmount::new(consumed.raw() * config.resource_interaction.energy_per_resource).unwrap();
                    let metabolism_heat = consumed.raw() * config.resource_interaction.heat_per_resource;
                    let metabolism_waste = consumed.raw() * config.resource_interaction.waste_per_resource;

                    metabolism_heat_total += metabolism_heat;
                    metabolism_waste_total += metabolism_waste;

                    if metabolism_energy.raw() > 0.0 {
                        let cells = self.world.cells_mut_for_commit();
                        let current = cells.energy(index);
                        let new_current = current.current().saturating_add(metabolism_energy).clamp_max(current.capacity());
                        cells.set_energy(index, EnergyBuffer::new(new_current, current.capacity()));
                    }
                } else {
                    process_rejections += 1;
                }
            }

            // 3. Material Synthesis
            let candidate_synthesis = ActionCandidate {
                process_id: ProcessId::MaterialSynthesis,
                requested_amount: 1.0,
            };
            if self.world.validate_feasibility(index, &candidate_synthesis).is_feasible() {
                let _ = self.world.execute_synthesis(index);
            }

            // 4. Structural Growth
            if config.growth_enabled && config.resource_interaction.enabled {
                let candidate_growth = ActionCandidate {
                    process_id: ProcessId::GrowthResourceAllocation,
                    requested_amount: 1.0,
                };
                if self.world.validate_feasibility(index, &candidate_growth).is_feasible() {
                    let _ = self.world.execute_growth_for_test(index, &candidate_growth);
                }
            }

            // 5. Contractile Displacement
            let candidate_displacement = ActionCandidate {
                process_id: ProcessId::ContractileDisplacement,
                requested_amount: 1.0,
            };
            if self.world.validate_feasibility(index, &candidate_displacement).is_feasible() {
                let _ = self.world.execute_displacement(index);
            }
        }
```

---

## Task 4: Integration Verification & Tests

**Files:**
- Create: [phase2_reflex_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_reflex_smoke.rs)

- [ ] **Step 1: Write integration tests for Synthesis & Contractile Displacement**
In [phase2_reflex_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_reflex_smoke.rs):
Add test `test_autonomous_synthesis_and_displacement` loading a scenario where:
1. Cells have synthesis capabilities and abundant internal resources, automatically converting them to structural materials and growing.
2. Colliding cells with contractile materials and pressure automatically shift positions away from each other over Ticks.

- [ ] **Step 2: Run complete test suite**
Run: `cargo test`
Expected: PASS (83+ tests)

- [ ] **Step 3: Run Clippy & Formatter**
Verify no warning/error is emitted.
Expected: PASS

---

## Acceptance Check
- Cells autonomously execute synthesis and displacement based on reflexes.
- Contractile displacement resolves collision pressure deterministically.
- All workspace tests pass.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
