# genome-representation.md

> **Genome Representation — вибір формального представлення Genome**

---

# Призначення

Цей документ описує варіанти представлення Genome у Artificial Life Engine і фіксує поточну рекомендовану модель.

`biology/genome.md` описує Genome як фізичну біологічну конструкцію клітини.

`genetics/regulatory-network.md` описує Genome як граф регуляції.

`genetics/genome-runtime.md` описує виконання цього графа.

`genome-representation.md` пояснює, чому обрано саме таку модель і які альтернативи були розглянуті.

---

# Статус

```text
Status: Research / Mostly Stable
```

Базовий напрямок визначено:

```text
Genome = фізичний носій спадкової регуляторної мережі
```

MVP-представлення:

```text
Genome as Direct Regulatory Graph
```

Майбутній розвиток:

```text
Fragment-compatible / plasmid-like Genome Pool
```

---

# Основна проблема

Genome повинен одночасно бути:

* спадковим;
* фізичним;
* мутованим;
* копійованим;
* придатним до recombination;
* придатним до horizontal transfer;
* здатним регулювати клітинні процеси;
* не бути behavior tree;
* не бути готовим blueprint організму;
* не містити hardcoded органи або типи клітин.

Потрібна модель, у якій Genome може еволюціонувати природно, але не перетворюється на штучну програму з командами.

---

# Вимоги до представлення Genome

Представлення Genome повинно підтримувати:

* local inputs;
* regulatory nodes;
* process outputs;
* mutation;
* copying;
* inheritance;
* recombination;
* HGT;
* fragments;
* physical cost;
* runtime cost;
* invalid or non-viable variants;
* deterministic execution;
* debugging;
* future multicellular development.

---

# Чого не повинно бути

Genome representation не повинно вимагати:

* hardcoded species;
* hardcoded plant/animal/microbe classes;
* hardcoded organs;
* hardcoded neurons;
* behavior scripts;
* explicit survival goals;
* fitness score input;
* global world access;
* perfect graph alignment during recombination;
* guaranteed viable offspring.

---

# Варіант 1. Behavior Tree Genome

## Ідея

Genome зберігає дерево поведінки.

```text
if energy low:
    find resource
if damaged:
    repair
if large:
    divide
```

## Плюси

* легко реалізувати;
* легко дебажити;
* швидко дає видиму поведінку;
* зрозуміло для agent/code generation.

## Мінуси

* занадто штучно;
* Genome стає поведінковим скриптом;
* складно отримати справжню emergence;
* важко уникнути hardcoded логіки;
* погано масштабується до невідомих стратегій;
* не відповідає принципу “Genome regulates, not behaves”.

## Рішення

```text
Rejected
```

Behavior Tree не використовувати як Genome.

---

# Варіант 2. Linear Instruction Genome

## Ідея

Genome — це послідовність інструкцій, схожа на маленьку програму.

```text
READ energy
IF_LOW activate produce_energy
WRITE repair_priority
JUMP ...
```

## Плюси

* добре мутується;
* легко копіюється;
* природно має порядок;
* можна робити crossover;
* схоже на artificial life systems типу “genetic programming”.

## Мінуси

* виглядає як штучний процесор;
* потрібні registers або stack;
* легко скотитися в VM;
* складно пояснити як фізичний носій регуляції;
* Genome стає програмою, а не регуляторною мережею;
* погано узгоджується з матеріальною клітинною моделлю.

## Рішення

```text
Rejected for MVP
```

Можна розглядати як окремий експеримент, але не як базову модель.

---

# Варіант 3. Chemical Tag Genome

## Ідея

Genome складається з елементів із tags.

Елементи взаємодіють, якщо їхні tags сумісні.

```text
tag_A activates tag_B
tag_B suppresses output_X
tag_C binds resource_signal
```

## Плюси

* більш “хімічно” виглядає;
* добре підтримує нечітку сумісність;
* природно підходить для HGT;
* не потребує жорсткого graph alignment;
* може давати emergent modules.

## Мінуси

