# REPORT: Phase 1 Rust Config Validation

## Goal
Validate Phase 1 tuned/stability scenarios against the authoritative Rust Phase 1 core, and compare behavior against the Python preflight estimator.

## Authority Rule
- **Rust Phase 1 core** is the authoritative implementation of Phase 1 simulation behavior.
- **Python early-stability** is a preflight estimator/tuner and is used only for sanity checks and diagnosis.

## Scenarios Checked

| Scenario | Expected Rust Result | Actual Rust Result | Expected Reason | Actual Reason | Expected Tick | Actual Tick | Expected Lifecycle | Actual Lifecycle | Notes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| **single_cell_survival** | Stable | Stable | None | None | 100 | 100 | Alive | Alive | Full convergence. |
| **single_cell_starvation** | Collapse | Collapse | MandatoryCostUnpaid | MandatoryCostUnpaid | 1 | 1 | Dead | Dead | Full convergence. |
| **single_cell_dormancy** | Collapse | Collapse | EnergyDepleted | EnergyDepleted | 2 | 2 | Dead | Dead | Matches dynamic simulator. (Python static check says `mandatory_cost_unpaid` due to estimator limitations). |
| **single_cell_heat_death** | Collapse | Collapse | HeatLimitExceeded | HeatLimitExceeded | 3 | 3 | Dead | Dead | Full convergence. |
| **single_cell_waste_death** | Collapse | Collapse | WasteLimitExceeded | WasteLimitExceeded | 3 | 3 | Dead | Dead | Full convergence. |
| **single_cell_over_capacity** | Collapse | Collapse | CapacityExceeded | CapacityExceeded | 1 | 1 | Dead | Dead | Rust collapses at Tick 1. (Python loader rejects statically as `invalid_config`). |

## Rust Result Dump
```text
scenario_id,survival_result,collapse_reason,tick,final_energy,heat,waste,lifecycle
single_cell_survival,Stable,None,100,50.000,0.000,0.000,Alive
single_cell_starvation,Collapse,MandatoryCostUnpaid,1,1.000,0.000,0.000,Dead
single_cell_dormancy,Collapse,EnergyDepleted,2,0.000,0.000,0.000,Dead
single_cell_heat_death,Collapse,HeatLimitExceeded,3,50.000,15.000,0.000,Dead
single_cell_waste_death,Collapse,WasteLimitExceeded,3,50.000,0.000,15.000,Dead
single_cell_over_capacity,Collapse,CapacityExceeded,1,53.000,0.000,0.000,Dead
```

## Diagnosis of Mismatch Cases

### 1. Dormancy Mismatch (`single_cell_dormancy`)
- **Actual Behavior:** Both the Python micro-simulator and the Rust core agree that the cell successfully enters dormancy at Tick 1 (`energy = 0.5`, state = `Dormant`) and collapses at Tick 2 with `EnergyDepleted` once its remaining energy drops to `0.0`.
- **Mismatch Cause:** The Python `batch` command runs a static budget check (`evaluate_static_bounds`) before deciding to run the simulator. Because the static check does not account for dormancy, it flags the scenario as `collapse` with `mandatory_cost_unpaid` and skips running the simulator.
- **Verdict:** Authoritative Rust behavior is correct. The difference is a Python preflight estimator limitation.

### 2. Over Capacity Mismatch (`single_cell_over_capacity`)
- **Actual Behavior:** Rust initializes the world state successfully, but during Tick 1 evaluation, the cell is flagged with critical capacity overrun and collapses with `CapacityExceeded`.
- **Mismatch Cause:** Python's config loader validates the initial resource and material sum against the capacity limit before starting. Because `20.0 (resources) + 5.0 (materials) > 15.0 (capacity_limit)`, it raises a validation error and aborts with `invalid_config`.
- **Verdict:** Both engines successfully prevent the over-capacity cell from surviving, but do so at different lifecycle stages. This is a semantics/validation boundary difference.

---

## Technical Actions & Fixes
- **Loop Control Bug Fixed:** Fixed an off-by-one bug in `TickExecutor::run_until_configured_tick()` where the executor would execute one extra step *after* a collapse had occurred, returning Tick 2 instead of Tick 1 for starvation/over-capacity.

---

## Verification
- All 25 Rust unit/integration tests pass.
- Clippy completed successfully with no warnings.
- All Python tools/early-stability pytest checks (93 passed) remain fully functional.
- No files have been committed to Git.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
