# PLAN: Phase 2 Debt Closure (`AL-002-S17`)

## Context & Motivation

Етап `AL-002` реалізував базову фізику симуляції, реєстратор процесів, ріст, ділення, смерть, розпад, хімію матеріалів, локальні взаємодії, механізми з'єднань (Joints) та інтегровану калібровку свіпів (`AL-002-S01` .. `AL-002-S09`).

Перед фінальною матрицею закриття блока `AL-002` (слайс `AL-002-S18`), слайс `AL-002-S17` має аудіювати та закрити або формально перенаправити (hand-off) наступні залишкові борги Phase 2:

1. **Material-specific decomposition rates & categories:** Аудит конверсії матеріалів та фрагментів при розпаді відмерлих клітин.
2. **Typed repair-resource & damage modeling:** Перевірка процесу `BasicRepair` та відновлення цілісності клітини.
3. **Boundary leakage/retention:** Перевірка витоку/утримання ресурсів клітинною мембраною за наявності відповідного матеріального профілю.
4. **Max local temperature metric:** Підтвердження метрики максимальної локальної температури в Observer для діагностики перегріву.
5. **JointRepair process disposition:** Формальне рішення щодо процесу `JointRepair` (зараз відхиляється через `RejectionReason::ProcessDisabled`).

## Scope & Boundaries

### In Scope
- Аудит та TDD перевірка залишкових боргів у `src/core/process.rs`, `src/core/world.rs`, `src/core/materials.rs`, `src/observer/`.
- Формалізація стану `JointRepair` (включення або канонічна відстрочка до фази з'єднань організмів `AL-008`).
- Набір integration/unit тестів у `tests/phase2_debt_closure.rs`.

### Out of Scope (Handoffs)
- Genome fragments/regulation/copying → `AL-003` (закрито).
- OrganismView authority & projections → `AL-004-S04`.
- Balance UI warnings → `AL-004-S05` / `AL-007-S12` (закрито).
- Performance & benchmark gates → `AL-006-S01`+.

## Acceptance Criteria (BDD)

- **`AL-002-S17-AC01` (JointRepair Disposition):** Процес `JointRepair` має чітко визначений канонічний статус: або виконується з валідацією витрат енергії/матеріалів, або повертає чітку причину відхилення (`ProcessDisabled` з документацією у Canon).
- **`AL-002-S17-AC02` (Material Decomposition & Repair):** Розпад відмерлих клітин зберігає загальну масу матерії та конвертує її у доступні ресурси/фрагменти; `BasicRepair` споживає матеріали та відновлює integrity клітини.
- **`AL-002-S17-AC03` (Boundary Retention & Diagnostics):** Мембрана з позитивним матеріальним профілем утримує внутрішні ресурси; метрика максимальної локальної температури коректно обраховується в Observer.

## Implementation Steps (TDD Workflow)

1. **Аналіз `JointRepair` та процесів:**
   - Перевірити `src/core/world.rs` та `src/core/process.rs` щодо `JointRepair`.
   - Визначити статус: реалізувати базовий ремонт з'єднання за наявності матеріалів або закріпити канонічне відхилення.
2. **Аналіз розпаду та ремонту клітини:**
   - Перевірити `WorldState::execute_decomposition_for_dead_cells` та `execute_basic_repair`.
3. **Створення тестів `tests/phase2_debt_closure.rs`:**
   - Покрити тестами `JointRepair`, `BasicRepair`, розпад матеріалів, утримання мембрани та метрику температури.
4. **Верифікація:**
   - Запустити `cargo test --test phase2_debt_closure`.
   - Створити підсумковий звіт `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-002-s17-phase-2-debt-closure.md`.
