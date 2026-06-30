# regulatory-network.md

> **Regulatory Network — графова модель спадкової регуляції**

---

# Призначення

Цей документ описує `Regulatory Network` — математичне представлення Genome як регуляторного графа.

`biology/genome.md` описує Genome як біологічну конструкцію клітини.

`genetics/regulatory-network.md` описує, як Genome може бути представлений формально:

```text
Inputs
    ↓
Regulatory Nodes
    ↓
Outputs
```

Regulatory Network не є поведінковим скриптом.

Regulatory Network не виконує фізичну роботу.

Regulatory Network обчислює регуляторні сигнали, які впливають на клітинні процеси.

---

# Основна ідея

Genome представлений як орієнтований граф.

```text
G = (V, E)
```

де:

* `V` — вузли регуляторної мережі;
* `E` — орієнтовані зв'язки між вузлами.

Граф перетворює локальні входи клітини на вихідні пріоритети процесів.

```text
Cell State
    ↓
Input Nodes
    ↓
Regulatory Nodes
    ↓
Output Nodes
    ↓
Process Priorities
```

---

# Що Regulatory Network НЕ є

Regulatory Network не є:

* нейромережею у класичному ML-сенсі;
* мозком;
* behavior tree;
* if/else-скриптом;
* списком дій;
* готовим планом виживання;
* списком органів;
* списком типів клітин;
* глобальним контролером організму.

Це спадкова структура регуляції.

Вона працює тільки з локальними входами клітини.

---

# Рівні моделі

У моделі є три основні рівні:

```text
Input Layer
Regulatory Layer
Output Layer
```

## Input Layer

Input Layer отримує локальні вимірювані стани клітини або середовища.

## Regulatory Layer

Regulatory Layer перетворює, комбінує, підсилює або пригнічує сигнали.

## Output Layer

Output Layer формує пріоритети клітинних процесів.

---

# Загальна схема

```text
[Energy Level] ─┐
[Heat Level] ───┼──> (Regulatory Node A) ──┐
[Resource A] ───┘                          │
                                           ├──> [Produce Energy Priority]
[Boundary] ─────> (Regulatory Node B) ─────┘

[Damage] ───────> (Regulatory Node C) ───────> [Repair Priority]
```

---

# Node Types

Regulatory Network може містити кілька типів вузлів.

Мінімальні типи Для базової моделі:

```text
InputNode
RegulatoryNode
OutputNode
```

Майбутні типи можуть бути додані пізніше, але вони не повинні ламати базову модель.

---

# InputNode

`InputNode` підключає конкретний локальний сигнал клітини до Genome Runtime.

Приклади InputNode:

```text
energy_level
free_capacity
resource_A_inside
resource_A_outside
material_X_amount
boundary_integrity
local_heat
local_pressure
joint_signal
contact_signal
field_light
damage_level
epigenetic_modifier
```

InputNode не має глобального знання про світ.

Він лише читає локальний стан клітини.

---

# Input Normalization

Входи повинні бути приведені до числового діапазону.

Базовий діапазон Для базової моделі:

```text
0.0 .. 1.0
```

Приклади:

```text
energy_level = current_energy / energy_capacity

free_capacity = free_capacity / total_capacity

boundary_integrity = boundary_material_current / boundary_material_required

heat_level = normalized(local_heat)

resource_A_inside = normalized(resource_amount)
```

Нормалізація потрібна, щоб Regulatory Network могла працювати стабільно для різних масштабів клітин.

---

# RegulatoryNode

`RegulatoryNode` — внутрішній вузол регуляції.

Він може:

* приймати кілька входів;
* комбінувати сигнали;
* підсилювати сигнал;
* пригнічувати сигнал;
* мати поріг активації;
* мати нелінійну функцію;
* мати затримку;
* мати внутрішній стан;
* бути мутованим;
* бути дубльованим;
* бути видаленим.

RegulatoryNode не є нейроном у біологічному сенсі.

Це формальний вузол спадкової регуляторної мережі.

---

# OutputNode

`OutputNode` відповідає за регуляторний пріоритет певного клітинного процесу.

Приклади OutputNode:

```text
uptake_resource_A
export_resource_B
produce_energy
synthesize_material_X
repair_boundary
repair_material_X
degrade_material_Y
move
create_joint
break_joint
copy_genome
prepare_division
enter_dormancy
increase_mutation_rate
allow_hgt_uptake
```

OutputNode не гарантує виконання процесу.

Він лише формує пріоритет.

Після цього процес проходить `Feasibility Check`.

---

# Edge

`Edge` — орієнтований зв'язок між двома вузлами.

