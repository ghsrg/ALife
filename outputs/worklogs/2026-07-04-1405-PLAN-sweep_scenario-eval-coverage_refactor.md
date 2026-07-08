# CRITICAL REQUIREMENT: Evolutionary Test Coverage

Balance Analyzer не повинен залишатися набором тестів для механік, створених у перших фазах.

Кожен новий підетап розробки додає:

```text
Material types
Capabilities
Processes
Actions
physical mechanisms
lifecycle mechanisms
interaction mechanisms
Genome regulation
Organism-level mechanisms
Environment mechanics
```

Система тестування повинна розширюватися разом із core та автоматично виявляти, що нова механіка ще не включена в аналіз.

## Main Rule

```text
No registered simulation mechanism without:
- activation scenario
- isolated mechanic test
- measurable benefit
- measurable cost
- relevant raw metrics
- integration coverage
- balance interpretation
```

Якщо в core додано новий Material, Capability, Process або Action, але analyzer його не бачить, build/test report повинен містити:

```text
UNTESTED_REGISTERED_MECHANISM
```

Нову фазу не можна вважати повністю покритою balance analysis, поки для її механік немає тестових сценаріїв та метрик.

---

# 0. Current Sweep Suite Stabilization

Before registry-driven coverage is implemented, the existing analytical sweeps must pass their scenario and accounting contracts.

## Required Scenario Mapping

```text
viability_threshold
  -> finite_resource_viability

passive_income_equilibrium
  -> passive_income_survival

upkeep_sensitivity
  -> steady_resource_flow

dormant_modifier
  -> dormancy_survival

transport_metabolism
  -> resource_abundance
```

Analytical sweeps without an explicit scenario must fail configuration validation.

`scenario = none` is allowed only for explicitly marked `smoke_regression` runs.

## Current Scenario Contracts

### Viability Threshold

```text
finite initial Resource
no regeneration
survival_ticks must respond to resource_density
all points may end in collapse
single-zone result is not automatically low-information
```

### Passive Income Equilibrium

```text
low initial Energy
little or no Resource
passive income is the main Energy source
must expose collapse, dormancy survival or active survival thresholds
```

### Upkeep Sensitivity

```text
steady weak Resource flow
must reach at least two meaningful regimes
preferred: stable, fragile/dormancy survival and collapse
```

### Dormant Modifier

```text
dormant_ticks > 0
dormant_fraction or survival_ticks must respond to the parameter
otherwise:
  SCENARIO_MECHANISM_NOT_ACTIVATED
  PARAMETER_HAS_NO_EFFECT
```

### Transport × Metabolism

```text
Resource exhaustion must not dominate all combinations
matrix must expose at least two throughput regimes
all-collapse result requires environment-dominance validation
```

## Conservation Gate

Before balance interpretation:

```text
Resource accounting has no unexplained mismatch
Energy accounting has no unexplained mismatch
all explicit sinks and losses are classified
configured tolerance is used only for numeric error
```

Do not increase tolerance to hide unaccounted state changes.

## Reporting Gate

The report must include:

```text
scenario mapping
mechanism activation status
zones reached
parameter effect
conservation result
warning codes
interpretation
recommended correction
```

## Stabilization Acceptance

```text
all analytical runs have explicit scenario_id
no unexplained conservation mismatch
dormant modifier activates dormancy
viability is evaluated by survival response, not zone count alone
upkeep produces multiple meaningful regimes
transport × metabolism is not dominated by finite Resource exhaustion
three-seed sensitivity run completes
```

---

# 1. Registry-Driven Coverage

Analyzer повинен отримувати або формувати перелік із реєстрів core:

```text
MaterialType registry
MaterialCapability registry
ProcessRegistry
Action/result types
Environment mechanism registry
Lifecycle mechanism registry
Interaction mechanism registry
```

Не підтримувати повний перелік лише вручну всередині analyzer.

Для кожного елемента зберігати:

```text
id
category
introduced_in_phase
enabled
required_capabilities
required_inputs
consumed_resources
consumed_energy
consumed_materials
produced_outputs
state_changes
activation_conditions
```

