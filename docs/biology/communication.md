# communication.md

> **Communication — локальна передача сигналів і впливів між клітинами**

---

# Призначення

Цей документ описує `Communication` — способи, якими клітини можуть впливати одна на одну без глобального контролера.

Communication дозволяє клітинам:

* координувати процеси;
* реагувати на сусідів;
* підтримувати колонії;
* формувати tissue-like структури;
* передавати stress state;
* створювати signal chains;
* підтримувати specialization;
* будувати organism-like системи.

Communication не є мовою у людському сенсі.

Communication не є глобальною командною системою.

Communication — це локальна передача фізичних, хімічних, механічних або сигнальних впливів.

---

# Основна ідея

Клітина не має глобального знання про організм або світ.

Вона може реагувати лише на локальні входи.

```text
Cell A
    ↓
Signal / Resource / Heat / Force / Trace
    ↓
Cell B
```

Communication виникає тоді, коли один стан клітини або середовища стає входом для іншої клітини.

```text
Output of one cell
    ↓
Local medium or Joint
    ↓
Input of another cell
```

---

# Що Communication НЕ є

Communication не є:

* телепатією;
* глобальним bus організму;
* прямою командою;
* behavior script;
* hardcoded nervous system;
* hardcoded hormone system;
* hardcoded immune system;
* hardcoded species recognition;
* прямим управлінням іншою клітиною;
* способом передавати Energy Buffer напряму.

Клітина може створити сигнал.

Інша клітина може його сприйняти.

Але реакція іншої клітини визначається її власними Materials, Genome Runtime, Epigenetic State і Feasibility Check.

---

# Канали Communication

Communication може відбуватися через:

```text
Joint
Environment
Resource gradients
Material traces
Heat
Pressure
Contact
Mechanical force
Field interaction
Genetic fragments
```

Для MVP основні канали:

```text
Joint signal
Resource transfer
Material trace
Contact / Pressure
Heat
```

---

# Signal

`Signal` — це локальний вплив, який може бути сприйнятий клітиною і використаний як input для Genome Runtime або Epigenetic State.

Signal не є командою.

Signal не гарантує реакцію.

```text
Signal
    ↓
InputNode / Material sensitivity
    ↓
Genome Runtime
    ↓
Process Priority
```

---

# Signal має матеріальну основу

Signal не повинен бути абстрактним повідомленням без носія.

Він може існувати як:

* Resource;
* Material trace;
* Joint state;
* Heat difference;
* Pressure;
* mechanical deformation;
* local field influence;
* internal state transmitted through Joint.

Якщо сигнал не має фізичної або матеріальної основи, його не слід додавати в модель.

---

# Signal не є командою

Якщо Cell A передає сигнал Cell B, Cell A не керує Cell B.

```text
Cell A signal output
    ↓
Cell B input
    ↓
Cell B Genome Runtime
    ↓
Cell B decides its own priorities
```

Cell B може:

* відреагувати;
* ігнорувати;
* посилити сигнал;
* приглушити сигнал;
* перейти в dormancy;
* змінити Epigenetic State;
* створити інший сигнал.

Це залежить від Cell B.

---

# Material-dependent Communication

Клітина може сприймати лише ті сигнали, для яких має матеріальну основу.

Наприклад:

```text
Light signal requires light-sensitive Material.
Pressure signal requires pressure-sensitive Material.
Joint signal requires Joint-compatible signaling Material.
Chemical trace requires detection-capable Boundary Material.
```

Якщо Genome має InputNode для сигналу, але клітина не має відповідного Material, signal input повинен бути `0.0` або `unavailable`.

Для MVP краще:

```text
missing material basis -> signal input = 0.0
```

---

# Communication через Joint

Joint є прямим каналом communication між клітинами.

Через Joint можуть передаватися:

* scalar signal;
* typed signal;
* Resources;
* Heat;
* mechanical force;
* Pressure;
* local state indicators;
* future genetic fragments.

