# epigenetics.md

> **Epigenetics — зміна виконання Genome без зміни спадкової структури**

---

# Призначення

Цей документ описує `Epigenetics` — механізм зміни роботи Genome без зміни самого Genome.

Epigenetics не змінює Regulatory Network як спадкову структуру.

Epigenetics змінює те, як ця структура виконується в конкретній клітині в конкретний момент часу.

```text
Genome
+
Epigenetic State
+
Cell State
    ↓
Effective Genome Runtime
```

Epigenetics дозволяє одній і тій самій спадковій регуляторній структурі давати різну поведінку залежно від історії клітини, середовища, сигналів, stress і розвитку.

---

# Основна ідея

Genome задає спадкову регуляторну структуру.

Epigenetic State задає тимчасовий або напівстабільний контекст виконання цієї структури.

```text
Same Genome
+
Different Epigenetic State
    ↓
Different Runtime Output
```

Це особливо важливо для:

* stress response;
* dormancy;
* development;
* specialization;
* asymmetric division;
* recovery після пошкоджень;
* багатоклітинної диференціації;
* короткочасної спадковості стану.

---

# Що Epigenetics НЕ є

Epigenetics не є:

* mutation;
* learning;
* behavior tree;
* окремим мозком;
* прямим переписуванням Genome;
* гарантовано спадковою зміною;
* fitness feedback;
* магічною адаптацією;
* hardcoded типом клітини;
* species memory;
* способом обійти фізику.

Epigenetics не створює Materials, Resources або Energy напряму.

Вона лише змінює регуляторний контекст.

---

# Epigenetics vs Mutation

Mutation змінює спадкову структуру Genome.

Epigenetics змінює виконання Genome без зміни його структури.

```text
Mutation:
  Genome weight = 0.8
  ↓
  Genome weight = 0.6

Epigenetics:
  Genome weight = 0.8
  Epigenetic modifier = -0.2
  ↓
  Effective weight = 0.6
```

Після зникнення epigenetic modifier Genome залишається таким самим.

---

# Epigenetics vs Learning-like State

Learning-like state — це зміна стану матеріалів або сигнальної системи клітини на основі попереднього досвіду.

Epigenetics — ширший регуляторний стан клітини, який може впливати на Genome Runtime.

Вони можуть перетинатися.

Наприклад:

```text
Repeated signal
    ↓
Stateful Material changes coefficient
    ↓
Epigenetic State changes signal sensitivity
    ↓
Future Genome Runtime changes
```

Але learning-like state не повинен напряму переписувати Genome.

---

# Epigenetics vs Runtime State

Runtime State — це технічний стан виконання Regulatory Network.

Приклади:

* accumulated signal;
* previous activation;
* delayed signal;
* node memory;
* impulse state.

Epigenetic State — це ширший клітинний стан, який може змінювати runtime-параметри.

```text
Runtime State:
  accumulated_signal = 0.7

Epigenetic State:
  stress_mode = high
  dormancy_bias = medium
  repair_bias = high
```

Runtime State частіше короткочасний.

Epigenetic State може бути довшим і частково успадковуваним.

---

# Epigenetic State

`Epigenetic State` — це набір параметрів клітини, які впливають на виконання Genome.

Приклади:

```text
stress_level
dormancy_bias
repair_bias
growth_bias
division_bias
signal_sensitivity_modifier
material_synthesis_bias
boundary_repair_bias
mutation_rate_modifier
hgt_openness_modifier
development_stage
specialization_state
```

Це не обов'язковий фінальний список.

Важливо, що ці параметри не є новим Genome.

---

# Effective Parameters

Epigenetic State створює effective runtime parameters.

```text
inherited_parameter
+
epigenetic_modifier
    ↓
effective_parameter
```

Приклади:

```text
effective_weight =
genome_weight + epigenetic_weight_modifier
```

```text
effective_threshold =
genome_threshold + epigenetic_threshold_modifier
```

```text
effective_output_priority =
raw_output_priority × epigenetic_output_modifier
```

Формули можуть бути іншими в реалізації.

Принцип: Genome не змінюється, змінюється лише його виконання.

---

# Що може змінювати Epigenetic State

Epigenetic State може впливати на:

* input values;
* edge weights;
* node bias;
* node thresholds;
* node activation gain;
* output priorities;
* доступність процесів;
* priority allocation;
* lifecycle gating;
* mutation_rate;
* HGT openness;
* dormancy depth;
* development path.

Не все потрібно реалізовувати в MVP.

---

# Input Modifiers

Epigenetic State може змінювати сприйняття входів.

Приклад:

