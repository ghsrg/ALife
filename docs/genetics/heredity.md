# heredity.md

> **Heredity — спадковість як передача еволюційно значущих властивостей**

---

# Призначення

Цей документ описує `Heredity` — принцип спадковості в Artificial Life Engine.

`inheritance.md` описує конкретний механізм передачі стану дочірній клітині під час поділу.

`heredity.md` описує, які властивості є спадковими, як вони зберігаються в lineage і що саме може еволюціонувати.

Heredity — це не один акт копіювання.

Heredity — це здатність властивостей переходити між поколіннями так, щоб selection могла діяти на них.

---

# Основна ідея

Властивість є спадковою, якщо вона може впливати на нащадків через переданий фізичний або регуляторний стан.

```text
Parent Cell
    ↓
Inherited State
    ↓
Daughter Cell
    ↓
Survival / Reproduction
    ↓
Lineage
```

У моделі спадковими можуть бути не лише Genome-параметри.

Але Genome залишається головним довготривалим носієм спадкової регуляції.

---

# Heredity vs Inheritance

`Inheritance` — це подія.

```text
Cell divides
    ↓
Daughter receives Genome, Materials, Resources, Energy, Epigenetic State
```

`Heredity` — це принцип.

```text
Some transmitted properties persist across generations
    ↓
They affect survival and reproduction
    ↓
Selection acts on them
```

Тобто:

```text
inheritance = how state is transferred

heredity = what remains evolutionarily transmissible
```

---

# Що Heredity НЕ є

Heredity не є:

* гарантією точного копіювання;
* гарантією життєздатності;
* передачею досвіду як мутації Genome;
* магічним збереженням виду;
* species memory;
* fitness score;
* зовнішньою оцінкою клітини;
* планом розвитку організму;
* механізмом покращення Genome.

Heredity лише забезпечує передачу варіацій між поколіннями.

Selection визначає, які варіації залишаються.

---

# Спадкові носії

У моделі можуть існувати різні носії спадковості.

Основні:

```text
Genome
Genome fragments
Epigenetic State
Material State
Initial Resources
Initial Energy capacity
Inherited damage
Inherited structure
Lineage context
```

Не всі вони однаково стабільні.

Не всі вони передаються довго.

---

# Primary Heredity

`Primary Heredity` — це спадковість через Genome.

Genome містить регуляторну структуру, яка може передаватися між поколіннями.

Вона визначає:

* які входи може читати клітина;
* як вони комбінуються;
* які процеси отримують пріоритет;
* як клітина будує Materials;
* як підтримує Boundary;
* як виробляє Energy;
* як готується до division;
* як реагує на стрес;
* яку mutation rate має lineage;
* наскільки lineage відкритий до HGT.

Primary Heredity є головним каналом довготривалої еволюції.

---

# Secondary Heredity

`Secondary Heredity` — це спадковість через негеномні стани, які можуть впливати на нащадків.

Приклади:

* epigenetic state;
* material coefficients;
* inherited signal state;
* inherited damage;
* inherited resource balance;
* inherited Boundary quality;
* inherited Heat;
* inherited Joint context.

Secondary Heredity зазвичай менш стабільна, ніж Genome.

Вона може зникати через кілька поколінь.

---

# Genome Heredity

Genome Heredity означає, що дочірні клітини отримують Genome або Genome fragments.

```text
Parent Genome
    ↓
Copy / Transfer
    ↓
Daughter Genome
```

Під час передачі можливі:

* exact copy;
* mutation;
* fragment loss;
* fragment duplication;
* partial inheritance;
* damaged copy;
* recombination;
* HGT-derived fragment inheritance.

Genome Heredity не гарантує збереження форми або поведінки.

Вона передає регуляторну структуру.

---

# Fragment Heredity

Якщо Genome складається з фрагментів, спадковість може бути частковою.

```text
Genome Pool
├── Core Fragment
├── Energy Fragment
├── Boundary Fragment
├── Signal Fragment
└── Mobile Fragment
```

Дочірня клітина може отримати:

* всі фрагменти;
* лише частину;
* дублікати;
* пошкоджені фрагменти;
* нові фрагменти після HGT;
* recombined fragments.

Fragment Heredity важлива для майбутніх моделей:

* plasmid-like genome;
* horizontal transfer;
* modular evolution;
* parasitic genetic elements;
* partial genome loss.

---

# Core Heredity

`Core Heredity` — це передача мінімальної регуляторної основи, потрібної для життєздатності.