```text
Cell A
    ↓
Joint
    ↓
Cell B
```

Joint communication детальніше описується в `biology/joint.md`.

---

# Joint Signal

Joint Signal — це сигнал, який передається через Joint.

Мінімальна модель:

```text
JointSignal
├── value
├── direction
├── decay
└── source_cell
```

Майбутня розширена модель:

```text
JointSignal
├── type
├── strength
├── duration
├── delay
├── frequency
├── accumulation
├── direction
└── material_carrier
```

Для MVP достатньо одного або кількох числових сигналів.

---

# Direction

Сигнал через Joint може бути:

* одностороннім;
* двостороннім;
* асиметричним;
* пасивним;
* активним;
* затухаючим.

```text
Cell A -> Cell B
Cell A <-> Cell B
```

Direction може залежати від:

* Joint Materials;
* signal-conductivity;
* регуляції клітин;
* damage Joint;
* Energy, якщо сигнал активний.

---

# Decay

Сигнал не повинен бути вічним.

Він може згасати:

```text
signal_next =
signal_current × (1 - decay_rate)
```

Decay залежить від:

* signal type;
* Joint Materials;
* Heat;
* damage;
* distance;
* medium;
* cell state.

---

# Delay

Сигнал може передаватися із затримкою.

Delay може виникати через:

* довжину Joint;
* Material conductivity;
* signal type;
* damage;
* Energy state.

Для MVP delay можна відкласти.

Але модель повинна дозволяти додати його пізніше.

---

# Communication через Resources

Клітини можуть комунікувати через виведення або споживання Resources.

Наприклад:

```text
Cell A exports Resource X
    ↓
Resource X accumulates nearby
    ↓
Cell B detects Resource X
```

Resource може бути:

* корисним;
* waste;
* reactive;
* signal-like;
* energy-rich;
* neutral.

Resource не є “повідомленням” сам по собі.

Він стає сигналом лише якщо інша клітина може його сприйняти або використати.

---

# Resource Gradient

Resource Gradient може створювати directional information.

```text
higher concentration nearby
    ↓
cell detects gradient
    ↓
movement or uptake priority changes
```

Клітина не бачить всю карту ресурсу.

Вона може відчувати лише локальну концентрацію або локальну різницю.

---

# Pheromone-like Trace

Pheromone-like Trace може виникати, коли клітина виділяє Resource або Material, який залишається в середовищі.

```text
Cell Material / Resource
    ↓
External Trace
    ↓
Local Signal
```

Це не hardcoded “pheromone system”.

Це звичайний матеріальний слід, який інші клітини можуть навчитися використовувати еволюційно.

---

# Material Trace

Material Trace виникає, коли Material клітини або Boundary поступово виходить у середовище або залишається після контакту.

Material Trace може:

* деградувати в Resources;
* бути сприйнятим іншими клітинами;
* вказувати на присутність певного типу клітин;
* бути корисним ресурсом;
* бути шкідливим через реакції;
* слугувати pathway signal.

У світі немає hardcoded `species_marker`.

Розпізнавання “свій/чужий” може виникати лише через матеріальні ознаки.

---

# Communication через Heat

Heat може бути каналом communication або впливу.

Одна клітина може:

* виробляти Heat;
* передавати Heat через контакт;
* передавати Heat через Joint;
* створювати локальну температурну зміну.

Інша клітина може реагувати на Heat, якщо має heat-sensitive Materials або відповідні Runtime inputs.

```text
Cell A high Heat
    ↓
local Heat transfer
    ↓
Cell B heat input
    ↓
stress / dormancy / repair response
```

Heat не є повідомленням з наміром.

Але він може стати сигналом.

---

# Communication через Pressure

Pressure або mechanical force може передавати інформацію про контакт, стискання, рух або деформацію.

```text
Cell A moves
    ↓
Joint stretch
    ↓
Cell B pressure input
```

