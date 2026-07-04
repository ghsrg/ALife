# TASK: Refactor Sweep Scenarios and Extend Balance Analysis

## Context

Поточний `sweep_analyzer.toml` використовує одну базову конфігурацію клітини та середовища для різних sweep-ів.

Це підходить для smoke/regression-перевірки:

```text
чи параметр впливає на результат
чи немає panic / NaN / overflow
чи прогін детермінований
```

Але різні механіки потребують різних умов активації:

```text
dormancy modifier неможливо оцінити, якщо Cell не входить у dormancy
transport × metabolism неможливо коректно оцінити, якщо весь Resource швидко вичерпується
upkeep sensitivity малоінформативний, якщо весь діапазон дає лише collapse
```

Тому перед повним аналізом `strategy × environment` потрібно переробити конфігурацію sweep-ів так, щоб кожен sweep запускався у власному сценарному контексті.

---

# 1. Balance Principle

Система повинна дозволяти різним стратегіям бути вигідними в різних умовах:

```text
кожна стратегія має власну екологічну нішу
кожна перевага має вимірювану ціну
жодна стратегія не повинна домінувати у всіх або більшості середовищ
```

Баланс не означає однаковий score у всіх сценаріях.

Правильний результат:

```text
різні стратегії виграють у різних умовах
```

Неправильний результат:

```text
одна стратегія виграє майже всюди
або всі стратегії штучно мають однаковий результат
```

## Dormancy

Dormancy повинна бути:

```text
тимчасовим механізмом переживання дефіциту
```

Dormancy не повинна бути найкращою довгостроковою стратегією.

Очікування:

```text
у тимчасовому дефіциті dormancy підвищує survival
у постійному достатку active growth/reproduction вигідніші
dormancy знижує розвиток, growth і reproduction
```

---

# 2. Stage 1 — Refactor Existing Sweep Analyzer

## Goal

Зберегти наявні механічні sweep-и, але дозволити кожному з них використовувати окремий базовий scenario preset.

Поточний підхід:

```text
one base Cell
+ one base Environment
+ changed parameter
```

Новий підхід:

```text
scenario preset
+ changed parameter
+ explicit expected metrics
```

## Required Config Structure

Додати підтримку сценаріїв:

```toml
[scenarios.finite_resource_viability]
# finite initial Resource, no regeneration

[scenarios.dormancy_survival]
# conditions force Cell into dormancy

[scenarios.steady_resource_flow]
# continuous weak Resource/energy input

[scenarios.resource_abundance]
# enough Resource to test throughput rather than exhaustion

[scenarios.resource_pulses]
# alternating abundance and scarcity
```

Кожен sweep або matrix повинен посилатися на сценарій:

```toml
[[sweeps]]
name = "viability_threshold"
scenario = "finite_resource_viability"

[[sweeps]]
name = "dormant_modifier"
scenario = "dormancy_survival"

[[sweeps]]
name = "upkeep_sensitivity"
scenario = "steady_resource_flow"

[[matrices]]
name = "transport_metabolism"
scenario = "resource_abundance"
```

Не hardcode сценарні значення в analyzer code.

---

# 3. Correct Scenario Assignment

## Viability Threshold

Призначення:

```text
оцінити, як початковий скінченний запас Resource впливає на survival time
```

Сценарій:

```text
finite Resource
no regeneration
fixed Cell configuration
```

Цей тест не зобов’язаний мати stable zone.

Допустимий результат:

```text
всі варіанти зрештою collapse,
але більший запас збільшує survival time
```

Основні метрики:

```text
survival_ticks
death_tick
Resource consumed
mean Energy
dormancy fraction
```

Не називати цей sweep тестом стійкої рівноваги.

## Passive Income Equilibrium

Призначення:

```text
перевірити, чи passive income може підтримати dormancy або activity
```

Сценарій:

```text
низька стартова Energy
мало або немає Resource
passive income є основним джерелом Energy
```

Основні метрики:

```text
survival
dormancy fraction
wake transitions
mean/final Energy
active fraction
```

Поточний sweep можна переважно зберегти.

Не потрібно зараз уточнювати точний поріг до малих десяткових значень.

## Dormant Modifier

Призначення:

```text
оцінити вартість підтримки Cell саме у dormancy
```

Сценарій повинен гарантовано створювати dormancy:

```text
initial Energy нижче active threshold
Resource відсутній або дуже обмежений
passive income близький до dormant upkeep
Cell проводить значну частину часу у dormancy
```

Обов’язковий acceptance:

```text
dormant_ticks > 0
dormant_fraction суттєво змінюється або змінюється survival time
```

Якщо `dormant_ticks == 0` для всіх рядків, sweep повинен отримати статус:

```text
INVALID_SCENARIO_NOT_ACTIVATED
```

а не створювати формальний balance result.

## Upkeep Sensitivity

Призначення:

```text
оцінити вплив mandatory upkeep на survival/activity
```

Сценарій:

```text
steady weak Resource flow або regeneration
```

Діапазон повинен містити кілька режимів:

```text
stable
fragile/dormancy survival
collapse
```

Якщо весь діапазон дає одну зону, analyzer повинен повідомляти:

```text
LOW_INFORMATION_SWEEP
```

а не автоматично вимагати дрібнішого кроку.

## Transport × Metabolism

Призначення:

```text
оцінити співвідношення uptake throughput і metabolism throughput
```

Сценарій:

```text
Resource abundance
або постійна regeneration
достатня тривалість
Resource exhaustion не повинно бути головною причиною collapse
```

Матриця повинна показувати режими:

```text
transport bottleneck
metabolism bottleneck
balanced throughput
storage overflow/pressure
Energy accumulation
insufficient Energy production
```

Якщо всі 100 комбінацій завершуються однаково через вичерпання світу, matrix повинна позначатися:

```text
ENVIRONMENT_DOMINATED_RESULT
```

---

# 4. Keep Existing Smoke Profile

Не видаляти поточний спільний базовий конфіг.

Залишити його як:

```text
smoke_regression
```

Він потрібен для перевірки:

```text
parameter is wired
run completes
result is deterministic
no invalid numeric state
CSV/report generation works
```

Але не використовувати його як єдину основу для висновків про баланс механік.

---

# 5. Required Metrics for Existing Sweeps

Розширити кожен raw CSV такими полями:

```text
scenario_id
scenario_version
config_hash
seed
ticks_requested
ticks_executed

parameter_name
parameter_value
secondary_parameter_name
secondary_parameter_value

zone
scenario_status
warning_codes

survived_to_end
survival_ticks
death_tick
death_reason

active_ticks
active_fraction
dormant_ticks
dormant_fraction
dormancy_enter_count
dormancy_exit_count

initial_energy
final_energy
min_energy
max_energy
mean_energy
energy_produced
passive_energy_received
energy_spent_upkeep
energy_spent_dormant_upkeep
energy_spent_movement
energy_spent_growth
energy_spent_repair
energy_spent_division

initial_world_resource
final_world_resource
resource_regenerated
resource_absorbed
resource_metabolized
internal_resource_final
resource_released
resource_explicit_sink

uptake_attempt_count
uptake_success_count
metabolism_attempt_count
metabolism_success_count
feasibility_rejections
```

Додавати лише доступні зараз процеси, але schema має бути розширюваною.

---

# 6. Conservation and Accounting

Додати перевірки Resource та Energy accounting.

## Resource

```text
initial world Resource
+ regenerated Resource
=
final world Resource
+ internal Resource
+ metabolized Resource
+ released Resource
+ explicit sinks
```

## Energy

```text
initial Energy
+ metabolism production
+ passive income
=
final Energy
+ upkeep
+ dormant upkeep
+ movement
+ growth
+ repair
+ division
+ explicit losses
```

Для кожного прогону зберігати:

```text
resource_balance_error
energy_balance_error
```

При перевищенні tolerance:

```text
CONSERVATION_MISMATCH
```

---

# 7. Desired Metrics in Scenario Config

Кожен сценарій повинен визначати не одне точне очікуване число, а допустимі режими та інформаційні критерії.

Приклад:

```toml
[scenarios.dormancy_survival.expected]
required_states = ["dormant"]
min_dormant_fraction = 0.20
require_parameter_effect = true

[scenarios.resource_abundance.expected]
forbidden_primary_death_reasons = ["resource_exhaustion"]
require_parameter_effect = true

[scenarios.steady_resource_flow.expected]
required_zone_count = 2
preferred_zone_count = 3
```

Додати:

```toml
[scenarios.<id>.warnings]
min_effect_size = 0.05
max_conservation_error = 0.000001
single_zone_warning = true
scenario_not_activated_warning = true
environment_dominance_warning = true
```

Значення мають бути config-driven.

---

# 8. Analyzer Warnings

Додати автоматичні попередження:

## Scenario Not Activated

```text
SCENARIO_MECHANISM_NOT_ACTIVATED
```

Приклади:

```text
dormant modifier tested, but dormant_ticks == 0
repair tested, but damage == 0
movement tested, but no gradient/contact stimulus exists
```

## Low Information Sweep

```text
LOW_INFORMATION_SWEEP
```

Умови:

```text
усі точки мають однакову zone
метрика майже не змінюється
parameter effect нижче min_effect_size
```

## Environment Dominated Result

```text
ENVIRONMENT_DOMINATED_RESULT
```

Приклад:

```text
transport × metabolism combinations all die because finite world Resource is exhausted
```

## Parameter Not Wired

```text
PARAMETER_HAS_NO_EFFECT
```

Умови:

```text
зміна параметра не змінює жодної релевантної метрики
```

## Expected Trade-off Missing

```text
EXPECTED_TRADEOFF_NOT_OBSERVED
```

---

# 9. Stage 2 — Strategy × Environment Balance Analysis

Після виправлення окремих sweep-ів додати окремий analyzer, не змішуючи його з механічними regression sweeps.

Рекомендована назва:

```text
balance_analyzer.toml
```

Запуск:

```text
strategy
× environment
× seed
```

## Strategy Profiles

Профілі є тестовими конфігураціями Material composition і process parameters, а не hardcoded Cell classes.

Мінімально:

```text
balanced
efficient
opportunist
storage-oriented
mobile
growth-oriented
repair-oriented
dormancy-oriented
```

## Environment Profiles

Мінімально:

```text
constant abundance
constant scarcity
Resource pulses
spatial patches
regenerating sparse world
high competition
hazard environment
mixed dynamic world
```

## Main Balance Rule

```text
кожна стратегія має хоча б одну нішу, де вона конкурентна
жодна стратегія не виграє майже всюди
перевага має вимірювану ціну
```

## Dormancy Rule

```text
dormancy допомагає пережити тимчасовий дефіцит
але програє активному growth/reproduction у стабільному достатку
```

---

# 10. Strategy Metrics

## Survival

```text
survived_to_end
survival_ticks
death_reason
active_fraction
dormancy_fraction
```

## Resource and Energy

```text
resource_capture_share
resource_efficiency
Energy production
Energy costs by process
mean/final Energy
storage utilization
```

## Development

```text
growth
division_ready_tick
division_count
offspring_count
offspring survival
generation reached
```

## Specialized Benefits

```text
movement benefit vs movement cost
repair benefit vs repair cost
storage benefit vs storage cost
dormancy survival benefit vs development loss
```

## Strategy Outcome

```text
scenario_score
relative_rank
win count
top-3 count
collapse count
dominance ratio
environment specialization
regret vs best
```

---

# 11. Scenario-Specific Scores

Не використовувати один універсальний score для всіх середовищ.

## Scarcity

Вищі ваги:

```text
survival
Resource efficiency
dormancy usefulness
```

## Abundance

Вищі ваги:

```text
growth
division
offspring
```

## Spatial Patches

Вищі ваги:

```text
Resource found
movement benefit
Energy per distance
```

## Hazard

Вищі ваги:

```text
survival
damage repaired
repair efficiency
```

Зберігати raw metrics поруч зі score.

---

# 12. Dominance Analysis

Виявляти:

## Dominant Strategy

```text
DOMINANT_STRATEGY
```

Якщо профіль:

```text
виграє у більшості середовищ
не має помітної ціни
має одночасно високі survival, efficiency, growth і reproduction
```

Звіт повинен пояснювати:

```text
де він домінує
за рахунок яких метрик
які параметри, ймовірно, створюють перекіс
у який бік їх варто змінити
```

## Non-Viable Strategy

```text
NON_VIABLE_STRATEGY
```

Якщо профіль:

```text
не має жодного середовища з перевагою
майже завжди collapses
не демонструє запланованого trade-off
```

## Dormancy Overperformance

```text
DORMANCY_OVERPERFORMS
```

Якщо dormancy-oriented:

```text
виграє в abundance
має високе reproduction
накопичує Resources/Energy без активної ціни
є найкращою середньою стратегією
```

## Dormancy Useless

```text
DORMANCY_UNREACHABLE_OR_USELESS
```

Якщо:

```text
dormancy не активується
не збільшує survival під час Resource gaps
```

---

# 13. Multi-Seed Analysis

Для механічних sweep-ів:

```text
1 seed допустимий для smoke/regression
3 seeds для перевірки чутливості
```

Для balance conclusions:

```text
мінімум 5 seeds
бажано 5-10
```

Зберігати:

```text
mean
median
min
max
standard deviation
win frequency
rank frequency
```

