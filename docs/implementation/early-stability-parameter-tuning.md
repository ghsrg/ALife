---
tags:
  - alife
  - implementation
  - tools
  - stability
  - tuning
---

# Early Stability Parameter Tuning Guide

> Інструкція для підбирання параметри через `tools/early-stability/`.

---

# Мета

Завдання не в тому, щоб "зробити конфіг красивим".

Завдання:

```text
запустити baseline
знайти причини collapse / fragile
рухати тільки дозволені параметри
знайти стабільний або найменш крихкий діапазон
пояснити, що саме тримає або ламає життєздатність
видати рекомендовані значення і min/max діапазони
```

Tool не змінює Canon і не редагує source-of-truth configs без окремої команди.

---

# Вхідні Документи

Перед роботою агент має прочитати:

- [[docs/implementation/early-stability-tool|Early Stability Tool]]
- [[docs/implementation/phase-1-design|Phase 1 Design]]
- [[docs/config/stability_bounds|Stability Bounds]]
- [[docs/world/energy|Energy Buffer]]
- [[docs/world/physics|Physics]]
- [[docs/biology/lifecycle|Lifecycle]]

Якщо результати tool-а суперечать Canon, агент не змінює Canon автоматично. Він фіксує проблему в report.

---

# Базові Команди

Запустити тести tool-а:

```powershell
python -m pytest .\tools\early-stability
```

Baseline batch:

```powershell
python .\tools\early-stability\src\cli.py batch `
  --scenarios .\tools\early-stability\scenarios `
  --out .\outputs\stability\baseline-batch `
  --with-simulation
```

Baseline tune:

```powershell
python .\tools\early-stability\src\cli.py tune `
  --scenario .\tools\early-stability\scenarios\single_cell_survival.toml `
  --tuning .\tools\early-stability\tuning\single_cell.toml `
  --out .\outputs\stability\baseline-tune
```

Якщо встановлено editable package:

```powershell
early-stability batch --scenarios .\tools\early-stability\scenarios --out .\outputs\stability\baseline-batch --with-simulation
early-stability tune --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --tuning .\tools\early-stability\tuning\single_cell.toml --out .\outputs\stability\baseline-tune
```

---

# Що Потрібно Ловити

## 1. Energy Collapse

Ознаки:

```text
survival_result = collapse
collapse_reason = energy_depleted
collapse_reason = mandatory_cost_unpaid
final_energy -> 0
history показує спад Energy кожен tick
```

Що перевірити:

```text
cell.initial_energy
cell.energy_capacity
cell.mandatory_cost_per_tick
cell.dormant_mandatory_cost_modifier
resources.passive_energy_income_placeholder
lifecycle.stress_energy_threshold
```

Типові висновки:

```text
mandatory cost зависокий
passive income замалий
initial energy достатній лише для короткого run
energy_capacity не дає накопичити запас
dormancy не рятує або робить клітину тільки fragile
```

---

## 2. Fragile Survival

Ознаки:

```text
survival_result = fragile
collapse_reason = none
history має state = stressed або dormant
final_energy близько до stress threshold
heat/waste близько до warning threshold
```

Що перевірити:

```text
мінімальний energy margin
heat_warning_threshold - max_heat
waste_warning_threshold - max_waste
скільки ticks клітина stressed/dormant
чи survival залежить від вузького діапазону одного параметра
```

Fragile не є повністю поганим результатом. Він корисний як `fragile_edge`, але не має ставати базовим default config без рішення.

---

## 3. Heat Instability

Ознаки:

```text
collapse_reason = heat_limit_exceeded
final_heat росте кожен tick
heat_generated_per_tick > heat_dissipation_rate
heat_warning_threshold швидко перетинається
```

Що рухати:

```text
environment.heat_dissipation_rate
environment.heat_warning_threshold
environment.heat_death_threshold
environment.heat_generated_per_tick
```

Обмеження:

```text
не можна просто підняти death threshold без пояснення
не можна ігнорувати heat accumulation, якщо sink відсутній
heat damage має залишатись пов'язаним з Material degradation thresholds
```

---

## 4. Waste Instability

Ознаки:

```text
collapse_reason = waste_limit_exceeded
final_waste росте кожен tick
waste_generated_per_tick > waste_sink_rate
waste_warning_threshold швидко перетинається
```

Що рухати:

```text
environment.waste_sink_rate
environment.waste_warning_threshold
environment.waste_death_threshold
environment.waste_generated_per_tick
```

Що ловити:

```text
чи stable виникає тільки через нереально високий sink
чи waste threshold просто маскує відсутність балансу
чи waste стабілізується, а не повільно накопичується до collapse
```

---

## 5. Capacity Failure

Ознаки:

```text
survival_result = invalid або collapse
collapse_reason = capacity_exceeded
initial_resources + initial_materials > capacity_limit
```

Що рухати:

```text
cell.capacity_limit
cell.initial_resources
cell.initial_materials
cell.radius
```

Не робити:

```text
не збільшувати capacity_limit без перевірки radius/space constraints
не видаляти Materials, які входять у minimum_viability_materials
```

---

## 6. Static Stable, Simulation Collapse

Ознаки:

```text
evaluate без --with-simulation дає stable
evaluate --with-simulation дає collapse
```

Це важливий сигнал.

Причина:

```text
static calculator бачить лише грубі bounds
micro simulator бачить cumulative tick dynamics
```

Що робити:

```text
вважати simulation result сильнішим сигналом
перевірити Energy depletion over time
перевірити Heat/waste accumulation over time
не приймати static-only stable як достатній результат
```

---

# Що Рухати Спочатку

Порядок підбору:

```text
1. Зробити single_cell_survival stable.
2. Перевірити, що single_cell_starvation collapse.
3. Перевірити, що single_cell_over_capacity invalid/collapse.
4. Перевірити, що heat/waste scenarios ловлять heat/waste failure.
5. Підібрати conservative stable для single_cell_survival.
6. Підібрати fragile_edge для межі.
7. Лише після цього дивитись growth/division/joint estimate scenarios.
```

Причина:

```text
немає сенсу підбирати growth/division,
якщо базова клітина не може стабільно оплатити mandatory costs.
```

---

# Дозволені Параметри Для Першого Підбору

Починати з малого набору:

```text
cell.initial_energy
cell.energy_capacity
cell.mandatory_cost_per_tick
resources.passive_energy_income_placeholder
environment.heat_dissipation_rate
environment.waste_sink_rate
lifecycle.stress_energy_threshold
```

Другий рівень, якщо перший не дає стабільності:

```text
cell.capacity_limit
cell.dormant_mandatory_cost_modifier
environment.heat_warning_threshold
environment.heat_death_threshold
environment.waste_warning_threshold
environment.waste_death_threshold
```

Estimate-only параметри:

```text
estimates.growth_cost_estimate
estimates.division_cost_estimate
estimates.resource_regeneration_or_inflow
estimates.population_space_limit
estimates.joint_count_estimate
estimates.joint_upkeep_cost
```

Estimate-only параметри не є runtime state.

---

# Заборонено

Агент не має:

```text
міняти Canon rules
міняти docs/PRINCIPLES.md під результат tool-а
редагувати source scenarios без окремої команди
приймати fragile як stable
ігнорувати --with-simulation
робити random search без seed/control
розширювати allowed_parameters без пояснення
змінювати collapse_reason vocabulary
додавати нові world laws у tool
```

---

# Як Оцінювати Результат

## Stable

Прийнятний результат:

```text
survival_result = stable
collapse_reason = none
final_energy > stress_energy_threshold з запасом
max_heat < heat_warning_threshold
max_waste < waste_warning_threshold
state не переходить у stressed/dormant більшість ticks
```

## Conservative Stable

Кращий кандидат для base config:

```text
має найбільший safety margin
не сидить біля thresholds
не потребує нереально високого passive income
не потребує абсурдно низького mandatory cost
```

## Fragile Edge

Корисний для boundary tests:

```text
виживає
але близький до stress/heat/waste threshold
показує мінімальні межі життєздатності
```

Fragile Edge не є default config.

## Collapse

Корисний негативний тест:

```text
starvation має падати
over capacity має падати або бути invalid
heat stress має ловити heat failure
waste overload має ловити waste failure
```