Pressure може впливати на:

* movement;
* Joint repair;
* dormancy;
* specialization;
* material synthesis;
* mechanical adaptation.

Клітина може сприймати Pressure лише через відповідні Materials.

---

# Communication через Contact

Contact — прямий локальний канал між клітинами.

Через контакт можуть виникати:

* mechanical signal;
* Resource exchange;
* Boundary reaction;
* Joint creation;
* Heat transfer;
* Material trace transfer;
* future HGT.

Contact не означає автоматичної communication.

Потрібна матеріальна сумісність.

---

# Communication через Environment

Клітини можуть комунікувати непрямо через середовище.

```text
Cell A changes local environment
    ↓
Environment stores trace / gradient / Heat / Resource
    ↓
Cell B later detects it
```

Це дозволяє delayed communication без прямого контакту.

Приклади:

* Resource depletion;
* Resource enrichment;
* Heat pattern;
* Material trace;
* local pH-like reactive state;
* dead cell remains.

---

# Stigmergy

`Stigmergy` — це непряма координація через зміну середовища.

Наприклад:

```text
Cell A leaves trace.
Cell B follows trace.
Cell C reinforces trace.
```

У рушії не треба вводити окремий stigmergy-модуль.

Це природний наслідок:

* Material Trace;
* Resource Gradient;
* local environment state;
* cell sensing.

---

# Communication і Genome Runtime

Сигнали стають входами для Genome Runtime.

```text
Signal
    ↓
InputNode
    ↓
Regulatory Network
    ↓
Output Priorities
```

Genome не читає “повідомлення” як текст.

Він читає нормалізований локальний input.

Приклад:

```text
joint_signal = 0.7
heat_level = 0.4
resource_X_nearby = 0.8
```

---

# Communication і Epigenetics

Повторні або стабільні сигнали можуть змінювати Epigenetic State.

```text
Repeated Signal
    ↓
Epigenetic modifier
    ↓
Future Runtime changes
```

Це дозволяє клітині змінювати реакцію на середовище без mutation.

Наприклад:

* підвищити signal sensitivity;
* знизити activation threshold;
* перейти в dormancy;
* підтримувати specialization;
* змінити repair bias.

---

# Communication і Learning-like State

Learning-like behavior може виникати, якщо клітина має Materials, які змінюють свій стан після сигналів.

```text
Repeated Signal
    ↓
Stateful Material
    ↓
Changed gain / threshold / decay
    ↓
Different future response
```

Це не окрема система learning.

Це властивість Materials, Runtime State і Epigenetic State.

---

# Communication і Neural-like Networks

У рушії немає hardcoded nervous system.

Але neural-like network може виникнути, якщо є:

* signal-producing cells;
* signal-sensitive cells;
* signal-conductive Joint;
* stateful Materials;
* threshold activation;
* impulse accumulation;
* adaptive gain;
* Energy для підтримки сигналів.

```text
Cell A signal
    ↓
Joint
Cell B accumulates
    ↓
threshold crossed
    ↓
Cell B signal
    ↓
Cell C
```

Це не клас `Neuron`.

Це emergent signal-processing graph.

---

# Signal Accumulation

Клітина може накопичувати повторні сигнали.

```text
accumulated_signal_next =
accumulated_signal_current × (1 - decay_rate)
+ incoming_signal
```

Якщо accumulated signal перевищує threshold, клітина може змінити output priorities.

```text
accumulated_signal > threshold
    ↓
output_signal priority increases
```

Це може бути основою impulse-like behavior.

---

# Signal Gain

Signal Gain визначає, наскільки сильно клітина реагує на вхідний сигнал.

```text
effective_signal =
incoming_signal × signal_gain
```

Signal Gain може залежати від:

* Materials;
* Epigenetic State;
* previous signals;
* Energy;
* damage;
* Heat;
* Joint quality.

Signal Gain не повинен бути глобальним параметром без матеріальної основи.

