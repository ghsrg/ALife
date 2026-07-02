# Phase 1 Rust Config Validation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Validate current Phase 1 stability scenarios against the Rust Phase 1 core and produce a worklog report with results, mismatches and follow-up recommendations.

**Architecture:** Rust Phase 1 core is the implementation truth for Phase 1 behavior. Python `early-stability` is a preflight estimator/tuner and may be used only as a sanity comparison when Rust results are surprising. This task adds Rust-side scenario fixtures and validation tests, runs them, optionally runs Python for mismatch diagnosis, and writes a report.

**Tech Stack:** Rust 2024, Cargo integration tests, existing Python `tools/early-stability` CLI only for optional sanity comparison, Markdown worklog report.

---

## Authority Rule

Do not treat Python `early-stability` as oracle for Phase 1 behavior.

Use this rule:

```text
Rust Phase 1 core = authoritative for implemented Phase 1 behavior.
Python early-stability = estimator, tuner and diagnostic comparison tool.
Mismatch with Python does not automatically mean Rust is wrong.
```

If Rust behavior conflicts with accepted docs, report it as a Rust bug candidate.

If Rust behavior differs from Python but matches accepted docs, report it as Python estimator limitation or config-semantics mismatch.

Do not tune parameters and do not change Rust core behavior in this task.

---

## Required Reading

Read before making changes:

- `docs/PRINCIPLES.md`
- `docs/GLOSSARY.md`
- `docs/implementation/phase-1-design.md`
- `docs/implementation/phase-1-data-model.md`
- `docs/implementation/phase-1-module-api.md`
- `docs/implementation/early-stability-tool.md`
- `tools/early-stability/README.md`
- `outputs/worklogs/2026-07-02-1359-REPORT-phase-1-rust-core.md`

---

## Scenario Scope

Validate these Phase 1 executable scenarios first:

```text
tools/early-stability/scenarios/single_cell_survival.toml
tools/early-stability/scenarios/single_cell_starvation.toml
tools/early-stability/scenarios/single_cell_dormancy.toml
tools/early-stability/scenarios/single_cell_heat_death.toml
tools/early-stability/scenarios/single_cell_waste_death.toml
tools/early-stability/scenarios/single_cell_over_capacity.toml
```

Do not validate these as Rust Phase 1 behavior scenarios:

```text
single_cell_growth_budget.toml
single_cell_division_loop_estimate.toml
population_growth_bound.toml
joint_upkeep_budget.toml
```

They contain estimate/post-Phase1 concepts and belong to later phases or Python-only research checks.

`waste_heat_balance.toml` and `single_cell_heat_stress.toml` may be added after the six core scenarios pass or produce a clear report.

---

## File Structure

Create:

```text
tests/phase1_config_validation.rs
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1-rust-config-validation.md
```

Modify:

```text
outputs/worklogs/README.md
```

Do not modify unless a compile error shows a missing public accessor:

```text
src/core/*
```

If a Rust scenario test fails because behavior is different from expected, stop and classify it in the report before changing core behavior.

---

## Task 1: Baseline Verification

**Files:**

- Read: `src/core/config.rs`
- Read: `src/core/tick.rs`
- Read: `src/core/summary.rs`
- Read: `src/core/cell_store.rs`
- Read: `tests/phase1_accounting.rs`
- Read: `tests/phase1_determinism.rs`

- [ ] **Step 1: Run current Rust tests**

Run:

```bash
cargo test
```

Expected:

```text
all existing Rust tests pass
```

- [ ] **Step 2: Run formatting check**

Run:

```bash
cargo fmt --check
```

Expected:

```text
formatting check passes
```

- [ ] **Step 3: Record baseline**

Record in notes for the final report:

```text
Rust baseline:
  cargo test: pass/fail
  cargo fmt --check: pass/fail
  existing test count from cargo output
```

Do not edit files in this task.

---

## Task 2: Add Rust Scenario Fixtures

**Files:**

- Create: `tests/phase1_config_validation.rs`

- [ ] **Step 1: Write failing validation tests**

Create `tests/phase1_config_validation.rs` with this complete content:

