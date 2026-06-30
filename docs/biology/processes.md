# processes.md

> **Cell Processes — локальні процеси клітини**

---

# Призначення

Цей документ описує базові процеси, які можуть відбуватися всередині клітини або на її межі.

`Process` — це не поведінковий скрипт.

`Process` — це локальне перетворення стану клітини або взаємодія клітини із середовищем.

Процеси поєднують:

* Resources;
* Materials;
* Energy Buffer;
* Boundary;
* Genome regulation;
* Physics;
* Fields;
* Joint;
* Tick.

---

# Основна ідея

Клітина не “виконує поведінку” напряму.

Клітина має локальні процеси.

Genome лише регулює пріоритети цих процесів.

```text
Genome Output
    ↓
Process Priorities
    ↓
Feasibility Check
    ↓
Process Execution
    ↓
Cell State Change
```

Процес може виконатися лише тоді, коли для нього є фізичні умови.

---

# Що Process НЕ є

Process не є:

* hardcoded поведінкою;
* органом;
* готовою функцією клітини;
* магічною дією;
* обходом фізики;
* прямим створенням ресурсів;
* прямим створенням матеріалів;
* прямим створенням Energy без механізму.

Process завжди працює через існуючий стан світу.

---

# Контракт процесів

Детальні cross-cutting правила винесені в окремі документи:

* `biology/feasibility.md` — універсальна перевірка active planned actions;
* `biology/process-progress.md` — накопичення прогресу довгих процесів без часткового фінального результату;
* `biology/process-capabilities.md` — зв'язок Materials → Capabilities → allowed process set;
* `world/reactions.md` — семантика passive і controlled reactions.

Цей документ описує доменні процеси, а ці контракти визначають їхню виконуваність.

---

# Загальна структура Process

Кожен процес має:

```text
Process
├── inputs
├── required_materials
├── required_energy
├── required_conditions
├── duration
├── outputs
├── side_effects
└── failure_mode
```

---

# Inputs

`inputs` — це те, що процес споживає або використовує.

Приклади:

* Resource;
* Material;
* Energy;
* Field influence;
* Heat;
* Signal;
* Contact;
* Joint state;
* free_capacity.

---

# Required Materials

Більшість активних процесів потребують відповідних матеріалів.

Наприклад:

```text
Resource + Catalytic Material -> Energy
```

або:

```text
Resource + Energy + Synthesis-capable Material -> Material
```

Матеріал не є процесом сам по собі.

Матеріал лише дозволяє або покращує процес.

---

# Required Energy

Активні процеси потребують Energy Buffer.

Пасивні процеси Energy не потребують.

Недостатня Energy може призвести до:

* скасування процесу;
* зменшення швидкості;
* деградації матеріалів;
* накопичення незавершених станів.

Активний процес не виконується, якщо Energy недостатньо для його повного виконання.

Якщо у Tick заплановано кілька активних процесів і Energy недостатньо для всього набору planned actions, planned actions не виконуються в цьому Tick.

Обов'язкові витрати підтримки обробляються окремо від planned actions.

---

# Required Conditions

Процес може вимагати певних умов:

* достатній free_capacity;
* наявність Boundary;
* контакт з ресурсом;
* контакт з іншою клітиною;
* наявність Joint;
* допустимий Heat;
* допустимий Pressure;
* доступ до Field;
* наявність потрібного ресурсу;
* наявність потрібного матеріалу.

---

# Outputs

Процес може створювати або змінювати:

* Resource;
* Material;
* Energy Buffer;
* Heat;
* Boundary integrity;
* Joint;
* signal;
* genome state;
* epigenetic state;
* physical state.

---

# Side Effects

Процес може мати побічні наслідки:

* Heat;
* Resource Product;
* Waste Resource;
* Material degradation;
* Energy loss;
* Pressure;
* signal emission;
* local trace;
* mutation risk.

Побічний ефект не є помилкою.

Він є частиною фізичної моделі процесу.

---

# Failure Mode

Якщо процес не може виконатися, він не повинен створювати результат магічно.

Можливі режими відмови:

* процес не запускається;
* створюється побічний продукт;
* виникає Heat;
* пошкоджується Material;
* клітина переходить у stressed або dormant state.

---

# Пасивні процеси

Пасивні процеси не потребують Energy і не залежать напряму від Genome.

Вони відбуваються через фізику, хімію або середовище.

Приклади:

* diffusion;
* passive transport;
* natural degradation;
* heat transfer;
* collision;
* pressure;
* passive leakage;
* material decay;
* resource reaction without active control.

Пасивні процеси все одно можуть впливати на клітину.

---

# Активні процеси

Активні процеси потребують Energy, матеріалів і регуляції.

Приклади:

* active transport;
* resource export;
* energy production;
* material synthesis;
* repair;
* controlled degradation;
* movement;
* Joint creation;
* Joint maintenance;
* division preparation;
* genome copying;
* dormancy regulation.

Активний процес не запускається лише тому, що умови існують.

Потрібен регуляторний сигнал.

---

# Feasibility Check

Перед виконанням активного процесу рушій перевіряє, чи можливий процес.

Перевіряються:

```text
Energy available
Resources available
Materials available
Free capacity
Boundary state
Physics constraints
Joint state
Cell lifecycle state
Local environment
```

Genome не може обійти Feasibility Check.

---

# Конфлікт процесів за Energy

Якщо кілька процесів одночасно хочуть використати однакові ресурси або Energy, рушій не повинен випадково обрати перший процес у порядку обходу.

Для базової моделі діє консервативне правило:

* mandatory costs обробляються першими, якщо Energy достатньо;
* якщо Energy недостатньо для всього набору planned actions, planned actions не виконуються;
* порядок ітерації Systems або процесів не створює пріоритету.

Приклад:

```text
Energy available = 10

repair_boundary wants 6
produce_material wants 5
movement wants 4
```

Клітина не може виконати все повністю.

У цьому Tick planned actions не виконуються.

---

# Process Progress

Довгі процеси можуть мати накопичуваний прогрес.

Наприклад:

```text
synthesis_progress
repair_progress
division_progress
genome_copy_progress
joint_build_progress
```

Якщо процес перервано, прогрес може:

* зберегтися;
* частково деградувати;
* повністю втратитися;
* створити побічний продукт.

Це визначається конкретним процесом.

---

# Resource Uptake

`Resource Uptake` — це потрапляння ресурсу в клітину.

Ресурс може потрапити через:

* пасивну дифузію;
* активний транспорт;
* пошкоджену Boundary;
* Joint;
* поглинання фрагмента.

```text
External Resource
    ↓
Boundary / Joint
    ↓
Internal Resource
```

Resource Uptake не означає, що ресурс корисний.

Після входу він може:

* бути використаний;
* накопичитися;
* вступити в реакцію;
* бути виведеним;
* засмітити клітину.

---

# Passive Transport

`Passive Transport` не потребує Energy.

Він залежить від:

* градієнта концентрації;
* permeability Boundary;
* властивостей ресурсу;
* diffusion_rate;
* пошкоджень Boundary;
* фізичних умов.

Genome може лише непрямо впливати на пасивний транспорт через зміну Boundary materials.

---

# Active Transport

`Active Transport` потребує:

* Energy;
* pump-capable Material;
* регуляторного сигналу;
* доступного ресурсу;
* фізичної можливості;
* вільного об'єму або зовнішнього напрямку.

```text
Genome Regulation
    ↓
Pump-capable Material
    ↓
Energy Buffer
    ↓
Resource Transport
```

Active Transport може працювати проти градієнта.

---

# Resource Export

`Resource Export` — це виведення ресурсу з клітини.

Причини:

* видалення зайвого ресурсу;
* створення сліду;
* передача ресурсу іншій клітині;
* звільнення внутрішнього об'єму;
* регуляція реакцій;
* захист від засмічення.

Resource Export може бути пасивним або активним.

---

# Resource Reaction

`Resource Reaction` — це перетворення ресурсів через `reaction_profile`.

Приклади:

```text
Acid + Base -> Neutral Resource + Heat
```

```text
Oxidizer + Reducer + Catalytic Material -> Energy + Resource Product
```

```text
Resource + Energy -> High-energy Resource
```

Resource Reaction може бути:

* пасивною;
* каталізованою матеріалом;
* контрольованою клітиною;
* шкідливою;
* корисною.

У світі немає окремої властивості `toxicity`.

Шкідливість виникає через реакції, об'єм, Heat або деградацію Material.

---

# Energy Production

`Energy Production` — це процес поповнення Energy Buffer.

Energy може бути отримана через:

* ресурс із `energy_value`;
* реакцію ресурсів;
* Field за участі відповідного Material;
* руйнування Material, якщо така реакція визначена.