```text
source_node -> target_node
```

Edge може мати параметри:

```text
weight
enabled
delay
stability
mutation_rate
```

Мінімально Для базової моделі достатньо:

```text
source
target
weight
```

---

# Weight

`weight` визначає силу впливу одного вузла на інший.

```text
positive weight  -> activation
negative weight  -> inhibition
zero weight      -> no effect
```

Приклад:

```text
energy_low -> produce_energy
weight = +0.8
```

```text
heat_high -> movement
weight = -0.6
```

---

# Activation

Кожен RegulatoryNode обчислює власне значення активації.

Базова схема:

```text
activation =
activation_function(
  Σ(input_value × edge_weight) + bias
)
```

Це не обов'язкова фінальна формула.

Це базовий принцип.

---

# Activation Function

Activation Function визначає, як вузол реагує на сумарний вхід.

Можливі варіанти:

```text
linear
sigmoid
tanh
relu
threshold
clamp
```

Для базової моделі краще почати з простих функцій:

```text
linear + clamp
```

або:

```text
sigmoid
```

---

# Bias

`bias` — базове зміщення вузла.

Він дозволяє вузлу бути активним навіть при слабких входах.

```text
node_activation =
activation_function(weighted_sum + bias)
```

Bias може мутувати.

---

# Threshold

`threshold` визначає, з якого рівня сигнал вважається активним.

Наприклад:

```text
if activation > threshold:
    output is active
```

Threshold може бути корисним для:

* запуску поділу;
* реакції на Heat;
* переходу в dormancy;
* активації repair;
* реакції на ресурс.

Threshold може мутувати.

---

# Delay

`delay` дозволяє сигналу впливати не миттєво, а через кілька Tick.

Це може дати:

* коливання;
* затримані реакції;
* ритмічну поведінку;
* повільну регуляцію.

Для базової моделі delay можна не реалізовувати.

Але модель повинна дозволяти його додати пізніше.

---

# Internal State

Деякі RegulatoryNode можуть мати внутрішній стан.

Наприклад:

```text
memory_value
accumulated_signal
decay_rate
```

Це дозволяє створювати просту регуляторну пам'ять.

Але треба відрізняти:

```text
Genome structure
```

від:

```text
Runtime state
```

Genome описує вузол.

Runtime зберігає поточний стан вузла.

---

# Runtime State не є Genome

Якщо вузол накопичує сигнал, це не означає, що Genome змінився.

```text
Genome:
  node has decay_rate = 0.1

Runtime:
  node.accumulated_signal = 0.73
```

`decay_rate` може бути спадковим.

`accumulated_signal` є станом клітини.

---

# Stateful Regulation

Stateful Regulation дозволяє клітині реагувати не лише на поточний стан, а й на історію сигналів.

Приклад:

```text
repeated_signal
    ↓
accumulated_state
    ↓
higher future response
```

Це близько до learning-like behavior, але не є окремою системою learning.

Це властивість регуляторної мережі та матеріального стану клітини.

---

# Зв'язок із learning-like materials

Regulatory Network може мати виходи, які впливають на stateful materials.

Наприклад:

```text
signal_input
    ↓
regulatory_node
    ↓
adjust_signal_gain_process
    ↓
material coefficient changes
```

Але сама зміна коефіцієнтів матеріалу не є зміною Genome.

Genome лише визначає здатність клітини створювати або регулювати такі матеріали.

---

# Output Value

OutputNode повертає числовий пріоритет.

Базовий діапазон:

```text
0.0 .. 1.0
```

Приклад:

```text
repair_boundary = 0.9
movement = 0.1
divide = 0.0
produce_energy = 0.8
```

Ці значення передаються в `Action Planning`.

---

# Action Priority не є Action

OutputNode не виконує процес.

Він лише задає бажання або пріоритет.

```text
Output Priority
    ↓
Feasibility Check
    ↓
Process Execution
```

Якщо клітина не має ресурсів, Energy або Materials, процес не відбудеться.

---

# Feasibility Check

Після виконання Regulatory Network кожен вихід проходить перевірку.

Перевіряються:

* Energy;
* Resources;
* Materials;
* free_capacity;
* Boundary;
* Heat;
* Pressure;
* Joint;
* lifecycle_state;
* physics constraints.

Regulatory Network не може обійти обмеження світу.

---

# Graph Topology

Regulatory Network може бути:

* простим;
* глибоким;
* розгалуженим;
* циклічним;
* ациклічним;
* модульним;
* фрагментованим.

Для базової моделі краще почати з обмеженої моделі.

---

# базова модель Topology

Для базової моделі рекомендується:

```text
InputNode -> RegulatoryNode -> OutputNode
```

з обмеженнями:

* обмежена кількість вузлів;
* обмежена кількість ребер;
* обмежена глибина виконання;
* контроль циклів;
* deterministic execution.

Це дозволить уникнути нескінченних циклів і непередбачуваного runtime.

---

# Cycles

Цикли в Regulatory Network можуть бути корисними.

Вони можуть давати:

* пам'ять;
* осциляції;
* стабільні стани;
* затриману реакцію;
* саморегуляцію.

Але цикли ускладнюють виконання.

Для базової моделі можливі два варіанти:

## Варіант A. Заборонити цикли

Простіше для реалізації.

```text
DAG only
```

Мінус: менше виразності.

## Варіант B. Дозволити цикли через runtime steps

Граф виконується кілька внутрішніх ітерацій.

```text
for step in runtime_steps:
    update node activations
```

Мінус: складніше балансувати.

Для базової моделі безпечніше почати з `DAG` або з фіксованої малої кількості runtime steps.

---

# Modular Fragments

Regulatory Network може складатися з фрагментів.

```text
Genome
├── fragment_A
├── fragment_B
└── fragment_C
```

Кожен фрагмент може містити:

* input bindings;
* regulatory nodes;
* edges;
* output bindings.

Це важливо для майбутніх моделей:

* duplication;
* recombination;
* horizontal transfer;
* plasmid-like genomes;
* partial genome loss;
* mobile fragments.

---

# Fragment

`Fragment` — частина Regulatory Network.

Фрагмент може мати:

```text
fragment_id
nodes
edges
input_bindings
output_bindings
copy_stability
mutation_rate
integration_rules
```

Для базової моделі фрагменти можна не реалізовувати повністю.

Але структура даних не повинна блокувати їх появу.

---

# Input Binding

Input Binding визначає, як зовнішній або внутрішній сигнал підключений до InputNode.

Приклад:

```text
InputNode:
  id: input_energy
  source: cell.energy_buffer.normalized_level
```

Binding не є фізичним сенсором сам по собі.

Клітина повинна мати матеріальну здатність вимірювати або отримувати цей сигнал.

---

# Material-dependent Inputs

Деякі входи доступні лише за наявності відповідних Materials.

Наприклад:

```text
Light input requires light-sensitive material.
Pressure input requires pressure-sensitive material.
Resource detection requires compatible boundary or sensor-like material.
Joint signal requires Joint.
```

Genome може мати InputNode для сигналу, але якщо клітина не має матеріальної основи, цей вхід буде нульовим або недоступним.

---

# Output Binding

Output Binding зв'язує OutputNode з конкретним клітинним процесом.

Приклад:

```text
OutputNode:
  id: output_repair_boundary
  target_process: repair_boundary
```

Output Binding не створює процес.

Він лише регулює його пріоритет.

---

# Material-dependent Outputs

Деякі виходи мають сенс лише за наявності Materials.

Наприклад:

```text
move requires movement-capable material.
active_transport requires pump-capable material.
produce_energy requires catalytic material.
create_joint requires compatible boundary material.
```

Якщо потрібних Materials немає, вихід може бути активним, але процес не виконається.

---

# Node Parameters

RegulatoryNode може мати параметри:

```text
bias
activation_function
threshold
decay_rate
statefulness
noise_level
mutation_rate
```

Не всі параметри потрібні у базовій моделі.

---

# Edge Parameters

Edge може мати параметри:

```text
weight
enabled
delay
stability
mutation_rate
```

Не всі параметри потрібні у базовій моделі.

---

# Genome Cost

Regulatory Network має вартість.

Вартість може залежати від:

* кількості вузлів;
* кількості ребер;
* кількості фрагментів;
* складності activation functions;
* runtime steps;
* копіювання;
* repair;
* фізичного об'єму Genome;
* ризику mutation.

Більший Genome не повинен бути безкоштовним.

---

# Size Pressure

Щоб Genome не зростав безмежно, повинні існувати природні обмеження:

* об'єм;
* Energy cost;
* copying cost;
* mutation risk;
* runtime cost;
* degradation risk.

Це створює еволюційний тиск на компактність.

---

# Determinism

За однакових умов, однакового Genome і однакового random seed Regulatory Network повинна давати однаковий результат.

Це потрібно для:

* тестування;
* дебагу;
* відтворюваності;
* наукового аналізу.

---

# Randomness

Regulatory Network не повинна бути випадковою під час звичайного виконання, якщо це не визначено явно.

Randomness може з'являтися у:

* mutation;
* stochastic node;
* noisy regulation;
* genome copying errors;
* damage;
* recombination.