```rust
use alife::core::cell_store::{CellIndex, LifecycleState};
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::summary::{CollapseReason, RunSummary, SurvivalResult};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

#[derive(Clone, Copy, Debug)]
struct ScenarioExpectation {
    scenario_id: &'static str,
    expected_result: SurvivalResult,
    expected_reason: CollapseReason,
    expected_tick: u64,
    expected_lifecycle: LifecycleState,
}

#[derive(Clone, Copy, Debug)]
struct ScenarioRun {
    summary: RunSummary,
    lifecycle: LifecycleState,
}

fn runtime_config(
    scenario_id: &str,
    tick_count: u64,
    passive_income: f32,
    initial_energy: f32,
    energy_capacity: f32,
    mandatory_cost: f32,
    dormant_modifier: f32,
    capacity_limit: f32,
    initial_resources_total: f32,
    initial_materials_total: f32,
    heat_current: f32,
    heat_generated: f32,
    heat_dissipation: f32,
    heat_warning: f32,
    heat_death: f32,
    waste_current: f32,
    waste_generated: f32,
    waste_sink: f32,
    waste_warning: f32,
    waste_death: f32,
    stress_energy_threshold: f32,
    dormancy_allowed: bool,
    critical_capacity_overrun: f32,
) -> RuntimeConfig {
    let _scenario_id_is_documentation_only = scenario_id;

    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(tick_count),
            seed: Seed::from_raw(42),
            size: WorldSize::new(512.0, 512.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
        },
        CellInitialConfig {
            position: Position::new(256.0, 256.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(initial_energy).unwrap(),
            energy_capacity: EnergyAmount::new(energy_capacity).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(mandatory_cost).unwrap(),
            passive_energy_income: EnergyAmount::new(passive_income).unwrap(),
            capacity_limit: CapacityAmount::new(capacity_limit).unwrap(),
            initial_resource_amount: ResourceAmount::new(initial_resources_total).unwrap(),
            initial_material_amount: MaterialAmount::new(initial_materials_total).unwrap(),
        },
        EnvironmentConfig {
            heat_current: HeatAmount::new(heat_current).unwrap(),
            heat_generated_per_tick: HeatAmount::new(heat_generated).unwrap(),
            heat_dissipation_rate: HeatAmount::new(heat_dissipation).unwrap(),
            heat_warning_threshold: HeatAmount::new(heat_warning).unwrap(),
            heat_death_threshold: HeatAmount::new(heat_death).unwrap(),
            waste_current: WasteAmount::new(waste_current).unwrap(),
            waste_generated_per_tick: WasteAmount::new(waste_generated).unwrap(),
            waste_sink_rate: WasteAmount::new(waste_sink).unwrap(),
            waste_warning_threshold: WasteAmount::new(waste_warning).unwrap(),
            waste_death_threshold: WasteAmount::new(waste_death).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(stress_energy_threshold).unwrap(),
            dormancy_allowed,
            dormant_mandatory_cost_modifier: dormant_modifier,
            critical_capacity_overrun: CapacityAmount::new(critical_capacity_overrun).unwrap(),
        },
    )
    .unwrap()
}

fn single_cell_survival_config() -> RuntimeConfig {
    runtime_config(
        "single_cell_survival",
        100,
        2.0,
        50.0,
        100.0,
        2.0,
        0.1,
        30.0,
        3.0,
        5.0,
        0.0,
        0.1,
        0.2,
        50.0,
        80.0,
        0.0,
        0.05,
        0.1,
        10.0,
        20.0,
        10.0,
        true,
        5.0,
    )
}

fn single_cell_starvation_config() -> RuntimeConfig {
    runtime_config(
        "single_cell_starvation",
        100,
        0.0,
        1.0,
        100.0,
        5.0,
        0.5,
        50.0,
        3.0,
        5.0,
        0.0,
        0.1,
        0.2,
        50.0,
        80.0,
        0.0,
        0.05,
        0.1,
        10.0,
        20.0,
        10.0,
        false,
        5.0,
    )
}

fn single_cell_dormancy_config() -> RuntimeConfig {
    runtime_config(
        "single_cell_dormancy",
        10,
        0.0,
        1.0,
        100.0,
        5.0,
        0.1,
        30.0,
        3.0,
        5.0,
        0.0,
        0.1,
        0.2,
        50.0,
        80.0,
        0.0,
        0.05,
        0.1,
        10.0,
        20.0,
        10.0,
        true,
        5.0,
    )
}

fn single_cell_heat_death_config() -> RuntimeConfig {
    runtime_config(
        "single_cell_heat_death",
        20,
        2.0,
        50.0,
        100.0,
        2.0,
        0.1,
        30.0,
        3.0,
        5.0,
        0.0,
        5.0,
        0.0,
        10.0,
        12.0,
        0.0,
        0.05,
        0.1,
        10.0,
        20.0,
        10.0,
        true,
        5.0,
    )
}

fn single_cell_waste_death_config() -> RuntimeConfig {
    runtime_config(
        "single_cell_waste_death",
        20,
        2.0,
        50.0,
        100.0,
        2.0,
        0.1,
        30.0,
        3.0,
        5.0,
        0.0,
        0.1,
        0.2,
        50.0,
        80.0,
        0.0,
        5.0,
        0.0,
        10.0,
        12.0,
        10.0,
        true,
        5.0,
    )
}

fn single_cell_over_capacity_config() -> RuntimeConfig {
    runtime_config(
        "single_cell_over_capacity",
        100,
        5.0,
        50.0,
        100.0,
        2.0,
        0.1,
        15.0,
        20.0,
        5.0,
        0.0,
        0.1,
        0.2,
        50.0,
        80.0,
        0.0,
        0.05,
        0.1,
        10.0,
        20.0,
        10.0,
        true,
        5.0,
    )
}

fn run(config: RuntimeConfig) -> ScenarioRun {
    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();
    let lifecycle = executor
        .world()
        .cells()
        .lifecycle_state(CellIndex::from_raw(0));
    ScenarioRun { summary, lifecycle }
}

fn assert_scenario(config: RuntimeConfig, expected: ScenarioExpectation) {
    let run = run(config);

    assert_eq!(
        run.summary.survival_result, expected.expected_result,
        "{} survival_result",
        expected.scenario_id
    );
    assert_eq!(
        run.summary.collapse_reason, expected.expected_reason,
        "{} collapse_reason",
        expected.scenario_id
    );
    assert_eq!(
        run.summary.tick.raw(),
        expected.expected_tick,
        "{} tick",
        expected.scenario_id
    );
    assert_eq!(
        run.lifecycle, expected.expected_lifecycle,
        "{} lifecycle",
        expected.scenario_id
    );
}

#[test]
fn current_survival_config_is_stable_in_rust() {
    assert_scenario(
        single_cell_survival_config(),
        ScenarioExpectation {
            scenario_id: "single_cell_survival",
            expected_result: SurvivalResult::Stable,
            expected_reason: CollapseReason::None,
            expected_tick: 100,
            expected_lifecycle: LifecycleState::Alive,
        },
    );
}

#[test]
fn current_starvation_config_collapses_in_rust() {
    assert_scenario(
        single_cell_starvation_config(),
        ScenarioExpectation {
            scenario_id: "single_cell_starvation",
            expected_result: SurvivalResult::Collapse,
            expected_reason: CollapseReason::MandatoryCostUnpaid,
            expected_tick: 1,
            expected_lifecycle: LifecycleState::Dead,
        },
    );
}

#[test]
fn current_dormancy_config_reaches_dormancy_then_depletes_energy_in_rust() {
    let mut executor = TickExecutor::new(single_cell_dormancy_config()).unwrap();

    let first = executor.step().unwrap();
    assert_eq!(first.survival_result, SurvivalResult::Fragile);
    assert_eq!(
        executor
            .world()
            .cells()
            .lifecycle_state(CellIndex::from_raw(0)),
        LifecycleState::Dormant
    );

    let second = executor.step().unwrap();
    assert_eq!(second.survival_result, SurvivalResult::Collapse);
    assert_eq!(second.collapse_reason, CollapseReason::EnergyDepleted);
}

#[test]
fn current_heat_death_config_collapses_in_rust() {
    assert_scenario(
        single_cell_heat_death_config(),
        ScenarioExpectation {
            scenario_id: "single_cell_heat_death",
            expected_result: SurvivalResult::Collapse,
            expected_reason: CollapseReason::HeatLimitExceeded,
            expected_tick: 3,
            expected_lifecycle: LifecycleState::Dead,
        },
    );
}

#[test]
fn current_waste_death_config_collapses_in_rust() {
    assert_scenario(
        single_cell_waste_death_config(),
        ScenarioExpectation {
            scenario_id: "single_cell_waste_death",
            expected_result: SurvivalResult::Collapse,
            expected_reason: CollapseReason::WasteLimitExceeded,
            expected_tick: 3,
            expected_lifecycle: LifecycleState::Dead,
        },
    );
}

#[test]
fn current_over_capacity_config_collapses_in_rust() {
    assert_scenario(
        single_cell_over_capacity_config(),
        ScenarioExpectation {
            scenario_id: "single_cell_over_capacity",
            expected_result: SurvivalResult::Collapse,
            expected_reason: CollapseReason::CapacityExceeded,
            expected_tick: 1,
            expected_lifecycle: LifecycleState::Dead,
        },
    );
}
```

