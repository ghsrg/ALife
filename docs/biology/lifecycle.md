# lifecycle.md

> **Cell Lifecycle — життєвий цикл клітини**

---

# Призначення

Цей документ описує життєвий цикл клітини в Artificial Life Engine.

Lifecycle визначає, як клітина:

* виникає;
* підтримує себе;
* росте;
* готується до поділу;
* ділиться;
* переходить у спокій;
* пошкоджується;
* помирає;
* розкладається;
* повертає свої ресурси та матеріали у світ.

Lifecycle не є поведінковим сценарієм.

Lifecycle — це послідовність можливих станів, які виникають із фізичного, матеріального, енергетичного та геномного стану клітини.

Функціональний контракт станів клітини описаний у `biology/cell-state.md`.

Фізичний розподіл стану під час поділу описаний у `biology/division-partition.md`.

---

# Основна ідея

Клітина живе, поки здатна підтримувати свою матеріальну структуру.

Життя не визначається HP.

Життя визначається здатністю клітини:

* утримувати Boundary;
* підтримувати внутрішній склад;
* виробляти або зберігати Energy;
* ремонтувати критичні матеріали;
* виконувати мінімальні процеси;
* зберігати або копіювати Genome;
* компенсувати деградацію.

```text
Birth
  ↓
Alive
  ↓
Growth / Maintenance / Adaptation
  ↓
Division or Dormancy or Damage
  ↓
Death
  ↓
Decomposition
```

---

# Lifecycle State

Клітина може мати технічний `lifecycle_state`.

Приклади:

```text
newborn
alive
growing
stressed
dormant
damaged
division_preparing
dividing
dead
decomposing
```

Ці стани потрібні рушію для організації процесів.

Вони не повинні підміняти фізичну модель.

Наприклад, `dead` не означає “HP = 0”.

`dead` означає, що клітина більше не може підтримувати мінімальну функціональну структуру.

---

# Birth / виникнення клітини

Клітина може виникнути через:

* поділ іншої клітини;
* відокремлення частини клітини;
* розпад колонії;
* майбутні репродуктивні механізми;
* експериментальне початкове створення світом у seed-сценарії.

У звичайній симуляції нова клітина повинна виникати з уже наявної матерії.

Клітина не створюється з нічого.

---

# Newborn Cell

Нова клітина отримує частину стану батьківської клітини або клітин.

Вона може отримати:

* Resources;
* Materials;
* Energy Buffer;
* Genome або Genome fragments;
* Epigenetic State;
* Boundary materials;
* Heat;
* пошкодження;
* внутрішній об'єм.

Нова клітина не обов'язково є повністю життєздатною.

Вона може бути:

* стабільною;
* слабкою;
* недобудованою;
* пошкодженою;
* нежиттєздатною.

Selection відфільтрує невдалі варіанти.

---

# Alive

Клітина вважається живою, якщо вона ще здатна підтримувати мінімальну функціональну структуру.

Мінімально жива клітина повинна мати:

* Boundary або часткову Boundary;
* внутрішній склад;
* критичні Materials;
* Energy Buffer або здатність його поповнювати;
* Genome або регуляторний носій;
* здатність виконувати хоча б частину maintenance;
* здатність компенсувати деградацію хоча б тимчасово.

Жива клітина не обов'язково активна.

Вона може бути в dormancy.

---

# Maintenance Phase

Maintenance — це постійна підтримка структури клітини.

Maintenance може включати:

* ремонт Boundary;
* ремонт критичних Materials;
* виведення зайвих Resources;
* підтримку Energy Buffer;
* стабілізацію Genome;
* зниження Heat;
* нейтралізацію небажаних реакцій;
* підтримку Joint.

Якщо maintenance не виконується, клітина може залишатися живою певний час, але поступово деградує.

---

# Growth

Growth — це збільшення матеріальної структури клітини.

Growth не є просто збільшенням числа `size`.

Growth виникає через:

* uptake Resources;
* Energy Production;
* Material Synthesis;
* Boundary expansion;
* збільшення внутрішнього об'єму;
* копіювання або підготовку Genome;
* накопичення ресурсів для майбутнього поділу.

