# Звіт: Впровадження розділення сценаріїв у Sweep Analyzer

## Що зроблено
1. **Зміни у `tests/phase2_sweep_parser.rs`**:
   - Додано тест `test_sweep_scenario_reference_mapping` для перевірки десеріалізації параметру `scenario` в конфігурації sweep-сканування.
2. **Зміни у `src/lib.rs`**:
   - Експортовано модуль `bin::sweep_analyzer` для доступу до його типів в інтеграційних тестах.
   - Додано `extern crate self as alife;` для дозволу локального імпортування `alife::...` усередині бібліотечних модулів.
3. **Зміни у `src/bin/sweep_analyzer.rs`**:
   - Зроблено публічними конфігураційні структури (`AnalyzerConfig`, `RunConfig`, `CellConfig`, `LifecycleRaw`, `ResourceInteractionRaw`, `EnvironmentRaw`, `SweepDef`, `MatrixDef`, `RawScenarioPreset`).
   - Додано поле `scenario: Option<String>` до `SweepDef` та `MatrixDef`.
   - Оновлено сигнатуру функції `build_config`:
     `pub fn build_config(cfg: &AnalyzerConfig, preset: Option<&RawScenarioPreset>, overrides: &std::collections::HashMap<&str, f32>) -> RuntimeConfig`
   - Реалізовано логіку перекриття параметрів у `build_config`: якщо передано сценарій (`preset` як `Some(p)`), його параметри стають базовими значеннями замість значень за замовчуванням з `cfg`. Усі передані `overrides` перекривають значення сценарію.
   - Ініціалізовано `GrowthConfig::default()`, `SynthesisConfig::default()`, `ContractilityConfig::default()`.
   - Встановлено значення `growth_enabled = preset.map(|p| p.growth_enabled).unwrap_or(false)`.
   - Оновлено `run_sweep`, `run_matrix` та `main()` для розділення сценарію за ім'ям із `cfg.scenarios` та передачі його у `build_config`.
   - Додано `#![allow(dead_code)]` для уникнення попереджень про невикористаний код під час компіляції файлу як частини бібліотеки.

## Результати перевірки
- Форматування коду перевірено за допомогою `cargo fmt --check` (успішно).
- Статичний аналіз успішно пройдено за допомогою `cargo clippy --all-targets` без попереджень та помилок.
- Усі інтеграційні тести виконано успішно за допомогою `cargo test` (включаючи новий тест `test_sweep_scenario_reference_mapping`).
