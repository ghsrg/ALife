---
tags:
  - alife
  - worklog/report
---

# Report: docs inconsistency audit

## Verdict

База загалом валідна: головна архітектурна лінія послідовна — емерджентність, фізичність, відсутність hardcoded біологічних shortcut-механік, локальна Energy, Material-driven функції, Genome як регуляція, Selection як єдиний фільтр.

Але документація ще не готова до реалізації без уточнення. Основні проблеми не в ідеї, а в межах MVP, статусі Genome-рішень, semantics виконання дій, ADR-дисципліні та застарілих посиланнях.

Перевірено 53 файли в `docs/`. Знайдено 44 секції `Open Questions` і 75 згадок `ADR-000X`.

---

# P0 — блокує планування реалізації

## 1. MVP Genome одночасно "обраний" і "ще треба вибрати"

У `genetics/genome-representation.md` сказано:

- `Status: Research / Mostly Stable` — `docs/genetics/genome-representation.md:24`
- MVP-представлення: `Genome as Direct Regulatory Graph` — `docs/genetics/genome-representation.md:33`
- `Accepted for MVP` — `docs/genetics/genome-representation.md:275`
- `Direct Regulatory Graph — базова модель Genome для першої реалізації` — `docs/genetics/genome-representation.md:278`

Але `biology/genome.md` все ще має Open Question:

- `Потрібно вибрати MVP-представлення` — `docs/biology/genome.md:1173`

Проблема: майбутній агент не знатиме, чи Direct Regulatory Graph вже канонічне MVP-рішення, чи лише сильна рекомендація.

Уточнення: або прийняти `Direct Regulatory Graph` як Canon/MVP baseline і прибрати Open Question, або повернути його в Research і не називати `Accepted for MVP`.

## 2. Межа "перша симуляція" / "MVP" суперечить сама собі

`ROADMAP.md` каже:

- першою реалізацією має бути `World → Resources → Materials → Energy → Single Cell` — `docs/ROADMAP.md:312`
- лише після цього додаються `поділ`, `Joint`, `геном`, `мутації` — `docs/ROADMAP.md:326`
- але критерії готовності MVP включають, що клітина може `ділитися` — `docs/ROADMAP.md:350`

Додатково `Stage 4 — Genome` включає `Recombination` і `Horizontal Gene Transfer` — `docs/ROADMAP.md:132`, `docs/ROADMAP.md:142`, `docs/ROADMAP.md:143`, але окремі genetics-документи часто кажуть, що HGT/recombination можна відкласти з MVP.

Проблема: змішані поняття `перша симуляція`, `MVP`, `MVP1`, `Stage`, `post-MVP`.

Уточнення: розвести мінімум на три рівні:

- Smoke Simulation: світ + одна клітина без поділу.
- MVP1: клітина живе, росте, ділиться, помирає.
- MVP2+: Joint, Genome, mutations, inheritance, evolution.

## 3. Feasibility Check: дія скасовується чи виконується частково

Жорстка версія:

- `Без достатнього запасу енергії дія не виконується` — `docs/world/laws.md:102`
- якщо хоча б одна умова не виконується, дія скасовується — `docs/world/tick.md:169`

Гнучка версія:

- дія не виконується або виконується частково — `docs/world/energy.md:236`
- процес може виконуватися частково — `docs/biology/processes.md:207`
- якщо Energy недостатньо, процес проходить через failure або partial execution — `docs/biology/processes.md:531`
- Open Question: чи дія може виконатися частково — `docs/world/energy.md:545`

Проблема: це прямо впливає на Scheduler, ProcessExecution, Energy balance і стабільність світу.

Уточнення: затвердити одне правило:

- або MVP має тільки all-or-nothing actions;
- або кожен Process має `partial_allowed`, `min_energy`, `scaling_rule`, `failure_mode`.

## 4. ADR-маркери масово присутні, але прийнятих ADR немає

`docs/decisions/README.md` каже:

- прийнятих ADR ще немає — `docs/decisions/README.md:40`
- `ADR-000X` є маркерами потреби, не рішеннями — `docs/decisions/README.md:42`

Але документи містять багато блоків `Потрібні ADR` і конкретні назви `ADR-000X`, наприклад:

- Genome Representation — `docs/genetics/genome-representation.md:1066`
- Cell — `docs/biology/cell.md:1166`
- Organism — `docs/biology/organism.md:1535`
- Physics 2D/3D — `docs/world/physics.md:654`

Проблема: зараз документація виглядає так, ніби рішення вже відомі, але юридично для проєкту вони не прийняті.

Уточнення: або створити перший пакет ADR для справді прийнятих рішень, або замінити `ADR-000X` на `Proposed ADR:` / `Needs decision:` без номера.

---

# P1 — створює двозначність