Core-регуляція може включати:

* Boundary maintenance;
* basic Energy production;
* Resource uptake;
* Material synthesis;
* repair;
* Genome copying;
* division preparation.

Це не означає, що Core Genome має бути hardcoded.

Core — це аналітичне поняття.

Selection покаже, які фрагменти фактично стали критичними.

---

# Epigenetic Heredity

Epigenetic State може частково передаватися дочірнім клітинам.

Це може впливати на:

* стартову спеціалізацію;
* dormancy;
* stress response;
* repair priority;
* development path;
* signal sensitivity;
* division readiness.

Epigenetic Heredity не змінює Genome.

```text
Same Genome
+
Different inherited epigenetic state
    ↓
Different early cell behavior
```

Epigenetic Heredity може бути короткочасною.

Вона може згасати через кілька Tick або поколінь.

---

# Material Heredity

Матеріали, які отримує дочірня клітина, також можуть створювати спадковий ефект.

Дочірня клітина може успадкувати:

* Boundary materials;
* Energy-conversion materials;
* signal-sensitive materials;
* stateful materials;
* repair-capable materials;
* storage materials;
* damaged materials.

Це може впливати на стартову життєздатність і ранню поведінку.

```text
Inherited Material Composition
    ↓
Initial Cell Capabilities
    ↓
Early Survival
```

Material Heredity не є Genome Heredity, але може впливати на selection.

---

# Learning-like State і Heredity

Learning-like state не повинен перетворюватися на Genome mutation.

Але якщо learning-like state фізично збережений у Materials або Runtime State, частина цього стану може бути передана дочірній клітині.

Приклад:

```text
Stateful Material coefficient = 0.8

Division:
  Daughter A receives material with coefficient = 0.7
  Daughter B receives material with coefficient = 0.1
```

Це не означає, що дочірня клітина успадкувала learned rule як Genome.

Вона отримала фізичний стан матеріалу.

Цей стан може з часом зникнути.

---

# Runtime State Heredity

Runtime State може включати:

* accumulated signal;
* delayed signal;
* previous activation;
* impulse memory;
* temporary regulatory state.

Більшість Runtime State краще не робити довготривало спадковою.

Для базової моделі бажано:

```text
Genome State      -> inherited
Epigenetic State  -> partially inherited
Runtime State     -> mostly reset
Material State    -> inherited only if physically attached to Materials
```

---

# Resource Heredity

Ресурси, отримані дочірньою клітиною, можуть впливати на її старт.

Наприклад:

* багато корисного ресурсу;
* багато waste;
* reactive resource;
* нестача critical resource;
* ресурс із `energy_value`;
* ресурс, потрібний для Boundary repair.

Resource Heredity зазвичай короткочасна.

Вона не є довготривалою спадковою програмою, але може впливати на виживання перших Tick після народження.

---

# Energy-related Heredity

Energy Buffer може бути розподілений між дочірніми клітинами.

Але довготривала спадковість Energy залежить не від самого запасу Energy, а від успадкованих Materials і Genome, які визначають:

* Energy capacity;
* Energy production;
* Energy losses;
* Heat resistance;
* dormancy strategy.

```text
Inherited Energy Buffer = short-term start

Inherited Energy System = long-term heritable capability
```

---

# Damage Heredity

Damage може передаватися між поколіннями.

Дочірня клітина може народитися з:

* damaged Boundary;
* damaged Materials;
* damaged Genome;
* high Heat;
* clogged internal volume;
* unstable state.

Damage Heredity може бути шкідливою, але іноді асиметричний розподіл damage може допомогти lineage.

Наприклад, одна дочірня клітина отримує більшість пошкоджень і гине, а інша отримує чистіший стан і виживає.

---

# Structural Heredity

Якщо клітина є частиною багатоклітинної структури, може існувати спадковість початкової структури.

Наприклад:

* дочірня клітина народжується приєднаною через Joint;
* отримує позицію в колонії;
* успадковує локальний сигнал від сусідів;
* має стартову спеціалізацію через epigenetic state;
* отримує частину Boundary/Joints.

Це не global organism blueprint.

Це локальна передача структурного контексту.

---

# Lineage

`Lineage` — це лінія походження клітин.

Lineage дозволяє аналізувати:

* які Genome поширюються;
* які mutations виживають;
* які inheritance patterns стабільні;
* які epigenetic effects тривають;
* які material states допомагають survival;
* які стратегії reproduction стабільні.

