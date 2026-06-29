# horizontal-transfer.md

> **Horizontal Transfer — горизонтальний перенос генетичного матеріалу**

---

# Призначення

Цей документ описує `Horizontal Transfer` або `Horizontal Gene Transfer` — передачу генетичного матеріалу між клітинами без прямого зв'язку “батько → нащадок”.

Horizontal Transfer дозволяє клітині отримати genetic fragment з іншого джерела:

* живої клітини;
* мертвої клітини;
* середовища;
* Joint;
* контакту;
* fragment carrier;
* майбутньої virus-like структури.

Horizontal Transfer не є inheritance.

Horizontal Transfer не є mutation.

Horizontal Transfer не є recombination, але може завершуватися recombination або integration.

---

# Основна ідея

Генетичний матеріал є фізичною сутністю.

Він може існувати:

* всередині клітини;
* як частина Genome;
* як mobile fragment;
* як пошкоджений fragment;
* у середовищі після смерті клітини;
* у контакті між клітинами;
* у Joint;
* у майбутній carrier-структурі.

```text
Donor Cell / Environment
    ↓
Genetic Fragment
    ↓
Recipient Cell
    ↓
Persistence / Integration / Degradation
```

Horizontal Transfer створює додатковий шлях еволюції, окрім мутацій і вертикальної спадковості.

---

# Що Horizontal Transfer НЕ є

Horizontal Transfer не є:

* магічним копіюванням Genome;
* гарантією корисного fragment;
* автоматичною інтеграцією;
* species-based compatibility;
* прямим покращенням Genome;
* learning;
* epigenetic change;
* mutation;
* обов'язковим процесом кожної клітини;
* вірусною системою у базовій моделі.

Horizontal Transfer лише переміщує генетичний матеріал між не-батьківськими клітинами або через середовище.

---

# Horizontal Transfer vs Inheritance

`Inheritance` — передача стану від батьківської клітини до дочірньої клітини.

```text
Parent Cell
    ↓
Daughter Cell
```

`Horizontal Transfer` — передача genetic fragment між клітинами без прямого поділу.

```text
Cell A
    ↓
Genetic Fragment
    ↓
Cell B
```

HGT стає еволюційно значущим лише тоді, коли отриманий fragment зберігається і потім передається через inheritance.

---

# Horizontal Transfer vs Mutation

Mutation змінює Genome випадково.

Horizontal Transfer додає або переносить вже існуючий genetic fragment.

```text
Mutation:
  Genome A -> Genome A'

Horizontal Transfer:
  Genome A + external Fragment B -> Genome A + Fragment B
```

Під час HGT можуть виникати mutation-like наслідки:

* пошкодження fragment;
* неповна інтеграція;
* broken merge;
* fragment degradation;
* зміна bindings;
* дублювання;
* invalid graph.

Але сам HGT — це передача, а не мутація.

---

# Horizontal Transfer vs Recombination

Horizontal Transfer — це шлях потрапляння genetic fragment до клітини.

Recombination — це спосіб змішати або інтегрувати цей fragment з існуючим Genome.

```text
HGT:
  external fragment enters cell

Recombination:
  fragment integrates into Genome
```

Fragment може потрапити в клітину, але не recombine.

Він може:

* залишитися temporary;
* бути silent;
* деградувати;
* бути переданим дочірній клітині;
* активуватися пізніше;
* пошкодити клітину.

---

# Genetic Fragment

Об'єктом Horizontal Transfer є `Genetic Fragment`.

Fragment може бути:

* частиною Regulatory Network;
* набором nodes;
* набором edges;
* input/output bindings;
* mobile fragment;
* damaged genome piece;
* plasmid-like element;
* future virus-like payload.

Fragment не обов'язково є повним Genome.

Він може бути неповним, шкідливим, silent або корисним лише у певному контексті.

---

# Physical Carrier

Genetic Fragment повинен мати фізичний носій.

Він:

* займає об'єм;
* має stability;
* може деградувати;
* може бути пошкоджений Heat;
* може вступати в реакції;
* може бути поглинутий;
* може бути виведений;
* може бути переданий дочірнім клітинам.

Genome не повинен існувати як абстрактна інформація поза світом.

---

# Sources of Genetic Fragments

Джерела genetic fragments:

```text
living cell
dead cell
decomposing cell
environment
Joint
cell contact
released fragment
mobile fragment
future virus-like carrier
```

Для базової моделі можна почати з простого джерела:

```text
dead cell decomposition -> genetic fragments in environment
```