```text
Resources
    +
Energy
    +
Synthesis Processes
    ↓
More Materials
    ↓
Larger Cell
```

---

# Growth Conditions

Клітина може рости лише якщо має:

* достатньо Resources;
* достатньо Energy;
* достатньо Materials для синтезу;
* вільний внутрішній об'єм;
* фізичний простір зовні;
* стабільну Boundary;
* регуляторний сигнал.

Genome може регулювати Growth, але не може створити Growth без фізичних умов.

---

# Stress

`stressed` — це стан, у якому клітина ще жива, але її стабільність погіршується.

Причини stress:

* низька Energy;
* нестача ресурсів;
* накопичення зайвих ресурсів;
* пошкодження Boundary;
* Heat;
* Pressure;
* пошкодження Genome;
* втрата Materials;
* сильні реакції;
* нестача free_capacity;
* розрив Joint;
* невдала підготовка до поділу.

Stress не є окремою шкалою HP.

Stress — це узагальнений технічний стан, який може впливати на пріоритети процесів.

---

# Dormancy

Dormancy — це стан низької активності.

У dormancy клітина може:

* зменшити Energy consumption;
* зупинити Growth;
* зупинити Movement;
* зменшити Active Transport;
* зменшити Material Synthesis;
* підтримувати лише критичний Maintenance;
* чекати кращих умов.

Dormancy не є смертю.

Dormancy може бути корисною стратегією виживання.

---

# Dormancy Conditions

Клітина може перейти в dormancy через:

* низьку Energy;
* низькі ресурси;
* високий Heat;
* надмірний Pressure;
* пошкодження;
* регуляторне рішення Genome;
* епігенетичний стан;
* сигнали від сусідів.

Вихід із dormancy може відбутися, якщо умови покращилися.

---

# Damage

Damage — це зміна або втрата матеріальної структури.

Damage може стосуватися:

* Boundary;
* внутрішніх Materials;
* Genome;
* Joint;
* Energy production materials;
* transport materials;
* storage materials.

Damage не є HP.

Damage завжди повинен бути пов'язаний із конкретною матеріальною або фізичною зміною.

---

# Damage Sources

Damage може виникати через:

* зіткнення;
* Pressure;
* Heat;
* замерзання;
* реакції ресурсів;
* нестачу Energy для Maintenance;
* природну деградацію;
* атаки інших клітин;
* розрив Joint;
* помилки поділу;
* пошкодження Genome;
* засмічення внутрішнього об'єму.

---

# Repair

Repair — це процес відновлення пошкоджених матеріалів.

Repair потребує:

* Energy;
* Resources;
* відповідних Materials або процесів;
* часу;
* регуляторного сигналу;
* free_capacity.

Repair не відновлює HP.

Repair відновлює матеріальний склад і функціональність.

---

# Aging

Aging не є окремим таймером смерті.

Aging — це наслідок того, що клітина з часом накопичує деградацію і не завжди може повністю її компенсувати.

Причини aging:

* природний material decay;
* imperfect repair;
* накопичення waste resources;
* genome damage;
* зниження Energy production;
* Heat history;
* помилки копіювання;
* втрата структурної організації.

Клітина може еволюційно зменшувати aging, але не через магічний параметр `age_resistance`.

Лише через матеріали, repair, регуляцію і контроль ресурсів.

---

# Division Preparation

Division Preparation — це підготовка клітини до поділу.

Вона може включати:

* збільшення Materials;
* накопичення Resources;
* накопичення Energy;
* копіювання Genome;
* підготовку Boundary;
* створення внутрішнього розділення;
* зміну shape;
* відключення або перебудову Joint;
* зміну Epigenetic State.

```text
Growth
  ↓
Genome Copying
  ↓
Boundary Expansion
  ↓
Division Readiness
```

---

# Division Readiness

Клітина готова до поділу, якщо має достатньо:

* Materials;
* Boundary materials;
* Resources;
* Energy;
* Genome copy або Genome fragments;
* внутрішнього об'єму;
* зовнішнього простору;
* стабільності;
* регуляторного сигналу.

Division readiness не повинна бути одним hardcoded порогом `size > X`.

Вона повинна залежати від матеріального і регуляторного стану клітини.

---

# Division

