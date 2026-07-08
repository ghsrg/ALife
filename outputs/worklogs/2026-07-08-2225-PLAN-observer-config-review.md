---
tags:
  - alife
  - audit
  - observer
  - config
---

# PLAN: Рецензія якості коду парсерів конфігурації Observer

## 1. Ціль

Провести огляд коду та виявити проблеми в реалізації парсерів конфігурацій Observer (`feat-observer-classification`), оцінити відповідність ідіомам Rust, безпеку серіалізації, ефективність пам'яті, консистентність назв і форматування коду.

## 2. Пов'язані документи

- [[docs/observer/INDEX]]
- [[docs/observer/classification-registry]]
- [[docs/observer/classification-contract]]
- [[outputs/worklogs/2026-07-08-2215-REPORT-observer-config-parsers]]

## 3. Оцінка реалізації

### 3.1 Сильні сторони (Strengths)
- **Де deserialization з Serde**: Використання стандартних можливостей `serde::Deserialize` для відображення TOML структур.
- **Детермінованість**: Сортування умов (`RuleClause`) за пріоритетом або алфавітом забезпечує передбачуваність при аналізі.
- **Наявність тестів**: Написано базовий інтеграційний тест `tests/phase2_observer_config.rs` для перевірки завантаження файлів конфігурації.

### 3.2 Проблеми (Issues)

#### Critical Issues:
1. **Невідповідність назв реєстру (Naming Registry Mismatch)**:
   - Функції `load_cell_role_classifier`, `load_behavior_profile_classifier` та `load_organism_archetype_classifier` у файлі [config.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/config.rs) примусово додають суфікс `-like` до ключів, які його не мають.
   - Це суперечить канонічним назвам з реєстру [classification-registry.toml](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/docs/config/observer/classification-registry.toml), де деякі лейбли закінчуються на `-supporting` (наприклад, `boundary-supporting`) або `-oriented` (наприклад, `synthesis-oriented`, `dormancy-oriented`).
   - Через це згенеровані ключі в HashMap (наприклад, `boundary-supporting-like` або `dormancy-oriented-like`) не будуть збігатися з офіційними лейблами реєстру, що зламає майбутню інтеграцію та класифікацію.
   - **Рішення**: Прибрати додавання суфікса `-like` в коді завантажувачів і використовувати оригінальні назви ключів із TOML файлів.

#### Minor / Code Quality Issues:
2. **Ідіоматика Rust (Clippy Warnings)**:
   - Ручне зрізання префіксів через `&key[4..]` після перевірки `key.starts_with("min_")` є небезпечним та неідіоматичним. Потрібно використовувати безпечний метод `strip_prefix`.
   - Гілка `key.starts_with("requires_")` повертає той самий кортеж `("==", key)`, що й `else`, створюючи надлишкове розгалуження (Identical blocks warning).
   - Невикористаний імпорт `load_organism_archetype_classifier` в [phase2_observer_config.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/tests/phase2_observer_config.rs).

3. **Форматування коду (Cargo Fmt Check Failed)**:
   - Виявлено чимало відхилень від стандартного форматування Rust у файлах:
     - [config.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/config.rs)
     - [mod.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/mod.rs)
     - [phase2_observer_config.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/tests/phase2_observer_config.rs)

---

## 4. Кроки виправлення (Fix Steps)

- [ ] **Крок 1**: Видалити нормалізацію суфіксів `-like` з функцій `load_cell_role_classifier`, `load_behavior_profile_classifier` та `load_organism_archetype_classifier` у [config.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/config.rs).
- [ ] **Крок 2**: Оптимізувати `map_field_to_clause` у [config.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/config.rs) за допомогою `strip_prefix` та спрощення умов розгалуження.
- [ ] **Крок 3**: Виправити інтеграційний тест [phase2_observer_config.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/tests/phase2_observer_config.rs):
  - Прибрати невикористаний імпорт `load_organism_archetype_classifier`.
  - Змінити очікувані ключі перевірок на оригінальні назви: `"boundary-supporting"`, `"dormancy-oriented"`.
- [ ] **Крок 4**: Запустити `cargo fmt --all` для автоматичного вирівнювання стилю коду.
- [ ] **Крок 5**: Перевірити проходження всіх тестів та відсутність попереджень clippy.

---

## 5. Вердикт

❌ **NEEDS FIXES** (Потребує виправлень)
