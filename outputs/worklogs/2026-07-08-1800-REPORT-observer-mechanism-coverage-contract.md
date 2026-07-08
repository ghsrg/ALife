# Звіт про впровадження Observer Mechanism Coverage Contract

Цей звіт підсумовує результати реалізації системи обліку покриття механізмів (Observer coverage layer) для інструменту `tools/early-stability`.

## Запуск та проходження тестів

У рамках впровадження було успішно написано та пройдено:
- **Focused Python tests** (23 тести):
  ```bash
  python -m pytest tools/early-stability/tests/test_reachability.py tools/early-stability/tests/test_reachability_writer.py tools/early-stability/tests/test_reachability_cli.py -q
  ```
  Результат: **PASS** (усі тести успішно пройшли).
- **All early-stability tests** (99 тестів):
  ```bash
  python -m pytest tools/early-stability -q
  ```
  Результат: **PASS** (усі тести успішно пройшли).
- **Rust Core tests** (111 тестів):
  ```bash
  cargo test --workspace
  ```
  Результат: **PASS** (жодних регресій у ядрі симуляції).

## Створені артефакти (Artifacts Produced)

При запуску команди `reachability` із прапорцем `--coverage` генеруються такі артефакти у вказаній директорії виводу:
1. `raw_data/mechanism_coverage.csv` — детальний звіт покриття кожного зареєстрованого механізму із зазначенням статусу, тестів, метрик та кодів попереджень.
2. `raw_data/phase_mechanism_delta.csv` — дельта-файл із переліком механізмів, категорій та фаз впровадження.
3. `raw_data/phase_test_coverage_delta.csv` — мапінг тестів та балансових сканів для кожного механізму.
4. `reports/mechanism-coverage-<timestamp>.json` — машиночитана версія статусу покриття.
5. `reports/mechanism-coverage-<timestamp>.md` — людиночитаний Markdown-звіт з таблицею покриття та попереджень.
6. `reports/phase_balance_impact.md` — звіт про вплив балансу на кожну фазу.
7. `reports/recommended-reruns-<timestamp>.md` — рекомендації щодо повторного запуску та покращення сценаріїв.

## Нова поведінка попереджень (New Warning/Status Behavior)

Система оцінює кожен механізм за наступними правилами:
- **`registered_but_disabled`**: якщо механізм не активовано (`status != "now"`).
- **`not_activated`**:
  - попередження `UNTESTED_REGISTERED_MECHANISM`: механізм увімкнений, але для нього немає результатів досяжності (reachability).
  - попередження `SCENARIO_MECHANISM_NOT_ACTIVATED`: сценарій досяжності заблокований або провалений.
- **`partially_covered`**:
  - попередження `MECHANIC_TRADEOFF_MISSING`: досяжність пройдена, але не налаштований балансовий скан (`balance_sweep`).
  - попередження `METRIC_MISSING`: досяжність пройдена, але відсутні метрики оцінки.
- **`covered`**: досяжність пройдена, і балансовий скан налаштований.

## Обмеження реалізації (Remaining Limitations)

- Поточна інтеграція є виключно діагностичним Observer-шаром на рівні інструментарію `early-stability` та не впливає на саме ядро симуляції чи конфігурацію клітин у Rust Core.
- Реєстр адаптерів завантажується з TOML файлів конфігурацій `mechanisms/*.toml`, оскільки Rust Core поки не підтримує експорт реєстру механізмів.

## Можливість продовження фази покриття (Phase 2 Coverage)

Система повністю готова для покриття механізмів Phase 2. Нові адаптери та сценарії тепер можуть бути додані безпосередньо в реєстри `mechanisms/*.toml` і будуть автоматично нормалізовані та валідовані.
