# PLAN: Refactor Sweep Scenarios and Extend Balance Analysis

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor the sweep analyzer configuration to use isolated scenario presets, calculate resource/energy conservation metrics, detect uninformative sweeps/imbalances, and separate mechanical regression checks from strategy dominance sweeps.

**Architecture:** Update TOML parsing in `sweep_analyzer.rs` to support `[scenarios.<name>]` blocks. Map sweeps to presets, calculate detailed input/output conservation errors, write warnings to reports, and split strategy sweeps into a separate `balance_analyzer.rs` runner.

**Tech Stack:** Rust, toml, serde, csv, std::fs.

---

## 1. Audit of Current Analyzer and Config

Currently, `sweep_analyzer.toml` defines:
- A single global set of properties (`[cell]`, `[lifecycle]`, `[resource_interaction]`, `[environment]`).
- Every sweep override patches this single base configuration.
- Output metrics are written to `outputs/raw_data/` with a trailing `# SUMMARY` commented block appended at the end of the CSV, which violates raw CSV schema standards.
- There are no checks for conservation errors, uninformative sweeps, or environment-dominated scenarios.

## 2. Proposed Scenario Preset Schema

We will add a `[scenarios]` table in `sweep_analyzer.toml` deserialized as:

```toml
[scenarios.finite_resource_viability]
world_size = [512.0, 512.0]
initial_resources = [10.0]
decay_rate = 0.01
cell_position = [256.0, 256.0]
cell_radius = 1.0
initial_energy = 50.0
energy_capacity = 100.0
mandatory_cost_per_tick = 2.0
passive_energy_income = 0.0
capacity_limit = 30.0
stress_energy_threshold = 10.0
dormancy_allowed = true
dormant_mandatory_cost_modifier = 0.1
critical_capacity_overrun = 5.0
heat_dissipation_rate = 0.2
heat_warning_threshold = 50.0
heat_death_threshold = 80.0
waste_sink_rate = 0.1
waste_warning_threshold = 10.0
waste_death_threshold = 20.0
growth_enabled = false
```

## 3. Mapping of Current Sweeps to Scenarios

Every sweep/matrix definition will declare an optional `scenario` string referencing a preset:

1. **`viability_threshold`** -> scenario: `finite_resource_viability` (Initial resource stock, no regeneration/passive income).
2. **`passive_income_equilibrium`** -> scenario: `steady_resource_flow` (No initial resources, passive income acts as primary energy supply).
3. **`upkeep_sensitivity`** -> scenario: `steady_resource_flow` (Low initial energy, steady income).
4. **`dormant_modifier`** -> scenario: `dormancy_survival` (Initial energy below stress threshold, zero resources, forces dormancy).
5. **`transport_metabolism`** -> scenario: `resource_abundance` (High initial resource density to test conversion throughput).

## 4. Proposed CSV and Report Schemas

### Raw CSV Columns (First Row Header)
`scenario_id,scenario_version,config_hash,seed,ticks_requested,ticks_executed,parameter_name,parameter_value,secondary_parameter_name,secondary_parameter_value,zone,scenario_status,warning_codes,survived_to_end,survival_ticks,death_tick,death_reason,active_ticks,active_fraction,dormant_ticks,dormant_fraction,dormancy_enter_count,dormancy_exit_count,initial_energy,final_energy,min_energy,max_energy,mean_energy,energy_produced,passive_energy_received,energy_spent_upkeep,energy_spent_dormant_upkeep,energy_spent_movement,energy_spent_growth,energy_spent_repair,energy_spent_division,initial_world_resource,final_world_resource,resource_regenerated,resource_absorbed,resource_metabolized,internal_resource_final,resource_released,resource_explicit_sink,resource_balance_error,energy_balance_error`

### Summary Files
- `sweep_scenario_summary.csv`: Aggregated min, max, mean, and ideal range parameters per sweep.
- `sweep_activation_report.csv`: Tracks whether configured mechanisms (e.g. dormancy, repair) were active.

---

## 5. TDD Implementation Plan

### Task 1: Scenario Preset Config Schema & Parser

**Files:**
- Modify: `src/bin/sweep_analyzer.rs` (update structs and parser logic)
- Create: `tests/phase2_sweep_parser.rs`

- [ ] **Step 1: Write the failing test**

Create `tests/phase2_sweep_parser.rs` with:

