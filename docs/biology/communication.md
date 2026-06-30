---
tags:
  - alife
  - canon
  - area/biology
---

# communication.md

> `Communication` — локальна передача сигналів і впливів між клітинами.

---

# Призначення

Communication описує, як стан однієї клітини або середовища стає локальним input для іншої клітини.

Це не мова, не command system і не global organism bus. Приймаюча клітина реагує тільки через власні Materials, Genome Runtime, Epigenetic State і Feasibility Check.

---

# Канонічні правила

- Communication є локальною.
- Кожен активний signal має фізичну або матеріальну основу.
- Signal є input, а не command.
- Receiver сам визначає реакцію через власний runtime.
- Active signal production має Energy/Resource/Material cost.
- Communication може згасати, шуміти, блокуватися або пошкоджуватися.
- Сигнал, створений у Tick N, читається іншими клітинами не раніше стабільної reading phase Tick N+1, якщо виняток не описаний явно.
- Same-tick feedback loops заборонені.

---

# Канали

Мінімальні канали:

```text
Joint signal
Resource concentration / gradient
Material trace
Heat
Pressure / contact
Mechanical force
```

Future-compatible канали:

```text
typed signals
delay
frequency-like signals
genetic fragments
field-mediated effects
```

---

# Signal Contract

Мінімальний signal:

```text
Signal
├── channel
├── value
├── carrier
├── decay_rate
├── source
└── readable_from_tick
```

Для Joint:

```text
JointSignal
├── joint_id
├── source_cell_id
├── direction
├── value
└── decay_rate
```

Якщо клітина не має матеріальної основи для сприйняття сигналу, input має бути `0.0` або `unavailable`. Для першої реалізації базовий default: `0.0`.

---

# State Layers

Повторні сигнали можуть змінювати різні шари:

```text
Runtime State        коротке накопичення, thresholds, cooldowns
Epigenetic State     довший bias без зміни Genome
Stateful Materials   фізичний стан матеріалу
Joint State          локальна провідність, damage, signal residue
```

Ці шари не слід змішувати з mutation. Epigenetic і runtime зміни не переписують Genome.

---

# Заборонено

Не вводити:

- global message bus;
- direct command from one cell to another;
- hardcoded nervous/hormone/immune system;
- hardcoded species marker;
- signal without carrier;
- free active signal;
- direct Energy Buffer communication;
- same-tick infinite feedback loop.

---

# Open Questions

- Чи перша реалізація підтримує тільки scalar signal або кілька typed signals.
- Чи signal delay відкладається повністю.
- Які state layers реалізуються першими: Runtime State, Epigenetic State, Stateful Materials або Joint State.
- Як саме моделюється Material Trace: Resource-like concentration, Material fragment або окремий trace object.
- Мінімальний формат Communication Trace для debug UI.

---

# Semantic Links

- uses: [[docs/biology/joint|Joint]]
- uses local: [[docs/world/fields|Fields]]
- affects inputs for: [[docs/genetics/genome-runtime|Genome Runtime]]
- can shape: [[docs/biology/specialization|Specialization]]

# Пов'язані документи

- `biology/cell.md`
- `biology/joint.md`
- `biology/specialization.md`
- `biology/processes.md`
- `biology/cell-state.md`
- `genetics/genome-runtime.md`
- `genetics/epigenetics.md`
- `world/materials.md`
- `world/resources.md`
- `world/tick-semantics.md`
- `engine/scheduler.md`
- `docs/examples/biology-examples.md`
