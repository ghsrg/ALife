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
asymmetric inheritance
damage history
lifecycle state
position in Cell-Joint graph
```

---

# Observer Profile

Для аналізу можна рахувати:

```text
SpecializationProfile
├── material_profile
├── process_profile
├── signal_profile
├── joint_profile
├── energy_profile
├── epigenetic_profile
└── lifecycle_profile
```

Цей профіль не повинен напряму керувати клітиною.

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
- fixed neural/muscle/skin classes.

---

# Open Questions

- Які observer metrics потрібні для першого `SpecializationProfile`.
- Який мінімальний набір stateful Materials потрібен для signal-plastic behavior.
- Чи asymmetric inheritance входить у першу реалізацію або лише підтримується схемою.
- Як відрізняти тимчасовий стан від стабільної спеціалізації в debug UI.

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
