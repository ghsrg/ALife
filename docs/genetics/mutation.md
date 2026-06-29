# mutation.md

> **Mutation — випадкова зміна спадкової регуляторної структури**

---

# Призначення

Цей документ описує `Mutation` — випадкову зміну Genome або його частини.

Mutation змінює спадкову структуру, яка може бути передана дочірнім клітинам.

Mutation не є learning.

Mutation не є адаптацією протягом життя.

Mutation не має мети.

Mutation не знає, чи буде зміна корисною.

Selection визначає наслідки мутацій через виживання і розмноження.

---

# Основна ідея

Genome може бути представлений як Regulatory Network.

Мутація змінює цю мережу.

```text
Genome
    ↓
Mutation
    ↓
Changed Genome
    ↓
Changed Regulation
    ↓
Changed Cell Behavior
    ↓
Selection
```

Мутація не оцінює результат.

Вона лише створює варіацію.

---

# Що Mutation НЕ є

Mutation не є:

* learning;
* epigenetic change;
* repair;
* deliberate improvement;
* reaction to survive;
* directed optimization;
* behavior adaptation;
* автоматичним виправленням Genome;
* гарантовано корисною зміною.

Mutation може бути:

* корисною;
* шкідливою;
* нейтральною;
* смертельною;
* майже непомітною;
* корисною лише в певному середовищі.

---

# Mutation і Learning

Learning-like state змінює стан клітини або матеріалів.

Mutation змінює спадкову структуру Genome.

```text
Learning-like change:
  repeated signal -> material coefficient changes

Mutation:
  Genome edge weight changes
```

Learning-like state не передається як зміна Genome.

Mutation може бути передана дочірнім клітинам.

---

# Mutation і Epigenetics

Epigenetic State змінює виконання Genome без зміни спадкової структури.

Mutation змінює саму спадкову структуру.

```text
Genome weight = 0.8
Epigenetic modifier = -0.2
Effective weight = 0.6
```

Це не mutation.

Mutation буде:

```text
Genome weight = 0.8
Mutation
Genome weight = 0.65
```

---

# Коли виникає Mutation

Mutation може виникати під час:

* Genome copying;
* cell division;
* Genome repair;
* Genome damage;
* recombination;
* horizontal gene transfer;
* integration of genetic fragment;
* radiation effect;
* heat damage;
* chemical reaction with Genome material;
* degradation;
* experimental world seed generation.

У базовій моделі основне місце мутацій:

```text
Genome copying during division
```

---

# Джерела Mutation

Mutation може мати різні джерела.

```text
copying_error
radiation
heat_damage
chemical_damage
repair_error
recombination_error
hgt_integration_error
fragment_degradation
random_drift
```

Не всі джерела потрібні у базовій моделі.

Для базової моделі достатньо:

```text
copying_error
random_drift
```

---

# Mutation Rate

`mutation_rate` визначає ймовірність мутацій.

Mutation Rate може бути:

* глобальним параметром світу;
* параметром Genome;
* параметром Fragment;
* параметром конкретного Node або Edge;
* залежним від середовища;
* залежним від стану клітини;
* залежним від Materials.

Приклад:

```text
effective_mutation_rate =
base_mutation_rate
× genome_modifier
× environment_modifier
× material_protection_modifier
```

Формула не є обов'язковою Для базової моделі.

Це принцип.

---

# Спадкова зміна mutation_rate

Genome може містити параметри, які впливають на власну мутабельність.

Наприклад:

```text
mutation_rate_modifier
repair_priority
genome_isolation_level
copy_stability
```

Але Genome не може обирати корисні мутації.

Він може лише змінювати ймовірність або типи випадкових змін.

---

# Trade-off Mutation Rate

Високий mutation rate дає:

* більше варіацій;
* швидшу адаптацію популяції;
* більше шансів знайти нові рішення;
* більший ризик руйнування життєздатної регуляції.

Низький mutation rate дає:

* стабільність;
* менше шкідливих змін;
* повільнішу адаптацію;
* ризик застрягнути в поганій стратегії.

Selection визначає, які рівні мутабельності виживають.

---

# Об'єкти мутації

Мутація може змінювати:

* Node;
* Edge;
* Input Binding;
* Output Binding;
* Fragment;
* Genome-level parameter;
* mutation parameters;
* runtime-related inherited parameters;
* copy stability;
* integration rules.

