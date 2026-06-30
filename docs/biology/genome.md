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

# Minimal Carrier State

Genome in the first implementation is:

```text
Genome information + physical carrier state
```

Minimum carrier state:

```text
genome_carrier
├── integrity: 0.0..1.0
├── amount
├── material_id
└── copy_progress
```

Genome information is stored on a physical carrier. The carrier occupies capacity, has integrity and can be damaged.

---

# Carrier Damage

```text
genome_damage = degradation of genome carrier
```

Genome carrier damage is tracked as `genome_carrier_state`, not as mutation of the regulatory graph.

Carrier damage may cause:

- runtime errors;
- reduced regulatory stability;
- copy errors;
- mutation during explicit repair/copying mechanisms;
- nonfunctional Genome.

Carrier damage is not automatically mutation.

Mutation, repair, copying and HGT are explicit mechanisms.

---

# Genome Trace

Minimum observer-only Genome Trace:

```text
genome_trace
├── genome_id
├── parent_genome_id
├── carrier_integrity
├── copy_progress
├── mutation_events
├── repair_events
├── copy_errors
├── hgt_events
└── division_copy_result
```

Genome Trace is observer-only. Cells, Genome Runtime, Feasibility Check and Processes must not read Genome Trace as behavior input.

---

# Future Carrier Extensions

Future models may add:

- fragments;
- multiple genome segments;
- mobile genetic elements;
- HGT carriers;
- recombination;
- spatial genome organization.

Base model requires only one physical carrier state with integrity/copy/damage tracking.

---

# Invariant

```text
Genome is information stored on a physical carrier.
Carrier damage is not automatically mutation.
Mutation, repair, copying and HGT are explicit mechanisms.
Genome Trace is observer-only.
```

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
