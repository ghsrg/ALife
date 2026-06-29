# recombination.md

> **Recombination — змішування спадкових регуляторних структур**

---

# Призначення

Цей документ описує `Recombination` — процес змішування генетичного матеріалу з двох або більше джерел.

Recombination може поєднувати:

* Genome;
* Genome fragments;
* regulatory subgraphs;
* mobile genetic fragments;
* inherited modules;
* пошкоджені або неповні генетичні структури.

Recombination не є mutation, хоча може створювати mutation-like наслідки.

Recombination не є гарантовано корисною.

Recombination створює нові комбінації спадкової регуляції.

---

# Основна ідея

Genome у моделі може бути представлений як Regulatory Network.

Recombination змішує частини таких мереж.

```text
Parent Genome A
+
Parent Genome B
    ↓
Recombination
    ↓
Recombined Genome
    ↓
Inheritance
    ↓
Daughter Cell
```

Recombination не повинна вимагати, щоб два Genome мали однакову структуру.

Графи можуть бути різними.

Фрагменти можуть не збігатися.

Результат може бути:

* життєздатним;
* слабким;
* нейтральним;
* пошкодженим;
* технічно valid, але біологічно нежиттєздатним;
* технічно invalid, якщо оператор зламав структуру даних.

---

# Що Recombination НЕ є

Recombination не є:

* точним crossover як у класичних генетичних алгоритмах;
* гарантованим покращенням;
* магічним вирівнюванням двох Genome;
* пошуком “правильних” вузлів;
* оптимізацією fitness;
* repair;
* learning;
* epigenetic change;
* обов'язковим етапом кожного поділу.

Recombination лише змішує спадковий матеріал.

---

# Recombination vs Mutation

Mutation змінює Genome випадково.

Recombination змішує вже існуючі спадкові структури.

```text
Mutation:
  Genome A -> Genome A'

Recombination:
  Genome A + Genome B -> Genome C
```

Але recombination може створити ефекти, схожі на mutation:

* втрату фрагмента;
* дублювання;
* зміну контексту Edge;
* появу нового шляху;
* злам регуляції;
* invalid binding;
* несподіване підсилення сигналу.

---

# Recombination vs Inheritance

Recombination створює новий або змінений спадковий матеріал.

Inheritance передає цей матеріал дочірній клітині.

```text
Recombination:
  mix genetic material

Inheritance:
  assign result to daughter cell
```

Не кожне inheritance включає recombination.

Але recombined Genome стає еволюційно значущим лише якщо він успадковується нащадками.

---

# Recombination vs HGT

HGT передає генетичний матеріал між не-батьківськими клітинами.

Recombination може бути механізмом інтеграції цього матеріалу.

```text
HGT:
  foreign fragment enters cell

Recombination:
  fragment integrates or mixes with existing Genome
```

Тобто HGT — це шлях передачі.

Recombination — це спосіб змішування або інтеграції.

---

# Проблема graph crossover

Якщо Genome — це граф, класичний crossover складний.

У двох графах може не бути однакових вузлів.

```text
Genome A:
  energy -> node_1 -> repair

Genome B:
  heat -> node_9 -> dormancy
```

Немає очевидної відповіді, які вузли “відповідають” один одному.

Тому не треба вимагати ідеального зіставлення.

Recombination повинна працювати з фрагментами, підграфами або вставками.

---

# Принцип rough recombination

Модель повинна дозволяти грубе змішування.

```text
insert fragment
replace fragment
merge fragment
delete fragment
duplicate fragment
partial overwrite
broken merge
```

Більшість таких операцій можуть бути шкідливими.

Це нормально.

Selection відфільтрує невдалі варіанти.

---

# Genetic Fragment

Для recombination бажано мати поняття `Genetic Fragment`.

Fragment — це частина Genome або Regulatory Network.

Фрагмент може містити:

```text
nodes
edges
input_bindings
output_bindings
fragment_parameters
mutation_rate
copy_stability
integration_rules
```

