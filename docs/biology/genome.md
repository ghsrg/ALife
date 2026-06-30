---
tags:
  - alife
  - canon
  - area/biology
---

# genome.md

> `Genome` — фізичний носій спадкової регуляції клітини.

---

# Призначення

Цей файл фіксує біологічний сенс Genome на рівні клітини.

Деталі структури, runtime, mutation і inheritance живуть у `docs/genetics/`.

---

# Канонічні правила

- Genome є спадковою регуляторною системою.
- Genome не є pure information.
- Genome регулює process priorities через Genome Runtime.
- Genome не є мозком, behavior script, body blueprint або source of Energy.
- Genome працює тільки з локальними inputs клітини.
- Genome output не є action і проходить Feasibility Check.
- Genome має фізичний carrier/cost і може копіюватися, пошкоджуватися, мутувати або partition-итися.
- Поточна базова реалізація: `Genome as Direct Regulatory Graph`.

---

# Physical Basis

Genome складається з:

```text
Genome information
physical genome carrier
genome runtime machinery
```

Genome information не є речовиною.

Physical genome carrier є фізичною структурою, створеною з Resources, Materials або internal precursor fragments.

Genome не є Material category на кшталт boundary/strength/movement material, але має physical carrier, зроблений з матерії.

Genome runtime machinery читає, копіює, ремонтує й виконує Genome Runtime. Ця machinery також має physical/material basis.

Genome copying не створюється Energy. Energy powers copying, repair and organization, але не є source of matter.

---

# Межа Відповідальності

```text
biology/genome.md
  meaning of Genome for Cell

genetics/genome-representation.md
  data shape and Direct Regulatory Graph

genetics/regulatory-network.md
  graph internals

genetics/genome-runtime.md
  evaluation in Tick

genetics/mutation.md
  hereditary changes

genetics/inheritance.md
  transfer during reproduction/division
```

---

# Заборонено

Не вводити:

- global world reads;
- species behavior;
- direct organism controller;
- hardcoded organs/tissues/cell types;
- direct matter or Energy creation;
- mutation that bypasses validation.

---

# Open Questions

- Який мінімальний physical carrier Genome потрібен у першій реалізації.
- Чи genome damage буде окремим state або частиною material/internal fragment model.
- Який мінімальний Genome Trace потрібен для debug.

---

# Semantic Links

- regulates: [[docs/biology/processes|Processes]]
- executed by: [[docs/genetics/genome-runtime|Genome Runtime]]
- represented by: [[docs/genetics/genome-representation|Genome Representation]]
- constrained by: [[docs/genetics/regulatory-interface|Regulatory Interface]]
- inherited through: [[docs/genetics/inheritance|Inheritance]]
- physically carried by: [[docs/world/materials|Materials]]

# Пов'язані документи

- `genetics/genome-representation.md`
- `genetics/regulatory-network.md`
- `genetics/genome-runtime.md`
- `genetics/regulatory-interface.md`
- `genetics/mutation.md`
- `genetics/inheritance.md`
- `biology/feasibility.md`