* складніше реалізувати;
* складніше дебажити;
* важко пояснювати агентам;
* важко контролювати першу MVP-симуляцію;
* багато параметрів сумісності;
* високий ризик хаотичної поведінки без зрозумілої причини.

## Рішення

```text
Future Research
```

Цікава модель, але не для першої реалізації.

---

# Варіант 4. Direct Regulatory Graph

## Ідея

Genome представлений як орієнтований граф.

```text
Input Nodes
    ↓
Regulatory Nodes
    ↓
Output Nodes
```

Вузли й ребра формують регуляторну мережу, яка перетворює локальний стан клітини на пріоритети процесів.

```text
energy_level
    ↓
Regulatory Node
    ↓
produce_energy_priority
```

## Плюси

* добре відповідає ідеї регуляції;
* не є behavior tree;
* легко пояснюється;
* легко дебажиться;
* можна мутувати weights, nodes, edges;
* можна запускати як DAG;
* можна поступово розширювати до fragments;
* добре інтегрується з Cell Decision;
* outputs є priorities, а не actions.

## Мінуси

* graph recombination складна;
* важко змішувати два різні графи;
* може роздуватися без cost;
* потрібна technical validation;
* потрібен контроль циклів;
* потрібні ліміти на nodes/edges.

## Рішення

```text
Accepted for MVP
```

Direct Regulatory Graph — базова модель Genome для першої реалізації.

---

# Варіант 5. Fragment-based Regulatory Graph

## Ідея

Genome складається не з одного монолітного графа, а з фрагментів.

```text
Genome Pool
├── Boundary Fragment
├── Energy Fragment
├── Repair Fragment
├── Signal Fragment
└── Mobile Fragment
```

Кожен fragment може містити частину regulatory graph.

## Плюси

* краще підтримує recombination;
* краще підтримує HGT;
* дозволяє fragment loss;
* дозволяє fragment duplication;
* дозволяє mobile genetic elements;
* дозволяє plasmid-like behavior;
* краще масштабується до складної еволюції.

## Мінуси

* складніше для MVP;
* потрібно визначити fragment boundaries;
* потрібна integration model;
* потрібна cost model;
* складніше дебажити;
* можливі parasitic fragments.

## Рішення

```text
Post-MVP Direction
```

Архітектура MVP повинна не блокувати цей перехід.

---

# Варіант 6. Plasmid-like Genome Pool

## Ідея

Клітина має не один Genome, а пул genetic fragments.

```text
Genome Pool
├── Core Genome
├── Plasmid-like Fragment A
├── Plasmid-like Fragment B
└── Mobile Fragment C
```

Фрагменти можуть:

* копіюватися;
* втрачатися;
* мутувати;
* передаватися через HGT;
* впливати на Runtime;
* бути корисними;
* бути паразитичними;
* залишатися silent.

## Плюси

* природно підтримує HGT;
* добре підходить для простих організмів;
* дозволяє модульну еволюцію;
* дозволяє parasitic genetic elements;
* дає багату еволюційну динаміку;
* не потребує hardcoded species.

## Мінуси

* складна модель;
* потрібна genome pool execution;
* потрібна fragment competition;
* потрібна copy-rate модель;
* складно балансувати;
* не потрібно для MVP.

## Рішення

```text
Future Direction
```

Це перспективна цільова модель після базового regulatory graph.

---

# Поточне рішення

Для MVP використовується:

```text
Direct Regulatory Graph
```

Але з обмеженням:

```text
Design must remain fragment-compatible.
```

Тобто перша реалізація може мати один Genome на клітину, але структура даних не повинна заважати майбутньому переходу до fragments.

---

# MVP Representation

MVP Genome:

```text
Genome
├── nodes
├── edges
├── input_bindings
├── output_bindings
├── mutation_parameters
└── metadata
```

---

# Node

```text
Node
├── id
├── type
├── bias
├── threshold
├── activation_function
├── statefulness
└── mutation_rate
```

Типи вузлів:

```text
InputNode
RegulatoryNode
OutputNode
```

