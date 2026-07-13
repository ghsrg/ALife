# Phase 3A Genome Bootstrap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the Phase 3A Genome Bootstrap vertical slice: scenario Genome templates, deterministic per-cell variation, physical carrier capacity, registered priority outputs, ActionPlan ordering, and deterministic replay.

**Architecture:** Add small Core-owned Genome modules instead of Runner-only behavior. `RuntimeConfig` stores validated templates and per-initial-cell Genome assignments; `WorldState` owns concrete Genome state and per-cell Genome references; Tick builds an ActionPlan from Genome priorities, then existing Feasibility remains the execution authority. Phase 3A uses constant-output direct regulatory graphs only; local inputs, mutation, inheritance, runtime memory, and lineage are out of scope.

**Tech Stack:** Rust 2024, existing `alife::core` modules, `serde` + `toml` parser in `src/runner/config_parser.rs`, integration tests under `tests/`, no new dependencies.

---

## Source Context

Read these before implementation:

```text
docs/PRINCIPLES.md
docs/mechanics/genome-action-pipeline.md
docs/biology/genome.md
docs/genetics/genome-runtime.md
docs/genetics/regulatory-interface.md
docs/genetics/regulatory-network.md
docs/biology/action-process-registry.md
outputs/worklogs/2026-07-02-1935-PLAN-phase-3-global-roadmap.md
```

Phase 3A constraints:

```text
Genome outputs priorities, not actions.
Genome outputs only registered current process priorities.
Feasibility remains final authority.
Genome has a physical carrier and consumes capacity.
Same seed + same config must replay the same initial Genome outputs.
Different initial cell ordinals may vary, but the variation must be deterministic.
No local inputs, mutation, inheritance, recurrent nodes, epigenetics, or lineage in Phase 3A.
```

Important existing mismatch:

```text
docs/biology/action-process-registry.md uses canonical outputs:
  resource_uptake_priority
  resource_export_priority
  energy_conversion_priority
  material_synthesis_priority
  repair_priority
  signal_emit_priority
  movement_priority
  division_preparation_priority
  genome_copying_priority
  division_partition_priority
  dormancy_bias
  internal_rebalance_priority

src/core/process.rs currently uses Rust ProcessId values:
  LocalResourceUptake
  MetabolismEnergyConversion
  MaterialSynthesis
  GrowthResourceAllocation
  Division
  ContractileDisplacement
  RepairBoundary
  JointCreate
```

Phase 3A should support only mappings that are defensible now:

```text
resource_uptake_priority -> ProcessId::LocalResourceUptake
energy_conversion_priority -> ProcessId::MetabolismEnergyConversion
material_synthesis_priority -> ProcessId::MaterialSynthesis
repair_priority -> ProcessId::RepairBoundary
movement_priority -> ProcessId::ContractileDisplacement
division_preparation_priority -> ProcessId::GrowthResourceAllocation
```

Do not accept `growth_priority`, `joint_create_priority`, or raw unregistered names in Phase 3A. If Joint creation needs Genome control, first update the Canon registry in a separate doc task.

---

## File Structure

```text
src/core/
  action_plan.rs        [NEW]    ActionPlan, planned process ordering, stable tie-breaks
  genome.rs             [NEW]    GenomeId, GenomeTemplate, GenomeCarrierState, GenomeState, GenomeOutputId
  genome_bootstrap.rs   [NEW]    deterministic template variation and initial state construction
  config.rs             [MODIFY] RuntimeConfig genome fields and ConfigError variants
  cell_store.rs         [MODIFY] per-cell GenomeId reference and carrier capacity accounting
  world.rs              [MODIFY] World-owned genome storage and bootstrap from RuntimeConfig
  tick.rs               [MODIFY] use ActionPlan ordering for Phase A planned actions
  process.rs            [MODIFY] canonical Genome output binding metadata for current ProcessId values
  summary.rs            [MODIFY] minimal observer-only Genome trace counters
  mod.rs                [MODIFY] export new modules

src/runner/
  config_parser.rs      [MODIFY] parse [genome_templates.*] and cell genome_template ids

tests/
  phase3a_genome_config.rs       [NEW] parser and validation tests
  phase3a_genome_bootstrap.rs    [NEW] deterministic variation and carrier capacity tests
  phase3a_action_plan.rs         [NEW] output binding and ordering tests
  phase3a_tick_integration.rs    [NEW] Tick behavior and replay tests

config/scenarios/genome/
  phase3a_genome_bootstrap.toml  [NEW] minimal demo scenario after tests pass
```

---

## Task 1: Core Genome Domain Types

**Files:**
- Create: `src/core/genome.rs`
- Modify: `src/core/mod.rs`
- Test: `tests/phase3a_genome_bootstrap.rs`

- [ ] **Step 1: Write the failing domain type tests**

```rust
// tests/phase3a_genome_bootstrap.rs
use alife::core::genome::{
    GenomeCarrierState, GenomeOutputId, GenomeOutputValue, GenomeTemplate, GenomeTemplateId,
};

#[test]
fn genome_output_value_clamps_to_canonical_range() {
    assert_eq!(GenomeOutputValue::new(-1.5).raw(), -1.0);
    assert_eq!(GenomeOutputValue::new(1.5).raw(), 1.0);
    assert_eq!(GenomeOutputValue::new(0.25).raw(), 0.25);
}

#[test]
fn genome_output_id_accepts_registered_phase3a_outputs() {
    assert_eq!(
        GenomeOutputId::parse("resource_uptake_priority").unwrap(),
        GenomeOutputId::ResourceUptakePriority
    );
    assert_eq!(
        GenomeOutputId::parse("energy_conversion_priority").unwrap(),
        GenomeOutputId::EnergyConversionPriority
    );
    assert_eq!(
        GenomeOutputId::parse("material_synthesis_priority").unwrap(),
        GenomeOutputId::MaterialSynthesisPriority
    );
    assert_eq!(
        GenomeOutputId::parse("repair_priority").unwrap(),
        GenomeOutputId::RepairPriority
    );
    assert_eq!(
        GenomeOutputId::parse("movement_priority").unwrap(),
        GenomeOutputId::MovementPriority
    );
    assert_eq!(
        GenomeOutputId::parse("division_preparation_priority").unwrap(),
        GenomeOutputId::DivisionPreparationPriority
    );
}

#[test]
fn genome_output_id_rejects_unregistered_phase3a_outputs() {
    assert!(GenomeOutputId::parse("growth_priority").is_err());
    assert!(GenomeOutputId::parse("joint_create_priority").is_err());
    assert!(GenomeOutputId::parse("observer_fitness").is_err());
}

#[test]
fn genome_template_requires_non_negative_variation_and_carrier() {
    let carrier = GenomeCarrierState::new("genome_carrier_A".to_string(), 1.0, 1.0).unwrap();
    let template = GenomeTemplate::new(
        GenomeTemplateId::new("balanced").unwrap(),
        0.08,
        1,
        carrier,
        vec![(GenomeOutputId::ResourceUptakePriority, GenomeOutputValue::new(0.7))],
    )
    .unwrap();

    assert_eq!(template.id().as_str(), "balanced");
    assert_eq!(template.variation_amplitude(), 0.08);
    assert_eq!(template.runtime_interval_ticks(), 1);
}
```

- [ ] **Step 2: Run the new test and verify RED**

Run:

```bash
cargo test --test phase3a_genome_bootstrap genome_output
```

Expected: compile failure because `alife::core::genome` does not exist.

- [ ] **Step 3: Add `src/core/genome.rs` with minimal value types**

