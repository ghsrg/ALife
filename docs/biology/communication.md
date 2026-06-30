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

Communication описує, як фізичний локальний scalar stimulus від однієї клітини або середовища стає локальним input для іншої клітини.

Це не мова, не command system і не global organism bus. Приймаюча клітина реагує тільки через власні Materials, RuntimeState, local context, Genome Runtime і Feasibility Check.

---

# Канонічні правила

- Communication є локальною.
- Кожен активний signal має фізичну або матеріальну основу.
- Signal є input, а не command.
- Signal has no semantic type in the base model.
- Receiver сам визначає реакцію через власний runtime.
- Active signal production має Energy/Resource/Material cost.
- Communication може згасати, шуміти, блокуватися або пошкоджуватися.
- Сигнал, створений у Tick N, читається іншими клітинами як external input на початку Tick N+1.
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
delay
frequency-like signals
genetic fragments
field-mediated effects
typed/vector signal models through ADR
```

---

# Signal Contract

Base communication uses one scalar signal type only:

```text
signal_level: 0.0..1.0
```

There are no typed signals such as `damage`, `food`, `pain`, `move`, `stress` or `resource_need`.

Signal meaning is not stored in the signal itself. Meaning emerges from the receiver's Materials, RuntimeState, local context and Genome Runtime.

Typed or vector signals are not part of the base model and require an explicit ADR.

If future work needs different chemical-like signals, they should be modeled as different Resource-like trace substances with physical properties, not as typed commands.

Мінімальний signal:

```text
Signal
├── channel
├── signal_level
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
├── signal_level
├── readable_from_tick
└── decay_rate
```

Якщо клітина не має матеріальної основи для сприйняття сигналу, input має бути `0.0` або `unavailable`. Для першої реалізації базовий default: `0.0`.

Signal may be transmitted through:

```text
contact
Joint
local material trace
```

Signal emitted during Tick N is not read immediately by another cell's decision in the same phase:

```text
signals emitted during Tick N become external input for receivers at the start of Tick N+1
```

At the beginning of Tick N+1, each receiving cell samples local signal sources and converts them into local `runtime_state.signal_state`.

---

# State Layers

Повторні сигнали можуть змінювати різні шари:

```text
runtime_state.signal_state
  short-term accumulated signal level, decay, threshold, cooldown

MaterialState
  physical adaptation: conductivity, fatigue, stored signal, sensitivity, contraction state

JointState
  signal currently passing through or stored in a Joint, with delay/decay

EpigeneticState
  not used for base signal accumulation; only future long-term regulatory adaptation
```

Base model uses RuntimeState for short-term signal accumulation, MaterialState for physical signal-sensitive/plastic material changes and JointState for signal propagation through joints.

Ordinary signals do not modify EpigeneticState in the base model.

Ці шари не слід змішувати з mutation. Runtime, MaterialState і JointState зміни не переписують Genome.

---

# Material Trace

Communication trace must not be information in the air.

If trace affects cells, it must be physical:

```text
signal_trace_resource
├── amount
├── location
├── diffusion_rate
├── decay_rate
└── signal_effect_strength
```

Cells can sense `signal_trace_resource` only through local sampling and signal-sensitive Materials.

Different chemical-like signal traces must be represented as different Resource-like trace substances with their own diffusion, decay, reactivity and sensing constraints.

If trace is debug-only, it must not be readable by cells.

---

# Debug Communication Trace

Engine may log observer-only communication trace:

```text
source_cell
target_cell or local area
medium: contact | joint | trace
signal_value
tick_emitted
tick_received
distance
decay_applied
receiver_signal_state_before
receiver_signal_state_after
```

This log is observer-only. Genome Runtime, Feasibility and Processes must not read it.

---

# Заборонено

Не вводити:

- global message bus;
- direct command from one cell to another;
- hardcoded nervous/hormone/immune system;
- hardcoded species marker;
- typed semantic signals;
- signal without carrier;
- free active signal;
- direct Energy Buffer communication;
- same-tick infinite feedback loop;
- debug trace readable by cells.

---

# Invariant

```text
Signal is not a command.
Signal has no semantic type in the base model.
Signal is a scalar physical stimulus.
Its effect depends on receiver Materials, RuntimeState, JointState, local context and Tick/Scheduler rules.
Debug traces describe communication; they do not participate in behavior.
```

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
