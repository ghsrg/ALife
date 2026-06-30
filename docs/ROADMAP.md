---
tags:
  - alife
  - docs/index
---

# ROADMAP.md

> **Поточний стан документації Artificial Life Engine**

---

# Призначення

Цей документ є єдиним місцем для поточних статусів документації.

Roadmap не є специфікацією світу, біології або рушія.

Він не визначає етапи реалізації й не фіксує план кодування.

План реалізації буде створений окремо після узгодження документації, вибору технологічного стеку та архітектури рушія.

---

# Статуси документації

| Документ / розділ | Статус |
| --- | --- |
| `README.md` | Current |
| `docs/README.md` | Current |
| `docs/PRINCIPLES.md` | Base requirements |
| `docs/GLOSSARY.md` | Current |
| `docs/STYLE_GUIDE.md` | Current |
| `docs/world/` | Draft / P0-P2 synced |
| `docs/biology/` | Draft / P0-P2 synced |
| `docs/genetics/` | Draft / P0-P2 synced |
| `docs/evolution/` | Draft / audit in progress |
| `docs/config/` | Draft / P0-P2 synced |
| `docs/engine/` | Draft / tech stack accepted |
| `docs/implementation/` | Phase roadmap created |
| `docs/research/` | Reference / future options |
| `docs/decisions/` | ADR-0001 accepted |

---

# Поточний фокус

Зараз проєкт перебуває на етапі валідації документації.

Поточні роботи:

* виявити та уточнити розбіжності;
* виявити сірі зони;
* очистити документацію від води;
* перенести розмазаний зміст у правильні місця;
* перевірити відповідність базовим фізичним, хімічним і біологічним обмеженням;
* підготувати документацію до Obsidian-навігації;
* лише після цього планувати реалізацію.

P0-P2 уточнення після повторного physics/logic audit внесені в Canon. Технологічний стек прийнято в `docs/decisions/ADR-0001-tech-stack.md` і деталізовано в `docs/engine/technology-stack.md`. High-level план реалізації створено в `docs/implementation/implementation-phases.md`.

---

# Майбутня рамка реалізації

Реалізаційні фази не є частиною Canon-документації.

Після завершення аудиту буде створений окремий план реалізації. Базова рамка для майбутнього планування:

```text
Phase 0: Technical Foundation
Phase 1: Deterministic World Smoke
Phase 2: Processes, Feasibility And Lifecycle
Phase 3: Genome Runtime And Inheritance
Phase 4: Joints, Signals And Multicellular Structures
Phase 5: Evolution Analytics And Stability Experiments
Phase 6: Performance Scale-Up
Phase 7: Advanced Evolution Capability
```

Ця рамка деталізована в `docs/implementation/implementation-phases.md` і не є вимогою Canon.

---

# Відкриті рішення

Ці рішення потрібно уточнити під час наступних аудитів або перед плануванням реалізації:

* межа між `world/`, `biology/`, `genetics/`, `engine/`;
* остаточні Obsidian links і backlinks;
* які базові вимоги потребують реального ADR;
* межі smoke simulation;
* мінімальні параметри стабільного світу;
* конкретні threshold values для stability bounds після експериментальної калібровки.

Прийняті рішення:

* технологічний стек;
* формат конфігурацій: TOML;
* storage boundary: binary snapshots/events, SQLite metadata/index, Parquet analytics.

---

# Довгострокова мета

Створити симуляцію, у якій складні багатоклітинні організми, навчання, спеціалізація тканин, соціальна поведінка та екосистеми виникають без жорстко запрограмованих біологічних механік, а є наслідком універсальних законів світу та еволюції.

# Semantic Links

- tracks status of: [[docs/PRINCIPLES|Principles]]
- tracks status of: [[docs/world/laws|World Laws]]
- tracks status of: [[docs/biology/cell|Cell]]
- tracks status of: [[docs/genetics/genome-representation|Genome Representation]]
- informs: [[docs/config/stability_bounds|Stability Bounds]]
- tracks accepted: [[docs/decisions/ADR-0001-tech-stack|ADR-0001 Technology Stack]]
- tracks plan: [[docs/implementation/implementation-phases|Implementation Phases]]
