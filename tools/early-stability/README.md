# Early Stability Tool

The Early Stability Tool is a deterministic micro-simulation, configuration validator, and parameter tuning utility designed to evaluate cell survival, metabolic upkeep budgets, thermal dissipation, waste accumulation, and carrying capacity constraints for the ALife Simulation project.

---

## Purpose of the Tool

This tool serves as an execution pipeline to analyze cell configurations before launching full-scale distributed 2D simulations. By running lightweight, headless tick loops and performing grid-search optimization, it guarantees configuration stability under deterministic conditions.

---

## Directory Structure

```text
tools/early-stability/
├── src/
│   ├── cli.py                  # Entrypoint, subcommands, and batch runner
│   ├── config_loader.py        # Validates config format, types, and capacity limits
│   ├── micro_simulator.py      # Run tick loops tracking energy, heat, and waste updates
│   ├── report_writer.py        # Generates REPORT.md with sensitivity rankings
│   ├── result_writer.py        # Standardizes JSON outputs (results, ranges, runs)
│   ├── static_calculator.py    # Performs scenario-aware static bounds evaluations
│   └── tuner.py                # Grid search Cartesian product tuning engine
├── scenarios/                  # Collection of standard test scenario TOMLs
├── tuning/                     # Example tuning setup configuration files
├── tests/                      # Pytest test suite files
├── pyproject.toml              # Project dependencies and manifest metadata
└── README.md                   # This documentation guide
```

---

## CLI Commands Usage

The tool exposes four subcommands via `cli.py`:

### 1. `evaluate`
Performs static budget and capacity checks without running a simulation.

```bash
python src/cli.py evaluate --scenario scenarios/single_cell_survival.toml --out out_dir
```

### 2. `simulate`
Runs a headless micro-simulation tick loop up to the configured `tick_count` to track detailed state history.

```bash
python src/cli.py simulate --scenario scenarios/single_cell_survival.toml --out out_dir
```

### 3. `tune`
Executes deterministic grid search parameter tuning.

```bash
python src/cli.py tune --scenario scenarios/single_cell_survival.toml --tuning tuning/single_cell.toml --out out_dir
```

### 4. `batch`
Recursively evaluates all TOML scenario files located in a target directory alphabetically.

```bash
python src/cli.py batch --scenarios scenarios/ --out out_dir
```

---

## File Format Specifications

### Scenario TOML Format
Scenarios must include all mandatory root sections:

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

[estimates]
growth_cost_estimate = 10.0
division_cost_estimate = 20.0
resource_regeneration_or_inflow = 5.0
population_space_limit = 100
joint_count_estimate = 0
joint_upkeep_cost = 0.0
```

### Tuning TOML Format
Configures parameters to tune via grid search:

```toml
[tuning]
max_iterations = 100
seeds = [42, 100, 2026]
objective = "find_first_stable"  # Options: map_stable_ranges, find_first_stable, find_conservative_stable
allowed_parameters = ["cell.initial_energy", "environment.heat_dissipation_rate"]

[tuning.ranges]
"cell.initial_energy" = [5.0, 50.0, 5.0]             # Format: [start, end, step]
"environment.heat_dissipation_rate" = [0.05, 0.5, 0.05]
```

---

## Generated Output Artifacts

The tool writes output artifacts to the specified `--out` directory:

1. **`results.json`**: Core summary recording execution outcomes, seed evaluation statistics, config hash, and run results.
2. **`REPORT.md`**: Markdown report detailing:
   - Best stable candidate details (final energy, final heat, final waste).
   - Recommended values table.
   - Empirical tested & stable ranges table.
   - Sensitivity parameter rank (evaluated by stable-to-tested range narrowness ratio).
   - Collapse reasons summary.
   - User warnings and limits of evidence.
3. **`ranges.json`**: Numerical ranges mapped for tested parameters, including mid-point recommended values and evaluation confidence.
4. **`recommended-configs/`**: TOML configuration files generated for matching profiles:
   - `best_stable.toml`: Candidate that achieved the highest final energy/lowest heat & waste.
   - `conservative_stable.toml`: Candidate that maximized safety margins from warning thresholds.
   - `fragile_edge.toml`: Surviving candidate closest to collapse warnings.
5. **`runs/`**: Detailed state step history files (`run_0001.json`, `run_0002.json`, etc.) mapping variables at every tick of execution.
