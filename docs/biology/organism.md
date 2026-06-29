# organism.md

> **Organism — стабільна багатоклітинна структура з взаємозалежних клітин**

---

# Призначення

Цей документ описує `Organism` — багатоклітинну структуру, яка виникає з клітин, Joint, communication, specialization і shared lifecycle.

Organism не є hardcoded сутністю.

Organism не є готовим класом `Animal`, `Plant`, `Fungus`, `Body` або `Creature`.

Organism — це emergent структура, яка може бути розпізнана як стабільний граф клітин, що разом підтримують життя, ріст, repair, reproduction або іншу довготривалу функціональність.

Світ не використовує `Organism` як поведінкову сутність.

Observer, debug UI, lineage tools і research metrics можуть відстежувати `Organism` як derived analytical view над Cells + Joints + Genome lineage.

---

# Основна ідея

На базовому рівні organism — це граф:

```text id="c8z2ub"
Organism =
Cells
+
Joints
+
Communication
+
Resource flows
+
Specialization
+
Shared stability
```

У формальному вигляді:

```text id="te7w9l"
OrganismGraph = (Cells, Joints)
```

де:

```text id="3jf2qo"
Cell = node
Joint = edge
```

Але не кожен connected graph клітин є organism.

Проста група клітин може бути temporary cluster або colony.

Organism виникає тоді, коли між клітинами з’являється стійка функціональна взаємозалежність.

---

# Що Organism НЕ є

Organism не є:

* hardcoded body;
* глобальним контролером;
* централізованим мозком;
* species class;
* готовою твариною;
* готовою рослиною;
* набором органів із шаблону;
* blueprint, записаним у Genome;
* контейнером, який керує клітинами напряму;
* магічною надклітинною сутністю;
* способом обійти фізику клітин.

Organism не повинен виконувати процеси замість клітин.

Усі процеси виконуються клітинами.

Organism — це рівень аналізу й emergent організації.

---

# Відмінність від Colony

`Colony` — це група клітин, які можуть бути з'єднані або жити поруч, але значна частина клітин може виживати самостійно.

```text id="ajgaor"
Colony:
  weak dependency
  loose coordination
  cells may survive separately
```

`Organism` — це структура, де клітини дедалі більше залежать одна від одної.

```text id="vu2mum"
Organism:
  strong dependency
  stable communication
  specialization
  shared repair / resource flow
  cells may fail outside the structure
```

Межа між colony і organism не є абсолютно чіткою.

Це континуум.

---

# Colony-to-Organism Continuum

Багатоклітинність може розвиватися поступово.

```text id="ru7m6s"
single cell
    ↓
temporary cluster
    ↓
colony
    ↓
cooperative colony
    ↓
specialized colony
    ↓
organism-like structure
    ↓
highly integrated organism
```

Рушій не повинен насильно класифікувати структуру на ранніх етапах.

Краще вимірювати показники:

* connectedness;
* dependency;
* resource sharing;
* signal coordination;
* specialization;
* reproduction coupling;
* death propagation;
* repair coordination.

---

# Мінімальні ознаки Organism

Структуру можна вважати organism-like, якщо вона має кілька ознак:

```text id="t2bvb1"
stable connected component
persistent Joints
resource sharing
signal communication
cell specialization
shared repair
coordinated growth
shared reproduction path
dependency between cells
```

Не всі ознаки обов'язкові.

Але що більше їх є, то більше структура схожа на organism.

---

# Connected Component

Найпростіша технічна основа organism — connected component у Cell-Joint Graph.

```text id="a5ogca"
Cell A -- Joint -- Cell B -- Joint -- Cell C
```

Connected component не дорівнює organism автоматично.

Але це кандидат на organism-like structure.

---

# Organism Boundary

Organism може мати зовнішню межу, але вона не є окремою магічною оболонкою.

Organism boundary виникає з:

* outer cells;
* Boundary materials;
* Joint network;
* mechanical shape;
* resource gradients;
* communication context;
* external pressure;
* environmental exposure.

```text id="qnu8nb"
outer cells
+
strong Joints
+
Boundary-supporting specialization
    ↓
organism-like boundary
```

У рушії не потрібно hardcoded `OrganismMembrane`.