## Coverage Manifest

Створити машинозчитуваний manifest:

```text
outputs/raw_data/mechanism_coverage.csv
outputs/reports/mechanism-coverage-<timestamp>.json
outputs/reports/mechanism-coverage-<timestamp>.md
```

Поля:

```text
mechanism_id
category
introduced_in_phase
registered
enabled
activation_scenario
isolated_test
integration_test
raw_metrics
cost_metrics
benefit_metrics
balance_sweep
status
warning_codes
```

Статуси:

```text
covered
partially_covered
registered_but_disabled
not_activated
missing_scenario
missing_metrics
missing_balance_test
```

---

# 2. Current Material Coverage

Перевірити поточну реалізацію щонайменше для таких матеріалів.

## Boundary Material

Призначення:

```text
утримання внутрішнього середовища
проникність
пасивний обмін
захист межі Cell
```

Потрібні метрики:

```text
passive uptake
passive loss
permeability rate
boundary damage
resource leakage
maintenance cost
```

## Transport Material

Призначення:

```text
активне поглинання та експорт Resources
```

Метрики:

```text
uptake throughput
uptake Energy cost
Resource capture share
failed uptake
transport bottleneck
```

## Metabolic Material

Призначення:

```text
перетворення Resources на Energy
```

Метрики:

```text
Resource metabolized
Energy produced
conversion efficiency
metabolism throughput
Heat/Waste production
metabolism bottleneck
```

## Storage Material

Призначення:

```text
збільшення Resource та Energy capacity
```

Метрики:

```text
capacity
utilization
overflow prevented
Resource retained during scarcity
material cost
upkeep cost
growth penalty
```

## Synthesis Material

Призначення:

```text
перетворення Resources і Energy на Materials
```

Метрики:

```text
Material synthesized by type
Resource cost
Energy cost
synthesis throughput
failed synthesis
synthesis bottleneck
```

## Structural Material

Призначення:

```text
фізична структура
growth
radius
міцність
division preparation
```

Метрики:

```text
radius growth
structural Material spent
Energy spent
growth speed
physical pressure
division readiness
```

## Repair Material

Призначення:

```text
відновлення damage і структурної цілісності
```

Метрики:

```text
damage repaired
Energy cost
Material cost
survival benefit
development opportunity cost
```

## Contractile Material

Призначення:

```text
скорочення
створення сили
active displacement
```

Метрики:

```text
active displacement
force generated
Energy cost
distance travelled
Resource reached
escape success
movement efficiency
```

## Sensory Material

Призначення:

```text
локальне сприйняття Resource gradients
pressure
damage
Environment signals
```

Метрики:

```text
stimuli detected
candidate actions generated
useful responses
false or ineffective responses
sensory Material cost
sensory upkeep
```

Sensory Material не повинно оцінюватися лише за фактом наявності сигналу. Треба оцінювати, чи сигнал створив корисну дію.

## Future Materials

Для нових MaterialType analyzer повинен автоматично вимагати:

```text
опис користі
опис вартості
activation scenario
isolated sweep
integration scenario
raw metrics
```

Не обмежувати систему наведеним списком.

---

# 3. Current Process And Action Coverage

Перевірити щонайменше такі процеси:

```text
MandatoryUpkeep
PassiveUptake
ActiveUptake
ResourceExport
Metabolism
MaterialSynthesis
Growth
Repair
Contraction
ReflexiveDisplacement
DivisionPreparation
Division
Death
Decomposition
PassiveContactExchange
ContactStimulus
Adhesion/Detach when implemented
```

Для кожного процесу перевіряти:

```text
registered
enabled
required Material capability exists
activation condition is reachable
ActionCandidate is generated
Feasibility accepts valid candidate
Feasibility rejects invalid candidate
execution changes only intended state
cost is charged
output is produced
conservation holds
event is emitted
```

## Action Coverage

Відстежувати всі зміни стану:

```text
Energy delta
Resource delta
Material delta
radius delta
damage delta
position/velocity/force delta
lifecycle transition
Cell birth
Cell death
decomposition release
contact exchange
```

