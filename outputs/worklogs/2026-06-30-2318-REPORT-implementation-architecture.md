---
tags:
  - alife
  - worklog/report
  - implementation
---

# REPORT: Implementation Architecture

Date: 2026-06-30 23:18

---

# Goal

Зафіксувати в implementation-документації погоджену архітектуру:

```text
Simulation Core = data-oriented deterministic architecture
Application Shell = clean / hexagonal boundaries
```

---

# Scope

Зміни внесено лише в documentation / implementation planning.

Canon-документи та ADR-0001 не змінювались за змістом.

---

# Decisions Fixed

- Classical Clean Architecture не є основною архітектурою simulation core.
- Core будується як deterministic data pipeline з SoA/ECS storage, stable ids, deltas і deterministic commit.
- Clean / hexagonal boundaries застосовуються на зовнішньому рівні: config, storage, viewer, analysis.
- Viewer, storage і analysis є projections, а не simulation authority.
- External adapters не входять у hot path.
- Object-heavy per-entity classes і virtual behavior не мають бути основою core state.

---

# Files Changed

- `docs/implementation/architecture.md`
  - створено новий implementation architecture документ;
  - описано core rule, layer shape, modules, application boundaries, data flow, testing strategy, invariants.

- `docs/implementation/README.md`
  - додано посилання на architecture document.

- `docs/implementation/implementation-phases.md`
  - додано принцип: data-oriented deterministic core + clean/hexagonal outer boundaries.

- `docs/ROADMAP.md`
  - оновлено статус `docs/implementation/`;
  - додано посилання на architecture document.

---

# Verification

Local link audit:

```text
Broken local links: 0
```

---

# Open Questions

Немає блокуючих питань.

Детальні crate names, module boundaries, interfaces і data layouts мають бути уточнені в Phase 0 detailed plan.
