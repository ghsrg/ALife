# ecs.md

> ECS — технічна організація стану та систем рушія.

---

# Призначення

ECS розділяє data components і systems, щоб симуляція була відтворюваною, тестованою і масштабованою.

ECS не вводить доменні закони. Він реалізує контракти з `world/*`, `biology/*` і `genetics/*`.

---

# Мінімальні Entities

```text
Cell
Joint
ResourceField / ResourceParcel
MaterialFragment
GenomeFragment
Trace
ObserverView
```

`Organism` не є поведінковою entity світу. `OrganismView` може бути observer-side derived view.

`MaterialFragment` є external physical remains with material identity. It is not active Cell material unless a process converts or incorporates it into a living Cell context.

---

# Канонічні правила

- Components зберігають стан, systems змінюють стан.
- Systems не читають майбутній стан Tick.
- Observer views read-only для simulation behavior.
- Порядок systems не повинен створювати приховані біологічні пріоритети.
- Domain validation має бути явною.

---

# Заборонено

Не вводити:

- hardcoded organism controller;
- species-specific systems;
- behavior у component constructors;
- global mutable shortcuts;
- observer metrics as simulation inputs.

---

# Пов'язані документи

- `engine/scheduler.md`
- `world/tick-semantics.md`
- `biology/cell.md`
- `biology/organism.md`
