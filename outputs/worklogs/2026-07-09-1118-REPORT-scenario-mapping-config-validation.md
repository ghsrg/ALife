# Звіт: Мапінг сценаріїв та валідація конфігурації

## Опис виконаних робіт
В рамках виконання завдання було реалізовано:
1. Оновлено TOML конфігурації `config/analyzer/sweep_analyzer.toml` та `config/analyzer/sweep_analyzer_smoke.toml`:
   - Усі параметричні сканування (sweeps) та матриці (matrices) перейменовано та додано/встановлено параметри `scenario`.
   - Встановлено відповідності:
     - `viability_threshold` -> `finite_resource_viability` (`scenario = "finite_resource_viability"`)
     - `passive_income_equilibrium` -> `passive_income_survival` (`scenario = "passive_income_survival"`)
     - `upkeep_sensitivity` -> `steady_resource_flow` (`scenario = "steady_resource_flow"`)
     - `dormant_modifier` -> `dormancy_survival` (`scenario = "dormancy_survival"`)
     - `transport_metabolism` -> `resource_abundance` (`scenario = "resource_abundance"`)
2. Модифіковано `src/bin/sweep_analyzer.rs`:
   - Всі поля структури `RawScenarioPreset` обгорнуто в `Option<T>` для гнучкості пресетів.
   - Функцію `build_config` адаптовано для роботи з опціональними полями пресетів через `.and_then` / `.unwrap_or` із використанням значень за замовчуванням з базового конфігу клітини, життєвого циклу тощо.
   - У `main()` після парсингу TOML додано блок валідації конфігурації:
     - Перевіряється, чи кожен `scenario` присутній, не порожній та входить до списку 5 дозволених назв.
     - Якщо блок `scenarios` присутній у TOML, валідується наявність вказаного сценарію серед описаних пресетів `cfg.scenarios`.
     - У разі виявлення помилок виводиться повідомлення в stderr та виконується вихід з кодом 1 (`std::process::exit(1)`).
3. Додано новий інтеграційний тест `test_sweep_analyzer_invalid_scenario_validation` у `tests/phase2_sweep_observer_outputs.rs`, який запускає `sweep_analyzer` CLI з невалідною конфігурацією та перевіряє, що утиліта повертає статус помилки.
4. Проведено верифікацію за допомогою `cargo test --workspace`, всі тести успішно пройшли.
5. Зміни збережено в git-коміт.