---

# Node Mutation

Node Mutation змінює вузол Regulatory Network.

Можливі типи:

```text
node_addition
node_deletion
node_duplication
node_parameter_shift
node_activation_change
node_threshold_change
node_bias_change
node_statefulness_change
```

---

# Node Addition

`node_addition` додає новий RegulatoryNode.

Новий вузол може:

* не мати впливу;
* отримати випадкові входи;
* підключитися до існуючого OutputNode;
* дублювати частину існуючого вузла;
* створити нову регуляторну комбінацію.

Приклад:

```text
Before:
  energy_level -> produce_energy

After:
  energy_level -> new_node -> produce_energy
```

Node Addition може бути корисним або шкідливим.

---

# Node Deletion

`node_deletion` видаляє вузол.

Наслідки:

* втрата регуляторного шляху;
* спрощення Genome;
* зменшення runtime cost;
* втрата критичної функції;
* нейтральний ефект, якщо вузол не використовувався.

Якщо видалений вузол був потрібен для repair або Energy production, клітина може стати нежиттєздатною.

---

# Node Duplication

`node_duplication` копіює існуючий вузол.

Дубльований вузол може:

* одразу створити надлишкову регуляцію;
* поступово мутувати в нову функцію;
* підсилити існуючий сигнал;
* збільшити Genome cost;
* створити конфлікт.

Дублювання є важливим джерелом нової складності.

---

# Node Parameter Shift

`node_parameter_shift` змінює параметри вузла.

Наприклад:

```text
bias: 0.20 -> 0.35
threshold: 0.60 -> 0.55
decay_rate: 0.10 -> 0.14
```

Малі зміни частіше дають плавну еволюцію.

Великі зміни частіше руйнують регуляцію.

---

# Edge Mutation

Edge Mutation змінює зв'язок між вузлами.

Можливі типи:

```text
edge_addition
edge_deletion
edge_weight_shift
edge_weight_flip
edge_source_change
edge_target_change
edge_delay_change
edge_enable_disable
```

---

# Edge Addition

`edge_addition` додає новий зв'язок.

Приклад:

```text
heat_level -> enter_dormancy
```

Новий Edge може:

* активувати нову реакцію;
* створити конфлікт;
* бути нейтральним;
* зробити регуляцію нестабільною;
* дати нову адаптивну можливість.

---

# Edge Deletion

`edge_deletion` видаляє зв'язок.

Наслідки:

* втрата реакції на певний вхід;
* спрощення графа;
* зменшення cost;
* злам критичної регуляції;
* усунення шкідливого впливу.

---

# Edge Weight Shift

`edge_weight_shift` змінює силу зв'язку.

Приклад:

```text
weight: 0.40 -> 0.48
```

або:

```text
weight: -0.30 -> -0.55
```

Це базовий і найпростіший тип мутації.

Для базової моделі саме `edge_weight_shift` може бути основним механізмом.

---

# Edge Weight Flip

`edge_weight_flip` змінює знак впливу.

```text
+0.6 -> -0.6
```

Активація перетворюється на пригнічення або навпаки.

Це сильна мутація.

Вона може різко змінити поведінку клітини.

---

# Binding Mutation

Binding Mutation змінює, які входи або виходи підключені до Genome.

Можливі типи:

```text
input_binding_addition
input_binding_deletion
input_binding_change
output_binding_addition
output_binding_deletion
output_binding_change
```

---

# Input Binding Mutation

Input Binding Mutation змінює, які локальні сигнали може читати Genome.

Приклад:

```text
Before:
  input_node reads heat_level

After:
  input_node reads pressure_level
```

Але вхід має сенс лише за наявності матеріальної основи.

Genome може мати InputNode для Light, але без light-sensitive Material Runtime отримає `0.0`.

---

# Output Binding Mutation

Output Binding Mutation змінює, який процес регулює OutputNode.

Приклад:

```text
Before:
  output_node -> movement

After:
  output_node -> repair_boundary
```

Це може радикально змінити поведінку клітини.

Output Binding не гарантує виконання процесу.

Він лише задає пріоритет.

---

# Fragment Mutation

Якщо Genome складається з фрагментів, мутації можуть змінювати фрагменти.