## 5. `Space` названий Resource, хоча Resource визначений як рухома речовина

`GLOSSARY.md`:

- `Resource` — рухома речовина світу — `docs/GLOSSARY.md:103`, `docs/GLOSSARY.md:107`

`world/laws.md`:

- вільний простір є таким самим ресурсом, як будь-яка інша речовина — `docs/world/laws.md:116`

Проблема: простір не є речовиною, не дифундує і не синтезується як Resource. Це ламає термінологічну чистоту.

Уточнення: замінити на `Space is a limited capacity/constraint`, а не `Resource`. Якщо треба лишити аналогію, явно написати: "простір є обмеженням, подібним до ресурсу, але не ResourceType".

## 6. Термін "нейронні клітини" конфліктує з забороною нейронів

`PRINCIPLES.md` забороняє готове поняття нейрона:

- у рушії не існує нейрона — `docs/PRINCIPLES.md:23`, `docs/PRINCIPLES.md:29`

Але нижче там же:

- навчання може змінювати `параметри нейронних клітин` — `docs/PRINCIPLES.md:209`

Так само:

- `GLOSSARY.md` визначає Learning як зміну параметрів `нейронних клітин` — `docs/GLOSSARY.md:352`
- `world/tick.md` згадує `ваги нейронних клітин` — `docs/world/tick.md:235`

Інші документи вже використовують кращу форму: `neural-like Materials`, `Neural-like Cells`, без hardcoded `NeuronCell`.

Уточнення: замінити "нейронні клітини" на `neural-like cells/material states` або українське "сигнально-пластичні клітини".

## 7. Energy не передається, але Energy Buffer ділиться між дочірніми клітинами

Правило:

- Energy не передається між клітинами напряму — `docs/GLOSSARY.md:156`, `docs/world/energy.md:317`, `docs/world/energy.md:475`

Виняток/інший процес:

- Energy Buffer може розподілятися між дочірніми клітинами — `docs/biology/lifecycle.md:510`, `docs/genetics/inheritance.md:361`

Це можна зробити послідовним, але зараз не вистачає явного формулювання.

Уточнення: поділ клітини — це partition локального стану однієї клітини під час reproduction, а не transport між незалежними клітинами. Це треба додати в `world/energy.md` або `biology/lifecycle.md`.

## 8. Heat має три ролі: локальний стан, Field і канал передачі

Документи одночасно кажуть:

- Heat може бути локальним станом клітини — `docs/world/energy.md:538`
- у базовій моделі достатньо локального Heat і передачі через контакт/Joint — `docs/world/energy.md:309`
- `engine/ecs.md` має `HeatField` — `docs/engine/ecs.md:145`
- `engine/scheduler.md` має `HeatDiffusionSystem` — `docs/engine/scheduler.md:46`

Проблема: незрозуміло, чи Heat в MVP — cell-local scalar, world field layer, object state, чи всі три рівні.

Уточнення: затвердити MVP heat model:

- MVP1: local cell/object heat only;
- MVP2: contact/Joint transfer;
- MVP3: field/layer diffusion.

## 9. Tick model і Scheduler model не повністю збігаються

`world/tick.md` визначає незмінний порядок із трьох етапів:

- Environment Update → Cell Decision → Action Execution — `docs/world/tick.md:43`
- цей порядок є незмінним — `docs/world/tick.md:63`

`engine/scheduler.md` має п'ять фаз:

- Environment Update, Cell Decision, Action Execution, Physics and Cleanup, Statistics and Snapshots — `docs/engine/scheduler.md:20`

Також `world/tick.md` включає lifecycle/recombination/HGT у Action Execution, тоді як `engine/scheduler.md` переносить Physics, Lifecycle, Decomposition у окрему Phase 4.

Проблема: це не обов'язково протиріччя, але потрібна явна ієрархія: Scheduler фази 4-5 є підфазами Action Execution чи окремими етапами Tick?

Уточнення: зробити `world/tick.md` концептуальним Canon, а `engine/scheduler.md` — деталізацією, де Phase 4/5 описані як post-action commit/cleanup/analytics.

## 10. World-рівень містить забагато біологічних прикладів

Поточний принцип: світ не містить готових біологічних понять — `docs/PRINCIPLES.md:21`.

Але `world/` документи часто заходять у клітини, Genome, learning, поділ, HGT, організми. Наприклад:

- `world/tick.md` описує Cell Decision і Genome execution — `docs/world/tick.md:95`, `docs/world/tick.md:130`
- `world/laws.md` описує навчання — `docs/world/laws.md:196`
- `world/resources.md` каже, що клітини можуть "еволюційно навчитися" — `docs/world/resources.md:208`

Проблема: частина цього є корисними прикладами, але межа `World` vs `Biology` розмита.

