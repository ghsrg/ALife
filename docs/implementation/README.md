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

- [[docs/implementation/implementation-phases|Implementation Phases]]

---

# Правила

- High-level фазовий план тримається тут.
- Детальний план кожної фази створюється окремо перед початком цієї фази.
- Архітектура, класи, інтерфейси, storage format і test strategy можуть бути додані сюди після затвердження фаз.
- Worklogs лишаються в `outputs/worklogs/`.

---

# Semantic Links

- implements: [[docs/decisions/ADR-0001-tech-stack|ADR-0001 Technology Stack]]
- follows: [[docs/engine/technology-stack|Technology Stack]]
- follows: [[docs/PRINCIPLES|Principles]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]
