# inheritance.md

> **Inheritance — передача спадкового і стартового стану дочірнім клітинам**

---

# Призначення

Цей документ описує `Inheritance` — механізм передачі стану від батьківської клітини до дочірньої клітини.

Inheritance включає не лише Genome.

Під час поділу або іншого репродуктивного процесу дочірня клітина може отримати:

* Genome або Genome fragments;
* Resources;
* Materials;
* Energy Buffer;
* Boundary materials;
* Epigenetic State;
* Runtime State;
* Heat;
* damage;
* internal fragments;
* lineage metadata.

Inheritance не є гарантією життєздатності.

Дочірня клітина може отримати неповний, пошкоджений або погано збалансований стартовий стан.

---

# Основна ідея

Нова клітина не виникає з нічого.

Вона отримує частину фізичного і регуляторного стану від батьківської клітини або кількох клітин.

```text
Parent Cell
    ↓
Inheritance
    ↓
Daughter Cell State
```

Успадкування має бути матеріальним.

Якщо дочірня клітина отримала Genome, але не отримала достатньо Boundary materials або Energy, вона може бути нежиттєздатною.

---

# Що Inheritance НЕ є

Inheritance не є:

* магічним копіюванням ідеальної клітини;
* гарантією виживання;
* точним дублюванням батьківської клітини;
* передачею learned behavior як зміни Genome;
* автоматичним repair;
* способом обійти Resources, Materials або Energy;
* hardcoded species reproduction;
* завжди рівним розподілом.

Inheritance — це розподіл фізичного і регуляторного стану.

---

# Inheritance vs Mutation

Inheritance передає Genome або його копію.

Mutation змінює Genome.

```text
Inheritance:
  parent Genome -> daughter Genome

Mutation:
  copied Genome -> changed copied Genome
```

Під час inheritance може виникнути mutation, але це різні поняття.

---

# Inheritance vs Learning

Learning-like state може змінювати стан клітини протягом життя.

Це не означає, що Genome змінився.

Дочірня клітина може частково отримати стан матеріалів або epigenetic state, але вона не успадковує learning як мутацію Genome.

```text
Learning-like state:
  changed material coefficient

Inheritance:
  part of that material state may be transferred

Mutation:
  Genome structure changes
```

---

# Inheritance vs Epigenetics

Epigenetic State може частково передаватися дочірнім клітинам.

Але це не є зміною Genome.

```text
Genome:
  inherited regulatory structure

Epigenetic State:
  inherited or partially inherited runtime modifier
```

Epigenetic inheritance може впливати на стартовий стан дочірньої клітини, але має бути відокремлене від Genome inheritance.

---

# Що може успадковуватися

Під час поділу або репродукції можуть передаватися:

```text
Genome
Genome fragments
Resources
Materials
Energy Buffer
Boundary materials
Epigenetic State
Runtime State
Damage
Heat
Internal fragments
Lineage metadata
```

Не все повинно передаватися однаково.

Різні частини стану можуть мати різні правила розподілу.

---

# Genome Inheritance

Genome Inheritance — це передача Genome або його копії дочірній клітині.

Можливі варіанти:

* точна копія;
* копія з mutation;
* неповна копія;
* пошкоджена копія;
* дубльована частина Genome;
* втрата фрагмента;
* нерівномірний розподіл Genome fragments.

Для MVP можна почати з одного Genome на клітину.

```text
Parent Genome
    ↓
Copy
    ↓
Mutation
    ↓
Daughter Genome
```

---

# Genome Copying

Копіювання Genome потребує:

* Resources;
* Energy;
* часу;
* free_capacity;
* відповідних Materials або процесів;
* регуляторного сигналу.

Genome не повинен копіюватися безкоштовно.

Більший Genome має більшу вартість копіювання.

---

# Genome Copying Errors

Під час копіювання можуть виникати помилки:

* edge weight shift;
* node deletion;
* node duplication;
* binding change;
* fragment loss;
* fragment duplication;
* damaged fragment;
* invalid connection.

Не всі помилки роблять Genome технічно invalid.

Більшість просто створюють нову спадкову регуляцію.

---

# Genome Fragments

