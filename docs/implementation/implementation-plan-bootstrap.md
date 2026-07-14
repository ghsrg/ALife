---
tags:
  - alife
  - implementation
  - bootstrap
  - roadmap
---

# Bootstrap Implementation Plan

> **For agentic workers:** this is the high-level parent plan for the Bootstrap application module. Before implementing any slice, create a dedicated `outputs/worklogs/YYYY-MM-DD-HHMM-PLAN-bootstrap-*.md` TDD worklog with files, tests, steps, and acceptance gates.

## Purpose

Bootstrap is the deterministic application module that prepares complete Tick 0 world state before Runner starts Core.

It is not a random world painter and not a runtime mechanic. It converts an immutable resolved `ScenarioDocument` into a concrete `PreparedWorld`, `BootstrapManifest`, and warnings.

Canonical source:

```text
docs/runner/bootstrap.md
```

## Authority And Boundaries

Bootstrap must follow:

```text
docs/PRINCIPLES.md
docs/runner/runner.md
docs/runner/scenario-resolution.md
docs/runner/bootstrap.md
docs/runner/projections.md
docs/config/INDEX.md
docs/world/INDEX.md
docs/biology/INDEX.md
docs/genetics/INDEX.md
```

Bootstrap must:

- prepare Tick 0 state;
- execute no Tick;
- use only immutable resolved Scenario input;
- derive independent deterministic seed domains;
- generate bounded spatial Resource and Field layers;
- place initial Cells within world/capacity/spacing constraints;
- assign initial Energy, Resource inventory, Material inventory, and Genome state;
- validate cross-state invariants before Core start;
- produce `BootstrapManifest` with counts, ranges, hashes, warnings, and generator versions.

Bootstrap must not:

- define world laws or runtime mechanics;
- depend on CLI, HTTP, UI, or rendering state;
- expose mutable `WorldState`;
- silently invent unsupported resources, materials, genomes, fields, or organisms;
- guarantee survival by scripted behavior.

## Global Flow

```text
ScenarioSource
  -> Scenario Resolution
  -> ScenarioDocument
  -> Bootstrap
  -> PreparedWorld
  -> BootstrapManifest
  -> Runner StartRun
  -> Core TickExecutor
```

## Deterministic Constrained Generation

The first Bootstrap implementation should be deterministic and constrained, not rich procedural worldbuilding.

Allowed in Bootstrap-1:

- `uniform` Resource layer generator;
- `patches` Resource layer generator;
- constant Field layer generator;
- deterministic initial Cell placement:
  - explicit positions;
  - grid placement;
  - near-resource placement with minimum spacing;
- starter material profiles;
- starter Energy and Resource ranges;
- Genome template assignment using existing Phase 3A genome instantiation;
- viability envelope checks.

Out of scope for Bootstrap-1:

- seasons;
- temperature cycles;
- catastrophes;
- terrain/ecology storytelling;
- adaptive world generation based on runtime outcomes;
- non-deterministic random sampling;
- multi-run experiment optimization.

Future Bootstrap phases may add richer world maps, seasonal resource variation, temperature bands, disasters, geological/material structures, and scenario families. Those must remain generator modules behind the same `ScenarioDocument -> PreparedWorld` contract.

## Planned Slices

### Bootstrap-1 — Foundation And Minimal Viable World

Goal: create the application-level Bootstrap contract and deterministic constrained generation sufficient for Runner-1 to start a living world.

Build:

```text
src/bootstrap/
  mod.rs
  prepared.rs
  manifest.rs
  seed_domains.rs
  resource_layers.rs
  field_layers.rs
  cell_placement.rs
  starter_state.rs
  viability.rs
```

Gate:

```text
same ScenarioDocument + seed -> same PreparedWorld hash
changing one seed domain perturbs only that generated domain
BootstrapManifest records generator versions, counts, resource totals, field ranges, warnings
Bootstrap executes no Tick
PreparedWorld passes Core construction smoke test
minimal viable scenario survives a short smoke window without scripted behavior
```

Detailed worklog:

```text
outputs/worklogs/2026-07-14-1635-PLAN-bootstrap-1-foundation.md
```

### Bootstrap-2 — Rich Spatial Generators

Goal: add more expressive but still deterministic spatial initialization.

Candidates:

- layered resource patches with falloff;
- material fragment fields;
- blocked/solid material regions;
- gradients and bands for field layers;
- safe starting niches with explicit viability warnings.

Non-goal: seasons/catastrophes.

### Bootstrap-3 — World Families And Seasonal Inputs

Goal: introduce world family presets and controlled temporal initial conditions without runtime season mechanics.

Candidates:

- dry/wet/frozen/volatile starting profiles;
- resource scarcity gradients;
- temperature and field initial maps;
- catastrophe scars as initial conditions.

Runtime seasons and catastrophes require separate Core mechanics and must not be hidden inside Bootstrap.

### Bootstrap-4 — Preview, Reports, And Calibration

Goal: expose Bootstrap manifests and previews for humans and batch tools.

Candidates:

- CLI `bootstrap` mode;
- manifest export;
- optional static preview maps;
- viability envelope reports;
- seed sweep for starting-world quality.

## Integration With Runner

Runner plans depend on Bootstrap as follows:

```text
Runner-1:
  StartRun -> ScenarioDocument -> Bootstrap -> PreparedWorld -> Core start

Runner-2:
  HTTP /run/start -> shared StartRun command -> Bootstrap

Runner-3:
  WS frames stream projections after Core commits, not Bootstrap internals

Runner-4:
  hardens Bootstrap errors, manifests, status, and replay identity
```

Runner must not recreate Bootstrap logic.

## Integration With Genome

Genome-specific initialization belongs to domain modules, but Bootstrap owns the application-level decision of assigning initial Genome state to starting Cells.

Existing/future genome modules provide:

```text
GenomeTemplate -> deterministic GenomeState
```

Bootstrap calls those modules while preparing Tick 0 and records the result in `BootstrapManifest`.

## Acceptance Gate

Bootstrap plan is complete when:

```text
Bootstrap-1 worklog exists and is execution-grade TDD
Runner-1 worklog references Bootstrap-1 as prerequisite
Implementation index links this plan
Parent Runner plan references this plan
No runner plan requires direct TOML -> RuntimeConfig -> Core startup
```

## Semantic Links

- constrained by: [[docs/runner/bootstrap|Bootstrap Canon]]
- depends on: [[docs/runner/scenario-resolution|Scenario Resolution]]
- feeds: [[docs/implementation/implementation-plan-runner|Runner Implementation Plan]]
- uses config from: [[docs/config/INDEX|Config Index]]
- prepares world described by: [[docs/world/INDEX|World Index]]