Lineage metadata не повинна напряму впливати на поведінку клітини.

---

# Heritable Trait

`Heritable Trait` — це властивість, яка може передаватися між поколіннями і впливати на survival/reproduction.

Приклади:

* tendency to repair Boundary;
* ability to produce Energy from Resource A;
* ability to synthesize Material X;
* high mutation rate;
* low mutation rate;
* HGT openness;
* dormancy response;
* signal sensitivity;
* joint formation tendency;
* asymmetric division tendency;
* learning-like material production.

Trait не обов'язково є одним Gene.

Trait може виникати з багатьох вузлів, Materials і процесів.

---

# Trait Expression

Спадкова властивість може не проявлятися завжди.

Вона може залежати від:

* environment;
* available Resources;
* Materials;
* Fields;
* epigenetic state;
* lifecycle state;
* signals;
* Joint context;
* damage.

```text
Inherited Genome
+
Environment
+
Cell State
    ↓
Trait Expression
```

Тому Genome не є прямою таблицею ознак.

---

# Heritability не означає Determinism

Якщо властивість спадкова, це не означає, що вона завжди проявиться.

Наприклад:

Genome може мати регуляцію для Light-sensitive Material.

Але якщо немає потрібних Resources або Light, ця властивість не проявиться.

```text
Inherited potential
    ≠
Guaranteed expression
```

---

# Genotype-like і Phenotype-like рівні

У моделі можна умовно розрізняти:

```text
Genotype-like:
  inherited regulatory structure

Phenotype-like:
  actual cell structure and behavior
```

Але ці терміни не повинні жорстко копіювати реальну біологію.

У нашій моделі:

```text
Genome + Materials + Resources + Energy + Environment + Runtime State
    ↓
Observed Cell Behavior
```

---

# Heredity і Selection

Selection діє не на Genome напряму, а на наслідки спадкових властивостей.

```text
Heritable Variation
    ↓
Different Cell Behavior
    ↓
Different Survival / Reproduction
    ↓
Population Change
```

Немає окремого `selection_score`, який Genome читає або оптимізує.

Fitness є результатом взаємодії клітини зі світом.

---

# Heredity і Mutation

Mutation створює нову спадкову варіацію.

Якщо мутація передається дочірнім клітинам і впливає на lineage, вона стає частиною heredity.

```text
Mutation
    ↓
Inherited by offspring
    ↓
Selection acts on lineage
```

Мутація, яка виникла в клітині, але не потрапила до нащадків, не має довготривалого еволюційного ефекту.

---

# Heredity і Recombination

Recombination змішує спадковий матеріал.

Heredity забезпечує передачу результату recombination у lineage.

```text
Parent A hereditary material
+
Parent B hereditary material
    ↓
Recombination
    ↓
Offspring hereditary material
```

Recombination може створювати нові комбінації властивостей без створення всього з нуля.

---

# Heredity і HGT

Horizontal Gene Transfer може створити новий спадковий шлях.

Фрагмент, отриманий через HGT, стає еволюційно значущим лише якщо:

* зберігся в клітині;
* впливає на регуляцію;
* передався дочірнім клітинам;
* допоміг або не завадив lineage вижити.

```text
HGT fragment
    ↓
Integration / Persistence
    ↓
Division
    ↓
Fragment inherited
    ↓
Lineage effect
```

---

# Heredity і Reproductive Strategy

Стратегія розмноження також може бути спадковою.

Genome може впливати на:

* division threshold;
* mutation rate;
* recombination tendency;
* HGT openness;
* genome isolation;
* fragment copy count;
* asymmetric division;
* gamete-like export;
* fusion readiness.

Ці властивості можуть еволюціонувати, якщо впливають на виживання lineage.

---

# Heredity і Species

У рушії не повинно бути hardcoded `species_id`.

Species-like clusters можуть виникати як стабільні lineage, які мають схожі спадкові властивості.

```text
Stable heredity
+
limited exchange
+
selection
+
time
    ↓
species-like cluster
```

Але рушій не повинен використовувати species id для поведінки клітин.

---

# Heredity і Organism

У багатоклітинних структурах спадковість може стосуватися не лише однієї клітини, а й здатності lineage будувати організм.

Але Genome не повинен містити готовий blueprint організму.

Організм виникає через:

* local regulation;
* development;
* Joint formation;
* epigenetic differentiation;
* resource sharing;
* selection.

Спадковою є здатність до такого розвитку, а не готова форма.

---

# Heredity і Development