У базовій моделі краще мінімізувати runtime randomness.

---

# Mutation Compatibility

Regulatory Network повинна бути придатною до мутацій.

Мутації можуть:

* змінювати weight;
* змінювати bias;
* змінювати threshold;
* додавати node;
* видаляти node;
* додавати edge;
* видаляти edge;
* змінювати activation function;
* змінювати input binding;
* змінювати output binding;
* дублювати fragment;
* видаляти fragment.

Не всі мутації повинні створювати життєздатний Genome.

Це нормально.

---

# Recombination Compatibility

Regulatory Network повинна дозволяти змішування генетичного матеріалу без вимоги ідеального вирівнювання.

Можливі оператори:

* insert fragment;
* replace fragment;
* merge fragment;
* duplicate fragment;
* split fragment;
* partial overwrite;
* broken merge.

Якщо після recombination граф стає поганим, selection відфільтрує його.

---

# HGT Compatibility

Regulatory Network повинна дозволяти горизонтальне перенесення фрагментів.

Це означає, що фрагмент може бути:

* поглинутий клітиною;
* тимчасово існувати;
* інтегрований;
* відкинутий;
* деградований;
* дубльований;
* почати впливати на регуляцію.

Для цього модель не повинна бути жорстко монолітною.

---

# Runtime Overview

Повний runtime описується в `genetics/genome-runtime.md`.

У цьому документі достатньо зафіксувати загальний потік:

```text
1. Collect local inputs
2. Normalize inputs
3. Apply epigenetic modifiers
4. Run regulatory graph
5. Produce output priorities
6. Send priorities to Action Planning
```

---

# Epigenetic Modifiers

Epigenetic State може змінювати роботу Regulatory Network без зміни Genome.

Наприклад:

* змінити effective threshold;
* змінити effective weight;
* приглушити частину виходів;
* підсилити repair;
* зменшити movement;
* перевести клітину в dormancy;
* змінити чутливість до Heat.

```text
Genome parameter
    +
Epigenetic modifier
    ↓
Effective runtime parameter
```

Це не є mutation.

---

# Effective Parameters

Потрібно розрізняти спадкові параметри і runtime-параметри.

```text
Inherited weight = 0.6
Epigenetic modifier = -0.2
Effective weight = 0.4
```

Genome не змінився.

Змінилася його поточна інтерпретація.

---

# Interaction with Cell Materials

Regulatory Network може працювати лише через клітинні процеси й матеріали.

Наприклад:

```text
Output: synthesize_material_X = 0.8
```

не створює Material X напряму.

Потрібні:

* Resources;
* Energy;
* synthesis-capable Material;
* free_capacity;
* time;
* feasibility check.

---

# Interaction with Signals

Regulatory Network може читати сигнали.

Сигнали можуть надходити з:

* Fields;
* Resources;
* Materials;
* Boundary;
* Joint;
* contact;
* Heat;
* Pressure;
* internal state.

Сигнал не є командою.

Сигнал є входом.

---

# Interaction with Neural-like Cells

У моделі немає hardcoded `Neuron`.

Але Regulatory Network може керувати клітиною, яка має neural-like Materials.

Такі матеріали можуть підтримувати:

* signal accumulation;
* impulse decay;
* threshold activation;
* signal conduction;
* adaptive gain;
* state plasticity;
* connection modulation.

Regulatory Network у такій клітині може регулювати:

* чутливість;
* пороги;
* підтримку signal state;
* синтез neural-like materials;
* Energy allocation;
* Joint signal output.

Таким чином neural-like behavior виникає з матеріалів, сигналів, Joint і регуляції.

---

# Minimal базова модель Network

Для першої реалізації достатньо невеликої мережі.

Приклад входів:

```text
energy_level
free_capacity
resource_A_inside
resource_A_outside
boundary_integrity
heat_level
damage_level
```

Приклад виходів:

```text
uptake_resource_A
produce_energy
synthesize_boundary_material
repair_boundary
grow
prepare_division
enter_dormancy
```

Приклад структури:

```text
7 InputNodes
3-8 RegulatoryNodes
7 OutputNodes
10-30 Edges
```

Цього достатньо, щоб отримати просту еволюційну динаміку.

---

# Example: Energy Regulation

```text
Input:
  energy_level low
  resource_A_inside high

Network:
  energy_level -- negative weight --> dormancy
  resource_A_inside -- positive weight --> produce_energy
  energy_level -- negative weight --> produce_energy

Output:
  produce_energy high
  movement low
  dormancy medium
```

Інтерпретація:

```text
Клітина має ресурс і мало Energy.
Мережа підвищує пріоритет виробництва Energy і знижує рух.
```

