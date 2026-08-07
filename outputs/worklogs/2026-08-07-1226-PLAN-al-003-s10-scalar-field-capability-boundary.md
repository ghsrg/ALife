---
plan_id: AL-003-S10
status: proposed
date: 2026-08-07
---

# Scalar Field Capability Boundary And Profile Semantics Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the current Field runtime contract explicit, testable, and visible: all supported profile names are scalar-grid profiles only, and no profile name implies direct Energy, movement, mutation, damage, Resource transport, or Genome behavior.

**Architecture:** Keep `FieldGrid` as dense, World-owned scalar hot-path state. Add a small Core value-object/API that describes effect-profile semantics without executing effects, then bind parser validation, runtime negative controls, and canonical scenario manifest text to that contract. Do not add vector storage, flow physics, radiation mutation, light energy conversion, or direct profile behavior.

**Tech Stack:** Rust 2024, existing `alife` crate modules, TOML scenario parser, focused Rust integration tests.

---

## Roadmap-Control Classification

Type: `TDD_PLAN_REQUEST`.

Selected Plan ID: `AL-003-S10`.

Selected slice: `Scalar Field Capability Boundary And Profile Semantics`.

Current status: `planned`.

Approval gate: implementation is not authorized until the user replies `OK EXECUTE AL-003-S10`.

## Source-Of-Truth Hierarchy

1. `docs/PRINCIPLES.md`
2. `docs/GLOSSARY.md`
3. `docs/world/fields.md`
4. `docs/world/field-semantics.md`
5. `docs/world/physics.md`
6. `docs/world/materials.md`
7. `docs/config/fields_config.md`
8. `docs/mechanics/field-local-effect.md`
9. `docs/biology/process-capabilities.md`
10. `docs/delivery/roadmap.md`
11. `docs/delivery/acceptance.md`
12. Existing S09 implementation/report evidence.

## Files And Responsibilities

