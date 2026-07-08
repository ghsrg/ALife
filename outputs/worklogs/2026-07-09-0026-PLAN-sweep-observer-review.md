# План аудиту та виправлення коду Task 6: Sweep Analyzer CLI Integration & Output Writer

## Огляд змін
Цей документ містить план перевірки та покращення якості коду для задачі інтеграції CLI аналізатора параметричних досліджень та генератора звітів (`sweep_analyzer`).

## Виявлені зауваження

### 1. Попередження Clippy (Clippy Warnings)
- У файлі [sweep_analyzer.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/bin/sweep_analyzer.rs#L1611) присутнє попередження `clippy::unnecessary_sort_by`:
  ```rust
  sorted_sweeps.sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));
  ```
  Метод `a.0` є типом `String`, тому виклик `.to_string()` є зайвим, створює додаткові алокації під час кожного порівняння.
  **Рішення**: Змінити на `sorted_sweeps.sort_by(|a, b| a.0.cmp(&b.0));` або `sorted_sweeps.sort_unstable_by(|a, b| a.0.cmp(&b.0));`.

- У файлі [phase2_sweep_observer_outputs.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/tests/phase2_sweep_observer_outputs.rs#L7) присутнє попередження `clippy::needless_borrows_for_generic_args`:
  ```rust
  cmd.args(&["run", "--bin", "sweep_analyzer", "--", "sweep_analyzer_smoke.toml"]);
  ```
  **Рішення**: Прибрати непотрібне запозичення `&` для масиву аргументів, щоб отримати `cmd.args(["run", "--bin", "sweep_analyzer", "--", "sweep_analyzer_smoke.toml"]);`.

### 2. Захист від ділення на нуль (Division by Zero Protection)
- У функції `classify` у файлі [sweep_analyzer.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/bin/sweep_analyzer.rs#L1008-L1023):
  ```rust
  let dormant_pct = res.dormant_ticks as f32 / ticks as f32;
  let active_pct = res.active_ticks as f32 / ticks as f32;
  ```
  Якщо параметр `ticks` дорівнює `0`, це призведе до ділення на нуль і отримання `NaN`/`Infinity`.
  Хоча в більшості місць передається валідна кількість тіків, краще зробити функцію стійкою до некоректних аргументів.
  **Рішення**: Захистити дільник за допомогою `.max(1)`:
  ```rust
  let ticks_f = ticks.max(1) as f32;
  let dormant_pct = res.dormant_ticks as f32 / ticks_f;
  let active_pct = res.active_ticks as f32 / ticks_f;
  ```

## План дій

1. Застосувати виправлення у файлі [sweep_analyzer.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/bin/sweep_analyzer.rs).
2. Застосувати виправлення у файлі [phase2_sweep_observer_outputs.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/tests/phase2_sweep_observer_outputs.rs).
3. Запустити `cargo clippy --all-targets` для підтвердження відсутності попереджень.
4. Запустити `cargo test` для підтвердження проходження інтеграційних та юніт-тестів.
5. Зафіксувати результати у звіті `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-sweep-observer-review.md`.