```text
Resource
    +
Catalytic Material
    ↓
Energy Buffer
    +
Reaction Products
```

```text
Light
    +
Photosensitive Material
    ↓
Energy Buffer
```

Energy Production не може відбутися без механізму.

Resource або Field самі по собі не створюють Energy Buffer.

---

# Energy Consumption

`Energy Consumption` — це витрата Energy Buffer на активну роботу.

Energy витрачається на:

* transport;
* synthesis;
* repair;
* movement;
* Joint operations;
* genome copying;
* learning;
* maintenance;
* division.

Якщо Energy недостатньо, активний процес не виконується.

---

# Heat Production

Heat може виникати як побічний результат:

* надлишку Energy;
* реакцій;
* тертя;
* зіткнення;
* деградації;
* неефективного процесу.

Heat може:

* прискорювати реакції;
* пошкоджувати Materials;
* передаватися через контакт;
* передаватися через Joint;
* впливати на сусідні клітини.

Heat не є Energy Buffer.

---

# Material Synthesis

`Material Synthesis` — це створення Material з Resources.

Синтез потребує:

* Resource recipe;
* Energy;
* synthesis-capable Material або процес;
* free_capacity;
* часу;
* регуляторного сигналу.

```text
Resources
    +
Energy
    +
Synthesis-capable Material
    ↓
Material
```

Material не виникає напряму з Genome.

Genome лише регулює синтез.

---

# Material Repair

`Material Repair` — це відновлення пошкодженого Material.

Repair потребує:

* Energy;
* Resources;
* відповідного Material або процесу;
* часу;
* регуляторного сигналу.

Repair не відновлює HP.

Repair відновлює матеріальний склад.

---

# Material Degradation

`Material Degradation` — це втрата або перетворення Material.

Degradation може бути:

* природною;
* спричиненою Heat;
* спричиненою Pressure;
* спричиненою Resource Reaction;
* спричиненою зіткненням;
* контрольованою клітиною;
* наслідком нестачі Energy для maintenance.

Результатом можуть бути:

* Resource;
* простіший Material;
* Material fragment;
* Heat;
* waste resource.

---

# Controlled Degradation

Клітина може навмисно руйнувати власні Materials.

Причини:

* отримання Resources;
* отримання Energy, якщо реакція дозволена;
* перебудова структури;
* видалення пошкодженого Material;
* підготовка до поділу;
* адаптація до середовища.

Controlled Degradation потребує регуляції.

Може потребувати Energy.

---

# Boundary Repair

`Boundary Repair` — спеціальний випадок Material Repair.

Він відновлює матеріали Boundary.

Потребує:

* Energy;
* Resources;
* Boundary-compatible Materials;
* часу;
* регуляторного сигналу.

Boundary Repair важливий, бо Boundary утримує внутрішній стан клітини.

Але він не є HP recovery.

---

# Maintenance

`Maintenance` — це підтримка складних матеріалів і внутрішнього стану.

Maintenance може потребувати Energy навіть тоді, коли клітина не виконує активних дій.

Якщо Maintenance неможливий:

* Materials деградують;
* Boundary слабшає;
* Energy production падає;
* Genome може пошкоджуватися;
* клітина втрачає функціональність.

Maintenance не повинен бути великим прихованим податком У базовій моделі, але має існувати як принцип.

---

# Movement

`Movement` — це активна зміна фізичного стану клітини.

Movement потребує:

* Energy;
* contractile або movement-capable Material;
* регуляторного сигналу;
* фізичної можливості;
* контакту або середовища, через яке рух має сенс.

```text
Genome Regulation
    ↓
Movement-capable Material
    ↓
Energy
    ↓
Force / Movement
```

Movement не є hardcoded поведінкою.

Він виникає з матеріалів і регуляції.

---

# Shape Change

`Shape Change` — це зміна форми клітини або Boundary.

Може виникати через:

* Pressure;
* Joint;
* Movement;
* contractile Material;
* growth;
* damage;
* division preparation.

У базовій моделі shape може бути спрощена до кола, а Shape Change — до зміни radius або elasticity.

---

# Signal Production

`Signal Production` — це створення локального впливу для себе або інших клітин.

Сигнал може бути:

* Resource release;
* Material trace;
* Joint signal;
* Heat change;
* mechanical signal;
* Field-related response.