- Modify `src/core/fields.rs`: own typed Field profile semantics and scalar-only runtime support descriptors.
- Modify `src/runner/config_parser.rs`: make non-scalar Field kind rejection explicit and preserve the six supported scalar effect profiles.
- Modify `tests/phase3h_local_fields.rs`: add parser, semantics, runtime negative-control, and stable hash/config coverage for `AL-003-S10`.
- Modify `tests/phase3f_canonical_test_world.rs`: assert canonical scenario exposes Field capability status.
- Modify `config/scenarios/demo/canonical_test_world.toml`: disclose scalar-only runtime status and unsupported direct profile behaviors in manifest sections.
- Modify `docs/config/fields_config.md`: document current supported profile names and scalar-only runtime boundary.
- Modify `docs/world/field-semantics.md`: clarify current implementation status versus future mechanisms without changing Canon allowances.
- Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-003-s10-scalar-field-capability-boundary.md` during closure.
- Modify `outputs/worklogs/index.md` during closure to add the report link.

## Rust Domain Modeling Notes

- Concept category: `FieldEffectProfile` is a value object / config semantics enum, not an entity.
- Stable identity: Field layers have `FieldTypeId`; effect profiles do not own state and should not become IDs.
- State owner: `WorldState` owns `FieldGrid`; parser owns `FieldRuntimeConfig` construction.
- Mutation authority: only scheduled Field update phases may mutate `FieldGrid`; profile descriptors must be read-only metadata.
- Hot path: keep scalar arrays in `FieldGrid`; do not introduce per-cell heap objects, vector structs, or dynamic dispatch for this slice.
- Forbidden shortcuts: no direct Field -> Energy Buffer, Field -> Genome mutation, Field -> movement, Field -> HP-like damage, or profile-name-driven behavior.

## Assumptions

- `FieldKind` remains `Scalar` only in this slice.
- All six profile names remain accepted in config as scalar effect profiles: `temperature`, `light`, `pressure`, `radiation`, `chemical_gradient`, `flow`.
- Direct effects are all `false` unless they already exist through explicit mechanism hooks from `AL-003-S09`: field-conditioned reactions and field-mediated material degradation.
- `flow` as a profile name is accepted for scalar local sampling/manifest semantics, but vector/flow spatial representation is not implemented.

## Open Questions

- None blocking. The requested boundary is clear enough for TDD planning.

## Agent Scenario Cards

### `AL-003-S10-AC01` Scalar Profile Acceptance And Non-Scalar Rejection

Source links:

- `docs/world/fields.md`
- `docs/world/field-semantics.md`
- `docs/config/fields_config.md`
- `docs/mechanics/field-local-effect.md`

Intent: accepted profile names must not imply vector support; the runtime is scalar-grid only.

Priority: P1.

Given a scenario config with scalar Field runtime definitions for `temperature`, `light`, `pressure`, `radiation`, `chemical_gradient`, and `flow`  
When the TOML parser resolves the scenario  
Then each profile is accepted as `FieldKind::Scalar` with bounded values.

Given a scenario config with `kind = "vector"` or `kind = "flow_vector"`  
When the TOML parser resolves the scenario  
Then it rejects the config with an explicit scalar-only runtime error.

Independent verification: `cargo test --test phase3h_local_fields field_runtime_accepts_declared_scalar_profiles_only`.

Evidence IDs: `AL-003-S10-EV01`, `AL-003-S10-EV02`.

### `AL-003-S10-AC02` Profile Names Do Not Execute Direct Behavior

Source links:

- `docs/world/field-semantics.md`
- `docs/world/physics.md`
- `docs/world/materials.md`
- `docs/biology/process-capabilities.md`

Intent: profile names are descriptors until an explicit material/reaction/process/physics mechanism uses them.

Priority: P1.

Given a world with scalar `light`, `radiation`, `flow`, `pressure`, and `chemical_gradient` layers and no registered effect hooks  
When the world advances one Tick  
Then Energy Buffer, Genome state/hash contribution, Cell position, Cell material amounts, and Resource amounts are unchanged except for normal mandatory costs or scheduled scalar Field update changes.

Given code asks for profile semantics  
When the descriptor is read  
Then direct Energy, direct movement, direct mutation, direct damage, direct Resource transport, and direct Genome behavior are all reported as unsupported.

Independent verification: `cargo test --test phase3h_local_fields field_profiles_do_not_execute_direct_behavior`.

Evidence IDs: `AL-003-S10-EV03`, `AL-003-S10-EV04`.

### `AL-003-S10-AC03` Canonical Scenario Discloses Capability Status

Source links:

- `config/scenarios/demo/canonical_test_world.toml`
- `docs/config/fields_config.md`
- `tests/phase3f_canonical_test_world.rs`
- `tests/runner_scenario_loader.rs`

Intent: UI/debug/runner users can distinguish executable scalar runtime from future or manifest-only field semantics.

Priority: P1.

Given `canonical_test_world.toml`  
When scenario resolution loads the canonical source  
Then the manifest contains scalar-only Field runtime status, supported scalar profile names, and explicit unsupported direct behaviors.

Independent verification: `cargo test --test phase3f_canonical_test_world canonical_test_world_resolves_resource_derived_material_synthesis_surface`.

Evidence IDs: `AL-003-S10-EV05`, `AL-003-S10-EV06`.

## Acceptance Mapping

| Acceptance ID | Scenario cards | Evidence IDs |
| --- | --- | --- |
| `AL-003-S10-AC01` | Scalar Profile Acceptance And Non-Scalar Rejection | `AL-003-S10-EV01`, `AL-003-S10-EV02` |
| `AL-003-S10-AC02` | Profile Names Do Not Execute Direct Behavior | `AL-003-S10-EV03`, `AL-003-S10-EV04` |
| `AL-003-S10-AC03` | Canonical Scenario Discloses Capability Status | `AL-003-S10-EV05`, `AL-003-S10-EV06` |

## TDD Tasks

### `AL-003-S10-T01`: RED for `AL-003-S10-AC01`

**Files:**

- Modify: `tests/phase3h_local_fields.rs`

- [ ] Add a failing parser test named `field_runtime_accepts_declared_scalar_profiles_only`.

Use this exact test shape:

```rust
#[test]
fn field_runtime_accepts_declared_scalar_profiles_only() {
    let profiles = [
        ("temperature", FieldEffectProfile::Temperature),
        ("light", FieldEffectProfile::Light),
        ("pressure", FieldEffectProfile::Pressure),
        ("radiation", FieldEffectProfile::Radiation),
        ("chemical_gradient", FieldEffectProfile::ChemicalGradient),
        ("flow", FieldEffectProfile::Flow),
    ];

    for (profile, expected) in profiles {
        let field_id = format!("field_{profile}");
        let field = format!(
            r#"
[fields.{field_id}]
kind = "scalar"
initial_value = 0.5
diffusion_rate = 0.0
decay_rate = 0.0
min_value = 0.0
max_value = 1.0
effect_profile = "{profile}"
conserved_behavior = "abstracted"
"#
        );
        let parsed = RawScenarioConfig::parse(&minimal_field_toml(&field, ""))
            .expect("supported scalar field profile should parse");
        assert_eq!(parsed.fields[0].kind, FieldKind::Scalar);
        assert_eq!(parsed.fields[0].effect_profile, expected);
    }

    let vector_field = temperature_field_config().replace("kind = \"scalar\"", "kind = \"vector\"");
    let error = RawScenarioConfig::parse(&minimal_field_toml(&vector_field, ""))
        .expect_err("non-scalar runtime field kind should be rejected");
    assert!(
        format!("{error:?}").contains("scalar Field runtime"),
        "error should explain scalar-only support, got: {error:?}"
    );
}
```

- [ ] Run:

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields field_runtime_accepts_declared_scalar_profiles_only
```