---

# Threshold

Threshold визначає, коли сигнал стає достатньо сильним для реакції.

```text
if effective_signal >= threshold:
    response activated
```

Threshold може бути:

* спадковим параметром Genome;
* зміненим Epigenetic State;
* залежним від Materials;
* зміненим learning-like state;
* пошкодженим через damage.

---

# Impulse-like Communication

Impulse-like communication може виникати, коли сигнал:

* накопичується;
* проходить threshold;
* створює короткий output;
* затухає;
* передається через Joint.

```text
incoming_signal
    ↓
accumulation
    ↓
threshold
    ↓
impulse output
    ↓
decay / reset
```

Це не hardcoded nerve impulse.

Це загальна сигнальна динаміка.

---

# Signal Cost

Signal Production не повинна бути безкоштовною.

Вона може мати вартість:

* Energy;
* Resources;
* Material degradation;
* Heat;
* Joint maintenance;
* runtime cost;
* opportunity cost.

Сигнали без вартості можуть швидко зламати баланс симуляції.

---

# Signal Noise

Communication може мати noise.

Noise може виникати через:

* weak signal;
* damaged Joint;
* Heat;
* Resource diffusion;
* environment mixing;
* Material instability;
* distance;
* decay;
* random molecular-like effects.

Noise не повинен підміняти детермінізм.

За однакового seed noise має бути відтворюваним.

---

# Signal Conflict

Клітина може отримати суперечливі сигнали.

Наприклад:

```text
Joint A signal -> increase growth
Joint B signal -> increase dormancy
Heat signal -> increase stress
Resource signal -> increase uptake
```

Genome Runtime і Epigenetic State повинні об'єднати ці входи у process priorities.

Рушій не повинен сам вирішувати “правильну” реакцію.

---

# Communication і Specialization

Стабільні communication patterns можуть створювати specialization.

Наприклад:

```text
Outer cells receive pressure and light.
Inner cells receive resource signals from neighbors.
Connected cells receive repeated joint signals.
```

Однаковий Genome може дати різні клітинні стани через різні локальні сигнали.

```text
Same Genome
+
Different Communication Context
    ↓
Different Cell Specialization
```

---

# Communication і Organism

Organism-like behavior виникає, коли communication дозволяє графу клітин координуватися.

Організм не має глобального мозку за замовчуванням.

Він має:

```text
Cells
+
Joints
+
Signals
+
Resource flows
+
Epigenetic states
```

Глобальна поведінка виникає з локальної взаємодії.

---

# Communication і Colony

Колонія може мати слабку communication:

* Resource sharing;
* local traces;
* simple stress signals;
* Heat sharing;
* mechanical contact.

Цього достатньо для простих колективних структур без складної спеціалізації.

---

# Communication і Tissue-like Structure

Tissue-like structure може виникнути, якщо група клітин має:

* стабільні Joint;
* схожі Materials;
* спільні сигнали;
* локальний flow Resources;
* схожий Epigenetic State;
* механічну роль.

Communication підтримує цю стабільність.

Але tissue не є hardcoded класом.

---

# Communication і Organ-like Structure

Organ-like structure може виникнути, якщо communication і specialization створюють функціональний підграф.

Приклади:

* signal-processing cluster;
* resource-transport cluster;
* boundary-supporting layer;
* movement-supporting structure;
* storage cluster.

Organ-like structure не задається рушієм.

Вона виникає як стабільний патерн.

---

# Communication і Death

Смерть клітини може створювати communication-like ефекти:

* release Resources;
* release Material traces;
* release Heat;
* break Joint;
* stop signals;
* create stress in connected cells;
* release genetic fragments.

Смерть не є повідомленням, але її наслідки можуть бути сприйняті іншими клітинами.

---

# Communication і HGT

Horizontal Transfer може розглядатися як особливий канал передачі genetic fragments.

Але genetic fragment не є звичайним signal.