```rust
use crate::core::process::ProcessId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GenomeId(u32);

impl GenomeId {
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GenomeTemplateId(String);

impl GenomeTemplateId {
    pub fn new(value: impl Into<String>) -> Result<Self, GenomeError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(GenomeError::EmptyTemplateId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GenomeOutputId {
    ResourceUptakePriority,
    EnergyConversionPriority,
    MaterialSynthesisPriority,
    RepairPriority,
    MovementPriority,
    DivisionPreparationPriority,
}

impl GenomeOutputId {
    pub fn parse(value: &str) -> Result<Self, GenomeError> {
        match value {
            "resource_uptake_priority" => Ok(Self::ResourceUptakePriority),
            "energy_conversion_priority" => Ok(Self::EnergyConversionPriority),
            "material_synthesis_priority" => Ok(Self::MaterialSynthesisPriority),
            "repair_priority" => Ok(Self::RepairPriority),
            "movement_priority" => Ok(Self::MovementPriority),
            "division_preparation_priority" => Ok(Self::DivisionPreparationPriority),
            other => Err(GenomeError::UnknownOutputId(other.to_string())),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResourceUptakePriority => "resource_uptake_priority",
            Self::EnergyConversionPriority => "energy_conversion_priority",
            Self::MaterialSynthesisPriority => "material_synthesis_priority",
            Self::RepairPriority => "repair_priority",
            Self::MovementPriority => "movement_priority",
            Self::DivisionPreparationPriority => "division_preparation_priority",
        }
    }

    pub const fn process_id(self) -> ProcessId {
        match self {
            Self::ResourceUptakePriority => ProcessId::LocalResourceUptake,
            Self::EnergyConversionPriority => ProcessId::MetabolismEnergyConversion,
            Self::MaterialSynthesisPriority => ProcessId::MaterialSynthesis,
            Self::RepairPriority => ProcessId::RepairBoundary,
            Self::MovementPriority => ProcessId::ContractileDisplacement,
            Self::DivisionPreparationPriority => ProcessId::GrowthResourceAllocation,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GenomeOutputValue(f32);

impl GenomeOutputValue {
    pub fn new(value: f32) -> Self {
        if !value.is_finite() {
            return Self(0.0);
        }
        Self(value.clamp(-1.0, 1.0))
    }

    pub const fn raw(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenomeCarrierState {
    pub material_id: String,
    pub amount: f32,
    pub integrity: f32,
}

impl GenomeCarrierState {
    pub fn new(material_id: String, amount: f32, integrity: f32) -> Result<Self, GenomeError> {
        if material_id.trim().is_empty() {
            return Err(GenomeError::EmptyCarrierMaterialId);
        }
        if !amount.is_finite() || amount <= 0.0 {
            return Err(GenomeError::InvalidCarrierAmount);
        }
        if !integrity.is_finite() || !(0.0..=1.0).contains(&integrity) {
            return Err(GenomeError::InvalidCarrierIntegrity);
        }
        Ok(Self {
            material_id,
            amount,
            integrity,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenomeTemplate {
    id: GenomeTemplateId,
    variation_amplitude: f32,
    runtime_interval_ticks: u64,
    carrier: GenomeCarrierState,
    outputs: Vec<(GenomeOutputId, GenomeOutputValue)>,
}

impl GenomeTemplate {
    pub fn new(
        id: GenomeTemplateId,
        variation_amplitude: f32,
        runtime_interval_ticks: u64,
        carrier: GenomeCarrierState,
        mut outputs: Vec<(GenomeOutputId, GenomeOutputValue)>,
    ) -> Result<Self, GenomeError> {
        if !variation_amplitude.is_finite() || !(0.0..=1.0).contains(&variation_amplitude) {
            return Err(GenomeError::InvalidVariationAmplitude);
        }
        if runtime_interval_ticks == 0 {
            return Err(GenomeError::InvalidRuntimeInterval);
        }
        outputs.sort_by_key(|(id, _)| id.as_str());
        outputs.dedup_by_key(|(id, _)| *id);
        Ok(Self {
            id,
            variation_amplitude,
            runtime_interval_ticks,
            carrier,
            outputs,
        })
    }

    pub fn id(&self) -> &GenomeTemplateId {
        &self.id
    }

    pub const fn variation_amplitude(&self) -> f32 {
        self.variation_amplitude
    }

    pub const fn runtime_interval_ticks(&self) -> u64 {
        self.runtime_interval_ticks
    }

    pub fn carrier(&self) -> &GenomeCarrierState {
        &self.carrier
    }

    pub fn outputs(&self) -> &[(GenomeOutputId, GenomeOutputValue)] {
        &self.outputs
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenomeState {
    pub id: GenomeId,
    pub template_id: GenomeTemplateId,
    pub carrier: GenomeCarrierState,
    pub outputs: Vec<(GenomeOutputId, GenomeOutputValue)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenomeError {
    EmptyTemplateId,
    UnknownOutputId(String),
    EmptyCarrierMaterialId,
    InvalidCarrierAmount,
    InvalidCarrierIntegrity,
    InvalidVariationAmplitude,
    InvalidRuntimeInterval,
    UnknownTemplate(String),
}
```

- [ ] **Step 4: Export the module**

Modify `src/core/mod.rs`:

```rust
pub mod genome;
```

- [ ] **Step 5: Run test and verify GREEN**

Run:

```bash
cargo test --test phase3a_genome_bootstrap genome_output
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/core/genome.rs src/core/mod.rs tests/phase3a_genome_bootstrap.rs
git commit -m "feat: add phase3a genome domain types"
```

---

## Task 2: RuntimeConfig Genome Fields

**Files:**
- Modify: `src/core/config.rs`
- Test: `tests/phase3a_genome_bootstrap.rs`

- [ ] **Step 1: Add failing RuntimeConfig tests**

Append to `tests/phase3a_genome_bootstrap.rs`:

```rust
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn base_cell() -> CellInitialConfig {
    CellInitialConfig {
        position: Position::new(8.0, 8.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(10.0).unwrap(),
        energy_capacity: EnergyAmount::new(20.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(1.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(2.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::new(1.0).unwrap(),
        initial_sensory_material: MaterialAmount::new(1.0).unwrap(),
    }
}

fn base_runtime_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(5),
            seed: Seed::from_raw(42),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(4.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        base_cell(),
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(20.0).unwrap(),
            heat_death_threshold: HeatAmount::new(40.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(20.0).unwrap(),
            waste_death_threshold: WasteAmount::new(40.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(2.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap()
}

#[test]
fn runtime_config_defaults_to_no_genome_templates() {
    let config = base_runtime_config();

    assert!(config.genome_templates.is_empty());
    assert_eq!(config.initial_cell_genome_templates, vec![None]);
}

#[test]
fn runtime_config_hash_changes_when_genome_template_changes() {
    use alife::core::genome::{
        GenomeCarrierState, GenomeOutputId, GenomeOutputValue, GenomeTemplate, GenomeTemplateId,
    };

    let config_a = base_runtime_config();
    let mut config_b = base_runtime_config();
    config_b.genome_templates.push(
        GenomeTemplate::new(
            GenomeTemplateId::new("balanced").unwrap(),
            0.08,
            1,
            GenomeCarrierState::new("genome_carrier_A".to_string(), 1.0, 1.0).unwrap(),
            vec![(GenomeOutputId::ResourceUptakePriority, GenomeOutputValue::new(0.7))],
        )
        .unwrap(),
    );
    config_b.initial_cell_genome_templates = vec![Some(GenomeTemplateId::new("balanced").unwrap())];

    assert_ne!(config_a.config_hash(), config_b.config_hash());
}
```

- [ ] **Step 2: Run and verify RED**

Run:

```bash
cargo test --test phase3a_genome_bootstrap runtime_config
```

Expected: compile failure for missing `RuntimeConfig::genome_templates` and `initial_cell_genome_templates`.

- [ ] **Step 3: Add config fields and hash coverage**

Modify `src/core/config.rs`:

```rust
use crate::core::genome::{GenomeTemplate, GenomeTemplateId};
```

Add fields to `RuntimeConfig`:

```rust
pub genome_templates: Vec<GenomeTemplate>,
pub initial_cell_genome_templates: Vec<Option<GenomeTemplateId>>,
```

Initialize in `RuntimeConfig::new`:

```rust
genome_templates: Vec::new(),
initial_cell_genome_templates: vec![None],
```

Update `RuntimeConfig::with_cells` so assignment count matches cell count:

```rust
if self.initial_cell_genome_templates.len() != cells.len() {
    self.initial_cell_genome_templates = vec![None; cells.len()];
}
```