Expected: FAIL because the non-scalar rejection message is generic (`Unknown field kind`) and does not yet expose the scalar-only runtime contract.

Record result as `AL-003-S10-EV01`.

### `AL-003-S10-T02`: GREEN for `AL-003-S10-AC01`

**Files:**

- Modify: `src/runner/config_parser.rs`
- Modify: `tests/phase3h_local_fields.rs`

- [ ] Change non-scalar Field kind validation to return an explicit message while keeping accepted runtime kinds unchanged.

Use this parser branch:

```rust
let kind = match value.kind.as_str() {
    "scalar" => FieldKind::Scalar,
    other => {
        return Err(ParseError::ValidationError(format!(
            "Unsupported field kind '{other}': current Field runtime supports scalar Field runtime only"
        )));
    }
};
```

- [ ] Run:

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields field_runtime_accepts_declared_scalar_profiles_only
```

Expected: PASS.

Record result as `AL-003-S10-EV02`.

### `AL-003-S10-T03`: RED for `AL-003-S10-AC02`

**Files:**

- Modify: `tests/phase3h_local_fields.rs`

- [ ] Add a failing semantics-descriptor test named `field_profile_semantics_are_non_command_metadata`.

Use this exact test shape:

```rust
#[test]
fn field_profile_semantics_are_non_command_metadata() {
    for profile in [
        FieldEffectProfile::Temperature,
        FieldEffectProfile::Light,
        FieldEffectProfile::Pressure,
        FieldEffectProfile::Radiation,
        FieldEffectProfile::ChemicalGradient,
        FieldEffectProfile::Flow,
    ] {
        let semantics = profile.semantics();
        assert_eq!(semantics.runtime_kind, FieldKind::Scalar);
        assert!(semantics.scalar_grid_supported);
        assert!(!semantics.direct_energy_buffer);
        assert!(!semantics.direct_cell_movement);
        assert!(!semantics.direct_genome_mutation);
        assert!(!semantics.direct_material_damage);
        assert!(!semantics.direct_resource_transport);
        assert!(!semantics.direct_genome_behavior);
    }
}
```

- [ ] Run:

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields field_profile_semantics_are_non_command_metadata
```

Expected: FAIL because `FieldEffectProfile::semantics()` and `FieldProfileSemantics` do not exist.

Record result as `AL-003-S10-EV03`.

### `AL-003-S10-T04`: GREEN for `AL-003-S10-AC02`

**Files:**

- Modify: `src/core/fields.rs`

- [ ] Add a read-only semantics value object.