Можливі типи:

```text
fragment_duplication
fragment_deletion
fragment_split
fragment_merge
fragment_inversion
fragment_reorder
fragment_partial_damage
fragment_copy_rate_shift
```

Фрагменти важливі для майбутніх моделей:

* recombination;
* horizontal transfer;
* plasmid-like genome;
* mobile genetic elements;
* partial inheritance.

Для базової моделі фрагменти можна відкласти, але не блокувати архітектурно.

---

# Genome-level Mutation

Genome-level Mutation змінює параметри всього Genome.

Приклади:

```text
base_mutation_rate
copy_stability
repair_affinity
fragment_integration_probability
genome_isolation_level
max_runtime_steps
```

Такі мутації можуть впливати на темп еволюції, стабільність і відкритість до HGT.

---

# Structural Mutation

Structural Mutation змінює топологію графа.

Приклади:

```text
add node
delete node
add edge
delete edge
duplicate subgraph
delete subgraph
merge fragments
split fragment
```

Structural Mutation зазвичай має сильніший ефект, ніж зміна ваги.

---

# Parametric Mutation

Parametric Mutation змінює числові параметри без зміни топології.

Приклади:

```text
weight shift
bias shift
threshold shift
decay_rate shift
mutation_rate shift
copy_stability shift
```

Parametric Mutation частіше дає плавні зміни.

---

# Silent Mutation

Silent Mutation не змінює помітну поведінку клітини.

Причини:

* вузол неактивний;
* Edge не використовується;
* вхід недоступний через відсутність Material;
* вихід не проходить Feasibility Check;
* зміна занадто мала;
* існує дублюючий регуляторний шлях.

Silent Mutation може стати важливою пізніше, якщо інші мутації активують цей шлях.

---

# Neutral Mutation

Neutral Mutation не впливає на fitness у поточному середовищі.

Вона може бути корисною або шкідливою в іншому середовищі.

Neutral mutations дозволяють накопичувати приховану варіативність.

---

# Harmful Mutation

Harmful Mutation знижує життєздатність.

Приклади:

* repair більше не активується;
* Energy production не запускається;
* Boundary не синтезується;
* division запускається занадто рано;
* клітина накопичує waste;
* Genome runtime стає занадто дорогим;
* регуляція стає хаотичною.

Рушій не повинен забороняти harmful mutations.

Selection відфільтрує їх.

---

# Beneficial Mutation

Beneficial Mutation покращує виживання або розмноження в конкретному середовищі.

Приклади:

* краща реакція на Heat;
* швидший repair Boundary;
* ефективніша Energy production;
* здатність використовувати новий Resource;
* контроль dormancy;
* краща підготовка до division;
* корисна взаємодія через Joint.

Beneficial Mutation не знає, що вона корисна.

Вона стає корисною лише через наслідки.

---

# Lethal Mutation

Lethal Mutation робить клітину або дочірню клітину нежиттєздатною.

Приклади:

* Genome втратив усі outputs;
* відсутня Boundary maintenance;
* Energy production неможлива;
* critical process постійно вимкнений;
* graph invalid;
* division створює клітини без Genome;
* mutation rate став занадто високим.

Lethal mutations дозволені.

Вони є нормальною частиною еволюції.

---

# Mutation During Copying

Під час копіювання Genome може виникати Mutation.

Загальна схема:

```text
Parent Genome
    ↓
Copy
    ↓
Copy Error / Mutation
    ↓
Daughter Genome
```

Порядок:

```text
1. Copy Genome
2. Apply copying errors
3. Validate structure
4. Assign Genome to daughter cell
```

Validation не повинна “ремонтувати” Genome до корисного стану.

Вона лише повинна гарантувати, що структура даних не ламає рушій.

---

# Mutation і Validation

Після мутації Genome може бути:

* valid and viable;
* valid but non-viable;
* valid but inefficient;
* invalid as data structure.

Рушій повинен відрізняти:

```text
biologically non-viable
```

від:

```text
technically invalid
```

Біологічно нежиттєздатні Genome дозволені.

Технічно invalid Genome треба або відкидати, або приводити до безпечного технічного стану.

---

# Technical Validity

Genome технічно valid, якщо:

* усі Node мають унікальні id;
* Edge посилаються на існуючі Node;
* немає заборонених self-reference, якщо вони заборонені базова модель;
* кількість вузлів у межах ліміту;
* кількість ребер у межах ліміту;
* activation functions відомі рушію;
* output bindings посилаються на відомі process types;
* input bindings посилаються на відомі local signals.

Technical validity не означає життєздатність.

---

# Non-viable but Valid Genome

Genome може бути технічно valid, але біологічно поганим.

Приклад:

```text
Genome has no repair output.
Genome always requests division.
Genome ignores Energy level.
Genome never produces Boundary Material.
```

Такий Genome не треба виправляти.

Він просто не виживе.

---

# Mutation і Constraints

Mutation повинна дотримуватися технічних обмежень.

Приклади обмежень:

```text
max_nodes
max_edges
max_fragments
max_weight
min_weight
max_threshold
min_threshold
allowed_activation_functions
allowed_process_outputs
```

Ці обмеження потрібні для стабільності симуляції.

Вони не повинні бути біологічним “захистом від поганих рішень”.

---

# Mutation і Genome Cost

Мутації можуть змінювати cost Genome.

Наприклад:

* node addition збільшує cost;
* edge addition збільшує runtime cost;
* fragment duplication збільшує copy cost;
* node deletion зменшує cost;
* simpler activation function зменшує cost.

Genome Cost може впливати на selection.

Більша складність має мати ціну.

---

# Mutation і Materials

Mutation не змінює Materials напряму.

Але змінений Genome може інакше регулювати:

* Material synthesis;
* Material repair;
* Material degradation;
* Boundary maintenance;
* signal-sensitive Materials;
* stateful Materials;
* Energy conversion Materials.

```text
Mutation in Genome
    ↓
Changed Output Priorities
    ↓
Changed Material Processes
    ↓
Changed Cell Structure
```

---

# Mutation і Energy

Mutation не створює Energy.

Але може змінити:

* пріоритет Energy production;
* здатність синтезувати Energy-conversion Materials;
* перехід у dormancy;
* витрати на movement;
* витрати на growth;
* Genome runtime cost;
* efficiency indirectly through Materials.

---

# Mutation і Boundary

Mutation може впливати на Boundary лише через регуляцію процесів.

Приклади:

* repair_boundary активується раніше;
* synthesize_boundary_material слабшає;
* permeability-related Materials більше не виробляються;
* Joint-compatible Boundary Materials з'являються частіше.

Mutation не робить Boundary міцною напряму.

---

# Mutation і Neural-like Behavior

Mutation може змінити здатність клітини до neural-like behavior.

Наприклад, вона може змінити регуляцію:

* signal-sensitive Materials;
* stateful Materials;
* impulse accumulation;
* impulse decay;
* signal gain;
* activation threshold;
* Joint signal output.

Але Mutation не є learning.

Learning-like behavior змінює стан протягом життя.

Mutation змінює спадкову здатність до такого стану.

---

# Mutation і Environment

Середовище може впливати на ймовірність мутацій.

Наприклад:

* Radiation підвищує mutation risk;
* Heat може пошкоджувати Genome;
* reactive Resources можуть руйнувати genetic material;
* unstable Materials можуть гірше захищати Genome;
* repair Materials можуть зменшувати пошкодження.

Але середовище не обирає корисні зміни.

Воно лише змінює ймовірності або типи пошкоджень.

---

# Mutation і Selection

Selection працює після Mutation через наслідки.

```text
Mutation
    ↓
Changed Cell Regulation
    ↓
Changed Survival / Reproduction
    ↓
Population Change
```

Немає окремого `fitness score`, який Genome читає або оптимізує.

Fitness є результатом взаємодії клітини зі світом.

---

# Mutation і Recombination

Mutation і Recombination — різні процеси.

Mutation змінює Genome випадково.

Recombination змішує генетичні матеріали.

Але Recombination може створювати Mutation-like effects:

* broken merge;
* fragment loss;
* duplicated fragment;
* changed edge context;
* invalid connection;
* disrupted regulation.

Це буде описано в `genetics/recombination.md`.

---

# Mutation і HGT

Під час Horizontal Gene Transfer мутації можуть виникати через:

* пошкодження фрагмента;
* неповну інтеграцію;
* конфлікт з існуючим Genome;
* випадкове підключення fragment outputs;
* degradation before integration.

