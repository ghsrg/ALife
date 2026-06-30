# genome.md

> **Genome — фізичний носій спадкової регуляції клітини**

---

# Призначення

Цей документ описує `Genome` — фізичну спадкову структуру, яка регулює процеси клітини.

Genome не є мозком.

Genome не є програмою поведінки.

Genome не є списком готових дій.

Genome не є таблицею “як вижити”.

Genome є спадковою регуляторною системою, яка впливає на те, як клітина реагує на локальний стан, ресурси, матеріали, Energy Buffer, поля, сигнали та пошкодження.

---

# Основна ідея

Клітина має фізичний генетичний носій.

Цей носій може:

- копіюватися;
- мутувати;
- пошкоджуватися;
- передаватися дочірнім клітинам;
- частково втрачатися;
- частково дублюватися;
- змішуватися з іншим генетичним матеріалом;
- поглинатися з середовища;
- брати участь у горизонтальному переносі.

Genome регулює клітинні процеси.

```text
Inputs
    ↓
Genome / Regulation
    ↓
Cell Process Priorities
    ↓
Action Plan
    ↓
Cell State Changes
```

Точний інтерфейс між Genome Runtime, ActionPlan, Feasibility Check і Process Execution описаний у `genetics/regulatory-interface.md`.

---

# Що Genome НЕ є

Genome не є:

- поведінковим деревом;
- if/else-скриптом;
- готовим мозком;
- списком органів;
- списком видів клітин;
- списком цілей;
- планом організму у готовому вигляді;
- прямим контролером світу;
- джерелом фізичної енергії.

Genome не створює матерію.

Genome не виконує фізичну роботу.

Genome не знає глобальний стан світу.

Genome працює лише з локальними входами клітини.

---

# Genome як фізичний об'єкт

Genome має фізичний носій.

Це означає, що він:

- займає внутрішній об'єм клітини;
- має масу;
- може пошкоджуватися;
- може копіюватися;
- може деградувати;
- може бути втрачений;
- може бути переданий;
- може існувати як фрагмент після смерті клітини.

Genome не є абстрактним JSON, який висить поза світом.

Навіть якщо в реалізації він зберігається як структура даних, у моделі світу він повинен поводитися як фізична спадкова структура.

---

# Genome і Cell

Genome належить клітині або знаходиться всередині клітини.

Клітина може мати:

- один Genome;
- кілька генетичних фрагментів;
- plasmid-like fragments;
- пошкоджений Genome;
- неповний Genome;
- тимчасово поглинутий генетичний матеріал.

У базовій моделі можна почати з одного Genome на клітину.

Але модель не повинна блокувати майбутній перехід до множинних генетичних фрагментів.

---

# Genome і регуляція

Genome регулює, які процеси клітина намагається виконати.

Наприклад, Genome може впливати на:

- поглинання ресурсів;
- виведення ресурсів;
- виробництво Energy;
- синтез матеріалів;
- ремонт;
- деградацію матеріалів;
- рух;
- створення Joint;
- розрив Joint;
- поділ;
- спокій;
- реакцію на Heat;
- реакцію на Pressure;
- реакцію на сигнали сусідів;
- зміну епігенетичного стану.

Genome не виконує ці дії напряму.

Він формує регуляторний вихід.

---

# Вхідні сигнали Genome

Genome отримує не весь стан світу, а лише локальні входи клітини.

Приклади входів:

```text
internal_energy_level
free_capacity
resource_concentrations
material_levels
boundary_integrity
local_heat
local_pressure
damage_level
field_inputs
joint_signals
contact_signals
epigenetic_state
cell_age
division_readiness
```

Ці входи не обов'язково існують як фіксований список назавжди.

Але будь-який вхід Genome повинен бути отриманий з фізичного або клітинного стану.

---

# Вихідні сигнали Genome

Genome не повертає готову дію.

Genome повертає регуляторні сигнали або пріоритети процесів.

Приклади виходів:

```text
uptake_resource_A_priority
export_resource_B_priority
synthesize_material_X_priority
repair_boundary_priority
produce_energy_priority
divide_priority
create_joint_priority
break_joint_priority
movement_intensity
dormancy_level
mutation_rate_modifier
hgt_uptake_level
```

Рушій потім перевіряє, чи ці дії фізично можливі.

---

# Genome не обходить обмеження

Навіть якщо Genome “хоче” виконати дію, дія може бути неможливою.

Причини:

- недостатньо Energy;
- недостатньо ресурсів;
- недостатньо матеріалів;
- недостатньо внутрішнього об'єму;
- пошкоджена Boundary;
- немає потрібного Material;
- немає фізичного контакту;
- дія суперечить physics.md;
- клітина мертва або нежиттєздатна.

Genome пропонує регуляторний план.

Світ вирішує, що реально може відбутися.

---

# Genome і Materials

Genome не створює матеріали напряму.

Genome може лише регулювати процеси, які синтезують, ремонтують або деградують матеріали.

```text
Genome output
    ↓
Synthesis Process
    ↓
Resource + Energy
    ↓
Material
```

Матеріал виникає лише через фізичний процес.

---

# Genome і Resources

Genome не створює ресурси з нічого.

Genome може регулювати:

- поглинання ресурсу;
- виведення ресурсу;
- перетворення ресурсу;
- використання ресурсу для Energy;
- використання ресурсу для Material synthesis;
- нейтралізацію ресурсу;
- накопичення ресурсу.

Ресурс завжди залишається фізичною речовиною.

---

# Genome і Energy

Genome не є джерелом Energy.

Genome може регулювати:

- виробництво Energy;
- витрати Energy;
- пріоритети активних процесів;
- перехід у dormancy;
- синтез матеріалів, що збільшують Energy Buffer capacity;
- реакцію на нестачу Energy;
- реакцію на надлишок Heat.

Genome inference сам може мати невелику вартість Energy, якщо така модель увімкнена.

---

# Genome і Boundary

Genome може впливати на Boundary лише через матеріальні процеси.

Наприклад:

```text
Genome regulation
    ↓
Synthesize Boundary Material
    ↓
Boundary Integrity changes
```

Genome не може напряму зробити Boundary міцною, проникною або селективною.

Це завжди результат матеріалів.

---

# Genome і Joint

Genome може регулювати створення, підтримку або руйнування Joint.

Але Joint виникає лише якщо є:

- контакт;
- відповідні Boundary materials;
- Energy;
- фізична можливість;
- сумісна локальна структура;
- регуляторний сигнал.

Genome не створює Joint дистанційно.

---

# Genome і Learning

Learning не змінює Genome напряму.

Learning змінює тимчасовий або довготривалий стан клітини чи організму:

- epigenetic_state;
- neural-like material state;
- signal weights;
- local regulatory memory;
- behavior adaptation.

Genome може визначати здатність до learning.

Але learning не є спадковою мутацією.

---

# Genome і Epigenetic State

Epigenetic State — це проміжний шар між Genome і поточним станом клітини.

```text
Genome
    +
Epigenetic State
    +
Inputs
    ↓
Regulatory Output
```

Epigenetic State може змінювати:

- чутливість до входів;
- пороги активації;
- пріоритети процесів;
- стан dormancy;
- схильність до поділу;
- спеціалізацію клітини.

Epigenetic State не змінює сам Genome.

Він може частково передаватися дочірнім клітинам.

---

# Genome і Development

Development — це процес, у якому один і той самий Genome може приводити до різних клітинних станів залежно від:

- локального середовища;
- сигналів сусідів;
- історії клітини;
- положення в організмі;
- epigenetic_state;
- доступних ресурсів;
- матеріального складу.

Genome не повинен містити готову форму організму.

Організм виникає через локальні правила розвитку.

---

# Genome і спеціалізація клітин

У рушії не існує hardcoded типів клітин.

Немає:

- MuscleCell;
- NeuronCell;
- SkinCell;
- SensorCell;
- StorageCell.