---

# Dead Cell Source

Після смерті клітини Genome не зникає миттєво.

Він може:

* деградувати;
* розпастися на fragments;
* залишитися в середовищі;
* бути поглинутим іншою клітиною;
* втратити частину структури;
* стати джерелом HGT.

```text
Dead Cell
    ↓
Genome Fragments
    ↓
Environment
    ↓
Recipient Cell
```

Це дозволяє мертвим клітинам бути частиною еволюційного циклу.

---

# Living Cell Source

Жива клітина може потенційно передавати genetic fragments.

Механізми:

* direct contact;
* Joint;
* controlled export;
* damaged Boundary leakage;
* mobile genetic fragment release;
* future carrier.

У базовій моделі це можна відкласти.

---

# Environmental Fragment

Genetic Fragment у середовищі є фізичним об'єктом або локальною концентрацією.

Він має:

```text
position
stability
degradation_rate
size
genetic_content
damage_level
uptake_difficulty
```

У спрощеній базовій моделі fragment може бути абстрактним локальним object.

---

# Fragment Stability

Fragment не повинен бути вічним.

Його stability залежить від:

* Heat;
* Radiation;
* reactive Resources;
* environment;
* protective Materials;
* time;
* physical damage.

```text
fragment_stability_next =
fragment_stability_current
- degradation
```

Якщо stability падає нижче мінімуму, fragment деградує в Resources або inert remains.

---

# Fragment Damage

Пошкоджений fragment може:

* втратити nodes;
* втратити edges;
* втратити bindings;
* змінити parameters;
* стати silent;
* стати harmful;
* стати technically invalid;
* деградувати повністю.

Пошкодження fragment не повинно автоматично виправлятися.

---

# Fragment Uptake

Клітина може поглинути external genetic fragment лише за певних умов.

Потрібні:

* контакт із fragment;
* Boundary permeability або спеціальні Materials;
* free_capacity;
* Energy, якщо uptake активний;
* регуляторний сигнал, якщо uptake контрольований;
* достатня stability fragment;
* відсутність повного rejection.

```text
External Genetic Fragment
    ↓
Boundary / Uptake Process
    ↓
Internal Genetic Fragment
```

---

# Passive Uptake

Passive Uptake може відбутися без спеціального контролю, якщо Boundary пошкоджена або проникна.

Наслідки:

* fragment може потрапити всередину випадково;
* fragment може бути шкідливим;
* fragment може засмітити клітину;
* fragment може деградувати;
* fragment може інтегруватися пізніше.

Passive Uptake не потребує Genome-рішення, але залежить від фізики Boundary.

---

# Active Uptake

Active Uptake потребує:

* Energy;
* uptake-capable Material;
* регуляторного сигналу;
* fragment detection;
* free_capacity;
* Boundary transport process.

Genome може регулювати openness до HGT.

Але Genome не повинен знати, чи fragment буде корисним.

---

# HGT Openness

Клітина може мати різний рівень відкритості до зовнішнього genetic material.

```text
hgt_uptake_level
genome_isolation_level
fragment_rejection_level
integration_probability
```

Висока openness може давати:

* швидшу адаптацію;
* нові можливості;
* ризик шкідливих fragments;
* ризик паразитичних fragments;
* genome instability.

Низька openness може давати:

* стабільність;
* захист;
* повільнішу адаптацію;
* втрату корисних можливостей.

Selection визначає, яка стратегія вигідна.

---

# Fragment Rejection

Клітина може еволюційно мати механізми rejection.

Fragment може бути:

* не поглинутий;
* виведений;
* деградований;
* ізольований;
* залишений silent;
* не інтегрований.

Rejection не повинен працювати через `species_id`.

Він повинен виникати через:

* Boundary materials;
* genome isolation level;
* fragment properties;
* internal reactions;
* regulatory outputs;
* material compatibility.

---

# Internal Fragment States

Після потрапляння в клітину fragment може мати стани:

```text
external
internal_free
temporary
silent
integrating
integrated
rejected
degraded
damaged
```

Для базової моделі достатньо:

```text
external
internal_free
integrated
degraded
```

---

# Temporary Fragment

Temporary Fragment знаходиться всередині клітини, але ще не є частиною active Genome.

Він може:

* деградувати;
* бути інтегрований;
* бути виведений;
* бути переданий дочірній клітині;
* залишатися silent;
* впливати на cost;
* бути пошкодженим.

---

# Integration

Integration — це включення fragment у Genome або Genome Pool.

