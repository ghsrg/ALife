# joint.md

> **Joint — матеріальний зв'язок між клітинами**

---

# Призначення

Цей документ описує `Joint` — матеріальний зв'язок між двома або більше клітинами.

Joint дозволяє клітинам утворювати:

* колонії;
* ланцюги;
* тканиноподібні структури;
* органоподібні структури;
* багатоклітинні організми;
* сигнальні мережі;
* транспортні мережі;
* механічні структури.

Joint не є окремим органом.

Joint не є нервом, судиною, м'язом або тканиною.

Joint — це універсальний матеріальний механізм з'єднання клітин.

---

# Основна ідея

Клітини можуть існувати окремо.

Але якщо між ними виникає стабільний матеріальний зв'язок, вони можуть утворювати структуру вищого рівня.

```text
Cell A
  ↓
Joint
  ↓
Cell B
```

Joint може передавати:

* силу;
* механічне навантаження;
* Resources;
* Signals;
* Heat;
* локальний стан;
* обмеження руху.

Joint не передає Energy Buffer напряму як абстрактне число.

Але через Joint можуть передаватися Resources з `energy_value`, продукти реакцій, Heat або сигнали, які змінюють Energy production у клітині-одержувачі.

---

# Що Joint НЕ є

Joint не є:

* магічним зв'язком;
* телепатією між клітинами;
* глобальною мережею організму;
* нервом у hardcoded сенсі;
* кровоносною судиною;
* м'язом;
* органом;
* тканиною;
* direct organism controller;
* способом передавати Energy напряму;
* способом обійти фізику.

Joint завжди має матеріальну основу.

---

# Joint як фізична структура

Joint є фізичною структурою.

Він може мати:

```text
joint_id
cell_a
cell_b
joint_materials
strength
elasticity
length
rest_length
max_length
conductivity
permeability
signal_capacity
resource_capacity
heat_conductivity
damage_state
stability
```

Не всі поля потрібні у базовій моделі.

Для базової моделі достатньо:

```text
cell_a
cell_b
strength
permeability
signal_strength
resource_transfer_rate
damage_state
```

---

# Матеріальна основа Joint

Joint формується з Materials клітин.

Для створення Joint потрібні:

* контакт між клітинами;
* Boundary materials;
* Energy;
* Resources;
* регуляторний сигнал Genome;
* фізична можливість;
* сумісність поверхонь або Materials.

```text
Cell A Boundary
+
Cell B Boundary
+
Joint-capable Materials
+
Energy
    ↓
Joint
```

Joint не виникає без матеріального процесу.

---

# Joint-capable Materials

Будь-який Material може брати участь у Joint, якщо має відповідні властивості.

Приклади властивостей:

```text
adhesion
joint_affinity
elasticity
strength
conductivity
permeability
signal_conductivity
resource_transport_efficiency
heat_conductivity
repairability
```

Матеріал не є готовим Joint.

Він лише створює потенційну можливість для Joint.

---

# Joint і Boundary

Joint зазвичай створюється через Boundary клітини.

Boundary визначає:

* чи може клітина приєднатися;
* наскільки міцним буде з'єднання;
* чи можна передавати Resources;
* чи можна передавати Signals;
* чи можна передавати Heat;
* чи є ризик витоку внутрішніх Resources;
* чи може Joint бути розірваний.

```text
Boundary Material
    ↓
Joint Formation
```

Якщо Boundary пошкоджена, Joint може бути нестабільним або небезпечним.

---

# Типи Joint

У базовій моделі не потрібно hardcode багато типів Joint.

Але концептуально Joint може мати різні режими:

```text
adhesive_joint
mechanical_joint
transport_joint
signal_joint
mixed_joint
temporary_joint
```

Це не повинні бути жорсткі біологічні класи.

Краще вважати їх результатом різних параметрів одного Joint.

---

# Adhesive Joint

Adhesive Joint просто утримує клітини разом.

Він може:

* обмежувати відстань;
* передавати механічне навантаження;
* стабілізувати колонію;
* утримувати клітини в контакті;
* бути основою для майбутнього transport або signal.

```text
Cell A -- adhesion -- Cell B
```

---

# Mechanical Joint

Mechanical Joint передає силу.

