---
tags:
  - alife
  - canon
  - area/biology
---

# specialization.md

> `Specialization` — стабільні функціональні стани клітин, що виникають локально.

---

# Призначення

Specialization описує, як клітини з однаковим або схожим Genome можуть набувати різних стійких станів через різні локальні умови.

У рушії немає hardcoded `NeuronCell`, `MuscleCell`, `SkinCell`, `SensorCell`, `StorageCell`, `TransportCell`, `LeafCell` або `RootCell`.

---

# Канонічні правила

- Specialization не є hardcoded cell type.
- Genome задає регуляторні правила, а не список готових ролей.
- Роль клітини виникає з Materials, Epigenetic State, Joint context, local signals, lifecycle і selection.
- `SpecializationProfile` є observer/debug view, а не input для behavior.
- Однаковий Genome може давати різні стани через різні local inputs.
- Neural-like behavior дозволений тільки як `signal-plastic cell/material state`, без hardcoded neurons.
- Specialization is inferred from persistent material/process patterns, not from one Tick or one signal impulse.
- Temporary RuntimeState is not specialization.
- No cell may read its `SpecializationProfile` as behavior input.

---

# Джерела Specialization

```text
local Fields
Resource gradients
Material composition
Energy availability
Joint context
Signals
Pressure / Heat / contact
Epigenetic State
asymmetric inheritance (future explicit division rule only)
damage history
lifecycle state
position in Cell-Joint graph
```

---

# Observer Profile

Мінімальний observer-only профіль:

```text
SpecializationProfile
├── dominant_materials
├── dominant_processes
├── average_regulatory_outputs
├── signal_sensitivity_level
├── signal_conductivity_level
├── storage_ratio
├── repair_ratio
├── movement_ratio
├── boundary_ratio
├── joint_ratio
└── stability_ticks
```

Це не "тип клітини". Це observer/debug profile, який описує стійкий функціональний патерн.

Genome Runtime, Feasibility Check і Processes не читають `SpecializationProfile`.

---

# Signal-Plastic Materials

Мінімальні Material properties для signal-plastic behavior:

```text
signal_sensitivity
signal_storage
signal_conductivity
```

Мінімальні MaterialState поля:

```text
stored_signal
fatigue
conductivity_modifier
```

Цього достатньо, щоб Material міг накопичувати сигнал, втомлюватись, гасити або передавати імпульс без hardcoded neurons.

---

# Inheritance Boundary

У базовій division model:

```text
Resources / Materials / Energy -> noisy proportional split
Genome information -> copied to physical carrier
EpigeneticState -> reset або attenuated by explicit rule
```

Asymmetric inheritance підтримується схемою як future-compatible option, але не є базовою поведінкою і не має виникати неявно.

---

# Temporary vs Stable

Debug UI має відрізняти тимчасовий стан від стабільної спеціалізації:

```text
temporary state:
  short-lived changes in runtime_state or MaterialState

stable specialization:
  repeated process bias + stable material composition + persistence over N ticks
```

Мінімальні debug UI fields:

```text
state_now
profile_window
stability_ticks
profile_confidence
```

Клітина не стає `signal-plastic` після одного імпульсу. Такий label може з'являтися тільки після стабільно високих signal-related Materials/processes протягом заданого вікна.

---

# Role Labels

Допустимі observer labels:

```text
boundary-supporting
transport-like
signal-plastic
storage-like
repair-focused
energy-production-like
mechanical-support-like
reproduction-supporting
```

Це не класи рушія. Label означає поточний функціональний патерн, який може змінитися.

---

# Заборонено

Не вводити:

- hardcoded cell type;
- global role assignment;
- organism command to specialize;
- blueprint органу;
- species-specific role;
- `if position == X then role Y`;
- fixed neural/muscle/skin classes;
- implicit asymmetric inheritance;
- treating one Tick of RuntimeState as stable specialization.

---

# Semantic Links

- emerges from: [[docs/biology/genome|Genome]]
- emerges from: [[docs/world/materials|Materials]]
- influenced by: [[docs/biology/joint|Joint]]
- stabilized by: [[docs/genetics/epigenetics|Epigenetics]]
- selected by: [[docs/evolution/selection|Selection]]

# Пов'язані документи

- `biology/cell.md`
- `biology/joint.md`
- `biology/communication.md`
- `biology/lifecycle.md`
- `genetics/epigenetics.md`
- `genetics/genome-runtime.md`
- `world/materials.md`
- `docs/examples/biology-examples.md`
