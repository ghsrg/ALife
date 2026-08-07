# Observer Configured Field Projection TDD Plan

> **For agentic workers:** REQUIRED SUB-SKILL: use `test-driven-development` for every implementation task. If executing task-by-task, use `executing-plans` or equivalent checkpointed execution. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** make `VisualWorldProjection.fields` expose configured scalar Field layers such as `temperature`, `light`, `pressure`, `radiation`, `chemical_gradient`, and `flow`, instead of exposing only summary `heat` and `waste`.

**Architecture:** Observer remains read-only. The source of truth must be committed runtime Field state or an explicit committed/snapshot field projection, not UI state and not scenario TOML read ad hoc by the viewer. `heat` and `waste` remain summary metrics or compatibility fields until a separate cleanup removes them from the projection contract.

**Tech Stack:** Rust core/observer, existing `CommittedSnapshot`, `VisualWorldProjection`, runner HTTP projection serialization, UI debug projection adapter.

---

## Current Evidence

- `config/scenarios/demo/canonical_test_world.toml` already defines configured scalar fields under `[[bootstrap.fields]]` and `[fields.*]`.
- `src/observer/projection.rs` currently hardcodes `VisualWorldProjection.fields` to only:
  - `heat`
  - `waste`
- `src/viewer_server/api/projections.rs` serializes whatever Observer provides.
- Therefore UI `No config` is correct for current live payload: configured fields are absent from `VisualWorldProjection.fields`.

## Non-Goals

- Do not make Field profiles execute direct behavior. `light` must not credit energy, `radiation` must not mutate genome, and `flow` must not move cells/resources.
- Do not read scenario TOML directly from UI or viewer server to fake configured fields.
- Do not introduce vector/flow field mechanics in this slice.
- Do not make Observer projections influence simulation.

## Files

- Modify: `src/core/snapshot.rs`
  - Add committed scalar Field summary/tile data if it is not already present in the committed snapshot.
- Modify: `src/observer/payloads.rs`
  - Extend `FieldSummaryPayload` only if needed to distinguish summary metrics from configured field layers.
- Modify: `src/observer/projection.rs`
  - Build `VisualWorldProjection.fields` from committed configured scalar Field state.
- Modify: `src/viewer_server/api/projections.rs`
  - Preserve serialized field IDs, values, and source metrics.
- Modify: `tests/observer_projection_payloads.rs`
  - Add direct Observer projection tests.
- Modify: `tests/runner_http_projections.rs`
  - Add HTTP-level contract test for configured fields.
- Modify: `ui/control-center/src/components/LayerPanel.test.tsx`
  - Update UI expectation after backend support lands.

---

### Task 1: Add a direct Observer RED test for configured scalar fields

**Files:**
- Modify: `tests/observer_projection_payloads.rs`

- [ ] **Step 1: Write the failing test**

Add a test that constructs a `CommittedSnapshot` with committed configured field summaries. If `CommittedSnapshot` does not yet have such a field, add the test using the intended API first; it must fail to compile or fail at assertion before implementation.

Target test shape:

```rust
#[test]
fn visual_world_projection_exposes_configured_scalar_fields() {
    let snapshot = committed_snapshot_with_scalar_fields(vec![
        ("temperature", 25.0, "CommittedSnapshot.fields.temperature"),
        ("light", 0.75, "CommittedSnapshot.fields.light"),
        ("pressure", 1.1, "CommittedSnapshot.fields.pressure"),
    ]);

    let payload = build_visual_world_projection(&snapshot);

    let fields: Vec<(&str, f32, &str)> = payload
        .fields
        .iter()
        .map(|field| {
            (
                field.field_id.as_str(),
                field.value,
                field.source_metric.source_path.as_str(),
            )
        })
        .collect();

    assert!(fields.contains(&("temperature", 25.0, "CommittedSnapshot.fields.temperature")));
    assert!(fields.contains(&("light", 0.75, "CommittedSnapshot.fields.light")));
    assert!(fields.contains(&("pressure", 1.1, "CommittedSnapshot.fields.pressure")));
}
```

- [ ] **Step 2: Run the test to verify RED**

Run:

```powershell
cargo test --test observer_projection_payloads visual_world_projection_exposes_configured_scalar_fields
```