Він може працювати як:

* пружний зв'язок;
* жорсткий зв'язок;
* гнучкий зв'язок;
* шарнір;
* розтяжний зв'язок.

У базовій моделі достатньо спростити його до spring-like моделі.

```text
force =
stiffness × stretch
```

Це не обов'язкова фінальна формула, а базовий принцип.

---

# Transport Joint

Transport Joint дозволяє передавати Resources між клітинами.

```text
Cell A Resource
    ↓
Joint
    ↓
Cell B Resource
```

Передача може бути:

* пасивною;
* активною;
* регульованою;
* обмеженою permeability;
* обмеженою capacity;
* залежною від Energy;
* залежною від Materials.

Transport Joint не передає Energy Buffer напряму.

---

# Signal Joint

Signal Joint дозволяє передавати сигнали.

Сигнал може бути:

* хімічним;
* механічним;
* тепловим;
* електричним у майбутній моделі;
* імпульсним;
* накопичуваним;
* затухаючим.

```text
Cell A Signal Output
    ↓
Joint
    ↓
Cell B Signal Input
```

Signal Joint може бути основою neural-like мереж без hardcoded NeuronCell.

---

# Mixed Joint

Більшість реальних Joint у симуляції можуть бути змішаними.

Один Joint може одночасно:

* тримати клітини разом;
* передавати слабкий сигнал;
* передавати частину Resources;
* проводити Heat;
* передавати механічне навантаження.

Тому краще не робити багато окремих класів.

Краще мати один Joint з параметрами.

---

# Temporary Joint

Temporary Joint — короткочасне з'єднання.

Воно може виникати для:

* контакту;
* обміну Resources;
* передачі fragment;
* короткого сигналу;
* репродуктивної взаємодії;
* HGT;
* тимчасової колонії.

Temporary Joint може швидко деградувати, якщо його не підтримувати.

---

# Створення Joint

Joint створюється через процес `Joint Creation`.

Потрібні умови:

```text
cells are close enough
compatible Boundary materials exist
Energy available
Resources available
Genome output requests joint creation
physical space allows connection
```

Приклад:

```text
Genome Output:
  create_joint = 0.8

Feasibility Check:
  contact exists
  joint-capable material exists
  Energy sufficient

Result:
  Joint created
```

---

# Joint не створюється дистанційно

Клітина не може створити Joint з далекою клітиною.

Потрібні:

* контакт;
* близькість;
* матеріальний місток;
* або вже існуюча структура.

Це відповідає принципам locality і physics.

---

# Підтримка Joint

Joint може потребувати Maintenance.

Maintenance може включати:

* ремонт Joint Materials;
* поповнення Resources;
* витрати Energy;
* підтримку натягу;
* стабілізацію signal pathway;
* контроль permeability;
* захист від Heat або Pressure.

Якщо Joint не підтримується, він може деградувати.

---

# Joint Cost

Joint не є безкоштовним.

Він може мати вартість:

* створення;
* підтримки;
* ремонту;
* передачі Resources;
* передачі Signals;
* механічного навантаження;
* runtime processing;
* додаткової маси;
* додаткового ризику пошкодження.

Це важливо, щоб організми не створювали нескінченну кількість Joint без ціни.

---

# Руйнування Joint

Joint може руйнуватися через:

* надмірне розтягнення;
* зіткнення;
* Pressure;
* Heat;
* деградацію Materials;
* нестачу Maintenance;
* реакції Resources;
* активне рішення клітини;
* смерть однієї з клітин;
* розрив під час movement;
* division.

Руйнування Joint означає втрату або пошкодження матеріальної структури.

Це не HP damage.

---

# Joint Damage

Joint Damage — це зміна або втрата Joint Materials.

Наслідки:

* зменшення strength;
* зменшення conductivity;
* зменшення resource transfer;
* збільшення leakage;
* зниження signal quality;
* підвищення chance rupture;
* механічна нестабільність.

Joint Damage повинен бути пов'язаний з Materials.

---

# Joint Repair

Joint може ремонтуватися.

Потрібні:

* Energy;
* Resources;
* repair-capable Materials;
* доступ із однієї або обох клітин;
* регуляторний сигнал;
* час.