Development — це прояв спадкової регуляції в конкретних умовах.

```text
Inherited Genome
+
Inherited State
+
Local Environment
+
Signals
    ↓
Development Path
```

Той самий Genome може давати різні клітинні стани залежно від умов.

Це важливо для багатоклітинності й спеціалізації.

---

# Heredity і Neural-like Behavior

Neural-like behavior може бути спадковим як здатність, але не як конкретний learned state.

Спадковими можуть бути:

* здатність синтезувати signal-sensitive Materials;
* здатність створювати stateful Materials;
* thresholds у Regulatory Network;
* impulse decay parameters;
* Joint signal tendencies;
* signal gain regulation.

Неспадковим або слабо спадковим є:

* конкретний накопичений імпульс;
* конкретний досвід;
* тимчасова зміна коефіцієнта;
* runtime activation history.

```text
Inherited:
  ability to learn-like adapt

Not directly inherited:
  learned response itself
```

---

# Heredity і Stability

Щоб спадковість працювала, потрібен баланс між стабільністю і варіативністю.

Занадто висока стабільність:

* мало варіацій;
* повільна адаптація;
* ризик вимирання при зміні середовища.

Занадто низька стабільність:

* lineage не зберігає корисні властивості;
* багато lethal offspring;
* collapse через genome instability.

Selection може підтримувати проміжний баланс.

---

# Heredity і Fidelity

`fidelity` — це точність передачі спадкового стану.

Вона може стосуватися:

* Genome copy fidelity;
* Fragment inheritance fidelity;
* Epigenetic inheritance fidelity;
* Material state inheritance fidelity;
* Boundary distribution fidelity.

Висока fidelity зберігає успішні властивості.

Низька fidelity створює більше варіацій.

---

# Heredity і Variation

Variation — це різниця між нащадками.

Джерела variation:

* mutation;
* recombination;
* HGT;
* asymmetric inheritance;
* epigenetic variation;
* resource/material distribution noise;
* damage inheritance;
* random developmental differences.

Без variation selection не має з чим працювати.

---

# Heredity і Drift

Не всі зміни lineage є результатом selection.

Можливий drift — випадкова зміна частоти властивостей.

Наприклад:

* нейтральна mutation випадково поширилась;
* мала популяція втратила корисний fragment;
* випадковий розподіл resources дав перевагу одному lineage;
* bottleneck зменшив різноманіття.

Drift не потребує окремого керуючого механізму.

Він виникає з випадковості, population size і survival events.

---

# Heredity Metrics

Для аналізу можна зберігати метрики:

```text
genome_id
lineage_id
generation
parent_ids
mutation_count
inherited_fragments
epigenetic_inheritance_score
material_inheritance_score
offspring_viability
division_success_rate
descendant_count
```

Ці метрики не повинні керувати поведінкою клітини.

Вони потрібні для дослідження.

---

# Minimal Heredity Для базової моделі

Для базової моделі достатньо:

```text
Genome is copied to daughters.
Mutation may occur during copying.
Resources and Materials are split.
Energy Buffer is split within capacity.
Epigenetic State is partially inherited or reset.
Runtime State is mostly reset.
Lineage metadata is tracked for analysis.
```

Цього достатньо, щоб selection могла діяти на спадкову регуляцію.

---

# базова модель Heredity Model

Стартова модель:

```text
Hereditary Core:
  Genome regulatory graph

Short-term inherited state:
  Resources
  Materials
  Energy
  Epigenetic State
  Damage
  Heat

Mostly non-hereditary:
  Runtime activations
  accumulated signals
  temporary learning-like responses
```

---

# Приклад 1. Спадкова здатність до Energy production

```text
Parent Genome:
  Resource A input -> produce_energy output

Parent Cell:
  can produce Energy from Resource A

Daughter Cell:
  inherits Genome

Environment:
  Resource A exists

Result:
  daughter can also produce Energy from Resource A
```

Це спадкова властивість.

---

# Приклад 2. Властивість не проявилась

```text
Daughter inherits Genome for Light-sensitive Material.

Environment:
  no Light
  no Resource needed to synthesize Light-sensitive Material

Result:
  trait exists as inherited potential
  but does not express
```

Спадковість не гарантує прояву.

---

# Приклад 3. Epigenetic стартова спеціалізація

```text
Parent Cell:
  epigenetic_state biases Boundary repair

Daughter:
  inherits part of epigenetic_state

Result:
  daughter starts with higher repair priority
```