```text
raw_heat_input = 0.5
stress_modifier = +0.2

effective_heat_input = 0.7
```

Або:

```text
raw_signal_input = 0.6
dormancy_modifier = -0.4

effective_signal_input = 0.2
```

Це дозволяє клітині в різних станах по-різному реагувати на один і той самий сигнал.

---

# Weight Modifiers

Epigenetic State може тимчасово змінювати силу зв'язків у Regulatory Network.

```text
Genome edge weight = 0.8
Epigenetic modifier = -0.3
Effective edge weight = 0.5
```

Це не mutation.

Якщо epigenetic state зникне, Genome weight залишиться `0.8`.

---

# Threshold Modifiers

Epigenetic State може змінювати пороги активації.

Приклад:

```text
repair_threshold = 0.6
stress_modifier = -0.2
effective_repair_threshold = 0.4
```

У stressed state клітина швидше активує repair.

Або:

```text
division_threshold = 0.7
damage_modifier = +0.3
effective_division_threshold = 1.0
```

Пошкоджена клітина відкладає division.

---

# Output Modifiers

Epigenetic State може підсилювати або пригнічувати вихідні пріоритети.

Приклади:

```text
repair_boundary = repair_boundary × 1.5
movement = movement × 0.2
synthesis = synthesis × 0.5
dormancy = dormancy × 1.3
```

Output modifier не виконує процес.

Він лише змінює priority, який потім проходить Feasibility Check.

---

# Lifecycle Gating

Epigenetic State може бути пов'язаний із lifecycle state.

Наприклад:

```text
stressed:
  repair_bias high
  growth_bias low
  division_bias low

dormant:
  movement_bias low
  synthesis_bias low
  maintenance_bias medium

division_preparing:
  genome_copy_bias high
  boundary_expansion_bias high
```

Це не повинно перетворитися на hardcoded behavior tree.

Lifecycle gating має бути обмеженим шаром модифікації Runtime.

---

# Development State

Epigenetic State може зберігати development context.

Наприклад:

```text
development_stage = early
specialization_state = boundary_like
signal_environment = high
neighbor_contact = stable
```

Це дозволяє клітинам з однаковим Genome переходити в різні стабільні стани.

```text
Same Genome
+
Different Local Signals
+
Different Epigenetic State
    ↓
Different Material Composition
```

Так може виникати спеціалізація без hardcoded типів клітин.

---

# Specialization

Specialization — це стабільна різниця клітинних станів, яка виникає з локальної регуляції.

Клітина може стати більш:

* boundary-supporting;
* energy-producing;
* signal-conducting;
* storage-like;
* movement-supporting;
* repair-focused;
* joint-forming.

Але в рушії не існує hardcoded:

* NeuronCell;
* MuscleCell;
* SkinCell;
* SensorCell;
* StorageCell.

Ці ролі мають виникати з Materials, Genome Runtime, Epigenetic State і Joint context.

---

# Sources of Epigenetic State

Epigenetic State може змінюватися під впливом:

* Heat;
* Pressure;
* Energy shortage;
* Resource availability;
* Material damage;
* repeated signals;
* Joint signals;
* contact;
* lifecycle state;
* development history;
* stress;
* failed division;
* HGT fragment presence;
* internal clogging;
* environmental fields.

Усі джерела повинні бути локальними або внутрішніми для клітини.

---

# Stress-driven Epigenetics

Stress може змінювати Epigenetic State.

Приклади:

```text
low Energy
    ↓
increase dormancy_bias
decrease movement_bias
increase produce_energy_bias
```

```text
Boundary damage
    ↓
increase repair_bias
decrease division_bias
```

```text
high Heat
    ↓
increase heat_response_bias
decrease synthesis_bias
```

Це дозволяє клітині мати контекстну реакцію без зміни Genome.

---

# Signal-driven Epigenetics

Повторні сигнали можуть змінювати epigenetic context.

```text
repeated_joint_signal
    ↓
signal_sensitivity_modifier changes
    ↓
future response changes
```

Це може підтримувати learning-like behavior, але не повинно бути окремим learning-модулем.

Зміна може зберігатися певний час, деградувати або частково передаватися під час division.

---

# Material-driven Epigenetics

Матеріали можуть підтримувати epigenetic state.

Наприклад:

* stateful materials;
* signal-sensitive materials;
* memory-stable materials;
* stress-marker materials;
* damaged materials;
* repair-modulating materials.

```text
Material State
    ↓
Epigenetic Modifier
    ↓
Effective Genome Runtime
```

Це важливо: Epigenetics має матеріальну основу, а не є абстрактним прапорцем без фізики.

---

