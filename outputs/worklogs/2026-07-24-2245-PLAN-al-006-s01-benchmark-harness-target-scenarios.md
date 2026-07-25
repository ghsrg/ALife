# PLAN: AL-006-S01 Benchmark Harness And Target Scenarios

**Дата:** 24 липня 2026  
**Слайс:** `AL-006-S01`  
**Назва:** Benchmark Harness And Target Scenarios  
**Залежності:** `AL-002-S09` (done), `AL-003-S03` (done)  

---

## 1. Контекст та Мета

Для безпечного проведення будь-яких оптимізацій продуктивності (`AL-006-S02` SoA / dirty regions, `AL-006-S03` parallel partitions) необхідно мати надійну, детерміновану систему вимірювання продуктивності (Benchmark Harness).

Слайс `AL-006-S01` створює цільові високонавантажені сценарії та автоматизований бенчмарк-каркас у Rust, який:
1. Вимірює виконання симуляції на **20,000 клітинах** та **20,000–40,000 з'єднаннях (Joints)**.
2. Фіксує ключові метрики: `ns/tick`, `ticks/sec`, час оновлення просторового індексу (`spatial_index_update_ns`), час перевірки фізичних контактів (`contact_resolution_ns`), час розрахунку хімії та дифузії (`chemistry_ns`).
3. Гарантує збереження детермінізму (`stable_state_hash` за однакового seed).
4. Забезпечує baseline метрики без зміни математики симуляції.

---

## 2. Критерії Прийняття (Acceptance Criteria)

- **AC-1 (Target Scenarios):** Створено сценарії та генератори масштабного масштабування (`scale_20k_cells.toml`, `scale_40k_joints.toml`), які детерміновано ініціалізують 20,000 клітин та 20,000–40,000 з'єднань.
- **AC-2 (Benchmark Suite):** Впроваджено бенчмарк-тести у `benches/core_scale_benchmarks.rs` (або `tests/scale_benchmark_harness.rs`), які вимірюють продуктивність фаз симуляції Tick без витоків пам'яті.
- **AC-3 (Metrics & Determinism Validation):** Забезпечено автоматичну перевірку `stable_state_hash` після N тіків симуляції для підтвердження того, що високе навантаження не порушує детермінізму та розраховується з високою точністю.

---

## 3. План ТТД (Test-Driven Development)

1. **Фаза 1: Сценарії Масштабування**
   - Додати конфігураційні файли сценаріїв `config/scenarios/benchmark/scale_20k_cells.toml` та `config/scenarios/benchmark/scale_40k_joints.toml`.
   - Написати інтеграційний тест `tests/scale_scenarios_smoke.rs` для перевірки ініціалізації 20k клітин та 40k з'єднань через `Bootstrap::prepare`.

2. **Фаза 2: Harness для вимірювання Tick-фаз**
   - Створити `tests/scale_benchmark_harness.rs` із чіткими вимірюваннями часу виконання 100 тіків.
   - Вивести профіль витрат часу по фазах: Spatial Index, Contact Overlap, Process Execution, Joint Stress, Chemistry.

3. **Фаза 3: Верифікація Детермінізму під навантаженням**
   - Підтвердити однаковий `stable_state_hash` при повторних прогонах з однакового seed.

---

## 4. Верифікація

- `cargo test --test scale_scenarios_smoke`
- `cargo test --test scale_benchmark_harness`
