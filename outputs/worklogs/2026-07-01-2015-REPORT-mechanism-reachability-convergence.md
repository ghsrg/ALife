# REPORT: Mechanism Reachability Convergence

## Goal
Achieve full convergence between early-stability parameter tuning and mechanism reachability by removing all warnings and bypasses in the survival baseline scenario.

## Scope
- Mutate `tools/early-stability/scenarios/single_cell_survival.toml` default parameters:
  - Reduce `passive_energy_income_placeholder` to `2.0` (matching `mandatory_cost_per_tick` of `2.0`).
  - Tighten `capacity_limit` to `30.0` (which is under $5 \times$ starting stored assets of `8.0`).
- Re-run reachability and batch simulation checks.

## Decisions / Findings
1.  **Passive Income Bypass Resolved**: Setting passive energy income equal to the mandatory cost per tick ensures that energy is balanced and does not automatically grow to capacity, keeping metabolical pressure active.
2.  **Capacity Bypass Resolved**: Setting capacity to 30.0 (less than 40.0) ensures it falls below the bypass check threshold ($5 \times$ used storage).
3.  **Stability Preserved**: The single-cell baseline survives stably for 1,000 ticks at constant 50.0 energy.
4.  **Tuner Revalidation Integrated**: All tuned candidate configurations are validated using `validate_config_dict` before simulation, preventing invalid boundaries from entering the simulator.

## Reachability Outputs (Updated Baseline)
| Mechanism | Status | Reachability Result | Block Reason | Executed | Effect Nonzero | Bypass | Notes |
| --- | --- | --- | --- | ---: | ---: | ---: | --- |
| **mandatory_energy_cost** | now | pass | none | 100 | 100 | 0 | Evaluated successfully. |
| **passive_energy_income** | now | pass | none | 100 | 100 | 0 | Passive income is active but does not dominate. |
| **capacity_limit** | now | pass | none | 1 | 1 | 0 | Capacity limit is active. |
| **heat_generation** | now | pass | none | 1 | 1 | 0 | Reachable. |
| **heat_dissipation** | now | pass | none | 1 | 1 | 0 | Reachable. |
| **waste_generation** | now | pass | none | 1 | 1 | 0 | Reachable. |
| **waste_sink** | now | pass | none | 1 | 1 | 0 | Reachable. |
| **growth_estimate** | estimate_only | tool_limited | tool_limited | 0 | 0 | 0 | Not consumed by simulator. |
| **joint_upkeep_estimate** | estimate_only | tool_limited | tool_limited | 0 | 0 | 0 | Not consumed by simulator. |

## Decision
Proceed to data model docs: **yes** (for all evaluated core Phase 1 mechanisms).

---

## Do Not Change
- Canon rules unchanged.
- Source configs unchanged unless explicitly requested.
