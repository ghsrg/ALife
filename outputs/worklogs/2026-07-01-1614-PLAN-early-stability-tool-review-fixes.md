---
tags:
  - alife
  - worklog/plan
  - review
  - stability
---

# PLAN: Early Stability Tool Review Fixes

Date: 2026-07-01 16:14

---

# Goal

Довести `tools/early-stability/` від робочого MVP до надійного інструмента, який відповідає `docs/implementation/early-stability-tool.md` і дає корисні candidate configs, ranges і reports.

---

# Review Summary

Інструмент корисний і суттєво просунувся:

- є Python CLI;
- є `evaluate`, `simulate`, `tune`, `batch`;
- є static calculator, micro simulator, tuner, writers;
- є стандартні scenarios і tuning example;
- користувач підтвердив `56 passed in 1.24s`.

Але є кілька недоробок, які заважають вважати tool завершеним.

---

# P0. Tune report не отримує ranges, runs і best candidate metrics

## Проблема

`cli.py` викликає `write_report_markdown()` для tune mode без:

```text
parameter_ranges
runs
best_stable_metrics
base_config_path
```

Через це `report_writer.py` має код для recommended values, ranges, sensitivity і failure reasons, але tune report фактично не отримує ці дані.

## Вплив

`REPORT.md` може бути формально створений, але не виконувати головну роль: пояснити recommended values, empirical ranges, failure reasons і sensitivity.

## Пропозиція

У `run_tune_mode()`:

- передати `parameter_ranges = ranges`;
- передати `runs = runs`;
- передати `base_config_path = scenario_path`;
- обчислити `best_stable_metrics` з відповідного candidate/run;
- додати test, який перевіряє, що tune `REPORT.md` містить recommended values, ranges і failure reasons.

---

# P1. `runs/*.json` для tune не містить tick history

## Проблема

`tuner.py` викликає `run_micro_simulation()`, але в `runs` записує тільки:

```text
parameters
seed
survival_result
collapse_reason
```

`history` не зберігається.

## Вплив

Документація обіцяє detailed state step history files, але tune artifacts не дають побачити, як саме candidate прийшов до stable/collapse.

## Пропозиція

У tune run record додати:

```text
history
final_energy
final_heat
final_waste
final_state
```

Для великих grids пізніше можна додати `--history-mode full|summary`, але перша версія має відповідати README/spec.

---

# P1. `evaluate` і `batch` не запускають optional micro simulation

## Проблема

`evaluate` зараз використовує тільки `evaluate_static_bounds()`. `batch` також використовує тільки static calculator.

Документ каже:

```text
evaluate = validation + static calculator + optional configured simulation
```

## Вплив

Static calculator може дати `stable`, хоча micro simulation за `tick_count` може вийти `fragile` або `collapse` через Energy depletion, dormancy або thresholds.

## Пропозиція

Додати CLI flag:

```text
evaluate --with-simulation
batch --with-simulation
```

Поведінка:

- без flag лишається static-only;
- з flag після static stable/fragile запускається `run_micro_simulation()`;
- final result бере гірший результат між static і simulation.

Додати tests для розбіжності static stable -> simulation collapse.

---

# P1. Validation не перевіряє всі required numeric fields

## Проблема

`config_loader.py` перевіряє частину `cell.*`, але не всі числові поля з Phase 1 / tool contract:

```text
tick_count
space.spatial_grid_size
resources.passive_energy_income_placeholder
environment.* rates/current/thresholds
lifecycle.stress_energy_threshold
lifecycle.critical_capacity_overrun
estimates.*
```

## Вплив

Invalid configs можуть пройти validation і зламати tool пізніше або дати некоректний result.

## Пропозиція

Додати helper validation:

```text
require_number(path, non_negative=true)
require_bool(path)
require_non_empty_list(path)
require_known_enum(path, values)
```

Покрити tests:

- negative heat/waste rates;
- negative tick_count;
- zero/negative spatial_grid_size;
- invalid `world.boundary_mode`;
- negative estimates.

---

# P1. `allowed_parameters` silently ignores unknown/ranged params

## Проблема

`tuner.py` фільтрує `ranges` по `allowed_parameters`, але:

