---
tags:
  - alife
  - worklog/report
  - implementation
---

# REPORT: Phase 1 Design Audit And Early Stability Sync

Date: 2026-07-01 10:15

---

# Goal

Провести аудит `docs/implementation/phase-1-design.md`, доповнити його до рівня робочого джерела для Phase 1 і синхронізувати `docs/implementation/early-stability-tool.md` перед передачею іншому агенту.

---

# Audit Findings

Виявлено і виправлено:

- Phase 1 не мав явного shared vocabulary для `survival_result`, `collapse_reason` і scenario ids.
- Phase 1 config surface і Early Stability Tool inputs використовували різні назви для Energy/capacity/heat/waste полів.
- Lifecycle rules були занадто загальними для calculator/simulator handoff.
- Accounting contract не фіксував мінімальні формули для Energy, capacity, Heat і waste.
- У `phase-1-design.md` були некоректно закодовані tree characters у `Cell` state block.

---

# Changes

`docs/implementation/phase-1-design.md`:

- додано `Accounting Contract` з формулами:
  - `energy_after_mandatory`;
  - `mandatory_paid`;
  - `used_capacity`;
  - `free_capacity`;
  - `heat_next`;
  - `waste_next`.
- додано `Lifecycle Contract` для `alive`, `stressed`, `dormant`, `dead` і правила пріоритету переходів.
- уточнено config surface:
  - `mandatory_cost_per_tick`;
  - `capacity_limit`;
  - `minimum_viability_materials`;
  - heat/waste current, generated, sink/dissipation і thresholds;
  - lifecycle thresholds.
- додано `Event And Output Contract`.
- уніфіковано scenario ids:
  - `single_cell_survival`;
  - `single_cell_starvation`;
  - `single_cell_over_capacity`;
  - `single_cell_heat_stress`;
  - `deterministic_replay_smoke`;
  - `viewer_snapshot_smoke`.
- додано `survival_result` і `collapse_reason` vocabulary.

`docs/implementation/early-stability-tool.md`:

- синхронізовано inputs із Phase 1 config concepts.
- розділено Phase 1-aligned scenarios і tool-only estimate scenarios.
- додано `Sync Contract With Phase 1`.
- додано tool-only `estimates.*` namespace, щоб агент не переносив estimate-поля в Phase 1 runtime state.
- синхронізовано result/collapse vocabulary.

---

# Verification

Local link audit:

```text
Broken local links: 0
```

Placeholder scan:

```text
No matches for TBD, TODO, "заповнити потім", "fill in later".
```

Old-name scan:

```text
No matches for single_cell_survives, single_cell_starves, mandatory_costs,
capacity_limits, heat_dissipation_placeholder, waste_sink_placeholder.
```

---

# Open Questions

Немає блокуючих питань.

Наступний крок: або деталізувати Phase 1 implementation plan, або передати `early-stability-tool.md` іншому агенту для планування/реалізації `tools/early-stability/`.