- [ ] **Step 2: Run the new validation tests**

Run:

```bash
cargo test --test phase1_config_validation
```

Expected:

```text
6 tests pass
```

If tests fail, do not change Rust behavior immediately. Capture exact failing scenario, actual value and expected value for the report.

---

## Task 3: Add Optional Rust Result Dump Test

**Files:**

- Modify: `tests/phase1_config_validation.rs`

- [ ] **Step 1: Add an ignored result dump test**

Append to `tests/phase1_config_validation.rs`:

```rust
#[test]
#[ignore = "manual result dump for worklog report"]
fn dump_current_phase1_rust_config_results() {
    let scenarios: [(&str, RuntimeConfig); 6] = [
        ("single_cell_survival", single_cell_survival_config()),
        ("single_cell_starvation", single_cell_starvation_config()),
        ("single_cell_dormancy", single_cell_dormancy_config()),
        ("single_cell_heat_death", single_cell_heat_death_config()),
        ("single_cell_waste_death", single_cell_waste_death_config()),
        ("single_cell_over_capacity", single_cell_over_capacity_config()),
    ];

    println!("scenario_id,survival_result,collapse_reason,tick,final_energy,heat,waste,lifecycle");
    for (scenario_id, config) in scenarios {
        let run = run(config);
        println!(
            "{},{:?},{:?},{},{:.3},{:.3},{:.3},{:?}",
            scenario_id,
            run.summary.survival_result,
            run.summary.collapse_reason,
            run.summary.tick.raw(),
            run.summary.metrics.final_energy,
            run.summary.metrics.heat,
            run.summary.metrics.waste,
            run.lifecycle
        );
    }
}
```

