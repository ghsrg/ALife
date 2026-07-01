---
tags:
  - alife
  - worklog/plan
---

# PLAN: physics-logic-repeat-audit

Дата: 2026-06-30 14:05

Мета: повторний аудит після великої чистки документації. Перевірено логічну достатність Canon, стики між world/biology/genetics/engine/config та відповідність базовим фізичним і хімічним обмеженням.

Результат: нижче лише реальні проблеми або суттєві сірі місця. Дрібні стилістичні питання, Obsidian links і навігаційні backlinks не включені.

---

# P0. Energy займає об'єм у `PRINCIPLES.md`, але не займає об'єм у поточному Canon

## Проблема

`docs/PRINCIPLES.md` описує Energy як таку, що "займає внутрішній об'єм".

Поточні уточнені документи кажуть інше:

- `docs/world/energy.md`: Energy Buffer є локальним станом, не Resource і не Material.
- `docs/biology/cell.md`: Energy Buffer не займає об'єм напряму; capacity визначається Materials.
- `docs/world/units.md`: Stored Energy не додає окремий material volume, якщо майбутнє правило явно не вводить таку модель.

## Вплив

Це пряма суперечність між найвищим рівнем правил і нижчим Canon.

Якщо залишити як є, реалізація Energy capacity буде неоднозначною:

- або Energy стане квазі-речовиною з volume;
- або Energy буде станом storage Materials;
- або різні модулі реалізують це по-різному.

## Пропозиція

Оновити `docs/PRINCIPLES.md`:

- прибрати твердження, що Energy сама займає внутрішній об'єм;
- зафіксувати: Energy Buffer не є речовиною, але його capacity задається Materials, які займають volume;
- додати примітку: якщо у майбутньому потрібна модель energy-rich substance, це має бути Resource або Material, а не Energy Buffer.

---

# P0. Genome copy фізичний, але джерело матерії для копії не описане достатньо

## Проблема

`docs/biology/division-partition.md` каже:

```text
Genome | copied, not partitioned | copy cost paid before completion
```

При цьому `PRINCIPLES.md`, `world/laws.md`, `biology/genome.md` і `genetics/genome-representation.md` фіксують, що Genome має фізичний carrier/cost.

Зараз "copy cost" читається переважно як Energy/process cost, але не описує матеріальне джерело для другої фізичної копії Genome.

## Вплив

Це ризик порушення закону збереження матерії.

Якщо Genome фізичний, його копія не може з'явитися тільки за Energy cost. Потрібні Resources/Materials/internal fragments, які перетворюються на Genome carrier.

## Пропозиція

Уточнити `docs/biology/division-partition.md`, `docs/genetics/inheritance.md`, `docs/biology/genome.md`:

- genome copying є long-running/controlled process;
- до partition має існувати фізична копія або явно неповний fragment;
- копія Genome створюється з Resources/Materials згідно з recipe/cost;
- Energy є лише вартістю роботи, не джерелом матерії;
- якщо copy incomplete, daughter може отримати missing/damaged/nonfunctional Genome state.

---

# P0. Tick causality суперечить видимості Environment Update у тому самому Tick

## Проблема

`docs/world/laws.md` формулює:

```text
Зміни, що виникли в Tick N, можуть впливати лише на Tick N+1 або пізніше.
```

`docs/world/tick.md` описує:

```text
Environment Update -> Cell Decision -> Action Execution
```

і каже, що після Environment Update формується стан середовища, який бачать клітини в цьому Tick.

`docs/world/tick-semantics.md` ближчий до правильного контракту через phase commits, але `tick.md` і `laws.md` можна прочитати як суперечливі.

## Вплив

Це критично для реалізації Scheduler:

- чи passive diffusion/reactions Tick N видимі Decision Tick N;
- чи всі зміни Tick N видимі тільки Tick N+1;
- чи Environment Phase є підготовкою snapshot для поточного Tick або частиною переходу з попереднього Tick.

Без уточнення можуть з'явитися same-tick race conditions або різні трактування причинності.

## Пропозиція

Синхронізувати `docs/world/laws.md`, `docs/world/tick.md`, `docs/world/tick-semantics.md`, `docs/engine/scheduler.md`.

Вибрати й явно описати одну модель:

```text
Tick N starts from committed state N.
Environment Phase computes environment snapshot N.
Decision Phase reads environment snapshot N.
Actions committed produce state N+1.
Cell decisions cannot read other cells' decisions/actions from Tick N.
```

