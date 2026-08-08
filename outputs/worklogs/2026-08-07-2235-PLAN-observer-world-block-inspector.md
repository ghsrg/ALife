# Observer World Block Inspector Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: use `test-driven-development` for every implementation task. If executing task-by-task, use `executing-plans` or `subagent-driven-development` with review checkpoints. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** make configured resource layers and scalar fields visible, selectable, and inspectable in the Control Center `World` level Inspector.

**Architecture:** Observer remains read-only and projects committed Core state into bounded UI/debug payloads. `VisualWorldProjection` becomes the source for layer lists, world tile hit targets, selected world-block details, resource amounts, and scalar field samples. UI must not read scenario TOML or infer hidden runtime state; it renders only projection data and explicit completeness states.

**Tech Stack:** Rust core snapshots, Rust Observer payloads/projections, Axum runner `/projections/latest`, TypeScript projection adapters, Zustand app state, React Control Center components, Vitest, Rust integration tests.

---

## Current Evidence

- `WorldViewer` already emits `world-block` selections at `LEVEL = World`, but these selections are only coordinate/bounds objects.
- `CellInspector` was renamed to `Inspector`, but it still renders only `selectedCell` data.
- `VisualWorldProjection.resource_layers` contains bounded resource grid cells and resource names.
- `VisualWorldProjection.fields` currently exposes only summary `heat` and `waste`; configured fields such as `temperature`, `light`, `pressure`, `radiation`, `chemical_gradient`, and `flow` are absent.
- UI can list resource layers from debug projection, but does not expose selectable world tile details in Inspector.
- There is no stable UI payload shape for selected world-block resources/fields.

## Non-Goals

- Do not add Field mechanics behavior in this slice.
  - `light` must not grant energy.
  - `radiation` must not mutate genomes.
  - `flow` must not move cells/resources.
- Do not add vector/flow field physics.
- Do not make Observer data mutate or influence Core.
- Do not read TOML from UI to fake configured Fields or Resources.
- Do not implement organism/cell behavioral classification in this slice.

## Acceptance Criteria

- **AC01:** `/projections/latest` for `canonical_test_world` returns named configured resource layers and configured scalar field layers.
- **AC02:** Each scalar Field layer has bounded tile/sample data, not only a global summary value.
- **AC03:** UI `Layers & Filters` shows configured Fields as rows with `name + color + switch`, not `No config`, when Observer provides configured fields.
- **AC04:** UI `World Grid` visual effect displays world/resource tile boundaries.
- **AC05:** At `LEVEL = World`, clicking a visible world tile selects a `world-block` with stable `blockX`, `blockY`, bounds, and completeness.
- **AC06:** Inspector renders selected world-block details:
  - tile coordinates and bounds;
  - resource amounts for all configured resource layers at that tile;
  - scalar field samples for all configured scalar field layers at that tile;
  - completeness/source information.
- **AC07:** At `LEVEL = Cells`, clicking a Cell continues to show Cell details in Inspector.
- **AC08:** If configured fields are absent, `Fields` shows `No config` and Inspector shows resource data without fake field rows.

## File Map

- Modify: `src/core/snapshot.rs`
  - Add committed scalar field grid/tile snapshots.
  - Keep snapshot read-only and bounded.
- Modify: `src/core/world.rs` and/or field storage accessors
  - Expose scalar field layer read access to snapshot creation.
- Modify: `src/observer/payloads.rs`
  - Add field layer grid payload types.
  - Add world block inspection payload if stored server-side, or ensure `VisualWorldProjection` carries enough data for client-side lookup.
- Modify: `src/observer/projection.rs`
  - Project configured scalar field layers from committed snapshot.
  - Preserve resource layer projection.
- Modify: `src/viewer_server/api/projections.rs`
  - Serialize resource/field grid payloads with snake_case wire names.
- Modify: `tests/observer_projection_payloads.rs`
  - Add direct Observer projection tests.
- Modify: `tests/runner_http_projections.rs`
  - Add API contract tests for configured field/resource grids.
- Modify: `ui/control-center/src/projection/types.ts`
  - Add `DebugFieldLayer`, `DebugFieldCell`, `WorldBlockInspection`-ready types.
- Modify: `ui/control-center/src/projection/debugProjectionAdapter.ts`
  - Normalize field layer grid payload from snake_case wire JSON.
- Modify: `ui/control-center/src/app/selectionModel.ts`
  - Keep `world-block` selection; add helper for stable tile key if needed.
- Modify: `ui/control-center/src/app/appState.ts`
  - Preserve selected `world-block` across frame/debug projection updates.
  - Add selector/helper for current world block inspection model if needed.
- Modify: `ui/control-center/src/viewer/worldRenderer.ts`
  - Render selected world-block highlight when `World Grid` is on or when selected.
- Modify: `ui/control-center/src/viewer/worldRenderer.test.ts`
  - Verify grid and selected world-block render plan.
- Modify: `ui/control-center/src/components/WorldViewer.tsx`
  - Add explicit world tile hit targets or reliable viewer-surface world-coordinate conversion.
- Modify: `ui/control-center/src/components/WorldViewer.test.tsx`
  - Verify world tile click selection and no Cell selection at `World` level.
