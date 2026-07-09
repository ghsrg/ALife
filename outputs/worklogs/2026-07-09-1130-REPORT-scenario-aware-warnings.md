---
tags:
  - alife
  - worklog/report
---

# Звіт: Scenario-Aware Warnings (Сценаріє-залежні попередження)

## Опис виконаних робіт

У межах реалізації **Task 5: Scenario-Aware Warnings** на гілці `feat-scenario-mapping-accounting` було виконано такі роботи:

1. **Написання TDD-тестів (`tests/phase2_sweep_warnings.rs`)**:
   - Реалізовано допоміжну функцію `mock_result_custom` для гнучкого моделювання результатів симуляції (`SimResult`) із налаштуванням кількості виконаних тиків (`ticks_executed`), причини смерті (`death_reason`), активності та стану колапсу.
   - **Test 1**: Для сценарію `"finite_resource_viability"` перевірено, що якщо всі симуляції завершилися колапсом, але діапазон тривалості виживання (`survival_ticks`/`ticks_executed`) є значним (наприклад, min = 10, max = 80), то попередження `LOW_INFORMATION_SWEEP` **не** генерується. Якщо діапазон незначний (наприклад, min = 10, max = 12), попередження генерується.
   - **Test 2**: Для сценарію `"dormancy_survival"` перевірено, що якщо `dormant_ticks` дорівнює `0` для всіх запусків, обов'язково повертається попередження `SCENARIO_MECHANISM_NOT_ACTIVATED`.
   - **Test 3**: Для сценарію `"resource_abundance"` перевірено, що якщо всі запуски колапсували, і принаймні один із них завершився через екологічні ліміти (причина смерті містить `"Heat"` або `"Waste"`), повертається попередження `ENVIRONMENT_DOMINATED_RESULT`.
   - **Test 4**: Для сценарію `"steady_resource_flow"` перевірено, що якщо загальна кількість запусків велика (наприклад, 20), а стабільними є лише <= 5% (наприклад, 1 стабільна, 19 колапсованих), то генерується `LOW_INFORMATION_SWEEP` та додається рекомендація щодо звуження діапазону параметрів.

2. **Модифікація `detect_warnings` у `src/bin/sweep_analyzer.rs`**:
   - Інтегровано логіку чутливості до сценаріїв (з case-insensitive перевіркою `scenario_id`).
   - Для `"finite_resource_viability"`: додано супресію `LOW_INFORMATION_SWEEP` за умови, що різниця між `max_survival` та `min_survival` перевищує `5` тиків.
   - Для `"dormancy_survival"`: забезпечено додавання `SCENARIO_MECHANISM_NOT_ACTIVATED` у випадку відсутності періодів сплячки.
   - Для `"resource_abundance"`: реалізовано детекцію домінування екологічних лімітів (регістронезалежний пошук `"heat"` та `"waste"` у полі `death_reason`) для генерування `ENVIRONMENT_DOMINATED_RESULT`.
   - Для `"steady_resource_flow"`: додано детекцію низької щільності стабільних рішень (<= 5%) із додаванням попередження `LOW_INFORMATION_SWEEP` та рекомендації `LOW_INFORMATION_SWEEP: Recommend narrowing the parameter range since stable runs are very sparse.`.

3. **Верифікація**:
   - Усі тести успішно скомпілювалися та пройшли без попереджень (pristine output).

## Результати тестування

Команда `cargo test --workspace` пройшла успішно:

```text
running 6 tests
test test_detects_low_information_sweep ... ok
test test_scenario_dormancy_survival_no_activation ... ok
test test_scenario_finite_resource_viability_ticks_responsive ... ok
test test_detects_mechanism_not_activated ... ok
test test_scenario_resource_abundance_environment_dominated ... ok
test test_scenario_steady_resource_flow_low_stable_density ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
