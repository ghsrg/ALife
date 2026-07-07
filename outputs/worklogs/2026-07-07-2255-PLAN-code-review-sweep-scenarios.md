# PLAN: Code Quality Review of Sweep Scenario Reference Mapping & Overrides

**Дата:** 2026-07-07  
**Роль:** Code Quality Reviewer  
**Статус:** ✅ APPROVED (з рекомендаціями до рефакторингу)

---

## 1. Сильні сторони (Strengths)

1. **Повна функціональність (Full functionality):** Реалізація повністю відповідає вимогам Task 2. Сценарії зчитуються та накладаються як базові значення, а перекриття (overrides) пріоритетно змінюють їх.
2. **Тести (Test Coverage):** Додано інтеграційні тести у [phase2_sweep_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-sweep-scenarios/tests/phase2_sweep_parser.rs), які валідують десеріалізацію TOML конфігурації та прив'язку сценаріїв до сканувань.
3. **Чистота коду (Build hygiene):** Проект успішно компілюється без попереджень. Перевірка `cargo clippy --all-targets` проходить чистим результатом.

---

## 2. Критичні зауваження (Issues - Critical)

*Критичних зауважень, які заважають роботі поточної реалізації або призводять до збоїв у тестах, не виявлено.*

---

## 3. Другорядні зауваження та рекомендації (Issues - Minor)

### 3.1 Потенційний `panic` при некоректній довжині масивів конфігурації
У структурі `RawScenarioPreset` поля `world_size` та `cell_position` визначені як `Vec<f32>`:
```rust
pub struct RawScenarioPreset {
    pub world_size: Vec<f32>,
    pub cell_position: Vec<f32>,
    ...
}
```
У функції `build_config` доступ до них здійснюється за індексами `[0]` та `[1]`:
```rust
    let (world_w, world_h) = if let Some(p) = preset {
        (p.world_size[0], p.world_size[1])
    } else { ... };
```
Якщо користувач вкаже порожній масив або масив з одного елемента в TOML, парсинг пройде успішно, але програма впаде з `panic` під час виконання.

* **Рекомендація:** Змінити типи в `RawScenarioPreset` на `[f32; 2]` або `(f32, f32)`. Серде автоматично поверне помилку десеріалізації TOML при невідповідності розміру масиву, запобігаючи паніці під час виконання.

### 3.2 Нетипова організація модулів
У [lib.rs](file:///c:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-sweep-scenarios/src/lib.rs) оголошено:
```rust
pub mod bin {
    pub mod sweep_analyzer;
}
```
Це не є ідіоматичним для Rust, оскільки бінарні файли у `src/bin/` мають компілюватися як окремі бінарні crates, а не модулі бібліотеки. Це призводить до подвійної компіляції файлу `sweep_analyzer.rs`.

* **Рекомендація:** Винести спільні типи (`RawScenarioPreset`, `AnalyzerConfig`) та функцію `build_config` у саму бібліотеку (наприклад, у `src/runner/sweep.rs`), а з бінарного файлу та тестів імпортувати їх стандартно.

### 3.3 Дублювання коду в тестах
У [phase2_sweep_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-sweep-scenarios/tests/phase2_sweep_parser.rs) повністю продубльовано опис структури `RawScenarioPreset` та `TestConfig`.

* **Рекомендація:** Імпортувати `RawScenarioPreset` з `alife::bin::sweep_analyzer::RawScenarioPreset` замість дублювання коду.

### 3.4 Дублювання коду в `build_config`
Блок коду для отримання значень з перекриттям повторюється 17 разів:
```rust
    let cell_radius_base = if let Some(p) = preset {
        p.cell_radius
    } else {
        cell_cfg.radius
    };
    let cell_radius = overrides
        .get("cell_radius")
        .copied()
        .unwrap_or(cell_radius_base);
```

* **Рекомендація:** Використати локальне замикання (closure), що скоротить функцію на ~150 рядків:
```rust
    let get_val = |key: &str, preset_val: Option<f32>, default_val: f32| -> f32 {
        overrides
            .get(key)
            .copied()
            .unwrap_or_else(|| preset_val.unwrap_or(default_val))
    };

    let cell_radius = get_val("cell_radius", preset.map(|p| p.cell_radius), cell_cfg.radius);
    let initial_energy = get_val("initial_energy", preset.map(|p| p.initial_energy), cell_cfg.initial_energy);
    // ...
```

### 3.5 Використання неідіоматичного порівняння булевих значень
У тестах використовується `#[allow(clippy::bool_assert_comparison)]` для дозволу `assert_eq!(preset.growth_enabled, true)`.

* **Рекомендація:** Використати `assert!(preset.growth_enabled)` замість `assert_eq!(..., true)`.

---

## 4. План дій з покращення коду (Fix Plan)

1. [ ] Змінити типи `world_size` та `cell_position` на `[f32; 2]` у `RawScenarioPreset`.
2. [ ] Впровадити closure `get_val` у `build_config` для очищення дублювання логіки перекриттів.
3. [ ] Усунути дублювання `RawScenarioPreset` в інтеграційних тестах.
4. [ ] Замінити `assert_eq!(..., true)` на `assert!(...)` у тестах та прибрати `#[allow(clippy::bool_assert_comparison)]`.
5. [ ] (Опціонально/Майбутнє) Винести логіку конфігурації sweep-аналізатора з `src/bin/sweep_analyzer.rs` до бібліотечного модуля, наприклад `src/runner/sweep.rs`, та прибрати `pub mod bin` з `src/lib.rs`.
