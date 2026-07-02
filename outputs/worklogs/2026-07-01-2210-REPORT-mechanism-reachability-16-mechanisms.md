# REPORT: Mechanism Reachability with 16 Mechanisms

## Goal
Evaluate mechanism reachability on the expanded 16-mechanisms registry and verify that baseline parameters are properly tuned to prevent warnings and bypasses.

## Scope
- Run the reachability parser on `single_cell_survival` scenario using the updated `mechanisms/phase1.toml` containing 16 mechanisms.
- Document convergence and check for warnings.

## Decisions / Findings
1.  **Baseline Parameter Stability**: The previously adjusted baseline parameters (`passive_energy_income_placeholder = 2.0` and `capacity_limit = 30.0`) successfully satisfy all 16 mechanisms.
2.  **No Warnings or Bypasses**:
    *   `passive_energy_income` is stable without bypassing uptake mechanisms since income (2.0) equals the mandatory cost per tick (2.0).
    *   `capacity_limit` successfully constrains storage without triggering a bypass warning since capacity (30.0) is under $5 \times$ starting stored assets (8.0).
3.  **New Mechanisms Passing**:
    *   `energy_buffer_clamp` passes (energy stays <= capacity).
    *   `stress_state` and `dormancy` pass (registered and verified).
    *   `death_by_energy`, `death_by_heat`, and `death_by_waste` guards are verified and pass without being triggered.
    *   `candidate_config_validation` passes.

## Reachability Outputs (16 Mechanisms)
| Mechanism | Status | Reachability Result | Block Reason | Executed | Effect Nonzero | Bypass | Notes |
| --- | --- | --- | --- | ---: | ---: | ---: | --- |
| **mandatory_energy_cost** | now | pass | none | 100 | 100 | 0 | Evaluated successfully. |
| **passive_energy_income** | now | pass | none | 100 | 100 | 0 | Passive income is active but does not dominate. |
| **capacity_limit** | now | pass | none | 1 | 1 | 0 | Capacity limit is active. |
| **energy_buffer_clamp** | now | pass | none | 100 | 0 | 0 | Energy stayed within capacity. |
| **heat_generation** | now | pass | none | 1 | 1 | 0 | Reachable. |
| **heat_dissipation** | now | pass | none | 1 | 1 | 0 | Reachable. |
| **waste_generation** | now | pass | none | 1 | 1 | 0 | Reachable. |
| **waste_sink** | now | pass | none | 1 | 1 | 0 | Reachable. |
| **stress_state** | now | pass | none | 100 | 0 | 0 | Stress state guard active; 0 matching ticks. |
| **dormancy** | now | pass | none | 100 | 0 | 0 | Dormancy state guard active; 0 matching ticks. |
| **death_by_energy** | now | pass | none | 1 | 0 | 0 | Energy death guard active; not triggered. |
| **death_by_heat** | now | pass | none | 1 | 0 | 0 | Heat death guard active; not triggered. |
| **death_by_waste** | now | pass | none | 1 | 0 | 0 | Waste death guard active; not triggered. |
| **candidate_config_validation** | now | pass | none | 1 | 1 | 0 | Validation accepted the config. |
| **growth_estimate** | estimate_only | tool_limited | tool_limited | 0 | 0 | 0 | Not consumed by simulator. |
| **joint_upkeep_estimate** | estimate_only | tool_limited | tool_limited | 0 | 0 | 0 | Not consumed by simulator. |

## Decision
Proceed to data model docs: **yes** (for all evaluated 14 core Phase 1 mechanisms).

---

## Do Not Change
- Canon rules unchanged.
- Source configs unchanged unless explicitly requested.
