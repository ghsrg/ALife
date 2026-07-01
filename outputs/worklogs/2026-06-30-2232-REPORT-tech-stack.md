---
tags:
  - alife
  - worklog/report
---

# REPORT: Technology Stack

## Goal

Зафіксувати прийнятий технологічний стек для першої реалізації Artificial Life Engine.

## Scope

Прийнято:

```text
Core Simulation:
  Rust
  custom data-oriented SoA/ECS
  deterministic scheduler
  headless source of truth

Viewer:
  local Web Viewer
  WebSocket binary frames
  WebGL2 instanced rendering
  read-only

Analysis:
  Python
  Parquet / DuckDB / Polars / Pandas

Storage:
  versioned binary snapshots
  binary event logs
  SQLite metadata/index
  Parquet analytics exports

Configs:
  TOML

Numeric:
  fixed-point wrappers for behavior-critical accounting
  f32 for geometry/visual/soft continuous values with deterministic order
```

## Decisions

- Simulation core is headless and deterministic.
- Viewer is a read-only projection and cannot affect behavior.
- GPU/UHD is used for rendering only in the first implementation.
- Base runtime target is 2D; architecture remains 2D-first, 3D-compatible.
- Genome Runtime, traces, Fields and observer metrics may use configurable scheduler cadence.
- Storage/DB are outside hot path; simulation does not block on SQLite, Parquet or viewer output by default.
- Deterministic replay is required within same config, seed, initial state, engine version, binary/runtime, deterministic mode and platform mode.

## Files Changed

- `docs/decisions/ADR-0001-tech-stack.md`
- `docs/engine/technology-stack.md`
- `docs/engine/performance.md`
- `docs/engine/scheduler.md`
- `docs/engine/storage.md`
- `docs/decisions/README.md`
- `docs/ROADMAP.md`

## Verification

- Local link check: `Broken local links: 0`.
- Targeted search confirmed accepted stack terms in ADR, engine docs, ROADMAP and this report.
- Domain `# Open Questions` remain closed; only non-domain `docs/STYLE_GUIDE.md` still has that section.

## Next

- Use this stack decision as input for the implementation roadmap phases.
- Define initial stability/calibration calculator and seed configs after roadmap planning.