---

# Organism Identity

Organism identity — це технічна або аналітична мітка connected component.

Вона потрібна для:

* debug UI;
* lineage analysis;
* measuring survival;
* grouping cells;
* tracking structures;
* visualizing multicellularity.

Organism identity не повинна бути входом для поведінки клітин.

Клітина не повинна приймати рішення через:

```text id="eoj36t"
my_organism_id
```

Клітина реагує лише на локальні сигнали, Materials, Joint і state.

---

# Organism як Graph

Organism Graph може мати властивості:

```text id="lh8vht"
cell_count
joint_count
connectedness
average_degree
max_degree
resource_flow_paths
signal_flow_paths
mechanical_stability
boundary_cells
internal_cells
specialization_clusters
damage_propagation_paths
```

Ці властивості корисні для аналізу, але не повинні напряму керувати клітинною поведінкою.

---

# Організм без глобального контролера

Organism не має central controller за замовчуванням.

Глобальна поведінка виникає з локальних взаємодій.

```text id="wgs9sb"
Local cell rules
+
Joint graph
+
Communication
+
Selection
    ↓
Organism-like behavior
```

Навіть якщо з часом виникне neural-like control system, вона має бути emergent network клітин і Joint, а не вбудований контролер.

---

# Organism і Genome

Genome окремої клітини не повинен містити повний blueprint організму.

Genome задає локальні регуляторні правила.

```text id="zt9eyl"
Genome:
  local rules for sensing, material synthesis, signaling, joint formation, division

Not Genome:
  full body map
  predefined organs
  global layout
```

Organism формується через development:

```text id="ug95g6"
Local rules
+
growth
+
division
+
joint formation
+
signals
+
epigenetic states
+
selection
    ↓
organism structure
```

---

# Shared Genome

На ранніх етапах organism-like структура може складатися з клітин із однаковим або дуже схожим Genome.

Це може виникати через поділ однієї клітини.

```text id="xg4b5r"
Founder Cell
    ↓
Division
    ↓
Cells with related Genome
    ↓
Joint-connected structure
```

Shared Genome полегшує координацію, але не є обов'язковим для всіх multicellular structures.

Колонії можуть включати генетично різні клітини.

---

# Genetic Relatedness

Клітини в organism-like structure можуть бути:

* клональними;
* близькими lineage;
* частково спорідненими;
* мозаїчними після HGT;
* змішаними;
* симбіотичними.

Рушій не повинен вимагати `same_genome`.

Organism-like structure визначається функціональною інтеграцією, а не species_id.

---

# Organism і Inheritance

Organism може розмножуватися не лише через поділ однієї клітини, а й через передачу частини багатоклітинної структури.

Можливі сценарії:

```text id="qw87mr"
single founder cell reproduction
fragmentation
bud-like growth
spore-like cell
specialized reproductive cell
cluster split
fusion-like reproduction
```

У базовій моделі достатньо, щоб organism-like structure виникав з поділу клітин і Joint.

Складні репродуктивні стратегії можна описати пізніше.

---

# Organism і Heredity

Спадковою може бути не готова форма organism, а здатність lineage будувати певні структури.

```text id="pu73f6"
Inherited:
  regulatory rules
  joint formation tendency
  signal response
  material synthesis
  development bias

Not directly inherited:
  exact organism shape
  exact organ layout
  fixed body plan
```

Organism shape може повторюватися, якщо локальні правила стабільно створюють схожий development path.

---

# Organism і Development

Development — це процес формування organism-like structure з клітин.

Він може включати:

* growth;
* division;
* joint creation;
* asymmetric inheritance;
* epigenetic differentiation;
* resource redistribution;
* communication;
* specialization;
* death of unstable cells;
* repair of structure.

```text id="y4awsk"
Founder Cell
    ↓
Divisions
    ↓
Joint-connected cluster
    ↓
Signals and gradients
    ↓
Specialization
    ↓
Organism-like structure
```

Development не потребує blueprint.

---

# Organism і Specialization

Specialization є ключовою ознакою organism-like structure.

Клітини можуть ставати:

