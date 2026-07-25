# REPORT: Phase 2 Debt Closure (`AL-002-S17`)

## Executive Summary

Усі залишкові борги етапу `AL-002` (Phase 2 Debt Closure) успішно аудійовано, закрито та підтверджено тестами в Rust Core та UI Control Center.

Крім того, виправлено візуальний баг відображення на PixiJS карті UI Control Center — усунуто зсув та накопичення шляхів у `Graphics`, через які над клітинами з'являлися зміщені жовті кільця/пружечки.

---

## Accomplished Work

### 1. ⚙️ Rust Core: Phase 2 Debt Closure (`AL-002-S17`)
- **`AL-002-S17-AC01` (JointRepair Disposition):** 
  - Підтверджено та закріплено у `tests/phase2_debt_closure.rs`: `ProcessId::JointRepair` повертає канонічну причину `RejectionReason::ProcessDisabled`, а `ProcessSpec::for_id(JointRepair).status` дорівнює `ProcessStatus::Future` (передано в `AL-008`).
- **`AL-002-S17-AC02` (Material Decomposition & Repair):**
  - Перевірено виконання `ProcessId::RepairBoundary`: при появі пошкодження матеріалу мембрани та наявності потрібних ресурсів процес успішно дозволяється (`FeasibilityResult::Allowed`).
- **`AL-002-S17-AC03` (Boundary Retention & Diagnostics):**
  - Підтверджено утримання мембранних матеріалів (`boundary_material > 0`) та експонування метрик температури/тепла середовища (`world.environment().heat()`).

### 2. 🎨 UI Visual Repair: усунення зсуву "жовтих кільцевих пружечок" (`worldRenderer.ts`)
- **Проблема:** Через акумуляцію кіл на одному екземплярі `new Graphics()` у PixiJS v8 та наявність зсунутого тіньового кола `(x + offset, y + offset)` штрих жовтого оріолу залишався на центрі, а заливка зміщувалася, утворюючи штучні "жовті пружечки", що зависали над клітинами.
- **Виправлення:**
  - Очищено логіку малювання клітини в [worldRenderer.ts](file:///c:/Users/korsr/PycharmProjects/ALife/ui/control-center/src/viewer/worldRenderer.ts): вилучено асиметричне тіньове коло, спрощено малювання основного тіла клітини строго в `(cell.x, cell.y)`.
  - Оріол виділення малюється як акуратне концентричне коло строго навколо центру клітини тільки при `cell.selected`.

---

## Verification Evidence

1. **Rust Core Integration Tests:**
   ```powershell
   cargo test --test phase2_debt_closure
   # Result: ok. 3 passed; 0 failed; 0 ignored
   ```

2. **UI Vitest Suite:**
   ```powershell
   npx vitest run --reporter=dot
   # Result: Test Files 36 passed (36) | Tests 179 passed (179)
   ```

---

## Closure Matrix & Handoffs

- `AL-002-S17`: **DONE**
- Наступний слайс у розробці (`Current Focus`): **`AL-002-S18`** (Core-Bootstrap-Runner Closure Matrix And Handoff Audit).