Use this exact structure:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldProfileSemantics {
    pub runtime_kind: FieldKind,
    pub scalar_grid_supported: bool,
    pub direct_energy_buffer: bool,
    pub direct_cell_movement: bool,
    pub direct_genome_mutation: bool,
    pub direct_material_damage: bool,
    pub direct_resource_transport: bool,
    pub direct_genome_behavior: bool,
}

impl FieldProfileSemantics {
    pub const SCALAR_NON_COMMAND: Self = Self {
        runtime_kind: FieldKind::Scalar,
        scalar_grid_supported: true,
        direct_energy_buffer: false,
        direct_cell_movement: false,
        direct_genome_mutation: false,
        direct_material_damage: false,
        direct_resource_transport: false,
        direct_genome_behavior: false,
    };
}

impl FieldEffectProfile {
    pub const fn semantics(self) -> FieldProfileSemantics {
        match self {
            Self::Temperature
            | Self::Light
            | Self::Pressure
            | Self::Radiation
            | Self::ChemicalGradient
            | Self::Flow => FieldProfileSemantics::SCALAR_NON_COMMAND,
        }
    }
}
```

- [ ] Run:

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields field_profile_semantics_are_non_command_metadata
```

Expected: PASS.

Record result as `AL-003-S10-EV04`.

### `AL-003-S10-T05`: RED/CHARACTERIZATION for direct runtime negative controls

**Files:**

- Modify: `tests/phase3h_local_fields.rs`

- [ ] Add a runtime test named `field_profiles_do_not_execute_direct_behavior`.

Use this test body:

```rust
#[test]
fn field_profiles_do_not_execute_direct_behavior() {
    let fields = r#"
[fields.light]
kind = "scalar"
initial_value = 1.0
diffusion_rate = 0.0
decay_rate = 0.0
min_value = 0.0
max_value = 1.0
effect_profile = "light"
conserved_behavior = "abstracted"

[fields.radiation]
kind = "scalar"
initial_value = 1.0
diffusion_rate = 0.0
decay_rate = 0.0
min_value = 0.0
max_value = 1.0
effect_profile = "radiation"
conserved_behavior = "abstracted"

[fields.flow]
kind = "scalar"
initial_value = 1.0
diffusion_rate = 0.0
decay_rate = 0.0
min_value = 0.0
max_value = 1.0
effect_profile = "flow"
conserved_behavior = "abstracted"

[fields.pressure]
kind = "scalar"
initial_value = 1.0
diffusion_rate = 0.0
decay_rate = 0.0
min_value = 0.0
max_value = 1.0
effect_profile = "pressure"
conserved_behavior = "abstracted"

[fields.chemical_gradient]
kind = "scalar"
initial_value = 1.0
diffusion_rate = 0.0
decay_rate = 0.0
min_value = 0.0
max_value = 1.0
effect_profile = "chemical_gradient"
conserved_behavior = "abstracted"
"#;
    let config = RawScenarioConfig::parse(&minimal_field_toml(fields, ""))
        .expect("all scalar profiles should parse");
    let mut executor = TickExecutor::new(config).unwrap();
    let cell = alife::core::cell_store::CellIndex::from_raw(0);
    let before_energy = executor.world().cells().energy(cell);
    let before_position = executor.world().cells().position(cell);
    let before_genome_id = executor.world().cells().genome_id(cell);
    let before_fuel = executor
        .world()
        .cells()
        .typed_resource_amount(cell, alife::core::ids::ResourceTypeId::from_raw(0))
        .unwrap();
    let before_material = executor.world().cells().total_materials(cell);

    executor.step().unwrap();

    assert_eq!(executor.world().cells().energy(cell), before_energy);
    assert_eq!(executor.world().cells().position(cell), before_position);
    assert_eq!(executor.world().cells().genome_id(cell), before_genome_id);
    assert_eq!(
        executor
            .world()
            .cells()
            .typed_resource_amount(cell, alife::core::ids::ResourceTypeId::from_raw(0))
            .unwrap(),
        before_fuel
    );
    assert_eq!(executor.world().cells().total_materials(cell), before_material);
}
```