Фрагмент не обов'язково є повністю самодостатнім.

Він може мати сенс лише в контексті іншого Genome.

---

# Fragment Boundary

Fragment Boundary визначає, де фрагмент можна відокремити або вставити.

Можливі варіанти:

* випадковий підграф;
* connected subgraph;
* regulatory module;
* output-centered fragment;
* input-output path;
* mobile fragment;
* plasmid-like fragment;
* damaged fragment.

Для базової моделі можна використовувати простий random subgraph або predefined fragment container.

---

# Recombination Operators

Базові оператори recombination:

```text
fragment_insert
fragment_replace
fragment_merge
fragment_delete
fragment_duplicate
fragment_split
partial_overwrite
broken_merge
```

Не всі потрібні у базовій моделі.

---

# fragment_insert

`fragment_insert` додає фрагмент з одного Genome до іншого.

```text
Genome A
+
Fragment from Genome B
    ↓
Genome A + Fragment B
```

Можливі наслідки:

* нова здатність;
* зайвий Genome cost;
* конфлікт outputs;
* неактивний silent fragment;
* invalid connections;
* нова регуляторна поведінка.

---

# fragment_replace

`fragment_replace` замінює частину Genome A фрагментом Genome B.

```text
Genome A fragment X
    ↓ replaced by
Genome B fragment Y
```

Це може:

* змінити існуючу функцію;
* зламати critical process;
* покращити регуляцію;
* створити непередбачуваний зв'язок;
* видалити важливий pathway.

---

# fragment_merge

`fragment_merge` об'єднує два фрагменти.

```text
Fragment A
+
Fragment B
    ↓
Merged Fragment
```

Можливі проблеми:

* duplicate OutputNodes;
* conflicting edges;
* цикли;
* надмірне підсилення сигналу;
* invalid bindings;
* завеликий Genome cost.

---

# fragment_delete

`fragment_delete` видаляє фрагмент під час recombination.

Це може бути наслідком:

* неповного змішування;
* помилки копіювання;
* пошкодження;
* конфлікту фрагментів;
* випадкового втрачання.

Fragment deletion може бути:

* корисним, якщо видалено шкідливий fragment;
* нейтральним;
* смертельним, якщо видалено critical regulation.

---

# fragment_duplicate

`fragment_duplicate` дублює фрагмент.

Дубльований fragment може:

* підсилити існуючу регуляцію;
* збільшити cost;
* створити redundancy;
* з часом мутувати в нову функцію;
* зламати баланс outputs.

Дублювання є важливим джерелом еволюційної складності.

---

# fragment_split

`fragment_split` розділяє один fragment на кілька частин.

Це може створити:

* mobile fragments;
* incomplete fragments;
* partial regulatory modules;
* silent fragments;
* future HGT-compatible pieces.

Split не повинен гарантувати збереження функції.

---

# partial_overwrite

`partial_overwrite` замінює частину параметрів або Edge/Node фрагмента.

Приклад:

```text
Fragment A keeps nodes
but receives edge weights from Fragment B
```

Це може бути корисно, якщо два фрагменти мають схожу структуру.

Але не треба вимагати повного вирівнювання.

---

# broken_merge

`broken_merge` — це невдале або грубе об'єднання.

Воно може створити:

* missing edges;
* duplicate nodes;
* dangling bindings;
* disabled outputs;
* inactive fragments;
* invalid graph;
* non-viable Genome.

Broken merge дозволений як біологічний результат, але технічно invalid graph не повинен ламати рушій.

---

# Homology-like Matching

У майбутньому можна додати приблизне зіставлення фрагментів.

Наприклад, фрагменти можуть вважатися схожими, якщо мають:

* схожі output bindings;
* схожі input bindings;
* спільне походження;
* близькі fragment tags;
* схожу topology;
* спільні inherited ids.

Це можна назвати `homology-like matching`.

Але базова модель не повинна залежати від складного matching.

---

# Heritable IDs