Окремо перевірити:

```text
same config + same seed = identical result
```

---

# 14. Required Raw Data

Зберігати у:

```text
outputs/raw_data/
```

## Existing Files

Розширити:

```text
viability_threshold.csv
passive_income_equilibrium.csv
upkeep_sensitivity.csv
dormant_modifier.csv
transport_metabolism_matrix.csv
```

## New Mechanical Summary Files

```text
sweep_scenario_summary.csv
sweep_activation_report.csv
sweep_warning_report.csv
conservation_report.csv
```

## Future Strategy Matrices

```text
strategy_environment_matrix.csv
strategy_environment_seed_matrix.csv
strategy_win_matrix.csv
strategy_rank_matrix.csv
dominance_matrix.csv
tradeoff_matrix.csv
dormancy_balance_matrix.csv
resource_efficiency_matrix.csv
energy_cost_matrix.csv
growth_reproduction_matrix.csv
movement_benefit_cost_matrix.csv
repair_benefit_cost_matrix.csv
```

Не записувати summary-коментарі у raw CSV рядками з `#`.

Використовувати окремо:

```text
raw CSV
summary CSV/JSON
Markdown report
```

---

# 15. Reports

## Mechanical Sweep Report

Створювати:

```text
outputs/reports/sweep-analysis-<timestamp>.md
outputs/reports/sweep-analysis-<timestamp>.json
```

Розділи:

```text
run metadata
scenario definitions
sweep definitions
metric definitions
mechanism activation status
zones reached
parameter effects
conservation results
warnings
recommended scenario/config corrections
generated files
```

Пояснення повинні розрізняти:

```text
parameter imbalance
scenario not activated
finite Resource exhaustion
missing parameter effect
real stable/fragile/collapse transition
```

## Balance Report

Після Stage 2:

```text
outputs/reports/balance-analysis-<timestamp>.md
outputs/reports/balance-analysis-<timestamp>.json
```

Розділи:

```text
tested strategies
tested environments
strategy × environment results
winners by environment
dominant strategies
non-viable strategies
dormancy analysis
trade-off analysis
recommended parameter changes
seed variance and confidence
```

---

# 16. Required Tests

## Unit

```text
scenario preset resolution
sweep scenario override
mechanism activation detection
single-zone detection
effect-size detection
environment-dominance detection
conservation calculations
zone classification
dominance detection
dormancy overperformance detection
```

## Integration

```text
each sweep uses its configured scenario
dormant modifier scenario reaches dormancy
transport/metabolism scenario avoids immediate Resource exhaustion
upkeep scenario produces multiple regimes
raw CSV schema is parseable
summary files are separate from raw data
same seed is deterministic
reports include explanations and warning codes
```

---

# 17. Implementation Order

1. Audit current `sweep_analyzer.toml`, runner and CSV writer.
2. Document which base fields are currently shared by all sweeps.
3. Add config-driven scenario presets.
4. Add scenario reference to every sweep/matrix.
5. Preserve current config as `smoke_regression`.
6. Add activation and low-information warnings.
7. Add extended raw metrics and accounting.
8. Re-run existing sweep suite.
9. Verify that each sweep tests its intended mechanism.
10. Only then implement `strategy × environment × seed`.
11. Add dominance and parameter recommendation reports.

---

# Acceptance Criteria

Task is complete when:

```text
current smoke/regression sweeps still run
each analytical sweep references an explicit scenario
dormant modifier actually activates dormancy
transport × metabolism tests throughput rather than only finite Resource exhaustion
upkeep sensitivity covers more than one meaningful regime
viability threshold remains explicitly a finite-stock survival test
passive income equilibrium remains operational
uninformative sweeps are detected automatically
parameter-with-no-effect cases are detected
Resource and Energy accounting are reported
raw CSV files contain scenario/config metadata
summary data is stored separately from raw CSV
Markdown and JSON reports explain results
strategy balance analyzer is separated from mechanical sweep analyzer
future strategy analysis can detect universal dominance and dormancy overperformance
```

## Important Constraint

Do not begin by manually tuning exact threshold values.

First ensure:

```text
the intended mechanism is activated
the scenario isolates the intended pressure
the measured metrics represent that mechanism
the sweep produces interpretable variation
```

Then parameter tuning can be based on evidence.

Before implementation, provide:

1. audit of the current analyzer and config;
2. proposed scenario preset schema;
3. mapping of each current sweep to its scenario;
4. proposed CSV and report schemas;
5. separate TDD implementation plan.