Update `config_hash()` after `initial_typed_resources` hashing:

```rust
for template in &self.genome_templates {
    add_text(&mut hash, template.id().as_str());
    add(&mut hash, template.variation_amplitude().to_bits() as u64);
    add(&mut hash, template.runtime_interval_ticks());
    add_text(&mut hash, &template.carrier().material_id);
    add(&mut hash, template.carrier().amount.to_bits() as u64);
    add(&mut hash, template.carrier().integrity.to_bits() as u64);
    for (output_id, value) in template.outputs() {
        add_text(&mut hash, output_id.as_str());
        add(&mut hash, value.raw().to_bits() as u64);
    }
}
for assignment in &self.initial_cell_genome_templates {
    add_text(
        &mut hash,
        assignment
            .as_ref()
            .map(|id| id.as_str())
            .unwrap_or(""),
    );
}
```

- [ ] **Step 4: Run and verify GREEN**

Run:

```bash
cargo test --test phase3a_genome_bootstrap runtime_config
```

Expected: PASS.

- [ ] **Step 5: Run existing hash tests**

Run:

```bash
cargo test --test phase2_config_hash
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/core/config.rs tests/phase3a_genome_bootstrap.rs
git commit -m "feat: add genome templates to runtime config"
```

---

## Task 3: Scenario Parser For Genome Templates

**Files:**
- Modify: `src/runner/config_parser.rs`
- Test: `tests/phase3a_genome_config.rs`

- [ ] **Step 1: Write parser tests first**

```rust
// tests/phase3a_genome_config.rs
use alife::core::genome::GenomeOutputId;
use alife::runner::config_parser::{ParseError, RawScenarioConfig};

fn fixture_with_genome(extra: &str) -> String {
    format!(
        r#"
scenario_id = "phase3a_genome_bootstrap"
seed = 42
tick_count = 5
legacy_material_distribution = false

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 1

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = {{ nutrient_A = 2.0 }}
initial_materials = {{ boundary = 1.0, transport = 1.0, metabolic = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0 }}
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 1.0
capacity_limit = 20.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.0
heat_warning_threshold = 20.0
heat_death_threshold = 40.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.0
waste_warning_threshold = 20.0
waste_death_threshold = 40.0

[lifecycle]
stress_energy_threshold = 2.0
dormancy_allowed = false
critical_capacity_overrun = 5.0

{extra}
"#
    )
}

#[test]
fn parser_loads_genome_template_and_cell_assignment() {
    let config = RawScenarioConfig::parse(&fixture_with_genome(
        r#"
[genome_templates.balanced]
variation_amplitude = 0.08
runtime_interval_ticks = 1

[genome_templates.balanced.carrier]
material_id = "genome_carrier_A"
amount = 1.0
integrity = 1.0

[genome_templates.balanced.outputs]
resource_uptake_priority = 0.7
energy_conversion_priority = 0.6
material_synthesis_priority = 0.2
repair_priority = 0.1

[cell.genome]
template = "balanced"
"#,
    ))
    .unwrap();

    assert_eq!(config.genome_templates.len(), 1);
    assert_eq!(config.genome_templates[0].id().as_str(), "balanced");
    assert_eq!(
        config.genome_templates[0].outputs()[0].0,
        GenomeOutputId::EnergyConversionPriority
    );
    assert_eq!(
        config.initial_cell_genome_templates[0].as_ref().unwrap().as_str(),
        "balanced"
    );
}

#[test]
fn parser_rejects_unknown_genome_output_id() {
    let err = RawScenarioConfig::parse(&fixture_with_genome(
        r#"
[genome_templates.bad]
variation_amplitude = 0.08
runtime_interval_ticks = 1
[genome_templates.bad.carrier]
material_id = "genome_carrier_A"
amount = 1.0
integrity = 1.0
[genome_templates.bad.outputs]
joint_create_priority = 0.2
[cell.genome]
template = "bad"
"#,
    ))
    .unwrap_err();

    assert!(matches!(err, ParseError::ValidationError(message) if message.contains("Unknown Genome output")));
}

#[test]
fn parser_rejects_unknown_cell_genome_template() {
    let err = RawScenarioConfig::parse(&fixture_with_genome(
        r#"
[cell.genome]
template = "missing"
"#,
    ))
    .unwrap_err();

    assert!(matches!(err, ParseError::ValidationError(message) if message.contains("Unknown Genome template")));
}
```

- [ ] **Step 2: Run and verify RED**

Run:

```bash
cargo test --test phase3a_genome_config
```

Expected: compile failure because parser raw structs do not include Genome fields.

- [ ] **Step 3: Add raw parser structs**

Modify `src/runner/config_parser.rs` imports:

```rust
use crate::core::genome::{
    GenomeCarrierState, GenomeOutputId, GenomeOutputValue, GenomeTemplate, GenomeTemplateId,
};
```

Add raw structs:

```rust
#[derive(Deserialize, Debug)]
pub struct RawCellGenome {
    pub template: String,
}

#[derive(Deserialize, Debug)]
pub struct RawGenomeTemplate {
    pub variation_amplitude: f32,
    pub runtime_interval_ticks: Option<u64>,
    pub carrier: RawGenomeCarrier,
    #[serde(default)]
    pub outputs: HashMap<String, f32>,
}

#[derive(Deserialize, Debug)]
pub struct RawGenomeCarrier {
    pub material_id: String,
    pub amount: f32,
    pub integrity: f32,
}
```

Add to `RawCell`:

```rust
pub genome: Option<RawCellGenome>,
```

Add to `RawScenarioConfig`:

```rust
#[serde(default)]
pub genome_templates: HashMap<String, RawGenomeTemplate>,
```

- [ ] **Step 4: Add parser conversion**

Add helper in `src/runner/config_parser.rs`:

```rust
fn parse_genome_templates(
    raw: &HashMap<String, RawGenomeTemplate>,
) -> Result<Vec<GenomeTemplate>, ParseError> {
    let mut names: Vec<_> = raw.keys().cloned().collect();
    names.sort();
    names
        .into_iter()
        .map(|name| {
            let value = &raw[&name];
            let outputs = value
                .outputs
                .iter()
                .map(|(id, output)| {
                    let id = GenomeOutputId::parse(id).map_err(|_| {
                        ParseError::ValidationError(format!("Unknown Genome output: {id}"))
                    })?;
                    Ok((id, GenomeOutputValue::new(*output)))
                })
                .collect::<Result<Vec<_>, ParseError>>()?;
            GenomeTemplate::new(
                GenomeTemplateId::new(name.clone()).map_err(|error| {
                    ParseError::ValidationError(format!("Invalid Genome template id: {error:?}"))
                })?,
                value.variation_amplitude,
                value.runtime_interval_ticks.unwrap_or(1),
                GenomeCarrierState::new(
                    value.carrier.material_id.clone(),
                    value.carrier.amount,
                    value.carrier.integrity,
                )
                .map_err(|error| {
                    ParseError::ValidationError(format!("Invalid Genome carrier: {error:?}"))
                })?,
                outputs,
            )
            .map_err(|error| {
                ParseError::ValidationError(format!("Invalid Genome template: {error:?}"))
            })
        })
        .collect()
}
```

In `RawScenarioConfig::to_runtime_config`, after `runtime_config` is created and before return:

```rust
let genome_templates = parse_genome_templates(&self.genome_templates)?;
let known_templates: std::collections::HashSet<_> =
    genome_templates.iter().map(|template| template.id().as_str()).collect();
let mut initial_cell_genome_templates = vec![self
    .cell
    .genome
    .as_ref()
    .map(|genome| GenomeTemplateId::new(genome.template.clone()))
    .transpose()
    .map_err(|error| {
        ParseError::ValidationError(format!("Invalid Genome template reference: {error:?}"))
    })?];

if let Some(raw_cells) = &self.cells {
    initial_cell_genome_templates = raw_cells
        .iter()
        .map(|cell| {
            cell.genome
                .as_ref()
                .map(|genome| GenomeTemplateId::new(genome.template.clone()))
                .transpose()
                .map_err(|error| {
                    ParseError::ValidationError(format!(
                        "Invalid Genome template reference: {error:?}"
                    ))
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
}

for assignment in initial_cell_genome_templates.iter().flatten() {
    if !known_templates.contains(assignment.as_str()) {
        return Err(ParseError::ValidationError(format!(
            "Unknown Genome template: {}",
            assignment.as_str()
        )));
    }
}

runtime_config.genome_templates = genome_templates;
runtime_config.initial_cell_genome_templates = initial_cell_genome_templates;
```