Він може змінити спадковий стан клітини, якщо буде інтегрований і успадкований.

HGT описується окремо в `genetics/horizontal-transfer.md`.

---

# Communication у Tick

Communication повинна бути узгоджена з Tick.

Базовий порядок:

```text
Environment Update:
  passive signal diffusion
  trace decay
  heat transfer
  resource diffusion

Cell Decision:
  cells read signals
  Genome Runtime computes priorities

Action Execution:
  cells produce signals
  cells transfer resources
  joints update
  traces are emitted
```

Сигнал, створений у Action Execution Tick N, повинен впливати на інші клітини не раніше наступного стабільного reading phase, якщо не визначено інше.

Це зберігає causality.

---

# Double Buffering

Communication повинна використовувати snapshot.

```text
Read:
  signals_after_environment_update

Write:
  signals_for_next_tick
```

Клітина не повинна в межах одного Tick нескінченно реагувати на реакцію іншої клітини.

Це запобігає same-tick feedback loops.

---

# Communication Trace

Для дебагу корисно зберігати Communication Trace.

Приклад:

```text
Tick 500

Cell A:
  produced joint_signal = 0.8

Joint 12:
  transferred signal = 0.6

Cell B:
  received joint_signal = 0.6
  repair_bias increased
```

Trace не повинен бути обов'язковим для production-режиму.

Але він критично важливий для аналізу emergent багатоклітинності.

---

# MVP Communication Model

Для MVP достатньо:

```text
Joint signal:
  scalar value
  decay
  direction

Resource-based signal:
  local resource concentration

Material trace:
  local trace concentration with decay

Heat:
  local heat value

Pressure/contact:
  local mechanical input
```

MVP не потребує:

* typed hormone system;
* real nervous system;
* electrical model;
* complex chemical signaling;
* long-range messaging;
* global organism bus.

---

# MVP Signal Data

Мінімальна структура:

```text
Signal
├── source_cell_id
├── target_cell_id or local_position
├── channel
├── value
├── decay_rate
├── created_tick
└── active
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

Для trace:

```text
Trace
├── position
├── material_or_resource_type
├── concentration
├── decay_rate
└── diffusion_rate
```

---

# MVP Communication Update

Кожен Tick:

```text
1. Decay existing signals.
2. Diffuse environment traces.
3. Transfer Joint signals.
4. Compute local signal inputs for cells.
5. Cells use signals during Genome Runtime.
6. Cells may emit new signals during Action Execution.
7. New signals become readable in next Tick.
```

---

# Приклад 1. Joint stress signal

```text
Cell A:
  boundary damaged

Genome Runtime:
  output_signal = 0.8

Joint:
  transfers signal to Cell B

Cell B:
  receives joint_signal = 0.6
  increases repair_bias
  decreases growth_bias
```

Cell A не наказує Cell B ремонтуватися.

Cell B реагує через власну регуляцію.

---

# Приклад 2. Trace following

```text
Cell A moves and leaks Material Trace.

Environment:
  trace remains for several Tick

Cell B:
  detects trace gradient
  movement priority changes

Result:
  Cell B may follow trace
```

Це не hardcoded pathfinding.

Це локальна реакція на матеріальний слід.

---

# Приклад 3. Heat warning

```text
Cell A overheats.

Heat transfers through Joint and contact.

Cell B:
  heat_input increases
  dormancy_bias increases
  synthesis decreases
```

Heat не є повідомленням з наміром.

Але він працює як локальний сигнал.

---

# Приклад 4. Signal chain

```text
Cell A receives pressure.
Cell A sends signal through Joint.
Cell B accumulates signal.
Cell B crosses threshold.
Cell B sends signal to Cell C.
```

Так може виникнути signal pathway.

Це не hardcoded нервова система.

---

# Приклад 5. Conflicting signals

```text
Cell receives:
  joint_signal_growth = 0.8
  heat_stress = 0.7
  resource_signal = 0.5

