---
tags:
  - alife
  - worklog/report
---

# Звіт: Detailed Accounting Category Breakdown

## Опис виконаних робіт

У рамках реалізації Task 2: Detailed Accounting Category Breakdown було внесено такі зміни:

1. **Оновлено структуру `SimResult` в `src/bin/sweep_analyzer.rs`**:
   - Додано нові аналітичні поля:
     - `explicit_energy_loss: f32`
     - `death_cleanup_loss_energy: f32`
     - `death_cleanup_loss_resources: f32`
     - `clamping_loss: f32`
     - `unpaid_mandatory_cost: f32`
     - `resource_decay: f32`
     - `resource_sink: f32`
     - `numerical_error_energy: f32`
     - `numerical_error_resources: f32`
     - `unclassified_loss_energy: f32`
     - `unclassified_loss_resources: f32`

2. **Оновлено розрахунки балансу в `run_simulation`**:
   - Реалізовано накопичення `clamping_cumulative` на основі різниці між очікуваною та фактичною енергією після кожного тику.
   - Реалізовано розрахунок нестягненої обов'язкової плати (`unpaid_mandatory_cost_cumulative`) для клітин, у яких `mandatory_paid == false`.
   - Реалізовано детекцію смерті клітин (перехід від живого стану до `LifecycleState::Dead`) з накопиченням їхніх залишків енергії (`death_cleanup_loss_energy`) та ресурсів/матеріалів (`death_cleanup_loss_resources`).
   - Оновлено розрахунки похибок балансу енергії та ресурсів з порівнянням за толерантністю `0.01`, що розділяє балансові відхилення на `numerical_error_*` та `unclassified_loss_*`.

3. **Оновлено серіалізацію в CSV та Markdown репорти**:
   - Оновлено заголовки та формати рядків у функціях `run_sweep` та `run_matrix` для врахування нових полів.
   - Оновлено `write_report` для додавання секції з описом нових деталізованих категорій балансу.

4. **Оновлено тести**:
   - Додано нові поля у mock `SimResult` в `tests/phase2_sweep_warnings.rs`.

5. **Очищено невикористовувані змінні**:
   - Видалено `initial_total_res` та `final_cell_mat` для усунення попереджень компілятора.

## Результати верифікації

Усі тести успішно пройшли локально в робочій директорії за допомогою команди `cargo test --workspace`.
