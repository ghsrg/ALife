# genome-runtime.md

> **Genome Runtime — виконання регуляторної мережі під час Cell Decision**

---

# Призначення

Цей документ описує `Genome Runtime` — процес виконання спадкової регуляторної мережі клітини під час фази `Cell Decision`.

`genetics/regulatory-network.md` описує структуру графа.

`genetics/genome-runtime.md` описує, як цей граф виконується в Tick.

Genome Runtime не змінює світ напряму.

Genome Runtime читає локальний стан клітини, обчислює регуляторні сигнали і передає їх у `Action Planning`.

Нормативний інтерфейс входів, виходів, output scale, forbidden inputs і runtime limits описаний у `genetics/regulatory-interface.md`.

---

# Основна ідея

Під час кожного Tick клітина:

```text
1. Читає локальний стан
2. Нормалізує входи
3. Застосовує epigenetic modifiers
4. Виконує Regulatory Network
5. Отримує output priorities
6. Передає їх у Action Planning
7. Процеси проходять Feasibility Check
```

Загальна схема:

```text
Cell State
    ↓
Input Collection
    ↓
Input Normalization
    ↓
Epigenetic Modifiers
    ↓
Regulatory Network Execution
    ↓
Output Priorities
    ↓
Action Planning
    ↓
Feasibility Check
    ↓
Process Execution
```

---

# Що Genome Runtime НЕ є

Genome Runtime не є:

* фізичною дією;
* поведінковим деревом;
* мозком;
* симуляцією всього організму;
* прямим запуском процесів;
* прямим створенням Materials;
* прямим створенням Energy;
* прямим читанням глобального світу;
* місцем мутацій;
* місцем selection.

Genome Runtime лише обчислює регуляторний вихід.

---

# Місце в Tick

Genome Runtime виконується під час фази `Cell Decision`.

```text
Environment Update
    ↓
Cell Decision
    ├── Read local environment
    ├── Read internal state
    ├── Genome Runtime
    └── Build Action Plan
    ↓
Action Execution
```

Genome Runtime не повинен змінювати стан світу під час `Cell Decision`.

Він лише створює план або пріоритети.

---

# Вхідні дані Runtime

Genome Runtime отримує локальні дані клітини.

Приклади:

```text
energy_level
energy_capacity
free_capacity
resource_concentrations
material_levels
boundary_integrity
local_heat
local_pressure
damage_level
field_inputs
joint_signals
contact_signals
epigenetic_state
lifecycle_state
cell_age
division_readiness
```

Ці дані вже повинні бути доступні клітині після `Environment Update`.

---

# Локальність

Genome Runtime не має права читати:

* глобальну карту світу;
* список усіх клітин;
* координати всіх ресурсів;
* майбутні події;
* fitness score;
* species id;
* organism-level command;
* результат selection.

Genome Runtime працює лише з локальним станом клітини та її найближчого середовища.

---

# Input Collection

`Input Collection` — це збір значень для `InputNode`.

Приклад:

```text
InputNode: energy_level
Source: cell.energy_buffer.current / cell.energy_buffer.capacity
```

```text
InputNode: boundary_integrity
Source: cell.boundary.integrity
```

```text
InputNode: joint_signal_A
Source: aggregated signal from connected Joint
```

Якщо вхід недоступний через відсутність відповідного Material, його значення повинно бути:

```text
0.0
```

або спеціальний стан:

```text
unavailable
```

Для базової моделі простіше використовувати `0.0`.

---

# Material-dependent Inputs

Не всі входи автоматично доступні клітині.

Наприклад:

```text
Light input requires light-sensitive material.
Pressure input requires pressure-sensitive material.
Resource detection requires compatible boundary or sensor-like material.
Joint signal requires Joint.
```

Якщо Genome має InputNode для Light, але клітина не має light-sensitive Material, цей вхід не повинен давати корисний сигнал.

```text
Genome has field_light input
Cell has no light-sensitive Material
Runtime value = 0.0
```

