# specialization.md

> **Specialization — виникнення різних функціональних станів клітин**

---

# Призначення

Цей документ описує `Specialization` — процес, у якому клітини набувають різних стабільних функціональних станів.

Specialization дозволяє багатоклітинним структурам формувати:

* colony-like структури;
* tissue-like структури;
* organ-like підграфи;
* signal-processing clusters;
* resource-transport clusters;
* boundary-supporting layers;
* movement-supporting structures;
* storage-like regions.

Specialization не є hardcoded типом клітини.

У рушії не існує готових класів:

* `NeuronCell`;
* `MuscleCell`;
* `SkinCell`;
* `SensorCell`;
* `StorageCell`;
* `TransportCell`;
* `LeafCell`;
* `RootCell`.

Такі ролі можуть виникати лише через Materials, Genome Runtime, Epigenetic State, Joint context, local signals і selection.

---

# Основна ідея

Клітини з однаковим або схожим Genome можуть стати різними через різні локальні умови.

```text
Same Genome
+
Different Local Inputs
+
Different Epigenetic State
+
Different Joint Context
+
Different Materials
    ↓
Different Cell State
    ↓
Specialization
```

Specialization — це не команда рушія.

Specialization — це стабільний наслідок локальної регуляції.

---

# Що Specialization НЕ є

Specialization не є:

* hardcoded cell type;
* класом клітини;
* blueprint органу;
* species-specific rule;
* прямим наказом організму;
* поведінковим скриптом;
* global role assignment;
* результатом `if cell_position == X`;
* готовою біологічною тканиною.

Specialization не повинна задаватися рушієм напряму.

Рушій може лише надати універсальні механізми, через які спеціалізація виникає.

---

# Джерела Specialization

Specialization може виникати через:

* різні локальні Fields;
* різні Resource gradients;
* різний доступ до Energy;
* різний Material composition;
* різні Joint connections;
* різні Signals;
* різний Pressure;
* різний Heat;
* різний contact context;
* різний Epigenetic State;
* asymmetric inheritance;
* damage history;
* lifecycle state;
* local position у cell-joint graph.

---

# Specialization і Genome

Genome не містить готовий список типів клітин.

Genome містить регуляторні правила.

```text
Inputs
    ↓
Genome Runtime
    ↓
Process Priorities
    ↓
Materials / Energy / Signals / Joint
    ↓
Cell State
```

Specialization виникає тоді, коли регуляторні правила стабільно приводять клітину до певного Material і Process profile.

---

# Specialization і Epigenetics

Epigenetic State є ключовим механізмом спеціалізації.

Він може змінювати:

* process priorities;
* material synthesis bias;
* repair bias;
* signal sensitivity;
* growth bias;
* division bias;
* dormancy bias;
* Joint formation tendency;
* response thresholds.

```text
Local Signal
    ↓
Epigenetic State
    ↓
Different Genome Runtime
    ↓
Different Material Composition
```

Epigenetic State не змінює Genome.

Але він може зробити клітини з однаковим Genome різними.

---

# Specialization і Materials

Матеріали визначають функціональні можливості клітини.

Клітина може спеціалізуватися через накопичення певних Materials.

Приклади Material-based ролей:

```text
high strength + elasticity
    ↓
mechanical support-like role
```

```text
high signal_conductivity + state_plasticity
    ↓
neural-like signal role
```

```text
high pump_efficiency + permeability control
    ↓
transport-like role
```

```text
high energy_conversion_efficiency
    ↓
energy-production-like role
```

```text
high storage_capacity
    ↓
storage-like role
```

Ці ролі не є класами.

Це emergent функціональні стани.

---

# Specialization і Joint

Joint створює локальний контекст, який може підтримувати спеціалізацію.

Через Joint клітина може:

* отримувати Resources;
* передавати Resources;
* отримувати Signals;
* передавати Signals;
* отримувати Pressure;
* передавати Force;
* отримувати Heat;
* бути частиною стабільної структури.

```text
Joint Context
    ↓
Local Inputs
    ↓
Epigenetic State
    ↓
Specialized Cell State
```

Клітина в центрі графа і клітина на краю графа можуть мати різні стани навіть з однаковим Genome.

---

# Specialization і Communication

