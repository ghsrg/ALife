---
tags:
  - alife
  - implementation
  - docs/index
---

# implementation

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
- [[docs/implementation/early-stability-tool|Early Stability Tool]] — handoff-документ для окремого агента, який реалізуватиме `tools/early-stability/`: static calculator, micro headless simulator, CLI, scenarios, outputs і validation rules.

---

# Правила

- High-level фазовий план тримається тут.
- Детальний план кожної фази створюється окремо перед початком цієї фази.
- Архітектура, класи, інтерфейси, storage format і test strategy можуть бути додані сюди після затвердження фаз.
- Early Stability Tool є допоміжним implementation/research tool і не є частиною simulation hot path.
- Worklogs лишаються в `outputs/worklogs/`.

---

# Semantic Links

- implements: [[docs/decisions/ADR-0001-tech-stack|ADR-0001 Technology Stack]]
- defines implementation: [[docs/implementation/architecture|Architecture]]
- defines phase: [[docs/implementation/phase-1-design|Phase 1 Design]]
- hands off tool: [[docs/implementation/early-stability-tool|Early Stability Tool]]
- follows: [[docs/engine/technology-stack|Technology Stack]]
- follows: [[docs/PRINCIPLES|Principles]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]