У майбутній моделі Genome може складатися з фрагментів.

```text
Genome Pool
├── Core Fragment
├── Regulatory Fragment
├── Transport Fragment
├── Energy Fragment
└── Mobile Fragment
```

Під час inheritance фрагменти можуть розподілятися нерівномірно.

Це відкриває шлях до:

* plasmid-like genome;
* partial inheritance;
* fragment loss;
* fragment duplication;
* recombination;
* HGT.

Для MVP фрагменти можна залишити як future-compatible концепцію.

---

# Core Genome

`Core Genome` — умовна частина Genome, без якої клітина майже напевно нежиттєздатна.

Вона може містити регуляцію:

* Boundary maintenance;
* Energy production;
* basic resource uptake;
* Material synthesis;
* Genome copying;
* minimal repair.

Це не hardcoded окремий тип Genome.

Це аналітичне поняття для опису мінімальної життєздатності.

---

# Mobile Genetic Fragments

Mobile fragments можуть передаватися не так стабільно, як Core Genome.

Вони можуть:

* копіюватися частіше;
* втрачатися;
* передаватися через HGT;
* давати додаткові регуляторні можливості;
* бути паразитичними;
* збільшувати Genome cost.

У MVP це можна відкласти.

---

# Resource Inheritance

Дочірня клітина отримує частину внутрішніх Resources батьківської клітини.

Ресурси можуть розподілятися:

* рівномірно;
* пропорційно до об'єму;
* випадково;
* за локальною позицією;
* за регуляторним сигналом;
* за фізичною структурою.

Для MVP можна використати простий пропорційний розподіл із невеликим шумом.

```text
Parent resources:
  Resource A = 100

Division:
  Daughter A = 52
  Daughter B = 48
```

---

# Resource Inheritance не гарантує користь

Дочірня клітина може отримати:

* корисні Resources;
* waste Resources;
* reactive Resources;
* забагато об'єму;
* занадто мало потрібного ресурсу.

Це може вплинути на життєздатність.

Inheritance не повинен фільтрувати Resources за корисністю.

---

# Material Inheritance

Materials також розподіляються між дочірніми клітинами.

Особливо важливі:

* Boundary materials;
* Energy-conversion materials;
* synthesis-capable materials;
* repair-capable materials;
* structural materials;
* signal-sensitive materials;
* stateful materials;
* Genome-protection materials.

Якщо дочірня клітина не отримала критичних Materials, вона може швидко деградувати.

---

# Boundary Material Inheritance

Boundary materials критичні для виживання нової клітини.

Під час поділу кожна дочірня клітина повинна отримати або швидко створити власну Boundary.

Можливі проблеми:

* одна дочірня клітина отримала недостатньо Boundary;
* Boundary не закрилася;
* Boundary пошкоджена;
* Boundary занадто проникна;
* Boundary не має потрібної міцності.

Це може призвести до failed division або нежиттєздатної дочірньої клітини.

---

# Energy Buffer Inheritance

Energy Buffer може розподілятися між дочірніми клітинами.

Energy не є речовиною, але під час поділу локальний стан батьківської клітини розділяється між дочірніми клітинами.

```text
Parent Energy Buffer = 10

Daughter A = 5.5
Daughter B = 4.5
```

Energy capacity дочірньої клітини визначається її Materials.

Якщо дочірня клітина отримала більше Energy, ніж дозволяє її capacity, надлишок повинен перейти в Heat або втратитися за правилами `world/energy.md`.

---

# Energy Capacity After Inheritance

Energy capacity не копіюється як окреме число.

Вона обчислюється з Materials дочірньої клітини.

```text
energy_capacity =
Σ(material_amount × material.energy_capacity)
```

Тому можлива ситуація:

```text
Daughter A receives many storage materials -> high capacity
Daughter B receives few storage materials -> low capacity
```

Це впливає на стартову життєздатність.

---

# Epigenetic Inheritance

Epigenetic State може частково передаватися дочірнім клітинам.

Він може впливати на:

* стартову спеціалізацію;
* dormancy;
* stress response;
* repair priority;
* division readiness;
* Material synthesis bias;
* sensitivity to signals;
* development path.