# Energy і Epigenetics

Epigenetic State може впливати на витрати Energy через пріоритети процесів.

Але сам Epigenetic State не створює Energy.

Він може:

* зменшити movement;
* зменшити synthesis;
* збільшити dormancy;
* підвищити produce_energy priority;
* зменшити non-critical processes;
* збільшити maintenance.

Якщо Energy недостатньо, навіть epigenetically boosted процес може не виконатися.

---

# HGT і Epigenetics

Наявність external або internal genetic fragment може впливати на Epigenetic State.

Наприклад:

* підвищити genome_isolation;
* активувати rejection;
* збільшити integration probability;
* збільшити stress;
* змінити repair priority;
* тимчасово приглушити деякі outputs.

Це не означає, що fragment автоматично став частиною Genome.

---

# Mutation Rate Modulation

Epigenetic State може впливати на effective mutation rate.

Наприклад:

```text
base_mutation_rate = 0.01
stress_modifier = ×2.0
effective_mutation_rate = 0.02
```

Це не directed mutation.

Клітина не обирає корисні зміни.

Вона лише змінює ймовірність випадкових змін залежно від стану.

---

# HGT Openness Modulation

Epigenetic State може тимчасово змінювати відкритість до HGT.

```text
low resources
    ↓
increase hgt_openness
```

або:

```text
many harmful fragments
    ↓
increase rejection
```

Але клітина не повинна знати наперед, чи fragment корисний.

Selection визначить, чи така стратегія вигідна.

---

# Epigenetic Duration

Epigenetic State може мати різну тривалість:

```text
short-term
medium-term
long-term
partially inherited
```

## short-term

Триває кілька Tick.

Приклад: реакція на Heat.

## medium-term

Триває багато Tick.

Приклад: stress після пошкодження.

## long-term

Може зберігатися значну частину життя клітини.

Приклад: specialization state.

## partially inherited

Може передаватися дочірнім клітинам частково.

Приклад: стартовий development bias.

---

# Epigenetic Decay

Epigenetic State не повинен бути вічним без механізму підтримки.

Він може поступово згасати.

```text
epigenetic_value_next =
epigenetic_value_current × (1 - decay_rate)
```

Або може підтримуватися, якщо існують відповідні Materials і сигнали.

```text
signal continues
+
stateful material stable
    ↓
epigenetic state persists
```

---

# Epigenetic Reinforcement

Повторні умови можуть підсилювати epigenetic state.

```text
repeated stress
    ↓
stress_level increases
```

```text
stable neighbor signal
    ↓
specialization_state reinforced
```

Це дозволяє клітині закріплювати контекст без зміни Genome.

---

# Epigenetic Reset

Певні події можуть скидати Epigenetic State:

* division;
* severe damage;
* death;
* dormancy exit;
* environmental change;
* loss of supporting Materials;
* Genome damage;
* failed repair;
* decomposition.

Reset може бути повним або частковим.

---

# Epigenetic Inheritance

Під час division частина Epigenetic State може передаватися дочірнім клітинам.

```text
daughter_epigenetic_state =
parent_epigenetic_state × inheritance_factor
+ noise
```

Дочірні клітини можуть отримати різний epigenetic state.

Це дозволяє asymmetric inheritance і ранню спеціалізацію.

---

# Epigenetic Asymmetry

Під час поділу Epigenetic State може розподілятися нерівномірно.

```text
Parent:
  specialization_state = signal_conducting

Daughter A:
  specialization_state = 0.8

Daughter B:
  specialization_state = 0.2
```

Це може дати різні стартові траєкторії розвитку.

---

# Epigenetics і Development

Development може бути реалізований через комбінацію:

* Genome Runtime;
* Epigenetic State;
* local signals;
* Materials;
* Joint context;
* lifecycle state.

```text
Local Signals
+
Epigenetic State
+
Genome Runtime
    ↓
Material Synthesis Bias
    ↓
Cell Specialization
```

Genome не зберігає готовий blueprint організму.

Він дає регуляторні правила, які проявляються через epigenetic context.

---

# Epigenetics і Organism

У багатоклітинних структурах epigenetic state може допомагати клітинам утримувати різні ролі.

Наприклад:

* одна клітина підтримує Boundary;
* інша проводить сигнал;
* інша синтезує Energy Materials;
* інша формує Joint;
* інша накопичує Resources.

Це не hardcoded tissue.

Це стабільні локальні стани, підтримані сигналами, Materials і Genome Runtime.

---

# Epigenetics і Neural-like Behavior

Neural-like behavior може спиратися на Epigenetic State.

Приклади epigenetic параметрів:

```text
signal_gain_modifier
activation_threshold_modifier
impulse_decay_modifier
signal_memory_stability
repeated_signal_sensitivity
```

Клітина з neural-like Materials може змінювати майбутню реакцію без зміни Genome.

```text
Repeated Signal
    ↓
Epigenetic / Material State
    ↓
Lower threshold or higher gain
    ↓
Stronger future response
```

Це не Genome mutation.

---

# Epigenetic State Storage

Epigenetic State повинен мати матеріальну або клітинну основу.

Він може зберігатися у:

* material coefficients;
* stateful materials;
* internal regulatory state;
* concentration of marker resources;
* Boundary state;
* Runtime State;
* local structural configuration.

Не треба робити Epigenetic State повністю абстрактним прапорцем без зв'язку з клітинною фізикою.

---

# MVP Epigenetics

Для MVP можна використати просту модель:

```text
EpigeneticState
├── stress_level
├── dormancy_bias
├── repair_bias
├── growth_bias
├── division_bias
└── signal_sensitivity_modifier
```

Цього достатньо для:

* stress response;
* dormancy;
* repair prioritization;
* suppression of division;
* early specialization experiments.

---

# MVP Update Rules

Простий варіант оновлення:

```text
if energy_level < threshold:
    stress_level += stress_gain
    dormancy_bias += dormancy_gain

if boundary_integrity < threshold:
    repair_bias += repair_gain
    division_bias -= division_suppression

if heat_level > threshold:
    stress_level += heat_stress_gain
    growth_bias -= growth_suppression

each Tick:
    epigenetic values decay toward baseline
```

Усі значення мають бути обмежені через `clamp`.

---

# MVP Runtime Integration

Під час Genome Runtime:

```text
1. Collect Inputs
2. Normalize Inputs
3. Apply Epigenetic Input Modifiers
4. Execute Regulatory Network
5. Apply Epigenetic Output Modifiers
6. Produce Process Priorities
```

Epigenetics не повинна запускати процеси напряму.

Вона лише змінює Runtime.

---

# MVP Inheritance

Для MVP epigenetic inheritance можна зробити простою:

```text
daughter_epigenetic_state =
parent_epigenetic_state × 0.5
+ small_noise
```

Або ще простіше:

```text
daughter_epigenetic_state = baseline
```

Рекомендація:

```text
MVP:
  stress_level partially inherited
  specialization_state reset or disabled
  runtime_state reset
```

---

# Technical Validation

Epigenetic State повинен бути технічно валідним:

* усі значення в допустимих межах;
* немає NaN;
* немає нескінченних значень;
* відомі поля;
* значення clamp після оновлення;
* state не посилається на неіснуючі node/process без перевірки.

Technical validation не означає, що epigenetic state корисний.

---

# Epigenetic Trace

Для дебагу корисно зберігати Epigenetic Trace.

Приклад:

```text
Tick 240
Cell 88

stress_level:
  0.20 -> 0.55

Reason:
  low energy
  boundary damage

Modifiers:
  repair_bias +0.30
  movement_bias -0.40
  division_bias -0.60
```

Trace не повинен бути обов'язковим у production-режимі.

Але він важливий для аналізу emergent behavior.

---

# Приклад 1. Stress після пошкодження Boundary

```text
Input:
  boundary_integrity = 0.3
  energy_level = 0.6

Epigenetic update:
  stress_level increases
  repair_bias increases
  division_bias decreases

Genome Runtime:
  repair_boundary output is amplified
  prepare_division output is suppressed

Result:
  cell prioritizes repair
```

Genome не змінився.

---

# Приклад 2. Dormancy через нестачу Energy

```text
Input:
  energy_level = 0.1
  resource_A_outside = 0.05

Epigenetic update:
  stress_level increases
  dormancy_bias increases
  movement_bias decreases
  synthesis_bias decreases

Runtime output:
  enter_dormancy high
  movement low
  growth low

Result:
  cell enters dormancy
```

---

# Приклад 3. Клітини з однаковим Genome стали різними

```text
Daughter A:
  receives strong neighbor signal
  epigenetic signal_state increases
  synthesizes signal-sensitive Materials

Daughter B:
  no neighbor signal
  epigenetic signal_state remains low
  synthesizes mostly Boundary Materials

Result:
  same Genome
  different cell states
```

Це основа майбутньої спеціалізації.

---

# Приклад 4. Epigenetic inheritance

```text
Parent:
  stress_level = 0.8

Division:
  Daughter A stress_level = 0.4
  Daughter B stress_level = 0.3

Result:
  daughters start more cautious,
  but state decays if stress disappears
```