Це важливо, щоб Genome не отримував магічні сенсори без матеріальної основи.

---

# Input Normalization

Перед виконанням графа всі входи нормалізуються.

Базовий діапазон:

```text
0.0 .. 1.0
```

Приклади:

```text
energy_level =
current_energy / energy_capacity
```

```text
free_capacity =
free_capacity / total_capacity
```

```text
boundary_integrity =
current_boundary_material / required_boundary_material
```

```text
resource_A_inside =
normalize(resource_amount)
```

Нормалізація потрібна для стабільності роботи графа.

---

# Clamp

Після нормалізації значення бажано обмежувати:

```text
value = clamp(value, 0.0, 1.0)
```

Це запобігає неконтрольованому зростанню сигналів.

Для деяких входів у майбутньому можна дозволити діапазон:

```text
-1.0 .. 1.0
```

але базова модель краще тримати простим.

---

# Epigenetic Modifiers

Epigenetic State може змінювати виконання Regulatory Network без зміни Genome.

Він може впливати на:

* effective input value;
* effective edge weight;
* effective node bias;
* effective threshold;
* output priority;
* доступність певних процесів;
* dormancy mode;
* stress response.

```text
Inherited parameter
    +
Epigenetic modifier
    ↓
Effective runtime parameter
```

Genome при цьому не змінюється.

---

# Effective Parameters

Потрібно розрізняти спадкові параметри й runtime-параметри.

Приклад:

```text
Genome weight = 0.8
Epigenetic modifier = -0.3
Effective weight = 0.5
```

```text
Genome threshold = 0.6
Stress modifier = -0.2
Effective threshold = 0.4
```

`effective_weight` або `effective_threshold` не успадковуються напряму.

Вони існують тільки під час виконання.

---

# Runtime State

Деякі вузли можуть мати стан виконання.

Приклади:

```text
accumulated_signal
previous_activation
decay_state
delayed_signal
oscillation_phase
```

Runtime State належить клітині.

Він не є Genome.

Genome може описувати правила такого стану, наприклад:

```text
decay_rate
statefulness
memory_stability
```

Але конкретне накопичене значення є набутим станом клітини.

---

# Stateful Nodes

`Stateful Node` може накопичувати сигнал між Tick.

Приклад:

```text
accumulated_signal_next =
accumulated_signal_current * (1 - decay_rate)
+ input_signal
```

Це дозволяє клітині реагувати на історію сигналів.

```text
Repeated weak signal
    ↓
Accumulation
    ↓
Threshold reached
    ↓
Output activated
```

Для базової моделі stateful nodes можна відкласти, але модель повинна не блокувати їх появу.

---

# Learning-like Runtime State

Learning-like behavior може виникати, якщо Runtime State або Material State змінює майбутню реакцію клітини.

Приклад:

```text
Repeated Signal
    ↓
Stateful Material
    ↓
Changed coefficient
    ↓
Different future response
```

Це не mutation.

Це не переписування Genome.

Це зміна стану клітини або матеріалу.

Genome може визначати здатність до такого стану, але сам набутий стан не є спадковою зміною Genome.

---

# Execution Modes

Genome Runtime може виконувати граф різними способами.

Можливі режими:

```text
DAG execution
fixed-step recurrent execution
event-like propagation
```

Для базової моделі краще вибрати один простий режим.

---

# Базовий Execution Mode

Рекомендований простий режим:

```text
DAG execution
```

Обмеження:

* граф без циклів;
* topological order;
* один прохід;
* deterministic result;
* прості activation functions;
* outputs у діапазоні `0.0 .. 1.0`.

Це найпростіше тестувати й дебажити.

---

# DAG Execution

Якщо Regulatory Network є DAG, вузли виконуються в топологічному порядку.

```text
InputNodes
    ↓
RegulatoryNodes
    ↓
OutputNodes
```

Алгоритм:

```text
1. Set InputNode values
2. Sort graph topologically
3. For each node:
     collect incoming values
     compute weighted sum
     apply activation function
4. Read OutputNode values
```