Epigenetic State не є Genome.

Він може деградувати, скидатися або поступово змінюватися після поділу.

---

# Epigenetic Reset

Під час поділу epigenetic_state може:

* повністю копіюватися;
* частково копіюватися;
* скидатися;
* розподілятися асиметрично;
* змінюватися через stress поділу.

Для MVP можна використати просте правило:

```text
daughter_epigenetic_state =
parent_epigenetic_state × inheritance_factor
+ small_noise
```

де `inheritance_factor` може бути менше `1.0`.

---

# Runtime State Inheritance

Runtime State — це поточні значення виконання регуляторної мережі або learning-like стану.

Приклади:

* accumulated_signal;
* previous_activation;
* delayed_signal;
* material coefficient state;
* impulse memory;
* signal gain state.

Runtime State не є Genome.

Він може:

* не передаватися;
* передаватися частково;
* скидатися;
* розподілятися за Materials;
* залежати від epigenetic_state.

Для MVP краще більшість Runtime State скидати або передавати мінімально.

---

# Learning-like State Inheritance

Learning-like state не повинен напряму перетворюватися на Genome mutation.

Але якщо learning-like state фізично збережений у Materials, частина цього стану може потрапити в дочірню клітину разом із Materials.

Приклад:

```text
Stateful Material coefficient = 0.8

Division:
  Daughter A receives material with coefficient 0.7
  Daughter B receives material with coefficient 0.2
```

Це не означає, що дочірня клітина успадкувала learned rule як Genome.

Вона успадкувала фізичний стан матеріалу.

---

# Damage Inheritance

Пошкодження може передаватися дочірнім клітинам.

Дочірня клітина може отримати:

* пошкоджений Boundary material;
* damaged Genome;
* degraded Materials;
* waste Resources;
* high Heat;
* unstable internal state.

Інколи поділ може зменшити damage для однієї дочірньої клітини, але збільшити для іншої.

Це створює можливість асиметричного поділу.

---

# Heat Inheritance

Heat може розподілятися між дочірніми клітинами.

Якщо одна дочірня клітина отримала більше Heat або менше heat-resistant Materials, вона може мати гірший старт.

Heat не є Energy Buffer.

Він є фізичним станом, що впливає на Materials і процеси.

---

# Joint Context Inheritance

Якщо батьківська клітина була частиною організму або колонії, під час поділу треба визначити, що відбувається з Joint.

Можливі варіанти:

* старі Joint залишаються з однією дочірньою клітиною;
* Joint розриваються;
* Joint перебудовуються;
* нові Joint створюються між дочірніми клітинами;
* дочірня клітина народжується без Joint;
* частина Joint materials розподіляється.

Для MVP можна зробити просте правило:

```text
existing Joints remain attached to the daughter cell closest to their original position
```

Але це залежить від фізичної моделі.

---

# Symmetric Inheritance

Symmetric Inheritance — це приблизно рівний розподіл стану між дочірніми клітинами.

```text
Parent
  ↓
Daughter A ≈ Daughter B
```

Переваги:

* стабільність;
* передбачуваність;
* простота;
* менше нежиттєздатних дочірніх клітин.

Недоліки:

* менше різноманіття;
* гірша спеціалізація;
* менше можливостей для розвитку складних організмів.

---

# Asymmetric Inheritance

Asymmetric Inheritance — це нерівномірний розподіл стану.

```text
Parent
  ↓
Daughter A ≠ Daughter B
```

Відрізнятися можуть:

* Resources;
* Materials;
* Genome fragments;
* Epigenetic State;
* Runtime State;
* damage;
* Heat;
* Joint context.

Це може створювати:

* спеціалізацію;
* різні ролі дочірніх клітин;
* ризик нежиттєздатності;
* швидшу адаптацію;
* розвиток організмів.

---

# Controlled Asymmetry

Genome може регулювати процеси, які впливають на асиметрію поділу.

Але він не повинен магічно розкладати все ідеально.

Controlled asymmetry повинна працювати через:

* Materials;
* internal organization;
* division process;
* Boundary formation;
* resource transport;
* epigenetic state.

---