Integration може відбуватися через:

* recombination;
* fragment insert;
* fragment replace;
* fragment merge;
* plasmid-like persistence;
* temporary activation;
* damaged merge.

```text
Internal Fragment
    ↓
Integration
    ↓
Genome Pool / Regulatory Network
```

Integration не повинна гарантувати корисність.

---

# Plasmid-like Persistence

Fragment може не інтегруватися в Core Genome, а існувати як окремий plasmid-like елемент.

Він може:

* мати власну copy rate;
* мати власну stability;
* впливати на Runtime;
* бути переданим дочірнім клітинам;
* втрачатися;
* дублюватися;
* бути паразитичним;
* бути корисним.

Це future-compatible модель.

Для базової моделі достатньо залишити це як можливість.

---

# Silent Fragment

Fragment може бути silent.

Причини:

* немає active output binding;
* input недоступний;
* немає потрібних Materials;
* fragment не інтегрований;
* fragment пошкоджений;
* output не проходить Feasibility Check.

Silent fragments можуть накопичуватись і створювати future variation.

Але вони мають cost.

---

# Harmful Fragment

Fragment може бути шкідливим.

Приклади:

* активує division занадто рано;
* пригнічує repair;
* збільшує mutation_rate;
* споживає Energy;
* збільшує Genome runtime cost;
* створює конфлікт outputs;
* блокує Energy production;
* руйнує Boundary regulation.

Рушій не повинен забороняти harmful fragments.

Selection відфільтрує лінії, які їх не витримують.

---

# Parasitic Fragment

Parasitic Fragment — це fragment, який поширюється, але шкодить клітині або використовує її ресурси.

Він може:

* підвищувати власну copy rate;
* збільшувати HGT release;
* не давати корисних outputs;
* збільшувати Genome cost;
* заважати Core Genome;
* передаватися дочірнім клітинам.

Це не треба hardcode як “virus”.

Це може виникнути з fragment properties і selection.

---

# Beneficial Fragment

Beneficial Fragment може надати корисну регуляцію.

Наприклад:

* здатність використовувати новий Resource;
* нову stress response;
* кращу Boundary repair;
* dormancy trigger;
* HGT protection;
* signal processing;
* Energy production pathway.

Beneficial Fragment не знає, що він корисний.

Він просто створює ефект у певному середовищі.

---

# Fragment Cost

Fragment має вартість.

Вартість може включати:

* об'єм;
* copying cost;
* runtime cost;
* repair cost;
* mutation risk;
* Energy cost;
* integration cost;
* instability risk.

Навіть silent fragment не повинен бути повністю безкоштовним.

---

# HGT and Selection

Selection не керує HGT напряму.

HGT створює нові спадкові комбінації.

Selection діє через наслідки:

```text
HGT Event
    ↓
Changed Genome / Genome Pool
    ↓
Changed Cell Regulation
    ↓
Survival / Reproduction
    ↓
Lineage success or extinction
```

---

# HGT and Heredity

HGT має довготривалий еволюційний ефект лише якщо fragment передається нащадкам.

```text
Fragment enters cell
    ↓
Cell survives
    ↓
Cell divides
    ↓
Fragment inherited
```

Якщо fragment не потрапив у lineage, його ефект короткочасний.

---

# HGT and Recombination

HGT часто потребує recombination або integration.

Можливі сценарії:

```text
Fragment enters
    ↓
stays temporary
```

```text
Fragment enters
    ↓
integrates by recombination
```

```text
Fragment enters
    ↓
degrades
```

```text
Fragment enters
    ↓
is inherited as plasmid-like fragment
```

---

# HGT and Mutation

Fragment може мутувати:

* до uptake;
* під час uptake;
* під час integration;
* після integration;
* під час copying;
* під час degradation.

HGT збільшує простір варіацій, але не є directed improvement.

---

# HGT and Genome Isolation

Genome Isolation — це здатність lineage обмежувати HGT.

Genome може регулювати:

* uptake probability;
* rejection probability;
* integration probability;
* fragment degradation;
* Boundary permeability;
* protective Materials;
* HGT export.

Genome Isolation не базується на species_id.

Це еволюційна властивість клітини.

---

# HGT через Joint

Якщо клітини з'єднані Joint, genetic fragment може потенційно передаватися через нього.

Потрібні:

* Joint channel або сумісний Material;
* fragment stability;
* regulatory permission;
* Energy, якщо transfer active;
* compatibility with recipient internal state.