- [ ] **Step 5: Run parser tests and verify GREEN**

Run:

```bash
cargo test --test phase3a_genome_config
```

Expected: PASS.

- [ ] **Step 6: Run existing world config parser tests**

Run:

```bash
cargo test --test phase2i_world_configs
```

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src/runner/config_parser.rs tests/phase3a_genome_config.rs
git commit -m "feat: parse phase3a genome templates"
```

---

## Task 4: Deterministic Genome Bootstrap

**Files:**
- Create: `src/core/genome_bootstrap.rs`
- Modify: `src/core/genome.rs`
- Modify: `src/core/mod.rs`
- Test: `tests/phase3a_genome_bootstrap.rs`

- [ ] **Step 1: Add failing deterministic variation tests**

Append to `tests/phase3a_genome_bootstrap.rs`:

```rust
use alife::core::genome_bootstrap::instantiate_initial_genome;

#[test]
fn genome_bootstrap_is_deterministic_for_same_seed_cell_and_template() {
    let template = GenomeTemplate::new(
        GenomeTemplateId::new("balanced").unwrap(),
        0.08,
        1,
        GenomeCarrierState::new("genome_carrier_A".to_string(), 1.0, 1.0).unwrap(),
        vec![
            (GenomeOutputId::ResourceUptakePriority, GenomeOutputValue::new(0.7)),
            (GenomeOutputId::EnergyConversionPriority, GenomeOutputValue::new(0.6)),
        ],
    )
    .unwrap();

    let a = instantiate_initial_genome(42, 0, &template);
    let b = instantiate_initial_genome(42, 0, &template);

    assert_eq!(a.outputs, b.outputs);
}

#[test]
fn genome_bootstrap_varies_different_initial_cell_ordinals() {
    let template = GenomeTemplate::new(
        GenomeTemplateId::new("balanced").unwrap(),
        0.08,
        1,
        GenomeCarrierState::new("genome_carrier_A".to_string(), 1.0, 1.0).unwrap(),
        vec![(GenomeOutputId::ResourceUptakePriority, GenomeOutputValue::new(0.7))],
    )
    .unwrap();

    let a = instantiate_initial_genome(42, 0, &template);
    let b = instantiate_initial_genome(42, 1, &template);

    assert_ne!(a.outputs, b.outputs);
}

#[test]
fn genome_bootstrap_noise_stream_is_stable_when_new_output_is_added() {
    let carrier = GenomeCarrierState::new("genome_carrier_A".to_string(), 1.0, 1.0).unwrap();
    let base = GenomeTemplate::new(
        GenomeTemplateId::new("base").unwrap(),
        0.08,
        1,
        carrier.clone(),
        vec![(GenomeOutputId::ResourceUptakePriority, GenomeOutputValue::new(0.7))],
    )
    .unwrap();
    let extended = GenomeTemplate::new(
        GenomeTemplateId::new("base").unwrap(),
        0.08,
        1,
        carrier,
        vec![
            (GenomeOutputId::ResourceUptakePriority, GenomeOutputValue::new(0.7)),
            (GenomeOutputId::EnergyConversionPriority, GenomeOutputValue::new(0.6)),
        ],
    )
    .unwrap();

    let base_genome = instantiate_initial_genome(42, 0, &base);
    let extended_genome = instantiate_initial_genome(42, 0, &extended);
    let base_uptake = base_genome.output(GenomeOutputId::ResourceUptakePriority).unwrap();
    let extended_uptake = extended_genome
        .output(GenomeOutputId::ResourceUptakePriority)
        .unwrap();

    assert_eq!(base_uptake, extended_uptake);
}
```

- [ ] **Step 2: Run and verify RED**

Run:

```bash
cargo test --test phase3a_genome_bootstrap genome_bootstrap
```

Expected: compile failure because `genome_bootstrap` and `GenomeState::output` do not exist.

- [ ] **Step 3: Add `GenomeState::output`**

Modify `src/core/genome.rs`:

```rust
impl GenomeState {
    pub fn output(&self, id: GenomeOutputId) -> Option<GenomeOutputValue> {
        self.outputs
            .iter()
            .find(|(candidate, _)| *candidate == id)
            .map(|(_, value)| *value)
    }
}
```

- [ ] **Step 4: Add deterministic bootstrap module**

Create `src/core/genome_bootstrap.rs`:

```rust
use crate::core::genome::{GenomeId, GenomeOutputValue, GenomeState, GenomeTemplate};

pub fn instantiate_initial_genome(
    world_seed: u64,
    initial_cell_ordinal: usize,
    template: &GenomeTemplate,
) -> GenomeState {
    let outputs = template
        .outputs()
        .iter()
        .map(|(output_id, value)| {
            let noise = deterministic_noise(world_seed, initial_cell_ordinal, output_id.as_str());
            (
                *output_id,
                GenomeOutputValue::new(
                    value.raw() + noise * template.variation_amplitude(),
                ),
            )
        })
        .collect();
    GenomeState {
        id: GenomeId::from_raw((initial_cell_ordinal as u32) + 1),
        template_id: template.id().clone(),
        carrier: template.carrier().clone(),
        outputs,
    }
}