Analyzer повинен виявляти керовані Cell зміни, які обходять pipeline:

```text
ActionCandidate
→ validate_feasibility
→ execution
→ accounting/event
```

Warning:

```text
DIRECT_STATE_MUTATION_OUTSIDE_PROCESS_PIPELINE
```

Пасивна фізика, collision correction і wall correction можуть залишатися physics pipeline, але також повинні мати окреме покриття.

---

# 4. Mechanic Test Contract

Для кожної механіки потрібні чотири рівні тестування.

## Level 1 — Reachability

```text
чи може механіка активуватися
```

Приклад:

```text
Contractility exists
+ Energy exists
+ pressure stimulus exists
→ Contraction candidate is generated
```

## Level 2 — Directional Effect

```text
чи зміна параметра змінює результат у правильному напрямку
```

Приклад:

```text
higher contractile force
→ greater active displacement
→ greater Energy cost
```

## Level 3 — Trade-Off

```text
чи користь має ціну
```

Приклад:

```text
more Storage
→ better scarcity survival
→ higher Material/upkeep cost or slower growth
```

## Level 4 — Integration

```text
чи механіка коректно працює разом з іншими механіками
```

Приклад:

```text
Sensory
+ Contractility
+ Movement
+ Uptake
+ Metabolism
+ Growth
```

Механіка вважається повністю покритою лише після проходження всіх релевантних рівнів.

---

# 5. Phase Increment Contract

Кожен підетап розробки повинен додавати до тестової системи:

```text
new mechanism registry entries
new activation scenarios
new raw metrics
new isolated sweeps
new integration scenarios
new balance expectations
new report explanations
```

При завершенні підетапу генерувати:

```text
phase_mechanism_delta.csv
phase_test_coverage_delta.csv
phase_balance_impact.md
```

## Phase Delta Report

Звіт повинен пояснювати:

```text
що додано у core
які нові стани стали досяжними
які нові витрати з’явилися
які нові переваги з’явилися
які старі сценарії могли змінитися
які конфіги варто повторно прогнати
чи виникла нова домінантна стратегія
```

---

# 6. Automatic Regression Selection

Після додавання механіки analyzer повинен визначати, які попередні сценарії треба перезапустити.

Приклади:

```text
Storage added
→ rerun scarcity, pulses, growth and dormancy scenarios

Contractility changed
→ rerun spatial patches, competition, collision and Energy balance

Repair added
→ rerun hazard, upkeep, growth and Material synthesis

Division changed
→ rerun abundance, competition, population growth and conservation

Sensory changed
→ rerun movement, gradient response and Resource patch scenarios
```

Створювати файл:

```text
outputs/reports/recommended-reruns-<timestamp>.md
```

---

# 7. Dynamic Config Recommendations

Analyzer повинен рекомендувати не лише зміну чисел, але й зміну механіки, якщо числове налаштування не вирішує проблему.

## Parameter Recommendation

Приклад:

```text
Transport dominates most scenarios.

Possible config changes:
- increase transport Energy cost
- increase Transport Material upkeep
- reduce maximum uptake
```

## Mechanic Recommendation

Приклад:

```text
Storage remains universally beneficial across the tested range.

Possible mechanic issue:
- Storage has capacity benefit but no mass/upkeep/growth penalty.
- Parameter tuning alone may only move the dominance boundary.
- Add explicit Storage Material synthesis and maintenance cost.
```

Рекомендації класифікувати:

```text
CONFIG_TUNING_RECOMMENDED
MECHANIC_TRADEOFF_MISSING
SCENARIO_COVERAGE_MISSING
METRIC_MISSING
IMPLEMENTATION_SUSPECTED
```

---

# 8. Automatic Config Candidate Generation

Analyzer повинен мати режим пропозиції нових конфігів без автоматичного оголошення їх правильними.

Створювати:

```text
outputs/recommended_configs/
```

Приклади:

```text
transport-cost-increase-01.toml
storage-upkeep-penalty-01.toml
dormancy-development-penalty-01.toml
repair-cost-adjustment-01.toml
```