Спеціалізація виникає, якщо клітини з однаковим або схожим Genome переходять у різні стабільні стани через різні локальні умови.

```text
Same Genome
    +
Different Signals
    +
Different Epigenetic State
    ↓
Different Material Composition
    ↓
Different Cell Function
```

---

# Мінімальна структура Genome

У базовій моделі Genome представлений як Direct Regulatory Graph.

Мінімально Genome повинен дозволяти:

- читати локальні входи;
- формувати вихідні регуляторні сигнали;
- мутувати;
- копіюватися;
- передаватися дочірнім клітинам;
- мати фізичну вартість;
- мати обмежений розмір;
- впливати на виживання лише через клітинні процеси.

---

# Базова модель: Direct Regulatory Graph

Поточний базовий варіант:

```text
Inputs
    ↓
Regulatory Nodes
    ↓
Outputs
```

Genome складається з вузлів і зв'язків.

```text
Genome
├── input bindings
├── regulatory nodes
├── regulatory edges
├── output bindings
└── mutation parameters
```

Це дозволяє моделювати просту спадкову регуляцію без hardcoded поведінки.

Direct Regulatory Graph є прийнятим напрямком, який зійшовся по суті й механіці. Він ще не є монолітом: можна коригувати bindings, limits, mutation parameters і шлях до fragment-compatible моделі.

---

# Input Bindings

Input Binding визначає, який клітинний або зовнішній сигнал може бути використаний Genome.

Приклади:

```text
Energy level
Heat level
Resource A concentration
Material X amount
Boundary integrity
Joint signal
Pressure
Light
Damage level
Free capacity
```

Input Binding не дає глобального знання.

Він лише підключає локальний вимірюваний стан.

---

# Regulatory Nodes

Regulatory Node — внутрішній вузол геномної регуляторної мережі.

Він може:

- комбінувати сигнали;
- підсилювати сигнал;
- пригнічувати сигнал;
- задавати пороги;
- створювати нелінійну реакцію;
- підтримувати внутрішній регуляторний стан.

Regulatory Node не є нейроном у біологічному сенсі.

Це абстрактний вузол спадкової регуляції.

---

# Regulatory Edges

Regulatory Edge визначає вплив одного вузла на інший.

Edge може мати:

```text
source
target
weight
activation_type
delay
stability
mutation_rate
```

Edge не є фізичною силою.

Це спадкова регуляторна залежність.

---

# Output Bindings

Output Binding зв'язує вихід Genome з клітинним процесом.

Приклади:

```text
produce_energy
synthesize_material_X
repair_material_X
export_resource_A
uptake_resource_B
create_joint
break_joint
move
divide
enter_dormancy
increase_mutation_rate
allow_hgt_uptake
```

Output Binding не гарантує виконання дії.

Він лише задає регуляторний пріоритет.

---

# Action Planning

Genome створює не дії, а регуляторні пріоритети.

Після цього клітина будує Action Plan.

```text
Genome Output
    ↓
Action Priorities
    ↓
Feasibility Check
    ↓
Action Execution
```

Feasibility Check перевіряє:

- Energy;
- ресурси;
- матеріали;
- об'єм;
- Boundary;
- Physics;
- Joint;
- стан клітини.

Genome output не може обходити `biology/feasibility.md` і не є direct command.

---

# Genome Size

Genome має розмір.

Розмір Genome впливає на:

- об'єм;
- вартість копіювання;
- вартість підтримки;
- вартість inference;
- ризик мутацій;
- складність регуляції.

Більший Genome може бути корисним, але не є безкоштовним.

---

# Genome Cost

Genome може мати такі витрати:

- storage cost;
- replication cost;
- repair cost;
- inference cost;
- mutation risk;
- degradation risk.

Це створює природний тиск проти необмеженого зростання Genome.

---

# Genome Copying

При поділі клітини Genome повинен копіюватися або розподілятися.

Копіювання може бути:

- точним;
- частково пошкодженим;
- з мутаціями;
- неповним;
- надлишковим;
- з дублюванням фрагментів.