Щоб полегшити recombination, вузли або фрагменти можуть мати спадкові `origin_id`.

```text
node_id       = technical runtime id
origin_id     = inherited ancestry id
fragment_id   = technical fragment id
origin_lineage = ancestry marker
```

`origin_id` може допомогти знайти схожі частини графа.

Але він не повинен бути species_id і не повинен керувати поведінкою клітини.

---

# Fragment Tags

Fragment може мати tags, які допомагають інтеграції.

Приклади:

```text
energy_related
boundary_related
signal_related
transport_related
mobile
core_like
```

Але треба бути обережним.

Tags не повинні перетворитися на hardcoded органи або готові функції.

Для базової моделі краще або не використовувати tags, або використовувати їх лише для технічного групування, а не поведінки.

---

# Integration

Після recombination результат має бути інтегрований у Genome або Genome Pool.

Можливі стани:

```text
integrated
temporary
silent
disabled
rejected
degraded
damaged
```

Не кожен fragment повинен одразу впливати на Runtime.

---

# Temporary Fragment

Фрагмент може тимчасово існувати всередині клітини без повної інтеграції.

Він може:

* деградувати;
* копіюватися;
* бути переданим дочірній клітині;
* активуватися пізніше;
* інтегруватися під час repair/recombination;
* залишатися silent.

Це важливо для plasmid-like і HGT моделей.

---

# Silent Fragment

Silent Fragment технічно присутній, але не впливає на Runtime.

Причини:

* немає output bindings;
* input недоступний;
* fragment disabled;
* немає потрібних Materials;
* fragment не інтегрований;
* outputs не проходять Feasibility Check.

Silent fragments можуть стати активними після майбутніх мутацій або змін середовища.

---

# Conflict Resolution

Після recombination можуть виникнути конфлікти.

Приклади:

* два outputs керують одним процесом;
* один fragment активує repair, інший пригнічує repair;
* дублікати node ids;
* цикли;
* edge посилається на відсутній node;
* output binding веде до невідомого process;
* input binding веде до недоступного signal.

Потрібно відрізняти:

```text
technical conflict
```

від:

```text
biological conflict
```

Technical conflict не повинен ламати рушій.

Biological conflict може залишитися й бути відфільтрований selection.

---

# Technical Validation

Після recombination Genome має пройти technical validation.

Перевіряється:

* унікальність technical ids;
* edges посилаються на існуючі nodes;
* output bindings відомі рушію;
* input bindings відомі рушію;
* кількість nodes у межах ліміту;
* кількість edges у межах ліміту;
* activation functions валідні;
* graph execution mode можливий;
* fragments мають валідну структуру.

Technical validation не повинна “покращувати” Genome.

Вона лише захищає рушій від зламаних структур даних.

---

# Biological Viability

Genome може бути технічно valid, але біологічно нежиттєздатним.

Наприклад:

```text
Genome has no Boundary repair output.
Genome always activates division.
Genome cannot produce Energy.
Genome ignores damage.
Genome runtime cost is too high.
```

Такі Genome не треба забороняти.

Вони просто не виживуть.

---

# Recombination Cost

Recombination не повинна бути безкоштовною.

Вона може потребувати:

* Energy;
* Resources;
* Genome-copying Materials;
* часу;
* physical contact;
* compatible Boundary;
* Joint;
* fusion-like state;
* genetic fragment stability.

У базовій моделі recombination можна не реалізовувати фізично, але принцип вартості треба зберегти.

---

# Recombination Timing

Recombination може відбуватися:

* перед division;
* під час Genome copying;
* під час fusion-like reproduction;
* після HGT integration;
* під час Genome repair;
* під час пошкодження Genome;
* у спеціальному reproductive state.

Для базової моделі recombination можна залишити як future model після стабілізації mutation та inheritance.

---

# One-parent Recombination

Recombination не обов'язково вимагає двох батьків.

Можлива внутрішня перебудова одного Genome:

```text
Genome fragment A
+
Genome fragment B
from same cell
    ↓
rearranged Genome
```

Це може включати:

* duplication;
* inversion;
* fragment merge;
* fragment split;
* partial overwrite.

Така recombination близька до structural mutation, але працює з більшими фрагментами.

---

# Two-parent Recombination

Two-parent recombination змішує genetic material від двох джерел.

```text
Genome A
+
Genome B
    ↓
Recombined Genome
```

Джерела можуть бути:

* дві клітини;
* два Genome Pools;
* батьківська клітина і HGT fragment;
* фрагменти з мертвої клітини;
* gamete-like fragments.

---

# Multi-parent Recombination

У майбутньому можливе змішування кількох джерел.

```text
Fragment A
+
Fragment B
+
Fragment C
    ↓
Genome Pool
```

Це може бути корисно для:

* HGT-rich середовища;
* plasmid-like genomes;
* microbial-like evolution;
* modular adaptation.

базова модель не потребує multi-parent recombination.

---

# Recombination and Species-like Isolation

У рушії немає hardcoded species_id.

Але lineage може еволюційно створити механізми genome isolation.

Genome може регулювати:

* openness to external fragments;
* recombination probability;
* fragment rejection;
* Boundary compatibility;
* HGT uptake;
* fusion readiness.

Так можуть виникати species-like barriers без hardcoded виду.

---

# Recombination і Selection

Selection не контролює recombination напряму.

Recombination створює нові комбінації.

Selection діє через наслідки:

```text
Recombined Genome
    ↓
Cell Regulation
    ↓
Survival / Reproduction
    ↓
Lineage success or extinction
```

Більшість recombination events можуть бути невдалими.

Це нормально.

---

# Recommendation базової моделі

Для базової моделі recombination можна не реалізовувати повністю.

Рекомендований підхід:

```text
базова модель:
  mutation + inheritance

у майбутньому:
  fragment_insert
  fragment_replace
  fragment_duplicate

Later:
  homology-like matching
  multi-parent recombination
  plasmid-like genome pools
```

Але архітектура Genome вже зараз не повинна блокувати fragments.

---

# Minimal Future Recombination Model

Перший простий варіант recombination:

```text
1. Select recipient Genome.
2. Select donor Fragment.
3. Choose operator:
   - insert
   - replace
   - duplicate
4. Apply operator.
5. Rename technical ids if needed.
6. Validate technical structure.
7. Mark biological viability as unknown.
8. Let runtime and selection decide.
```

---

# Приклад 1. Insert fragment

```text
Genome A:
  energy_level -> produce_energy

Genome B Fragment:
  heat_level -> enter_dormancy

Recombination:
  insert Fragment B into Genome A

Result:
  New Genome can both produce Energy and enter dormancy under Heat.
```

Це може бути корисним, якщо Heat справді небезпечний.

---

# Приклад 2. Replace fragment

```text
Genome A Fragment:
  damage_level -> repair_boundary

Genome B Fragment:
  damage_level -> movement

Recombination:
  replace A fragment with B fragment

Result:
  damaged cell now tries to move instead of repair.
```

Це може бути шкідливо.

Рушій не повинен це виправляти.

---

# Приклад 3. Duplicate fragment

```text
Original Fragment:
  resource_A_inside -> produce_energy

Recombination:
  duplicate fragment

Result:
  produce_energy signal may become stronger.
```

Можливі наслідки:

* краща Energy production;
* зайві витрати;
* перегрів;
* надмірне споживання Resource A.

---

# Приклад 4. Broken merge

```text
Fragment A:
  node_1 -> repair_boundary

Fragment B:
  energy_level -> node_9

Broken merge:
  energy_level -> missing_node
  node_1 duplicated
  repair output disconnected

Result:
  Genome technically invalid or non-functional.
```

Якщо технічно invalid — рушій повинен безпечно обробити структуру.

Якщо технічно valid, але non-functional — selection відфільтрує.

---

