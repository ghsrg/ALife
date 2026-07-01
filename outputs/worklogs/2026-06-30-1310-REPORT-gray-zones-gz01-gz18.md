---
tags:
  - alife
  - worklog/report
---

# REPORT: Gray Zones GZ-01..GZ-18

Дата: 2026-06-30 13:10

## Scope

Об'єднано плани:

- `outputs/worklogs/2026-06-30-0837-PLAN-gray-zones-gz01-gz04.md`
- `outputs/worklogs/2026-06-30-1000-PLAN-gray-zones-gz05-gz08.md`
- `outputs/worklogs/2026-06-30-1125-PLAN-gray-zones-gz09-gz12.md`
- `outputs/worklogs/2026-06-30-1248-PLAN-gray-zones-gz13-gz18.md`

Мета: внести правки по всіх 18 сірих зонах в один смисловий контекст без створення MVP-плану в Canon-документації.

## Додані contract-документи

### World

- `docs/world/units.md` — умовні одиниці, normalized values, per-Tick rates, стартові діапазони для стабільних конфігів.
- `docs/world/tick-semantics.md` — stable snapshot, delta, commit, same-tick visibility, deterministic conflict resolution.
- `docs/world/reactions.md` — passive/controlled reactions, локальність, відсутність `toxicity` shortcut і direct Energy magic.
- `docs/world/field-semantics.md` — Field як локально sampled input/effect, не direct behavior command.

### Biology

- `docs/biology/feasibility.md` — єдиний read-only Feasibility Check для active planned actions.
- `docs/biology/process-progress.md` — ProcessProgress як стан довгих процесів без часткового фінального результату.
- `docs/biology/cell-state.md` — функціональні стани клітини без HP/hidden viability score.
- `docs/biology/division-partition.md` — поділ як partition локального стану parent cell.
- `docs/biology/process-capabilities.md` — `Materials -> Capabilities -> allowed process set`.

### Genetics

- `docs/genetics/regulatory-interface.md` — межа Genome Runtime, ActionPlan і Feasibility.

### Config

- `docs/config/reactions_config.md` — формат reaction config без hardcoded chemistry/toxicity/direct kill/direct Energy.
- `docs/config/stability_bounds.md` — calibration stages, seed configs, validation levels і stability bounds.

## Основні внесені рішення

- Direct Regulatory Graph зафіксовано як поточний базовий напрямок Genome, але не як закритий моноліт. Дозволені майбутні корекції bindings, limits, mutation parameters і fragment-compatible шляху.
- Canon-документація не описує MVP-фази. Залишені лише вимоги та реалізаційні контракти; згадки "перша симуляція" замінено на "перша реалізація".
- Feasibility Check став обов'язковою межею: rejected action не має cost, output, partial final product або ProcessProgress increase.
- Energy conflict rule закріплено як conservative baseline: mandatory costs обробляються окремо; якщо Energy не вистачає на весь набір planned actions, planned actions не виконуються в цьому Tick.
- Heat розведено на `temperature` клітини і Heat transfer. Поточне правило: передача через контакт/Joint; global Heat Field є future extension.
- Scheduler уточнений як engine optimization, що має зберігати Tick semantics, але не мусить мати ті самі фази один-в-один.
- Space уточнений як окремий contract локальності, координат, boundary і spatial representation, не як Physics.
- Organism зафіксовано як observer/analytics view над Cells + Joints, а не active world-controller.
- Joint уточнено як один object з каналами `mechanical`, `resource`, `signal`, `heat`; Joint не дублюється при division.
- Signal baseline: scalar local signal with carrier/decay/direction; typed/delayed/frequency models лишаються future extensions.
- HGT не входить у першу реалізацію, але Genome/fragment model не повинна блокувати future HGT.
- RuntimeState, MaterialState, EpigeneticState і ProcessProgress розмежовано як різні state layers.

## Оновлені існуючі документи

- `docs/README.md` — додано нові документи у структуру.
- `docs/GLOSSARY.md` — додано ключові терміни: Snapshot, Delta, Commit, Feasibility Check, ProcessProgress, Capability, Living Function Continuity, Division Partition, Regulatory Intent, Runtime State, Material State, Genetic Fragment, Observer Layer, Stability Bound, Seed Config.
- `docs/world/tick.md` — додано посилання на `tick-semantics.md`, уточнено scope Open Questions.
- `docs/engine/scheduler.md` — прив'язано до `tick-semantics.md`, прибрано закриті same-tick питання.
- `docs/biology/processes.md` — додано contract references на Feasibility, ProcessProgress, Capabilities, Reactions.
- `docs/biology/lifecycle.md` — додано `cell-state.md` і `division-partition.md`, уточнено failed division.
- `docs/biology/cell.md` — додано cell-state, Feasibility і division partition references; "neural-like" подано як сигнально-пластичний стан, не hardcoded NeuronCell.
- `docs/biology/genome.md` — уточнено Direct Regulatory Graph і Regulatory Interface.
- `docs/genetics/genome-runtime.md` — додано Regulatory Interface, output/runtime constraints і activation baseline.
- `docs/biology/joint.md` — уточнено Joint channels і division behavior.
- `docs/world/space.md` — додано Locality Contract і уточнено resource/field/boundary/zones decisions.
- `docs/world/fields.md` — додано Field Semantics і поточне Heat rule.
- `docs/world/materials.md` — додано Capability reference.
- `docs/config/world_config.md`, `fields_config.md`, `resources_config.md`, `materials_config.md` — синхронізовано з reactions, units, stability bounds і capability matrix.
- `docs/engine/chemistry.md`, `docs/engine/ecs.md` — додано reactions config, observer read-only boundary і Joint channels.
- `docs/biology/communication.md`, `docs/genetics/inheritance.md`, `docs/genetics/horizontal-transfer.md`, `docs/genetics/epigenetics.md` — уточнено signal baseline, inheritance partition, HGT future boundary і state layer boundaries.
- `docs/genetics/regulatory-network.md` — замінено "перша симуляція" на "перша реалізація".

## Що залишено відкритим

Ці питання залишені не як протиріччя вимог, а як реалізаційні параметри:

- конкретні числові ranges для першої config schema;
- конкретний conflict resolution algorithm: stable id, proportional allocation або seeded arbitration;
- DAG-only чи fixed-step recurrent runtime для першої реалізації;
- конкретний набір Material properties, що увійде в першу schema;
- grid/sparse grid/entity representation для Resources і Fields;
- default rule для external Joint during division: reassign closest daughter або break unless maintained;
- мінімальні debug traces для Runtime, Feasibility, Communication, Inheritance, HGT і OrganismView.

## Verification

- Перевірено, що всі 12 нових contract-документів створені.
- Перевірено, що в `docs/` і `README.md` не лишилось `MVP`, `перша симуляція`, `first simulation`, `першої симуляції`.
- Перевірено, що фраза `базова модель базової моделі` прибрана.
- Нові файли збережені в UTF-8; PowerShell без `-Encoding utf8` може показувати кирилицю як mojibake, але `Get-Content -Encoding utf8` читає коректно.
- Link-аудит показав існуюче змішання стилів посилань (`biology/...`, `world/...`, `docs/...`) у старій документації. Це не виправлялось у межах цього завдання, бо окремий Obsidian link-audit запланований пунктом 7.