* boundary-supporting;
* transport-like;
* signal-processing-like;
* storage-like;
* repair-focused;
* energy-production-like;
* mechanical-support-like;
* reproduction-supporting.

Ці ролі не є hardcoded класами.

Вони виникають з Materials, Joint context, communication і epigenetics.

---

# Tissue-like Structures

Tissue-like structure — це група клітин зі схожим станом і спільною локальною функцією.

```text id="mmhcq5"
Tissue-like pattern =
similar cell states
+
stable Joints
+
shared signals
+
shared mechanical/resource role
```

Приклади tissue-like patterns:

* boundary layer;
* signal-conducting chain;
* storage cluster;
* transport layer;
* mechanical support sheet;
* repair zone.

У рушії не існує hardcoded `Tissue`.

Tissue-like pattern — це emergent кластер або аналітична мітка.

---

# Organ-like Structures

Organ-like structure — це функціональний підграф organism.

```text id="c6tcj4"
Organ-like pattern =
specialized cells
+
stable Joint subgraph
+
resource/signal/force function
+
benefit for larger structure
```

Приклади organ-like patterns:

* signal-processing cluster;
* resource transport path;
* energy production region;
* boundary support shell;
* movement-supporting region;
* reproduction-supporting cluster.

У рушії не існує hardcoded `Organ`.

Organ-like structure не має fixed blueprint.

---

# Organism і Resource Flow

Resource Flow — один із головних механізмів organism integration.

Через Joint клітини можуть:

* передавати Resources;
* розподіляти продукти реакцій;
* підтримувати пошкоджені клітини;
* живити спеціалізовані клітини;
* виводити waste;
* підтримувати growth.

```text id="qbo0dr"
Resource-rich Cell
    ↓
Transport-like Cells
    ↓
Resource-poor Cell
```

Resource Flow не є кровоносною системою за замовчуванням.

Кровоносно-подібні структури можуть виникнути як спеціалізований transport graph.

---

# Organism і Energy

Energy Buffer не передається напряму між клітинами.

Організм може мати енергетичну кооперацію через:

* transfer of Resources with `energy_value`;
* transfer of reaction products;
* Heat transfer;
* signals that regulate Energy production;
* specialized energy-production-like cells;
* storage-like cells;
* shared Resource distribution.

```text id="8u7wpa"
No:
  Cell A Energy Buffer -> Cell B Energy Buffer

Yes:
  Cell A Resource -> Joint -> Cell B -> Energy production
```

Це зберігає локальну природу Energy.

---

# Organism і Communication

Communication дозволяє organism-like structure координуватися.

Канали:

* Joint signals;
* Resource gradients;
* Material traces;
* Heat;
* Pressure;
* contact;
* mechanical force.

```text id="w20fk3"
Cell detects stress
    ↓
Signal through Joint
    ↓
Neighbor cells change priorities
```

Communication не є command system.

Приймаюча клітина реагує через власний Genome Runtime.

---

# Organism і Neural-like Networks

Neural-like network може виникнути всередині organism, якщо є:

* signal-sensitive cells;
* signal-conductive Joint;
* stateful Materials;
* impulse accumulation;
* threshold activation;
* adaptive gain;
* signal chains;
* Energy support.

```text id="vnqhlf"
Signal-producing Cell
    ↓
Joint
Signal-processing Cell
    ↓
Joint
Response Cell
```

Це не hardcoded nervous system.

Це emergent signal-processing subgraph.

---

# Organism і Mechanical Structure

Organism може мати механічну форму.

Форма виникає через:

* Joint;
* cell shape;
* material strength;
* elasticity;
* pressure;
* movement;
* collision;
* growth;
* death;
* repair.

```text id="u7rwde"
Cells + Joints + Materials + Physics
    ↓
Body-like shape
```

Body-like shape не задається класом `Body`.

---

# Organism і Movement

Organism movement може виникнути, якщо локальні клітинні рухи координуються через Joint і communication.

```text id="fzrq1t"
Cell A contracts / moves
    ↓
Joint transmits force
    ↓
Cell B displacement
    ↓
Whole structure moves
```

У рушії не повинно бути hardcoded `walk`, `swim`, `crawl`.

Movement patterns мають виникати з клітинних процесів, Joint physics і selection.

---