---

# Example: Boundary Repair

```text
Input:
  boundary_integrity low
  energy_level medium
  resource_A_inside high

Network:
  boundary_integrity -- negative weight --> repair_boundary
  resource_A_inside -- positive weight --> repair_boundary
  energy_level -- positive weight --> repair_boundary

Output:
  repair_boundary high
  grow low
  divide low
```

Інтерпретація:

```text
Клітина пошкоджена.
Регуляція переносить ресурси з росту на ремонт.
```

---

# Example: Failed Regulation

```text
Input:
  energy_level low
  boundary_integrity low
  resource_A_inside low

Network Output:
  divide high
  move high
  repair low

Feasibility Check:
  division impossible
  movement weak or impossible
  repair not prioritized

Result:
  cell degrades and may die
```

Це допустимо.

Рушій не повинен забороняти погану регуляцію.

Selection відфільтрує її.

---

# Example: Dormancy

```text
Input:
  energy_level low
  resource_A_outside low
  heat_level high

Network Output:
  enter_dormancy high
  movement low
  synthesis low
  maintenance medium

Result:
  cell enters dormant state
  Energy consumption decreases
```

---

# Example: Neural-like Signal Response

```text
Input:
  repeated_joint_signal high
  energy_level sufficient

Network Output:
  increase_signal_gain high
  maintain_stateful_material high
  output_signal medium

Material Process:
  stateful material changes coefficient

Future:
  same signal causes stronger response
```

Genome не змінився.

Змінився стан матеріалу або клітини.

---

# Правила

## Rule 1. Regulatory Network is a graph

Genome може бути представлений як орієнтований граф регуляції.

## Rule 2. Inputs are local

InputNode може читати лише локальні стани клітини або її найближчого середовища.

## Rule 3. Outputs are priorities

OutputNode повертає пріоритет процесу, а не гарантовану дію.

## Rule 4. Genome cannot bypass feasibility

Будь-який вихід Regulatory Network проходить через Feasibility Check.

## Rule 5. Materials are required

Input і Output мають сенс лише за наявності відповідної матеріальної основи.

## Rule 6. Runtime state is not Genome

Накопичений сигнал, learning-like state або epigenetic modifier не змінюють Genome напряму.

## Rule 7. Graph must mutate

Модель Regulatory Network повинна дозволяти мутації вузлів, ребер і параметрів.

## Rule 8. Graph must support inheritance

Regulatory Network повинна копіюватися і передаватися дочірнім клітинам.

## Rule 9. Graph should allow fragments

Модель не повинна блокувати майбутні фрагменти, recombination і HGT.

## Rule 10. Larger graphs have cost

Більша мережа повинна мати фізичну, енергетичну або runtime-вартість.

---

# Заборонено

Не вводити:

* behavior tree як Genome;
* hardcoded survival script;
* hardcoded neuron;
* hardcoded organ outputs;
* global world inputs;
* direct material creation;
* direct Energy creation;
* direct action execution from Genome;
* mutation with goal;
* selection score inside Genome;
* learning as direct Genome rewrite.

---

# Пов'язані документи

* `biology/genome.md`
* `biology/cell.md`
* `biology/processes.md`
* `biology/membrane.md`
* `world/materials.md`
* `world/resources.md`
* `world/energy.md`
* `world/tick.md`
* `genetics/genome-runtime.md`
* `genetics/mutation.md`
* `genetics/inheritance.md`
* `genetics/recombination.md`
* `genetics/horizontal-transfer.md`
* `genetics/epigenetics.md`

# Open Questions

## базова модель topology

Потрібно остаточно вирішити:

* DAG only;
* cyclic graph with fixed runtime steps;
* hybrid model.

## Activation functions

Потрібно вибрати стартовий набір activation functions.

Кандидати:

```text
linear
clamp
sigmoid
threshold
```

## Node state

Потрібно вирішити, чи підтримує базова модель stateful RegulatoryNode.

## Fragment support

Потрібно вирішити, чи Genome одразу має складатися з fragments, чи це буде додано пізніше.

## Input availability

Потрібно визначити, як саме Materials відкривають або блокують InputNode.

## Output feasibility

Потрібно визначити, чи OutputNode для неможливого процесу:

* просто ігнорується;
* зберігає “намір”;
* впливає на epigenetic_state;
* створює stress.

## Runtime cost

Потрібно визначити, чи виконання Regulatory Network витрачає Energy У базовій моделі.

## Network size limits

Потрібно визначити стартові обмеження:

* max nodes;
* max edges;
* max depth;
* max fragments;
* max runtime steps.


