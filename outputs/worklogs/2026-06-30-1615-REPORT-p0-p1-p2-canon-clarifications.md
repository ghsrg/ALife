---
tags:
  - alife
  - worklog/report
---

# REPORT: p0-p1-p2-canon-clarifications

Дата: 2026-06-30 16:15

Плани:

- `outputs/worklogs/2026-06-30-1457-PLAN-p0-accepted-clarifications.md`
- `outputs/worklogs/2026-06-30-1558-PLAN-p1-p2-accepted-clarifications.md`

---

# Що зроблено

## P0

Оновлено Energy model:

- `Energy Buffer` більше не трактується як речовина або volume-bearing entity.
- Energy capacity задається storage-capable Materials, структурою та internal organization.
- Заборонено трактувати Energy Buffer як Resource або Material.

Оновлено Genome copying:

- Genome розділено на information, physical genome carrier і runtime machinery.
- Genome copy потребує physical carrier з Resources/Materials/internal precursor fragments.
- Energy powers copying, але не створює matter.
- Daughter cell не отримує functional Genome без physical copy або valid inherited carrier.

Оновлено Tick causality:

- Причинність стала phase-based.
- Environment Phase може створити committed environment snapshot для Decision Phase того самого Tick.
- Same-phase feedback і uncommitted reads заборонені.

Оновлено mandatory costs:

- Mandatory costs оплачуються до Feasibility для planned actions.
- Feasibility використовує `committed_state_after_mandatory_costs`.

## P1

Додано Reaction Accounting Contract:

- reactions use material/amount accounting, not strict SI mass accounting;
- `energy_output` не замінює material outputs;
- configured sink/loss має бути явним;
- validation має warning/fatal правила для незбалансованих reactions.

Розширено `field-semantics.md` до Field Effect Contract:

- Field не є командою;
- Field effects потребують material/process/reaction/physics mediation;
- Heat описано як concrete profile з `temperature`, `heat_capacity`, transfer, dissipation і material tolerance.

Уточнено 2D capacity:

- `volume_capacity` є abstract internal capacity, not SI volume;
- bounded by `radius`, `cell_area` і storage-capable Materials;
- стартова формула: `base_capacity_per_area * cell_area * storage_material_modifier`.

Стиснуто `tick.md`:

- тепер це conceptual time document;
- visibility/causality винесено в `tick-semantics.md`;
- implementation order належить `engine/scheduler.md`.

Створено:

```text
docs/biology/action-process-registry.md
```

Реєстр став canonical source для process ids, Genome output bindings, Feasibility scope і duration.

Уточнено MaterialFragment:

- external `MaterialFragment` не є active Cell Material;
- capabilities працюють тільки у правильному Cell/Joint context;
- MaterialFragment стає Resource тільки через explicit degradation/reaction/conversion.

## P2

Синхронізовано:

- `docs/GLOSSARY.md`;
- `docs/ROADMAP.md`;
- `docs/README.md`;
- `docs/config/stability_bounds.md`.

У `stability_bounds.md` додано:

- Hard Invalid Examples;
- Warning Ranges;
- Scenario-Specific Experimental Ranges.

`PRINCIPLES.md` уточнено: Organism є observer-side organism-like graph view, а connected component є кандидатом, не автоматичним Organism.

---

# Основні Файли

Змістовно оновлено:

```text
docs/PRINCIPLES.md
docs/GLOSSARY.md
docs/ROADMAP.md
docs/README.md
docs/world/laws.md
docs/world/tick.md
docs/world/tick-semantics.md
docs/world/energy.md
docs/world/reactions.md
docs/world/field-semantics.md
docs/world/fields.md
docs/world/physics.md
docs/world/space.md
docs/world/units.md
docs/world/materials.md
docs/world/resources.md
docs/biology/action-process-registry.md
docs/biology/cell.md
docs/biology/division-partition.md
docs/biology/feasibility.md
docs/biology/genome.md
docs/biology/joint.md
docs/biology/lifecycle.md
docs/biology/processes.md
docs/biology/process-progress.md
docs/genetics/inheritance.md
docs/genetics/genome-runtime.md
docs/genetics/regulatory-interface.md
docs/genetics/regulatory-network.md
docs/config/fields_config.md
docs/config/materials_config.md
docs/config/reactions_config.md
docs/config/stability_bounds.md
docs/config/world_config.md
docs/engine/chemistry.md
docs/engine/ecs.md
docs/engine/physics.md
docs/engine/scheduler.md
```

---

# Перевірка

Команди:

```powershell
rg -n "MVP|first simulation|перша симуляція|Energy as Resource|Energy as Material|Genome copy from Energy only|Field as command|Light creates Energy directly|Heat damage as HP|literal scheduler order|unbounded capacity" docs README.md
```

Результат: exit 1, збігів немає.

```powershell
rg -n "Reaction Accounting|Field Effect Contract|Action / Process Registry|MaterialFragment|phase commit|Mandatory costs|physical genome carrier|volume_capacity" docs\world docs\biology docs\genetics docs\config docs\engine docs\GLOSSARY.md docs\PRINCIPLES.md
```

Результат: exit 0, потрібні ключові контракти знайдені в очікуваних файлах.

```powershell
Test-Path docs\biology\action-process-registry.md
```

Результат: `True`.

---

# Відомі Залишки

Unrelated Obsidian changes лишаються в working tree і не чіпалися:

```text
D docs/.obsidian/app.json
D docs/.obsidian/appearance.json
D docs/.obsidian/core-plugins.json
D docs/.obsidian/graph.json
D docs/.obsidian/workspace.json
?? .obsidian/
```

`docs/examples/` також лишається untracked з попереднього кроку чистки документації.

Git продовжує показувати CRLF/LF warnings для markdown-файлів при `git diff --stat`; line endings policy ще не нормалізовано.
