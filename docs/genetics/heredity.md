---
tags:
  - alife
  - canon
  - area/genetics
---

# heredity.md

> `Heredity` — збереження властивостей lineage через спадкові носії.

---

# Призначення

Heredity описує, як lineage може зберігати здатність створювати певні клітинні стани або структури. Успадковується не готова форма organism, а правила і bias, які можуть відтворювати схожий development path.

---

# Канонічні правила

- Heredity базується на Genome, genetic fragments і частково inherited epigenetic state.
- Genome не містить exact body plan.
- Selection оцінює наслідки через survival і reproduction, а не через explicit fitness command.
- Organism-like traits можуть бути спадковими тільки через локальні правила клітин.
- Lineage metrics є observer-side і не керують поведінкою.

---

# Відмежування

```text
Inheritance  = передача стану під час reproduction/division
Mutation     = спадкова зміна Genome
Epigenetics  = регуляторний bias без зміни Genome
Learning     = runtime/material state без спадкової гарантії
Heredity     = довготривала повторюваність traits у lineage
```

---

# Metrics

Observer може рахувати:

```text
lineage_id
parent_id
generation
survival_time
offspring_count
genome_distance
trait_recurrence
structure_recurrence
```

Ці метрики не є input для Genome Runtime.

---

# Заборонено

Не вводити:

- species essence;
- hardcoded body inheritance;
- global fitness score read by cells;
- lineage command behavior;
- guaranteed trait preservation.

---

# Semantic Links

- defines continuity of: [[docs/biology/genome|Genome]]
- implemented through: [[docs/genetics/inheritance|Inheritance]]
- creates variation with: [[docs/genetics/mutation|Mutation]]
- exposed to: [[docs/evolution/selection|Selection]]

# Пов'язані документи

- `genetics/inheritance.md`
- `genetics/mutation.md`
- `genetics/epigenetics.md`
- `biology/lifecycle.md`
- `biology/organism.md`
