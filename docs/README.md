---
tags:
  - alife
  - docs/readme
  - audience/human
---

# Документація ALife

`docs/` — головний каталог знань про Artificial Life Engine.

Тут описані не лише майбутні модулі програми, а й правила світу: фізика, ресурси, матеріали, клітини, процеси, геном, еволюція, рушій, UI, конфігурації та дослідницькі припущення.

Документація є джерелом істини для проєкту. Якщо код суперечить документації, потрібно або виправити код, або змінити документацію через явне рішення.

Для швидкої агентської навігації використовується окремий індекс: [[docs/INDEX|Documentation Index]]. Для перехресних взаємодій між документами використовується [[docs/mechanics/INDEX|Mechanics Index]].

Робочі плани й звіти зібрані окремо: [[outputs/worklogs/index|Worklogs]].

---

# Як читати

## Якщо ви вперше відкрили проєкт

Починайте з базових документів:

1. [[docs/PRINCIPLES|Principles]] — верхній рівень правил.
2. [[docs/GLOSSARY|Glossary]] — єдині терміни.
3. [[docs/ROADMAP|Roadmap]] — поточні статуси документації та фокус робіт.
4. [[docs/world/laws|World Laws]] — фундаментальні закони світу.
5. [[docs/biology/cell|Cell]] — базова сутність життя.
6. [[docs/biology/processes|Processes]] і [[docs/biology/feasibility|Feasibility]] — як клітина може діяти.
7. [[docs/genetics/genome-representation|Genome Representation]] і [[docs/genetics/genome-runtime|Genome Runtime]] — як геном регулює поведінку.
8. [[docs/evolution/population-dynamics|Population Dynamics]] — як аналізуються популяції.
9. [[docs/observer/observer-layer|Observer Layer]] — read-only шар для projection, analytics і coverage.
10. [[docs/engine/technology-stack|Technology Stack]] — прийнятий стек.
11. [[docs/implementation/implementation-phases|Implementation Phases]] — порядок розробки.

## Якщо шукаєте конкретну тему

Використовуйте:

- [[docs/INDEX|Documentation Index]] — ієрархічний агентський роутер по локальних індексах;
- [[docs/mechanics/INDEX|Mechanics Index]] — pre-flight cards для взаємодій, constraints і TDD checks;
- [[docs/GLOSSARY|Glossary]] — якщо незрозумілий термін;
- [[docs/ROADMAP|Roadmap]] — якщо потрібно зрозуміти статус;
- [[outputs/worklogs/index|Worklogs]] — якщо потрібна історія планів і звітів.

---

# Рівні документації

```text
Principles
  -> Canon
  -> accepted ADR
  -> Implementation
  -> Mechanics cards
  -> Research / Examples / Worklogs
```

## Principles

[[docs/PRINCIPLES|Principles]] — найвищий рівень правил. Нижчі документи не повинні їм суперечити.

## Canon

Canon описує прийняті правила світу та моделі.

До Canon належать основні документи в:

- `world/`
- `biology/`
- `genetics/`
- `evolution/`
- `config/`
- `engine/`
- `ui/`

## Implementation

[[docs/implementation/INDEX|Implementation Index]] і [[docs/implementation/README|Implementation]] описують, як прийняті правила планується реалізувати в коді: фази, архітектуру, моделі даних, API, інструменти й UI-плани.

Implementation-документи не змінюють Canon.

## ADR

[[docs/decisions/INDEX|Decisions Index]] і [[docs/decisions/README|Decisions]] містять прийняті архітектурні рішення.

ADR потрібен, коли є реальний вибір між альтернативами або фундаментальна зміна напряму.

## Mechanics Cards

[[docs/mechanics/INDEX|Mechanics Index]] містить agent-facing routing checklists для взаємодій між Canon-документами. Mechanics cards не є джерелом істини й не вводять нових правил.

## Research

`research/` містить ідеї, альтернативи й майбутні варіанти.

Research не є вимогою до реалізації, доки рішення не перенесене в Canon або ADR.

## Examples

[[docs/examples/README|Examples]] ілюструють правила.

Приклад не створює нового правила. Якщо приклад суперечить Canon, правильним вважається Canon.

---

# Каталоги

## `world/`

Agent index: [[docs/world/INDEX|World Index]].

Закони світу: простір, час, поля, ресурси, матеріали, реакції, енергія, фізика та одиниці виміру.

Ключові входи:

- [[docs/world/laws|World Laws]]
- [[docs/world/space|Space]]
- [[docs/world/tick-semantics|Tick Semantics]]
- [[docs/world/resources|Resources]]
- [[docs/world/materials|Materials]]
- [[docs/world/energy|Energy]]
- [[docs/world/physics|Physics]]

## `biology/`

Agent index: [[docs/biology/INDEX|Biology Index]].

Механіка живих сутностей: клітина, мембрана, процеси, capability, feasibility, життєвий цикл, поділ, Joint, комунікація, organism view і спеціалізація.

Ключові входи:

- [[docs/biology/cell|Cell]]
- [[docs/biology/cell-state|Cell State]]
- [[docs/biology/membrane|Membrane]]
- [[docs/biology/processes|Processes]]
- [[docs/biology/action-process-registry|Action Process Registry]]
- [[docs/biology/feasibility|Feasibility]]
- [[docs/biology/lifecycle|Lifecycle]]
- [[docs/biology/joint|Joint]]
- [[docs/biology/organism|Organism View]]

## `genetics/`

Agent index: [[docs/genetics/INDEX|Genetics Index]].

Геном, спадковість, мутації, genome runtime, regulatory graph, epigenetics, recombination і horizontal transfer.

Ключові входи:

- [[docs/genetics/genome-representation|Genome Representation]]
- [[docs/genetics/genome-runtime|Genome Runtime]]
- [[docs/genetics/inheritance|Inheritance]]
- [[docs/genetics/mutation|Mutation]]
- [[docs/genetics/regulatory-network|Regulatory Network]]

## `evolution/`

Agent index: [[docs/evolution/INDEX|Evolution Index]].

Observer-side еволюційна аналітика: adaptation, population dynamics, selection і species-like clusters.

Ключові входи:

- [[docs/evolution/adaptation|Adaptation]]
- [[docs/evolution/population-dynamics|Population Dynamics]]
- [[docs/evolution/selection|Selection]]
- [[docs/evolution/species-like-clusters|Species-like Clusters]]

## `observer/`

Agent index: [[docs/observer/INDEX|Observer Index]].

Read-only шар для projection, analytics, OrganismView, lineage summaries, mechanism coverage і balance/reachability reports.

Ключові входи:

- [[docs/observer/observer-layer|Observer Layer]]
- [[docs/observer/mechanism-coverage|Mechanism Coverage Contract]]
- [[docs/observer/projection-contract|Projection Contract]]

## `config/`

Agent index: [[docs/config/INDEX|Config Index]].

Майбутні конфігурації світу, ресурсів, матеріалів, реакцій, полів і stability bounds.

Ключові входи:

- [[docs/config/world_config|World Config]]
- [[docs/config/resources_config|Resources Config]]
- [[docs/config/materials_config|Materials Config]]
- [[docs/config/stability_bounds|Stability Bounds]]

## `engine/`

Agent index: [[docs/engine/INDEX|Engine Index]].

Технічна архітектура рушія: scheduler, performance, storage, serialization, physics, rendering, chemistry, ECS та technology stack.

Ключові входи:

- [[docs/engine/technology-stack|Technology Stack]]
- [[docs/engine/performance|Performance]]
- [[docs/engine/scheduler|Scheduler]]
- [[docs/engine/storage|Storage]]
- [[docs/engine/rendering|Rendering]]

## `ui/`

Agent index: [[docs/ui/INDEX|UI Index]].

UI Canon для `ALife Control Center`: принципи, архітектура, навігація, візуалізація, аналітика, exploration, presentation, interaction і quality.

Ключові входи:

- [[docs/ui/README|UI Layer]]
- [[docs/ui/principles|UI Principles]]
- [[docs/ui/architecture|UI Architecture]]
- [[docs/ui/visualization|UI Visualization]]
- [[docs/ui/interaction|UI Interaction]]
- [[docs/ui/quality|UI Quality]]
- [[docs/implementation/implementation-plan-ui|UI Implementation Plan]]

## `implementation/`

Agent index: [[docs/implementation/INDEX|Implementation Index]].

Плани реалізації, фазування, архітектура, data model, module API, optimization paths, stability tools і UI implementation plan.

Ключові входи:

- [[docs/implementation/README|Implementation]]
- [[docs/implementation/implementation-phases|Implementation Phases]]
- [[docs/implementation/architecture|Architecture]]
- [[docs/implementation/optimization-paths|Optimization Paths]]
- [[docs/implementation/implementation-plan-ui|UI Implementation Plan]]

## `research/`

Гіпотези, альтернативи та відкладені ідеї.

## `decisions/`

Agent index: [[docs/decisions/INDEX|Decisions Index]].

ADR-журнал прийнятих рішень.

## `examples/`

Agent index: [[docs/examples/INDEX|Examples Index]].

Приклади, які допомагають читати Canon.

---

# Правила змін

Перед зміною документації або коду:

1. прочитайте [[docs/PRINCIPLES|Principles]];
2. перевірте терміни в [[docs/GLOSSARY|Glossary]];
3. знайдіть відповідний Canon-документ;
4. перевірте пов'язані implementation-документи;
5. перевірте ADR, якщо зміна архітектурна;
6. якщо правила немає, не вигадуйте його мовчки: створіть питання, план або ADR-пропозицію.

---

# Semantic Links

- agent index: [[docs/INDEX|Documentation Index]]
- mechanics router: [[docs/mechanics/INDEX|Mechanics Index]]
- governed by: [[docs/PRINCIPLES|Principles]]
- uses terms from: [[docs/GLOSSARY|Glossary]]
- tracks status in: [[docs/ROADMAP|Roadmap]]
- implements through: [[docs/implementation/README|Implementation]]
- includes UI canon: [[docs/ui/README|UI Layer]]
- includes observer layer: [[docs/observer/README|Observer]]
- records decisions in: [[docs/decisions/README|Decisions]]
- records work in: [[outputs/worklogs/index|Worklogs]]
