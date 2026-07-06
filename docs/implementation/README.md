---
tags:
  - alife
  - implementation
  - docs/index
---

# implementation

Agent index: [[docs/implementation/INDEX|Implementation Index]]. Mechanics pre-flight: [[docs/mechanics/INDEX|Mechanics Index]].

> Implementation planning — високорівневі плани реалізації, фазування, архітектурні нотатки та майбутні інтерфейси.

---

# Призначення

`docs/implementation/` містить документи, які переводять Canon, ADR і engine-рішення у план майбутньої розробки.

Цей каталог не змінює закони світу. Якщо реалізаційний план суперечить Canon або ADR, потрібно змінити план або створити нове рішення.

---

# Документи

- [[docs/implementation/architecture|Architecture]] — базова архітектурна рамка реалізації: data-oriented deterministic core, clean/hexagonal outer shell, межі `alife-core`, runner, storage, viewer і analysis.
- [[docs/implementation/implementation-phases|Implementation Phases]] — high-level фазовий roadmap: що будуємо в Phase 0-7, які gates має пройти кожна фаза, де починаються stability/calibration tools.
- [[docs/implementation/phase-1-design|Phase 1 Design]] — детальніший дизайн першої runnable smoke-фази: мінімальний `WorldState`, `CellStore`, `ResourceGrid`, Tick pipeline, configs, scenarios і acceptance gates.
- [[docs/implementation/phase-1-data-model|Phase 1 Data Model]] — Rust-side Phase 1 data model: typed ids, typed accounting wrappers, `WorldState`, `CellStore`, `ResourceGrid`, lifecycle state, deltas, events, snapshots and explicit future placeholders.
- [[docs/implementation/phase-1-module-api|Phase 1 Module API]] — Rust module/API contract before coding: module ownership, public API shapes, dependency direction, Tick executor, runner boundary, errors and tests.
- [[docs/implementation/optimization-paths|Optimization Paths]] — guardrail для майбутньої продуктивності: які storage/API рішення треба залишити відкритими, щоб не заблокувати SIMD, sparse/chunked grids, domain decomposition, LOD viewer і deterministic parallelism.
- [[docs/implementation/early-stability-tool|Early Stability Tool]] — handoff-документ для окремого агента, який реалізує `tools/early-stability/`: static calculator, micro headless simulator, CLI, scenarios, outputs і validation rules.
- [[docs/implementation/implementation-plan-ui|UI Implementation Plan]] — high-level parent plan для `ALife Control Center`: Start, Debug, Research, projection gateway, shared viewer, command boundary, worklog model і UI acceptance gates.

---

# Правила

- High-level фазовий план тримається тут.
- Before TDD planning, use [[docs/mechanics/INDEX|Mechanics Index]] and read relevant mechanics cards.
- Детальний план кожної фази створюється окремо перед початком цієї фази.
- Архітектура, класи, інтерфейси, storage format і test strategy можуть бути додані сюди після затвердження фаз.
- Early Stability Tool є допоміжним implementation/research tool і не є частиною simulation hot path.
- UI Implementation Plan спирається на [[docs/ui/README|UI Canon]] і не надає UI simulation authority.
- Worklogs лишаються в `outputs/worklogs/`.

---

# Semantic Links

- implements: [[docs/decisions/ADR-0001-tech-stack|ADR-0001 Technology Stack]]
- defines implementation: [[docs/implementation/architecture|Architecture]]
- defines phase: [[docs/implementation/phase-1-design|Phase 1 Design]]
- defines data model: [[docs/implementation/phase-1-data-model|Phase 1 Data Model]]
- defines module API: [[docs/implementation/phase-1-module-api|Phase 1 Module API]]
- preserves optimization paths: [[docs/implementation/optimization-paths|Optimization Paths]]
- hands off tool: [[docs/implementation/early-stability-tool|Early Stability Tool]]
- plans UI: [[docs/implementation/implementation-plan-ui|UI Implementation Plan]]
- follows UI canon: [[docs/ui/README|UI Layer]]
- follows: [[docs/engine/technology-stack|Technology Stack]]
- follows: [[docs/PRINCIPLES|Principles]]
- uses pre-flight cards: [[docs/mechanics/INDEX|Mechanics Index]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]