Communication підтримує стабільні відмінності між клітинами.

Приклади:

* зовнішні клітини отримують більше Pressure і Field input;
* внутрішні клітини отримують більше Resource signals від сусідів;
* клітини на транспортному шляху отримують повторні Joint signals;
* клітини біля пошкодження отримують stress signals;
* клітини поруч із Resource source отримують інший gradient.

```text
Different Communication Context
    ↓
Different Runtime Output
    ↓
Different Specialization
```

---

# Specialization і Asymmetric Inheritance

Під час поділу дочірні клітини можуть отримати різний стартовий стан.

Відрізнятися можуть:

* Materials;
* Resources;
* Energy Buffer;
* Epigenetic State;
* Runtime State;
* damage;
* Heat;
* Joint context.

```text
Parent Cell
    ↓
Daughter A ≠ Daughter B
    ↓
Different Development Path
```

Asymmetric inheritance може бути джерелом ранньої спеціалізації.

---

# Specialization і Development

Development — це процес, у якому клітини поступово формують різні стани в межах зростаючої структури.

Specialization є частиною Development.

```text
Growth
+
Joint Formation
+
Communication
+
Epigenetic Changes
    ↓
Specialized Cell Patterns
```

Genome не повинен містити готовий blueprint організму.

Він повинен дозволяти локальні правила, які можуть створити стабільні структури.

---

# Specialization Profile

Для аналізу можна описувати поточний профіль клітини.

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

Це аналітична структура.

Вона не повинна напряму керувати поведінкою клітини.

---

# Material Profile

`material_profile` описує, які Materials домінують у клітині.

Наприклад:

```text
structural_material_high
boundary_material_high
signal_material_high
storage_material_high
contractile_material_high
energy_conversion_material_high
```

Це допомагає аналізувати emergent roles.

Але це не hardcoded cell type.

---

# Process Profile

`process_profile` описує, які процеси клітина виконує найчастіше або з найвищими пріоритетами.

Приклади:

```text
repair-focused
energy-production-focused
transport-focused
signal-focused
growth-focused
joint-maintenance-focused
dormant
```

Це результат Genome Runtime і Feasibility Check.

---

# Signal Profile

`signal_profile` описує, як клітина працює з сигналами.

Клітина може:

* переважно отримувати сигнали;
* переважно передавати сигнали;
* накопичувати сигнали;
* підсилювати сигнали;
* приглушувати сигнали;
* бути майже нечутливою.

Це може бути основою neural-like або coordination-like ролей.

---

# Joint Profile

`joint_profile` описує роль клітини в графі Joint.

Параметри:

* кількість Joint;
* сила Joint;
* resource flow через Joint;
* signal flow через Joint;
* mechanical load;
* position in connected component;
* damage propagation role.

Клітина з багатьма Joint може виконувати hub-like роль, але це не hardcoded hub class.

---

# Energy Profile

`energy_profile` описує енергетичний стан клітини.

Клітина може бути:

* energy-producing-like;
* energy-consuming-like;
* storage-like;
* heat-producing;
* heat-dissipating;
* energy-starved;
* dormant.

Energy role виникає через Materials і Resources, а не через клас.

---

# Boundary-supporting Specialization

Клітини можуть спеціалізуватися на підтримці зовнішньої межі структури.

Вони можуть мати:

* високий рівень Boundary Materials;
* сильні Joint;
* високу repair priority;
* високу pressure resistance;
* низьку permeability;
* високу реакцію на damage.

```text
Outer Position
+
Pressure / Contact
+
Boundary Signals
    ↓
Boundary-supporting State
```

Це може створити skin-like або shell-like структуру без hardcoded SkinCell.

---

# Transport-like Specialization

Клітини можуть спеціалізуватися на передачі Resources.

Вони можуть мати:

* високий pump_efficiency;
* transport-capable Materials;
* багато Transport Joint;
* регуляцію Resource export/import;
* високу permeability control;
* низьку власну витрату Resources.

```text
Resource-rich region
    ↓
Transport cells
    ↓
Resource-poor region
```

Це може створити vessel-like або channel-like структуру без hardcoded судин.

---

# Signal-processing Specialization

Клітини можуть спеціалізуватися на обробці сигналів.

