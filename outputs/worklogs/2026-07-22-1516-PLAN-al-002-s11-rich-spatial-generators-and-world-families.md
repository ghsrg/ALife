---
tags:
  - alife
  - worklog/plan
  - delivery/tdd
  - bootstrap
  - rust
plan_id: AL-002-S11
status: planned
---

# AL-002-S11 Rich Spatial Generators And World Families TDD Plan

Plan ID: `AL-002-S11`
Status at planning: `planned`
Legacy refs: `Bootstrap-2`, `Bootstrap-3`

## Goal

Implement deterministic rich Bootstrap generators and world-family presets that convert an immutable resolved `ScenarioDocument` into concrete Tick 0 `PreparedWorld` data, with source-backed manifest evidence and no unresolved generator instructions reaching Core.

## Architecture

Extend Bootstrap as an application-layer generation boundary, not as a Core runtime mechanic. `ScenarioDocument` may carry typed Bootstrap generation input, Bootstrap resolves it into concrete `RuntimeConfig`/prepared resource initialization plus manifest summaries and warnings, and Core receives only prepared Tick 0 state.

Resource richness should integrate with the existing `ResourceGrid` path by preparing explicit spatial layer quantities. Field richness is bounded in this slice: Bootstrap may parse and summarize initial field bands/gradients in the manifest, but must not pretend Core has spatial Field runtime support until a separate Core field model exists.

## Source Of Truth