- [ ] Run:

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields field_profiles_do_not_execute_direct_behavior
```

Expected: This may PASS immediately if current runtime already has no direct behavior. Treat an immediate pass as characterization evidence, not as skipped TDD. If it fails, the failure identifies a real direct-effect bug and must be fixed before continuing.

Record result as `AL-003-S10-EV05`.

### `AL-003-S10-T06`: GREEN for any direct-effect regression found in `AL-003-S10-T05`

**Files:**

- Modify only the specific Core module that caused a direct effect if `AL-003-S10-T05` fails.

- [ ] If `AL-003-S10-T05` fails because mandatory cost changed Energy, set `mandatory_cost_per_tick = 0.0` in the test fixture and rerun; do not change production code.
- [ ] If it fails because a profile name directly mutates position, resources, materials, or genome state without a reaction/process/physics hook, remove that direct path and gate it behind an explicit mechanism.
- [ ] Run:

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields field_profiles_do_not_execute_direct_behavior
```

Expected: PASS.

Record result as `AL-003-S10-EV06` only if production code changed. If no production code changed, note that `AL-003-S10-EV05` covered the negative control as characterization.

### `AL-003-S10-T07`: RED for `AL-003-S10-AC03`

**Files:**

- Modify: `tests/phase3f_canonical_test_world.rs`

- [ ] Extend `canonical_test_world_resolves_resource_derived_material_synthesis_surface` with manifest assertions.

Use this exact assertion block:

```rust
    assert!(
        document
            .canonical_source
            .contains("runtime_kind = \"scalar_only\"")
    );
    assert!(
        document
            .canonical_source
            .contains("supported_scalar_profiles = [\"temperature\", \"light\", \"pressure\", \"radiation\", \"chemical_gradient\", \"flow\"]")
    );
    assert!(
        document
            .canonical_source
            .contains("unsupported_direct_effects = [\"field_to_energy_buffer\", \"field_to_cell_movement\", \"field_to_genome_mutation\", \"field_to_material_damage\", \"field_to_resource_transport\", \"field_to_genome_behavior\"]")
    );
```

- [ ] Run:

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3f_canonical_test_world canonical_test_world_resolves_resource_derived_material_synthesis_surface
```

Expected: FAIL because the canonical manifest does not yet disclose scalar-only support and unsupported direct effects.

Record result as `AL-003-S10-EV07`.

### `AL-003-S10-T08`: GREEN for `AL-003-S10-AC03`

**Files:**

- Modify: `config/scenarios/demo/canonical_test_world.toml`

- [ ] Add or update canonical Field manifest disclosure.

Use this TOML shape under the existing canonical manifest section:

```toml
[canonical_manifests.fields]
runtime_kind = "scalar_only"
supported_scalar_profiles = ["temperature", "light", "pressure", "radiation", "chemical_gradient", "flow"]
unsupported_direct_effects = ["field_to_energy_buffer", "field_to_cell_movement", "field_to_genome_mutation", "field_to_material_damage", "field_to_resource_transport", "field_to_genome_behavior"]
notes = "Profile names are accepted as bounded scalar runtime fields only; direct behavior requires a separate explicit material, reaction, process, or physics mechanism."
```

- [ ] Run:

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3f_canonical_test_world canonical_test_world_resolves_resource_derived_material_synthesis_surface
```

Expected: PASS.

Record result as `AL-003-S10-EV08`.

### `AL-003-S10-T09`: Documentation update for current Field support boundary

**Files:**

- Modify: `docs/config/fields_config.md`
- Modify: `docs/world/field-semantics.md`

- [ ] In `docs/config/fields_config.md`, add a section named `Current Runtime Support` with these exact facts:

```text
Current runtime support:

- kind: scalar only
- supported effect_profile names: temperature, light, pressure, radiation, chemical_gradient, flow
- unsupported runtime representations: vector, flow_vector, tensor, function layer behavior inputs
- profile names do not create direct behavior
- direct Energy, movement, mutation, damage, Resource transport, or Genome behavior requires a separate explicit mechanism
```

- [ ] In `docs/world/field-semantics.md`, add a short implementation-status note below `Concrete Profiles`:

```text
Current implementation status:

The runtime currently supports bounded scalar Field grids and explicit hooks only. The profile names below define semantics and allowed future mechanisms; they do not imply that those mechanisms are currently active.
```

