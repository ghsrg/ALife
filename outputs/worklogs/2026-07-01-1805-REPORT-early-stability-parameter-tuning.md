# REPORT: Early Stability Parameter Tuning

## Commands Run
- `python cli.py batch --scenarios ../scenarios --out ../../../outputs/stability/baseline-batch --with-simulation`
- `python cli.py tune --scenario ../scenarios/single_cell_survival.toml --tuning ../tuning/single_cell.toml --out ../../../outputs/stability/single_cell_tune/`

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

## Tune Results
| Run | Stable | Fragile | Collapse | Invalid |
| --- | --- | --- | --- | --- |
| single_cell_tune (3000 runs) | 1350 | 750 | 900 | 0 |

## Recommended Base Values
| Parameter | Current | Recommended | Tested Min | Tested Max | Stable Min | Stable Max | Reason |
| --- | --- | --- | --- | --- | --- | --- | --- |
| cell.initial_energy | 50.0 | 30.0 | 5.0 | 50.0 | 10.0 | 50.0 | Midpoint of stable range [10.0, 50.0] avoids initial stress state (stress threshold is 10.0). |
| cell.mandatory_cost_per_tick | 2.0 | 3.0 | 1.0 | 10.0 | 1.0 | 5.0 | Midpoint of stable range [1.0, 5.0] where passive energy income (5.0) can cover costs. |
| environment.heat_dissipation_rate | 0.2 | 0.275 | 0.05 | 0.5 | 0.05 | 0.5 | Midpoint of stable range [0.05, 0.5]. Heat dissipation is stable across the whole tested range since heat generation is low (0.1). |

## Boundary Values
| Parameter | Fragile Min | Fragile Max | Collapse Edge | Notes |
| --- | --- | --- | --- | --- |
| cell.initial_energy | 5.0 | 5.0 | 0.0 | Initial energy < 10.0 triggers immediate stress warning (fragile), but eventually recovers. |
| cell.mandatory_cost_per_tick | 6.0 | 7.0 | 8.0 | Cost > passive income (5.0) drains energy, triggering dormancy cycles or starvation. |
| environment.heat_dissipation_rate | N/A | N/A | < 0.05 | Heat generated (0.1) is dissipation rate limit. |

## Failure Reasons
- `energy_depleted`: energy becomes <= 0.0 when costs exceed income and capacity/initial energy is depleted.
- `mandatory_cost_unpaid`: active cost cannot be paid and dormancy is either disabled or cannot pay dormant costs.
- `heat_limit_exceeded`: heat exceeds death threshold (20.0).

## Proposed Changes
- Proposed scenario default updates (not modified in source code):
  - Change default `cell.initial_energy` to `30.0` (current: `50.0`, both are stable but `30.0` represents a balanced midpoint).
  - Change default `cell.mandatory_cost_per_tick` to `3.0` (current: `2.0`, both are stable but `3.0` provides a better stability margin check).
  - Change default `environment.heat_dissipation_rate` to `0.275` (current: `0.2`).

## Do Not Change
- Canon rules unchanged
- Source configs unchanged unless explicitly requested