Це буде описано в `genetics/horizontal-transfer.md`.

---

# Mutation Operators for базова модель

Для базової моделі достатньо мінімального набору операторів:

```text
edge_weight_shift
edge_addition
edge_deletion
node_addition
node_deletion
bias_shift
threshold_shift
input_binding_change
output_binding_change
```

Можна почати ще простіше:

```text
edge_weight_shift
bias_shift
threshold_shift
```

А structural mutations додати після стабілізації runtime.

---

# Recommended базова модель Mutation Order

Під час копіювання Genome:

```text
1. Copy Genome
2. For each edge:
     maybe shift weight
3. For each node:
     maybe shift bias
     maybe shift threshold
4. Maybe add or remove one edge
5. Maybe add or remove one node
6. Validate technical structure
7. Assign to daughter cell
```

Цей порядок простий і контрольований.

---

# Mutation Intensity

Мутація може мати інтенсивність.

```text
small mutation
medium mutation
large mutation
```

Приклад:

```text
small:
  weight + random(-0.05, 0.05)

medium:
  weight + random(-0.20, 0.20)

large:
  weight replaced by random value
```

Для базової моделі краще починати з малих мутацій і рідкісних структурних змін.

---

# Mutation Distribution

Випадкові зміни можуть братися з різних розподілів.

Прості варіанти:

```text
uniform
normal
bounded normal
```

Для ваг і threshold зручно використовувати bounded normal:

```text
new_value = old_value + random_normal(0, sigma)
new_value = clamp(new_value, min, max)
```

---

# Determinism

Mutation повинна бути детермінованою за однакового seed.

Це потрібно для:

* повторюваних експериментів;
* дебагу;
* тестів;
* наукового аналізу.

Усі випадкові рішення повинні братися з контрольованого RNG.

---

# Mutation Trace

Для дебагу й аналізу бажано зберігати Mutation Trace.

Приклад:

```text
Tick 120
Cell 45 division
Mutation:
  edge_weight_shift edge_12: 0.42 -> 0.48
  threshold_shift node_7: 0.60 -> 0.57
```

Mutation Trace не треба зберігати для всіх клітин завжди.

Але він критично корисний для експериментів.

---

# Mutation і Lineage

Mutation повинна бути пов'язана з lineage.

Бажано зберігати:

```text
parent_cell_id
daughter_cell_id
parent_genome_id
daughter_genome_id
mutation_events
generation
```

Це не повинно керувати поведінкою клітини.

Це потрібно для аналізу еволюції.

---

# Приклад 1. Малий зсув ваги

```text
Before:
  energy_level -> produce_energy
  weight = -0.60

Mutation:
  weight_shift +0.08

After:
  weight = -0.52
```

Ефект:

```text
Клітина трохи слабше реагує на низьку Energy.
Це може бути нейтральним, шкідливим або корисним залежно від середовища.
```

---

# Приклад 2. Видалення Edge до repair

```text
Before:
  damage_level -> repair_boundary
  weight = +0.90

Mutation:
  edge deleted

After:
  damage більше не активує repair_boundary
```

Можливий результат:

```text
Клітина не ремонтує Boundary після пошкодження.
Лінія швидко зникає.
```

---

# Приклад 3. Новий Input Binding

```text
Before:
  node_A reads heat_level

Mutation:
  input_binding_change

After:
  node_A reads pressure_level
```

Можливий результат:

```text
Реакція, яка раніше запускалася Heat, тепер запускається Pressure.
```

---

# Приклад 4. Silent Mutation

```text
Mutation:
  new edge added:
    field_light -> produce_energy

Cell:
  has no light-sensitive Material

Runtime:
  field_light = 0.0

Effect:
  no visible change
```

Пізніше інша мутація може створити light-sensitive Material.

Тоді цей Edge стане важливим.

---

# Приклад 5. Mutation дає нову здатність

```text
Mutation:
  output_binding_change
  node_X now activates synthesize_material_L

Material L:
  light_sensitive = 0.8

Result:
  cell can synthesize light-sensitive Material
  future cells may use Light input
```

Це не означає, що Mutation “знала” про світло.

Вона випадково створила шлях, який виявився корисним.

---

# Приклад 6. Надмірний mutation_rate

