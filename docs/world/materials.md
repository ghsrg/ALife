# materials.md

> `Material` — структурна або функціональна основа клітин, Boundary і Joint.

---

# Призначення

Material визначає physical/function capabilities. Він не є поведінкою і не є готовою роллю клітини.

---

# Канонічні правила

- Materials займають capacity.
- Materials можуть надавати capabilities для processes.
- Materials можуть деградувати, ремонтуватися, синтезуватися або розпадатися.
- Boundary і Joint мають Material basis.
- Stateful Materials можуть підтримувати signal-plastic behavior.
- Genome регулює використання/синтез Materials, але не створює capability без Material.
- Material capabilities require proper biological context.
- External MaterialFragment is matter with material identity, not active Cell material.

---

# Мінімальні Властивості

```text
id
volume
stability
strength
permeability
energy_capacity
capabilities
decay_rate
repair_requirements
reaction_profile
```

---

# MaterialFragment

```text
Material inside Cell      -> functional cell material
MaterialFragment outside  -> physical remains / structure / debris
Resource                  -> movable or consumable substance
```

MaterialFragment may retain:

```text
material_id
amount
location
mass / volume
stability
damage state
decay_rate
remaining structural properties
```

Outside a Cell, MaterialFragment does not provide active cell capabilities such as repair, energy conversion, signal processing, genome execution, movement, controlled transport or reproduction.

Context-specific capability interpretation:

```text
inside living Cell  -> may enable processes
inside Joint        -> may enable joint behavior
outside Cell        -> passive physical/material properties only
```

A MaterialFragment becomes Resource only through explicit degradation, reaction or conversion rules.

---

# Заборонено

Не вводити:

- Material as hardcoded behavior;
- capability without material basis;
- indestructible material by default;
- free repair;
- direct species marker.
- active cell capability from external MaterialFragment without Cell context.

---

# Пов'язані документи

- `biology/process-capabilities.md`
- `biology/cell.md`
- `biology/membrane.md`
- `biology/joint.md`
- `docs/config/materials_config.md`