- Modify: `ui/control-center/src/components/LayerPanel.tsx`
  - Show configured field rows from Observer.
- Modify: `ui/control-center/src/components/LayerPanel.test.tsx`
  - Verify configured field rows and `No config` fallback.
- Modify: `ui/control-center/src/components/InspectorPanel.tsx`
  - Route by `currentSelection.kind`.
- Create: `ui/control-center/src/components/WorldBlockInspector.tsx`
  - Render selected world tile resources and fields.
- Create: `ui/control-center/src/components/WorldBlockInspector.test.tsx`
  - Verify resource/field detail rendering.

---

## Task 1: Add direct Observer RED test for configured scalar field grids

**Files:**
- Modify: `tests/observer_projection_payloads.rs`

- [ ] **Step 1: Write the failing test**

Add this test. It intentionally uses the desired snapshot API first; it should fail until `CommittedSnapshot` supports scalar field layers.

```rust
#[test]
fn visual_world_projection_exposes_configured_scalar_field_layers_with_cells() {
    use alife::core::snapshot::{
        CommittedSnapshot, FieldLayerCellSnapshot, ScalarFieldLayerSnapshot,
    };
    use alife::core::units::Tick;
    use alife::observer::projection::build_visual_world_projection;

    let snapshot = CommittedSnapshot {
        tick: Tick::from_raw(7),
        cells: vec![],
        joints: vec![],
        organisms: vec![],
        heat: 0.0,
        waste: 0.0,
        resource_layer_totals: vec![],
        resource_layers: vec![],
        scalar_field_layers: vec![
            ScalarFieldLayerSnapshot {
                field_id: "temperature".to_string(),
                width: 2,
                height: 2,
                summary_value: 25.0,
                cells: vec![
                    FieldLayerCellSnapshot { x: 0, y: 0, value: 20.0 },
                    FieldLayerCellSnapshot { x: 1, y: 0, value: 30.0 },
                    FieldLayerCellSnapshot { x: 0, y: 1, value: 22.0 },
                    FieldLayerCellSnapshot { x: 1, y: 1, value: 28.0 },
                ],
            },
            ScalarFieldLayerSnapshot {
                field_id: "light".to_string(),
                width: 2,
                height: 2,
                summary_value: 0.5,
                cells: vec![
                    FieldLayerCellSnapshot { x: 0, y: 0, value: 0.1 },
                    FieldLayerCellSnapshot { x: 1, y: 0, value: 0.2 },
                    FieldLayerCellSnapshot { x: 0, y: 1, value: 0.8 },
                    FieldLayerCellSnapshot { x: 1, y: 1, value: 0.9 },
                ],
            },
        ],
    };

    let projection = build_visual_world_projection(&snapshot);

    let temperature = projection
        .field_layers
        .iter()
        .find(|layer| layer.field_id == "temperature")
        .expect("temperature field layer should be projected");
    assert_eq!(temperature.width, 2);
    assert_eq!(temperature.height, 2);
    assert_eq!(temperature.summary_value, 25.0);
    assert!(temperature.cells.iter().any(|cell| {
        cell.x == 1 && cell.y == 0 && (cell.value - 30.0).abs() < f32::EPSILON
    }));

    let field_ids: Vec<&str> = projection
        .fields
        .iter()
        .map(|field| field.field_id.as_str())
        .collect();
    assert!(field_ids.contains(&"temperature"));
    assert!(field_ids.contains(&"light"));
}
```

- [ ] **Step 2: Run RED**

Run:

```powershell
cargo test --test observer_projection_payloads visual_world_projection_exposes_configured_scalar_field_layers_with_cells
```

Expected: FAIL to compile because `ScalarFieldLayerSnapshot`, `FieldLayerCellSnapshot`, `CommittedSnapshot.scalar_field_layers`, or `VisualWorldProjection.field_layers` do not exist yet.

---

## Task 2: Add committed scalar field layer snapshots

**Files:**
- Modify: `src/core/snapshot.rs`
- Modify direct `CommittedSnapshot` constructors in tests.

- [ ] **Step 1: Implement the minimal snapshot types**