# Organism і Boundary Cells

Зовнішні клітини organism можуть мати особливий стан через локальні умови:

* більше контакту із середовищем;
* більше Pressure;
* більше damage;
* інший Resource access;
* інший Field exposure;
* інші сигнали.

Вони можуть ставати boundary-supporting.

Це може створити skin-like або shell-like pattern.

Це не hardcoded skin.

---

# Organism і Internal Cells

Внутрішні клітини можуть мати інші умови:

* менше contact із середовищем;
* більше Joint signals;
* інший Resource flow;
* менше Field exposure;
* інший Heat balance;
* більше dependency.

Вони можуть ставати transport-like, storage-like, signal-processing-like або repair-focused.

---

# Organism і Dependency

Dependency — ключова ознака organism.

Клітина стає organism-dependent, якщо її survival залежить від інших клітин.

Приклади:

```text id="xmyqjz"
Cell cannot produce enough Energy-related Resources alone.
Cell receives critical Resource through Joint.
Cell lost independent Boundary repair.
Cell depends on signals to maintain state.
Cell cannot reproduce alone.
```

Чим більше dependency, тим більше структура схожа на organism, а не colony.

---

# Dependency Metrics

Для аналізу можна вимірювати:

```text id="kfrxlp"
resource_dependency
signal_dependency
mechanical_dependency
repair_dependency
energy_resource_dependency
reproduction_dependency
survival_without_joint_probability
```

Ці метрики не повинні керувати клітиною.

Вони потрібні для оцінки рівня інтеграції organism.

---

# Organism Viability

Organism viability — це здатність connected structure підтримувати себе як ціле.

Вона може залежати від:

* survival окремих клітин;
* repair of Joints;
* resource flow;
* communication;
* boundary stability;
* internal specialization;
* reproduction ability;
* resistance to damage;
* heat balance;
* mechanical stability.

Organism viability не замінює cell viability.

Клітини все ще живуть або гинуть індивідуально.

---

# Organism Damage

Organism damage може бути локальним або системним.

Локальний damage:

* одна клітина пошкоджена;
* один Joint розірваний;
* локальний Heat spike;
* local Resource depletion.

Системний damage:

* розрив resource path;
* signal network collapse;
* boundary layer failure;
* mechanical collapse;
* cascade of cell deaths;
* loss of reproductive cells.

Organism damage не є HP.

Це зміна структури клітин, Joint, Materials і flows.

---

# Cascading Failure

У organism-like structure смерть однієї клітини може впливати на інші.

```text id="u25fig"
Cell A dies
    ↓
Resource flow stops
    ↓
Cell B starves
    ↓
Joint network weakens
    ↓
Cell C loses signal support
```

Це виникає з dependency.

Рушій не повинен створювати окремий “organism HP”.

---

# Organism Repair

Organism repair виникає через локальні дії клітин.

Можливі repair patterns:

* клітини ремонтують власні Materials;
* клітини ремонтують Joint;
* сусідні клітини передають Resources;
* boundary cells закривають пошкодження;
* signal cells запускають stress response;
* cells divide to replace lost structure.

```text id="a1unur"
Damage
    ↓
Local signals
    ↓
Cell-level repair processes
    ↓
Structure stabilizes
```

Немає глобальної команди repair.

---

# Organism Growth

Organism growth виникає через division і Joint formation.

```text id="z9o10d"
Cell division
+
Joint creation
+
Resource flow
+
specialization
    ↓
larger structure
```

Growth може бути:

* uncontrolled;
* regulated by local signals;
* boundary-limited;
* resource-limited;
* mechanically constrained;
* development-like.

Genome не містить повної карти росту.

---

# Organism Reproduction

Organism-like reproduction може мати різні форми.

Можливі варіанти:

```text id="bcop95"
single founder cell leaves structure
cluster fragmentation
bud-like growth
spore-like reproductive cell
specialized reproductive cells
fusion-like reproduction
```

Для базової моделі достатньо cell-level division.

Organism-level reproduction можна вважати emergent, якщо від структури відокремлюється життєздатний fragment або founder cell.

---

# Fragmentation

Fragmentation — це коли частина organism-like structure відокремлюється.