```text
Damaged Joint
+
Resources
+
Energy
+
Repair Process
    ↓
Restored Joint Materials
```

Joint Repair не відновлює HP.

Він відновлює матеріальний стан Joint.

---

# Передача Resources через Joint

Joint може дозволяти Resource Transfer.

Передача залежить від:

* resource_transfer_rate;
* Joint permeability;
* Resource properties;
* концентрації в обох клітинах;
* регуляції;
* Energy, якщо transfer активний;
* direction control;
* damage_state;
* capacity.

```text
Resource amount in A > Resource amount in B
    ↓
passive flow through Joint
```

або:

```text
Genome Regulation
+
Energy
+
Joint transport Material
    ↓
active transfer through Joint
```

---

# Passive Resource Transfer

Passive transfer не потребує Energy.

Він виникає через різницю концентрацій або тиску.

Приклад:

```text
Cell A has Resource A = 100
Cell B has Resource A = 10

Joint permeability > 0

Resource A moves from A to B
```

Passive transfer може бути корисним або шкідливим.

---

# Active Resource Transfer

Active transfer потребує:

* Energy;
* transport-capable Joint Materials;
* регуляторного сигналу;
* доступного Resource;
* приймаючої клітини з free_capacity;
* функціонального Joint.

Active transfer може працювати проти градієнта.

```text
Cell A pushes Resource A to Cell B
```

Це може бути основою майбутніх органоподібних транспортних систем.

---

# Передача Energy через Joint

Energy Buffer не передається напряму.

Заборонено:

```text
Cell A Energy Buffer -> Cell B Energy Buffer
```

Дозволено:

```text
Cell A Resource with energy_value -> Cell B
Cell A reaction product -> Cell B
Heat transfer through Joint
Signal causing Cell B to produce Energy
```

Тобто Joint може підтримувати енергетичну кооперацію, але не магічний обмін Energy.

---

# Heat Transfer through Joint

Joint може передавати Heat.

Передача залежить від:

* heat_conductivity;
* температурної різниці;
* Joint Materials;
* damage_state;
* contact area;
* Heat capacity клітин.

Heat Transfer може бути:

* корисним для охолодження;
* шкідливим через перегрів сусідів;
* способом стабілізації температури колонії;
* джерелом cascading damage.

---

# Signal Transfer through Joint

Joint може передавати Signals.

Сигнал може мати параметри:

```text
signal_type
signal_strength
signal_decay
signal_delay
direction
frequency
accumulation
```

У базовій моделі достатньо:

```text
signal_value
signal_decay
signal_direction
```

Signal Transfer не є командою.

Приймаюча клітина може реагувати лише якщо має відповідний InputNode і Material basis.

---

# Signal Direction

Joint Signal може бути:

* одностороннім;
* двостороннім;
* асиметричним;
* затухаючим;
* активним;
* пасивним.

```text
Cell A -> Cell B
Cell A <-> Cell B
```

Direction може залежати від Joint Materials або регуляції клітин.

---

# Signal Delay

Сигнал може передаватися не миттєво.

Delay може виникати через:

* довжину Joint;
* Materials;
* signal type;
* Heat;
* damage;
* Energy state.

Для базової моделі delay можна не реалізовувати.

Але модель повинна дозволяти його в майбутньому.

---

# Signal Accumulation

Клітина може накопичувати повторні сигнали, якщо має stateful Materials або Runtime State.

```text
Repeated Joint Signal
    ↓
Accumulated Signal
    ↓
Threshold crossed
    ↓
Output response
```

Це дозволяє neural-like behavior без hardcoded Neuron.

Joint лише передає сигнал.

Накопичення відбувається у клітині або Material State.

---

# Neural-like Joint Networks

У рушії не існує hardcoded neural network.

Але якщо клітини мають:

* signal-sensitive Materials;
* stateful Materials;
* signal-conductive Joint;
* threshold-based Runtime;
* adaptive signal gain;
* Energy для підтримки сигналів,

то може виникнути neural-like мережа.

```text
Signal Cell A
    ↓
Joint
    ↓
Signal Cell B
    ↓
Joint
    ↓
Signal Cell C
```

Це emergent структура, а не окремий тип органу.

---