---

# Cyclic Execution

Цикли можуть бути корисними, але складнішими.

Вони дозволяють:

* пам'ять;
* осциляції;
* стабільні стани;
* delayed response;
* recurrent regulation.

Якщо цикли дозволені, Runtime повинен мати обмеження:

```text
max_runtime_steps
max_signal_value
convergence_threshold
deterministic update order
```

Для базової моделі цикли краще не вмикати або дозволити тільки через явно обмежену кількість кроків.

Якщо цикли дозволені, вони повинні виконуватися fixed-step recurrent mode з малим фіксованим `runtime_steps`, без convergence-loop всередині Tick.

---

# Fixed-step Recurrent Execution

Можлива майбутня модель:

```text
for step in 1..N:
    compute next activation for all nodes
    apply clamp
```

Приклад:

```text
runtime_steps = 3
```

Це дозволяє циклам проявитися, але не створює нескінченного виконання.

---

# Node Computation

Базова формула вузла:

```text
weighted_sum =
Σ(source_activation × edge_weight) + bias
```

```text
node_activation =
activation_function(weighted_sum)
```

Після цього:

```text
node_activation = clamp(node_activation, min_value, max_value)
```

Для базової моделі:

```text
min_value = 0.0
max_value = 1.0
```

---

# Activation Functions

Мінімальний набір Для базової моделі:

```text
linear_clamped
threshold
sigmoid
```

Інші функції потребують окремого рішення, бо ускладнюють мутації й аналіз.

Можливий future-набір:

```text
sigmoid
tanh
relu
step
```

Для початку краще не робити багато функцій, бо це ускладнить мутації й аналіз.

---

# linear_clamp

`linear_clamp` просто обмежує значення:

```text
activation = clamp(weighted_sum, 0.0, 1.0)
```

Переваги:

* проста;
* швидка;
* зрозуміла;
* легко дебажити.

Недоліки:

* менш виразна;
* може давати грубі реакції.

---

# threshold

`threshold` активує вузол лише після порогу.

```text
if weighted_sum >= threshold:
    activation = 1.0
else:
    activation = 0.0
```

Або м'якший варіант:

```text
activation = clamp((weighted_sum - threshold) * gain, 0.0, 1.0)
```

Threshold корисний для:

* division readiness;
* dormancy;
* repair trigger;
* reaction to Heat;
* signal firing.

---

# Output Priorities

Після виконання графа OutputNodes повертають пріоритети процесів.

Приклад:

```text
produce_energy = 0.85
repair_boundary = 0.70
movement = 0.10
prepare_division = 0.00
enter_dormancy = 0.30
```

Ці значення не є діями.

Вони передаються в `Action Planning`.

---

# Priority Interpretation

Output priority можна інтерпретувати як:

* бажаний рівень процесу;
* силу запиту;
* частку доступного бюджету;
* порядок важливості;
* інтенсивність.

Для базової моделі краще трактувати priority як силу запиту:

```text
0.0 = процес не запитується
1.0 = максимальний запит процесу
```

Розподіл Energy і Resources між процесами виконується вже після Runtime.

---

# Action Planning

`Action Planning` перетворює output priorities на кандидатні процеси.

Приклад:

```text
Output:
  repair_boundary = 0.8
  produce_energy = 0.6
  move = 0.2

Action Plan:
  request repair_boundary
  request produce_energy
  request weak movement
```

Action Planning ще не гарантує виконання.

Після нього йде `Feasibility Check`.

---

# Feasibility Check

Feasibility Check перевіряє:

* чи є Energy;
* чи є Resources;
* чи є Materials;
* чи є free_capacity;
* чи дозволяє Boundary;
* чи дозволяє Physics;
* чи дозволяє lifecycle_state;
* чи існує потрібний Joint;
* чи не блокує Heat або Pressure.

Якщо процес неможливий, він:

* не виконується;
* переходить у failure mode;
* створює побічний ефект, якщо це визначено процесом.