# Random Asymmetry

Навіть без спеціальної регуляції поділ може бути трохи асиметричним через:

* шум;
* локальний розподіл Materials;
* фізику;
* damage;
* Heat;
* позицію ресурсів;
* помилки копіювання.

Це нормальне джерело варіативності.

---

# Multi-parent Inheritance

У майбутніх моделях дочірня клітина може отримувати генетичний матеріал від кількох джерел.

Приклади:

* recombination;
* fusion-like reproduction;
* gamete-like fragments;
* HGT before division;
* plasmid-like exchange.

У такому випадку inheritance може включати:

```text
Parent A Genome fragments
+
Parent B Genome fragments
+
Cellular material from one or both parents
```

Це буде деталізовано в `genetics/recombination.md`.

---

# Inheritance and Recombination

Recombination — це змішування генетичного матеріалу.

Inheritance — це передача результату цього змішування дочірній клітині.

```text
Parent A Genome
+
Parent B Genome
    ↓
Recombination
    ↓
Recombined Genome
    ↓
Inheritance
    ↓
Daughter Cell
```

Не кожне inheritance має recombination.

Але результат recombination завжди має бути успадкований якоюсь клітиною, щоб мати еволюційний ефект.

---

# Inheritance and HGT

Horizontal Gene Transfer може змінити Genome клітини не через батьківський поділ.

Якщо після HGT клітина ділиться, інтегрований або збережений genetic fragment може бути переданий дочірнім клітинам.

```text
HGT fragment enters cell
    ↓
fragment persists or integrates
    ↓
cell divides
    ↓
fragment inherited by daughter cell
```

Таким чином HGT може стати спадковим лише якщо фрагмент зберігся до репродукції.

---

# Inheritance and Selection

Selection не контролює inheritance напряму.

Inheritance створює дочірні клітини з певним стартовим станом.

Якщо цей стан дозволяє їм:

* вижити;
* підтримувати структуру;
* виробляти Energy;
* рости;
* ділитися;
* передавати Genome,

то lineage продовжується.

Якщо ні — зникає.

---

# Inheritance не гарантує рівність

Рушій не повинен автоматично вирівнювати дочірні клітини до “нормального” стану.

Поганий розподіл дозволений.

Приклади:

* одна клітина отримала Genome, інша ні;
* одна клітина отримала Boundary, інша ні;
* одна клітина отримала Energy, інша майже ні;
* одна клітина отримала damage;
* одна клітина отримала waste;
* одна клітина отримала корисний fragment.

Це може створювати selection pressure.

---

# Minimal Inheritance for MVP

Для MVP достатньо моделі:

```text
1 parent cell -> 2 daughter cells
```

Передаються:

```text
Genome copy
Resources
Materials
Energy Buffer
Epigenetic State
Damage
Heat
```

Мінімальні правила:

```text
Genome:
  copied to both daughters with mutation chance

Resources:
  split approximately 50/50 with noise

Materials:
  split approximately 50/50 with noise

Energy:
  split by daughter capacity or proportionally

Epigenetic State:
  partially inherited or reset

Damage:
  distributed with Materials

Heat:
  distributed proportionally

Joints:
  dropped or assigned by simple rule
```

---

# MVP Inheritance Algorithm

Приклад простого алгоритму:

```text
1. Check division readiness.
2. Create two empty daughter cell states.
3. Copy Genome for each daughter.
4. Apply mutation during copying.
5. Split Resources between daughters.
6. Split Materials between daughters.
7. Recompute Energy capacity for each daughter.
8. Split Energy Buffer within capacity limits.
9. Split or reset Epigenetic State.
10. Distribute damage with damaged Materials.
11. Distribute Heat.
12. Assign or drop Joints.
13. Validate technical cell state.
14. Add daughters to world.
15. Mark parent as divided/removed.
```

Validation не повинна робити дочірні клітини життєздатними штучно.

Вона лише повинна гарантувати, що структура даних не ламає рушій.

---

# Technical Validation

Після inheritance потрібно перевірити:

