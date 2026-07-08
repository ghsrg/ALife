---
tags:
  - alife
  - plan
  - observer
  - classification
---

# PLAN: Аудит якості коду класифікатора архетипів організмів (Task 4)

## Мета

Провести аудит якості коду змін у гілці `feat-observer-classification` (зокрема `classify_organism_archetypes` у `src/observer/classifiers.rs` та `tests/phase2_observer_archetypes.rs`) та перевірити відповідність вимогам.

## Кроки аудиту

1. **Аналіз відповідності вимогам:**
   - Перевірка Rust-ідіом, структури полів, запозичень (borrows) та форматування.
   - Перевірка правильності обробки графових правил (розмір компоненти, час життя, стійкість зв'язків).
   - Перевірка збігу типізованих ознак (mapping) з конфігураційного файлу TOML.

2. **Розширення тестового покриття:**
   - Наразі `tests/phase2_observer_archetypes.rs` тестує лише один архетип (`stable-colony`).
   - Додати тестові кейси для `transient-cluster` та `resource-sharing-colony` для повної впевненості у коректності роботи класифікатора.

3. **Локальний запуск та перевірка:**
   - Запуск `cargo clippy --workspace --all-targets --all-features -- -D warnings`
   - Запуск `cargo test --workspace`

4. **Остаточний звіт:**
   - Надання детального звіту із зазначенням сильних сторін, знайдених зауважень та фінального вердикту (✅ APPROVED або ❌ NEEDS FIXES).