Add near `ResourceLayerCellSnapshot`:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct FieldLayerCellSnapshot {
    pub x: u32,
    pub y: u32,
    pub value: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScalarFieldLayerSnapshot {
    pub field_id: String,
    pub width: u32,
    pub height: u32,
    pub summary_value: f32,
    pub cells: Vec<FieldLayerCellSnapshot>,
}
```

Add to `CommittedSnapshot`:

```rust
pub scalar_field_layers: Vec<ScalarFieldLayerSnapshot>,
```

- [ ] **Step 2: Keep existing constructors compiling**

For every direct `CommittedSnapshot { ... }` literal that does not care about fields, add:

```rust
scalar_field_layers: vec![],
```

- [ ] **Step 3: Run compiler-focused tests**

Run:

```powershell
cargo test --test observer_projection_payloads visual_world_projection_exposes_configured_scalar_field_layers_with_cells
```

Expected: still FAIL because Observer payload/projection does not expose `field_layers` yet.

---

## Task 3: Add Observer payload types for scalar field layers

**Files:**
- Modify: `src/observer/payloads.rs`

- [ ] **Step 1: Write/extend payload structs**

Add these serializable payloads:

```rust
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct FieldLayerCellPayload {
    pub x: u32,
    pub y: u32,
    pub value: f32,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct FieldLayerSummaryPayload {
    pub field_id: String,
    pub width: u32,
    pub height: u32,
    pub summary_value: f32,
    pub cells: Vec<FieldLayerCellPayload>,
    pub completeness: ProjectionCompleteness,
}
```

Extend `VisualWorldProjection`:

```rust
pub field_layers: Vec<FieldLayerSummaryPayload>,
```

- [ ] **Step 2: Update all `VisualWorldProjection` constructors**

For tests or helpers not relevant to field layers, set:

```rust
field_layers: vec![],
```

- [ ] **Step 3: Run RED again**

Run:

```powershell
cargo test --test observer_projection_payloads visual_world_projection_exposes_configured_scalar_field_layers_with_cells
```

Expected: FAIL assertion because `field_layers` is empty or configured `fields` still miss configured IDs.

---

## Task 4: Project configured scalar fields from committed snapshot

**Files:**
- Modify: `src/observer/projection.rs`

- [ ] **Step 1: Add field layer projection**

In `build_visual_world_projection_sampled`, project `snapshot.scalar_field_layers` using the same `grid_stride` rule as resource layers:

```rust
let field_layers = snapshot
    .scalar_field_layers
    .iter()
    .map(|layer| FieldLayerSummaryPayload {
        field_id: layer.field_id.clone(),
        width: layer.width,
        height: layer.height,
        summary_value: layer.summary_value,
        cells: layer
            .cells
            .iter()
            .filter(|cell| stride == 1 || (cell.x % stride == 0 && cell.y % stride == 0))
            .map(|cell| FieldLayerCellPayload {
                x: cell.x,
                y: cell.y,
                value: cell.value,
            })
            .collect(),
        completeness: ProjectionCompleteness::bounded(
            "CommittedSnapshot exposes scalar field grid cells for this bounded world.",
        ),
    })
    .collect();
```

- [ ] **Step 2: Add configured fields to summary `fields`**

Replace the current hardcoded-only `fields` with configured field summaries plus compatibility summaries:

```rust
let mut fields: Vec<FieldSummaryPayload> = snapshot
    .scalar_field_layers
    .iter()
    .map(|layer| FieldSummaryPayload {
        field_id: layer.field_id.clone(),
        value: layer.summary_value,
        source_metric: source_metric(
            &layer.field_id,
            &format!("CommittedSnapshot.scalar_field_layers.{}", layer.field_id),
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

- [ ] **Step 3: Add configured field source metrics**

Before building `VisualWorldProjection`, extend `source_metrics`:

```rust
source_metrics.extend(snapshot.scalar_field_layers.iter().map(|layer| {
    source_metric(
        &layer.field_id,
        &format!("CommittedSnapshot.scalar_field_layers.{}", layer.field_id),
    )
}));
```

- [ ] **Step 4: Run GREEN**

Run:

```powershell
cargo test --test observer_projection_payloads visual_world_projection_exposes_configured_scalar_field_layers_with_cells
```

Expected: PASS.

---

## Task 5: Populate scalar field layers from runtime world state

**Files:**
- Modify: `src/core/snapshot.rs`
- Inspect and modify only the actual scalar field storage owner, likely under `src/core/world.rs` or field-related modules.
- Modify: `tests/phase3h_local_fields.rs` or create a focused test if that file is not the correct owner.

- [ ] **Step 1: Write the failing runtime snapshot test**

Add a test that starts from a scenario/config with two scalar fields and asserts `CommittedSnapshot::from_world` contains field grids.

Target test shape:

```rust
#[test]
fn committed_snapshot_contains_configured_scalar_field_layers() {
    let world = world_from_inline_scenario(r#"
scenario_id = "snapshot_field_projection_test"
seed = 11

[world]
width = 20
height = 20

[[bootstrap.fields]]
field = "temperature"
shape = "rect"
x = 0
y = 0
width = 2
height = 2
value = 25.0

[[bootstrap.fields]]
field = "light"
shape = "rect"
x = 0
y = 0
width = 2
height = 2
value = 0.75

[fields.temperature]
kind = "scalar"
initial_value = 10.0
diffusion_rate = 0.0
decay_rate = 0.0
min_value = 0.0
max_value = 100.0
effect_profile = "temperature"

[fields.light]
kind = "scalar"
initial_value = 0.0
diffusion_rate = 0.0
decay_rate = 0.0
min_value = 0.0
max_value = 1.0
effect_profile = "light"
"#);

    let snapshot = CommittedSnapshot::from_world(&world);

    let temperature = snapshot
        .scalar_field_layers
        .iter()
        .find(|layer| layer.field_id == "temperature")
        .expect("temperature should be committed");
    assert!(temperature.cells.iter().any(|cell| cell.value > 0.0));

    let light = snapshot
        .scalar_field_layers
        .iter()
        .find(|layer| layer.field_id == "light")
        .expect("light should be committed");
    assert!(light.cells.iter().any(|cell| cell.value > 0.0));
}
```

Use existing scenario/config helpers instead of inventing `world_from_inline_scenario` if the repository already has a canonical helper. The test must prove runtime state, not a manually fabricated snapshot.

- [ ] **Step 2: Run RED**

Run:

```powershell
cargo test --test phase3h_local_fields committed_snapshot_contains_configured_scalar_field_layers
```

Expected: FAIL because `CommittedSnapshot::from_world` does not populate configured scalar field layers yet.

- [ ] **Step 3: Implement snapshot extraction**

In `CommittedSnapshot::from_world`, read scalar field layers from `WorldState` and create one `ScalarFieldLayerSnapshot` per configured field:

```rust
let scalar_field_layers = world
    .fields()
    .scalar_layers()
    .iter()
    .map(|layer| {
        let mut cells = Vec::with_capacity(layer.cell_count());
        let mut sum = 0.0f32;
        for y in 0..layer.height() {
            for x in 0..layer.width() {
                let value = layer.value_at(x, y);
                sum += value;
                cells.push(FieldLayerCellSnapshot {
                    x: x as u32,
                    y: y as u32,
                    value,
                });
            }
        }
        let summary_value = if cells.is_empty() {
            0.0
        } else {
            sum / cells.len() as f32
        };
        ScalarFieldLayerSnapshot {
            field_id: layer.id().to_string(),
            width: layer.width() as u32,
            height: layer.height() as u32,
            summary_value,
            cells,
        }
    })
    .collect();
```

Adapt method names to the actual field storage API. Do not add new write paths or behavior effects.

- [ ] **Step 4: Run GREEN**

Run:

```powershell
cargo test --test phase3h_local_fields committed_snapshot_contains_configured_scalar_field_layers
```

Expected: PASS.

---

## Task 6: Add HTTP projection contract for configured resources and fields

**Files:**
- Modify: `tests/runner_http_projections.rs`

- [ ] **Step 1: Write failing API test**

Add a test:

```rust
#[tokio::test]
async fn latest_projections_expose_configured_resource_and_field_grids() {
    let base_url = spawn_test_server().await;
    let client = reqwest::Client::new();

    let start = client
        .post(format!("{base_url}/run/start"))
        .json(&serde_json::json!({
            "scenario_id": "canonical_test_world",
            "request_id": "observer-world-block-inspector-test"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(start.status(), 200);

    let response = client
        .get(format!("{base_url}/projections/latest"))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let json: serde_json::Value = response.json().await.unwrap();

    let resource_ids: Vec<&str> = json["visual_world"]["resource_layers"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|layer| layer["resource_id"].as_str())
        .collect();
    assert!(resource_ids.contains(&"amino_acid"));
    assert!(resource_ids.contains(&"nucleotide_precursor"));

    let field_ids: Vec<&str> = json["visual_world"]["field_layers"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|layer| layer["field_id"].as_str())
        .collect();
    for expected in [
        "temperature",
        "light",
        "pressure",
        "radiation",
        "chemical_gradient",
        "flow",
    ] {
        assert!(field_ids.contains(&expected), "missing field layer {expected}");
    }

    let first_field = &json["visual_world"]["field_layers"][0];
    assert!(first_field["cells"].as_array().unwrap().len() > 0);
    assert!(first_field["cells"][0]["value"].is_number());
}
```

- [ ] **Step 2: Run API test**

Run:

```powershell
cargo test --test runner_http_projections latest_projections_expose_configured_resource_and_field_grids
```

Expected after Tasks 1-5: PASS. If it fails only because `/projections/latest` races before first committed snapshot, use the existing polling helper pattern from current projection tests.

---

## Task 7: Normalize field layers in the UI projection adapter

**Files:**
- Modify: `ui/control-center/src/projection/types.ts`
- Modify: `ui/control-center/src/projection/debugProjectionAdapter.ts`
- Modify: existing adapter tests or create a focused test beside adapter tests.

- [ ] **Step 1: Write RED adapter test**

Add a test with wire JSON:

```ts
it('normalizes configured scalar field layers with grid cells', () => {
  const state = normalizeDebugProjectionBundle({
    projection_kind: 'DebugProjectionBundle',
    run_id: 'run-1',
    tick: 4,
    visual_world: {
      projection_kind: 'VisualWorldProjection',
      completeness: { state: 'bounded', missing_fields: [], reason: null },
      cells: [],
      joints: [],
      organisms: [],
      resource_layers: [],
      fields: [
        {
          field_id: 'temperature',
          value: 25,
          source_metric: {
            field_id: 'temperature',
            source_owner: 'CoreCommittedSnapshot',
            source_path: 'CommittedSnapshot.scalar_field_layers.temperature'
          }
        }
      ],
      field_layers: [
        {
          field_id: 'temperature',
          width: 2,
          height: 2,
          summary_value: 25,
          cells: [{ x: 1, y: 0, value: 30 }],
          completeness: { state: 'bounded', missing_fields: [], reason: null }
        }
      ],
      source_metrics: []
    },
    coverage: emptyCoverageWire(),
    warnings: emptyWarningsWire(),
    classifications: emptyClassificationsWire(),
    balance_findings: emptyBalanceFindingsWire()
  });

  expect(state.status).toBe('available');
  if (state.status !== 'available') return;
  expect(state.visualWorld.fieldLayers).toEqual([
    {
      fieldId: 'temperature',
      width: 2,
      height: 2,
      summaryValue: 25,
      cells: [{ x: 1, y: 0, value: 30 }],
      completeness: { state: 'bounded', missingFields: [], reason: null }
    }
  ]);
});
```

- [ ] **Step 2: Run RED**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/projection/debugProjectionAdapter.test.ts
```

Expected: FAIL because `fieldLayers` is not normalized.

- [ ] **Step 3: Add UI types**

In `types.ts`:

```ts
export interface DebugFieldCell {
  x: number;
  y: number;
  value: number;
}

export interface DebugFieldLayer {
  fieldId: string;
  width: number;
  height: number;
  summaryValue: number;
  cells: DebugFieldCell[];
  completeness: DebugProjectionCompleteness;
}
```

Extend `DebugVisualWorldProjection`:

```ts
fieldLayers: DebugFieldLayer[];
```

- [ ] **Step 4: Normalize wire payload**

In `debugProjectionAdapter.ts`, map `field_layers` to `fieldLayers`:

```ts
fieldLayers: Array.isArray(visual.field_layers)
  ? visual.field_layers.map((layer) => ({
      fieldId: String(layer.field_id),
      width: Number(layer.width),
      height: Number(layer.height),
      summaryValue: Number(layer.summary_value),
      cells: Array.isArray(layer.cells)
        ? layer.cells.map((cell) => ({
            x: Number(cell.x),
            y: Number(cell.y),
            value: Number(cell.value)
          }))
        : [],
      completeness: normalizeCompleteness(layer.completeness)
    }))
  : [],
```

- [ ] **Step 5: Run GREEN**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/projection/debugProjectionAdapter.test.ts
```

Expected: PASS.

---

## Task 8: Show configured Fields in Layers & Filters

**Files:**
- Modify: `ui/control-center/src/components/LayerPanel.test.tsx`
- Modify: `ui/control-center/src/components/LayerPanel.tsx`

- [ ] **Step 1: Write/adjust RED test**

Use a debug projection fixture with `visualWorld.fields` or `visualWorld.fieldLayers` containing `temperature` and `light`.

```ts
expect(screen.getByLabelText('Toggle field layer temperature')).toBeInTheDocument();
expect(screen.getByText('temperature')).toBeInTheDocument();
expect(screen.getByLabelText('Toggle field layer light')).toBeInTheDocument();
expect(screen.queryByText('No config')).not.toBeInTheDocument();
```

- [ ] **Step 2: Run RED**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/components/LayerPanel.test.tsx
```

Expected: FAIL if `LayerPanel` still treats configured fields as unavailable or only uses summary `heat/waste`.

- [ ] **Step 3: Implement minimal rendering rule**

Use configured field layer IDs as the primary source:

```ts
const configuredFieldRows =
  state.debugProjections.status === 'available'
    ? state.debugProjections.visualWorld.fieldLayers.map((layer) => ({
        id: layer.fieldId,
        value: layer.summaryValue
      }))
    : [];
```

Fallback:

```ts
const hasConfiguredFields = configuredFieldRows.length > 0;
```

Render:

```tsx
{hasConfiguredFields ? (
  configuredFieldRows.map((field) => (
    <LayerToggleRow
      key={field.id}
      label={field.id}
      ariaLabel={`Toggle field layer ${field.id}`}
      checked={!state.disabledFieldLayers.includes(field.id)}
      onChange={() => state.toggleFieldLayer(field.id)}
      color={colorForField(field.id)}
    />
  ))
) : (
  <UnavailableRow label="No config" />
)}
```

- [ ] **Step 4: Run GREEN**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/components/LayerPanel.test.tsx
```

Expected: PASS.

---

## Task 9: Add explicit world tile hit targets and selected tile state

**Files:**
- Modify: `ui/control-center/src/components/WorldViewer.test.tsx`
- Modify: `ui/control-center/src/components/WorldViewer.tsx`
- Modify: `ui/control-center/src/app/selectionModel.ts` only if a stable helper is useful.

- [ ] **Step 1: Write RED test for visible tile target**

Add:

```ts
it('exposes selectable World tile targets at World level', async () => {
  const onSelectTarget = vi.fn();

  render(
    <WorldViewer
      frame={{
        ...ui1aFixture.frame,
        resources: [
          [{ organic: 1, mineral: 0, energy: 0 }, { organic: 2, mineral: 0, energy: 0 }],
          [{ organic: 3, mineral: 0, energy: 0 }, { organic: 4, mineral: 0, energy: 0 }]
        ]
      }}
      selectedCellId={null}
      activeLevel="world"
      onSelectCell={vi.fn()}
      onSelectTarget={onSelectTarget}
    />
  );

  await userEvent.click(screen.getByLabelText('Select world tile 1,0'));

  expect(onSelectTarget).toHaveBeenCalledWith(expect.objectContaining({
    kind: 'world-block',
    blockX: 1,
    blockY: 0
  }));
});
```

- [ ] **Step 2: Run RED**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/components/WorldViewer.test.tsx
```

Expected: FAIL because no explicit `Select world tile x,y` hit targets exist.

- [ ] **Step 3: Implement bounded tile hit targets**

When `activeLevel === 'world'`, render one button per resource grid tile:

```tsx
{activeLevel === 'world'
  ? buildWorldTileTargets(frame, viewport, camera).map((tile) => (
      <button
        key={`${tile.blockX}:${tile.blockY}`}
        type="button"
        className="world-tile-hotspot"
        style={tile.style}
        aria-label={`Select world tile ${tile.blockX},${tile.blockY}`}
        onClick={(event) => {
          event.stopPropagation();
          onSelectTarget?.(createWorldBlockSelection({
            runId: frame.runId,
            tick: frame.tick,
            blockX: tile.blockX,
            blockY: tile.blockY,
            bounds: tile.bounds,
            completeness: tile.completeness
          }));
        }}
      />
    ))
  : null}
```

Use resource grid dimensions for tile count. If no resource grid exists, render no tile targets and keep empty-map click fallback.

- [ ] **Step 4: Run GREEN**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/components/WorldViewer.test.tsx
```

Expected: PASS.

---

## Task 10: Build a world block inspection view model

**Files:**
- Create: `ui/control-center/src/app/worldBlockInspection.ts`
- Create: `ui/control-center/src/app/worldBlockInspection.test.ts`

- [ ] **Step 1: Write RED tests**

Create:

```ts
import { describe, expect, it } from 'vitest';
import { buildWorldBlockInspection } from './worldBlockInspection';

describe('buildWorldBlockInspection', () => {
  it('collects all resource and field values for selected world block', () => {
    const inspection = buildWorldBlockInspection({
      selection: {
        kind: 'world-block',
        runId: 'run-1',
        tick: 5,
        blockX: 1,
        blockY: 0,
        bounds: { x: 10, y: 0, width: 10, height: 10 },
        completeness: 'bounded'
      },
      debugProjections: {
        status: 'available',
        runId: 'run-1',
        tick: 5,
        visualWorld: {
          projectionKind: 'VisualWorldProjection',
          completeness: { state: 'bounded', missingFields: [], reason: null },
          cells: [],
          resourceLayers: [
            {
              layerIndex: 0,
              resourceTypeId: 0,
              resourceId: 'amino_acid',
              width: 2,
              height: 2,
              totalAmount: 10,
              cells: [{ x: 1, y: 0, amount: 3 }],
              completeness: { state: 'bounded', missingFields: [], reason: null }
            }
          ],
          fields: [],
          fieldLayers: [
            {
              fieldId: 'temperature',
              width: 2,
              height: 2,
              summaryValue: 25,
              cells: [{ x: 1, y: 0, value: 30 }],
              completeness: { state: 'bounded', missingFields: [], reason: null }
            }
          ],
          sourceMetrics: []
        },
        coverage: { projectionKind: 'CoverageProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, mechanisms: [] },
        warnings: { projectionKind: 'WarningProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, warnings: [] },
        classifications: { projectionKind: 'ClassificationProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, classifications: [] },
        balanceFindings: { projectionKind: 'BalanceFindingProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, findings: [] }
      }
    });

    expect(inspection).toMatchObject({
      blockX: 1,
      blockY: 0,
      resources: [{ id: 'amino_acid', amount: 3 }],
      fields: [{ id: 'temperature', value: 30 }]
    });
  });
});
```

- [ ] **Step 2: Run RED**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/app/worldBlockInspection.test.ts
```

Expected: FAIL because `worldBlockInspection.ts` does not exist.

- [ ] **Step 3: Implement view model**

Create `worldBlockInspection.ts`:

```ts
import type { DebugProjectionState } from '../projection/types';
import type { WorldBlockSelection } from './selectionModel';

export interface WorldBlockInspection {
  blockX: number;
  blockY: number;
  bounds: WorldBlockSelection['bounds'];
  completeness: WorldBlockSelection['completeness'];
  resources: Array<{ id: string; amount: number }>;
  fields: Array<{ id: string; value: number }>;
  source: string;
}

export function buildWorldBlockInspection(args: {
  selection: WorldBlockSelection;
  debugProjections: DebugProjectionState;
}): WorldBlockInspection | null {
  const { selection, debugProjections } = args;
  if (debugProjections.status !== 'available') {
    return null;
  }

  const resources = debugProjections.visualWorld.resourceLayers.map((layer) => ({
    id: layer.resourceId,
    amount: layer.cells.find((cell) => cell.x === selection.blockX && cell.y === selection.blockY)?.amount ?? 0
  }));

  const fields = debugProjections.visualWorld.fieldLayers.map((layer) => ({
    id: layer.fieldId,
    value: layer.cells.find((cell) => cell.x === selection.blockX && cell.y === selection.blockY)?.value ?? 0
  }));

  return {
    blockX: selection.blockX,
    blockY: selection.blockY,
    bounds: selection.bounds,
    completeness: selection.completeness,
    resources,
    fields,
    source: 'VisualWorldProjection'
  };
}
```

- [ ] **Step 4: Run GREEN**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/app/worldBlockInspection.test.ts
```

Expected: PASS.

---

## Task 11: Render WorldBlockInspector in InspectorPanel

**Files:**
- Create: `ui/control-center/src/components/WorldBlockInspector.tsx`
- Create: `ui/control-center/src/components/WorldBlockInspector.test.tsx`
- Modify: `ui/control-center/src/components/InspectorPanel.tsx`
- Modify: `ui/control-center/src/components/AppShell.tsx`

- [ ] **Step 1: Write RED component test**

Create:

```tsx
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import type { WorldBlockInspection } from '../app/worldBlockInspection';
import { WorldBlockInspector } from './WorldBlockInspector';

describe('WorldBlockInspector', () => {
  it('renders selected world tile resources and fields', () => {
    const inspection: WorldBlockInspection = {
      blockX: 1,
      blockY: 0,
      bounds: { x: 10, y: 0, width: 10, height: 10 },
      completeness: 'bounded',
      source: 'VisualWorldProjection',
      resources: [
        { id: 'amino_acid', amount: 3 },
        { id: 'nucleotide_precursor', amount: 1.5 }
      ],
      fields: [
        { id: 'temperature', value: 30 },
        { id: 'light', value: 0.75 }
      ]
    };

    render(<WorldBlockInspector inspection={inspection} />);

    expect(screen.getByRole('heading', { name: 'World Tile 1,0' })).toBeInTheDocument();
    expect(screen.getByText('amino_acid')).toBeInTheDocument();
    expect(screen.getByText('3.00')).toBeInTheDocument();
    expect(screen.getByText('temperature')).toBeInTheDocument();
    expect(screen.getByText('30.00')).toBeInTheDocument();
  });
});
```

- [ ] **Step 2: Run RED**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/components/WorldBlockInspector.test.tsx
```

Expected: FAIL because component does not exist.

- [ ] **Step 3: Implement component**

Create:

```tsx
import type { WorldBlockInspection } from '../app/worldBlockInspection';

export function WorldBlockInspector({ inspection }: { inspection: WorldBlockInspection }) {
  return (
    <section className="world-block-inspector" aria-label="World tile inspector">
      <h3>{`World Tile ${inspection.blockX},${inspection.blockY}`}</h3>
      <div className="metric-list">
        <div><span>Bounds</span><strong>{`${inspection.bounds.x},${inspection.bounds.y} ${inspection.bounds.width}x${inspection.bounds.height}`}</strong></div>
        <div><span>Completeness</span><strong>{inspection.completeness}</strong></div>
      </div>
      <section className="inspector-data-section">
        <h3>{`Resources (${inspection.resources.length})`}</h3>
        <div className="inspector-data-grid">
          {inspection.resources.map((resource) => (
            <div key={resource.id}>
              <span>{resource.id}</span>
              <strong>{resource.amount.toFixed(2)}</strong>
            </div>
          ))}
        </div>
      </section>
      <section className="inspector-data-section">
        <h3>{`Fields (${inspection.fields.length})`}</h3>
        <div className="inspector-data-grid">
          {inspection.fields.map((field) => (
            <div key={field.id}>
              <span>{field.id}</span>
              <strong>{field.value.toFixed(2)}</strong>
            </div>
          ))}
        </div>
      </section>
    </section>
  );
}
```

- [ ] **Step 4: Route InspectorPanel by selection kind**

Change `InspectorPanel` props to accept:

```ts
currentSelection: MonitorSelection;
debugProjections: DebugProjectionState;
```

Build inspection:

```tsx
const worldInspection =
  currentSelection.kind === 'world-block'
    ? buildWorldBlockInspection({ selection: currentSelection, debugProjections })
    : null;
```

Render order:

```tsx
{worldInspection ? (
  <WorldBlockInspector inspection={worldInspection} />
) : (
  <CellInspector selectedCell={selectedCell} selectionNotice={selectionNotice} />
)}
```

Update `AppShell`:

```tsx
<InspectorPanel
  selectedCell={state.selectedCell}
  currentSelection={state.currentSelection}
  debugProjections={state.debugProjections}
  selectionNotice={state.selectionNotice}
  displayedTick={state.frame?.tick || 0}
  onSelectCell={(cellId) => store.getState().selectCell(cellId)}
/>
```

- [ ] **Step 5: Run GREEN component tests**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/components/WorldBlockInspector.test.tsx src/components/WorldViewer.test.tsx
```

Expected: PASS.

---

## Task 12: Render selected world tile highlight

**Files:**
- Modify: `ui/control-center/src/viewer/worldRenderer.test.ts`
- Modify: `ui/control-center/src/viewer/worldRenderer.ts`
- Modify: `ui/control-center/src/components/WorldViewer.tsx` if selected tile is passed into renderer.

- [ ] **Step 1: Write RED renderer test**

Add a test that renders with selected tile metadata and asserts a grid/highlight drawing command exists. Use the existing renderer mock inspection pattern.

Expected behavior:

```ts
renderer.renderFrame(frame, null, camera, [0], visualEffects, {
  kind: 'world-block',
  blockX: 1,
  blockY: 0,
  bounds: { x: 10, y: 0, width: 10, height: 10 },
  completeness: 'bounded',
  runId: 'run-1',
  tick: 1
});
expect(graphics.rect).toHaveBeenCalledWith(10, 0, 10, 10);
expect(graphics.stroke).toHaveBeenCalledWith(expect.objectContaining({
  color: expect.any(Number)
}));
```

- [ ] **Step 2: Run RED**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/viewer/worldRenderer.test.ts
```

Expected: FAIL because selected world-block is not passed/rendered.

- [ ] **Step 3: Implement minimal highlight**

Extend renderer argument with optional `currentSelection`. If `currentSelection.kind === 'world-block'`, draw a high-contrast rectangle using `selection.bounds`. The highlight must render even if `World Grid` visual effect is off.

- [ ] **Step 4: Run GREEN**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/viewer/worldRenderer.test.ts src/components/WorldViewer.test.tsx
```

Expected: PASS.

---

## Task 13: End-to-end UI integration test for World level Inspector

**Files:**
- Modify: `ui/control-center/src/App.test.tsx` or create focused integration test if existing setup is too broad.

- [ ] **Step 1: Write RED integration test**

Test flow:

```ts
it('shows selected World tile resources and fields in Inspector', async () => {
  mockRunner.apiInstance.getRunStatus.mockResolvedValue(runningStatus);
  mockRunner.apiInstance.getLatestDebugProjections.mockResolvedValue(debugProjectionWithResourcesAndFieldLayers());

  render(<App />);

  await userEvent.click(screen.getByRole('button', { name: 'World level' }));
  await userEvent.click(await screen.findByLabelText('Select world tile 1,0'));

  expect(screen.getByRole('heading', { name: 'World Tile 1,0' })).toBeInTheDocument();
  expect(screen.getByText('amino_acid')).toBeInTheDocument();
  expect(screen.getByText('temperature')).toBeInTheDocument();
});
```

- [ ] **Step 2: Run RED**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/App.test.tsx
```

Expected: FAIL until all UI integration wiring is complete.

- [ ] **Step 3: Implement remaining prop wiring only**

Do not add new behavior. Wire the already-tested pieces together:

- pass `currentSelection` to `WorldViewer` and renderer;
- pass `currentSelection` and `debugProjections` to `InspectorPanel`;
- ensure `LayerPanel` field toggles use `disabledFieldLayers`.

- [ ] **Step 4: Run GREEN**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/App.test.tsx
```

Expected: PASS.

---

## Task 14: Final verification

**Files:**
- No code changes unless verification finds a failed contract.

- [ ] **Step 1: Format Rust**

Run:

```powershell
cargo fmt --check
```

Expected: PASS.

- [ ] **Step 2: Run focused Rust tests**

Run:

```powershell
cargo test --test observer_projection_payloads
cargo test --test phase3h_local_fields committed_snapshot_contains_configured_scalar_field_layers
cargo test --test runner_http_projections latest_projections_expose_configured_resource_and_field_grids
```

Expected: PASS.

- [ ] **Step 3: Run focused UI tests**

Run:

```powershell
cd ui/control-center
npm.cmd test -- --run src/projection/debugProjectionAdapter.test.ts src/components/LayerPanel.test.tsx src/app/worldBlockInspection.test.ts src/components/WorldBlockInspector.test.tsx src/components/WorldViewer.test.tsx src/viewer/worldRenderer.test.ts src/App.test.tsx
```

Expected: PASS. Existing React `act(...)` warnings should be treated as known debt unless a new failing assertion appears.

- [ ] **Step 4: Run UI build**

Run:

```powershell
cd ui/control-center
npm.cmd run build
```

Expected: PASS. Existing Vite chunk-size warning is non-blocking.

- [ ] **Step 5: Check worktree**

Run:

```powershell
git diff --check
git status --short
```

Expected:

- `git diff --check` has no whitespace errors.
- `git status --short` contains only files intentionally changed by this slice.

---

## Risks and Decisions

### Decision 1: Field summary value

Use deterministic average of scalar field grid values for `summary_value`.

Pros:

- stable and cheap;
- useful for layer row summaries;
- does not require viewport-specific sampling.

Cons:

- hides local peaks;
- not enough for detailed science/debug by itself;
- may need min/max later.

Confidence: medium-high.

### Decision 2: World tile size

Use resource/field grid cells as the first world tile granularity.

Pros:

- directly aligns Inspector values with projected resources/fields;
- simple and testable;
- no separate spatial index required.

Cons:

- tile density may be high for large worlds;
- sampled projections need explicit completeness;
- not equivalent to arbitrary pixel-level world coordinates.

Confidence: high for current debug/control UI.

### Decision 3: `heat` and `waste`

Keep `heat` and `waste` as compatibility summary fields for now, but do not treat them as configured Field layers unless they appear in `field_layers`.

Pros:

- avoids breaking existing UI/debug tests;
- preserves old summary visibility;
- cleanly separates configured fields from legacy summaries.

Cons:

- two field-like concepts remain temporarily;
- UI must avoid showing `heat/waste` as configured rows when `field_layers` is empty;
- later cleanup likely needed.

Confidence: high.

---

## Execution Notes

- Implement in order. Do not start UI Inspector work until Rust projection contract is green or a test fixture explicitly represents the intended wire payload.
- Commit after each stable task group if executing interactively:
  - Rust snapshot/projection contract;
  - HTTP projection contract;
  - UI adapter/layers;
  - world tile selection/Inspector;
  - final verification/report.
- If runtime field storage does not expose read-only scalar layers, stop and write the minimal accessor at the storage boundary. Do not duplicate field data outside Core state.
- If `canonical_test_world` lacks a configured field at runtime, fix config/bootstrap mapping before UI work.

