---
tags:
  - alife
  - canon
  - area/genetics
---

# genome-representation.md

> `Genome Representation` — структура спадкового регуляторного носія.

---

# Призначення

Genome описує спадкові регуляторні правила клітини. Він не є blueprint тіла, списком органів або поведінковим скриптом.

Поточна базова реалізація: `Genome as Direct Regulatory Graph`. Це не монолітне остаточне рішення; модель можна коригувати, якщо збережено локальність, матеріальну обґрунтованість і сумісність з runtime.

---

# Канонічні правила

- Genome регулює process priorities.
- Genome не виконує фізичну роботу.
- Genome не створює Resources, Materials або Energy напряму.
- Genome читає тільки normalized local inputs.
- Genome outputs проходять Feasibility Check.
- Genome не містить global body map, organism_id behavior або species behavior.
- Genome має фізичний carrier/cost через Materials або internal structure.

---

# Мінімальна форма

```text
Genome
├── inputs
├── regulatory_nodes
├── edges
├── outputs
├── mutation_parameters
└── validation_limits
```

`inputs` і `outputs` мають відповідати `genetics/regulatory-interface.md`.

---

# Future Compatibility

Модель повинна дозволяти:

- fragments;
- duplication;
- deletion;
- recombination;
- horizontal transfer;
- epigenetic modifiers;
- topology limits.

Ці механіки не обов'язково входять у першу реалізацію.

---

# Заборонено

Не вводити:

- hardcoded behavior script;
- full organism blueprint;
- direct access to world state;
- direct access to organism labels;
- free unlimited genome size;
- mutation operators без validation.

---

# Semantic Links

- represents: [[docs/biology/genome|Genome]]
- executed by: [[docs/genetics/genome-runtime|Genome Runtime]]
- structured as: [[docs/genetics/regulatory-network|Regulatory Network]]
- compared with: [[docs/research/genome-representation-options|Genome Representation Options]]

# Пов'язані документи

- `genetics/regulatory-network.md`
- `genetics/genome-runtime.md`
- `genetics/regulatory-interface.md`
- `genetics/mutation.md`
- `biology/feasibility.md`
