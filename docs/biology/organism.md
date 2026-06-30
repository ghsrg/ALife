---
tags:
  - alife
  - canon
  - area/biology
---

# organism.md

> `Organism` — observer-side уявлення про стабільну багатоклітинну структуру.

---

# Призначення

`Organism` описує не нову поведінкову сутність рушія, а аналітичний погляд на граф `Cell + Joint`.

Світ не використовує `Organism` для прийняття рішень. Клітини живуть, діють і гинуть локально через власні Materials, Genome Runtime, Energy, Joint, Signals і Feasibility Check.

`Observer`, debug UI, lineage tools і research metrics можуть будувати `OrganismView` як derived view над клітинами, Joint і lineage.

---

# Канонічні правила

- `Organism` не є класом, який керує клітинами.
- Базова структура: `OrganismGraph = (Cells, Joints)`.
- Detection у першій реалізації: `OrganismView = connected component of Cell-Joint graph`.
- Межа між `cluster`, `colony-like` і `organism-like` є континуумом.
- Dependency, resource flow, signal flow, specialization, repair coupling і reproduction coupling є observer/debug/research metrics, а не умовами detection.
- `Organism identity` є аналітичною міткою, а не входом для Genome Runtime.
- Усі процеси виконуються клітинами, а не organism-level controller.
- `Organism death` означає структурний collapse, а не HP event.

---

# Мінімальна модель

Для першої реалізації достатньо:

```text
OrganismView = connected component of Cell-Joint graph
```

Observer може рахувати:

```text
cell_count
joint_count
component_age_ticks
connectedness
shared_lineage_ratio
resource_transfer_edges_count
signal_edges_count
fragmentation_events_count
merge_events_count
collapse_reason
```

Ці метрики не повинні повертатися в поведінку клітини як глобальний control signal.

---

# Dependency

Dependency metrics describe structure. They do not define whether OrganismView exists and do not control behavior.

Minimum `connectedness` can start as:

```text
connectedness =
  joint_count / max(1, cell_count - 1)
```

This is a debug metric, not a quality score.

Future dependency analysis may include:

```text
resource_dependency
signal_dependency
mechanical_dependency
repair_dependency
reproduction_dependency
survival_without_structure_probability
```

Low dependency is closer to colony-like structure. High dependency is closer to organism-like structure. This classification is observer-side only.

---

# View Lineage Events

Do not model complex organism heredity in the first implementation.

Track observer-only events:

```text
component_created
component_split
component_merged
component_collapsed
component_extinct
```

For each OrganismView:

```text
view_id
parent_view_ids
child_view_ids
dominant_lineage_refs
lineage_distribution
genome_distribution
mixedness_score
created_tick
ended_tick
```

If a component fragments, new components get new `view_id` values and reference the parent view.

If components merge, the new `view_id` has multiple parents.

---

# Minimum OrganismView

```text
OrganismView
├── view_id
├── tick_created
├── tick_updated
├── cell_ids optional
├── cell_count
├── joint_count
├── bounding_box
├── center_of_mass
├── dominant_lineage_refs
├── lineage_distribution
├── genome_distribution
├── mixedness_score
├── component_age_ticks
├── resource_flow_summary
├── signal_flow_summary
└── event_refs
```

For performance, `cell_ids` may be stored only in selected/debug mode.

General statistics can use only `cell_count`, `joint_count`, `bounding_box` and lineage summary.

---

# Reproduction-like Events

Organism-level reproduction не є окремим hardcoded процесом.

Observer може позначати reproduction-like event, якщо від структури відокремлюється життєздатний fragment або founder cell, здатний продовжити lineage.

Рушій при цьому виконує тільки локальні процеси: division, Joint break/reassign, movement, resource flow, damage і repair.

---

# Заборонено

Не вводити:

- organism HP;
- global organism bus;
- hardcoded body plan;
- hardcoded tissues or organs;
- species_id-based behavior;
- central controller;
- direct Energy Buffer sharing;
- global role assignment.

---

# Invariant

```text
OrganismView is observer-only.
Detection is based on Cell-Joint connected components.
Dependency metrics describe structure, but do not define behavior.
Cells cannot read OrganismView, dependency score, fitness or component id.
```

---

# Semantic Links

- derived from: [[docs/biology/cell|Cell]]
- derived from: [[docs/biology/joint|Joint]]
- observed by: [[docs/engine/rendering|Rendering]]
- analyzed by: [[docs/evolution/population-dynamics|Population Dynamics]]
- not controlling: [[docs/biology/processes|Processes]]

# Пов'язані документи

- `biology/cell.md`
- `biology/joint.md`
- `biology/communication.md`
- `biology/specialization.md`
- `biology/lifecycle.md`
- `genetics/genome-runtime.md`
- `genetics/heredity.md`
- `engine/ecs.md`
- `engine/scheduler.md`
- `docs/examples/biology-examples.md`