# Joint і Movement

Joint впливає на movement.

Якщо одна клітина рухається, Joint може:

* тягнути сусідню клітину;
* обмежувати рух;
* створювати натяг;
* розтягуватися;
* передавати силу;
* розірватися;
* деформувати клітини.

```text
Cell A movement
    ↓
Joint force
    ↓
Cell B displacement
```

---

# Joint і Shape

Joint може впливати на форму клітин і колонії.

Багато Joint можуть створювати:

* ланцюг;
* сітку;
* шар;
* грудку;
* нитку;
* оболонку;
* внутрішній каркас;
* тканиноподібну структуру.

Форма не задається blueprint.

Вона виникає з локальних Joint, growth і physics.

---

# Joint і Pressure

Joint може передавати Pressure або механічне навантаження.

Якщо структура стискається, Joint може:

* передати навантаження сусіднім клітинам;
* деформуватися;
* пошкодитися;
* розірватися;
* змінити signal state;
* активувати pressure-sensitive Inputs у клітин.

Це може бути основою механічного sensing.

---

# Joint і Division

Коли клітина ділиться, Joint можуть:

* залишитися з однією дочірньою клітиною;
* розірватися;
* перебудуватися;
* створити новий Joint між дочірніми клітинами;
* розподілити Joint Materials;
* стати пошкодженими.

Для базової моделі можна використати просте правило:

```text
Existing Joint follows the daughter cell closest to original attachment point.
```

Або ще простіше:

```text
Division breaks existing external Joints unless explicitly maintained.
```

Потрібно вибрати в `lifecycle.md` або `engine/physics.md`.

---

# Joint і Death

Якщо клітина помирає, її Joint не повинні зникати миттєво.

Можливі варіанти:

* Joint деградує;
* Joint розривається;
* Joint лишається як Material fragment;
* Joint передає Resources з мертвої клітини;
* Joint проводить Heat;
* Joint стає механічним вантажем;
* сусідні клітини можуть його розібрати.

Мертва клітина залишається матеріальним об'єктом.

---

# Joint і Decomposition

Під час decomposition Joint Materials можуть:

* розпастися в Resources;
* залишитися як fragments;
* бути поглинутими іншими клітинами;
* вступити в реакції;
* передати залишкові Resources;
* стати джерелом локального сліду.

Це підтримує conservation of matter.

---

# Joint і HGT

Joint може бути шляхом горизонтального переносу genetic fragments.

```text
Cell A Genetic Fragment
    ↓
Joint
    ↓
Cell B
```

Але це не повинно бути у базовій моделі.

Для майбутньої моделі потрібні:

* fragment-compatible Joint;
* permeability для genetic fragments;
* genome isolation;
* uptake regulation;
* integration model.

---

# Joint і Organism

Organism — це граф клітин, пов'язаних Joint.

```text
Organism =
Cells
+
Joints
```

Joint є ребрами графа організму.

```text
Cell = node
Joint = edge
```

Це дозволяє описувати організм без hardcoded тіла, органів або тканин.

---

# Joint Graph

На рівні багатоклітинності структура може бути представлена як граф:

```text
OrganismGraph = (Cells, Joints)
```

Властивості графа:

* connected components;
* degree;
* paths;
* clusters;
* resource flow;
* signal flow;
* mechanical stability;
* damage propagation.

Це не означає, що організм має глобальний контролер.

Це аналітична і engine-level структура.

---

# Колонія

Колонія може виникнути, якщо клітини з'єднані Joint, але не мають сильної спеціалізації або централізованої залежності.

```text
Cell -- Joint -- Cell -- Joint -- Cell
```

Колонія може мати:

* спільну стабільність;
* частковий обмін Resources;
* локальні Signals;
* спільний Heat balance;
* слабку координацію.

---

# Тканиноподібна структура

Тканиноподібна структура може виникнути, якщо група клітин має:

* подібний Material composition;
* багато стабільних Joint;
* спільну механічну роль;
* локальний resource або signal flow;
* схожий epigenetic state.

У рушії не існує hardcoded `Tissue`.

Tissue-like pattern є emergent кластером клітин.

---

# Органоподібна структура

Органоподібна структура може виникнути, якщо група спеціалізованих клітин виконує стабільну функцію для більшої структури.

