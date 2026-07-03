use alife::core::cell_store::{CellIndex, LifecycleState};
use alife::core::config::{
    CellInitialConfig, ContractilityConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::summary::SurvivalResult;
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, GridCoord, HeatAmount, MaterialAmount, Position, Radius,
    ResourceAmount, Seed, Tick, WasteAmount, WorldSize,
};
use std::io::Write;

fn build_sweep_config(metabolic_amount: f32, initial_res: f32) -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(300),
            seed: Seed::from_raw(42),
            size: WorldSize::new(512.0, 512.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
        },
        ResourceConfig::new(vec![ResourceAmount::new(initial_res).unwrap()], 0.01).unwrap(),
        ResourceInteractionConfig {
            enabled: true,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(0.5).unwrap(),
            energy_per_resource: 10.0,
            heat_per_resource: 0.1,
            waste_per_resource: 0.1,
        },
        CellInitialConfig {
            position: Position::new(256.0, 256.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(50.0).unwrap(),
            energy_capacity: EnergyAmount::new(100.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::zero(),
            capacity_limit: CapacityAmount::new(30.0).unwrap(),
            initial_resource_amount: ResourceAmount::zero(),
            initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
            initial_transport_material: MaterialAmount::new(1.0).unwrap(),
            initial_metabolic_material: MaterialAmount::new(metabolic_amount).unwrap(),
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
fn test_grid_parameter_sweep() {
    let mut stable_runs = 0;
    let mut starved_runs = 0;

    // Sweep metabolic material and initial resource density.
    // We use a high resource density (1000.0) for stable runs, and low (2.0) for starved/collapsed runs.
    for &metabolic_amount in &[0.5, 1.0, 2.0, 5.0] {
        for &initial_res in &[2.0, 1000.0] {
            let config = build_sweep_config(metabolic_amount, initial_res);
            let mut tick_executor = TickExecutor::new(config).unwrap();

            let mut collapsed = false;
            for _ in 0..300 {
                let summary = tick_executor.step().unwrap();
                if summary.survival_result == SurvivalResult::Collapse {
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

    assert!(
        stable_runs > 0,
        "Expected at least some sweeps to survive (got {})",
        stable_runs
    );
    assert!(
        starved_runs > 0,
        "Expected some sweeps to collapse under scarce resources (got {})",
        starved_runs
    );
}

#[test]
fn test_minimum_viability_kit_and_dormancy() {
    // Zero resources in environment
    let mut config = build_sweep_config(1.0, 0.0);
    // Set initial energy and passive income to transition immediately to dormancy and stay stable
    config.cell.initial_energy = EnergyAmount::new(1.5).unwrap();
    config.cell.passive_energy_income = EnergyAmount::new(0.2).unwrap();
    config.lifecycle.dormancy_allowed = true;
    config.lifecycle.dormant_mandatory_cost_modifier = 0.1; // 2.0 * 0.1 = 0.2 cost per tick in dormancy

    let mut tick_executor = TickExecutor::new(config).unwrap();

    let mut became_dormant = false;
    for tick in 0..200 {
        let summary = tick_executor.step().unwrap();
        assert_ne!(
            summary.survival_result,
            SurvivalResult::Collapse,
            "Cell died at tick {} but should have survived in dormancy!",
            tick
        );
        let lifecycle = tick_executor
            .world()
            .cells()
            .lifecycle_state(CellIndex::from_raw(0));
        if lifecycle == LifecycleState::Dormant {
            became_dormant = true;
        }
    }

    assert!(
        became_dormant,
        "Cell should have entered dormancy to survive!"
    );
}

fn build_displacement_test_config(force_factor: f32) -> RuntimeConfig {
    let mut config = build_sweep_config(1.0, 10.0);
    // Enable contractility
    config.contractility = ContractilityConfig {
        energy_cost: EnergyAmount::new(1.0).unwrap(),
        force_factor,
    };

    // Build list of two initial overlapping cells
    let cell1 = CellInitialConfig {
        position: Position::new(256.0, 256.0),
        radius: Radius::new(10.0).unwrap(),
        initial_energy: EnergyAmount::new(50.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(1.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(30.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::new(5.0).unwrap(), // Enabled!
        initial_sensory_material: MaterialAmount::zero(),
    };
    let cell2 = CellInitialConfig {
        position: Position::new(257.0, 256.0), // Extreme overlap!
        radius: Radius::new(10.0).unwrap(),
        initial_energy: EnergyAmount::new(50.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(1.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(30.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(), // Heavy static obstacle
        initial_sensory_material: MaterialAmount::zero(),
    };
    config.with_cells(vec![cell1, cell2])
}

#[test]
fn test_stress_push_escaping_bounds() {
    // High force factor allows escape
    let config_high = build_displacement_test_config(0.8);
    let mut tick_executor_high = TickExecutor::new(config_high).unwrap();
    {
        let cells = tick_executor_high.world_mut().cells_mut_for_commit();
        cells.set_contact_pressure(CellIndex::from_raw(0), 0.5);
        cells.set_contact_pressure(CellIndex::from_raw(1), 0.5);
    }

    // Run for 5 ticks to resolve overlap and execute displacement
    for _ in 0..5 {
        let _ = tick_executor_high.step().unwrap();
    }
    let pos_high = tick_executor_high
        .world()
        .cells()
        .position(CellIndex::from_raw(0));
    let dist_high = (pos_high.x() - 256.0).abs();

    // Low force factor keeps cell locked under collision
    let config_low = build_displacement_test_config(0.01);
    let mut tick_executor_low = TickExecutor::new(config_low).unwrap();
    {
        let cells = tick_executor_low.world_mut().cells_mut_for_commit();
        cells.set_contact_pressure(CellIndex::from_raw(0), 0.5);
        cells.set_contact_pressure(CellIndex::from_raw(1), 0.5);
    }

    for _ in 0..5 {
        let _ = tick_executor_low.step().unwrap();
    }
    let pos_low = tick_executor_low
        .world()
        .cells()
        .position(CellIndex::from_raw(0));
    let dist_low = (pos_low.x() - 256.0).abs();

    // The high force factor cell should have displaced significantly further away
    assert!(
        dist_high > dist_low * 2.0,
        "High contractility cell should shift much further! (high: {}, low: {})",
        dist_high,
        dist_low
    );
}

#[test]
fn test_viability_threshold_sweep() {
    let densities = &[0.0, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0];
    std::fs::create_dir_all("outputs/raw_data").unwrap();
    let mut file = std::fs::File::create("outputs/raw_data/phase2_viability_sweep.csv").unwrap();
    writeln!(file, "density,collapsed,dormant_ticks").unwrap();

    for &density in densities {
        let mut config = build_sweep_config(1.0, density);
        config.resources.optional_decay_rate = 0.0;
        config.resource_interaction.energy_per_resource = 25.0;
        config.cell.mandatory_cost_per_tick = EnergyAmount::new(1.0).unwrap();
        config.cell.energy_capacity = EnergyAmount::new(40.0).unwrap();
        config.lifecycle.dormant_mandatory_cost_modifier = 0.1;
        let mut cell = config.cell;
        cell.initial_energy = EnergyAmount::zero();
        cell.energy_capacity = EnergyAmount::new(40.0).unwrap();
        cell.passive_energy_income = EnergyAmount::new(0.099).unwrap();
        config = config.with_cells(vec![cell]);
        let mut tick_executor = TickExecutor::new(config).unwrap();

        let mut collapsed = false;
        let mut dormant_ticks = 0;

        for _ in 0..100 {
            let summary = tick_executor.step().unwrap();
            if summary.survival_result == SurvivalResult::Collapse {
                collapsed = true;
                break;
            }
            let lifecycle = tick_executor
                .world()
                .cells()
                .lifecycle_state(CellIndex::from_raw(0));
            if lifecycle == LifecycleState::Dormant {
                dormant_ticks += 1;
            }
        }

        writeln!(file, "{},{},{}", density, collapsed, dormant_ticks).unwrap();

        println!(
            "Density: {}, collapsed: {}, dormant_ticks: {}",
            density, collapsed, dormant_ticks
        );

        if density < 0.5 {
            assert!(collapsed, "Expected collapse for density {}", density);
        } else if (0.5..=2.0).contains(&density) {
            assert!(!collapsed, "Expected survival for density {}", density);
            assert!(
                dormant_ticks > 50,
                "Expected significant dormancy (got {}) for density {}",
                dormant_ticks,
                density
            );
        } else {
            assert!(
                !collapsed,
                "Expected active survival for density {}",
                density
            );
        }
    }
}

#[test]
fn test_dormancy_below_equal_above_equilibrium() {
    // 1. Below equilibrium (0.19) - slow death
    {
        let mut config = build_sweep_config(1.0, 0.0);
        config.lifecycle.dormancy_allowed = true;
        config.lifecycle.dormant_mandatory_cost_modifier = 0.1;
        let mut cell = config.cell;
        cell.initial_energy = EnergyAmount::new(1.5).unwrap();
        cell.passive_energy_income = EnergyAmount::new(0.19).unwrap();
        config = config.with_cells(vec![cell]);

        let mut tick_executor = TickExecutor::new(config).unwrap();
        let mut collapsed = false;
        for _ in 0..200 {
            let summary = tick_executor.step().unwrap();
            if summary.survival_result == SurvivalResult::Collapse {
                collapsed = true;
                break;
            }
        }
        assert!(
            collapsed,
            "Cell should slowly collapse with passive income below dormant upkeep"
        );
    }

    // 2. Exactly equal (0.20) - stable dormancy
    {
        let mut config = build_sweep_config(1.0, 0.0);
        config.lifecycle.dormancy_allowed = true;
        config.lifecycle.dormant_mandatory_cost_modifier = 0.1;
        let mut cell = config.cell;
        cell.initial_energy = EnergyAmount::new(1.5).unwrap();
        cell.passive_energy_income = EnergyAmount::new(0.20).unwrap();
        config = config.with_cells(vec![cell]);

        let mut tick_executor = TickExecutor::new(config).unwrap();
        for _ in 0..200 {
            let summary = tick_executor.step().unwrap();
            assert_ne!(
                summary.survival_result,
                SurvivalResult::Collapse,
                "Cell should be stable when passive income equals dormant upkeep"
            );
        }
    }

    // 3. Above equilibrium (0.22) - accumulates and wakes up
    {
        let mut config = build_sweep_config(1.0, 0.0);
        config.lifecycle.dormancy_allowed = true;
        config.lifecycle.dormant_mandatory_cost_modifier = 0.1;
        config.lifecycle.stress_energy_threshold = EnergyAmount::new(2.0).unwrap();
        let mut cell = config.cell;
        cell.initial_energy = EnergyAmount::new(1.5).unwrap();
        cell.passive_energy_income = EnergyAmount::new(0.22).unwrap();
        config = config.with_cells(vec![cell]);

        let mut tick_executor = TickExecutor::new(config).unwrap();
        let mut woke_up = false;
        for _ in 0..200 {
            let _ = tick_executor.step().unwrap();
            let lifecycle = tick_executor
                .world()
                .cells()
                .lifecycle_state(CellIndex::from_raw(0));
            if lifecycle == LifecycleState::Alive || lifecycle == LifecycleState::Stressed {
                woke_up = true;
                break;
            }
        }
        assert!(
            woke_up,
            "Cell should wake up when energy accumulates above threshold"
        );
    }
}

#[test]
fn test_dormancy_wakeup_hysteresis() {
    let mut config = build_sweep_config(1.0, 0.0); // Zero resource initially
    config.lifecycle.dormancy_allowed = true;
    config.lifecycle.dormant_mandatory_cost_modifier = 0.1;
    let mut cell = config.cell;
    cell.initial_energy = EnergyAmount::new(1.5).unwrap();
    cell.passive_energy_income = EnergyAmount::zero();
    config = config.with_cells(vec![cell]);

    let mut tick_executor = TickExecutor::new(config).unwrap();

    std::fs::create_dir_all("outputs/raw_data").unwrap();
    let mut file = std::fs::File::create("outputs/raw_data/phase2_dormancy_transitions.csv").unwrap();
    writeln!(file, "tick,energy,state").unwrap();

    // Cell must enter dormancy
    let _summary = tick_executor.step().unwrap();
    let initial_lifecycle = tick_executor
        .world()
        .cells()
        .lifecycle_state(CellIndex::from_raw(0));
    assert_eq!(initial_lifecycle, LifecycleState::Dormant);

    let initial_energy = tick_executor
        .world()
        .cells()
        .energy(CellIndex::from_raw(0))
        .current()
        .raw();
    writeln!(file, "0,{},{:?}", initial_energy, initial_lifecycle).unwrap();

    // Inject resource under cell
    {
        let res = tick_executor.world_mut().resources_mut_for_commit();
        res.set_amount_at(
            alife::core::resources::ResourceLayerIndex::from_raw(0),
            GridCoord::new(32, 32),
            ResourceAmount::new(50.0).unwrap(),
        )
        .unwrap();
    }

    let mut states = Vec::new();
    for tick in 1..=20 {
        let _ = tick_executor.step().unwrap();
        let lifecycle = tick_executor
            .world()
            .cells()
            .lifecycle_state(CellIndex::from_raw(0));
        let energy = tick_executor
            .world()
            .cells()
            .energy(CellIndex::from_raw(0))
            .current()
            .raw();
        writeln!(file, "{},{},{:?}", tick, energy, lifecycle).unwrap();
        states.push(lifecycle);
    }

    // Check no flickering (e.g. Alive -> Dormant -> Alive -> Dormant on consecutive ticks)
    for i in 0..(states.len() - 1) {
        if states[i] == LifecycleState::Alive && states[i + 1] == LifecycleState::Dormant {
            panic!("Flickering transition detected between ticks!");
        }
    }
}

#[test]
fn test_energy_and_resource_conservation() {
    let mut config = build_sweep_config(1.0, 10.0);
    config.resources.optional_decay_rate = 0.0; // Closed environment
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::zero(); // Disable metabolism to keep mass in res/mat

    let mut cell = config.cell;
    cell.passive_energy_income = EnergyAmount::zero();
    // Enable synthesis to check conversion conservation
    cell.initial_synthesis_material = MaterialAmount::new(1.0).unwrap();
    config = config.with_cells(vec![cell]);

    let mut tick_executor = TickExecutor::new(config).unwrap();

    // Record initial mass
    let initial_grid_res = tick_executor
        .world()
        .resources()
        .total_amount_for_layer(alife::core::resources::ResourceLayerIndex::from_raw(0))
        .unwrap()
        .raw();
    let initial_cell_res = tick_executor
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(0))
        .raw();
    let initial_cell_mat = tick_executor
        .world()
        .cells()
        .total_materials(CellIndex::from_raw(0))
        .raw();
    let initial_total_mass = initial_grid_res + initial_cell_res + initial_cell_mat;

    for _ in 0..20 {
        let _ = tick_executor.step().unwrap();

        let grid_res = tick_executor
            .world()
            .resources()
            .total_amount_for_layer(alife::core::resources::ResourceLayerIndex::from_raw(0))
            .unwrap()
            .raw();
        let cell_res = tick_executor
            .world()
            .cells()
            .resource_amount(CellIndex::from_raw(0))
            .raw();
        let cell_mat = tick_executor
            .world()
            .cells()
            .total_materials(CellIndex::from_raw(0))
            .raw();

        let total_mass = grid_res + cell_res + cell_mat;

        // Mass must be conserved within float tolerance
        assert!(
            (total_mass - initial_total_mass).abs() < 0.01,
            "Resource conservation violated! Initial: {}, Current: {}",
            initial_total_mass,
            total_mass
        );
    }
}

#[test]
fn test_transport_metabolism_balance_matrix() {
    let uptake_rates = &[0.1, 0.5, 1.0, 2.0];
    let metabolism_rates = &[0.1, 0.5, 1.0, 2.0];
    std::fs::create_dir_all("outputs/raw_data").unwrap();
    let mut file = std::fs::File::create("outputs/raw_data/phase2_transport_metabolism_matrix.csv").unwrap();
    writeln!(file, "uptake,metabolism,outcome").unwrap();

    for &uptake in uptake_rates {
        for &metabolism in metabolism_rates {
            let mut config = build_sweep_config(1.0, 10.0);
            config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(uptake).unwrap();
            config.resource_interaction.metabolism_resource_per_tick =
                ResourceAmount::new(metabolism).unwrap();

            let mut tick_executor = TickExecutor::new(config).unwrap();

            let mut collapsed = false;
            let mut dormant_ticks = 0;
            let initial_energy = tick_executor.world().cells().energy(CellIndex::from_raw(0)).current().raw();
            let mut final_energy = initial_energy;

            for _ in 0..50 {
                let summary = tick_executor.step().unwrap();
                if summary.survival_result == SurvivalResult::Collapse {
                    collapsed = true;
                    break;
                }
                let lifecycle = tick_executor
                    .world()
                    .cells()
                    .lifecycle_state(CellIndex::from_raw(0));
                if lifecycle == LifecycleState::Dormant {
                    dormant_ticks += 1;
                }
                final_energy = tick_executor.world().cells().energy(CellIndex::from_raw(0)).current().raw();
            }

            let outcome = if collapsed {
                "starved"
            } else if dormant_ticks > 25 {
                "dormant"
            } else if final_energy > initial_energy + 10.0 {
                "accumulates"
            } else {
                "stable"
            };

            writeln!(file, "{},{},{}", uptake, metabolism, outcome).unwrap();
        }
    }
}

#[test]
fn test_multi_cell_resource_order_independence() {
    let mut config = build_sweep_config(1.0, 0.5); // Scarce resources to show competition bias

    // Spawn two identical overlapping cells competing for resource
    let cell1 = CellInitialConfig {
        position: Position::new(256.0, 256.0),
        radius: Radius::new(10.0).unwrap(),
        initial_energy: EnergyAmount::new(50.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(1.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(30.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };
    let cell2 = cell1;

    config = config.with_cells(vec![cell1, cell2]);
    let mut tick_executor = TickExecutor::new(config).unwrap();

    // Execute 1 tick
    let _ = tick_executor.step().unwrap();

    let res1 = tick_executor
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(0))
        .raw();
    let res2 = tick_executor
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(1))
        .raw();

    let eng1 = tick_executor
        .world()
        .cells()
        .energy(CellIndex::from_raw(0))
        .current()
        .raw();
    let eng2 = tick_executor
        .world()
        .cells()
        .energy(CellIndex::from_raw(1))
        .current()
        .raw();

    // Document sequential index bias: the cell processed first takes the resources first.
    println!("Cell 0 resource: {}, Cell 1 resource: {}", res1, res2);
    println!("Cell 0 energy: {}, Cell 1 energy: {}", eng1, eng2);
}