Вони можуть мати:

* signal-sensitive Materials;
* signal-conductive Joint;
* stateful Materials;
* impulse accumulation;
* threshold activation;
* adaptive gain;
* low movement;
* high signal output.

```text
Incoming Signal
    ↓
Accumulation / Threshold
    ↓
Output Signal
```

Це neural-like роль, але не hardcoded NeuronCell.

---

# Mechanical Specialization

Клітини можуть спеціалізуватися на механічній підтримці або русі.

Вони можуть мати:

* high strength;
* high elasticity;
* contractile Materials;
* strong Joint;
* pressure sensitivity;
* movement-capable Materials;
* high repair of structural Materials.

Це може створити muscle-like або bone-like функції без hardcoded м'язів чи кісток.

---

# Storage-like Specialization

Клітини можуть накопичувати Resources або Materials.

Вони можуть мати:

* high internal capacity;
* storage Materials;
* low activity;
* low permeability;
* controlled export;
* high stability;
* dormancy bias.

Це може підтримувати інші клітини в колонії або організмі.

---

# Energy-production-like Specialization

Клітини можуть виробляти Energy або енергетичні Resources для структури.

Вони можуть мати:

* energy-conversion Materials;
* доступ до Resource з `energy_value`;
* доступ до Field;
* high heat resistance;
* Resource export через Joint;
* Heat management.

Вони не передають Energy Buffer напряму.

Але можуть передавати Resources, продукти реакцій або Heat.

---

# Repair-focused Specialization

Клітини можуть спеціалізуватися на ремонті власної або сусідньої структури.

Вони можуть:

* виробляти repair Materials;
* передавати repair Resources;
* підтримувати Joint;
* реагувати на damage signals;
* мати високий repair_bias;
* бути розташованими біля зон навантаження.

Це не hardcoded immune system.

Це локальна repair роль.

---

# Reproductive Specialization

У майбутньому деякі клітини можуть спеціалізуватися на reproduction-related процесах.

Наприклад:

* Genome copying;
* fragment export;
* fusion readiness;
* gamete-like export;
* high mutation control;
* division support.

Це не повинно бути hardcoded male/female.

Репродуктивні ролі мають виникати через регуляцію і selection.

---

# Specialization Stability

Specialization може бути:

```text
temporary
stable
reversible
irreversible
partially inherited
```

## temporary

Клітина тимчасово змінює стан через stress або signal.

## stable

Стан підтримується тривалими Signals, Materials або Joint context.

## reversible

Клітина може повернутися до іншого стану.

## irreversible

Клітина втратила можливість повернення через Materials або Genome damage.

## partially inherited

Частина стану передається дочірнім клітинам.

---

# Reversible Specialization

Reversible specialization дозволяє клітині змінювати роль залежно від умов.

Приклад:

```text
Resource-rich environment
    ↓
storage-like state

Resource-poor environment
    ↓
export / repair / dormancy state
```

Це корисно для простих колоній.

---

# Irreversible Specialization

Irreversible specialization може виникнути, якщо клітина:

* втратила частину Materials;
* змінила Material composition радикально;
* має stable epigenetic lock;
* не може більше ділитися;
* не може підтримувати інші процеси;
* стала повністю залежною від сусідів.

Це може бути корисним для складних організмів, але ризикованим.

---

# Specialization і Dependency

Спеціалізована клітина може стати залежною від інших клітин.

Наприклад:

* signal cell потребує Energy Resources від сусідів;
* boundary cell потребує repair Resources від внутрішніх клітин;
* storage cell потребує transport Joint;
* reproductive cell потребує protection.

```text
Higher specialization
    ↓
Higher dependency
```

Це створює перехід від колонії до organism-like структури.

---

# Specialization і Fitness

Selection не оцінює спеціалізацію напряму.

Specialization корисна лише якщо вона покращує survival або reproduction lineage.

```text
Specialized Pattern
    ↓
Better Structure Function
    ↓
More Survival / Reproduction
```

Погана спеціалізація може бути шкідливою.

Наприклад, клітина може стати signal-focused, але структура не має користі від сигналів.

---

# Tissue-like Pattern

Tissue-like pattern виникає, якщо група клітин має стабільно схожий стан.