```text id="gd13nk"
Organism-like structure
    ↓
Joint break / growth / movement
    ↓
Detached cluster
    ↓
New independent structure
```

Fragmentation може бути:

* випадковою;
* шкідливою;
* корисною;
* репродуктивною;
* наслідком damage.

Не треба hardcode “розмноження фрагментацією”.

Це може виникати з physics і Joint.

---

# Bud-like Growth

Bud-like pattern може виникнути, якщо локальна зона структури активно росте й поступово відокремлюється.

```text id="ekyt2i"
Local growth zone
    ↓
new connected cluster
    ↓
partial separation
    ↓
independent offspring structure
```

Це не hardcoded budding.

Це один із можливих emergent reproduction patterns.

---

# Reproductive Specialization

У складних organism-like structures можуть виникати клітини, які підтримують reproduction.

Вони можуть мати:

* high division readiness;
* Genome copying support;
* fragment export;
* lower specialization;
* high heredity stability;
* protective neighbors;
* resource support.

Це не hardcoded male/female.

Репродуктивні ролі виникають через regulation і selection.

---

# Organism і Death

Organism death — це не один event.

Organism-like structure може вважатися dead або collapsed, якщо:

* більшість клітин мертві;
* connected component розпався;
* resource flow зупинився;
* repair неможливий;
* critical specialized cells втрачені;
* reproduction неможливе;
* boundary зруйнована;
* structure no longer maintains itself.

Але окремі клітини можуть ще жити.

```text id="fw009a"
Organism collapse
    ≠
instant death of all cells
```

---

# Organism Remains

Після collapse organism-like structure залишає:

* dead cells;
* live isolated cells;
* Joint fragments;
* Resources;
* Materials;
* genetic fragments;
* Heat;
* environmental traces.

Це стає частиною екосистеми.

Нічого не повинно зникати миттєво без фізичного процесу.

---

# Organism і Selection

Selection діє не на organism як абстрактний об'єкт, а через survival і reproduction lineage.

Але якщо organism-like structure допомагає клітинам lineage виживати й розмножуватися, така структура може поширюватися.

```text id="x1xd2n"
Cell rules
    ↓
Organism-like structure
    ↓
Better survival / reproduction
    ↓
Lineage expands
```

Selection може підтримувати:

* better Joint formation;
* useful specialization;
* stronger communication;
* resource sharing;
* repair coordination;
* reproductive fragmentation;
* genome stability;
* organism-level robustness.

---

# Organism Fitness

Не треба вводити окремий `organism_fitness_score`, який читають клітини.

Для аналізу можна оцінювати:

```text id="cyryhs"
structure lifetime
offspring structures produced
cell survival rate
resource efficiency
repair success
fragmentation success
lineage persistence
```

Це аналітичні метрики.

Вони не повинні бути входом для Genome Runtime.

---

# Symbiosis-like Structures

Organism-like structure може включати генетично різні клітини.

Якщо вони стабільно взаємодіють і залежать одна від одної, може виникнути symbiosis-like structure.

```text id="lam3s4"
Lineage A cells
+
Lineage B cells
+
stable Joint / Resource exchange
    ↓
symbiosis-like structure
```

Це не hardcoded symbiosis.

Це emergent relationship.

---

# Chimeric Organism-like Structures

Якщо структура складається з клітин різного походження, її можна аналізувати як chimeric.

Причини:

* HGT;
* cell fusion;
* cluster merging;
* two colonies joining;
* environmental aggregation.

Chimeric structure може бути нестабільною або дуже успішною.

Рушій не повинен забороняти її через species_id.

---

# Organism і Species

Species-like groups можуть виникати як стабільні lineage з обмеженим genetic exchange.

Organism не повинен мати species_id, який керує поведінкою.

Клітини можуть розрізняти “своїх” і “чужих” лише через:

* Materials;
* Signals;
* Boundary compatibility;
* Joint compatibility;
* genetic fragments;
* local traces;
* history of interaction.

---

# Organism і Environment

Organism-like structure змінює середовище.

Вона може:

* споживати Resources;
* створювати gradients;
* залишати Material traces;
* виробляти Heat;
* змінювати mechanical landscape;
* накопичувати dead remains;
* створювати niches для інших клітин.