Копіювання потребує:

- ресурсів;
- Energy;
- часу;
- фізичного місця;
- відповідних матеріалів або процесів.

---

# Mutation

Mutation — це випадкова зміна Genome.

Мутації можуть:

- змінити weight;
- додати node;
- видалити node;
- додати edge;
- видалити edge;
- змінити output binding;
- змінити input binding;
- змінити mutation parameters;
- дублювати фрагмент;
- пошкодити фрагмент;
- змінити regulatory delay;
- змінити threshold.

Мутація не має мети.

Мутація не знає, чи буде корисною.

Selection визначає наслідки.

---

# Типи мутацій

Приклади можливих мутацій:

```text
weight_shift
edge_addition
edge_deletion
node_addition
node_deletion
binding_change
fragment_duplication
fragment_deletion
fragment_inversion
fragment_merge
fragment_split
mutation_rate_change
```

Не всі типи потрібні у базовій моделі.

---

# Mutation Rate

Mutation Rate може бути спадковою властивістю Genome.

Genome може регулювати власну схильність до мутацій, але не може обирати корисні мутації.

```text
higher mutation rate
    ↓
more variation
    ↓
higher risk
```

Mutation Rate може залежати від:

- геномної структури;
- стану клітини;
- Heat;
- Radiation;
- помилок копіювання;
- пошкоджень;
- ресурсів;
- Energy;
- спеціальних регуляторних механізмів.

---

# Genome Damage

Genome може пошкоджуватися через:

- Heat;
- Radiation;
- реакції ресурсів;
- нестачу repair;
- фізичне руйнування клітини;
- помилки копіювання;
- деградацію після смерті.

Genome Damage може призвести до:

- втрати функцій;
- неправильних виходів;
- нездатності до поділу;
- нежиттєздатних дочірніх клітин;
- нових варіацій;
- смерті клітини.

---

# Genome Repair

Genome може ремонтуватися, якщо у клітини є відповідні ресурси, Energy і матеріальні механізми.

Repair не гарантує точність.

Repair може:

- виправити пошкодження;
- створити помилку;
- видалити фрагмент;
- дублювати фрагмент;
- стабілізувати Genome.

Genome Repair є клітинним процесом, а не магічною операцією.

---

# Heredity

Heredity — це передача Genome або його частини дочірнім клітинам.

Під час поділу передаються:

- Genome;
- ресурси;
- матеріали;
- Energy Buffer;
- epigenetic_state;
- пошкодження;
- можливо, Joint context.

Дочірня клітина не успадковує “досвід” напряму.

Вона успадковує фізичний і регуляторний стан.

---

# Reproduction Strategy

Стратегія розмноження не повинна бути hardcoded режимом рушія.

Genome може регулювати параметри, які впливають на розмноження:

```text
division_threshold
genome_copy_rate
mutation_rate
recombination_rate
hgt_uptake_level
genome_isolation_level
fragment_copy_count
gamete_like_export
fusion_readiness
```

Таким чином різні стратегії розмноження можуть виникати еволюційно.

---

# Recombination

Recombination — це змішування генетичного матеріалу.

У моделі не потрібно вимагати ідеального “вирівнювання” двох Genome.

Можливі прості оператори:

- insert fragment;
- replace fragment;
- duplicate fragment;
- delete fragment;
- merge fragment;
- split fragment;
- broken merge;
- partial overwrite.

Більшість комбінацій можуть бути шкідливими.

Selection відфільтрує нежиттєздатні варіанти.

---

# Horizontal Gene Transfer

Horizontal Gene Transfer — це передача генетичного матеріалу між клітинами без прямого поділу батько-дитина.

Він може відбуватися через:

- поглинання genetic fragment;
- контакт;
- Joint;
- середовище;
- вірусоподібні структури в майбутньому;
- мертві клітини.

HGT не повинен бути окремою магією.

Це фізичне переміщення генетичного матеріалу.

---

# Genome Isolation

