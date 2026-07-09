---
tags:
  - alife
  - worklog/plan
  - tdd
  - phase/2d-b
  - rust
  - rust-domain-modeling
---

# План реалізації Phase 2D-B: Смерть та декомпозиція (Death & Decomposition)

## Мета
Реалізувати перехід клітини в мертвий стан, збереження її як фізичного залишку в ігровому світі (matter preservation) та поступовий розпад (decomposition) її внутрішніх ресурсів та матеріалів у локальні комірки `ResourceGrid`.

## Обмеження та нюанси калібрування Phase 2C/2I
- **Збереження речовини (Matter Accounting)**: Мертва клітина не повинна зникати безслідно. Її речовина (ресурси та матеріали) повертається у навколишнє середовище через декомпозицію.
- **Блокування активності**: Мертва клітина повинна ігнорувати всі активні процеси (поглинання, метаболізм, синтез, ріст, переміщення, поділ). Вона виступає лише як пасивне джерело речовини.
- **Стабільність індексів**: Для уникнення зсуву індексів у SoA-масивах, рядки мертвих клітин не видаляються фізично з пам'яті. Замість цього, після повного розпаду речовини клітина отримує прапорець `inert = true`, і її геометричний радіус перестає впливати на симуляцію.

---

## План завдань

### Завдання 1: Додавання конфігурації декомпозиції (Decomposition Config)
**Файли:**
- `src/core/config.rs`
- `src/runner/config_parser.rs`
- `tests/phase2_decomposition_smoke.rs`

- [ ] **Крок 1: Опис структури `DecompositionConfig`**
  ```rust
  #[derive(Clone, Copy, Debug, PartialEq)]
  pub struct DecompositionConfig {
      pub enabled: bool,
      pub resource_layer_index: usize,
      pub resources_per_tick: ResourceAmount,
      pub materials_per_tick: MaterialAmount,
      pub remove_when_empty: bool,
  }
  ```
- [ ] **Крок 2: Валідація конфігурації**
  Перевірити, що `resource_layer_index` відповідає наявному шару ресурсів у світі.
- [ ] **Крок 3: Додавання до `RuntimeConfig` та `config_hash()`**
  Включити поля `DecompositionConfig` у загальний хеш конфігурації симуляції.

### Завдання 2: Механізм декомпозиції мертвих клітин
**Файли:**
- `src/core/world.rs`
- `src/core/tick.rs`

- [ ] **Крок 1: Реалізація `execute_decomposition_for_dead_cells`**
  - Якщо декомпозиція вимкнена в конфігурації, нічого не робити.
  - Пройтися по всіх клітинах у SoA сховищі.
  - Для кожної клітини в стані `LifecycleState::Dead`:
    - Визначити координату ресурсного поля `GridCoord` під клітиною.
    - Вивільнити до `resources_per_tick` внутрішніх ресурсів у відповідну комірку сітки ресурсів.
    - Вивільнити до `materials_per_tick` матеріалів, зменшуючи послідовно матеріальні бакети (в порядку: `boundary`, `transport`, `metabolic`, `storage`, `synthesis`, `structural`, `repair`, `contractile`, `sensory`).
    - Кожен розпадений матеріал перетворюється на універсальний ресурс у сітці (заглушка Phase 2D).
    - Коли всі внутрішні ресурси та матеріали розпалися до `0.0`, встановити прапорець `inert = true` для клітини.

### Завдання 3: Інтеграція декомпозиції в Tick Executor
**Файли:**
- `src/core/tick.rs`
- `src/core/summary.rs`

- [ ] **Крок 1: Виклик декомпозиції в кроці симуляції**
  У `TickExecutor::step` після виконання поділу запустити:
  ```rust
  let decomposed_count = self.world.execute_decomposition_for_dead_cells();
  ```
- [ ] **Крок 2: Оновлення підсумкових лічильників**
  Передати `decomposed_cells_count` та `dead_cells_count` у `MetricsSummary`.

### Завдання 4: Блокування активних процесів для мертвих клітин
**Файли:**
- `src/core/tick.rs`
- `tests/phase2_decomposition_smoke.rs`

- [ ] **Крок 1: Перевірка життєвого циклу в процесах**
  Переконатися, що у фазах поглинання, метаболізму, синтезу, росту та переміщення всі мертві клітини (`LifecycleState::Dead`) повністю пропускаються та не здійснюють спроб процесів.