# Приклад 5. Silent inserted fragment

```text
Inserted Fragment:
  field_light -> synthesize_material_L

Cell:
  no light-sensitive Material
  no Resource for Material L

Runtime:
  fragment has no visible effect

Result:
  silent fragment persists.
```

Пізніше інші mutation можуть зробити fragment активним.

---

# Приклад 6. Recombination після HGT

```text
Cell absorbs genetic fragment from dead cell.

Fragment:
  resource_B_inside -> export_resource_B

Existing Genome:
  resource_B accumulation causes clogging

Recombination:
  fragment integrates

Result:
  cell can export Resource B
  lineage may survive better in this environment.
```

---

# Приклад 7. Genome isolation

```text
Lineage A:
  high openness to external fragments

Lineage B:
  low openness to external fragments

Environment:
  many harmful fragments

Result:
  Lineage B may survive better.
```

В іншому середовищі, де багато корисних fragments, Lineage A може мати перевагу.

---

# Правила

## Rule 1. Recombination mixes hereditary material

Recombination поєднує або перебудовує спадкові структури.

## Rule 2. Recombination is not guaranteed beneficial

Результат recombination може бути корисним, нейтральним, шкідливим або смертельним.

## Rule 3. No perfect graph alignment required

Модель не повинна вимагати ідеального зіставлення двох Regulatory Networks.

## Rule 4. Fragments are preferred units

Для graph-based Genome recombination краще працювати з fragments або subgraphs.

## Rule 5. Technical validity is required

Recombined Genome не повинен ламати рушій як структура даних.

## Rule 6. Biological viability is not guaranteed

Технічно valid recombined Genome може бути нежиттєздатним.

## Rule 7. Selection filters outcomes

Selection діє через survival і reproduction, а не через оцінку recombination.

## Rule 8. Recombination has cost

Recombination повинна мати фізичну, енергетичну або процесну вартість.

## Rule 9. Recombination must support HGT

Модель повинна дозволяти інтеграцію зовнішніх genetic fragments.

## Rule 10. No species id

Сумісність recombination не повинна базуватися на hardcoded species_id.

---

# Заборонено

Не вводити:

* perfect crossover requirement;
* automatic useful recombination;
* fitness-guided recombination;
* magic graph alignment;
* species_id compatibility;
* guaranteed viable offspring;
* hardcoded male/female reproduction;
* plant/animal-specific recombination rules;
* automatic repair of bad recombination;
* direct learning transfer as recombination.

---

# Пов'язані документи

* `genetics/regulatory-network.md`
* `genetics/genome-runtime.md`
* `genetics/mutation.md`
* `genetics/inheritance.md`
* `genetics/heredity.md`
* `genetics/horizontal-transfer.md`
* `genetics/epigenetics.md`
* `biology/genome.md`
* `biology/cell.md`
* `biology/lifecycle.md`
* `world/materials.md`
* `world/energy.md`
* `world/laws.md`

# Open Questions

## базова модель inclusion

Потрібно вирішити, чи recombination входить у базову модель, чи залишається у майбутньому.

## Fragment representation

Потрібно визначити, як саме представляти fragments:

* explicit fragment containers;
* random subgraphs;
* output-centered modules;
* mobile genetic objects.

## Homology-like matching

Потрібно вирішити, чи потрібні inherited origin ids для approximate matching.

## Technical validation

Потрібно визначити, що робити з technically invalid recombined Genome:

* reject operator;
* disable invalid fragment;
* mark Genome damaged;
* create non-viable daughter;
* fallback to parent Genome.

## Recombination cost

Потрібно визначити, які Resources, Materials або Energy потрібні для recombination.

## Compatibility

Потрібно визначити, чи compatibility виникає через Materials, Boundary, genome isolation або fragment properties.

## Silent fragments

Потрібно вирішити, чи silent fragments можуть зберігатися у базовій моделі.

## Recombination trace

Потрібно визначити мінімальний trace для аналізу recombination events.