- якщо `allowed_parameters` містить параметр без range, це не помилка;
- якщо після фільтрації ranges пустий, tuner може створити один empty candidate;
- невідомі parameter paths можуть створити нові nested fields через `set_nested_value()`.

## Вплив

Tool може “налаштовувати” параметр, якого немає в config, або запускати tune без реального tuning.

## Пропозиція

Додати validation tuning config:

```text
allowed_parameters must be non-empty
each allowed parameter must exist in ranges
each ranged parameter must be allowed
each parameter path must exist in base config or be explicitly under estimates.*
empty candidate grid is invalid
```

Додати tests для цих випадків.

---

# P2. README має encoding artifact

## Проблема

`tools/early-stability/README.md` містить mojibake tree characters:

```text
в”њв”Ђв”Ђ
```

## Вплив

Документ виглядає поламаним і погіршує handoff для агентів.

## Пропозиція

Замінити tree diagram на ASCII:

```text
tools/early-stability/
  src/
    cli.py
```

---

# P2. Tracked outputs конфліктують із `.gitignore`

## Проблема

`.gitignore` містить:

```text
outputs/
```

але в git tracked є:

```text
outputs/worklogs/*.md
outputs/worklogs/README.md
```

## Вплив

Незрозуміло, чи worklogs мають бути версіоновані. Поточний стан суперечливий: нові outputs ніби ігноруються, але частина вже в історії.

## Пропозиція

Прийняти одне правило:

1. Якщо worklogs мають бути локальними artifact-ами: прибрати tracked outputs з index через `git rm --cached outputs/...` і лишити `.gitignore`.
2. Якщо worklogs треба версіонувати: змінити `.gitignore` на:

```text
outputs/*
!outputs/worklogs/
!outputs/worklogs/*.md
```

Рекомендація: для цього проєкту worklogs корисні як історія рішень, тому краще явно дозволити `outputs/worklogs/*.md`, але ігнорувати `outputs/stability/`.

---

# P2. CLI package ergonomics слабкі

## Проблема

README показує запуск:

```text
python src/cli.py ...
```

але `pyproject.toml` не має console script entrypoint.

## Вплив

Tool важче запускати стабільно з root repo або після editable install.

## Пропозиція

Додати в `pyproject.toml`:

```toml
[project.scripts]
early-stability = "cli:main"
```

Або, якщо буде package layout, перейти на module path.

Оновити README команди:

```text
python -m cli ...
early-stability ...
```

---

# P3. Tests мають дубльований ручний TOML serializer

## Проблема

`test_config_loader.py` багато разів дублює локальний `dict_to_toml()`.

## Вплив

Тести шумні й важчі для підтримки.

## Пропозиція

Винести test helper:

```text
tests/helpers.py
```

або тестувати через маленькі TOML snippets без реконструкції всього config.

---

# Proposed Fix Order

1. Fix tune report data plumbing.
2. Add tune run history/final metrics.
3. Add validation for tuning config and parameter paths.
4. Add optional simulation for evaluate/batch.
5. Expand config validation.
6. Clean README encoding and CLI docs.
7. Decide tracked outputs policy.
8. Refactor noisy tests.

---

# Verification Plan

After fixes:

```powershell
python -m pytest .\tools\early-stability
python .\tools\early-stability\src\cli.py evaluate --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --out .\outputs\stability\review_evaluate
python .\tools\early-stability\src\cli.py simulate --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --ticks 20 --out .\outputs\stability\review_simulate
python .\tools\early-stability\src\cli.py tune --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --tuning .\tools\early-stability\tuning\single_cell.toml --out .\outputs\stability\review_tune
python .\tools\early-stability\src\cli.py batch --scenarios .\tools\early-stability\scenarios --out .\outputs\stability\review_batch
```

Manual artifact checks:

```text
review_tune/results.json
review_tune/ranges.json
review_tune/REPORT.md
review_tune/runs/run_0001.json
review_tune/recommended-configs/*.toml
```

Expected:

- tests pass;
- tune report contains ranges, recommended values, sensitivity and failure reasons;
- run files include history or explicit summary mode;
- README has no mojibake;
- git status does not include unwanted outputs or `.idea/`.