Genome Runtime не вирішує це сам.

Якщо Energy недостатньо для planned action, action не виконується.

---

# Energy Cost of Runtime

Genome Runtime може мати Energy cost.

Можливі варіанти:

## Варіант A. Runtime без Energy cost У базовій моделі

Простіше для базової моделі.

Підходить, якщо потрібно швидко отримати живу симуляцію.

## Варіант B. Runtime має малий Energy cost

Більш фізично послідовно.

Вартість може залежати від:

* кількості вузлів;
* кількості ребер;
* кількості runtime steps;
* stateful nodes;
* activation functions.

Приклад:

```text
runtime_cost =
base_cost
+ node_count * node_cost
+ edge_count * edge_cost
+ runtime_steps * step_cost
```

Для базової моделі можна почати без cost, але зафіксувати, що більший Genome має вартість через copying, storage і mutation risk.

---

# Runtime Failure

Якщо Runtime не може виконатися, клітина не повинна отримувати магічний fallback.

Причини Runtime Failure:

* Genome сильно пошкоджений;
* відсутній Genome;
* critical Input/Output bindings broken;
* energy cost runtime enabled, але Energy недостатньо;
* graph invalid;
* node data corrupted.

Можливі наслідки:

* output priorities = 0;
* fallback to minimal maintenance;
* cell enters stressed state;
* cell enters dormant state;
* cell dies if maintenance impossible.

Для базової моделі краще мати просте правило:

```text
Invalid Genome Runtime -> no regulated active processes
```

Пасивні процеси продовжуються.

---

# Damaged Genome Runtime

Пошкоджений Genome може давати:

* відсутні виходи;
* неправильні пріоритети;
* постійно активні процеси;
* відсутність repair;
* неможливість division;
* хаотичну регуляцію;
* нежиттєздатну поведінку.

Це допустимо.

Selection відфільтрує погані варіанти.

---

# Missing Inputs

Якщо InputNode існує, але відповідний сигнал недоступний, Runtime повинен обробити це детерміновано.

Варіанти:

```text
value = 0.0
```

або:

```text
value = default_value
```

або:

```text
input disabled
```

Для базової моделі:

```text
missing input = 0.0
```

---

# Missing Outputs

Якщо OutputNode вказує на процес, якого клітина не може виконати через відсутність Material, це не Runtime error.

Приклад:

```text
output_move = 0.9
cell has no movement-capable material
```

Результат:

```text
movement process fails feasibility check
```

Output може бути активним, але дія не виконається.

---

# Material Gating

Матеріали можуть відкривати або закривати можливість читати входи й виконувати виходи.

```text
Genome has pressure_input
Cell has no pressure-sensitive material
Runtime pressure value = 0.0
```

```text
Genome outputs active_transport
Cell has no pump-capable material
Feasibility Check fails
```

Це дозволяє Genome мати потенційні регуляторні зв'язки, які проявляються лише після появи відповідних Materials.

---

# Epigenetic Gating

Epigenetic State може тимчасово приглушити або підсилити частину Runtime.

Приклади:

```text
dormancy_state reduces movement output
stress_state increases repair output
development_state boosts material_X synthesis
```

Це не змінює Genome.

Це змінює ефективне виконання Genome.

---

# Lifecycle Gating

Lifecycle State також може впливати на Runtime.

Наприклад:

```text
dead -> no Genome Runtime
decomposing -> no Genome Runtime
dormant -> reduced outputs
division_preparing -> division-related outputs boosted
stressed -> repair and maintenance boosted
```

Це не повинно перетворитися на hardcoded behavior tree.

Lifecycle Gating має бути обмеженим технічним шаром, який не підміняє Genome.

---

# Deterministic Execution Order

Для відтворюваності Runtime повинен мати стабільний порядок виконання.

Потрібно фіксувати:

* порядок вузлів;
* порядок ребер;
* порядок застосування modifiers;
* порядок rounding/clamp;
* seed для stochastic nodes;
* поведінку invalid graph.