```text
Cells with similar:
  Materials
  Epigenetic State
  Joint Context
  Process Profile
    ↓
Tissue-like cluster
```

Tissue-like cluster не є класом рушія.

Це аналітичний або emergent патерн.

---

# Organ-like Pattern

Organ-like pattern виникає, якщо група спеціалізованих клітин разом виконує стабільну функцію.

Приклади:

* signal-processing cluster;
* resource transport path;
* boundary layer;
* mechanical support region;
* storage region;
* reproductive region.

```text
Specialized Cells
+
Joint Graph
+
Communication
+
Resource Flow
    ↓
Organ-like Function
```

Organ-like structure не задається blueprint.

---

# Colony vs Organism

Колонія може мати слабку або нестабільну спеціалізацію.

Організмоподібна структура має сильнішу залежність між клітинами.

```text
Colony:
  cells can often survive separately

Organism-like structure:
  cells increasingly depend on shared structure
```

Specialization є одним із ключових переходів від colony до organism.

---

# Specialization і Death

Смерть спеціалізованої клітини може мати різні наслідки.

Якщо вона була weakly specialized, структура може майже не постраждати.

Якщо вона була critical hub або transport cell, смерть може викликати:

* resource starvation;
* signal loss;
* mechanical collapse;
* Heat imbalance;
* Joint break cascade;
* death of dependent cells.

Це створює selection pressure на redundancy і repair.

---

# Specialization і Redundancy

Складні структури можуть потребувати redundancy.

Наприклад:

* кілька transport paths;
* кілька signal cells;
* дублювання boundary-supporting cells;
* repair резерви;
* alternative resource flows.

Redundancy має cost, але підвищує стійкість.

---

# Specialization і Development Failure

Development може створити погану спеціалізацію.

Приклади:

* занадто багато storage-like cells;
* немає boundary-supporting cells;
* signal cells без transport cells;
* reproductive cells без Energy support;
* transport cells без Resource source;
* cells depend on Joint that часто рветься.

Такі структури допустимі.

Selection відфільтрує невдалі патерни.

---

# базова модель Specialization

Для базової моделі не потрібно вводити явну систему cell types.

Достатньо логувати й аналізувати:

```text
material_profile
process_profile
joint_profile
signal_profile
energy_profile
epigenetic_profile
```

базова модель може мати просту класифікацію лише для debug UI.

Наприклад:

```text
observed_role = "repair-focused"
observed_role = "signal-focused"
observed_role = "storage-like"
```

Але `observed_role` не повинен керувати поведінкою.

---

# базова модель Role Detection

Role detection може бути аналітичною функцією.

```text
if signal_material_high and joint_signal_output_high:
    observed_role = signal-focused
```

Це лише label для людини.

Клітина не повинна читати власний `observed_role`.

---

# Specialization Metrics

Для аналізу можна зберігати:

```text
cell_id
material_profile
process_profile
epigenetic_state
joint_degree
resource_flow_in
resource_flow_out
signal_flow_in
signal_flow_out
energy_balance
division_rate
repair_rate
death_risk
observed_role
```

Ці метрики не є частиною біологічної поведінки.

---

# Приклад 1. Boundary layer

```text
Cluster grows.

Outer cells:
  receive Pressure
  contact environment
  lose more Boundary Material
  receive more damage signals

Genome Runtime:
  repair_boundary high
  synthesize_boundary_material high
  movement low

Result:
  outer cells become boundary-supporting.
```

Це skin-like роль без `SkinCell`.

---

# Приклад 2. Transport path

```text
Cell A near Resource source.
Cell C far from Resource source.
Cell B connects A and C through Joint.

Cell B:
  receives Resource from A
  exports Resource to C
  maintains transport-capable Joint

Result:
  Cell B becomes transport-like.
```

Це channel-like структура без hardcoded vessel.

---

# Приклад 3. Signal chain

```text
Cell A receives Pressure.
Cell A sends signal to Cell B.
Cell B accumulates signal.
Cell B sends signal to Cell C.

Repeated use:
  Cell B synthesizes more signal-conductive Material.

Result:
  Cell B becomes signal-processing-like.
```

Це neural-like спеціалізація без hardcoded neurons.

---