Наприклад:

* група клітин проводить сигнал;
* група клітин накопичує Resource;
* група клітин виробляє Energy-related Materials;
* група клітин створює механічну оболонку;
* група клітин транспортує Resources.

У рушії не існує hardcoded `Organ`.

Орган — це emergent функціональний підграф.

---

# Joint і Specialization

Joint може підтримувати спеціалізацію.

Клітина може спеціалізуватися, якщо:

* отримує стабільні сигнали через Joint;
* отримує Resources від сусідів;
* передає продукт іншим;
* має інший epigenetic state;
* має інше механічне навантаження;
* знаходиться в іншій позиції графа.

```text
Same Genome
+
Different Joint Context
    ↓
Different Cell State
```

Це буде деталізовано в `specialization.md`.

---

# Joint і Communication

Communication між клітинами може відбуватися через:

* Joint signals;
* Resource transfer;
* Heat transfer;
* mechanical force;
* local trace;
* contact;
* shared environment.

Joint є одним із головних каналів communication, але не єдиним.

Це буде деталізовано в `communication.md`.

---

# базова модель Joint Model

Для базової моделі достатньо простої моделі:

```text
Joint
├── cell_a
├── cell_b
├── strength
├── resource_transfer_rate
├── signal_value
├── signal_decay
├── damage
└── active
```

базова модель підтримує:

* створення Joint при контакті;
* розрив Joint;
* пасивний Resource transfer;
* простий Signal transfer;
* механічне обмеження відстані;
* damage/degradation;
* basic repair.

---

# базова модель Joint Creation

Простий алгоритм:

```text
1. Cell A and Cell B are close enough.
2. Both have compatible Boundary materials.
3. At least one cell outputs create_joint priority.
4. Feasibility Check passes.
5. Resources and Energy are consumed.
6. Joint object is created.
```

---

# базова модель Joint Update per Tick

Кожен Tick Joint може:

```text
1. Apply mechanical constraint.
2. Transfer Resources if allowed.
3. Transfer Signal if present.
4. Transfer Heat if enabled.
5. Apply degradation.
6. Apply repair if requested.
7. Break if damage or stretch exceeds threshold.
```

Порядок має бути узгоджений з `world/tick.md` і `engine/scheduler.md`.

---

# Technical Validation

Joint technically valid, якщо:

* обидві клітини існують;
* Joint має унікальний id;
* strength не NaN;
* resource_transfer_rate у допустимих межах;
* signal values у допустимих межах;
* damage у допустимих межах;
* немає broken references;
* Joint не з'єднує неіснуючі клітини;
* Joint не дублюється неконтрольовано.

Technical validity не означає, що Joint корисний.

---

# Joint Trace

Для дебагу корисно зберігати Joint Trace.

Приклад:

```text
Tick 340
Joint 12
Cell A -> Cell B:
  Resource A transferred: 2.4
  Signal value: 0.7
  Damage: 0.1 -> 0.15
```

Trace не повинен бути увімкнений завжди для всіх Joint.

Але він дуже важливий для аналізу emergent багатоклітинності.

---

# Приклад 1. Проста колонія

```text
Cell A and Cell B touch.
Both have adhesive Boundary Materials.
Cell A outputs create_joint = 0.8.

Joint is created.

Result:
  cells stay close
  weak Resource sharing becomes possible
```

Це початок colony-like structure.

---

# Приклад 2. Resource sharing

```text
Cell A:
  Resource A = 100

Cell B:
  Resource A = 10

Joint:
  resource_transfer_rate = 0.2

Result:
  Resource A moves from A to B
```

Якщо Cell B використовує Resource A для repair, Joint підвищує виживання структури.

---

# Приклад 3. Сигнал через Joint

```text
Cell A:
  detects high Heat

Cell A Genome Runtime:
  output_signal = 0.9

Joint:
  transfers signal to Cell B

Cell B:
  receives joint_signal = 0.7
  increases dormancy_bias
```

Це не глобальна команда.

Cell B реагує лише через власний Genome Runtime і Materials.

---

# Приклад 4. Розрив через рух