fn deterministic_noise(world_seed: u64, initial_cell_ordinal: usize, output_id: &str) -> f32 {
    let mut value = world_seed
        ^ (initial_cell_ordinal as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    for byte in output_id.as_bytes() {
        value ^= *byte as u64;
        value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value ^= value >> 27;
    }
    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;
    let sample = (value >> 40) as f32 / (1_u32 << 24) as f32;
    sample * 2.0 - 1.0
}
```

Export in `src/core/mod.rs`:

```rust
pub mod genome_bootstrap;
```

- [ ] **Step 5: Run and verify GREEN**

Run:

```bash
cargo test --test phase3a_genome_bootstrap genome_bootstrap
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/core/genome.rs src/core/genome_bootstrap.rs src/core/mod.rs tests/phase3a_genome_bootstrap.rs
git commit -m "feat: add deterministic genome bootstrap"
```

---

## Task 5: World-Owned Genome State And Physical Carrier Capacity

**Files:**
- Modify: `src/core/cell_store.rs`
- Modify: `src/core/world.rs`
- Test: `tests/phase3a_genome_bootstrap.rs`

- [ ] **Step 1: Add failing world bootstrap tests**

Append to `tests/phase3a_genome_bootstrap.rs`:

```rust
#[test]
fn world_initializes_genome_state_for_assigned_initial_cell() {
    use alife::core::tick::TickExecutor;

    let mut config = base_runtime_config();
    config.genome_templates.push(
        GenomeTemplate::new(
            GenomeTemplateId::new("balanced").unwrap(),
            0.0,
            1,
            GenomeCarrierState::new("genome_carrier_A".to_string(), 1.0, 1.0).unwrap(),
            vec![(GenomeOutputId::ResourceUptakePriority, GenomeOutputValue::new(0.7))],
        )
        .unwrap(),
    );
    config.initial_cell_genome_templates = vec![Some(GenomeTemplateId::new("balanced").unwrap())];

    let executor = TickExecutor::new(config).unwrap();
    let cell = alife::core::cell_store::CellIndex::from_raw(0);
    let genome_id = executor.world().cells().genome_id(cell).unwrap();
    let genome = executor.world().genome(genome_id).unwrap();

    assert_eq!(genome.template_id.as_str(), "balanced");
    assert_eq!(
        genome.output(GenomeOutputId::ResourceUptakePriority).unwrap().raw(),
        0.7
    );
}

#[test]
fn genome_carrier_amount_counts_against_used_capacity() {
    use alife::core::tick::TickExecutor;

    let mut config = base_runtime_config();
    config.cell.initial_resource_amount = ResourceAmount::zero();
    config.cell.capacity_limit = CapacityAmount::new(20.0).unwrap();
    config.genome_templates.push(
        GenomeTemplate::new(
            GenomeTemplateId::new("balanced").unwrap(),
            0.0,
            1,
            GenomeCarrierState::new("genome_carrier_A".to_string(), 1.5, 1.0).unwrap(),
            vec![(GenomeOutputId::RepairPriority, GenomeOutputValue::new(0.5))],
        )
        .unwrap(),
    );
    config.initial_cell_genome_templates = vec![Some(GenomeTemplateId::new("balanced").unwrap())];

    let executor = TickExecutor::new(config).unwrap();
    let cell = alife::core::cell_store::CellIndex::from_raw(0);

    assert!(
        executor.world().cells().used_capacity(cell).raw() >= 10.5,
        "9 material slots at 1.0 each plus 1.5 genome carrier should use at least 10.5 capacity"
    );
}
```

- [ ] **Step 2: Run and verify RED**

Run:

```bash
cargo test --test phase3a_genome_bootstrap world_initializes_genome_state_for_assigned_initial_cell
cargo test --test phase3a_genome_bootstrap genome_carrier_amount_counts_against_used_capacity
```

Expected: compile failure for `genome_id`, `WorldState::genome`, and missing carrier capacity.

- [ ] **Step 3: Store per-cell GenomeId in CellStore**

Modify `src/core/cell_store.rs`:

```rust
use crate::core::genome::GenomeId;
```

Add fields:

```rust
genome_ids: Vec<Option<GenomeId>>,
genome_carrier_amounts: Vec<f32>,
```

Initialize in `with_capacity`:

```rust
genome_ids: Vec::with_capacity(capacity),
genome_carrier_amounts: Vec::with_capacity(capacity),
```

Push defaults in `insert_initial`:

```rust
self.genome_ids.push(None);
self.genome_carrier_amounts.push(0.0);
```

Replace `genome_capacity_placeholder` in `used_capacity()`:

```rust
let genome_capacity_used = self.genome_carrier_amounts[index.raw()];
let used = self.resource_amount(index).raw()
    + self.total_materials(index).raw()
    + genome_capacity_used
    + internal_fragments_capacity_used;
```

Add accessors:

```rust
pub fn genome_id(&self, index: CellIndex) -> Option<GenomeId> {
    self.genome_ids[index.raw()]
}

pub fn set_genome_id(&mut self, index: CellIndex, genome_id: Option<GenomeId>) {
    self.genome_ids[index.raw()] = genome_id;
}

pub fn set_genome_carrier_amount(&mut self, index: CellIndex, amount: f32) {
    self.genome_carrier_amounts[index.raw()] = amount.max(0.0);
}
```

- [ ] **Step 4: Add World genome storage and bootstrap**

Modify `src/core/world.rs`:

```rust
use crate::core::genome::{GenomeId, GenomeState};
use crate::core::genome_bootstrap::instantiate_initial_genome;
```

Add field to `WorldState`:

```rust
genomes: Vec<GenomeState>,
```

Initialize before returning `Ok(Self { ... })` in `from_config`:

```rust
let mut genomes = Vec::new();
let world_seed = config.world.seed.raw();
for cell_raw in 0..cells.len() {
    let Some(template_id) = config
        .initial_cell_genome_templates
        .get(cell_raw)
        .and_then(|assignment| assignment.as_ref())
    else {
        continue;
    };
    let template = config
        .genome_templates
        .iter()
        .find(|candidate| candidate.id().as_str() == template_id.as_str())
        .ok_or(WorldInitError::InvalidInitialState)?;
    let genome = instantiate_initial_genome(world_seed, cell_raw, template);
    let genome_id = genome.id;
    let cell = CellIndex::from_raw(cell_raw);
    cells.set_genome_id(cell, Some(genome_id));
    cells.set_genome_carrier_amount(cell, genome.carrier.amount);
    genomes.push(genome);
}
```

Include in `Self`:

```rust
genomes,
```

Add getter:

```rust
pub fn genome(&self, id: GenomeId) -> Option<&GenomeState> {
    self.genomes.iter().find(|genome| genome.id == id)
}
```

- [ ] **Step 5: Run and verify GREEN**

Run:

```bash
cargo test --test phase3a_genome_bootstrap world_initializes_genome_state_for_assigned_initial_cell genome_carrier_amount_counts_against_used_capacity
```

Expected: PASS.

- [ ] **Step 6: Run capacity/accounting smoke tests**

Run:

```bash
cargo test --test phase2_process_smoke
cargo test --test phase2i_accounting
```

Expected: PASS. If `phase2i_accounting` fails from integrated matter accounting, add a planned follow-up to include Genome carrier in integrated accounting before continuing to Tick behavior.

- [ ] **Step 7: Commit**

```bash
git add src/core/cell_store.rs src/core/world.rs tests/phase3a_genome_bootstrap.rs
git commit -m "feat: store physical genome carrier state"
```

---

## Task 6: ActionPlan Ordering From Genome Outputs

**Files:**
- Create: `src/core/action_plan.rs`
- Modify: `src/core/mod.rs`
- Modify: `src/core/process.rs`
- Test: `tests/phase3a_action_plan.rs`

- [ ] **Step 1: Write failing ActionPlan tests**

```rust
// tests/phase3a_action_plan.rs
use alife::core::action_plan::ActionPlan;
use alife::core::genome::{GenomeOutputId, GenomeOutputValue, GenomeState, GenomeTemplateId};
use alife::core::process::ProcessId;

fn genome(outputs: Vec<(GenomeOutputId, f32)>) -> GenomeState {
    GenomeState {
        id: alife::core::genome::GenomeId::from_raw(1),
        template_id: GenomeTemplateId::new("balanced").unwrap(),
        carrier: alife::core::genome::GenomeCarrierState::new(
            "genome_carrier_A".to_string(),
            1.0,
            1.0,
        )
        .unwrap(),
        outputs: outputs
            .into_iter()
            .map(|(id, value)| (id, GenomeOutputValue::new(value)))
            .collect(),
    }
}

#[test]
fn action_plan_sorts_processes_by_descending_genome_priority() {
    let genome = genome(vec![
        (GenomeOutputId::ResourceUptakePriority, 0.1),
        (GenomeOutputId::EnergyConversionPriority, 0.8),
        (GenomeOutputId::MaterialSynthesisPriority, 0.3),
    ]);

    let plan = ActionPlan::from_genome(Some(&genome));
    assert_eq!(
        plan.ordered_processes(),
        &[
            ProcessId::MetabolismEnergyConversion,
            ProcessId::MaterialSynthesis,
            ProcessId::LocalResourceUptake,
            ProcessId::RepairBoundary,
            ProcessId::ContractileDisplacement,
            ProcessId::GrowthResourceAllocation,
        ]
    );
}

#[test]
fn action_plan_uses_stable_baseline_order_without_genome() {
    let plan = ActionPlan::from_genome(None);
    assert_eq!(
        plan.ordered_processes(),
        &[
            ProcessId::LocalResourceUptake,
            ProcessId::MetabolismEnergyConversion,
            ProcessId::MaterialSynthesis,
            ProcessId::GrowthResourceAllocation,
            ProcessId::ContractileDisplacement,
            ProcessId::RepairBoundary,
        ]
    );
}

#[test]
fn action_plan_keeps_stable_tie_break_order() {
    let genome = genome(vec![
        (GenomeOutputId::MaterialSynthesisPriority, 0.5),
        (GenomeOutputId::ResourceUptakePriority, 0.5),
    ]);

    let plan = ActionPlan::from_genome(Some(&genome));
    let order = plan.ordered_processes();
    let uptake = order
        .iter()
        .position(|id| *id == ProcessId::LocalResourceUptake)
        .unwrap();
    let synthesis = order
        .iter()
        .position(|id| *id == ProcessId::MaterialSynthesis)
        .unwrap();

    assert!(uptake < synthesis, "baseline order must break priority ties");
}
```

- [ ] **Step 2: Run and verify RED**

Run:

```bash
cargo test --test phase3a_action_plan
```

Expected: compile failure because `action_plan` does not exist.

- [ ] **Step 3: Add allowed Genome output binding metadata**

Modify `src/core/process.rs`:

```rust
impl ProcessId {
    pub const fn phase3a_baseline_order(self) -> Option<usize> {
        match self {
            ProcessId::LocalResourceUptake => Some(0),
            ProcessId::MetabolismEnergyConversion => Some(1),
            ProcessId::MaterialSynthesis => Some(2),
            ProcessId::GrowthResourceAllocation => Some(3),
            ProcessId::ContractileDisplacement => Some(4),
            ProcessId::RepairBoundary => Some(5),
            _ => None,
        }
    }
}
```

- [ ] **Step 4: Add `src/core/action_plan.rs`**

```rust
use crate::core::genome::GenomeState;
use crate::core::process::ProcessId;

const PHASE3A_BASELINE: [ProcessId; 6] = [
    ProcessId::LocalResourceUptake,
    ProcessId::MetabolismEnergyConversion,
    ProcessId::MaterialSynthesis,
    ProcessId::GrowthResourceAllocation,
    ProcessId::ContractileDisplacement,
    ProcessId::RepairBoundary,
];

#[derive(Clone, Debug, PartialEq)]
pub struct ActionPlan {
    ordered_processes: Vec<ProcessId>,
}

impl ActionPlan {
    pub fn from_genome(genome: Option<&GenomeState>) -> Self {
        let mut weighted = PHASE3A_BASELINE
            .iter()
            .copied()
            .map(|process| (process, priority_for_process(genome, process)))
            .collect::<Vec<_>>();
        weighted.sort_by(|(left_id, left_priority), (right_id, right_priority)| {
            right_priority
                .total_cmp(left_priority)
                .then_with(|| {
                    left_id
                        .phase3a_baseline_order()
                        .cmp(&right_id.phase3a_baseline_order())
                })
        });
        Self {
            ordered_processes: weighted.into_iter().map(|(process, _)| process).collect(),
        }
    }

    pub fn ordered_processes(&self) -> &[ProcessId] {
        &self.ordered_processes
    }
}

fn priority_for_process(genome: Option<&GenomeState>, process: ProcessId) -> f32 {
    let Some(genome) = genome else {
        return 0.0;
    };
    genome
        .outputs
        .iter()
        .find(|(output_id, _)| output_id.process_id() == process)
        .map(|(_, value)| value.raw())
        .unwrap_or(0.0)
}
```

Export in `src/core/mod.rs`:

```rust
pub mod action_plan;
```

- [ ] **Step 5: Run and verify GREEN**

Run:

```bash
cargo test --test phase3a_action_plan
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/core/action_plan.rs src/core/mod.rs src/core/process.rs tests/phase3a_action_plan.rs
git commit -m "feat: build action plan from genome priorities"
```

---

## Task 7: Tick Uses ActionPlan But Feasibility Remains Authority

**Files:**
- Modify: `src/core/tick.rs`
- Test: `tests/phase3a_tick_integration.rs`

- [ ] **Step 1: Write failing Tick ordering tests**

```rust
// tests/phase3a_tick_integration.rs
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::genome::{
    GenomeCarrierState, GenomeOutputId, GenomeOutputValue, GenomeTemplate, GenomeTemplateId,
};
use alife::core::process::ProcessId;
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn config_with_genome(outputs: Vec<(GenomeOutputId, f32)>) -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(16.0, 16.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(10.0).unwrap(),
        energy_capacity: EnergyAmount::new(20.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(1.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(3),
            seed: Seed::from_raw(42),
            size: WorldSize::new(32.0, 32.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: true,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(2.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
            energy_per_resource: 2.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
        cell,
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(20.0).unwrap(),
            heat_death_threshold: HeatAmount::new(40.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(20.0).unwrap(),
            waste_death_threshold: WasteAmount::new(40.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(2.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap();
    config.genome_templates.push(
        GenomeTemplate::new(
            GenomeTemplateId::new("balanced").unwrap(),
            0.0,
            1,
            GenomeCarrierState::new("genome_carrier_A".to_string(), 1.0, 1.0).unwrap(),
            outputs
                .into_iter()
                .map(|(id, value)| (id, GenomeOutputValue::new(value)))
                .collect(),
        )
        .unwrap(),
    );
    config.initial_cell_genome_templates = vec![Some(GenomeTemplateId::new("balanced").unwrap())];
    config
}

#[test]
fn genome_priority_changes_attempt_order_visible_in_diagnostics_trace() {
    let mut executor = TickExecutor::new(config_with_genome(vec![
        (GenomeOutputId::MaterialSynthesisPriority, 0.9),
        (GenomeOutputId::ResourceUptakePriority, 0.1),
    ]))
    .unwrap();

    let summary = executor.step().unwrap();

    assert_eq!(
        summary.diagnostics.attempt_order_by_process.get(0),
        Some(&ProcessId::MaterialSynthesis)
    );
    assert!(
        summary
            .diagnostics
            .attempt_order_by_process
            .contains(&ProcessId::LocalResourceUptake)
    );
}

#[test]
fn high_priority_missing_capability_is_still_rejected_by_feasibility() {
    let mut executor = TickExecutor::new(config_with_genome(vec![
        (GenomeOutputId::EnergyConversionPriority, 1.0),
        (GenomeOutputId::ResourceUptakePriority, 0.1),
    ]))
    .unwrap();

    let summary = executor.step().unwrap();
    let metabolism_rejections = summary
        .diagnostics
        .rejections_by_process
        .get(&ProcessId::MetabolismEnergyConversion)
        .copied()
        .unwrap_or(0);

    assert!(metabolism_rejections > 0);
}
```

- [ ] **Step 2: Run and verify RED**

Run:

```bash
cargo test --test phase3a_tick_integration
```

Expected: compile failure for `attempt_order_by_process`, then behavior failure until Tick uses `ActionPlan`.

- [ ] **Step 3: Add observer-only attempt order trace**

Modify `src/core/summary.rs`:

```rust
pub struct ProcessDiagnostics {
    pub attempts_by_process: HashMap<ProcessId, u32>,
    pub rejections_by_process: HashMap<ProcessId, u32>,
    pub rejections_by_reason: HashMap<RejectionReason, u32>,
    pub tool_limited_mechanisms: Vec<String>,
    pub attempt_order_by_process: Vec<ProcessId>,
}
```

Keep `#[derive(Default)]`; `Vec` defaults correctly.

Modify `run_process` in `src/core/tick.rs` after attempts increment:

```rust
diagnostics.attempt_order_by_process.push(process_id);
```

- [ ] **Step 4: Refactor Tick per-cell phase to use ActionPlan**

Modify imports in `src/core/tick.rs`:

```rust
use crate::core::action_plan::ActionPlan;
```

Inside the per-cell loop, before process execution, introduce the `ActionPlan` branch and move the existing process bodies into helper functions. This keeps the refactor testable and avoids duplicating large Tick blocks inside the `match`.

```rust
let genome = self
    .world
    .cells()
    .genome_id(index)
    .and_then(|id| self.world.genome(id));
let action_plan = ActionPlan::from_genome(genome);

for process_id in action_plan.ordered_processes() {
    match *process_id {
        ProcessId::LocalResourceUptake => {
            try_resource_uptake(
                &mut self.world,
                &config,
                index,
                &mut diagnostics,
                &mut process_attempts,
                &mut process_rejections,
            );
        }
        ProcessId::MetabolismEnergyConversion => {
            let delta = try_metabolism(
                &mut self.world,
                &config,
                index,
                &mut diagnostics,
                &mut process_attempts,
                &mut process_rejections,
            );
            metabolism_heat_total += delta.heat;
            metabolism_waste_total += delta.waste;
            phase2g_metrics.resource_metabolism_sink_amount += delta.resource_sink;
        }
        ProcessId::MaterialSynthesis => {
            try_material_synthesis(
                &mut self.world,
                index,
                &mut diagnostics,
                &mut process_attempts,
                &mut process_rejections,
            );
        }
        ProcessId::GrowthResourceAllocation => {
            try_growth(
                &mut self.world,
                &config,
                index,
                &mut diagnostics,
                &mut process_attempts,
                &mut process_rejections,
            );
        }
        ProcessId::ContractileDisplacement => {
            try_displacement(
                &mut self.world,
                index,
                &mut diagnostics,
                &mut process_attempts,
                &mut process_rejections,
            );
        }
        ProcessId::RepairBoundary => {
            // do not execute repair here yet; repair remains in the existing repair loop in Phase 3A
        }
        _ => {}
    }
}
```

Add helper structs/functions below `run_process`. The bodies are mechanical extractions of the current inline blocks from `TickExecutor::step`; keep the same `run_process`, `validate_feasibility`, and world commit calls.

```rust
#[derive(Clone, Copy, Debug, Default)]
struct MetabolismDelta {
    heat: f32,
    waste: f32,
    resource_sink: f32,
}

fn try_resource_uptake(
    world: &mut WorldState,
    config: &RuntimeConfig,
    index: CellIndex,
    diagnostics: &mut ProcessDiagnostics,
    process_attempts: &mut u32,
    process_rejections: &mut u32,
) {
    if !config.resource_interaction.enabled {
        return;
    }
    let uptake_level = world
        .cells()
        .capability_level(index, crate::core::process::MaterialCapability::ResourceUptake);
    let max_uptake = config.resource_interaction.max_uptake_per_tick.raw()
        * baseline_process_level(uptake_level)
        * config.material_effects.transport_uptake_per_unit;
    let (feasible, feasibility) = run_process(
        world,
        index,
        ProcessId::LocalResourceUptake,
        max_uptake,
        diagnostics,
        process_attempts,
        process_rejections,
    );
    if !feasible {
        return;
    }
    let accepted_amount = match feasibility {
        FeasibilityResult::Allowed { accepted_amount, .. } => accepted_amount,
        FeasibilityResult::Rejected(_) => 0.0,
    };
    let layer = ResourceLayerIndex::from_raw(config.resource_interaction.uptake_layer_index);
    let coord = world.resources().coord_for_position(world.cells().position(index));
    let external_available = world
        .resources()
        .amount_at(layer, coord)
        .expect("resource interaction layer is config-validated");
    let requested = ResourceAmount::new(external_available.raw().min(accepted_amount))
        .expect("requested uptake is clamped");
    let accepted = {
        let cells = world.cells_mut_for_commit();
        cells.add_resources_limited_by_effective_capacity(
            index,
            requested,
            config.material_effects.storage_capacity_per_unit,
        )
    };
    let remaining_external = external_available.saturating_sub(accepted);
    world
        .resources_mut_for_commit()
        .set_amount_at(layer, coord, remaining_external)
        .expect("resource interaction coord is derived from grid bounds");
}
```

For `try_metabolism`, `try_material_synthesis`, `try_growth`, and `try_displacement`, extract the existing inline behavior into helpers with the signatures used above and these exact observable effects:

```text
try_metabolism:
  calls run_process with ProcessId::MetabolismEnergyConversion
  returns MetabolismDelta { heat, waste, resource_sink }
  consumes only the Feasibility accepted_amount
  adds energy through EnergyBuffer::new(next_current, current.capacity())

try_material_synthesis:
  calls run_process with ProcessId::MaterialSynthesis and requested_amount = 1.0
  calls world.execute_synthesis(index) only when feasible

try_growth:
  returns immediately unless config.growth_enabled && config.resource_interaction.enabled
  calls run_process with ProcessId::GrowthResourceAllocation and requested_amount = 1.0
  calls world.execute_growth(index, &candidate_growth) only when feasible

try_displacement:
  calls run_process with ProcessId::ContractileDisplacement and requested_amount = 1.0
  calls world.execute_displacement(index) only when feasible
```

The expected behavior is covered by `phase2_process_smoke` and the new `phase3a_tick_integration` tests; do not change process costs or Feasibility calls during this extraction.

Preserve these existing semantics:

```text
Uptake only runs when config.resource_interaction.enabled.
Metabolism only runs when config.resource_interaction.enabled.
Growth only runs when config.growth_enabled && config.resource_interaction.enabled.
Displacement still uses existing pressure semantics.
Repair stays in its separate damaged-material loop in this task.
```

- [ ] **Step 5: Run and verify GREEN**

Run:

```bash
cargo test --test phase3a_tick_integration
```

Expected: PASS.

- [ ] **Step 6: Run process smoke tests**

Run:

```bash
cargo test --test phase2_process_smoke
cargo test --test phase2_process_registry
```

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src/core/tick.rs src/core/summary.rs tests/phase3a_tick_integration.rs
git commit -m "feat: apply genome action plan in tick"
```

---

## Task 8: Repair Priority Integration

**Files:**
- Modify: `src/core/tick.rs`
- Test: `tests/phase3a_tick_integration.rs`

- [ ] **Step 1: Add failing repair-priority test**

Append to `tests/phase3a_tick_integration.rs`:

```rust
#[test]
fn repair_priority_is_present_in_action_plan_trace_when_damage_exists() {
    let mut config = config_with_genome(vec![
        (GenomeOutputId::RepairPriority, 1.0),
        (GenomeOutputId::ResourceUptakePriority, 0.1),
    ]);
    config.chemistry.repair.enabled = true;
    config.chemistry.repair.energy_cost = 0.1;
    config.chemistry.repair.max_amount_per_tick = 0.5;
    config.cell.initial_repair_material = MaterialAmount::new(1.0).unwrap();
    config.cell.initial_boundary_material = MaterialAmount::new(1.0).unwrap();
    config.cell.initial_resource_amount = ResourceAmount::new(2.0).unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    let cell = alife::core::cell_store::CellIndex::from_raw(0);
    executor
        .world_mut()
        .cells_mut_for_commit()
        .set_material_damage(cell, alife::core::materials::MaterialSlot::Boundary, 0.5);

    let summary = executor.step().unwrap();

    assert_eq!(
        summary.diagnostics.attempt_order_by_process.get(0),
        Some(&ProcessId::RepairBoundary)
    );
}
```

- [ ] **Step 2: Run and verify RED**

Run:

```bash
cargo test --test phase3a_tick_integration repair_priority
```

Expected: failure because repair is still executed after the main ActionPlan loop.

- [ ] **Step 3: Move repair execution behind an ActionPlan branch**

In `src/core/tick.rs`, extract current repair body into a helper:

```rust
fn try_repair_boundary(
    world: &mut WorldState,
    config: &RuntimeConfig,
    index: CellIndex,
    diagnostics: &mut ProcessDiagnostics,
    process_attempts: &mut u32,
    process_rejections: &mut u32,
    repair_success_count: &mut u32,
    repair_rejection_count: &mut u32,
    phase2g_metrics: &mut Phase2GMetricsDelta,
) {
    let (feasible, feasibility) = run_process(
        world,
        index,
        ProcessId::RepairBoundary,
        config.chemistry.repair.max_amount_per_tick,
        diagnostics,
        process_attempts,
        process_rejections,
    );
    match feasibility {
        FeasibilityResult::Allowed {
            accepted_amount,
            energy_cost,
            resource_cost,
        } if feasible => {
            // Mechanical extraction requirements:
            // - consume typed repair resource when repair_resource_type_id(config) returns Some
            // - otherwise consume generic resources
            // - rollback consumed resource if the consumed amount is below resource_cost
            // - subtract accepted_amount from repair material
            // - add accepted_amount to boundary material
            // - reduce boundary damage by accepted_amount, clamped at 0.0
            // - deduct energy_cost from the cell energy buffer
            // - increment repair_success_count
            // - add consumed resource to phase2g_metrics.repair_resource_sink_amount
        }
        FeasibilityResult::Rejected(_) => {
            *repair_rejection_count += 1;
        }
        _ => {}
    }
}
```

Call it from the `ProcessId::RepairBoundary` branch when:

```rust
config.chemistry.repair.enabled
    && self.world.cells().material_damage(index, MaterialSlot::Boundary) > 0.0
```

Remove the old separate repair loop after the main per-cell loop.

- [ ] **Step 4: Run and verify GREEN**

Run:

```bash
cargo test --test phase3a_tick_integration repair_priority
```

Expected: PASS.

- [ ] **Step 5: Run repair and chemistry regressions**

Run:

```bash
cargo test --test phase2g_heat_boundary_repair
cargo test --test phase2g_reactions
cargo test --test phase2i_integrated_world
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/core/tick.rs tests/phase3a_tick_integration.rs
git commit -m "feat: route repair through genome action plan"
```

---

## Task 9: Demo Scenario And Replay Gate

**Files:**
- Create: `config/scenarios/genome/phase3a_genome_bootstrap.toml`
- Test: `tests/phase3a_genome_config.rs`
- Test: `tests/phase3a_tick_integration.rs`

- [ ] **Step 1: Add failing scenario parse/replay tests**

Append to `tests/phase3a_genome_config.rs`:

```rust
#[test]
fn phase3a_demo_scenario_parses_with_genome_template() {
    let text = std::fs::read_to_string("config/scenarios/genome/phase3a_genome_bootstrap.toml")
        .unwrap();
    let config = RawScenarioConfig::parse(&text).unwrap();

    assert_eq!(config.genome_templates.len(), 1);
    assert!(config.initial_cell_genome_templates.iter().any(|id| id.is_some()));
}
```

Append to `tests/phase3a_tick_integration.rs`:

```rust
#[test]
fn phase3a_demo_scenario_replays_same_seed_and_config() {
    let text = std::fs::read_to_string("config/scenarios/genome/phase3a_genome_bootstrap.toml")
        .unwrap();
    let config_a = alife::runner::config_parser::RawScenarioConfig::parse(&text).unwrap();
    let config_b = alife::runner::config_parser::RawScenarioConfig::parse(&text).unwrap();

    let mut a = TickExecutor::new(config_a).unwrap();
    let mut b = TickExecutor::new(config_b).unwrap();

    let summary_a = a.step().unwrap();
    let summary_b = b.step().unwrap();

    assert_eq!(summary_a.config_hash, summary_b.config_hash);
    assert_eq!(
        summary_a.diagnostics.attempt_order_by_process,
        summary_b.diagnostics.attempt_order_by_process
    );
    assert_eq!(summary_a.metrics.final_energy, summary_b.metrics.final_energy);
}
```

- [ ] **Step 2: Run and verify RED**

Run:

```bash
cargo test --test phase3a_genome_config phase3a_demo
cargo test --test phase3a_tick_integration phase3a_demo
```

Expected: file-not-found failure for missing demo scenario.

- [ ] **Step 3: Add demo scenario**

Create `config/scenarios/genome/phase3a_genome_bootstrap.toml`:

```toml
scenario_id = "phase3a_genome_bootstrap"
seed = 42
tick_count = 20
legacy_material_distribution = false

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A", "waste_A"]
initial_distribution = [20.0, 0.0]
optional_decay_rate = 0.0

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 2.0
metabolism_resource_per_tick = 1.0
energy_per_resource = 2.0
heat_per_resource = 0.05
waste_per_resource = 0.05

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = { nutrient_A = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 0.0, sensory = 0.0 }
initial_energy = 8.0
energy_capacity = 20.0
mandatory_cost_per_tick = 1.0
capacity_limit = 20.0

[cell.genome]
template = "balanced"

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.1
heat_warning_threshold = 20.0
heat_death_threshold = 40.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.1
waste_warning_threshold = 20.0
waste_death_threshold = 40.0

[lifecycle]
stress_energy_threshold = 2.0
dormancy_allowed = false
critical_capacity_overrun = 5.0

[synthesis]
cost_resource = 1.0
cost_energy = 1.0

[growth]
growth_cost_resource = 1.0
growth_cost_energy = 1.0
growth_target_radius = 1.5
max_division_pressure = 0.5

[genome_templates.balanced]
variation_amplitude = 0.08
runtime_interval_ticks = 1

[genome_templates.balanced.carrier]
material_id = "genome_carrier_A"
amount = 1.0
integrity = 1.0

[genome_templates.balanced.outputs]
resource_uptake_priority = 0.7
energy_conversion_priority = 0.6
material_synthesis_priority = 0.3
division_preparation_priority = 0.1
repair_priority = 0.0
```

- [ ] **Step 4: Run and verify GREEN**

Run:

```bash
cargo test --test phase3a_genome_config phase3a_demo
cargo test --test phase3a_tick_integration phase3a_demo
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add config/scenarios/genome/phase3a_genome_bootstrap.toml tests/phase3a_genome_config.rs tests/phase3a_tick_integration.rs
git commit -m "test: add phase3a genome bootstrap scenario"
```

---

## Task 10: Full Verification And Report

**Files:**
- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-3A-genome-bootstrap.md`

- [ ] **Step 1: Run focused Phase 3A tests**

Run:

```bash
cargo test --test phase3a_genome_bootstrap
cargo test --test phase3a_genome_config
cargo test --test phase3a_action_plan
cargo test --test phase3a_tick_integration
```

Expected: all PASS.

- [ ] **Step 2: Run relevant regression tests**

Run:

```bash
cargo test --test phase2_process_registry
cargo test --test phase2_process_smoke
cargo test --test phase2_config_hash
cargo test --test phase2g_reactions
cargo test --test phase2g_heat_boundary_repair
cargo test --test phase2h_joint_creation
cargo test --test phase2i_integrated_world
cargo test --test phase2i_accounting
```

Expected: all PASS. If one fails, stop and write the failing command/output into the report before fixing.

- [ ] **Step 3: Run workspace verification**

Run:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
```

Expected: all PASS.

- [ ] **Step 4: Write implementation report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-3A-genome-bootstrap.md`:

```markdown
# REPORT: Phase 3A Genome Bootstrap

## Summary

- Added Core-owned Genome domain state, templates, and deterministic initial variation.
- Added physical Genome carrier capacity accounting.
- Added registered priority output validation and ActionPlan ordering.
- Routed Tick planned process ordering through Genome priorities while keeping Feasibility authoritative.
- Added Phase 3A demo scenario.

## Verification

```text
cargo test --test phase3a_genome_bootstrap
cargo test --test phase3a_genome_config
cargo test --test phase3a_action_plan
cargo test --test phase3a_tick_integration
cargo test --workspace --all-targets
```

## Notes

- Phase 3A does not implement local inputs, mutation, inheritance, runtime memory, lineage, or Joint creation outputs.
- `joint_create_priority` remains intentionally unsupported until the Canon registry maps it.
```

- [ ] **Step 5: Commit final report**

```bash
git add outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-3A-genome-bootstrap.md
git commit -m "docs: report phase3a genome bootstrap"
```

---

## Self-Review Checklist

- [x] Covers roadmap gate: initial Cells can reference `genome_template`.
- [x] Covers deterministic seed + config replay.
- [x] Covers per-cell deterministic variation.
- [x] Covers registered output validation.
- [x] Covers bounded output values.
- [x] Covers physical carrier capacity.
- [x] Covers ActionPlan ordering.
- [x] Keeps Feasibility final authority.
- [x] Keeps Runner out of Genome internals.
- [x] Excludes local inputs, mutation, inheritance, lineage, recurrent nodes, epigenetics.
- [x] Avoids unregistered `growth_priority` and `joint_create_priority`.
- [x] Uses failing-test-first steps for every behavior change.

## Execution Notes

Prefer implementing one task per commit. Do not start production code in a task until its RED test has been run and failed for the expected reason. If a task reveals that integrated accounting must include Genome carrier as matter, add that accounting fix inside the same task before moving to Tick behavior; do not leave physical Genome carrier as capacity-only if existing accounting treats it as matter.