---

# Edge

```text
Edge
├── id
├── source_node_id
├── target_node_id
├── weight
├── enabled
└── mutation_rate
```

Для MVP достатньо:

```text
source_node_id
target_node_id
weight
```

---

# Input Binding

```text
InputBinding
├── node_id
├── source_signal
├── normalization
└── required_material
```

Приклади `source_signal`:

```text
energy_level
free_capacity
resource_A_inside
resource_A_outside
boundary_integrity
local_heat
local_pressure
joint_signal
damage_level
```

---

# Output Binding

```text
OutputBinding
├── node_id
├── target_process
└── required_material
```

Приклади `target_process`:

```text
uptake_resource_A
produce_energy
synthesize_material_X
repair_boundary
grow
prepare_division
enter_dormancy
create_joint
```

Output Binding не виконує процес напряму.

Він лише задає priority.

---

# Genome Metadata

```text
GenomeMetadata
├── genome_id
├── parent_genome_ids
├── generation
├── origin_tick
├── mutation_count
├── size_cost
├── copy_cost
└── runtime_cost
```

Metadata потрібна для аналізу.

Вона не повинна напряму керувати поведінкою клітини.

---

# Runtime

MVP Runtime:

```text
DAG execution
topological order
one pass
inputs 0.0..1.0
outputs 0.0..1.0
activation functions: linear_clamp, threshold
```

Цикли краще відкласти або дозволяти лише через fixed runtime steps у майбутньому.

---

# Mutation Compatibility

MVP Genome повинен підтримувати мутації:

```text
edge_weight_shift
bias_shift
threshold_shift
edge_addition
edge_deletion
node_addition
node_deletion
input_binding_change
output_binding_change
```

Спочатку можна увімкнути лише parametric mutations:

```text
edge_weight_shift
bias_shift
threshold_shift
```

---

# Recombination Compatibility

Навіть якщо recombination не входить у MVP, модель повинна дозволяти майбутні оператори:

```text
fragment_insert
fragment_replace
fragment_duplicate
fragment_delete
fragment_merge
partial_overwrite
broken_merge
```

Для цього бажано не робити Genome повністю monolithic на рівні архітектури.

---

# HGT Compatibility

Модель повинна дозволяти майбутнє існування:

```text
external genetic fragment
internal free fragment
integrated fragment
silent fragment
mobile fragment
plasmid-like fragment
```

Навіть якщо MVP має один Genome, формат повинен дозволяти пізніше додати `GenomePool`.

---

# Recommended Data Shape

Для MVP:

```text
Cell
└── genome: Genome
```

Але future-compatible форма:

```text
Cell
└── genome_pool
    ├── core_genome
    └── fragments
```

Рекомендація для коду:

```text
Cell.genetic_system
```

а не жорстко:

```text
Cell.genome
```

Щоб у майбутньому не ламати модель.

---

# GeneticSystem

Можна ввести обгортку:

```text
GeneticSystem
├── primary_genome
├── fragments
├── runtime_state
├── epigenetic_state
└── integration_queue
```

У MVP:

```text
fragments = []
integration_queue = []
primary_genome = Genome
```

Це дозволить не переписувати архітектуру при переході до HGT.

---

# Чому не `genetics/genome.md`

Файл `biology/genome.md` уже описує Genome як біологічну конструкцію.

Якщо створити ще один:

```text
genetics/genome.md
```

виникне плутанина.

Тому у `genetics/` краще мати конкретні файли:

```text
regulatory-network.md
genome-runtime.md
mutation.md
inheritance.md
heredity.md
recombination.md
horizontal-transfer.md
epigenetics.md
genome-representation.md
```

---

# Межа відповідальності файлів

```text
biology/genome.md
  що таке Genome для клітини

genetics/regulatory-network.md
  як Genome представлений як граф

genetics/genome-runtime.md
  як граф виконується

genetics/mutation.md
  як граф змінюється випадково

genetics/inheritance.md
  як стан передається дочірній клітині

genetics/heredity.md
  що є спадковим у lineage

genetics/recombination.md
  як змішуються спадкові структури

genetics/horizontal-transfer.md
  як genetic fragments передаються горизонтально

genetics/epigenetics.md
  як стан змінює runtime без зміни Genome

genetics/genome-representation.md
  чому обрано таку модель
```