```text
Cell A moves away.
Cell B remains in place.

Joint:
  stretch exceeds max_length

Result:
  Joint damage increases
  Joint breaks
```

Розрив є фізичним наслідком, а не командою видалення.

---

# Приклад 5. Neural-like pathway

```text
Cell A:
  receives external signal

Cell A:
  sends signal through Joint

Cell B:
  accumulates repeated signal

Cell B:
  threshold crossed
  sends signal to Cell C
```

Так виникає neural-like signal chain без hardcoded neurons.

---

# Приклад 6. Механічна оболонка

```text
Many cells form strong adhesive Joints.

Result:
  cluster becomes mechanically stable
  outer cells experience Pressure
  inner cells receive different signals
```

Це може стати основою tissue-like або organ-like структури.

---

# Приклад 7. Смерть клітини в колонії

```text
Cell A dies.

Joint to Cell B remains temporarily.

Cell B:
  receives Resource leakage from Cell A
  may absorb decomposition products

Joint:
  degrades over time
```

Мертва клітина не зникає миттєво.

---

# Правила

## Rule 1. Joint is material

Joint має матеріальну основу і не є магічним зв'язком.

## Rule 2. Joint connects cells locally

Joint виникає між близькими або контактними клітинами.

## Rule 3. Joint can transfer Resources

Joint може передавати Resources, але не Energy Buffer напряму.

## Rule 4. Joint can transfer Signals

Joint може передавати сигнали, але не команди.

## Rule 5. Joint can transfer force

Joint бере участь у physics і може передавати механічне навантаження.

## Rule 6. Joint can degrade

Joint може пошкоджуватися, старіти, руйнуватися і ремонтуватися.

## Rule 7. Joint has cost

Створення, підтримка і ремонт Joint не є безкоштовними.

## Rule 8. Joint enables multicellularity

Багатоклітинність виникає як граф Cells + Joints.

## Rule 9. No hardcoded tissues or organs

Тканини й органи не є класами рушія.

Вони виникають як стабільні патерни клітин і Joint.

## Rule 10. Joint must scale

Joint model повинна працювати для:

* двох клітин;
* колонії;
* тканиноподібної структури;
* багатоклітинного організму;
* 2D;
* майбутнього 3D.

---

# Заборонено

Не вводити:

* magic connection;
* direct Energy Buffer transfer;
* global organism bus;
* hardcoded nerve;
* hardcoded blood vessel;
* hardcoded muscle connection;
* hardcoded tissue class;
* hardcoded organ class;
* species_id compatibility;
* joint without Materials;
* joint without cost;
* instant disappearance after cell death.

---

# Пов'язані документи

* `biology/cell.md`
* `biology/membrane.md`
* `biology/processes.md`
* `biology/lifecycle.md`
* `biology/genome.md`
* `biology/communication.md`
* `biology/specialization.md`
* `biology/organism.md`
* `world/materials.md`
* `world/resources.md`
* `world/energy.md`
* `world/physics.md`
* `world/tick.md`
* `genetics/genome-runtime.md`
* `genetics/epigenetics.md`
* `genetics/horizontal-transfer.md`

# Open Questions

## базова модель Joint physics

Потрібно вирішити, чи Joint У базовій моделі буде:

* spring-like;
* rigid;
* adhesive only;
* distance constraint;
* soft-body connection.

## Resource transfer

Потрібно визначити формулу transfer:

* за градієнтом;
* за permeability;
* з active transport;
* з capacity limit.

## Signal model

Потрібно визначити базовий сигнал:

* scalar signal;
* typed signal;
* decaying signal;
* delayed signal;
* accumulated signal.

## Joint creation

Потрібно визначити, чи Joint створюється:

* якщо одна клітина хоче;
* якщо обидві клітини хочуть;
* якщо є contact + compatible Materials;
* через окремий negotiation process.

## Joint during division

Потрібно вибрати правило для поділу клітини з існуючими Joint.

## Joint death behavior

Потрібно визначити, що стається з Joint після смерті однієї клітини.

## HGT through Joint

Потрібно вирішити, чи Joint може передавати genetic fragments у базовій моделі або у майбутньому.

## Organism boundary

Потрібно визначити, коли connected component Cells + Joints вважати organism, colony або просто temporary cluster.