Expected: FAIL because `CommittedSnapshot` does not expose configured field summaries yet, or because `build_visual_world_projection` returns only `heat`/`waste`.

---

### Task 2: Add committed scalar Field projection data to the snapshot boundary

**Files:**
- Modify: `src/core/snapshot.rs`
- Update helper constructors in tests that instantiate `CommittedSnapshot` directly.

- [ ] **Step 1: Implement the minimal committed data shape**

Add a small source-backed structure. Keep it scalar-only.

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct ScalarFieldSnapshot {
    pub field_id: String,
    pub value: f32,
}
```

Add to `CommittedSnapshot`:

```rust
pub scalar_fields: Vec<ScalarFieldSnapshot>,
```

- [ ] **Step 2: Update direct snapshot constructors**

For existing tests that do not care about configured fields, set:

```rust
scalar_fields: vec![],
```

- [ ] **Step 3: Run snapshot/compiler checks**

Run:

```powershell
cargo test --test observer_projection_payloads visual_world_projection_is_bounded_and_source_backed
```

Expected: PASS after all direct constructors include `scalar_fields`.

---

### Task 3: Project configured scalar fields in Observer

**Files:**
- Modify: `src/observer/projection.rs`

- [ ] **Step 1: Replace hardcoded-only field construction**

Build configured fields from `snapshot.scalar_fields`, preserving summary `heat`/`waste` only if existing downstream tests require them.

Implementation shape:

```rust
let mut fields: Vec<FieldSummaryPayload> = snapshot
    .scalar_fields
    .iter()
    .map(|field| FieldSummaryPayload {
        field_id: field.field_id.clone(),
        value: field.value,
        source_metric: source_metric(
            &field.field_id,
            &format!("CommittedSnapshot.fields.{}", field.field_id),
        ),
    })
    .collect();