Це важливо для тестів і аналізу.

---

# Parallel Execution

Genome Runtime може виконуватися паралельно для багатьох клітин.

Але результат не повинен залежати від порядку потоків.

Правило:

```text
Runtime reads state from current Tick snapshot.
Runtime writes only Action Plan for the cell.
Runtime does not mutate shared world state.
```

Це дозволяє безпечно паралелити Cell Decision phase.

---

# Double Buffering

Genome Runtime повинен читати стан попередньої стабільної фази.

```text
Read:
  world_state_tick_N_after_environment_update

Write:
  cell_action_plan_tick_N
```

Він не повинен читати результати Action Execution інших клітин у тому самому Tick.

Це відповідає принципу causality з `world/laws.md`.

---

# Runtime Output Format

Результат Runtime може бути структурою:

```text
GenomeRuntimeOutput
├── process_priorities
├── internal_runtime_state_updates
├── debug_values
└── errors
```

Для базової моделі достатньо:

```text
process_priorities
```

Але для дебагу бажано зберігати:

* input values;
* node activations;
* output values;
* applied epigenetic modifiers;
* failed bindings.

---

# Debugging

Genome Runtime повинен бути дебажним.

Для кожної клітини бажано мати можливість подивитися:

```text
Input values
Node activations
Edge contributions
Output priorities
Feasibility results
Executed processes
```

Це критично для розуміння emergent behavior.

Без такого дебагу буде важко зрозуміти, чому клітина померла або почала розмножуватися.

---

# Runtime Trace

Для наукового аналізу можна зберігати короткий Runtime Trace.

Приклад:

```text
Tick 120:
  energy_level = 0.2
  boundary_integrity = 0.4
  repair_boundary = 0.9
  move = 0.0
  divide = 0.0
  dormancy = 0.3
```

Trace не повинен бути обов'язковим для всіх клітин у production-режимі, бо це може бути дорого.

---

# Example 1. Energy Shortage

```text
Input:
  energy_level = 0.15
  resource_A_inside = 0.80
  heat_level = 0.20

Runtime:
  produce_energy increases
  movement decreases
  division decreases

Output:
  produce_energy = 0.90
  movement = 0.05
  prepare_division = 0.00
```

Після цього Feasibility Check перевіряє, чи клітина справді має Material для перетворення Resource A на Energy.

---

# Example 2. Boundary Damage

```text
Input:
  boundary_integrity = 0.30
  energy_level = 0.60
  resource_A_inside = 0.70

Runtime:
  repair_boundary activated
  growth suppressed
  division suppressed

Output:
  repair_boundary = 0.95
  grow = 0.10
  prepare_division = 0.00
```

Клітина переносить пріоритет із росту на ремонт.

---

# Example 3. Missing Material

```text
Genome output:
  move = 0.80

Cell:
  movement-capable Material = 0.00

Feasibility Check:
  movement impossible

Result:
  no movement
```

Це не Runtime error.

Це нормальна ситуація: Genome має регуляторний намір, але матеріальна база відсутня.

---

# Example 4. Dormancy

```text
Input:
  energy_level = 0.10
  resource_A_outside = 0.05
  heat_level = 0.70

Runtime:
  enter_dormancy increases
  synthesis decreases
  movement decreases
  maintenance remains medium

Output:
  enter_dormancy = 0.85
  synthesize_material = 0.05
  movement = 0.00
  maintenance = 0.50
```

Action Planning може перевести клітину в dormancy, якщо стан життєздатний.

---

# Example 5. Stateful Signal Accumulation

```text
Tick 1:
  joint_signal = 0.30
  accumulated_signal = 0.20

Tick 2:
  joint_signal = 0.30
  accumulated_signal = 0.45

Tick 3:
  joint_signal = 0.30
  accumulated_signal = 0.67

Threshold:
  0.60

Output:
  output_signal activated
```

Це може дати neural-like behavior без hardcoded NeuronCell.

---