До кожного config candidate додавати metadata:

```text
source report
detected imbalance
changed parameters
expected effect
scenarios to rerun
confidence
```

Analyzer не повинен автоматично перезаписувати основні конфіги.

Правильний цикл:

```text
detect imbalance
→ explain evidence
→ propose candidate config
→ rerun relevant scenarios
→ compare before/after
→ accept or reject candidate
```

---

# 9. Required Cross-Mechanic Matrices

Окрім одновимірних sweep-ів, підтримувати матриці взаємодії.

Поточні та найближчі:

```text
Transport × Metabolism
Storage × Upkeep
Storage × Growth
Synthesis × Growth
Synthesis × Repair
Contractility × Sensory
Contractility × Movement Energy cost
Repair × Hazard intensity
Dormancy cost × Resource pulse interval
Boundary permeability × Resource gradient
Growth allocation × Division threshold
```

Після появи нової механіки analyzer повинен запропонувати можливі матриці з уже існуючими механіками.

Не створювати всі комбінації автоматично: формувати candidate list за спільними Resources, Energy, Materials або state transitions.

---

# 10. Full-System Integration Scenarios

Після isolated sweeps обов’язково мати сценарії, де механіки працюють одночасно.

## Autonomous Cell Scenario

```text
Boundary
Transport
Metabolism
Storage
Synthesis
Structural Growth
Repair
Contractility
Sensory
Upkeep
Dormancy
Death
Decomposition
```

Перевірити:

```text
усі доступні процеси можуть активуватися
дорогі процеси конкурують за Energy/Resources
немає подвійного списання
немає безкоштовних Actions
немає process execution після death
різні локальні умови породжують різні process profiles
```

## Lifecycle Scenario

```text
uptake
→ metabolism
→ synthesis
→ growth
→ division
→ starvation/damage
→ death
→ decomposition
```

## Mixed Environment Scenario

```text
abundant zones
scarce zones
Resource pulses
spatial patches
hazard zones
multiple Cells
```

Мета:

```text
не вимагати конкретного переможця,
а перевірити досяжність різних режимів,
accounting, детермінізм і відсутність універсального перекосу
```

---

# 11. Expanded Raw Data

Raw data має бути process- і material-aware.

## Per Material

```text
initial amount
synthesized
consumed
spent on upkeep
spent on action
released
final amount
capacity contribution
benefit metric
cost metric
```

## Per Process

```text
candidate count
accepted count
rejected count by reason
execution count
Energy consumed
Resources consumed
Materials consumed
outputs produced
state changes
```

## Per Capability

```text
present
strength/value
candidate processes enabled
successful executions
benefit
cost
```

## Per Phase

```text
mechanisms introduced
coverage added
new warnings
new matrices
regressions detected
```

---

# 12. Updated Acceptance Criteria

Task is complete only when:

```text
all registered current Materials appear in mechanism coverage
all registered Capabilities appear in mechanism coverage
all enabled Processes have activation scenarios
all enabled Processes have cost and benefit metrics
all controlled Actions are tracked
new mechanisms produce coverage failures until tests are added
each development subphase extends analyzer coverage
isolated mechanic sweeps remain available
cross-mechanic matrices are supported
full-system integration scenarios exist
reports distinguish config imbalance from missing mechanic trade-off
reports propose candidate config changes
reports may recommend mechanic changes when tuning is insufficient
candidate configs are saved separately
relevant previous scenarios are recommended for rerun
```

## Core Principle

Balance testing is a growing part of the simulation architecture, not a one-time Phase 1/2 test suite.

The expected development loop is:

```text
implement new mechanic
→ register mechanic
→ analyzer detects missing coverage
→ add activation scenario and metrics
→ run isolated sweep
→ run cross-mechanic matrices
→ run full integration scenarios
→ detect imbalance
→ propose config or mechanic changes
→ rerun affected scenarios
→ preserve regression coverage
```

Before implementation, update the TDD plan so that registry-driven coverage and the phase increment contract are implemented before adding more manually listed strategy profiles.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
