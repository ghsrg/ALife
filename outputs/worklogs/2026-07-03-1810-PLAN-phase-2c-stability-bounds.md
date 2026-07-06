# Phase 2C Stability Bounds and Sweep Analysis Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a comprehensive Rust-based stability analysis test suite (`tests/phase2_stability_analysis.rs`) comprising 6 advanced tests (viability threshold sweep, equilibrium bounds, wake up hysteresis, resource/energy conservation, transport/metabolism balance matrix, and multi-cell resource order bias).

**Architecture:**
- **Viability Threshold**: Sweep resource densities to find the collapse, dormancy, active, and growth zones.
- **Equilibrium Bounds**: Test passive incomes slightly below (`0.19`), exactly equal (`0.20`), and slightly above (`0.22`) the dormant upkeep cost.
- **Dormancy Wakeup Hysteresis**: Verify stable transitions without oscillation when resources are restored.
- **Energy and Resource Conservation**: Programmatically verify that resource mass and energy balances sum exactly to constants at each tick.
- **Transport/Metabolism Matrix**: Classify outcomes of varying uptake rates and metabolism efficiencies.
- **Resource Order Independence**: Check if the first cell in the array is biased to absorb resources first, documenting the behavior.

**Tech Stack:** Rust 2024, Cargo integration tests.

---

## File Structure

Modify:
- [phase2_stability_analysis.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_stability_analysis.rs): Integration test suite containing sweeps, dormancy strategies, and pressure escaping parameters.