Якщо негативні сценарії стають stable, це може означати, що tool занадто м'який або thresholds завищені.

---

# Очікуваний Report Від Агента

Агент має створити:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-early-stability-parameter-tuning.md
```

Мінімальний зміст:

```text
# REPORT: Early Stability Parameter Tuning

## Commands Run
- ...

## Baseline Results
| Scenario | Static Result | Simulation Result | Reason |

## Tune Results
| Run | Stable | Fragile | Collapse | Invalid |

## Recommended Base Values
| Parameter | Current | Recommended | Tested Min | Tested Max | Stable Min | Stable Max | Reason |

## Boundary Values
| Parameter | Fragile Min | Fragile Max | Collapse Edge | Notes |

## Failure Reasons
- ...

## Proposed Changes
- source config changes proposed, not applied

## Do Not Change
- Canon rules unchanged
- source configs unchanged unless explicitly requested
```

---

# Які Артефакти Зберігати

Усі запускі складати в:

```text
outputs/stability/<run-id>/
```

Наприклад:

```text
outputs/stability/baseline-batch/
outputs/stability/baseline-tune/
outputs/stability/single-cell-survival-tune-v2/
outputs/stability/heat-waste-bounds-v1/
```

Не комітити generated stability artifacts, якщо немає окремої команди.

---

# Що таке цикл підбору

Цикл = один повний прохід:
run baseline/tune
проаналізувати collapse/fragile причини
змінити allowed tuning ranges або tuning profile
запустити повторно
порівняти результат

---

# Покращення

Покращенням вважаємо хоча б одне:
stable_count збільшився
collapse_count зменшився
fragile_count перейшов у stable
з'явився conservative stable candidate
safety margin покращився
причина collapse стала менш критичною
діапазон stable values розширився

Не вважаємо покращенням:
ті самі stable/fragile/collapse counts
ті самі collapse reasons
ті самі або гірші margins
стабільність отримана тільки через абсурдні thresholds
негативні сценарії перестали падати

---

# Коли Зупиняти Підбір

Зупинити і принести report, якщо:

```text
знайдено conservative stable candidate
знайдено fragile edge candidate
жоден stable не знайдено після розумного розширення ranges
негативні сценарії перестали падати
потрібно змінювати Canon або semantics
потрібно рухати заборонені параметри
результати static і simulation системно розходяться
```

Не треба нескінченно підганяти числа. Зупинка після 3 циклів без покращення.
Do not continue numeric fitting if the same collapse reasons repeat.
Якщо покращення не суттєві то зупинка після 5 циклів

---

# Корисний Результат

Корисний результат від агента:

```text
не "тести пройшли"
а "ось діапазони, де клітина стабільна,
ось межа крихкості,
ось чому вона падає,
ось які значення пропоную для base config,
ось що не можна вирішити без зміни моделі"
```

---

# Наступні Групи Підбору

Після першого `single_cell_survival` sweep прийнято тільки ranges для:

```text
cell.initial_energy
cell.mandatory_cost_per_tick
environment.heat_dissipation_rate
```

Наступні підбори треба робити групами. Не змішувати всі параметри в один великий sweep, бо тоді неможливо зрозуміти, що саме покращило або зламало результат.

## Group 1: Energy Budget Sweep

Мета:

```text
визначити енергетичний запас, при якому Cell стабільно платить mandatory costs
відділити stable, fragile і starvation edges
```

Сценарії:

```text
single_cell_survival
single_cell_starvation
single_cell_growth_budget
```

Параметри:

```text
cell.initial_energy
cell.energy_capacity
cell.mandatory_cost_per_tick
resources.passive_energy_income_placeholder
lifecycle.stress_energy_threshold
cell.dormant_mandatory_cost_modifier
```

Ловити:

```text
energy_depleted
mandatory_cost_unpaid
fragile через low energy margin
dormancy loops
static stable / simulation collapse
```

Очікуваний результат:

```text
stable range для initial_energy / energy_capacity / mandatory_cost
fragile edge для starvation boundary
conservative baseline recommendation без зміни Canon
```

## Group 2: Heat And Waste Sweep

Мета:

```text
перевірити, чи Heat і waste мають явний sink і не накопичуються нескінченно
```

Сценарії:

```text
single_cell_survival
single_cell_heat_stress
waste_heat_balance
```

Параметри:

```text
environment.heat_generated_per_tick
environment.heat_dissipation_rate
environment.heat_warning_threshold
environment.heat_death_threshold
environment.waste_generated_per_tick
environment.waste_sink_rate
environment.waste_warning_threshold
environment.waste_death_threshold
```

Ловити:

```text
heat_limit_exceeded
waste_limit_exceeded
slow accumulation under warning threshold
stable тільки через завищені thresholds
negative scenarios becoming stable
```

Очікуваний результат:

```text
heat generation/dissipation stable range
waste generation/sink stable range
warning/death threshold recommendations
collapse edge для heat і waste
```

## Group 3: Capacity And Material Sweep

Мета:

```text
перевірити, що початкові Resources/Materials вміщуються фізично
і capacity не відривається від radius/material assumptions
```

Сценарії:

```text
single_cell_survival
single_cell_over_capacity
single_cell_growth_budget
```

Параметри:

```text
cell.capacity_limit
cell.initial_resources
cell.initial_materials
cell.radius
resources.initial_distribution
```

Ловити:

```text
invalid_config
capacity_exceeded
stored amount above capacity
minimum_viability_materials missing
capacity too high for radius without explanation
```

Очікуваний результат:

```text
capacity stable range
minimum viable material/resource pack
over-capacity invalid boundary
notes if tool cannot evaluate radius/capacity physics yet
```

## Group 4: Growth And Division Estimate Sweep

Мета:

```text
перевірити, чи базова Cell теоретично може рости і входити в division loop
без повної Phase 2 simulation
```

Сценарії:

```text
single_cell_growth_budget
single_cell_division_loop_estimate
population_growth_bound
```

Параметри:

```text
estimates.growth_cost_estimate
estimates.division_cost_estimate
estimates.resource_regeneration_or_inflow
estimates.population_space_limit
cell.energy_capacity
resources.passive_energy_income_placeholder
```

Ловити:

```text
growth impossible
division loop impossible
population_unbounded
resource budget insufficient
stable only because estimate is too optimistic
```

Очікуваний результат:

```text
growth budget range
division budget range
population pressure warning range
explicit limits of evidence because estimates are tool-only
```

## Group 5: Joint Upkeep Estimate Sweep

Мета:

```text
перевірити, чи multicellular structure теоретично може підтримувати Joints
на рівні estimate-tool, без повної Joint simulation
```

Сценарії:

```text
joint_upkeep_budget
population_growth_bound
single_cell_survival
```

Параметри:

```text
estimates.joint_count_estimate
estimates.joint_upkeep_cost
estimates.resource_regeneration_or_inflow
cell.mandatory_cost_per_tick
resources.passive_energy_income_placeholder
```

Ловити:

```text
joint_upkeep_impossible
energy budget consumed by upkeep
joint support possible only at narrow margins
multicellular estimate stable while single cell baseline is fragile
```

Очікуваний результат:

```text
joint_count / joint_upkeep stable ranges
fragile edge for multicellular upkeep
clear note that this is estimate-only until Joint model exists
```

## Group Stop Rules

Для кожної групи:

```text
max_cycles = 5
max_no_improvement_cycles = 3
max_iterations_per_cycle = 300..1000 depending on parameter count
```

Зупинитись і надати report, якщо:

```text
3 cycles поспіль без meaningful improvement
negative scenario перестав падати
для стабільності треба рухати заборонений параметр
потрібна зміна Canon або process semantics
tool не має потрібного механізму для чесної оцінки
```

Report має явно вказати:

```text
яку групу ганяли
які параметри були allowed
stable / fragile / collapse / invalid before/after
які ranges приймати
які source configs не змінювались
які питання залишились за межами tool-а
```

---

# Semantic Links

- uses: [[docs/implementation/early-stability-tool|Early Stability Tool]]
- supports: [[docs/implementation/phase-1-design|Phase 1 Design]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]
- checks: [[docs/world/energy|Energy Buffer]]
- checks: [[docs/world/physics|Physics]]
- checks: [[docs/biology/lifecycle|Lifecycle]]