---

# Physical Carrier

Незалежно від структури даних, Genome у світі повинен мати фізичний носій.

Він:

* займає об'єм;
* має cost;
* може бути пошкоджений;
* може деградувати;
* може копіюватися;
* може бути переданий;
* може залишитися після смерті клітини як fragment.

Тобто навіть якщо Genome у коді — graph object, у симуляції він є фізичною спадковою структурою.

---

# Cost Model

Genome representation повинна мати cost.

Можливі cost:

```text
storage_cost
copy_cost
runtime_cost
repair_cost
mutation_risk
degradation_risk
volume_cost
```

Більший або складніший Genome не повинен бути безкоштовним.

---

# Technical Validity

Genome technically valid, якщо:

* усі nodes мають унікальні ids;
* edges посилаються на існуючі nodes;
* activation functions відомі;
* input bindings відомі;
* output bindings відомі;
* graph відповідає execution mode;
* node/edge count у межах ліміту;
* немає NaN або invalid numeric values.

Technical validity не означає життєздатність.

---

# Biological Viability

Genome biologically viable, якщо його регуляція дозволяє клітині підтримувати мінімальну функціональну структуру.

Це не перевіряється як статична гарантія.

Genome може бути valid, але нежиттєздатним.

```text
valid graph
+
bad regulation
    ↓
cell dies
```

Це нормально.

---

# Debuggability

Genome representation повинна бути дебажною.

Потрібно мати змогу бачити:

* input values;
* node activations;
* edge contributions;
* output priorities;
* mutation events;
* inheritance trace;
* runtime trace;
* fragment integration trace;
* lineage history.

Без цього emergent behavior буде важко аналізувати.

---

# Serialization

Genome representation повинна серіалізуватися.

Потрібно зберігати:

```text
nodes
edges
bindings
parameters
metadata
fragments
runtime-compatible ids
```

Серіалізація не повинна залежати від порядку в пам'яті.

Потрібні стабільні ids.

---

# Determinism

За однакового Genome, однакового Cell State і однакового seed Runtime повинен давати однаковий результат.

Mutation, recombination і HGT також повинні бути відтворюваними за однакового seed.

Це критично для тестів і наукового аналізу.

---

# MVP Recommendation

Рекомендована MVP-модель:

```text
Genome:
  Direct Regulatory Graph

Runtime:
  DAG, topological order, one pass

Inputs:
  normalized local signals

Outputs:
  process priorities

Mutation:
  mostly parametric mutations first

Inheritance:
  copy Genome to daughters with mutation chance

Recombination:
  not in MVP, but data model fragment-compatible

HGT:
  not in MVP, but architecture future-compatible

Epigenetics:
  separate state modifying Runtime, not Genome
```

---

# Приклад MVP Genome

```text
Inputs:
  energy_level
  free_capacity
  resource_A_inside
  resource_A_outside
  boundary_integrity
  heat_level

Regulatory Nodes:
  node_1
  node_2
  node_3

Outputs:
  uptake_resource_A
  produce_energy
  repair_boundary
  synthesize_boundary_material
  grow
  prepare_division
  enter_dormancy

Edges:
  energy_level -> node_1
  resource_A_inside -> node_1
  node_1 -> produce_energy
  boundary_integrity -> node_2
  node_2 -> repair_boundary
  heat_level -> node_3
  node_3 -> enter_dormancy
```

---

# Приклад майбутнього Fragment Genome

```text
GeneticSystem
├── Core Genome
│   ├── boundary maintenance
│   ├── energy production
│   └── basic repair
│
├── Fragment A
│   └── resource_B export
│
├── Fragment B
│   └── signal-sensitive material synthesis
│
└── Mobile Fragment C
    └── increased HGT openness
```