* cell id assigned;
* Genome data technically valid;
* Resources non-negative;
* Materials non-negative;
* Energy not above capacity;
* capacity not negative;
* Boundary state computable;
* lifecycle_state assigned;
* position valid;
* no broken references;
* Joint references valid.

Technical validation не означає, що клітина жива або здорова.

---

# Viability Check

Після inheritance дочірня клітина може пройти viability check.

Перевіряються:

* Boundary integrity;
* critical Materials;
* Genome functionality;
* Energy production possibility;
* free_capacity;
* maintenance possibility.

Якщо viability низька, клітина може стартувати як:

```text
stressed
damaged
dormant
dead
```

Але рушій не повинен автоматично “ремонтувати” її.

---

# Lineage Metadata

Для аналізу бажано зберігати lineage metadata.

Приклади:

```text
cell_id
parent_cell_id
generation
birth_tick
parent_genome_id
genome_id
mutation_events
inheritance_mode
division_event_id
```

Ці дані не повинні напряму керувати поведінкою клітини.

Вони потрібні для дослідження еволюції.

---

# Inheritance Trace

Для дебагу й експериментів корисно зберігати Inheritance Trace.

Приклад:

```text
Tick 500
Parent Cell: 120
Daughter Cells: 301, 302

Genome:
  copied with 2 mutations

Resources:
  Daughter 301: 54%
  Daughter 302: 46%

Materials:
  Daughter 301: 49%
  Daughter 302: 51%

Energy:
  Daughter 301: 5.2
  Daughter 302: 4.8
```

Trace не треба зберігати завжди для всіх клітин, але він важливий для аналізу.

---

# Приклад 1. Нормальний симетричний поділ

```text
Parent Cell:
  Genome complete
  Boundary stable
  Energy = 10
  Resource A = 100
  Material X = 80

Inheritance:
  Daughter A receives:
    Genome copy
    Energy = 5
    Resource A = 51
    Material X = 39

  Daughter B receives:
    Genome copy
    Energy = 5
    Resource A = 49
    Material X = 41

Result:
  both daughters alive
```

---

# Приклад 2. Мутація під час inheritance

```text
Parent Genome:
  edge_12 weight = 0.40

Copying:
  Daughter A receives unchanged copy
  Daughter B receives mutation:
    edge_12 weight = 0.48

Result:
  Daughter B has slightly different regulation
```

Мутація не була цілеспрямованою.

Її наслідки визначить selection.

---

# Приклад 3. Нерівномірний розподіл Boundary

```text
Parent Cell:
  Boundary Material = 100

Division:
  Daughter A receives 75
  Daughter B receives 25

Result:
  Daughter A stable
  Daughter B boundary_integrity low
  Daughter B starts stressed
```

Якщо Daughter B не зможе швидко синтезувати або відремонтувати Boundary, вона помре.

---

# Приклад 4. Успадкування пошкодження

```text
Parent Cell:
  damaged Material X = 40
  intact Material X = 60

Division:
  Daughter A receives mostly intact Material
  Daughter B receives mostly damaged Material

Result:
  Daughter A alive
  Daughter B damaged
```

Це дозволяє асиметричне очищення або, навпаки, створення слабкої дочірньої клітини.

---

# Приклад 5. Learning-like state частково передається

```text
Parent Cell:
  stateful signal Material has gain = 0.8

Division:
  Daughter A receives this Material with gain = 0.7
  Daughter B receives little of this Material, gain = 0.1

Result:
  Daughter A has stronger initial signal response
  Daughter B almost resets signal response
```

Це не Genome mutation.

Це успадкування фізичного стану матеріалу.

---

# Приклад 6. Дочірня клітина без Genome

```text
Division error:
  Daughter A receives Genome copy
  Daughter B receives no functional Genome

Result:
  Daughter A may survive
  Daughter B cannot regulate active processes
  Daughter B may die after passive degradation
```

Рушій не повинен забороняти це як біологічний результат.

Але структура даних повинна залишатися валідною.

---

# Приклад 7. HGT fragment став спадковим

```text
Before division:
  Cell absorbed mobile genetic fragment through HGT.
  Fragment persisted in Genome Pool.

Division:
  Daughter A receives fragment.
  Daughter B does not.

Result:
  Daughter A may express new regulation.
  Daughter B remains unchanged.
```

