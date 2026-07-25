# PLAN: AL-005-S02 Analytics Export Foundation

**Дата:** 25 липня 2026  
**Слайс:** `AL-005-S02`  
**Назва:** Analytics Export Foundation  
**Залежності:** `AL-005-S01` (done), `AL-004-S05` (done)  

---

## 1. Контекст та Мета

Після завершення `AL-005-S01` (индексація метаданих прогонів у SQLite) та `AL-004-S05` (Observer-пейлоади балансу, покриття та класифікації), для аналізу тривалих еволюційних симуляцій необхідний **експорт аналітичних даних** (Analytics Export Foundation).

Слайс `AL-005-S02` створює типізовану систему експорту аналітичних зрізів симуляції у формати, зручні для Python/DuckDB/Polars/Pandas (JSON / CSV / Parquet-Friendly structure):
1. **Збереження Provenance & Observability**: Кожен експорт містить метадані прогону (`run_id`, `scenario_id`, `config_hash`, `seed`, `schema_version`, `completeness_status`, `warning_codes`).
2. **Аналітичні Зрізи (Analytics Datasets)**:
   - **Population Summary**: Динаміка популяції (живі, стресовані, мертві), народжуваність, смертність по тіках.
   - **Matter & Energy Balance**: Збереження маси, споживання енергії, неурахована різниця (`unaccounted_difference`).
   - **Lineage & Evolution Summary**: Загальна кількість ліній, частота мутацій, успадкування геномів.
   - **Environment & Waste Dynamics**: Накопичення тепла, відходів, розсіювання по епохах.
3. **Строге обмеження (Observer Boundary)**: Експортовані аналітичні файли є **виключно read-only** для аналізу і **НІКОЛИ** не стають вхідними даними для механік Core чи джерелом правди для повторного відтворення.

---

## 2. Критерії Прийняття (Acceptance Criteria)

- **AC-1 (Analytics Export Data Model):** Визначено типізований контракт `AnalyticsExportManifest` та пакети даних `PopulationAnalyticsRow`, `BalanceAnalyticsRow`, `LineageAnalyticsRow`, `EnvironmentAnalyticsRow`.
- **AC-2 (Export Serializer & Exporter):** Впроваджено модулі експорту `AnalyticsExporter` (JSON/CSV) у `src/storage/analytics_export.rs`, які підтримують повний запис аналітичного зрізу прогону із включенням версії схеми та кодувань застережень.
- **AC-3 (Observer Boundary Guard & Validation):** Написано інтеграційні тести у `tests/storage_analytics_export.rs`, які перевіряють повноту даних, валідність JSON/CSV структур та підтверджують відсутність зворотного впливу експорту на стан симуляції `WorldState`.

---

## 3. План ТТД (Test-Driven Development)

1. **Фаза 1: Модель Даних Аналітичних Експортів (`src/storage/analytics_export.rs`)**
   - Додати типізовані структури для рядків аналітики популяції, балансу речовини/енергії, родоводу та середовища.
   - Забезпечити обов'язкові поля `schema_version`, `run_id`, `config_hash`, `completeness`.

2. **Фаза 2: Реалізація Експортера (`AnalyticsExporter`)**
   - Реалізувати створення та експорт у файли/рядки JSON та CSV.
   - Додати підтримку форматування для аналізу в Python/DuckDB.

3. **Фаза 3: Інтеграційне Тестування (`tests/storage_analytics_export.rs`)**
   - Перевірити генерацію експорту з реального `RunSummary` / `ObserverProjectionSummary`.
   - Перевірити відповідність полів та неможливість мутації `WorldState` аналітичним модулем.

---

## 4. Верифікація

- `cargo test --test storage_analytics_export`
- `cargo fmt --check`