Це не потрібно для MVP, але структура повинна дозволити такий розвиток.

---

# Поточне рішення

```text
Accepted:
  Direct Regulatory Graph for MVP

Required:
  fragment-compatible architecture

Rejected:
  behavior tree genome
  instruction/register genome as baseline
  hardcoded species genome

Future:
  fragment-based genome
  plasmid-like genome pool
  chemical-tag compatibility model
```

---

# Правила

## Rule 1. Genome representation must express regulation

Genome повинен представляти спадкову регуляцію, а не готову поведінку.

## Rule 2. Direct Regulatory Graph is MVP baseline

Для MVP базова модель — орієнтований regulatory graph.

## Rule 3. Genome outputs priorities

Genome output — це пріоритети процесів, а не фізичні дії.

## Rule 4. Representation must be physical-compatible

Genome повинен мати фізичний carrier, cost, copying і degradation.

## Rule 5. Representation must mutate

Модель повинна підтримувати mutation вузлів, ребер і параметрів.

## Rule 6. Representation must support inheritance

Genome повинен передаватися дочірнім клітинам.

## Rule 7. Representation must remain fragment-compatible

Навіть MVP не повинен блокувати future fragments, recombination і HGT.

## Rule 8. Technical validity is separate from biological viability

Valid graph може бути нежиттєздатним.

## Rule 9. Representation must be debuggable

Потрібні runtime trace, mutation trace і lineage analysis.

## Rule 10. No hardcoded species or organs

Genome не повинен містити hardcoded species, organs або cell types.

---

# Заборонено

Не вводити:

* behavior tree as Genome;
* hardcoded survival strategy;
* hardcoded species genome;
* hardcoded plant/animal genome;
* hardcoded organ list;
* global world inputs;
* fitness score input;
* automatic useful mutation;
* perfect recombination requirement;
* genome without cost;
* genome without physical carrier;
* representation that blocks HGT.

---

# Пов'язані документи

* `biology/genome.md`
* `biology/cell.md`
* `biology/processes.md`
* `biology/lifecycle.md`
* `genetics/regulatory-network.md`
* `genetics/genome-runtime.md`
* `genetics/mutation.md`
* `genetics/inheritance.md`
* `genetics/heredity.md`
* `genetics/recombination.md`
* `genetics/horizontal-transfer.md`
* `genetics/epigenetics.md`
* `world/materials.md`
* `world/resources.md`
* `world/energy.md`
* `world/laws.md`
* `engine/serialization.md`

---

# ADR

Потрібні ADR:

```text
ADR-000X: Direct Regulatory Graph as MVP Genome Representation
ADR-000X: Reject Behavior Tree Genome
ADR-000X: Reject Linear Instruction Genome as Baseline
ADR-000X: Keep Genome Architecture Fragment-compatible
ADR-000X: Genome Representation Must Have Physical Carrier and Cost
```

---

# Open Questions

## GeneticSystem wrapper

Потрібно вирішити, чи одразу вводити `Cell.genetic_system`, навіть якщо в MVP там буде тільки один Genome.

## Fragment model

Потрібно визначити, коли переходити від monolithic Genome до fragment-based Genome.

## Cycles

Потрібно вирішити, чи Runtime graph у MVP є тільки DAG, чи дозволяє цикли через fixed runtime steps.

## Activation functions

Потрібно остаточно затвердити набір activation functions для MVP.

## Cost formula

Потрібно визначити формули:

```text
copy_cost
runtime_cost
storage_cost
repair_cost
```

## Physical carrier

Потрібно вирішити, як саме Genome займає об'єм у клітині:

* як агрегований genetic material;
* як набір fragments;
* як окремі internal objects.

## Serialization

Потрібно визначити формат збереження Genome:

* JSON;
* binary;
* graph format;
* custom schema.

## Debugging

Потрібно визначити мінімальний набір debug traces для MVP.

## HGT-ready architecture

Потрібно вирішити, які поля додати вже зараз, щоб потім не ламати модель під HGT.
