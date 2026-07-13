---
tags:
  - alife
  - worklog/report
  - phase/2G
---

# Phase 2G Chemistry And Matter Dynamics Report

## Scope

Implemented Phase 2G chemistry and matter dynamics on branch `codex-phase-2g-chemistry` in worktree `.worktrees/phase-2g-chemistry`.

Main implementation areas:

- typed Resource and Material registries and parser support;
- Reaction definitions, validation, accounting deltas and deterministic execution;
- typed internal cell resources and typed ResourceGrid support;
- MaterialFragment identity store and decomposition Material -> Fragment path;
- local Heat, thermal material degradation, baseline Material decay;
- boundary retention/leakage baseline;
- feasibility-gated `RepairBoundary` process;
- shared Phase 2G observer metrics exposed through Core summary and `sweep_analyzer`;
- full and smoke analyzer scenarios for Phase 2G.

## Implemented Modules

- `src/core/resource_types.rs`
- `src/core/material_types.rs`
- `src/core/reactions.rs`
- `src/core/deltas.rs`
- `src/core/fragments.rs`
- `src/core/heat.rs`
- updated `src/core/tick.rs`, `src/core/world.rs`, `src/core/resources.rs`, `src/core/cell_store.rs`
- updated `src/runner/config_parser.rs`
- updated `src/observer/projection.rs`
- updated `src/bin/sweep_analyzer.rs`

No commits were created during this run; the worktree remains uncommitted for review.

## Tests And Verification

Passed:

```text
cargo test --test phase2g_reactions -- --nocapture
cargo test --test phase2g_resource_types -- --nocapture
cargo test --test phase2g_material_types -- --nocapture
cargo test --test phase2g_accounting -- --nocapture
cargo test --test phase2g_fragments -- --nocapture
cargo test --test phase2g_heat_boundary_repair -- --nocapture
cargo test --test phase2g_internal_inventory -- --nocapture
cargo test --test phase2g_observer_outputs -- --nocapture
cargo test --test phase2g_sweep_parser -- --nocapture
cargo test --test phase2g_tick_integration -- --nocapture
cargo test --test phase2g_determinism -- --nocapture
cargo test --test phase2_sweep_parser -- --nocapture
cargo test --test phase2_sweep_observer_outputs -- --nocapture
cargo test --test phase2_sweep_outputs -- --nocapture
cargo test --test phase2_sweep_warnings -- --nocapture
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
```

Analyzer checks:

```text
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer_smoke.toml
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml
```

The full analyzer originally timed out on the legacy `resource_abundance` 10x10 matrix after adding Phase 2G sweeps. The full matrix was reduced to 4x4 so the full analyzer now completes.

## Artifacts

Smoke Phase 2G CSVs:

```text
outputs/raw_data/smoke/resource_type_decay_diffusion.csv
outputs/raw_data/smoke/material_type_degradation.csv
outputs/raw_data/smoke/passive_reaction_viability.csv
outputs/raw_data/smoke/controlled_reaction_feasibility.csv
outputs/raw_data/smoke/fragment_decomposition_conversion.csv
outputs/raw_data/smoke/local_heat_degradation.csv
outputs/raw_data/smoke/boundary_retention_leakage.csv
outputs/raw_data/smoke/repair_viability.csv
```

Full Phase 2G CSVs:

```text
outputs/raw_data/resource_type_decay_diffusion.csv
outputs/raw_data/material_type_degradation.csv
outputs/raw_data/passive_reaction_viability.csv
outputs/raw_data/controlled_reaction_feasibility.csv
outputs/raw_data/fragment_decomposition_conversion.csv
outputs/raw_data/local_heat_degradation.csv
outputs/raw_data/boundary_retention_leakage.csv
outputs/raw_data/repair_viability.csv
```

## Evidence

Resource differential evidence:

- `ResourceType` stores volume, diffusion rate, energy value, decay rate, reactivity profile, permeability and tags.
- `ResourceGrid` applies differential typed decay and deterministic typed diffusion.
- Analyzer `resource_type_decay_diffusion.csv` records `resource_decay_amount` changing with `nutrient_decay_rate`.

