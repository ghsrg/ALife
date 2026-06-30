---
tags:
  - alife
  - canon
  - area/genetics
---

# inheritance.md

> `Inheritance` — передача локального стану під час reproduction/division.

---

# Призначення

Inheritance описує, що саме переходить від parent cell до daughter cells. Поділ є partition локального стану, а не магічне копіювання.

---

# Канонічні правила

- Genome або його copy/fragments мають потрапити в дочірні клітини.
- Resources, Materials, Energy Buffer, Heat, damage, epigenetic state і runtime fragments partition-яться згідно з `biology/division-partition.md`.
- Energy Buffer при division розподіляється як локальний стан parent cell, а не transport між незалежними клітинами.
- Functional Genome copy потребує physical Genome carrier, створений з Resources, Materials або internal precursor fragments до partition.
- Energy powers copying, repair and organization, but does not create the physical Genome carrier.
- Epigenetic inheritance не змінює Genome.
- Asymmetric inheritance дозволена.
- Нежиттєздатні daughters дозволені.
- Joint не дублюється автоматично.

---

# Partitioned State

```text
resources
materials
energy_buffer
genome_state
epigenetic_state
runtime_state
heat/temperature
damage
internal_fragments
boundary materials
joint attachment context
```

---

# Validation

Після inheritance перевіряється:

```text
daughter has physical volume
capacity bounds respected
Genome state present or explicitly missing
functional Genome has physical carrier or explicitly valid inherited carrier
Energy Buffer within capacity
Materials/resources non-negative
Boundary viability evaluated
Joint references valid
```

Validation не повинна автоматично робити daughter життєздатною.

---

# Заборонено

Не вводити:

- guaranteed equal split;
- guaranteed viable offspring;
- direct Energy transfer between independent cells;
- automatic Joint duplication;
- automatic Genome repair;
- creating Genome copy using Energy without physical carrier matter;
- hidden species inheritance.

---

# Semantic Links

- transfers: [[docs/biology/genome|Genome]]
- occurs during: [[docs/biology/division-partition|Division Partition]]
- constrained by: [[docs/genetics/heredity|Heredity]]
- can include: [[docs/genetics/mutation|Mutation]]

# Пов'язані документи

- `biology/division-partition.md`
- `biology/lifecycle.md`
- `genetics/heredity.md`
- `genetics/mutation.md`
- `genetics/epigenetics.md`
- `docs/examples/genetics-examples.md`