```text
Cell A Genome Fragment
    ↓
Joint
    ↓
Cell B Internal Fragment
```

Для базової моделі це можна відкласти.

---

# HGT через контакт

При прямому контакті клітин fragment може переходити через Boundary.

Можливі умови:

* пошкоджена Boundary;
* adhesive contact;
* shared material bridge;
* active transfer process;
* fragment release by donor;
* uptake by recipient.

Це теж можна відкласти до у майбутньому.

---

# HGT через середовище

Найпростіший сценарій Для базової моделі або early у майбутньому:

```text
Dead Cell
    ↓
External Genetic Fragment
    ↓
Environment
    ↓
Recipient Cell Uptake
```

Цей механізм не потребує складного контакту живих клітин.

Він добре узгоджується з decomposition.

---

# Virus-like Carriers

У майбутньому можуть виникнути virus-like carriers.

Але в рушії не треба hardcode `Virus`.

Virus-like behavior може виникати, якщо fragment або carrier structure:

* копіюється;
* переноситься між клітинами;
* використовує клітинні ресурси;
* має високу transfer probability;
* шкодить або допомагає host;
* передається через середовище.

Це має бути emergent або fragment-based модель, а не окремий клас вірусу.

---

# HGT і Species-like Boundaries

У світі немає hardcoded species.

Але HGT може впливати на виникнення species-like clusters.

Якщо lineage має низький HGT openness, його Genome стає більш ізольованим.

Якщо lineage має високий HGT openness, genetic material може вільніше змішуватися.

```text
High HGT
    ↓
more gene flow

Low HGT
    ↓
more isolation
```

Species-like межі виникають з потоку genetic material, compatibility і selection.

---

# Recommendation базової моделі

Для базової моделі HGT можна не реалізовувати.

Але архітектура повинна не блокувати:

* Genome fragments;
* genetic fragments after death;
* fragment stability;
* fragment uptake;
* future Genome Pool;
* recombination integration.

Рекомендований шлях:

```text
базова модель:
  mutation + inheritance

у майбутньому 1:
  dead cell -> genetic fragments -> uptake -> degradation/integration

у майбутньому 2:
  Joint/contact transfer

у майбутньому 3:
  plasmid-like fragments and virus-like carriers
```

---

# Minimal HGT Model

Перший простий варіант HGT:

```text
1. Dead cell decomposes.
2. Genome breaks into genetic fragments.
3. Fragments appear in local environment.
4. Fragments degrade over time.
5. Living cell may uptake fragment if Boundary allows.
6. Fragment becomes internal_free.
7. Fragment either degrades or integrates.
8. If integrated and inherited, it affects lineage.
```

---

# Technical Validation

Якщо fragment інтегрується в Genome, результат проходить technical validation.

Перевіряється:

* valid nodes;
* valid edges;
* valid bindings;
* known activation functions;
* limits on node/edge count;
* graph execution compatibility;
* no broken references.

Technical validation не повинна робити fragment корисним.

Вона лише захищає рушій.

---

# Biological Viability

HGT-result може бути:

* beneficial;
* neutral;
* silent;
* harmful;
* lethal;
* parasitic;
* unstable.

Усе це допустимо.

Selection визначить, що залишиться.

---

# HGT Trace

Для аналізу бажано зберігати HGT Trace.

Приклад:

```text
Tick 800

Fragment:
  fragment_id = gf_102
  source = dead_cell_55
  recipient = cell_210
  state = integrated

Result:
  new output binding added:
    resource_B_inside -> export_resource_B
```

Trace не повинен керувати поведінкою клітини.

Він потрібен для дебагу й дослідження.

---

# Приклад 1. Fragment після смерті клітини

```text
Cell A dies.

Decomposition:
  Genome breaks into 3 fragments.

Fragment 1:
  degrades quickly.

Fragment 2:
  remains in environment.

Cell B:
  absorbs Fragment 2.

Result:
  Fragment 2 becomes internal_free in Cell B.
```

---

# Приклад 2. Silent HGT

```text
Fragment:
  field_light -> synthesize_material_L

Recipient Cell:
  no light-sensitive Material
  no Resource for Material L

Result:
  Fragment integrates but has no visible effect.
```

Пізніше інші mutation або зміна середовища можуть зробити fragment корисним.

---

# Приклад 3. Harmful HGT

```text
Fragment:
  damage_level -> suppress_repair

Recipient Cell:
  takes fragment through passive uptake.

Runtime:
  repair becomes weaker during damage.

Result:
  Cell dies after Boundary damage.
```