fields.push(FieldSummaryPayload {
    field_id: "heat".to_string(),
    value: snapshot.heat,
    source_metric: source_metric("heat", "CommittedSnapshot.heat"),
});
fields.push(FieldSummaryPayload {
    field_id: "waste".to_string(),
    value: snapshot.waste,
    source_metric: source_metric("waste", "CommittedSnapshot.waste"),
});
```

- [ ] **Step 2: Include field source metrics**

Append configured field source metrics to `source_metrics`:

```rust
source_metrics.extend(snapshot.scalar_fields.iter().map(|field| {
    source_metric(
        &field.field_id,
        &format!("CommittedSnapshot.fields.{}", field.field_id),
    )
}));
```

- [ ] **Step 3: Run RED test to verify GREEN**

Run:

```powershell
cargo test --test observer_projection_payloads visual_world_projection_exposes_configured_scalar_fields
```

Expected: PASS.

---

### Task 4: Populate committed scalar fields from runtime Field state

**Files:**
- Modify the snapshot builder that creates `CommittedSnapshot` from runtime state.
- Likely inspect before editing:
  - `src/core/snapshot.rs`
  - `src/core/world.rs`
  - `src/core/fields.rs`
  - any builder that already fills `resource_layers`, `heat`, and `waste`.

- [ ] **Step 1: Write a failing integration test**

Create or extend a test that runs a scenario with configured fields and reads the committed projection.

Preferred test location:

```text
tests/phase3h_local_fields.rs
```

Test shape:

```rust
#[test]
fn committed_snapshot_contains_configured_scalar_fields() {
    let scenario = RawScenarioConfig::parse(&minimal_field_toml(
        r#"
[fields.temperature]
kind = "scalar"
initial_value = 25.0
diffusion_rate = 0.0
decay_rate = 0.0
min_value = 0.0
max_value = 100.0
effect_profile = "temperature"

[fields.light]
kind = "scalar"
initial_value = 0.75
diffusion_rate = 0.0
decay_rate = 0.0
min_value = 0.0
max_value = 1.0
effect_profile = "light"
"#,
        "",
    ))
    .unwrap()
    .resolve()
    .unwrap();

    let snapshot = run_or_bootstrap_one_committed_snapshot(scenario);

    assert!(snapshot.scalar_fields.iter().any(|field| {
        field.field_id == "temperature" && (field.value - 25.0).abs() < 0.001
    }));
    assert!(snapshot.scalar_fields.iter().any(|field| {
        field.field_id == "light" && (field.value - 0.75).abs() < 0.001
    }));
}
```

Use the project’s existing helper for obtaining a committed snapshot; if no helper exists, add a small test-only helper beside the existing phase3h tests.

- [ ] **Step 2: Run the test to verify RED**

Run:

```powershell
cargo test --test phase3h_local_fields committed_snapshot_contains_configured_scalar_fields
```

Expected: FAIL because committed snapshots currently do not carry configured scalar field summaries.

- [ ] **Step 3: Implement snapshot extraction**

In the snapshot builder, iterate runtime scalar Field layers and emit:

```rust
ScalarFieldSnapshot {
    field_id: runtime_field.id.clone(),
    value: runtime_field.summary_value_or_center_sample(),
}
```

Use an existing bounded summary/sample function if available. If not available, implement a deterministic average over the scalar grid:

```rust
let value = field.grid().iter_values().sum::<f32>() / field.grid().len() as f32;
```

Do not add behavior effects; this is read-only projection data.

- [ ] **Step 4: Run the test to verify GREEN**

Run:

```powershell
cargo test --test phase3h_local_fields committed_snapshot_contains_configured_scalar_fields
```

Expected: PASS.

---

### Task 5: Add HTTP projection contract coverage

**Files:**
- Modify: `tests/runner_http_projections.rs`

- [ ] **Step 1: Write the failing HTTP-level test**

Add a test that starts or loads `canonical_test_world` and verifies `/projections` exposes configured fields.

Expected field IDs:

```rust
let expected = [
    "temperature",
    "light",
    "pressure",
    "radiation",
    "chemical_gradient",
    "flow",
];
```

Assert that each appears under:

```text
visual_world.fields[].field_id
```

- [ ] **Step 2: Run the test to verify RED/GREEN**

Run:

```powershell
cargo test --test runner_http_projections latest_projections_expose_configured_scalar_field_layers
```

Expected after Tasks 2-4: PASS.

---

### Task 6: Update UI expectation after Observer support lands

**Files:**
- Modify: `ui/control-center/src/components/LayerPanel.test.tsx`

- [ ] **Step 1: Replace the temporary `No config` expectation for live configured scenarios**

Keep `No config` only for payloads that truly lack configured fields.

Add or update a test fixture where fields contain:

```ts
[
  { fieldId: 'temperature', value: 25, sourceMetric: { fieldId: 'temperature', sourceOwner: 'CoreCommittedSnapshot', sourcePath: 'CommittedSnapshot.fields.temperature' } },
  { fieldId: 'light', value: 0.75, sourceMetric: { fieldId: 'light', sourceOwner: 'CoreCommittedSnapshot', sourcePath: 'CommittedSnapshot.fields.light' } }
]
```

Assert:

```ts
expect(screen.getByLabelText('Field layer Temperature')).toBeInTheDocument();
expect(screen.getByLabelText('Field layer Light')).toBeInTheDocument();
expect(screen.queryByText('No config')).not.toBeInTheDocument();
```

- [ ] **Step 2: Run UI focused tests**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/components/LayerPanel.test.tsx
```

Expected: PASS.

---

## Final Verification

Run:

```powershell
cargo fmt --check
cargo test --test observer_projection_payloads
cargo test --test phase3h_local_fields committed_snapshot_contains_configured_scalar_fields
cargo test --test runner_http_projections latest_projections_expose_configured_scalar_field_layers
cd ui/control-center
npm.cmd test -- --run src/components/LayerPanel.test.tsx
npm.cmd run build
```

Expected:

- Rust tests pass.
- UI focused tests pass.
- UI build passes.
- `VisualWorldProjection.fields` contains configured scalar Field IDs for configured scenarios.
- No simulation behavior changes.

## Open Questions Before Execution

1. Which committed scalar field summary is canonical for UI list rows: average, center sample, total-like summary, or bounded tile metadata?
2. Should `heat` and `waste` stay in `VisualWorldProjection.fields` as compatibility summary fields or move to a separate summary payload in a later cleanup?
3. Should UI display scalar field values in the row later, or keep the current `color + name + switch` contract only?