- [ ] **Step 2: Run the ignored dump test manually**

Run:

```bash
cargo test --test phase1_config_validation dump_current_phase1_rust_config_results -- --ignored --nocapture
```

Expected output contains a CSV-like table:

```text
scenario_id,survival_result,collapse_reason,tick,final_energy,heat,waste,lifecycle
single_cell_survival,...
single_cell_starvation,...
single_cell_dormancy,...
single_cell_heat_death,...
single_cell_waste_death,...
single_cell_over_capacity,...
```

Copy this table into the final report.

---

## Task 4: Optional Python Sanity Run For Mismatch Diagnosis

**Files:**

- No source file changes.
- Generated outputs go under `outputs/stability/phase1-rust-config-validation-python-sanity/`.

- [ ] **Step 1: Decide whether Python sanity run is needed**

Run Python only if at least one of these is true:

```text
Rust scenario result is surprising.
Rust result differs from the expected Phase 1 docs.
Rust result suggests current tuned configs are not viable.
The report needs explicit Python/Rust comparison evidence.
```

If none are true, write in the final report:

```text
Python early-stability was not used as oracle. No Python sanity run was required.
```

- [ ] **Step 2: Run Python batch sanity check if needed**

Run:

```bash
python .\tools\early-stability\src\cli.py batch --scenarios .\tools\early-stability\scenarios --out .\outputs\stability\phase1-rust-config-validation-python-sanity --with-simulation
```

Expected:

```text
outputs/stability/phase1-rust-config-validation-python-sanity contains batch result artifacts
```

Use Python output only for diagnosis. Do not edit Rust to match Python automatically.

---

## Task 5: Full Verification

**Files:**

- No source changes expected unless Task 2 or 3 required test file edits.

- [ ] **Step 1: Run formatting**

Run:

```bash
cargo fmt --check
```

Expected:

```text
formatting check passes
```

If formatting fails, run:

```bash
cargo fmt
```

Then rerun:

```bash
cargo fmt --check
```

- [ ] **Step 2: Run all Rust tests**

Run:

```bash
cargo test
```

Expected:

```text
all Rust tests pass
```

- [ ] **Step 3: Run current config validation tests**

Run:

```bash
cargo test --test phase1_config_validation
```

Expected:

```text
all phase1_config_validation tests pass
```

- [ ] **Step 4: Run current Python tool tests only if Python sanity was used**

Run:

```bash
python -m pytest .\tools\early-stability
```

Expected:

```text
all early-stability tests pass
```

If Python sanity was not used and no Python files changed, this command is optional.