- `docs/PRINCIPLES.md`
- `docs/GLOSSARY.md`
- `docs/runner/bootstrap.md`
- `docs/implementation/implementation-plan-bootstrap.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `src/bootstrap/mod.rs`
- `src/bootstrap/prepared.rs`
- `src/bootstrap/manifest.rs`
- `src/bootstrap/seed_domains.rs`
- `src/bootstrap/resource_layers.rs`
- `src/bootstrap/field_layers.rs`
- `src/bootstrap/cell_placement.rs`
- `src/bootstrap/starter_state.rs`
- `src/bootstrap/viability.rs`
- `src/runner/scenario_doc.rs`
- `src/runner/config_parser.rs`
- `src/core/config.rs`
- `src/core/resources.rs`
- `src/core/world.rs`
- existing `tests/bootstrap_*`

Worklogs used only as evidence:

- `outputs/worklogs/2026-07-22-1444-REPORT-al-007-s10-debug-visualization-mode-exact-layers.md`

## Deterministic Lint Result

`LINT_RESULT`: PASS for planning.

- `AL-002-S11` exists in `docs/delivery/roadmap.md`.
- `AL-002-S11` is first in `Candidate Next Work` and `docs/delivery/status.md`.
- Current dependencies are closed in the roadmap: `AL-002-S10`, `AL-002-S09`.
- Existing acceptance row `AL-002-S11-AC01` is planning-level and should be expanded by this plan.

## Design Choice

Recommended approach: implement typed Bootstrap generator specs and resolve them into concrete prepared data.

Pros:

- Preserves the canonical `ScenarioDocument -> Bootstrap -> PreparedWorld` contract.
- Gives UI/Observer future source-backed spatial data instead of fake colors.
- Keeps Core Tick behavior unchanged.

Cons:

- Requires parser/config/model changes before visible UI benefit is complete.
- Resource spatial grids need a small Core config extension so `WorldState` can initialize non-uniform layers.
- Spatial Field runtime remains partial until a later Core field slice.

Confidence: `medium-high`.

## BDD Agent Scenario Cards

### `AL-002-S11-AC01` Typed Rich Generator Spec

Source links: `docs/runner/bootstrap.md`, `docs/implementation/implementation-plan-bootstrap.md`, `src/runner/scenario_doc.rs`, `src/runner/config_parser.rs`.

Intent: Scenario input can describe deterministic rich Bootstrap generators without leaking unresolved generator instructions into Core.

Given a resolved scenario contains a `[bootstrap]` rich initialization section,
When `ScenarioDocument::resolve` parses it,
Then the document exposes typed Bootstrap generator input with generator ids, versions, seed-domain labels, bounds, and world-family id, while `RuntimeConfig` remains the current compatibility bridge until Bootstrap prepares concrete Tick 0 state.

TDD obligation: parser/document tests first; no production parsing change before a failing test.

Evidence: `AL-002-S11-EV01`, `AL-002-S11-EV02`.

### `AL-002-S11-AC02` Deterministic Seed Isolation

Source links: `docs/runner/bootstrap.md`, `src/bootstrap/seed_domains.rs`.

Intent: Changing one generator stream must not perturb unrelated generated data.

Given the same root seed and scenario hash,
When resource layers, field layers, cell placement, and starter-state generators run through separate seed domains,
Then repeated preparation produces identical outputs, and changing one generator spec changes only its manifest/source hash contribution and prepared domain.

TDD obligation: seed-domain and preparation determinism tests first.

Evidence: `AL-002-S11-EV03`, `AL-002-S11-EV04`.

### `AL-002-S11-AC03` Spatial Resource Layers

Source links: `docs/runner/bootstrap.md`, `src/core/resources.rs`, `src/core/world.rs`, `src/bootstrap/resource_layers.rs`.

Intent: Rich Resource layer generators become concrete Core-readable Tick 0 resource maps.

Given a scenario defines layered resource patches or gradients with finite bounds,
When Bootstrap prepares the world,
Then `PreparedWorld` contains concrete resource layer quantities that `WorldState::from_config` loads into `ResourceGrid`, and the manifest records total/min/max/cell-count/source generator version.

TDD obligation: failing Bootstrap/Core integration tests before changing `RuntimeConfig` or `ResourceGrid` initialization.

Evidence: `AL-002-S11-EV05`, `AL-002-S11-EV06`.

### `AL-002-S11-AC04` World Families And Starting Niches

Source links: `docs/runner/bootstrap.md`, `docs/implementation/implementation-plan-bootstrap.md`, `src/bootstrap/cell_placement.rs`, `src/bootstrap/starter_state.rs`.

Intent: World-family presets create deterministic initial conditions without scripted survival behavior.

Given a scenario selects a bounded world family such as `patchy_temperate_v1`,
When Bootstrap prepares cells and starter state,
Then generated cells are within bounds, respect radius/spacing/capacity, may use deterministic near-resource placement, and produce manifest family/niche summaries and warnings.

TDD obligation: placement and starter-state tests first; no hardcoded organism/species shortcuts.

Evidence: `AL-002-S11-EV07`, `AL-002-S11-EV08`.

### `AL-002-S11-AC05` Manifest And Warning Completeness

Source links: `docs/runner/bootstrap.md`, `src/bootstrap/manifest.rs`, `src/bootstrap/viability.rs`.

Intent: Human and downstream tools can explain what Bootstrap generated.

Given rich generators are used,
When Bootstrap returns a `BootstrapManifest`,
Then it records generator versions, world family, resource totals/ranges, field ranges/status, cell placement summary, prepared-state hash, and explicit warnings for bounded or not-core-integrated field data.

TDD obligation: manifest tests first; warnings must use stable codes.

Evidence: `AL-002-S11-EV09`, `AL-002-S11-EV10`.

### `AL-002-S11-AC06` Runner/Core Smoke Without Tick Authority Drift

Source links: `docs/runner/bootstrap.md`, `src/runner/engine.rs`, `src/core/world.rs`, `tests/runner_bootstrap.rs`.

Intent: Runner can start a rich prepared world, but Bootstrap still executes no Tick and does not become behavior authority.

Given a rich world-family scenario,
When Runner starts the scenario through the shared Bootstrap path,
Then Core starts at Tick 0 from concrete prepared data, short smoke execution is deterministic, and Bootstrap errors remain typed before any partial active world starts.

TDD obligation: runner/bootstrap smoke tests after Bootstrap unit coverage is green.

Evidence: `AL-002-S11-EV11`, `AL-002-S11-EV12`.

## Proposed File Plan

Rust source:

- Modify `src/runner/config_parser.rs` to parse a typed optional `[bootstrap]` section and world-family/generator settings.
- Modify `src/runner/scenario_doc.rs` to retain typed Bootstrap generation input beside `RuntimeConfig`.
- Add `src/bootstrap/generator_spec.rs` for typed generator ids, versions, bounded parameter structs, and validation errors.
- Add `src/bootstrap/world_families.rs` for deterministic preset expansion into generator specs.
- Extend `src/bootstrap/resource_layers.rs` with concrete spatial layer generation helpers that return per-grid quantities plus summaries.
- Extend `src/bootstrap/field_layers.rs` with bounded field summary generation and explicit non-Core integration status.
- Modify `src/bootstrap/prepared.rs` with prepared resource initialization data or a `RuntimeConfig` bridge field as needed.
- Modify `src/bootstrap/manifest.rs` to include world family, generator summaries, spatial resource cell count, and warning/status codes.
- Modify `src/bootstrap/mod.rs` to resolve generator specs into concrete `PreparedWorld` data.
- Modify `src/core/config.rs`, `src/core/resources.rs`, and `src/core/world.rs` only to support explicit prepared resource grid initialization; no Tick behavior changes.

Tests:

- Add `tests/bootstrap_rich_generators.rs`.
- Add `tests/bootstrap_world_families.rs`.
- Add or extend `tests/bootstrap_resource_layers.rs`.
- Add or extend `tests/bootstrap_prepared_world.rs`.
- Add or extend `tests/bootstrap_integration.rs`.
- Add or extend `tests/runner_bootstrap.rs`.

Config:

- Add `config/scenarios/bootstrap/rich_patchy_world.toml` as a bounded human-readable scenario for smoke/UI follow-up.

Delivery docs:

- Update `docs/delivery/status.md` during execution start/closure.
- Update `docs/delivery/roadmap.md`, `docs/delivery/acceptance.md`, `docs/delivery/worklog-ledger.md`, and `outputs/worklogs/index.md` during closure.

## Numbered TDD Tasks

### `AL-002-S11-T01`: RED for `AL-002-S11-AC01`

- [ ] Add a failing parser/document test in `tests/bootstrap_rich_generators.rs`.
- [ ] Test a scenario with `[bootstrap] family = "patchy_temperate_v1"` and resource/field generator entries.
- [ ] Assert `ScenarioDocument` exposes typed Bootstrap generator input and stable generator ids/versions.
- [ ] Run `cargo test --test bootstrap_rich_generators`.
- [ ] Capture expected failure as `AL-002-S11-EV01`.

### `AL-002-S11-T02`: GREEN for `AL-002-S11-AC01`

- [ ] Implement `src/bootstrap/generator_spec.rs`.
- [ ] Extend `RawScenarioConfig` and `ScenarioDocument` with optional typed Bootstrap spec.
- [ ] Validate finite/non-negative generator parameters and reject unknown generator ids with stable errors.
- [ ] Run `cargo test --test bootstrap_rich_generators --test bootstrap_scenario_doc`.
- [ ] Capture pass as `AL-002-S11-EV02`.

### `AL-002-S11-T03`: RED for `AL-002-S11-AC02`

- [ ] Add failing tests proving independent generator seed domains for resources, fields, placement, starter state, and world family expansion.
- [ ] Include a regression where changing a field generator does not change resource layer output.
- [ ] Run `cargo test --test bootstrap_world_families --test bootstrap_seed_domains`.
- [ ] Capture expected failure as `AL-002-S11-EV03`.

### `AL-002-S11-T04`: GREEN for `AL-002-S11-AC02`

- [ ] Add seed-domain labels for individual resource/field generator streams if needed.
- [ ] Implement deterministic generator execution ordering independent of TOML map iteration.
- [ ] Run `cargo test --test bootstrap_world_families --test bootstrap_seed_domains`.
- [ ] Capture pass as `AL-002-S11-EV04`.

### `AL-002-S11-T05`: RED for `AL-002-S11-AC03`

- [ ] Add failing resource layer tests for explicit spatial grid quantities: patch falloff, band/gradient, bounds, total/min/max, and deterministic repetition.
- [ ] Add a Core integration test proving `WorldState::from_config` reads non-uniform prepared resource quantities.
- [ ] Run `cargo test --test bootstrap_resource_layers --test bootstrap_prepared_world`.
- [ ] Capture expected failure as `AL-002-S11-EV05`.

### `AL-002-S11-T06`: GREEN for `AL-002-S11-AC03`

- [ ] Implement concrete spatial resource layer generation returning per-grid quantities.
- [ ] Extend Core config/world initialization minimally so prepared non-uniform resource maps reach `ResourceGrid`.
- [ ] Preserve existing flat `initial_distribution` scenarios without behavior changes.
- [ ] Run `cargo test --test bootstrap_resource_layers --test bootstrap_prepared_world --test phase1_resource_grid`.
- [ ] Capture pass as `AL-002-S11-EV06`.

### `AL-002-S11-T07`: RED for `AL-002-S11-AC04`

- [ ] Add failing world-family tests for `patchy_temperate_v1`: deterministic family expansion, near-resource starter niches, spacing/bounds, and capacity-safe starter cells.
- [ ] Run `cargo test --test bootstrap_world_families --test bootstrap_cell_placement --test bootstrap_starter_state`.
- [ ] Capture expected failure as `AL-002-S11-EV07`.

### `AL-002-S11-T08`: GREEN for `AL-002-S11-AC04`

- [ ] Implement `src/bootstrap/world_families.rs` with bounded preset expansion.
- [ ] Reuse `cell_placement` and `starter_state` helpers; do not add species/organism scripts.
- [ ] Emit explicit warnings when a family requests field richness that is manifest-only in this slice.
- [ ] Run `cargo test --test bootstrap_world_families --test bootstrap_cell_placement --test bootstrap_starter_state`.
- [ ] Capture pass as `AL-002-S11-EV08`.

### `AL-002-S11-T09`: RED for `AL-002-S11-AC05`

- [ ] Add failing manifest tests for generator versions, family id, resource totals/ranges, field status, warnings, and prepared-state hash sensitivity.
- [ ] Run `cargo test --test bootstrap_rich_generators --test bootstrap_prepared_world`.
- [ ] Capture expected failure as `AL-002-S11-EV09`.

### `AL-002-S11-T10`: GREEN for `AL-002-S11-AC05`

- [ ] Extend `BootstrapManifest` with rich generator summaries and stable warning codes.
- [ ] Ensure prepared-state hash includes concrete generated resource maps and relevant generator versions.
- [ ] Run `cargo test --test bootstrap_rich_generators --test bootstrap_prepared_world --test bootstrap_viability`.
- [ ] Capture pass as `AL-002-S11-EV10`.

### `AL-002-S11-T11`: RED for `AL-002-S11-AC06`

- [ ] Add `config/scenarios/bootstrap/rich_patchy_world.toml`.
- [ ] Add failing Runner/Core smoke tests proving Bootstrap prepares the rich scenario, Core starts at Tick 0, and repeated short runs match.
- [ ] Run `cargo test --test bootstrap_integration --test runner_bootstrap`.
- [ ] Capture expected failure as `AL-002-S11-EV11`.

### `AL-002-S11-T12`: GREEN for `AL-002-S11-AC06`

- [ ] Wire Runner shared Bootstrap path to accept the rich scenario without duplicating Bootstrap logic.
- [ ] Keep Bootstrap errors typed and pre-Core-start.
- [ ] Run `cargo test --test bootstrap_integration --test runner_bootstrap`.
- [ ] Capture pass as `AL-002-S11-EV12`.

### `AL-002-S11-T13`: REFACTOR for `AL-002-S11`

- [ ] Refactor generator specs and summaries only while tests stay green.
- [ ] Keep Core changes limited to initial resource grid loading.
- [ ] Run `cargo fmt --check`.
- [ ] Run `cargo test --test bootstrap_rich_generators --test bootstrap_world_families --test bootstrap_resource_layers --test bootstrap_prepared_world --test bootstrap_integration --test runner_bootstrap`.
- [ ] Capture pass as `AL-002-S11-EV13`.

### `AL-002-S11-T14`: Full Verification And Closure Report

- [ ] Run `cargo fmt --check`.
- [ ] Run `cargo test --test bootstrap_rich_generators --test bootstrap_world_families --test bootstrap_resource_layers --test bootstrap_prepared_world --test bootstrap_integration --test runner_bootstrap --test phase1_resource_grid`.
- [ ] Run `cargo test --test runner_http_projections --test observer_projection_payloads` to guard the UI/Observer projection path after richer resource data.
- [ ] Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-002-s11-rich-spatial-generators-and-world-families.md`.
- [ ] Update roadmap/status/acceptance/ledger/index and review `Candidate Next Work`.
- [ ] Capture final evidence as `AL-002-S11-EV14`.

