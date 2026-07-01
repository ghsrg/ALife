# REPORT: Early Stability Parameter Tuning (Multi-Group Sweeps)

## Commands Run
- `python cli.py batch --scenarios ../scenarios --out ../../../outputs/stability/baseline-batch-sim --with-simulation`
- `python cli.py tune --scenario ../scenarios/single_cell_survival.toml --tuning ../tuning/group1_energy.toml --out ../../../outputs/stability/group1_energy_tune/`
- `python cli.py tune --scenario ../scenarios/single_cell_survival.toml --tuning ../tuning/group2_heat_waste.toml --out ../../../outputs/stability/group2_heat_waste_tune/`
- `python cli.py tune --scenario ../scenarios/single_cell_survival.toml --tuning ../tuning/group3_capacity.toml --out ../../../outputs/stability/group3_capacity_tune/`
- `python cli.py tune --scenario ../scenarios/single_cell_survival.toml --tuning ../tuning/group4_growth.toml --out ../../../outputs/stability/group4_growth_tune/`
- `python cli.py tune --scenario ../scenarios/single_cell_survival.toml --tuning ../tuning/group5_joint.toml --out ../../../outputs/stability/group5_joint_tune/`

## Baseline Results
| Scenario | Static Result | Simulation Result | Reason |
| --- | --- | --- | --- |
| joint_upkeep_budget | stable | stable | none |
| population_growth_bound | stable | stable | none |
| single_cell_division_loop_estimate | stable | stable | none |
| single_cell_growth_budget | stable | stable | none |
| single_cell_heat_stress | collapse | collapse | heat_limit_exceeded |
| single_cell_over_capacity | invalid | invalid | invalid_config |
| single_cell_starvation | collapse | collapse | mandatory_cost_unpaid |
| single_cell_survival | stable | stable | none |
| waste_heat_balance | stable | stable | none |

## Tune Results (Per Group)
| Group | Sweep Focus | Stable | Fragile | Collapse | Invalid | Total Runs |
| --- | --- | --- | --- | --- | --- | --- |
| **Group 1** | Energy Budget Sweep | 1566 | 225 | 396 | 0 | 2187 |
| **Group 2** | Heat & Waste Sweep | 588 | 180 | 0 | 0 | 768 |
| **Group 3** | Capacity & Material Sweep | 639 | 90 | 0 | 0 | 729 |
| **Group 4** | Growth & Division Sweep | 2187 | 0 | 0 | 0 | 2187 |
| **Group 5** | Joint Upkeep Sweep | 567 | 81 | 81 | 0 | 729 |

---

## Recommended Base Values (Compiled Across Groups)

| Parameter ID | Current Base | Recommended | Stable Min | Stable Max | Sweep Confidence | Reason / Findings |
| --- | --- | --- | --- | --- | --- | --- |
| `cell.initial_energy` | 50.0 | **30.0** | 10.0 | 50.0 | High | Midpoint of stable range [10.0, 50.0] avoids initial stress warning. |
| `cell.energy_capacity` | 100.0 | **100.0** | 50.0 | 150.0 | High | Base value is stable. Higher capacity allows larger buffers. |
| `cell.mandatory_cost_per_tick` | 2.0 | **3.0** | 1.0 | 5.0 | High | Midpoint. Higher values are viable if passive income is adjusted. |
| `resources.passive_energy_income_placeholder` | 5.0 | **5.0** | 2.0 | 8.0 | High | Base value is well-suited. Must remain above mandatory cost. |
| `lifecycle.stress_energy_threshold` | 10.0 | **10.0** | 5.0 | 15.0 | High | Base value is suitable. Higher thresholds trigger fragile warnings earlier. |
| `cell.dormant_mandatory_cost_modifier` | 0.1 | **0.3** | 0.1 | 0.5 | High | Midpoint. Low modifier increases dormancy survival duration. |
| `environment.heat_generated_per_tick` | 0.1 | **0.15** | 0.05 | 0.25 | High | Moderate heat generation is stable with appropriate dissipation. |
| `environment.heat_dissipation_rate` | 0.2 | **0.20** | 0.1 | 0.3 | High | Base value is stable. Higher rate prevents thermal collapse. |
| `environment.waste_generated_per_tick` | 0.05 | **0.06** | 0.02 | 0.1 | High | Stable under current sink rates. |
| `environment.waste_sink_rate` | 0.1 | **0.10** | 0.05 | 0.15 | High | Base value is suitable. |
| `cell.capacity_limit` | 50.0 | **30.0** | 10.0 | 50.0 | High | Avoids config warnings while leaving room for water/nutrient/cell wall. |
| `cell.radius` | 1.0 | **1.0** | 0.5 | 1.5 | High | Radius has no direct micro-simulator impact, but bounds volume capacity. |

*Note: Estimate parameters (Group 4 and 5) have no runtime simulation impact, but are verified to align with baseline inputs.*

---

## Boundary / Fragile Values
| Parameter | Fragile Min | Fragile Max | Collapse Edge | Notes |
| --- | --- | --- | --- | --- |
| `cell.initial_energy` | 5.0 | 5.0 | 0.0 | Below stress threshold (10.0) triggers immediate stressed warning. |
| `cell.mandatory_cost_per_tick` | 6.0 | 7.0 | >= 8.0 | Forces cell into dormancy loops or starvation when cost > passive income. |
| `cell.capacity_limit` | < 8.0 | N/A | < 8.0 | Stressed state if capacity_limit < initial_resources + materials (currently 8.0). |

## Failure Reasons
- `energy_depleted`: energy <= 0.0 when costs exceed income and capacity/initial energy is depleted.
- `mandatory_cost_unpaid`: active cost cannot be paid and dormancy is either disabled or cannot pay dormant costs.
- `heat_limit_exceeded`: heat exceeds death threshold (20.0).

## Proposed Changes
- Default parameter suggestions for single-cell baseline:
  - Set default `cell.initial_energy` to `30.0` (midpoint of stable range).
  - Set default `cell.mandatory_cost_per_tick` to `3.0`.
  - Set default `environment.heat_dissipation_rate` to `0.20`.
- These changes are documented for calibration but not modified in source code.

## Do Not Change
- Canon rules unchanged.
- Source configs unchanged unless explicitly requested.