---

## Task 6: Write Worklog Report

**Files:**

- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1-rust-config-validation.md`
- Modify: `outputs/worklogs/README.md`

- [ ] **Step 1: Create report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1-rust-config-validation.md`:

```markdown
# REPORT: Phase 1 Rust Config Validation

## Goal

Validated current Phase 1 tuned/stability scenarios against Rust Phase 1 core.

## Authority Rule

Rust Phase 1 core is authoritative for implemented Phase 1 behavior.
Python early-stability is an estimator/tuner and was not used as oracle.

## Scenarios Checked

| Scenario | Expected Rust result | Actual Rust result | Expected reason | Actual reason | Expected tick | Actual tick | Expected lifecycle | Actual lifecycle | Notes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| single_cell_survival | Stable |  | None |  | 100 |  | Alive |  | Matches stable baseline when actual equals expected. |
| single_cell_starvation | Collapse |  | MandatoryCostUnpaid |  | 1 |  | Dead |  | Rust treats unpaid mandatory cost as terminal when dormancy is disabled. |
| single_cell_dormancy | Collapse after first reaching Dormant |  | EnergyDepleted |  | 2 |  | Dead |  | First tick should be Fragile/Dormant, second tick collapses by energy. |
| single_cell_heat_death | Collapse |  | HeatLimitExceeded |  | 3 |  | Dead |  | Heat crosses death threshold on Tick 3. |
| single_cell_waste_death | Collapse |  | WasteLimitExceeded |  | 3 |  | Dead |  | Waste crosses death threshold on Tick 3. |
| single_cell_over_capacity | Collapse |  | CapacityExceeded |  | 1 |  | Dead |  | Used capacity exceeds capacity plus critical overrun. |

## Rust Result Dump

```text
Paste output from:
cargo test --test phase1_config_validation dump_current_phase1_rust_config_results -- --ignored --nocapture
```

## Python Sanity Check

State one of:

```text
Python sanity run was not required.
```

or:

```text
Python sanity run was executed for mismatch diagnosis:
outputs/stability/phase1-rust-config-validation-python-sanity
```

## Findings

- List configs that pass Rust Phase 1 behavior.
- List configs that collapse in Rust and whether this is expected.
- List any mismatch with accepted docs.
- List any mismatch that appears to be Python estimator limitation.

## Verification

```text
cargo fmt --check
cargo test
cargo test --test phase1_config_validation
python -m pytest .\tools\early-stability   # only if Python sanity was used
```

## Follow-Up

- State whether current configs are sufficient to proceed with Phase 1B/1C development.
- State whether any config should be retuned later.
- State whether any Rust behavior needs a separate bugfix plan.
```

Fill every `Actual` cell before finishing the report.

- [ ] **Step 2: Add report to worklog index**

Add one line under `## Reports` in `outputs/worklogs/README.md`:

```markdown
- [[outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1-rust-config-validation|outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1-rust-config-validation]]
```

Use the actual report timestamp.

---

## Task 7: Final Status

**Files:**

- Read: `git status --short`

- [ ] **Step 1: Show changed files**

Run:

```bash
git status --short
```

Expected changed files:

```text
tests/phase1_config_validation.rs
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1-rust-config-validation.md
outputs/worklogs/README.md
```

Optional generated Python sanity artifacts may appear under:

```text
outputs/stability/phase1-rust-config-validation-python-sanity/
```

Do not commit unless the user explicitly asks.

- [ ] **Step 2: Final answer for user**

Report:

```text
Rust scenarios passed/failed:
Python sanity used/not used:
Report path:
Main finding:
Next recommended action:
```

---

## Acceptance Gates

This task is complete only when:

```text
Rust scenarios are represented as Rust fixtures matching current TOML values.
cargo test --test phase1_config_validation passes or failures are documented as findings.
Rust result dump table is captured in the report.
Python is not treated as oracle.
Python sanity comparison is used only when needed.
Report exists in outputs/worklogs.
outputs/worklogs/README.md links the report.
No Rust behavior is changed without a separate bugfix plan.
```

---

## Self-Review

Spec coverage:

- Current configs are run through Rust via scenario fixtures.
- Python is optional and diagnostic only.
- Report is mandatory.
- Mismatches are classified instead of silently fixed.

Known limits:

- This plan does not implement TOML parsing in Rust.
- Scenario fixtures manually mirror current TOML values.
- Estimate/post-Phase1 scenarios are excluded from Rust Phase 1 validation.