Фрагмент стає еволюційно значущим лише якщо потрапляє в lineage.

---

# Приклад 8. Energy overflow після поділу

```text
Parent Cell:
  Energy = 20

Daughter A:
  Energy capacity = 8

Daughter B:
  Energy capacity = 6

Inheritance:
  attempted Energy split = 10 / 10

Correction:
  Daughter A stores 8
  Daughter B stores 6
  excess 6 becomes Heat or is lost by Energy rules
```

Energy capacity визначається Materials, а не бажаним розподілом.

---

# Правила

## Rule 1. Inheritance transfers state

Inheritance передає фізичний і регуляторний стан дочірнім клітинам.

## Rule 2. Genome is inherited, not magically recreated

Genome або його копія повинні фізично перейти в дочірню клітину.

## Rule 3. Mutation may occur during inheritance

Під час копіювання Genome можуть виникати мутації.

## Rule 4. Materials and Resources are inherited physically

Resources і Materials розподіляються як частини клітинної матерії.

## Rule 5. Energy is limited by daughter capacity

Дочірня клітина не може зберігати Energy більше, ніж дозволяє її Energy capacity.

## Rule 6. Epigenetic State may be partially inherited

Epigenetic State може передаватися частково, скидатися або розподілятися асиметрично.

## Rule 7. Learning-like state is not Genome inheritance

Learning-like state може передаватися лише як фізичний стан Materials або Runtime State, а не як зміна Genome.

## Rule 8. Inheritance can be asymmetric

Дочірні клітини не зобов'язані бути однаковими.

## Rule 9. Non-viable offspring are allowed

Рушій не повинен гарантувати життєздатність дочірніх клітин.

## Rule 10. Technical validity is required

Inheritance не повинна створювати технічно зламані об'єкти рушія.

---

# Заборонено

Не вводити:

* guaranteed perfect copy;
* guaranteed equal division;
* automatic viable offspring;
* learning as Genome rewrite;
* Energy inheritance above capacity;
* free Resources or Materials during inheritance;
* species-specific inheritance rules;
* plant/animal-specific reproduction rules;
* engine-level repair of bad inheritance;
* hidden fitness-based correction.

---

# Пов'язані документи

* `biology/cell.md`
* `biology/lifecycle.md`
* `biology/processes.md`
* `biology/genome.md`
* `biology/membrane.md`
* `genetics/regulatory-network.md`
* `genetics/genome-runtime.md`
* `genetics/mutation.md`
* `genetics/recombination.md`
* `genetics/horizontal-transfer.md`
* `genetics/epigenetics.md`
* `world/resources.md`
* `world/materials.md`
* `world/energy.md`
* `world/physics.md`

---

# ADR

Потрібні ADR:

```text
ADR-000X: Inheritance Transfers Full Cell Start State
ADR-000X: Inheritance Does Not Guarantee Viability
ADR-000X: Learning-like State Is Not Genome Inheritance
ADR-000X: Asymmetric Inheritance Is Allowed
```

---

# Open Questions

## MVP inheritance balance

Потрібно визначити стартові коефіцієнти розподілу:

```text
resource_split_noise
material_split_noise
energy_split_rule
damage_split_rule
epigenetic_inheritance_factor
```

## Genome fragments

Потрібно вирішити, чи Genome fragments існують у MVP, чи лише в future model.

## Epigenetic inheritance

Потрібно визначити, які саме epigenetic fields можуть передаватися дочірнім клітинам.

## Runtime state inheritance

Потрібно вирішити, чи accumulated_signal, impulse memory та material coefficients передаються в MVP.

## Joint inheritance

Потрібно визначити, як поділ клітини впливає на існуючі Joint.

## Non-viable daughter handling

Потрібно визначити, чи нежиттєздатна дочірня клітина:

* народжується як `stressed`;
* народжується як `damaged`;
* народжується як `dead`;
* створюється як material fragment.

## Multi-parent inheritance

Потрібно описати окремо після `recombination.md`.

## Inheritance trace

Потрібно визначити мінімальний формат trace для експериментів і дебагу.