Уточнення: у `world/` залишити універсальні правила і короткі приклади; детальну клітинну поведінку перенести або лінкувати в `biology/`.

## 11. `Organism` не є сутністю рушія, але є Open Question про engine-level entity

Прийняті принципи:

- Organism не є окремою сутністю рушія — `docs/PRINCIPLES.md:160`
- Organism не є окремим типом об'єкта рушія — `docs/GLOSSARY.md:200`

Engine-документ коректно вводить `OrganismView` як derived analytical view:

- `OrganismView` не є активним керуючим Entity — `docs/engine/ecs.md:59`

Але `biology/organism.md` лишає питання:

- чи `Organism` буде engine-level entity — `docs/biology/organism.md:1590`

Проблема: ця Open Question суперечить уже прийнятому верхньому принципу, якщо під `entity` мається на увазі поведінкова сутність.

Уточнення: переформулювати питання на "cached derived view чи on-demand analytical view", без варіанту активної engine-level сутності.

---

# P2 — навігація, чистота, підготовка до Obsidian

## 12. Є застарілі посилання на неіснуючі файли

Приклади:

- `biology/development.md` — `docs/biology/cell.md:1160`, `docs/biology/lifecycle.md:1192`, `docs/biology/organism.md:1516`
- `biology/heredity.md`, `biology/mutations.md`, `biology/learning.md` — `docs/biology/genome.md:1139`, `docs/biology/genome.md:1140`, `docs/biology/genome.md:1141`
- старі research-назви: `research/genome-representation.md`, `research/graph-crossover.md`, `research/plasmid-genome.md`, `research/reproduction-strategies.md`, `research/genome-runtime.md` — `docs/biology/genome.md:1148`-`1152`

Фактичні файли мають інші назви, наприклад:

- `docs/genetics/heredity.md`
- `docs/genetics/mutation.md`
- `docs/research/genome-representation-options.md`
- `docs/research/graph-recombination-options.md`
- `docs/research/mobile-genetic-elements.md`
- `docs/research/reproduction-strategy-options.md`

Уточнення: зробити окремий link-audit і привести всі посилання до Obsidian-сумісної схеми.

## 13. `Space` є в ROADMAP, але немає окремого `world/space.md`

`ROADMAP.md` включає `Space` у Stage 1 — `docs/ROADMAP.md:92`.

Фактично простір описаний частинами в:

- `world/physics.md`
- `config/world_config.md`
- `engine/physics.md`

Проблема: або потрібен `world/space.md`, або ROADMAP має називати це `Physics / Space model`.

## 14. `Status` живе не тільки в ROADMAP

Після синхронізації статуси прибрані з `docs/README.md`, але статуси ще є в окремих документах:

- `genetics/genome-representation.md` має `Status: Research / Mostly Stable` — `docs/genetics/genome-representation.md:24`
- Research-файли мають власний `# Статус`
- `STYLE_GUIDE.md` має таблицю статусів.

Це не обов'язково помилка. Але треба вирішити політику:

- або статус кожного документа дозволений у frontmatter/header;
- або єдиний статусний реєстр тільки в `ROADMAP.md`.

---

# Recommended Next Decisions

1. Затвердити словник рівнів: `Smoke Simulation`, `MVP1`, `MVP2`, `Stage`, `post-MVP`.
2. Прийняти або відкласти `Direct Regulatory Graph` як MVP Genome baseline.
3. Затвердити semantics Feasibility Check: all-or-nothing vs partial execution.
4. Вирішити Heat MVP model: local scalar vs field/layer.
5. Перетворити справді прийняті `ADR-000X` на реальні ADR або прибрати номери.
6. Провести окремий link-audit і Obsidian-link normalization.

---

# Suggested Order For Fixing

1. `ROADMAP.md` — розвести MVP-рівні й першу симуляцію.
2. `docs/genetics/genome-representation.md` + `docs/biology/genome.md` — синхронізувати статус Genome baseline.
3. `docs/world/tick.md`, `docs/world/energy.md`, `docs/biology/processes.md` — узгодити partial execution.
4. `docs/decisions/` — створити перші ADR тільки для реально прийнятих рішень.
5. Link cleanup по всіх `Пов'язані документи`.

---

# Verification

- Переглянуто інвентар `docs/`.
- Перевірено ключові файли: `PRINCIPLES`, `GLOSSARY`, `ROADMAP`, `world/*`, `biology/*`, `genetics/*`, `engine/*`, `research/*`, `decisions/README.md`.
- Перевірено патерни: `Open Questions`, `ADR-000X`, `MVP`, `Energy`, `Heat`, `Genome`, `Recombination`, `HGT`, `Neuron`, dead links.
- Canon-документи не змінювалися в межах цього аудиту.