Це не mutation.

---

# Приклад 5. Neural-like sensitivity

```text
Repeated Joint Signal:
  signal_input appears for many Tick

Epigenetic update:
  signal_sensitivity_modifier increases
  activation_threshold_modifier decreases

Future:
  weaker signal can activate response
```

Genome не змінився.

Клітина змінила реакцію через stateful Materials і epigenetic context.

---

# Приклад 6. Epigenetic reset

```text
Cell exits dormancy.

Reset:
  dormancy_bias decreases
  movement_bias returns to baseline
  growth_bias partially restores

Result:
  cell becomes active again
```

---

# Приклад 7. Bad epigenetic state

```text
Cell:
  high stress_level persists too long

Effect:
  growth suppressed
  division suppressed
  repair remains high

Result:
  cell survives but does not reproduce
  lineage may lose competition
```

Epigenetic response не завжди корисна.

Selection оцінює наслідки.

---

# Правила

## Rule 1. Epigenetics modifies Genome Runtime

Epigenetic State змінює виконання Genome, але не змінює Genome.

## Rule 2. Epigenetics is not mutation

Epigenetic modifier не є спадковою зміною Regulatory Network.

## Rule 3. Epigenetics is not learning

Learning-like behavior може використовувати Epigenetic State, але не зводиться до нього.

## Rule 4. Epigenetics has local causes

Epigenetic State змінюється через локальні або внутрішні стани клітини.

## Rule 5. Epigenetics may be partially inherited

Epigenetic State може передаватися дочірнім клітинам частково, але не як Genome mutation.

## Rule 6. Epigenetics decays or requires maintenance

Epigenetic State не повинен бути вічним без механізму підтримки.

## Rule 7. Epigenetics cannot bypass feasibility

Epigenetic boost не виконує процес без Resources, Materials, Energy і фізичних умов.

## Rule 8. Epigenetics can support specialization

Різні Epigenetic States можуть давати різні стани клітин з однаковим Genome.

## Rule 9. Epigenetics must be material-grounded

Epigenetic State має бути пов'язаний із матеріальним, ресурсним або внутрішнім станом клітини.

## Rule 10. Epigenetics must be deterministic with seed

Оновлення Epigenetic State повинно бути відтворюваним за однакових умов і seed.

---

# Заборонено

Не вводити:

* epigenetics as Genome rewrite;
* epigenetics as directed beneficial mutation;
* epigenetics as magic adaptation;
* inherited learned behavior as Genome change;
* species-level memory;
* global environment knowledge;
* hardcoded cell types;
* hardcoded tissue roles;
* epigenetic state that bypasses Resources, Materials or Energy;
* permanent epigenetic state without mechanism.

---

# Пов'язані документи

* `genetics/regulatory-network.md`
* `genetics/genome-runtime.md`
* `genetics/mutation.md`
* `genetics/inheritance.md`
* `genetics/heredity.md`
* `biology/genome.md`
* `biology/cell.md`
* `biology/processes.md`
* `biology/lifecycle.md`
* `biology/development.md`
* `biology/organism.md`
* `world/materials.md`
* `world/energy.md`
* `world/tick.md`

---

# ADR

Потрібні ADR:

```text
ADR-000X: Epigenetics Modifies Runtime, Not Genome
ADR-000X: Epigenetic State May Be Partially Inherited
ADR-000X: Epigenetics Must Be Material-grounded
ADR-000X: Specialization Emerges from Genome Runtime and Epigenetic State
```

---

# Open Questions

## MVP fields

Потрібно остаточно визначити мінімальний набір полів EpigeneticState для MVP.

Кандидати:

```text
stress_level
dormancy_bias
repair_bias
growth_bias
division_bias
signal_sensitivity_modifier
```

## Decay formula

Потрібно визначити формулу згасання epigenetic values.

## Inheritance factor

Потрібно визначити, яка частина Epigenetic State передається дочірнім клітинам.

## Material grounding

Потрібно вирішити, які epigenetic states мають бути прямо пов'язані з Materials, а які можуть бути агрегованим клітинним станом.

## Runtime integration

Потрібно визначити, чи epigenetics змінює:

* inputs;
* weights;
* thresholds;
* outputs;
* process priorities;
* lifecycle gating.

## Specialization

Потрібно визначити, чи specialization_state входить у MVP або відкладається до development/organism рівня.

## Neural-like behavior

Потрібно уточнити, де проходить межа між:

* Runtime State;
* Stateful Material;
* Epigenetic State;
* learning-like adaptation.

## Trace

Потрібно визначити мінімальний Epigenetic Trace для дебагу й аналізу.