Organism є частиною world dynamics, а не ізольованим об'єктом.

---

# Organism і Ecosystem

Коли organism-like structures взаємодіють між собою, виникає ecosystem-level dynamics.

Можливі взаємодії:

* competition for Resources;
* contact;
* merging;
* predation-like behavior;
* parasitism-like fragments;
* symbiosis-like exchange;
* environmental modification;
* decomposition.

У рушії не треба hardcode predator/prey.

Такі ролі мають виникати з Materials, movement, resource flows, damage і selection.

---

# Organism Lifecycle

Organism lifecycle може включати:

```text id="vzr0nl"
emergence
growth
specialization
maintenance
reproduction
damage
repair
fragmentation
collapse
decomposition
```

Це не замінює cell lifecycle.

Organism lifecycle — це аналітичний рівень над cell lifecycle.

---

# Organism States

Для аналізу можна використовувати states:

```text id="03vscr"
forming
stable
growing
stressed
damaged
fragmenting
reproducing
collapsing
decomposing
```

Ці states не повинні напряму керувати клітинами.

Вони потрібні для debug UI і дослідження.

---

# базова модель Organism Model

Для базової моделі organism можна не робити активною engine-сутністю.

Достатньо обчислювати organism-like structures як connected components.

```text id="fa7rfb"
базова модель:
  organism-like structure = connected component of Cells through Joints
```

Додатково можна рахувати метрики:

```text id="3dwhl1"
cell_count
joint_count
average_joint_strength
resource_flow
signal_flow
specialization_diversity
dependency_score
lifetime
fragmentation_events
```

---

# базова модель Organism Detection

Просте правило:

```text id="w0ujdt"
If cells are connected by active Joints,
they belong to the same connected component.
```

Потім класифікація:

```text id="hudlcs"
1 cell:
  single cell

2-5 weakly connected cells:
  cluster

many weakly specialized cells:
  colony-like

cells with high dependency and specialization:
  organism-like
```

Це лише аналітичні labels.

Клітини не повинні читати ці labels.

---

# Organism Dependency Score

Для аналітики можна ввести dependency score.

Приклад компонентів:

```text id="xs4xmr"
resource_dependency
signal_dependency
joint_dependency
specialization_dependency
reproduction_dependency
survival_without_structure
```

Простий сенс:

```text id="snv46f"
low dependency -> colony-like
high dependency -> organism-like
```

Це не канонічна формула, а напрямок для future metrics.

---

# Organism Trace

Для дебагу корисно зберігати Organism Trace.

Приклад:

```text id="ooxw6x"
Tick 1200
Component 17

Cells: 42
Joints: 61
Average Joint Strength: 0.73
Resource Flow: active
Signal Flow: active
Specialization Diversity: medium
Dependency Score: 0.62

Event:
  fragmentation created Component 18
```

Trace не повинен керувати поведінкою.

Він потрібен для аналізу emergence.

---

# Приклад 1. Простий cluster

```text id="j75mkz"
Three cells are connected by weak adhesive Joints.

They share little Resource.
Signals are weak.
Each cell can survive alone.

Result:
  cluster / simple colony
```

Це ще не сильний organism.

---

# Приклад 2. Colony-like structure

```text id="w7uycc"
Cells remain connected for many Tick.
Resources pass slowly through Joints.
Stress signals spread locally.
Cells still mostly survive independently.

Result:
  colony-like structure
```

---

# Приклад 3. Organism-like dependency

```text id="j6ou7l"
Outer cells protect boundary.
Inner cells process Resources.
Signal cells coordinate stress response.
Transport-like cells move Resources.

If transport cells die:
  inner cells starve.

Result:
  organism-like dependency
```

---

# Приклад 4. Tissue-like boundary layer

```text id="jjtyzo"
Outer cells receive pressure and damage.
They synthesize stronger Boundary Materials.
They maintain strong Joints.
They repair often.

Result:
  boundary-supporting tissue-like layer
```

Це не hardcoded skin.

---

# Приклад 5. Organ-like transport path

```text id="vnv78a"
Resource-rich cells connect to resource-poor cells through a chain.

Middle cells:
  high transport Materials
  high Joint permeability
  low own consumption

Result:
  transport-like organ pattern
```