Division — це фізичне розділення однієї клітини на дві або більше частин.

Поділ не копіює клітину магічно. Він partition-ить локальний стан parent cell згідно з `biology/division-partition.md`.

У базовій моделі:

```text
Parent Cell
    ↓
Daughter Cell A
+
Daughter Cell B
```

Під час поділу розподіляються:

* Resources;
* Materials;
* Energy Buffer;
* Genome;
* Epigenetic State;
* Heat;
* пошкодження;
* internal fragments;
* можливо, Joint context.

---

# Division Cost

Поділ не є безкоштовним.

Він може вимагати:

* Energy;
* Boundary materials;
* Genome copying cost;
* structural rearrangement cost;
* часу;
* фізичного простору;
* стабільного стану.

Якщо клітина ділиться занадто рано, дочірні клітини можуть бути нежиттєздатними.

---

# Division Errors

Поділ може бути неідеальним.

Можливі помилки:

* одна дочірня клітина отримала більше ресурсів;
* одна дочірня клітина отримала менше Boundary;
* Genome скопійований з помилкою;
* Genome розподілений нерівномірно;
* Energy Buffer розподілений нерівномірно;
* пошкодження перейшло до однієї з дочірніх клітин;
* Boundary не закрилася повністю;
* дочірня клітина народилася недобудованою.

Такі помилки не треба забороняти.

Вони є джерелом варіативності й selection.

---

# Resource Distribution During Division

Ресурси можуть розподілятися:

* рівномірно;
* випадково;
* за локальною позицією;
* за регуляторним сигналом;
* за фізичною структурою клітини.

У базовій моделі можна почати з простого пропорційного розподілу з невеликим шумом.

---

# Material Distribution During Division

Materials можуть розподілятися:

* пропорційно;
* випадково;
* за локальною структурою;
* через спеціальні division processes;
* із помилками.

Boundary materials особливо важливі, бо кожна дочірня клітина повинна отримати або швидко синтезувати власну Boundary.

---

# Energy Distribution During Division

Energy Buffer може розподілятися між дочірніми клітинами.

Це не є transport Energy між незалежними клітинами.

Energy не передається як речовина в середовище або через Joint.

Під час поділу внутрішній локальний стан батьківської клітини partition-иться між дочірніми клітинами.

Якщо Energy недостатньо, одна або обидві дочірні клітини можуть швидко перейти в stress або dormancy.

---

# Genome Distribution During Division

Genome або його копії повинні потрапити в дочірні клітини.

Можливі варіанти:

* обидві дочірні клітини отримують копію Genome;
* одна отримує повний Genome, інша неповний;
* Genome пошкоджується під час копіювання;
* частина Genome дублюється;
* частина Genome втрачається;
* mobile fragments розподіляються нерівномірно.

Для базової моделі можна почати з копіювання одного Genome з шансом мутації.

---

# Epigenetic Inheritance

Epigenetic State може частково передаватися дочірнім клітинам.

Він може впливати на:

* стартову спеціалізацію;
* stress level;
* dormancy;
* division readiness;
* material synthesis priorities;
* реакцію на середовище.

Epigenetic inheritance не є зміною Genome.

---

# Failed Division

Поділ може провалитися.

Причини:

* недостатньо Energy;
* недостатньо Boundary materials;
* Genome не скопійований;
* клітина пошкоджена;
* немає зовнішнього простору;
* надмірний Pressure;
* надмірний Heat;
* нестача free_capacity;
* помилка процесу.

Наслідки failed division:

* клітина повертається до stressed state;
* частина ресурсів втрачена;
* виникає Heat;
* Boundary пошкоджена;
* створюється нежиттєздатний фрагмент;
* клітина помирає.

Якщо Feasibility Check відхиляє саму division action до partition, дочірні клітини не створюються, стан не partition-иться, а можливий progress preparation обробляється за правилами `biology/process-progress.md`.

Якщо partition вже committed, результатом можуть бути слабкі, leaking, inert або dead daughters. Рушій не повинен автоматично ремонтувати їх до життєздатного стану.

---

# Death

Death — це стан, у якому клітина більше не може підтримувати мінімальну функціональну структуру.

Death не є командою `kill(cell)`.