# Приклад 4. Storage cell

```text
Cell receives excess Resource.
Growth is limited.
Export is low.
Storage Materials increase.

Result:
  cell becomes storage-like.
```

Якщо сусіди можуть отримати Resource через Joint, така спеціалізація може бути корисною.

---

# Приклад 5. Погана спеціалізація

```text
Cell specializes into signal-focused state.

But:
  no other cells respond to its signals
  signal production has cost
  cell does not help structure survive

Result:
  lineage loses resources and may be selected out.
```

Specialization не гарантує користь.

---

# Приклад 6. Залежна клітина

```text
Cell becomes highly signal-focused.

It has:
  low Energy production
  high signal output
  low Resource uptake

It survives only because neighbor cells send Resources through Joint.

Result:
  cell is organism-dependent.
```

Це ознака переходу до справжньої багатоклітинності.

---

# Приклад 7. Irreversible specialization

```text
Cell stops producing division-support Materials.
Cell accumulates structural Materials.
Cell maintains strong Joint.

Result:
  cell becomes mechanical support-like
  but cannot reproduce independently.
```

Це може бути корисно для організму, але не для окремої клітини.

---

# Правила

## Rule 1. Specialization is emergent

Specialization виникає з локальної регуляції, Materials, Epigenetic State і Joint context.

## Rule 2. No hardcoded cell types

Рушій не повинен мати готові типи клітин на кшталт NeuronCell або MuscleCell.

## Rule 3. Same Genome can produce different states

Клітини з однаковим Genome можуть мати різні стани через різні локальні умови.

## Rule 4. Epigenetics supports specialization

Epigenetic State може стабілізувати або змінювати спеціалізацію.

## Rule 5. Materials define functional capability

Функціональна роль клітини виникає з Material composition і процесів.

## Rule 6. Joint context matters

Позиція клітини в Joint graph може впливати на її specialization.

## Rule 7. Specialization can create dependency

Чим сильніша спеціалізація, тим більше клітина може залежати від інших.

## Rule 8. Tissue and organ patterns are emergent

Тканини й органи є стабільними патернами, а не класами рушія.

## Rule 9. Observed roles are analytical labels

Будь-які role labels потрібні лише для аналізу й UI, не для поведінки клітин.

## Rule 10. Selection filters specialization

Корисність спеціалізації визначається survival і reproduction lineage.

---

# Заборонено

Не вводити:

* hardcoded cell_class;
* hardcoded tissue_class;
* hardcoded organ_class;
* hardcoded neuron;
* hardcoded muscle;
* hardcoded skin;
* hardcoded storage cell;
* global role assignment;
* organism blueprint;
* species-specific specialization;
* observed_role as behavior input.

---

# Пов'язані документи

* `biology/cell.md`
* `biology/joint.md`
* `biology/communication.md`
* `biology/organism.md`
* `biology/development.md`
* `biology/processes.md`
* `biology/lifecycle.md`
* `genetics/regulatory-network.md`
* `genetics/genome-runtime.md`
* `genetics/epigenetics.md`
* `genetics/heredity.md`
* `world/materials.md`
* `world/resources.md`
* `world/energy.md`
* `world/physics.md`

# Open Questions

## базова модель specialization tracking

Потрібно вирішити, чи базова модель буде логувати specialization metrics.

## Observed role labels

Потрібно визначити, чи debug UI показує observed roles:

* signal-focused;
* transport-like;
* storage-like;
* repair-focused;
* boundary-supporting;
* movement-supporting.

## Specialization stability

Потрібно визначити, як вимірювати стабільність спеціалізації в часі.

## Epigenetic lock

Потрібно вирішити, чи потрібен механізм epigenetic lock для довготривалої спеціалізації.

## Dependency metrics

Потрібно визначити, як вимірювати залежність клітини від організму або колонії.

## Tissue detection

Потрібно вирішити, чи tissue-like patterns будуть лише аналітикою, чи потрібні engine-level connected component metrics.

## Organ-like detection

Потрібно визначити, як виявляти organ-like функціональні підграфи без hardcoded органів.

## Irreversible specialization

Потрібно вирішити, чи дозволяти клітинам втрачати здатність до самостійного reproduction У базовій моделі.