Це не hardcoded vascular system.

---

# Приклад 6. Organism movement

```text id="k4jr2r"
Cells on one side contract.
Joint network transmits force.
Opposite side resists deformation.
Whole structure shifts position.

Result:
  movement-like organism behavior
```

Це не hardcoded walking.

---

# Приклад 7. Fragmentation reproduction

```text id="h69484"
Structure grows.
A side cluster becomes semi-independent.
Joints between main body and cluster weaken.
Cluster detaches and survives.

Result:
  reproduction-like fragmentation
```

Це може стати organism-level reproductive strategy через selection.

---

# Приклад 8. Collapse after hub death

```text id="32m7yo"
One central transport-like cell dies.
Resource flow breaks.
Several dependent cells lose Energy Resources.
Joint repair stops.
Component splits.

Result:
  organism-like collapse
```

Це не organism HP loss.

Це структурний cascade.

---

# Правила

## Rule 1. Organism is emergent

Organism виникає з клітин, Joint, communication, specialization і selection.

## Rule 2. Organism is not a controller

Organism не керує клітинами напряму.

## Rule 3. Cells remain primary agents

Усі процеси виконуються клітинами.

## Rule 4. Organism is a cell-joint graph

Базова структура organism — граф Cells + Joints.

## Rule 5. Colony and organism form a continuum

Межа між colony і organism поступова.

## Rule 6. Tissues are emergent patterns

Tissue-like structures не є hardcoded класами.

## Rule 7. Organs are emergent functional subgraphs

Organ-like structures не є hardcoded класами.

## Rule 8. Dependency indicates integration

Чим більша взаємозалежність клітин, тим більше структура схожа на organism.

## Rule 9. No species id

Organism identity і behavior не повинні базуватися на hardcoded species_id.

## Rule 10. Organism death is structural collapse

Organism death — це втрата підтримуваної структури, а не один HP event.

---

# Заборонено

Не вводити:

* hardcoded organism class that controls cells;
* organism HP;
* global organism bus;
* hardcoded body plan;
* hardcoded organs;
* hardcoded tissues;
* hardcoded predator/prey;
* hardcoded plant/animal organism types;
* species_id-based behavior;
* direct Energy Buffer sharing across organism;
* central brain by default;
* global role assignment.

---

# Пов'язані документи

* `biology/cell.md`
* `biology/joint.md`
* `biology/communication.md`
* `biology/specialization.md`
* `biology/development.md`
* `biology/lifecycle.md`
* `biology/processes.md`
* `biology/membrane.md`
* `genetics/genome-runtime.md`
* `genetics/heredity.md`
* `genetics/epigenetics.md`
* `world/materials.md`
* `world/resources.md`
* `world/energy.md`
* `world/physics.md`
* `world/tick.md`
* `engine/ecs.md`
* `engine/scheduler.md`

# Open Questions

## Organism detection

Потрібно визначити, чи organism detection У базовій моделі буде лише connected component, чи з dependency score.

## Colony vs organism threshold

Потрібно вирішити, чи потрібен поріг між colony-like і organism-like structures.

## Dependency metrics

Потрібно визначити мінімальні dependency metrics:

* resource dependency;
* signal dependency;
* mechanical dependency;
* reproduction dependency.

## Tissue detection

Потрібно вирішити, чи tissue-like clusters будуть визначатися через Material profile, Joint density або process profile.

## Organ-like detection

Потрібно визначити, як виявляти organ-like functional subgraphs без hardcoded органів.

## Organism reproduction

Потрібно описати окремо, коли fragmentation або bud-like growth вважати organism-level reproduction.

## Organism lineage

Потрібно визначити, як відстежувати lineage organism-like structures, якщо вони ростуть, діляться, зливаються або розпадаються.

## Mixed-genome structures

Потрібно вирішити, як аналізувати organism-like structures із клітин різного походження.

## Organism trace

Потрібно визначити мінімальний Organism Trace для debug UI і досліджень.

## Organism observer model

Потрібно визначити мінімальні дані observer-side `OrganismView` для debug UI, lineage tracking і research metrics.