Death виникає, коли клітина втратила здатність:

* утримувати Boundary;
* підтримувати критичні Materials;
* виробляти або використовувати Energy;
* ремонтувати пошкодження;
* зберігати Genome;
* підтримувати внутрішній склад;
* виводити або нейтралізувати критичне засмічення.

---

# Death Conditions

Клітина може вважатися мертвою, якщо виконуються одна або кілька умов:

* Boundary integrity нижча за мінімальну життєздатність;
* critical Materials нижчі за мінімум;
* Genome втрачений або повністю нефункціональний;
* Energy production неможлива і repair не підтримується;
* внутрішній об'єм повністю заблокований;
* Heat зруйнував критичні Materials;
* Pressure або collision зруйнували структуру;
* клітина не може виконати жоден критичний Maintenance process.

Точні пороги визначаються конфігурацією базової моделі.

---

# Death не повинна бути миттєвою

У більшості випадків клітина не повинна зникати одразу.

Вона переходить у `dead` або `decomposing`.

Її матеріали, ресурси і Genome fragments залишаються у світі.

Це важливо для:

* повернення матерії;
* живлення інших клітин;
* HGT;
* реакцій;
* слідів;
* екологічного циклу.

---

# Decomposition

Decomposition — це розпад мертвої клітини.

Під час decomposition:

* Boundary деградує;
* Resources виходять у середовище;
* Materials розпадаються;
* Genome fragments деградують або залишаються;
* Heat вирівнюється;
* Material fragments можуть бути поглинуті іншими клітинами;
* Resources повертаються в локальний світ.

```text
Dead Cell
    ↓
Materials + Resources + Genome Fragments
    ↓
Environment / Other Cells
```

---

# Decomposition Rate

Швидкість розпаду залежить від:

* stability Materials;
* Heat;
* Pressure;
* реакцій ресурсів;
* наявності інших клітин;
* середовища;
* складу Boundary;
* пошкодження до смерті.

Деякі мертві структури можуть розкладатися повільно і тимчасово впливати на середовище.

---

# Lifecycle і Joint

Якщо клітина має Joint, lifecycle впливає на сусідні клітини.

Приклади:

* dying cell може перестати підтримувати Joint;
* damaged cell може передавати Heat;
* decomposing cell може віддати ресурси;
* dividing cell може перебудувати Joint;
* dormant cell може зменшити resource transport;
* dead cell може стати фізичним вантажем або ресурсом.

Joint не робить клітину безсмертною.

Але багатоклітинна структура може підтримувати окрему клітину ресурсами або сигналами.

---

# Lifecycle і Organism

Організм не має окремого hardcoded lifecycle на цьому рівні.

Організм виникає як граф клітин.

Його “життя” є наслідком lifecycle окремих клітин і Joint між ними.

```text
Organism Lifecycle =
many Cell Lifecycles
+
Joint dynamics
+
resource sharing
+
development
```

Пізніше це буде описано в `biology/organism.md`.

---

# Lifecycle і Selection

Selection не оцінює клітину напряму.

Якщо lifecycle дозволяє клітині:

* вижити;
* рости;
* ділитися;
* передавати Genome;
* залишати життєздатних нащадків,

то така лінія продовжується.

Якщо ні — вона зникає.

---

# Мінімальний Lifecycle Для базової моделі

Для базової моделі достатньо таких станів:

```text
alive
stressed
dormant
division_preparing
dividing
dead
decomposing
```

Мінімальні переходи:

```text
alive -> stressed
stressed -> alive
stressed -> dormant
dormant -> alive
alive -> division_preparing
division_preparing -> dividing
dividing -> alive + alive
any living state -> dead
dead -> decomposing
decomposing -> removed / resources
```

---

# State Transitions

Переходи між станами повинні бути наслідком умов.

Приклад:

```text
alive -> stressed
if:
  Energy is low
  OR Boundary damaged
  OR Heat too high
  OR critical resources missing
```

```text
stressed -> dormant
if:
  Genome regulation chooses low activity
  AND minimum structure is still viable
```

```text
division_preparing -> dividing
if:
  division readiness is high
  AND Genome copied
  AND enough Boundary materials
  AND enough Energy
```

