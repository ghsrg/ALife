# REPORT: Mechanism Reachability

## Run Summary
- **Stability Ranges Reference**: `outputs/stability/` (Group 1 - 5 sweeps)
- **Scenarios Evaluated**: `single_cell_survival`, `single_cell_starvation`, `single_cell_heat_stress`, `waste_heat_balance`, `single_cell_growth_budget`, `single_cell_division_loop_estimate`, `population_growth_bound`, `joint_upkeep_budget`.
- **Mechanisms Evaluated**: Mandatory Cost, Passive Income, Capacity Limit, Heat Gen, Heat Diss, Waste Gen, Waste Sink, Growth Estimate, Joint Upkeep Estimate.

## Mechanism Reachability Table (Survival Scenario Baseline)
| Mechanism | Status | Reachability Result | Block Reason | Executed | Effect Nonzero | Bypass | Notes |
| --- | --- | --- | --- | ---: | ---: | ---: | --- |
| **mandatory_energy_cost** | now | pass | none | 100 | 100 | 0 | Evaluated successfully over history. |
| **passive_energy_income** | now | warning | competing_path_cheaper | 100 | 100 | 1 | Passive income may bypass uptake/metabolism pressure. |
| **capacity_limit** | now | warning | capacity_too_high | 1 | 0 | 1 | Capacity too high to constrain storage. |
| **heat_generation** | now | pass | none | 1 | 1 | 0 | Reachable and active. |
| **heat_dissipation** | now | pass | none | 1 | 1 | 0 | Reachable and active. |
| **waste_generation** | now | pass | none | 1 | 1 | 0 | Reachable and active. |
| **waste_sink** | now | pass | none | 1 | 1 | 0 | Reachable and active. |
| **growth_estimate** | estimate_only | tool_limited | tool_limited | 0 | 0 | 0 | Not consumed by micro-simulator. |
| **joint_upkeep_estimate** | estimate_only | tool_limited | tool_limited | 0 | 0 | 0 | Not consumed by micro-simulator. |

## Failed / Blocked / Bypassed Mechanisms
- **mandatory_energy_cost**: Blocked in `single_cell_starvation` due to `mandatory_cost_unpaid`. This proves that the mechanism is reachable and active.
- **passive_energy_income**: Bypassed in `single_cell_survival` because passive income (5.0) is higher than mandatory cost (2.0), leaving no pressure to forage/eat.
- **capacity_limit**: Bypassed in `single_cell_survival` because starting resources/materials (8.0) are far below the capacity limit (50.0). Constrained and blocked in `single_cell_over_capacity`.
- **heat_generation / heat_dissipation**: Reachable in survival, triggers death condition (`heat_limit_exceeded`) in `single_cell_heat_stress`.
- **waste_generation / waste_sink**: Reachable in survival, keeps waste stable in `waste_heat_balance`.

## Tool-Limited / Future-Only Mechanisms
- **growth_estimate** and **joint_upkeep_estimate**: Both are marked as `tool_limited` / `estimate_only` because the micro-simulator does not run actual cell division, cell growth, or multicellular joints. They are accepted by config but not consumed by the simulator.

## Recommended Tuning Adjustments
1. **Reduce Passive Energy Income**: To make active resource uptake and metabolism necessary (resolving the `competing_path_cheaper` bypass of passive energy), reduce `passive_energy_income_placeholder` from 5.0 to 1.0 or 0.0 in survival scenarios, and introduce resource absorption settings.
2. **Tighten Capacity Limit**: To make storage constraints active, reduce `capacity_limit` close to the initial resources+materials sum (e.g. from 50.0 to 10.0 or 15.0).

## Decision
Proceed to data model docs: **partial**
- Revisit **Energy Budget Sweep** (Group 1) to test low/zero passive income and introduce resource uptake mechanisms.
- Revisit **Capacity And Material Sweep** (Group 3) to test tighter capacity limits.
