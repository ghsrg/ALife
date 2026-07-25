# PLAN: AL-004-S04 Observer-Only OrganismView Projection

**Дата:** 25 липня 2026  
**Слайс:** `AL-004-S04`  
**Назва:** Observer-Only OrganismView Projection  
**Залежності:** `AL-002-S08` (done), `AL-003-S05` (done), `AL-004-S02` (done)  

---

## 1. Контекст та Мета

Для аналізу багатоклітинних організмів, їх агрегації, маси та просторової динаміки необхідний модульний Observer-проектор **`OrganismViewProjection`**.

У цьому слайсі:
1. **Зв'язні Компоненти (Connected-Component Graph)**: Організм визначається як граф зв'язності між активними клітинами, з'єднаними механічними та функціональними з'єднаннями (Joints). Окремі ізольовані клітини вважаються одноклітинними організмами.
2. **Метрики Організму (`OrganismViewPayload`)**:
   - `organism_id`: Унікальний ідентифікатор організму в поточній проекції.
   - `cell_ids`: Список індексів клітин, які входять до складу організму.
   - `primary_cell_id`: Головна/якірна клітина організму (клітина з найменшим індексом або перша за родоводним деревом).
   - `total_cells_count`: Кількість клітин у складі організму.
   - `total_mass`: Загальна маса всіх клітин у складі організму (сума матеріалів та ресурсів).
   - `total_energy`: Сумарний запас енергії клітин організму.
   - `total_joints_count`: Кількість внутрішніх з'єднань між клітинами організму.
   - `centroid_x`, `centroid_y`: Просторий центр мас організму.
3. **Строга Ізоляція Observer (Read-Only Boundary)**: Проекція `OrganismView` є **100% read-only** аналітичним зрізом і **НІКОЛИ** не повертає дані до механік Core, планувальника `TickExecutor`, перевірки `validate_feasibility` або `GenomeState`.

---

## 2. Критерії Прийняття (Acceptance Criteria)

- **AC-1 (Data Model & Structures):** У `src/observer/payloads.rs` додано типізовані структури `OrganismViewPayload` та `OrganismViewProjection`.
- **AC-2 (Connected Component Engine):** У `src/observer/organism_view.rs` реалізовано детермінований алгоритм побудови проекції `build_organism_view_projection(world: &WorldState)`, який групує клітини за з'єднаннями (Joints) та обчислює центр мас, сумарну енергію та кількість клітин.
- **AC-3 (Read-Only Boundary Guard & Tests):** Написано модуль тестування `tests/observer_organism_view.rs`, який перевіряє правильність об'єднання 2+ клітин у багатоклітинний організм, підраховує поодинокі клітини та підтверджує відсутність мутації стану `WorldState`.

---

## 3. План ТТД (Test-Driven Development)

1. **Фаза 1: Модель Даних (`src/observer/payloads.rs`)**
   - Додати `OrganismViewPayload` та `OrganismViewProjection`.
   - Забезпечити сумісність із версіованими конвертами проекцій `ProjectionCompleteness` та `ProjectionSourceMetricRef`.

2. **Фаза 2: Реалізація Організмового Аналізатора (`src/observer/organism_view.rs`)**
   - Побудувати алгоритм обходу графу зв'язності (BFS/Disjoint Set) над активними клітинами та з'єднаннями `world.joints()`.
   - Розрахувати `total_mass`, `total_energy`, `centroid_x`, `centroid_y`.

3. **Фаза 3: Інтеграційне Тестування (`tests/observer_organism_view.rs`)**
   - Написати тести для світу з поодинокими клітинами та світів із з'єднаннями (Joints).
   - Перевірити коректність обчислень центроїду та енергії.

---

## 4. Верифікація

- `cargo test --test observer_organism_view`
- `cargo fmt --check`