Сигнал не є глобальною командою.

Інші клітини можуть реагувати лише якщо мають відповідні матеріали або регуляторні входи.

---

# Signal Adaptation

`Signal Adaptation` — це процес зміни реакції клітини на основі попередніх сигналів.

Він може змінювати:

- signal threshold;
- signal gain;
- impulse accumulation;
- impulse decay rate;
- connection weight;
- sensitivity to repeated signals;
- output intensity.

Signal Adaptation потребує:

- stateful Material;
- Energy, якщо зміна активна;
- сигналу або історії сигналів;
- регуляторного дозволу Genome;
- часу.

```text
Repeated Signal
    ↓
Stateful Material
    ↓
Coefficient Change
    ↓
Future Response Changes
```

Signal Adaptation не змінює Genome напряму.

Це зміна внутрішнього стану клітини або матеріалу.


---

# Trace Emission

`Trace Emission` — це повільне виділення Materials або Resources у середовище.

```text
Cell Material
    ↓
External Trace
    ↓
Resource / Signal / Degradation Product
```

Trace може:

* бути відчутим іншими клітинами;
* розпадатися;
* бути використаним як Resource;
* створювати шлях або локальну мітку.

Немає hardcoded species marker.

---

# Joint Creation

`Joint Creation` — це створення матеріального зв'язку між клітинами.

Потребує:

* контакту;
* Energy;
* відповідних Boundary materials;
* сумісності поверхонь;
* регуляторного сигналу;
* фізичної можливості.

```text
Cell A Boundary
    +
Cell B Boundary
    +
Energy
    ↓
Joint
```

Joint не виникає дистанційно.

---

# Joint Maintenance

`Joint Maintenance` підтримує існуючий Joint.

Може потребувати:

* Energy;
* Materials;
* Resources;
* сигналів від клітин;
* допустимого Pressure;
* допустимого Heat.

Якщо Joint не підтримується, він може деградувати або розірватися.

---

# Joint Breaking

`Joint Breaking` може бути:

* пасивним через руйнування;
* активним через регуляторне рішення;
* наслідком Pressure;
* наслідком Heat;
* наслідком Resource Reaction;
* наслідком нестачі Maintenance.

Активне розривання може потребувати Energy.

---

# Genome Copying

`Genome Copying` — це копіювання генетичного матеріалу.

Потребує:

* Resources;
* Energy;
* free_capacity;
* часу;
* регуляторного сигналу;
* відповідного Material або процесу.

Копіювання може створювати:

* точну копію;
* мутацію;
* пошкоджену копію;
* неповну копію;
* дубльований фрагмент.

---

# Genome Repair

`Genome Repair` — це відновлення пошкодженого Genome.

Потребує:

* Resources;
* Energy;
* відповідного Material або процесу;
* часу.

Repair не гарантує ідеального результату.

Він може виправити пошкодження або створити нову помилку.

---

# Epigenetic Update

`Epigenetic Update` — це зміна тимчасового регуляторного стану клітини.

Він може залежати від:

* Heat;
* Energy;
* сигналів;
* ресурсів;
* пошкоджень;
* історії процесів;
* Joint;
* lifecycle state.

Epigenetic Update не змінює Genome напряму.

---

# Dormancy

`Dormancy` — це зниження активності клітини.

У dormancy клітина може:

* зменшити Energy consumption;
* зменшити transport;
* зупинити movement;
* зменшити synthesis;
* зберігати ресурси;
* чекати кращих умов.

Dormancy не є окремим типом клітини.

Це регуляторний стан.

---

# Growth

`Growth` — це збільшення матеріальної структури або внутрішньої місткості клітини.

Growth потребує:

* Resources;
* Energy;
* Material Synthesis;
* Boundary expansion;
* free external space;
* регуляторного сигналу.

Growth не є просто збільшенням числа `size`.

Він є наслідком матеріального синтезу.

---

# Division Preparation

`Division Preparation` — це підготовка клітини до поділу.

Вона може включати:

* накопичення Resources;
* збільшення Materials;
* копіювання Genome;
* збільшення Boundary;
* накопичення Energy;
* формування division readiness;
* перебудову внутрішнього стану.

Сам поділ описується в `biology/lifecycle.md`.

---

# Cell Death Trigger

Processes не повинні містити команду `kill(cell)`.

Але процеси можуть призвести до стану, у якому клітина більше не життєздатна.