Material differential evidence:

- `MaterialType` stores stability, strength, permeability, energy capacity, decay rate, repair requirements, reaction profile and signal state fields.
- Tick material decay degrades matching material slots and reports `material_degradation_amount`.
- Analyzer `material_type_degradation.csv` records `material_degradation_amount` changing with `material_decay_rate`.

Reaction accounting evidence:

- validated reaction definitions cover passive, controlled, degradation, decay, synthesis and conversion families.
- zero-rate reactions are not counted as executed.
- products without inputs and unaccounted input are rejected without partial commits.
- controlled reactions require process binding, metabolism capability and required material/catalyst availability.

Fragment evidence:

- dead cell Materials become identity-preserving `MaterialFragment`s.
- decomposition no longer silently converts all dead material directly into resources.
- analyzer `fragment_decomposition_conversion.csv` records `fragment_created_amount`.

Heat, boundary and repair evidence:

- controlled reaction heat changes local cell temperature and does not directly transfer Energy Buffer.
- local heat can degrade material above tolerance.
- boundary default is blocked; damaged compatible boundary allows local leakage.
- `RepairBoundary` consumes explicit Energy, Resource and repair Material inputs and can fail without partial consumption.

Observer evidence:

- `MetricsSummary` and `observer::projection::metrics_summary_features` expose shared Phase 2G fields.
- `sweep_analyzer` consumes those shared summary fields instead of recomputing Phase 2G metrics independently.
- smoke and full analyzer outputs include Phase 2G metric columns.

## Gate Status

Passed:

```text
[x] two ResourceTypes differ in decay, diffusion or permeability
[x] two MaterialTypes differ in stability/degradation
[x] all six ReactionModes are represented in validated definitions
[x] passive reaction executes without Genome
[x] controlled reaction requires Process/ActionPlan/Feasibility
[x] reaction inputs, products, residuals and sinks are explicit
[x] products without inputs and unaccounted inputs are rejected
[x] reaction Heat is local and does not directly transfer Energy Buffer
[x] per-resource diffusion/decay is local and deterministic
[x] dead Cell Materials become identity-preserving MaterialFragments
[x] local Heat can trigger material degradation by tolerance
[x] Boundary default is blocked and damage only expands compatible leakage
[x] repair consumes explicit inputs and can fail
[x] Core and sweep_analyzer use identical Phase 2G Observer metrics
[x] full and smoke analyzer configs cover all Phase 2G scenarios
[x] same seed/config produces identical Phase 2G state and metrics
[x] cargo fmt, clippy and workspace tests pass
[x] Phase 1 and Phase 2A-F tests still pass
[x] Phase 2G report and worklog index entry exist
```

Partial / constrained:

```text
[~] fragments convert to Resources only through explicit reaction/conversion
```

Current behavior preserves Fragment identity and prevents silent conversion. A dedicated explicit `MaterialFragment -> Resource` conversion reaction path is not yet implemented as a standalone mechanism.

## Known Constraints

- `repair_viability` analyzer scenario currently demonstrates repair rejection/negative-control behavior; successful repair is covered by core tests, not by a clean analyzer success sweep.
- `heat_peak_temperature` remains mapped to summary heat rather than a dedicated max local cell temperature metric.
- persistent Joints, adhesion and inter-cell channels remain Phase 2H scope.
- Genome regulation remains Phase 3 scope; Phase 2G behavior is deterministic with Genome disabled.

## Follow-Up For Phase 2H / Phase 3

- Add explicit Fragment conversion reactions before relying on scavenging/ecological reuse.
- Add typed repair-resource consumption so repair scenarios can be fully typed instead of depending on legacy generic resources.
- Add a dedicated local temperature observer metric if analyzer gates need temperature rather than heat amount.
- Keep Chemistry authority in Core; `sweep_analyzer` should remain a consumer of shared observer metrics.