Genome не змінився.

Це secondary heredity.

---

# Приклад 4. Learning-like state не став Genome

```text
Parent Cell:
  repeated signal changed stateful material coefficient

Division:
  Daughter receives part of this material state

Result:
  daughter starts with changed sensitivity

But:
  Genome regulatory network is unchanged
```

Це не mutation.

---

# Приклад 5. Mutation стала спадковою

```text
During Genome copying:
  edge weight changes from 0.4 to 0.6

Daughter survives and divides.

Granddaughter inherits changed Genome.

Result:
  mutation becomes part of lineage heredity
```

---

# Приклад 6. HGT fragment став спадковим

```text
Cell absorbs genetic fragment from environment.
Fragment integrates into Genome Pool.
Cell divides.
Daughter receives fragment.

Result:
  HGT-derived trait becomes hereditary.
```

---

# Приклад 7. Неспадкова зміна

```text
Cell has high Heat for several Tick.
Heat temporarily changes process speed.
Cell later divides.
Daughter does not receive Heat state.

Result:
  no hereditary effect
```

Якщо Heat пошкодив Genome або Materials, тоді ефект може стати спадковим через пошкоджений Genome або inherited Material State.

---

# Приклад 8. Drift

```text
Two neutral Genome variants exist:
  Variant A
  Variant B

Small population.
Random deaths remove most Variant B.

Result:
  Variant A dominates not because it is better,
  but because of random drift.
```

---

# Правила

## Rule 1. Heredity requires transmission

Властивість є спадковою лише якщо вона може передаватися нащадкам.

## Rule 2. Genome is primary hereditary carrier

Genome є головним довготривалим носієм спадкової регуляції.

## Rule 3. Non-genome state may be secondarily hereditary

Epigenetic, Material і стартові фізичні стани можуть частково передаватися, але зазвичай менш стабільні.

## Rule 4. Learning is not Genome heredity

Learning-like state не переписує Genome і не стає mutation.

## Rule 5. Heritable potential is not guaranteed expression

Спадкова властивість може не проявитися без відповідних умов.

## Rule 6. Selection acts on expressed outcomes

Selection діє через survival і reproduction, а не через пряме читання Genome.

## Rule 7. Variation is necessary

Mutation, recombination, HGT і asymmetric inheritance створюють варіативність.

## Rule 8. Drift is allowed

Не всі зміни lineage повинні бути adaptive.

## Rule 9. No species id

Species-like структури можуть виникати, але не мають бути hardcoded.

## Rule 10. Heredity must scale

Модель спадковості повинна працювати для одноклітинних, колоній і багатоклітинних структур.

---

# Заборонено

Не вводити:

* inherited learned behavior as Genome rewrite;
* guaranteed expression of inherited trait;
* hardcoded species heredity;
* direct fitness score inheritance;
* automatic beneficial heredity;
* perfect copying as required rule;
* organism blueprint;
* plant/animal-specific heredity rules;
* species_id as behavior input;
* engine-level protection of successful traits.

---

# Пов'язані документи

* `genetics/inheritance.md`
* `genetics/mutation.md`
* `genetics/recombination.md`
* `genetics/horizontal-transfer.md`
* `genetics/epigenetics.md`
* `genetics/regulatory-network.md`
* `genetics/genome-runtime.md`
* `biology/genome.md`
* `biology/cell.md`
* `biology/lifecycle.md`
* `biology/development.md`
* `biology/organism.md`
* `world/materials.md`
* `world/energy.md`
* `world/laws.md`

# Open Questions

## Boundary between inheritance and heredity

Потрібно уточнити, які частини `inheritance.md` є технічним механізмом, а які вважати heredity-level принципами.

## Epigenetic duration

Потрібно визначити, скільки поколінь може тривати epigenetic effect.

## Material state heredity

Потрібно вирішити, чи stateful material coefficients передаються у базовій моделі.

## Runtime state reset

Потрібно визначити, які runtime states завжди скидаються при поділі.

## Heredity metrics

Потрібно визначити мінімальний набір lineage metrics Для базової моделі.

## Drift analysis

Потрібно вирішити, чи буде рушій явно логувати drift events, чи це буде аналізуватися зовнішніми інструментами.

## Fragment heredity

Потрібно визначити, чи fragment-based heredity входить у базову модель або залишається future model.

## Species-like clustering

Потрібно вирішити, чи потрібен аналітичний інструмент для групування lineage у species-like clusters без впливу на поведінку клітин.