```text
any living state -> dead
if:
  minimum viable structure is lost
```

---

# Приклад 1. Нормальний ріст і поділ

```text
Tick 1-20:
  Cell absorbs Resource A passively.
  Boundary is stable.
  Energy production works.

Tick 21-40:
  Cell synthesizes Structural Material.
  Cell grows.
  Energy Buffer remains positive.

Tick 41-60:
  Genome copying starts.
  Boundary expansion starts.
  Division readiness increases.

Tick 61:
  Feasibility Check passes.
  Cell enters dividing state.

Tick 62:
  Parent Cell splits into Daughter A and Daughter B.

Result:
  two alive cells
  both receive Genome copy
  both receive Resources and Materials
  both have enough Boundary to survive
```

---

# Приклад 2. Занадто ранній поділ

```text
Cell has:
  enough size
  but low Boundary Material
  low Energy
  incomplete Genome copy

Genome output:
  divide_priority = high

Feasibility Check:
  division not fully possible

Result:
  division fails
  planned division action is rejected
  Boundary is damaged
  cell enters stressed state
```

Якщо підготовка вже пошкодила Boundary або Materials до Feasibility Check, ці пошкодження є окремими наслідками процесів підготовки, а не частковим виконанням division action.

---

# Приклад 3. Dormancy через нестачу ресурсів

```text
Environment:
  low Resource A
  low Energy production

Cell:
  Energy Buffer decreases
  Growth stops
  Repair becomes limited

Genome regulation:
  movement_priority = low
  synthesis_priority = low
  maintenance_priority = high
  dormancy_level = high

Result:
  cell enters dormant state
  Energy consumption decreases
  cell survives longer
```

Якщо ресурси повернуться:

```text
Resource A outside increases
Energy production resumes
Genome lowers dormancy_level
Cell returns to alive state
```

---

# Приклад 4. Смерть через пошкодження Boundary

```text
Collision occurs.

Physics:
  impact damages Boundary Material

Cell:
  boundary_integrity decreases
  internal Resources leak
  unwanted Resources enter
  Energy production decreases

Repair:
  not enough Energy
  not enough Resources

Result:
  Boundary continues degrading
  cell loses internal stability
  cell becomes dead
```

Після смерті:

```text
Boundary fragments remain
Materials degrade
Resources return to environment
Genome fragments may remain temporarily
```

---

# Приклад 5. Засмічення клітини

```text
Cell absorbs Resource X.

Resource X:
  low usefulness
  low reactivity
  high volume

Cell lacks:
  export mechanism
  degradation process
  useful reaction pathway

Result:
  Resource X accumulates
  free_capacity decreases
  synthesis slows
  repair slows
  division becomes impossible
```

Якщо засмічення критичне:

```text
free_capacity = 0
repair impossible
growth impossible
waste export impossible
cell becomes stressed
then dies from structural failure
```

Це не toxicity.

Це наслідок обмеженого об'єму.

---

# Приклад 6. Небезпечна реакція ресурсу

```text
Resource A enters cell.

Resource A reacts with Boundary Material.

Reaction:
  Resource A + Boundary Material
    -> Resource B + Material degradation + Heat

Result:
  Boundary weakens
  Heat increases
  permeability increases
  more Resource A enters
```

Можливий позитивний еволюційний варіант:

```text
Cell evolves Material X.

Material X converts Resource A into harmless Resource C.

Result:
  Resource A no longer destroys Boundary
  Resource C can be exported or used
```

---

# Приклад 7. Смерть від Heat

```text
Energy production is high.
Energy Buffer is full.
Excess Energy becomes local Heat.

Heat exceeds Material tolerance.

Effects:
  Energy materials degrade
  Boundary materials weaken
  Genome damage probability increases
  repair cost increases

Cell cannot compensate.

Result:
  critical Materials fall below viability threshold
  cell dies
```

Heat не вбиває напряму.

Heat руйнує матеріальну структуру.

---

# Приклад 8. Нерівномірний поділ

```text
Parent Cell prepares division.

During division:
  Daughter A receives:
    70% Resources
    60% Materials
    full Genome copy

  Daughter B receives:
    30% Resources
    40% Materials
    damaged Genome copy

Result:
  Daughter A survives and grows.
  Daughter B enters stressed state.
```