Genome Runtime output:
  growth = 0.2
  repair = 0.6
  dormancy = 0.4
```

Реакція залежить від регуляторної мережі клітини.

---

# Приклад 6. Tissue-like coordination

```text
Outer cells receive pressure.
Outer cells send joint signals inward.
Inner cells reduce growth and increase repair.
Cluster remains stable under compression.
```

Це може стати основою tissue-like behavior.

---

# Приклад 7. Communication failure

```text
Joint damaged.
Signal decay increases.
Cell B receives weak signal.

Result:
  Cell B does not activate repair.
  Structure destabilizes.
```

Communication failure є фізичним наслідком пошкодженого каналу.

---

# Правила

## Rule 1. Communication is local

Communication працює через локальні канали, а не глобальну пам'ять світу.

## Rule 2. Communication is material-grounded

Кожен сигнал має фізичну або матеріальну основу.

## Rule 3. Signal is not command

Сигнал є входом для клітини, а не наказом.

## Rule 4. Receiver decides through its own runtime

Реакція приймаючої клітини визначається її Genome Runtime, Materials, Epigenetic State і Feasibility Check.

## Rule 5. Joint is a communication channel

Joint може передавати Signals, Resources, Heat і Force.

## Rule 6. Environment can store traces

Середовище може зберігати локальні сліди, gradients і Heat.

## Rule 7. Communication has cost

Створення, передача і підтримка сигналів можуть мати Energy, Resource або Material cost.

## Rule 8. Communication may fail

Сигнали можуть згасати, шуміти, блокуватися або пошкоджуватися.

## Rule 9. No global organism bus

Багатоклітинна координація не повинна будуватися на глобальному каналі.

## Rule 10. Communication enables emergence

Колонії, tissue-like і organ-like структури виникають через локальну communication.

---

# Заборонено

Не вводити:

* global message bus;
* direct command from one cell to another;
* hardcoded nervous system;
* hardcoded hormone system;
* hardcoded immune signal;
* hardcoded species marker;
* signal without carrier;
* signal without cost if active;
* direct Energy Buffer communication;
* same-tick infinite feedback loop.

---

# Пов'язані документи

* `biology/cell.md`
* `biology/joint.md`
* `biology/specialization.md`
* `biology/organism.md`
* `biology/processes.md`
* `biology/membrane.md`
* `genetics/genome-runtime.md`
* `genetics/epigenetics.md`
* `world/materials.md`
* `world/resources.md`
* `world/energy.md`
* `world/physics.md`
* `world/tick.md`
* `engine/scheduler.md`

---

# ADR

Потрібні ADR:

```text
ADR-000X: Communication Is Signal Input, Not Command
ADR-000X: No Global Organism Bus
ADR-000X: Signals Must Be Material-grounded
ADR-000X: Neural-like Communication Is Emergent
```

---

# Open Questions

## Signal typing

Потрібно вирішити, чи MVP має:

* один scalar signal;
* кілька typed signals;
* resource/material-based signals only;
* hybrid model.

## Signal delay

Потрібно вирішити, чи delay входить у MVP.

## Signal accumulation

Потрібно визначити, чи accumulation зберігається в:

* Runtime State;
* Epigenetic State;
* Stateful Materials;
* Joint state.

## Signal cost

Потрібно визначити Energy/Resource cost для active signal production.

## Trace model

Потрібно визначити, чи Material Trace буде:

* окремим об'єктом;
* Resource-like concentration;
* Material fragment;
* temporary field-like layer.

## Communication through dead cells

Потрібно вирішити, чи мертві клітини можуть тимчасово проводити Heat, Resources або Signals через залишкові Joint.

## Same-tick effects

Потрібно уточнити, чи будь-який сигнал, створений у Tick N, читається лише в Tick N+1.

## Debug trace

Потрібно визначити мінімальний Communication Trace для аналізу.