Клітини можуть еволюційно збільшувати або зменшувати відкритість до чужого генетичного матеріалу.

Висока відкритість може давати:

- швидшу адаптацію;
- більше варіацій;
- ризик руйнування регуляції;
- ризик паразитичних фрагментів.

Висока ізоляція може давати:

- стабільність;
- захист від шкідливого Genome;
- повільнішу адаптацію.

---

# Plasmid-like Genome Model

У майбутній моделі Genome може бути не одним монолітом, а набором генетичних фрагментів.

```text
Genome Pool
├── Core Fragment
├── Regulatory Fragment
├── Metabolic Fragment
├── Transport Fragment
└── Mobile Fragment
```

Кожен фрагмент може мати:

- власні regulatory nodes;
- власні output bindings;
- власну copy rate;
- власну stability;
- власний mutation rate;
- власну здатність до передачі.

Це краще підтримує:

- HGT;
- дублювання;
- втрату фрагментів;
- паразитичні елементи;
- модульну еволюцію;
- варіативне розмноження.

Для базової моделі це можна залишити як Research/Future Work.

---

# Core Genome і Mobile Fragments

Можна розрізняти:

```text
Core Genome
Mobile Genetic Fragments
```

Core Genome містить мінімальну регуляторну основу клітини.

Mobile Fragments можуть додавати або змінювати регуляцію.

Але це не повинно бути hardcoded біологічним правилом.

Це лише можлива архітектура моделі.

---

# Genome Runtime

Genome Runtime — це процес виконання регуляторної мережі протягом Cell Decision phase.

Порядок:

```text
1. Collect inputs
2. Apply epigenetic modifiers
3. Run regulatory network
4. Produce output priorities
5. Build action plan
6. Pass plan to feasibility check
```

Genome Runtime не змінює світ напряму.

---

# Determinism

За однакових умов, однакового Genome і однакового random seed Genome Runtime повинен давати однаковий результат.

Це потрібно для:

- відтворюваності;
- тестування;
- дебагу;
- наукового аналізу.

---

# Randomness

Randomness дозволена лише в контрольованих місцях:

- мутації;
- пошкодження;
- копіювання;
- рекомбінація;
- деякі стохастичні регуляторні процеси;
- випадковий дрейф.

Randomness не повинна підміняти регуляцію.

---

# Genome і Selection

Selection не є окремим механізмом, який оцінює Genome.

Selection виникає природно.

Якщо Genome створює регуляцію, яка дозволяє клітині:

- вижити;
- отримувати ресурси;
- підтримувати структуру;
- відтворюватися;
- конкурувати;
- передавати спадковість,

то такий Genome поширюється.

Якщо ні — він зникає.

---

# Нежиттєздатний Genome

Genome може бути нежиттєздатним.

Це нормально.

Наприклад, Genome може:

- не виробляти Boundary materials;
- не підтримувати Energy;
- не копіюватися;
- не ремонтувати пошкодження;
- синтезувати зайві матеріали;
- накопичувати шкідливі ресурси;
- руйнувати власну клітину;
- не реагувати на середовище.

Рушій не повинен забороняти такі Genome.

Вони просто не виживуть.

---

# Мінімальний Genome для базової моделі

Мінімальний Genome повинен вміти:

- читати кілька локальних входів;
- керувати кількома процесами;
- мутувати;
- копіюватися;
- передаватися при поділі;
- мати вартість;
- мати обмежений розмір;
- давати як життєздатні, так і нежиттєздатні варіанти.

базова модель не потребує повної моделі реальної ДНК.

---

# Приклад Genome базової моделі

```text
Inputs:
  energy_level
  free_capacity
  resource_A_inside
  resource_A_outside
  material_boundary_level
  heat_level

Outputs:
  uptake_resource_A
  produce_energy
  synthesize_boundary_material
  repair_boundary
  divide
  enter_dormancy

Regulatory Nodes:
  node_1
  node_2
  node_3

Edges:
  energy_level -> node_1
  resource_A_inside -> node_2
  node_1 -> produce_energy
  node_2 -> synthesize_boundary_material
  heat_level -> enter_dormancy
```