# Example 6. Epigenetic Modifier

```text
Genome:
  repair_threshold = 0.60

Epigenetic State:
  stress_modifier = -0.20

Runtime:
  effective_repair_threshold = 0.40
```

Клітина у stressed state активує repair швидше.

Genome не змінився.

---

# Example 7. Damaged Genome

```text
Genome damage:
  edge to repair_boundary removed

Input:
  boundary_integrity = 0.20

Runtime Output:
  repair_boundary = 0.00

Result:
  cell fails to repair Boundary
  boundary degrades further
  cell may die
```

Це допустима еволюційна невдача.

---

# Runtime базової моделі

Для базової моделі рекомендується:

```text
Graph type: DAG
Execution: topological order
Runtime steps: 1
Input range: 0.0 .. 1.0
Output range: 0.0 .. 1.0
Activation functions: linear_clamp, threshold
Missing input: 0.0
Invalid output: ignored by Feasibility Check
Runtime energy cost: disabled or minimal
Runtime trace: optional debug mode
```

---

# Правила

## Rule 1. Runtime runs during Cell Decision

Genome Runtime виконується під час `Cell Decision`, а не під час `Action Execution`.

## Rule 2. Runtime reads only local state

Genome Runtime читає лише локальний стан клітини та її найближчого середовища.

## Rule 3. Runtime does not mutate world state

Genome Runtime не змінює Resources, Materials, Energy або Physics напряму.

## Rule 4. Runtime outputs priorities

Результатом Runtime є пріоритети процесів, а не виконані дії.

## Rule 5. Feasibility Check happens after Runtime

Будь-який output Runtime проходить через перевірку фізичної можливості.

## Rule 6. Runtime state is not Genome

Накопичений сигнал, тимчасова пам'ять або learning-like state не змінюють Genome напряму.

## Rule 7. Epigenetics modifies execution, not Genome

Epigenetic State змінює effective parameters Runtime, але не переписує спадкову структуру.

## Rule 8. Runtime must be deterministic

За однакових умов і seed результат Runtime має бути однаковим.

## Rule 9. Runtime must be parallel-safe

Genome Runtime не повинен залежати від порядку виконання клітин у потоках.

## Rule 10. Invalid Genome may produce bad regulation

Рушій не повинен забороняти поганий Genome. Selection відфільтрує нежиттєздатні варіанти.

---

# Заборонено

Не вводити:

* direct action execution inside Runtime;
* direct Resource creation;
* direct Material creation;
* direct Energy creation;
* global world access;
* species id input;
* organism command input;
* hidden fitness feedback;
* automatic survival correction;
* learning as direct Genome rewrite;
* nondeterministic runtime without seed;
* thread-order-dependent execution.

---

# Пов'язані документи

* `genetics/regulatory-interface.md`
* `genetics/regulatory-network.md`
* `genetics/mutation.md`
* `genetics/inheritance.md`
* `genetics/recombination.md`
* `genetics/horizontal-transfer.md`
* `genetics/epigenetics.md`
* `biology/genome.md`
* `biology/cell.md`
* `biology/processes.md`
* `biology/lifecycle.md`
* `world/tick.md`
* `world/laws.md`
* `engine/scheduler.md`
* `engine/serialization.md`

# Open Questions

## Cycles

Потрібно вирішити, чи перша реалізація починає з DAG-only або fixed-step recurrent mode з обмеженнями `genetics/regulatory-interface.md`.

## Stateful nodes

Потрібно визначити, чи базова модель підтримує накопичення сигналу у вузлах.

## Activation functions

Потрібно остаточно вибрати набір функцій:

```text
linear_clamped
threshold
sigmoid
```

## Epigenetic modifiers

Потрібно визначити, які саме параметри може змінювати epigenetic_state У базовій моделі.

## Debug trace

Потрібно визначити, який мінімальний Runtime Trace зберігати для дебагу.

## Parallel execution

Потрібно деталізувати правила scheduler для паралельного виконання Genome Runtime.