Причини:

* Boundary failure;
* critical material loss;
* Genome destruction;
* complete clogging;
* irreversible Heat damage;
* inability to maintain Energy Buffer;
* inability to repair.

Смерть визначається структурним станом клітини.

---

# Decomposition

`Decomposition` — це розпад мертвої клітини.

Після смерті Materials, Resources і Genome fragments можуть:

* залишитися у світі;
* розпастися;
* вступити в реакції;
* бути поглинутими іншими клітинами;
* стати джерелом HGT;
* стати ресурсами.

Мертва клітина не зникає миттєво.

---

# Process Execution у Tick

Процеси виконуються в Action Execution phase.

Загальний порядок:

```text
1. Feasibility Check
2. Transport
3. Reactions
4. Energy Production / Consumption
5. Material Synthesis / Repair / Degradation
6. Mechanics / Movement / Joint
7. Genome Copy / Epigenetic Update
8. Lifecycle Updates
9. Cleanup
```

Цей порядок може уточнюватися в `world/tick.md` або `engine/scheduler.md`.

---

# Determinism

За однакових умов і однакового random seed процеси повинні давати однаковий результат.

Це потрібно для:

* тестування;
* дебагу;
* відтворюваності;
* наукового аналізу.

---

# Randomness

Randomness дозволена лише там, де це явно визначено:

* mutation;
* genome copying errors;
* reaction variation;
* degradation;
* damage distribution;
* stochastic transport;
* fragment integration.

Randomness не повинна підміняти фізику або регуляцію.

---

# Мінімальний набір процесів Для базової моделі

Для першої життєздатної симуляції достатньо:

* passive resource diffusion;
* passive uptake;
* active uptake;
* energy production;
* material synthesis;
* material degradation;
* boundary repair;
* maintenance;
* growth;
* genome copying;
* division preparation;
* death / decomposition.

Movement, Joint, HGT, learning і складні сигнали можна додавати пізніше.

---

# Правила

## Rule 1. Processes transform state

Process змінює стан клітини або локального середовища.

## Rule 2. Active processes require Energy

Активні процеси потребують Energy Buffer.

## Rule 3. Passive processes do not require Energy

Пасивні процеси виникають через фізику, хімію або дифузію.

## Rule 4. Processes require materials

Більшість активних процесів потребують відповідних Materials.

## Rule 5. Genome regulates processes

Genome задає пріоритети процесів, але не виконує їх напряму.

## Rule 6. Feasibility check is mandatory

Кожен активний процес проходить перевірку можливості виконання.

## Rule 7. No magic creation

Process не може створити Resource, Material або Energy без визначеного механізму.

## Rule 8. Damage is material change

Пошкодження процесами означає зміну або втрату Materials.

## Rule 9. Waste is Resource state

Waste не є окремою сутністю.

Це ресурс або продукт реакції, який займає об'єм, має низьку корисність або створює небажані реакції.

## Rule 10. Process model must scale

Процеси повинні працювати для:

* однієї клітини;
* колонії;
* багатоклітинного організму;
* 2D;
* майбутнього 3D.

---

# Заборонено

Не вводити:

* hardcoded behavior scripts;
* HP repair;
* poison damage;
* magic detox;
* instant material creation;
* free Energy;
* process without physical inputs;
* direct species recognition;
* direct organism-level command;
* separate process rules for “plant” або “animal”.

---

# Пов'язані документи

* `biology/cell.md`
* `biology/membrane.md`
* `biology/feasibility.md`
* `biology/process-progress.md`
* `biology/process-capabilities.md`
* `biology/genome.md`
* `biology/lifecycle.md`
* `biology/joint.md`
* `world/resources.md`
* `world/materials.md`
* `world/energy.md`
* `world/reactions.md`
* `world/physics.md`
* `world/tick.md`
* `engine/scheduler.md`

# Open Questions

## Process duration

Потрібно уточнити конкретний список atomic actions і long-running processes для першої реалізації згідно з `biology/process-progress.md`.

## Failure modes

Потрібно деталізувати per-process наслідки після rejected або failed execution:

* повне скасування;
* побічний продукт;
* Heat;
* material damage.

## базова модель process list

Потрібно остаточно затвердити мінімальний список процесів для першої реалізації.

## Process formulas

Потрібно винести точні формули в `engine/chemistry.md`, `engine/physics.md` або `engine/scheduler.md`.