Це не сценарій поведінки.

Це спадкова регуляторна структура.

---

# Сумісність із майбутніми моделями

Документ не фіксує остаточно конкретний формат Genome.

Допустимі майбутні реалізації:

- direct regulatory graph;
- plasmid-like genome pool;
- modular regulatory fragments;
- hybrid graph-fragment model;
- chemical-tag model;
- інша модель, якщо вона зберігає принципи.

Будь-яка реалізація повинна відповідати правилам цього документа.

---

# Правила

## Rule 1. Genome is physical

Genome має фізичний носій у клітині.

## Rule 2. Genome regulates

Genome регулює процеси, але не виконує фізичну роботу.

## Rule 3. Genome has local inputs

Genome отримує лише локальні входи клітини.

## Rule 4. Genome outputs priorities

Genome повертає регуляторні пріоритети, а не гарантовані дії.

## Rule 5. Genome cannot bypass physics

Будь-який вихід Genome проходить через feasibility check.

## Rule 6. Genome mutates randomly

Мутації не мають мети.

## Rule 7. Selection evaluates outcomes

Selection діє через виживання і розмноження, а не через зовнішню оцінку Genome.

## Rule 8. Genome has cost

Genome має вартість зберігання, копіювання, підтримки та виконання.

## Rule 9. Learning is not Genome mutation

Learning не змінює Genome напряму.

## Rule 10. Genome model must allow future HGT

Модель Genome не повинна блокувати горизонтальний перенос генетичного матеріалу.

---

# Заборонено

Не вводити:

- behavior tree як Genome;
- hardcoded species genome;
- готові органи в Genome;
- готові типи клітин у Genome;
- пряме створення матеріалів без ресурсів;
- пряме створення Energy;
- глобальне знання світу;
- цільову корисну мутацію;
- magic recombination;
- selection score, який напряму оцінює Genome;
- навчання як пряме переписування Genome.

---

# Пов'язані документи

- `biology/cell.md`
- `biology/membrane.md`
- `biology/lifecycle.md`
- `biology/development.md`
- `biology/heredity.md`
- `biology/mutations.md`
- `biology/learning.md`
- `biology/joint.md`
- `world/resources.md`
- `world/materials.md`
- `world/energy.md`
- `world/physics.md`
- `world/tick.md`
- `genetics/regulatory-interface.md`
- `biology/feasibility.md`
- `research/genome-representation.md`
- `research/graph-crossover.md`
- `research/plasmid-genome.md`
- `research/reproduction-strategies.md`
- `research/genome-runtime.md`

# Open Questions

## Межі Direct Regulatory Graph

Direct Regulatory Graph є поточним базовим варіантом.

Потрібно уточнити, які частини моделі залишаються стабільними, а які можна коригувати після перших експериментів:

- набір input bindings;
- набір output bindings;
- ліміти nodes/edges;
- форма mutation parameters;
- шлях переходу до fragment-compatible моделі.

## Graph crossover

Потрібно деталізувати, як змішувати два regulatory graph без вимоги ідеального вирівнювання.

## Physical genome carrier

Потрібно визначити, як саме Genome займає об'єм і деградує:

- як агрегований genetic material;
- як набір fragments;
- як окремі physical objects;
- як внутрішня структура клітини.

## Genome inference cost

Потрібно визначити, чи Genome Runtime витрачає Energy У базовій моделі.

## Epigenetic inheritance

Потрібно визначити, яка частина epigenetic_state може передаватися дочірнім клітинам.

## HGT integration

Потрібно визначити, як клітина інтегрує поглинутий genetic fragment:

- автоматично;
- через регуляторний процес;
- через матеріальний механізм;
- з ризиком пошкодження Genome.

## Genome repair

Потрібно визначити, чи Genome repair входить у базову модель або залишається майбутнім розвитком.

## Minimal viable Genome

Потрібно визначити мінімальний набір inputs/outputs для першої реалізації.

