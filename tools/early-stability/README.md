# Early Stability Tool

The Early Stability Tool is a deterministic micro-simulation, configuration validator, static calculator, and parameter tuning utility for ALife Phase 1 stability checks.

It evaluates cell survival, mandatory energy budgets, heat and waste accumulation, capacity constraints, and bounded tuning ranges before full `alife-core` implementation exists.

---

## Purpose

This tool is an offline implementation/research helper. It is not Canon authority and must not mutate source configs automatically.

It should answer:

- whether mandatory costs kill the Cell;
- whether Energy and Resources can support growth budgets;
- whether Heat or waste accumulates without a sink;
- whether simple population and Joint upkeep estimates are plausible;
- which tested parameter ranges are stable, fragile, collapsing, or invalid.

---

## Directory Structure

```text
tools/early-stability/
  src/
    cli.py                  # Entrypoint, subcommands, and batch runner
    config_loader.py        # Validates config format, types, and capacity limits
    micro_simulator.py      # Runs tick loops tracking energy, heat, and waste updates
    report_writer.py        # Generates REPORT.md with sensitivity rankings
    result_writer.py        # Standardizes JSON outputs
    reachability.py         # Evaluates observer-only mechanism reachability
    reachability_writer.py  # Writes reachability JSON and Markdown artifacts
    static_calculator.py    # Performs static bounds evaluations
    tuner.py                # Deterministic grid-search tuning engine
  mechanisms/               # Mechanism registries for reachability checks
  scenarios/                # Standard test scenario TOMLs
  tuning/                   # Example tuning setup TOMLs
  tests/                    # Pytest suite
  pyproject.toml            # Package metadata and test config
  README.md                 # This guide
```

---

## Install

From the repository root:

```powershell
python -m pip install -e .\tools\early-stability[dev]
```

After installation, use either:

```powershell
early-stability --help
```

or direct script execution:

```powershell
python .\tools\early-stability\src\cli.py --help
```

---

## CLI Commands

### evaluate

Runs static budget and capacity checks without tick simulation.

```powershell
early-stability evaluate --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --out .\outputs\stability\manual_evaluate
```

Use `--with-simulation` to run micro simulation after static checks and keep the worse result:

```powershell
early-stability evaluate --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --out .\outputs\stability\manual_evaluate_sim --with-simulation
```

### simulate

Runs the bounded micro simulator and writes per-tick history.

```powershell
early-stability simulate --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --ticks 20 --out .\outputs\stability\manual_simulate
```

### tune

Runs deterministic grid-search tuning.

```powershell
early-stability tune --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --tuning .\tools\early-stability\tuning\single_cell.toml --out .\outputs\stability\manual_tune
```

### batch

Evaluates all `.toml` scenarios in a directory in deterministic filename order.

```powershell
early-stability batch --scenarios .\tools\early-stability\scenarios --out .\outputs\stability\manual_batch
```

Use `--with-simulation` to run the micro simulator for scenarios that pass static checks:

```powershell
early-stability batch --scenarios .\tools\early-stability\scenarios --out .\outputs\stability\manual_batch_sim --with-simulation
```

### reachability

Runs observer-only mechanism reachability analysis after stability tuning.

```powershell
early-stability reachability --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --mechanisms .\tools\early-stability\mechanisms\phase1.toml --stability-ranges-ref .\outputs\stability\group3_capacity_revalidated --out .\outputs\reachability\phase1_smoke
```

Use this after `tune`, not before it. If reachability reports bypassed or blocked mechanisms, return to parameter tuning and adjust the relevant tuning group.

---

## Scenario TOML

Scenarios must include these root sections:

```toml
scenario_id = "single_cell_survival"
seed = 42
tick_count = 100

[world]
size = [512.0, 512.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["water", "nutrient"]
initial_distribution = [10.0, 5.0]
passive_energy_income_placeholder = 5.0

[cell]
initial_position = [256.0, 256.0]
radius = 1.0
initial_resources = { water = 2.0, nutrient = 1.0 }
initial_materials = { cell_wall = 5.0 }
initial_energy = 50.0
energy_capacity = 100.0
mandatory_cost_per_tick = 2.0
dormant_mandatory_cost_modifier = 0.1
capacity_limit = 50.0
minimum_viability_materials = ["cell_wall"]

[environment]
ambient_temperature = 25.0
heat_current = 0.0
heat_generated_per_tick = 0.1
heat_dissipation_rate = 0.2
heat_warning_threshold = 40.0
heat_death_threshold = 80.0
waste_current = 0.0
waste_generated_per_tick = 0.05
waste_sink_rate = 0.1
waste_warning_threshold = 10.0
waste_death_threshold = 20.0

[lifecycle]
stress_energy_threshold = 10.0
dormancy_allowed = true
critical_capacity_overrun = 5.0
```

Optional `[estimates]` fields support static checks for growth, division, population bounds, and Joint upkeep.

---

## Tuning TOML

Tuning configs must explicitly list `allowed_parameters`. Every allowed parameter must have a matching range, and every range must be allowed.

```toml
[tuning]
max_iterations = 100
seeds = [42, 100, 2026]
objective = "map_stable_ranges"
allowed_parameters = ["cell.initial_energy", "environment.heat_dissipation_rate"]

[tuning.ranges]
"cell.initial_energy" = [5.0, 50.0, 5.0]
"environment.heat_dissipation_rate" = [0.05, 0.5, 0.05]
```

Supported objectives:

- `map_stable_ranges`
- `find_first_stable`
- `find_conservative_stable`

---

## Output Artifacts

The tool writes artifacts to the `--out` directory:

- `results.json`: summary result, config hash, seed, tick count, and metrics;
- `REPORT.md`: readable report with recommendations, tested/stable ranges, sensitivity, warnings, and evidence limits;
- `ranges.json`: tested and stable ranges for tuned parameters;
- `recommended-configs/*.toml`: generated candidate configs for stable profiles;
- `runs/run_XXXX.json`: detailed run records with parameters, seed, result, final metrics, and tick history.

Reachability runs write:

- `mechanisms.json`: per-mechanism counters and classification;
- `block-reasons.json`: aggregate block reason counts;
- `bypass.json`: mechanisms with detected bypass risk;
- `REPORT.md`: decision line and feedback loop back to parameter tuning.

Generated stability outputs should go under `outputs/stability/` and should not be committed.
Generated reachability outputs should go under `outputs/reachability/` and should not be committed unless explicitly requested.