Modify:
- [README.md](file:///c:/Users/korsr/PycharmProjects/ALife/outputs/worklogs/index.md): Register the plan and report.

---

## Task 1: Implement Grid Parameter Sweep & Equilibrium Bounds Tests

**Files:**
- Modify: [phase2_stability_analysis.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_stability_analysis.rs)

- [ ] **Step 1: Write the sweep test case**
Create [phase2_stability_analysis.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_stability_analysis.rs) and implement a grid search over metabolic/transport materials and resources, executing the simulation for 300 ticks and validating outcome bounds:

- [ ] **Step 2: Write test_dormancy_below_equal_above_equilibrium**
Write `test_dormancy_below_equal_above_equilibrium` verifying survival outcomes of a dormant cell with dormant upkeep cost `0.2` under passive incomes of `0.19`, `0.20`, and `0.22`.

```rust
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Tick, WasteAmount,
};
use alife::core::world::WorldState;
use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn test_grid_parameter_sweep() {
    let mut stable_runs = 0;
    let mut starved_runs = 0;

    // Sweep metabolic material and initial resource density
    for &metabolic_amount in &[0.5, 1.0, 2.0, 5.0] {
        for &initial_res in &[2.0, 5.0, 10.0] {
            let config = build_sweep_config(metabolic_amount, initial_res);
            let mut world = WorldState::initialize(config).unwrap();
            let mut tick_executor = alife::core::tick::TickExecutor::new(&mut world);

            let mut collapsed = false;
            for _ in 0..300 {
                let summary = tick_executor.step().unwrap();
                if summary.survival_result == alife::core::summary::SurvivalResult::Collapsed {
                    collapsed = true;
                    starved_runs += 1;
                    break;
                }
            }
            if !collapsed {
                stable_runs += 1;
            }
        }
    }

    assert!(stable_runs > 0, "Expected at least some sweeps to survive");
    assert!(starved_runs > 0, "Expected some sweeps to collapse under scarce resources");
}

fn build_sweep_config(metabolic: f32, resource: f32) -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(300),
            seed: alife::core::units::Seed::from_raw(42),
            size: alife::core::units::WorldSize::new(512.0, 512.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(
            vec![ResourceAmount::new(resource).unwrap()],
            0.01,
        )
        .unwrap(),
        ResourceInteractionConfig {
            enabled: true,
            uptake_layer_index: 0,
            uptake_rate_per_tick: ResourceAmount::new(1.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(0.5).unwrap(),
            energy_per_resource: 10.0,
            heat_per_resource: 0.1,
            waste_per_resource: 0.1,
        },
        CellInitialConfig {
            position: Position::new(256.0, 256.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(50.0).unwrap(),
            energy_capacity: 100.0,
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::zero(),
            capacity_limit: 30.0,
            initial_resource_amount: ResourceAmount::zero(),
            initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
            initial_transport_material: MaterialAmount::new(1.0).unwrap(),
            initial_metabolic_material: MaterialAmount::new(metabolic).unwrap(),
            initial_storage_material: MaterialAmount::zero(),
            initial_synthesis_material: MaterialAmount::zero(),
            initial_structural_material: MaterialAmount::new(1.0).unwrap(),
            initial_repair_material: MaterialAmount::zero(),
            initial_contractile_material: MaterialAmount::zero(),
            initial_sensory_material: MaterialAmount::zero(),
        },
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::new(0.2).unwrap(),
            heat_warning_threshold: HeatAmount::new(50.0).unwrap(),
            heat_death_threshold: HeatAmount::new(80.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::new(0.1).unwrap(),
            waste_warning_threshold: WasteAmount::new(10.0).unwrap(),
            waste_death_threshold: WasteAmount::new(20.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(10.0).unwrap(),
            dormancy_allowed: true,
            dormant_mandatory_cost_modifier: 0.1,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap()
}

#[test]
fn test_viability_threshold_sweep() {
    let densities = &[0.0, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0];
    
    for &density in densities {
        let config = build_sweep_config(1.0, density);
        let mut world = WorldState::initialize(config).unwrap();
        let mut tick_executor = alife::core::tick::TickExecutor::new(&mut world);
        
        let mut collapsed = false;
        let mut dormant_ticks = 0;
        
        for _ in 0..100 {
            let summary = tick_executor.step().unwrap();
            if summary.survival_result == alife::core::summary::SurvivalResult::Collapsed {
                collapsed = true;
                break;
            }
            if summary.lifecycle == alife::core::units::LifecycleState::Dormant {
                dormant_ticks += 1;
            }
        }
        
        if density < 0.5 {
            assert!(collapsed, "Expected collapse for density {}", density);
        } else if density >= 0.5 && density <= 2.0 {
            assert!(!collapsed, "Expected survival for density {}", density);
            assert!(dormant_ticks > 50, "Expected significant dormancy for density {}", density);
        } else {
            assert!(!collapsed, "Expected active survival for density {}", density);
        }
    }
}

#[test]
fn test_dormancy_below_equal_above_equilibrium() {
    // 1. Below equilibrium (0.19) - slow death
    {
        let mut config = build_sweep_config(1.0, 0.0);
        config.cell.initial_energy = EnergyAmount::new(5.0).unwrap();
        config.cell.passive_energy_income = EnergyAmount::new(0.19).unwrap();
        let mut world = WorldState::initialize(config).unwrap();
        let mut tick_executor = alife::core::tick::TickExecutor::new(&mut world);
        let mut collapsed = false;
        for _ in 0..200 {
            let summary = tick_executor.step().unwrap();
            if summary.survival_result == alife::core::summary::SurvivalResult::Collapsed {
                collapsed = true;
                break;
            }
        }
        assert!(collapsed, "Cell should slowly collapse with passive income below dormant upkeep");
    }

    // 2. Exactly equal (0.20) - stable dormancy
    {
        let mut config = build_sweep_config(1.0, 0.0);
        config.cell.initial_energy = EnergyAmount::new(5.0).unwrap();
        config.cell.passive_energy_income = EnergyAmount::new(0.20).unwrap();
        let mut world = WorldState::initialize(config).unwrap();
        let mut tick_executor = alife::core::tick::TickExecutor::new(&mut world);
        for _ in 0..200 {
            let summary = tick_executor.step().unwrap();
            assert_ne!(summary.survival_result, alife::core::summary::SurvivalResult::Collapsed);
        }
    }

    // 3. Above equilibrium (0.22) - accumulates and wakes up
    {
        let mut config = build_sweep_config(1.0, 0.0);
        config.cell.initial_energy = EnergyAmount::new(5.0).unwrap();
        config.cell.passive_energy_income = EnergyAmount::new(0.22).unwrap();
        config.lifecycle.stress_energy_threshold = EnergyAmount::new(6.0).unwrap();
        config.cell.energy_capacity = 100.0;
        let mut world = WorldState::initialize(config).unwrap();
        let mut tick_executor = alife::core::tick::TickExecutor::new(&mut world);
        let mut woke_up = false;
        for _ in 0..200 {
            let summary = tick_executor.step().unwrap();
            if summary.lifecycle == alife::core::units::LifecycleState::Alive {
                woke_up = true;
                break;
            }
        }
        assert!(woke_up, "Cell should wake up when energy accumulates above threshold");
    }
}

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