- [ ] Run:

```powershell
git diff --check -- docs/config/fields_config.md docs/world/field-semantics.md
```

Expected: PASS.

Record result as `AL-003-S10-EV09`.

### `AL-003-S10-T10`: Regression and formatting verification

**Files:**

- All files changed in `AL-003-S10`.

- [ ] Run focused tests:

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields --test phase3f_canonical_test_world --test runner_scenario_loader --test scheduler_world_cadence
```

Expected: PASS.

- [ ] Run scoped formatting:

```powershell
rustfmt --edition 2024 --check src/core/fields.rs src/runner/config_parser.rs tests/phase3h_local_fields.rs tests/phase3f_canonical_test_world.rs
```

Expected: PASS.

- [ ] Run broader regression if local resources allow:

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --workspace --all-targets
```

Expected: PASS. If Windows PDB/disk limits recur, record exact toolchain failure and preserve focused evidence.

Record results as `AL-003-S10-EV10`, `AL-003-S10-EV11`, and `AL-003-S10-EV12`.

### `AL-003-S10-T11`: Closure report and delivery status

**Files:**

- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-003-s10-scalar-field-capability-boundary.md`
- Modify: `outputs/worklogs/index.md`
- Modify: `docs/delivery/roadmap.md`
- Modify: `docs/delivery/status.md`
- Modify: `docs/delivery/acceptance.md`

- [ ] Create a closure report with:
  - purpose;
  - plan ID;
  - source documents read;
  - changed files summary;
  - verification commands and results;
  - coverage matrix for `AL-003-S10-AC01`, `AL-003-S10-AC02`, `AL-003-S10-AC03`;
  - explicit note that worklogs are evidence, not source of truth.
- [ ] Add report link to `outputs/worklogs/index.md`.
- [ ] Update `docs/delivery/roadmap.md` `AL-003-S10` row to `done` only if closure verification passes.
- [ ] Update `docs/delivery/status.md` to move `AL-003-S10` into Recently Closed and choose the next current focus from Candidate Next Work.
- [ ] Update `docs/delivery/acceptance.md` evidence column for `AL-003-S10-AC01` through `AL-003-S10-AC03` with final report and test evidence.
- [ ] Run:

```powershell
git diff --check -- docs/delivery/roadmap.md docs/delivery/status.md docs/delivery/acceptance.md outputs/worklogs/index.md
```

Expected: PASS.

Record result as `AL-003-S10-EV13`.

## Forbidden Scope

- Do not add vector, tensor, or direction-field storage.
- Do not make `flow` move Cells or Resources.
- Do not make `radiation` mutate Genome or damage Genome carrier.
- Do not make `light` credit Energy Buffer.
- Do not make `pressure` produce HP-like damage.
- Do not create sensing without Material capability and explicit process/reaction wiring.
- Do not add species, roles, organs, predators, prey, scripted behavior, or organism-level control.
- Do not route Observer/UI/debug projections into Core behavior.

## Verification Commands Summary

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3f_canonical_test_world
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields --test phase3f_canonical_test_world --test runner_scenario_loader --test scheduler_world_cadence
rustfmt --edition 2024 --check src/core/fields.rs src/runner/config_parser.rs tests/phase3h_local_fields.rs tests/phase3f_canonical_test_world.rs
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --workspace --all-targets
git diff --check
```

## Self-Review

- Spec coverage: `AL-003-S10-AC01` is covered by `T01` and `T02`; `AL-003-S10-AC02` is covered by `T03` through `T06`; `AL-003-S10-AC03` is covered by `T07` through `T09`; closure is covered by `T10` and `T11`.
- Placeholder scan: no placeholder tasks are used; optional fallback handling in `T06` is explicit and bounded to a known possible characterization-test outcome.
- Type consistency: all named Rust types already exist except `FieldProfileSemantics` and `FieldEffectProfile::semantics()`, which are introduced in `T04`.
- Scope check: plan intentionally excludes vector/flow mechanics and direct profile behavior.

## Approval Request

Reply `OK EXECUTE AL-003-S10` to authorize execution of this TDD plan.

Reply `CHANGE AL-003-S10` with corrections to revise the plan.