```rust
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct RawScenarioPreset {
    pub world_size: Vec<f32>,
    pub initial_resources: Vec<f32>,
    pub decay_rate: f32,
    pub cell_position: Vec<f32>,
    pub cell_radius: f32,
    pub initial_energy: f32,
    pub energy_capacity: f32,
    pub mandatory_cost_per_tick: f32,
    pub passive_energy_income: f32,
    pub capacity_limit: f32,
    pub stress_energy_threshold: f32,
    pub dormancy_allowed: bool,
    pub dormant_mandatory_cost_modifier: f32,
    pub critical_capacity_overrun: f32,
    pub heat_dissipation_rate: f32,
    pub heat_warning_threshold: f32,
    pub heat_death_threshold: f32,
    pub waste_sink_rate: f32,
    pub waste_warning_threshold: f32,
    pub waste_death_threshold: f32,
    pub growth_enabled: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct TestConfig {
    pub scenarios: HashMap<String, RawScenarioPreset>,
}

#[test]
fn test_parse_scenario_presets() {
    let toml_str = r#"
[scenarios.test_preset]
world_size = [128.0, 128.0]
initial_resources = [5.0]
decay_rate = 0.05
cell_position = [64.0, 64.0]
cell_radius = 2.0
initial_energy = 40.0
energy_capacity = 80.0
mandatory_cost_per_tick = 1.5
passive_energy_income = 0.1
capacity_limit = 25.0
stress_energy_threshold = 8.0
dormancy_allowed = true
dormant_mandatory_cost_modifier = 0.15
critical_capacity_overrun = 4.0
heat_dissipation_rate = 0.25
heat_warning_threshold = 45.0
heat_death_threshold = 75.0
waste_sink_rate = 0.15
waste_warning_threshold = 12.0
waste_death_threshold = 22.0
growth_enabled = true
"#;
    let config: TestConfig = toml::from_str(toml_str).unwrap();
    let preset = config.scenarios.get("test_preset").unwrap();
    assert_eq!(preset.decay_rate, 0.05);
    assert_eq!(preset.growth_enabled, true);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test phase2_sweep_parser`
Expected: Compile failure (deserialization type missing/wrong).

- [ ] **Step 3: Update `src/bin/sweep_analyzer.rs` with Preset structs**

Append `RawScenarioPreset` to `src/bin/sweep_analyzer.rs`:

```rust
#[derive(Debug, serde::Deserialize, Clone)]
struct RawScenarioPreset {
    world_size: Vec<f32>,
    initial_resources: Vec<f32>,
    decay_rate: f32,
    cell_position: Vec<f32>,
    cell_radius: f32,
    initial_energy: f32,
    energy_capacity: f32,
    mandatory_cost_per_tick: f32,
    passive_energy_income: f32,
    capacity_limit: f32,
    stress_energy_threshold: f32,
    dormancy_allowed: bool,
    dormant_mandatory_cost_modifier: f32,
    critical_capacity_overrun: f32,
    heat_dissipation_rate: f32,
    heat_warning_threshold: f32,
    heat_death_threshold: f32,
    waste_sink_rate: f32,
    waste_warning_threshold: f32,
    waste_death_threshold: f32,
    growth_enabled: bool,
}

// Add scenario table to AnalyzerConfig
// struct AnalyzerConfig {
//     run: RunConfig,
//     scenarios: Option<HashMap<String, RawScenarioPreset>>,
//     ...
// }
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test phase2_sweep_parser`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add tests/phase2_sweep_parser.rs src/bin/sweep_analyzer.rs
git commit -m "feat: add RawScenarioPreset deserialization to sweep analyzer"
```

---

### Task 2: Sweep Scenario Reference Mapping & Overrides

**Files:**
- Modify: `src/bin/sweep_analyzer.rs`
- Modify: `tests/phase2_sweep_parser.rs`

- [ ] **Step 1: Write the failing test**

Append this test to `tests/phase2_sweep_parser.rs`:

```rust
#[test]
fn test_sweep_scenario_reference_mapping() {
    let toml_str = r#"
[run]
output_dir = "outputs"
seed = 100
ticks = 100

[cell]
radius = 1.0
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 1.0
passive_energy_income = 0.0
capacity_limit = 10.0
initial_metabolic_material = 1.0
initial_transport_material = 1.0
initial_boundary_material = 1.0
initial_structural_material = 1.0

[lifecycle]
stress_energy_threshold = 5.0
dormancy_allowed = true
dormant_mandatory_cost_modifier = 0.1
critical_capacity_overrun = 2.0

[resource_interaction]
energy_per_resource = 1.0
heat_per_resource = 0.0
waste_per_resource = 0.0
decay_rate = 0.0
default_resource_density = 1.0
default_max_uptake_per_tick = 1.0
default_metabolism_resource_per_tick = 1.0

[environment]
heat_dissipation_rate = 0.1
heat_warning_threshold = 10.0
heat_death_threshold = 20.0
waste_sink_rate = 0.1
waste_warning_threshold = 10.0
waste_death_threshold = 20.0

[scenarios.test_preset]
world_size = [128.0, 128.0]
initial_resources = [5.0]
decay_rate = 0.05
cell_position = [64.0, 64.0]
cell_radius = 2.0
initial_energy = 40.0
energy_capacity = 80.0
mandatory_cost_per_tick = 1.5
passive_energy_income = 0.1
capacity_limit = 25.0
stress_energy_threshold = 8.0
dormancy_allowed = true
dormant_mandatory_cost_modifier = 0.15
critical_capacity_overrun = 4.0
heat_dissipation_rate = 0.25
heat_warning_threshold = 45.0
heat_death_threshold = 75.0
waste_sink_rate = 0.15
waste_warning_threshold = 12.0
waste_death_threshold = 22.0
growth_enabled = true

[[sweep]]
name = "test_sweep"
param = "resource_density"
from = 1.0
to = 10.0
steps = 5
scenario = "test_preset"
"#;
    // Expected: Parsing works and binds scenario "test_preset" to SweepDef
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test phase2_sweep_parser`
Expected: Compile error (`scenario` field missing on `SweepDef`).

- [ ] **Step 3: Modify SweepDef and MatrixDef**

Update structs in `src/bin/sweep_analyzer.rs`:

```rust
#[derive(Debug, serde::Deserialize)]
struct SweepDef {
    name: String,
    param: String,
    from: f32,
    to: f32,
    steps: usize,
    scenario: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct MatrixDef {
    name: String,
    param_x: String,
    from_x: f32,
    to_x: f32,
    steps_x: usize,
    param_y: String,
    from_y: f32,
    to_y: f32,
    steps_y: usize,
    scenario: Option<String>,
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test phase2_sweep_parser`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/bin/sweep_analyzer.rs tests/phase2_sweep_parser.rs
git commit -m "feat: add optional scenario override support to sweep and matrix definitions"
```

---

### Task 3: Conservation Accounting Metrics

**Files:**
- Modify: `src/bin/sweep_analyzer.rs` (update `run_simulation` and metric outputs)
- Create: `tests/phase2_sweep_conservation.rs`

- [ ] **Step 1: Write the failing test**

Create `tests/phase2_sweep_conservation.rs` with:

```rust
#[test]
fn test_conservation_error_calculation() {
    // Assert that we calculate delta_resource and delta_energy correctly:
    // resource_error = (initial + regenerated) - (final + metabolized + internal + released)
    // Assert balance error is below threshold or flags warning
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test phase2_sweep_conservation`
Expected: FAIL

- [ ] **Step 3: Implement accounting loop in simulation runner**

In `src/bin/sweep_analyzer.rs`, calculate resource and energy balances at each tick step and record the cumulative error. Add `resource_balance_error` and `energy_balance_error` to the CSV structure.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test phase2_sweep_conservation`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/bin/sweep_analyzer.rs tests/phase2_sweep_conservation.rs
git commit -m "feat: calculate and export cumulative resource/energy conservation balance errors"
```

---

### Task 4: Detection of Uninformative and Low-Information Sweeps

**Files:**
- Modify: `src/bin/sweep_analyzer.rs`
- Create: `tests/phase2_sweep_warnings.rs`

- [ ] **Step 1: Write the failing test**

Create `tests/phase2_sweep_warnings.rs` asserting warning flags for:
1. `SCENARIO_MECHANISM_NOT_ACTIVATED` (e.g. dormancy sweeps where cell remains 100% active).
2. `LOW_INFORMATION_SWEEP` (e.g. parameter sweep has no effect on any metrics or zone results).

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test phase2_sweep_warnings`
Expected: FAIL

- [ ] **Step 3: Implement warning detection**

Add logic to inspect the list of run results for a sweep and verify if dormant_fraction is constantly 0, or if all points result in the same zone. Flag warning codes accordingly.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test phase2_sweep_warnings`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/bin/sweep_analyzer.rs tests/phase2_sweep_warnings.rs
git commit -m "feat: implement SCENARIO_MECHANISM_NOT_ACTIVATED and LOW_INFORMATION_SWEEP warnings"
```

---

### Task 5: Separate Summary Files from Raw CSV

**Files:**
- Modify: `src/bin/sweep_analyzer.rs`

- [ ] **Step 1: Write the failing test**

Verify that raw CSV file has exactly the raw rows with no trailing `# SUMMARY` commented lines.

- [ ] **Step 2: Run test to verify it fails**

Run the smoke test to verify it fails if `#` header lines are found or check string content of mock CSV.

- [ ] **Step 3: Write separate file exports**

Remove `# SUMMARY` writer from raw CSV loop. Implement writing summary results to a dedicated `<output_dir>/sweep_scenario_summary.csv` and report MD instead.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --workspace`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/bin/sweep_analyzer.rs
git commit -m "refactor: export raw CSV without commented summary headers and save summaries to a separate file"
```