Подальший результат залежить від середовища:

```text
If resources are available:
  Daughter B may repair and survive.

If resources are scarce:
  Daughter B dies.
```

---

# Приклад 9. Мертва клітина як ресурс

```text
Cell dies.

Decomposition:
  Boundary Material -> Resource A
  Structural Material -> Resource B
  Genome Fragment remains temporarily

Neighbor Cell:
  absorbs Resource A
  uses Resource B for synthesis
  may absorb Genome Fragment if HGT mechanism exists

Result:
  dead cell becomes part of ecosystem cycle
```

Мертва клітина не є сміттям рушія.

Вона є матеріальним об'єктом світу.

---

# Приклад 10. Багатоклітинна підтримка слабкої клітини

```text
Cell A is damaged.
Cell B is connected via Joint.

Cell B exports Resource A through Joint.
Cell A uses Resource A for Boundary Repair.

Cell A:
  restores Boundary
  avoids death
```

Це не означає, що організм має глобальний HP.

Це лише локальна передача ресурсів між клітинами.

---

# Правила

## Rule 1. Lifecycle is state-based

Lifecycle визначається станом клітини, а не сценарієм.

## Rule 2. Life requires maintenance

Клітина жива, поки може підтримувати мінімальну функціональну структуру.

## Rule 3. Growth is material

Growth означає збільшення матеріальної структури.

## Rule 4. Division is physical

Division є фізичним розділенням клітини і її внутрішнього стану.

## Rule 5. Death is structural failure

Death виникає через втрату мінімальної життєздатності.

## Rule 6. Dead cells remain physical

Мертві клітини не зникають миттєво.

## Rule 7. No HP

Lifecycle не використовує HP.

## Rule 8. Errors are allowed

Невдалий поділ, пошкоджена спадковість і нежиттєздатні клітини дозволені.

## Rule 9. Selection filters outcomes

Рушій не повинен забороняти погані lifecycle-рішення.

Selection відфільтровує їх через виживання.

## Rule 10. Lifecycle must scale

Lifecycle повинен працювати для однієї клітини, колонії та багатоклітинного організму.

---

# Заборонено

Не вводити:

* HP;
* instant death без матеріальної причини;
* disappearance on death;
* guaranteed equal division;
* guaranteed viable offspring;
* hardcoded reproduction mode;
* age timer як причина смерті;
* species-based lifecycle;
* plant/animal-specific lifecycle;
* global organism lifecycle на рівні Cell.

---

# Пов'язані документи

* `biology/cell.md`
* `biology/cell-state.md`
* `biology/division-partition.md`
* `biology/feasibility.md`
* `biology/process-progress.md`
* `biology/membrane.md`
* `biology/processes.md`
* `biology/genome.md`
* `biology/joint.md`
* `biology/development.md`
* `biology/heredity.md`
* `world/resources.md`
* `world/materials.md`
* `world/energy.md`
* `world/physics.md`
* `world/tick.md`

# Open Questions

## Division model

Потрібно визначити стартові коефіцієнти та noise для реалізації `biology/division-partition.md`:

* `resource_split_noise`;
* `material_split_noise`;
* `energy_split_rule`;
* `epigenetic_inheritance_factor`;
* правило external Joint reassign/break.

## Dormancy

Потрібно визначити, наскільки dormancy зменшує:

* Energy consumption;
* material degradation;
* transport;
* synthesis.

## Decomposition

Потрібно визначити, як швидко мертва клітина розкладається і які продукти залишає.

## Epigenetic inheritance

Потрібно визначити конкретний `epigenetic_inheritance_factor` для першої реалізації.

## Genome fragments after death

Потрібно визначити, чи Genome fragments можуть існувати в середовищі у базовій моделі, чи це буде відкладено до HGT-моделі.

## Lifecycle metrics

Потрібно визначити, які технічні метрики зберігати:

* age_ticks;
* divisions_count;
* damage_history;
* stress_level;
* dormancy_ticks;
* parent_id;
* lineage_id.

Ці метрики не повинні напряму керувати поведінкою, але корисні для аналізу.