Рушій не повинен забороняти це.

---

# Приклад 4. Beneficial HGT

```text
Environment:
  Resource B accumulates and clogs cells.

Fragment from dead cell:
  resource_B_inside -> export_resource_B

Recipient Cell:
  integrates fragment.

Result:
  Recipient can export Resource B.
  Lineage survives better in this environment.
```

---

# Приклад 5. Parasitic Fragment

```text
Fragment:
  increases its own copy rate
  increases HGT release
  adds no useful cell process

Cell:
  pays copying and runtime cost

Result:
  Fragment spreads but harms host efficiency.
```

Це може бути основою virus-like behavior без hardcoded Virus class.

---

# Приклад 6. Genome Isolation

```text
Lineage A:
  high HGT uptake

Lineage B:
  low HGT uptake

Environment:
  many harmful fragments

Result:
  Lineage B survives better.
```

В іншому середовищі з багатьма корисними fragments Lineage A може мати перевагу.

---

# Приклад 7. HGT став спадковим

```text
Cell absorbs fragment.
Fragment integrates.
Cell survives.
Cell divides.
Daughter receives integrated fragment.

Result:
  HGT-derived trait becomes hereditary.
```

Якщо fragment не передався дочірній клітині, його еволюційний ефект обмежений.

---

# Правила

## Rule 1. HGT transfers genetic material horizontally

Horizontal Transfer передає genetic fragments між не-батьківськими клітинами або через середовище.

## Rule 2. HGT is physical

Genetic fragment має фізичний носій, stability, damage і degradation.

## Rule 3. HGT is not inheritance

Inheritance передає стан від parent до daughter. HGT передає fragment між іншими джерелами.

## Rule 4. HGT is not mutation

HGT переносить існуючий fragment. Mutation змінює Genome.

## Rule 5. Integration is not guaranteed

Fragment може деградувати, залишитися silent, бути rejected або інтегруватися.

## Rule 6. HGT may be harmful

HGT може бути beneficial, neutral, silent, harmful, lethal або parasitic.

## Rule 7. No species id

HGT compatibility не повинна базуватися на hardcoded species_id.

## Rule 8. HGT becomes hereditary only if inherited

Fragment має передатися нащадкам, щоб мати довготривалий еволюційний ефект.

## Rule 9. HGT has cost

Uptake, maintenance, integration і copying fragment повинні мати вартість.

## Rule 10. HGT must support future fragments

Модель повинна бути сумісною з fragment-based Genome, plasmid-like elements і recombination.

---

# Заборонено

Не вводити:

* magic gene transfer;
* automatic useful integration;
* species_id compatibility;
* hardcoded virus class in base model;
* direct learning transfer as HGT;
* HGT as mutation;
* guaranteed inheritance of HGT fragment;
* free fragment copying;
* perfect fragment repair;
* fitness-guided integration.

---

# Пов'язані документи

* `genetics/regulatory-network.md`
* `genetics/genome-runtime.md`
* `genetics/mutation.md`
* `genetics/inheritance.md`
* `genetics/heredity.md`
* `genetics/recombination.md`
* `genetics/epigenetics.md`
* `biology/genome.md`
* `biology/cell.md`
* `biology/lifecycle.md`
* `biology/processes.md`
* `biology/joint.md`
* `world/materials.md`
* `world/resources.md`
* `world/energy.md`
* `world/physics.md`

# Open Questions

## базова модель inclusion

Потрібно вирішити, чи HGT входить у базову модель, чи тільки у майбутньому.

## Fragment object

Потрібно визначити, як представляти external genetic fragment:

* physical object;
* resource-like packet;
* material fragment;
* special genetic fragment entity.

## Fragment degradation

Потрібно визначити формулу degradation:

* за Tick;
* через Heat;
* через Radiation;
* через reactive Resources;
* через environment.

## Uptake mechanism

Потрібно вирішити, як клітина поглинає fragment:

* passive uptake;
* active uptake;
* damaged Boundary;
* Joint/contact only.

## Integration model

Потрібно визначити, що значить `integrated`:

* inserted into Regulatory Network;
* stored in Genome Pool;
* plasmid-like independent fragment;
* temporary runtime influence.

## HGT cost

Потрібно визначити Energy, Resource і Material costs для uptake, storage, integration і copying.

## Rejection model

Потрібно визначити, як працює fragment rejection без species_id.

## HGT trace

Потрібно визначити мінімальний формат HGT Trace для аналізу.