## Verification Commands

```powershell
cargo fmt --check
```

```powershell
cargo test --test bootstrap_rich_generators --test bootstrap_world_families --test bootstrap_resource_layers --test bootstrap_prepared_world --test bootstrap_integration --test runner_bootstrap --test phase1_resource_grid
```

```powershell
cargo test --test runner_http_projections --test observer_projection_payloads
```

## Forbidden Scope

- No Core Tick behavior changes.
- No runtime seasons, catastrophes, weather cycles, or adaptive generation based on run outcomes.
- No UI rendering changes.
- No Observer projection expansion beyond regression guards.
- No hardcoded species, organism, predator/prey, or scripted survival behavior.
- No direct TOML-to-Core startup bypass around `ScenarioDocument -> Bootstrap -> PreparedWorld`.
- No full Field runtime grid unless a separate Core field model slice is approved.

## Open Questions

- `Needs Review`: whether spatial Field grids should become a new Core model in a future `AL-002-S19`/`AL-004` slice or stay Observer/manifest-only until UI needs exact field overlays.
- `Needs Review`: exact TOML names for `[bootstrap]`, `[[bootstrap.resources]]`, and `[[bootstrap.fields]]` should be finalized during `AL-002-S11-T01` RED tests.

## Approval Gate

Reply `OK EXECUTE AL-002-S11` to authorize execution of this TDD plan.

Reply `CHANGE AL-002-S11` with corrections to revise the plan.