```text
Mutation:
  base_mutation_rate increases from 0.02 to 0.30

Result:
  offspring have many changes
  most genomes become unstable
  lineage collapses
```

Але в мінливому середовищі помірне підвищення mutation_rate може бути корисним.

---

# Приклад 7. Neural-like властивість

```text
Mutation:
  threshold of signal accumulation node decreases

Before:
  repeated_joint_signal activates output after 5 Tick

After:
  repeated_joint_signal activates output after 3 Tick
```

Ефект:

```text
Клітина швидше реагує на повторні сигнали.
Це зміна спадкової регуляції, а не learning.
```

---

# Configuration базової моделі

Початкова конфігурація може бути такою:

```text
base_mutation_rate = 0.01
weight_shift_sigma = 0.05
bias_shift_sigma = 0.03
threshold_shift_sigma = 0.03

node_add_probability = 0.001
node_delete_probability = 0.001
edge_add_probability = 0.003
edge_delete_probability = 0.003
binding_change_probability = 0.001
```

Це не канонічні числа.

Це приклад стартових параметрів для експериментів.

---

# Правила

## Rule 1. Mutation changes Genome

Mutation змінює спадкову структуру Genome або її частину.

## Rule 2. Mutation is random

Mutation не має мети і не знає, чи буде корисною.

## Rule 3. Mutation is not learning

Learning-like state змінює стан клітини або матеріалів, але не Genome.

## Rule 4. Mutation is not epigenetics

Epigenetic State змінює виконання Genome, але не спадкову структуру.

## Rule 5. Mutation may be harmful

Рушій не повинен забороняти шкідливі або смертельні мутації.

## Rule 6. Selection filters

Selection діє через наслідки мутацій для виживання й розмноження.

## Rule 7. Technical validity is required

Genome після мутації не повинен ламати рушій як структура даних.

## Rule 8. Biological viability is not guaranteed

Технічно valid Genome може бути біологічно нежиттєздатним.

## Rule 9. Mutation must be deterministic with seed

За однакового seed мутаційні події повинні повторюватися.

## Rule 10. Mutation must support future fragments

Модель мутацій не повинна блокувати fragment-based Genome, recombination або HGT.

---

# Заборонено

Не вводити:

* directed beneficial mutation;
* automatic Genome improvement;
* mutation based on fitness score;
* mutation that reads future outcome;
* learning as Genome rewrite;
* epigenetic modifier as mutation;
* repair as guaranteed perfect restoration;
* engine-level ban on bad mutations;
* hardcoded species-specific mutation;
* plant/animal-specific mutation rules.

---

# Пов'язані документи

* `biology/genome.md`
* `biology/cell.md`
* `biology/lifecycle.md`
* `biology/processes.md`
* `genetics/regulatory-network.md`
* `genetics/genome-runtime.md`
* `genetics/inheritance.md`
* `genetics/recombination.md`
* `genetics/horizontal-transfer.md`
* `genetics/epigenetics.md`
* `world/fields.md`
* `world/materials.md`
* `world/energy.md`
* `world/laws.md`

# Open Questions

## базова модель mutation operators

Потрібно остаточно вибрати, які оператори включити у базовій моделі.

Кандидати:

```text
edge_weight_shift
bias_shift
threshold_shift
edge_addition
edge_deletion
node_addition
node_deletion
```

## Structural mutation timing

Потрібно вирішити, чи structural mutations будуть У базовій моделі, чи тільки після стабілізації базової еволюції.

## Fragment mutation

Потрібно вирішити, чи Genome одразу має fragment structure.

## Mutation rates

Потрібно підібрати стартові значення mutation_rate через експерименти.

## Genome repair interaction

Потрібно визначити, як repair відрізняється від mutation:

* repair може відновити;
* repair може створити помилку;
* repair не повинен бути магічно ідеальним.

## Environment-driven mutation risk

Потрібно визначити, чи Radiation, Heat і reactive Resources впливають на mutation_rate У базовій моделі.

## Mutation trace

Потрібно визначити мінімальний формат Mutation Trace для дебагу.

## Invalid Genome handling

Потрібно визначити, що робити з технічно invalid Genome:

* reject mutation;
* disable invalid part;
* mark genome damaged;
* produce non-viable daughter.


