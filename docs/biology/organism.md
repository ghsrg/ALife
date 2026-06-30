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
- Connected component є кандидатом на organism-like structure, але не автоматично organism.
- Межа між `cluster`, `colony-like` і `organism-like` є континуумом.
- Ступінь інтеграції визначається dependency, resource flow, signal flow, specialization, repair coupling і reproduction coupling.
- `Organism identity` є аналітичною міткою, а не входом для Genome Runtime.
- Усі процеси виконуються клітинами, а не organism-level controller.
- `Organism death` означає структурний collapse, а не HP event.

---

# Мінімальна модель

Для першої реалізації достатньо:

```text
organism-like candidate = connected component of alive/decomposing Cells through active Joints
```

Observer може рахувати:

```text
cell_count
joint_count
average_joint_strength
resource_flow
signal_flow
specialization_diversity
dependency_score
lifetime
fragmentation_events
```

Ці метрики не повинні повертатися в поведінку клітини як глобальний control signal.

---

# Dependency

Dependency показує, наскільки клітина залежить від структури:

```text
resource_dependency
signal_dependency
mechanical_dependency
repair_dependency
reproduction_dependency
survival_without_structure_probability
```

Низька dependency ближча до colony-like structure. Висока dependency ближча до organism-like structure.

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

# Open Questions

- Чи organism detection у першій реалізації буде тільки connected component, чи component + dependency score.
- Які мінімальні dependency metrics потрібні для debug UI.
- Як відстежувати lineage organism-like structures при fragmentation, merge і collapse.
- Який мінімальний `OrganismView` потрібен для research metrics.

---

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