Або альтернативно:

```text
All Environment updates from Tick N become visible only in Tick N+1.
```

Перший варіант практичніший, але треба пом'якшити закон причинності: заборонити same-phase feedback, а не всі same-tick phase commits.

---

# P0. Mandatory costs і Feasibility мають різний порядок у документах

## Проблема

`docs/genetics/genome-runtime.md` каже:

```text
planned actions потребують більше Energy, ніж доступно після mandatory costs
```

`docs/engine/scheduler.md` мінімальні фази:

```text
6. Feasibility Check.
7. Mandatory costs.
8. Action execution.
```

`docs/world/tick.md` також описує Feasibility перед виконанням, але mandatory costs як окреме правило.

## Вплив

Якщо Feasibility рахує Energy до mandatory costs, planned actions можуть бути allowed, а потім стати неможливими після списання maintenance/runtime cost.

Це створює order-dependent behavior або потребу в прихованому retry.

## Пропозиція

Уточнити порядок:

```text
1. Compute mandatory costs.
2. Apply/pay mandatory costs або mark cell stalled/inert/degrading.
3. Compute post-mandatory available Energy.
4. Run Feasibility for planned action set against post-mandatory state.
5. If action set affordable as a whole, execute; otherwise reject planned actions.
```

Оновити `docs/engine/scheduler.md`, `docs/biology/feasibility.md`, `docs/world/energy.md`, `docs/world/tick.md`.

---

# P1. Reaction accounting не має достатнього contract для збереження матерії

## Проблема

`docs/world/reactions.md` і `docs/config/reactions_config.md` кажуть, що mass/resource accounting має бути явним.

Але validation зараз перевіряє переважно:

```text
non-negative stoichiometry
known outputs
energy_output >= 0
heat_output >= 0
rate >= 0
```

Не описано:

- чи input/output amount має бути збалансований;
- як задається configured loss;
- чи `energy_output` є вивільненням потенціалу input Resource, а не перетворенням матерії в Energy;
- як не допустити "зникнення" частини Resources.

## Вплив

У хімії спрощення допустиме, але в поточній документації це стикається із законом збереження матерії.

Без reaction accounting реалізація може непомітно видаляти Resources або створювати products без source.

## Пропозиція

Додати в `docs/world/reactions.md` і `docs/config/reactions_config.md` короткий `Reaction Accounting Contract`:

- Resources/Materials мають amount units;
- reaction має явно описувати products, retained/internalized material або residual/waste;
- configured loss дозволений тільки як явно змодельований outflow/degradation sink, не як приховане зникнення;
- `energy_output` походить з energy potential inputs, але не замінює material output;
- validation має мати хоча б warning для незбалансованих reactions.

---

# P1. Heat/temperature має локальну модель, але не має мінімального energy/transfer accounting

## Проблема

Документи правильно розділили:

- Energy Buffer;
- Heat;
- cell `temperature`;
- optional HeatField.

Але не визначено мінімальні поля й правила:

- heat capacity;
- heat transfer rate through contact/Joint;
- dissipation/sink, якщо global environment HeatField не моделюється;
- чи Heat conserved, clamped або abstracted;
- як reaction heat_output змінює `temperature`.

## Вплив

Це не груба суперечність фізиці, але без цього Heat може стати або магічним damage shortcut, або нескінченним накопичувачем/зникачем енергії.

Також важко буде створити stability bounds для перегріву.

## Пропозиція

Додати мінімальний `Heat/Temperature Contract` у `docs/world/physics.md` або окремий `docs/world/heat.md`:

```text
cell.temperature
heat_capacity
heat_generated
heat_transfer_to_neighbor/contact/joint
heat_dissipation_rule
material_tolerance thresholds
```

Зафіксувати спрощення:

- на першому етапі Heat не є повним thermodynamics module;
- локальний sink/dissipation дозволений як явна модель;
- Heat damage працює тільки через Material degradation thresholds.

---

# P1. 2D Space використовує radius/volume/capacity без зв'язку між геометрією і місткістю

## Проблема

Документи одночасно використовують:

- 2D world;
- `cell_radius_min/max`;
- `volume_capacity`;
- `volume`, `capacity`, `mass`;
- `shape/radius`.

Але не описано, чи `volume` у 2D є:

- реальною площею;
- абстрактною capacity unit;
- функцією radius;
- незалежним параметром клітини.

## Вплив

Це не суперечить фізиці, якщо це simulation abstraction, але без явного правила реалізація collision/space/capacity/growth може розійтися.

Наприклад, клітина може мати малий radius і великий capacity без пояснення.

## Пропозиція

Уточнити `docs/world/space.md`, `docs/world/units.md`, `docs/config/world_config.md`, `docs/biology/cell.md`:

- `volume_capacity` у 2D є internal capacity unit, а не SI volume;
- для першої моделі або визначити зв'язок `radius -> capacity`, або явно дозволити незалежний capacity with validation bounds;
- якщо radius росте від Materials, описати мінімальну формулу/placeholder.

---

# P1. `tick.md` після чистки все ще дублює й місцями відстає від `tick-semantics.md`

## Проблема

`docs/world/tick.md` детально описує фази Action Execution:

- active transport;
- biochemistry;
- mechanics;
- lifecycle;
- learning-like state;
- cleanup.

Водночас `docs/world/tick-semantics.md` і `docs/engine/scheduler.md` вже є точнішими contract-документами.

Деякі фрази в `tick.md` звучать застаріло або змішують домени:

- "Усі дії виконуються в однаковому порядку";
- внутрішньоклітинна хімія згадується як дані для Decision;
- HGT/recombination стоять у lifecycle phase, хоча genetics каже future-compatible.

## Вплив

Після наступної реалізації розробник може взяти `tick.md` як literal scheduler і отримати конфлікт із новішим `tick-semantics.md`.

## Пропозиція

Стиснути `docs/world/tick.md` так само, як biology/genetics:

- залишити conceptual Tick;
- винести semantic visibility у `tick-semantics.md`;
- scheduler implementation лишити в `engine/scheduler.md`;
- прибрати literal phase list або позначити як conceptual, not implementation order;
- HGT/recombination не ставити в базовий phase list.

---

# P1. Process/action реєстр розмазаний між кількома файлами

## Проблема

Allowed process/output ids зараз присутні в:

- `docs/biology/processes.md`;
- `docs/biology/process-progress.md`;
- `docs/biology/process-capabilities.md`;
- `docs/genetics/regulatory-network.md`;
- `docs/genetics/regulatory-interface.md`;
- `docs/biology/feasibility.md`.

Є локальні розбіжності:

- `movement` у `process-progress.md` atomic, але в `processes.md` future-compatible;
- `joint_creation_priority` є output binding, але Joint може бути future-level;
- `signal_output` є binding, але signal typing/state layers ще open.

## Вплив

Без єдиного реєстру перша реалізація не матиме однозначного списку:

- що Genome може output-ити;
- які actions Feasibility приймає;
- які processes atomic/long-running;
- які capabilities потрібні.

## Пропозиція

Створити або доповнити один Canon-документ:

```text
docs/biology/action-process-registry.md
```

або розширити `docs/biology/processes.md` таблицею:

```text
process_id
kind: passive | active | controlled reaction
implementation_level: current | future-compatible
atomic_or_long_running
required_capabilities
allowed_genome_output
feasibility_scope
```

Після цього інші файли мають посилатися на реєстр, а не тримати власні списки.

---

# P1. MaterialFragment / external Material не має чіткої межі з Resource

## Проблема

`docs/world/materials.md` каже, що Material є основою клітин, Boundary і Joint.

`docs/engine/ecs.md` має `MaterialFragment`.

`docs/biology/lifecycle.md` каже, що decomposition повертає Materials, Resources, Heat і Genome fragments у світ.

Але `docs/world/resources.md` визначає Resource як рухому речовину, а Material як внутрішню/структурну функціональну речовину.

Не описано, коли зовнішній Material fragment:

- лишається Material;
- стає Resource;
- може бути поглинутий;
- займає Space/capacity;
- зберігає capabilities.

## Вплив

Це може зламати decomposition, conservation і re-use dead remains.

Також є ризик, що Material capabilities працюватимуть поза клітиною без Boundary/Cell context.

## Пропозиція

Уточнити в `docs/world/materials.md`, `docs/world/resources.md`, `docs/biology/lifecycle.md`:

- `MaterialFragment` є physical remains with material identity;
- поза клітиною MaterialFragment не виконує cell process capabilities без cell context;
- decomposition може конвертувати MaterialFragment у Resources за reaction/degradation rules;
- uptake MaterialFragment або його breakdown має бути explicit process/reaction.

---

# P2. Organism як "зв'язний граф" у принципах жорсткіший за observer-side Canon

## Проблема

`docs/PRINCIPLES.md` каже:

```text
Організм визначається як зв'язний граф клітин.
Якщо граф розпався — можуть утворитися кілька незалежних організмів.
```

`docs/biology/organism.md` після чистки точніше каже:

- connected component є кандидатом;
- organism-like status залежить від dependency/integration;
- OrganismView є observer-side.

## Вплив

Це не ламає фізику, але може повернути старе трактування, де будь-який connected graph автоматично organism.

Також мінімальна модель у `organism.md` включає `alive/decomposing Cells`, що корисно для collapse tracking, але може плутати organism detection.

## Пропозиція

Оновити `docs/PRINCIPLES.md`:

- "Organism is observer-side organism-like graph view";
- connected component is candidate, not automatic organism;
- decomposing cells можуть входити в collapse/remains tracking, але не роблять структуру living organism-like.

---

# P2. `GLOSSARY.md` і `ROADMAP.md` відстають від нового стану після чистки

## Проблема

`docs/ROADMAP.md` досі має:

```text
GLOSSARY.md | Needs synchronization
STYLE_GUIDE.md | Needs synchronization
```

`GLOSSARY.md` у цілому корисний, але після чистки потрібно звірити:

- Energy/capacity формулювання;
- MaterialFragment;
- Heat/temperature;
- Process registry terms;
- OrganismView / Observer Layer;
- Seed Config / Stability Bound.

## Вплив

Це не Canon-фізика, але словник є required reading. Якщо словник відстає, майбутня реалізація може взяти неправильне визначення як source of truth.

## Пропозиція

Після виправлення P0/P1 провести коротку синхронізацію:

- `docs/GLOSSARY.md`;
- `docs/ROADMAP.md`;
- за потреби `README.md` / `docs/README.md`.

Не додавати нові довгі пояснення; тільки привести терміни й статуси.

---

# P2. Stability bounds перелічують категорії, але не мають мінімальних invalid/warning правил для фізично небезпечних значень

## Проблема

`docs/config/stability_bounds.md` правильно каже, що точні значення експериментальні.

Але для очевидно небезпечних параметрів не вказано мінімальні hard invalid rules:

- negative amount/rate вже частково в config;
- diffusion/decay > 1 per Tick;
- reaction rate too high for dt;
- heat transfer causing unbounded oscillation;
- mutation rate above bounded graph validation;
- density/cell radius impossible for world size.

## Вплив

Це не суперечність, але без hard invalid/warning criteria буде важко робити seed configs для граничних можливостей.

## Пропозиція

Після P0/P1 додати в `stability_bounds.md` секцію:

```text
Hard invalid examples
Warning ranges
Scenario-specific experimental ranges
```

Не фіксувати фінальні числа там, де потрібна симуляційна калібровка.

---

# Не Виявлено Як Проблему

## `toxicity`

Окрема `toxicity` механіка не повернулась. Поточна модель через reactions/volume/Heat/Material degradation узгоджена.

## `species_id`

Hardcoded species behavior не знайдено. Згадки в evolution/research є аналітичними або заборонними.

## Neural-like behavior

Після чистки залишено `signal-plastic` / `neural-like` тільки як material/runtime state, без hardcoded `NeuronCell`. Це узгоджено.

## Space як Resource

Space більше не є ResourceType. Згадки узгоджені.

## ADR

Нових реальних ADR-причин під час цього аудиту не виявлено. P0/P1 виглядають як синхронізація Canon і уточнення contract, а не нові архітектурні розвилки.

---

# Рекомендований Порядок Виправлення

1. P0 Energy/capacity у `PRINCIPLES.md`.
2. P0 Genome copying material accounting.
3. P0 Tick causality + mandatory costs ordering.
4. P1 Reaction accounting + Heat/temperature contract.
5. P1 Geometry/capacity у 2D.
6. P1 Process/action registry.
7. P1 MaterialFragment vs Resource.
8. P2 Organism wording у `PRINCIPLES.md`.
9. P2 GLOSSARY/ROADMAP sync.
10. P2 Stability bounds hard invalid/warning rules.
