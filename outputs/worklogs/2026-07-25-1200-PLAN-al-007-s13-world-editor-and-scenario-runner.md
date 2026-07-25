# PLAN: AL-007-S13 World Editor And Scenario Runner

**Дата:** 25 липня 2026  
**Слайс:** `AL-007-S13`  
**Назва:** World Editor And Scenario Runner  
**Залежності:** `AL-002-S12` (done), `AL-002-S16` (done), `AL-007-S09` (done)  

---

## 1. Контекст та Мета

Для повноцінного керування симуляцією ALife з веб-інтерфейсу (UI Control Center) необхідний робочий простір **World Editor & Scenario Runner**. Він дозволяє редагувати, перевіряти, розраховувати хеш TOML-конфігурацій та перезапускати симуляцію з обраним або новим зерном (seed) без порушення **Read-Only** ізоляції активного стану `WorldState`.

У цьому слайсі:
1. **Активація Табу World Editor**: Вмикається кнопка перемикання вкладок у шапці UI (`AppShell.tsx`), яка відкриває робочий простір `WorldEditorWorkspace.tsx`.
2. **Селектор Сценаріїв та Шаблони**: Вибір вбудованих сценаріїв (`diverse_rich_world.toml`, `demo_world_resource.toml`, `scale_20k_cells.toml`, `scale_40k_joints.toml`) або редагування власної конфігурації.
3. **Pre-Run TOML Editor & Live Validation**:
   - Інтерактивний TOML редактор із підсвіткою помилок валідації (розмір світу `size.width`/`size.height`, радіуси клітин, початкова енергія, складові матеріалів та ресурсних шарів).
   - Автоматичний розрахунок SHA-256 хекшу TOML-конфігурації (`config_hash`).
   - Автозбереження чернетки (autosave draft) у `localStorage` із можливістю відновлення скинутих змін.
4. **Перезапуск Симуляції (Scenario Runner Relaunch Controls)**:
   - Кнопка **"Relaunch Simulation with Config"**: надсилає сформований TOML та обраний seed на сервер симуляції через Runner API (`restart` / `load_scenario`).
   - Кнопка **"Relaunch with New Random Seed"**: генерує нове детерміноване зерно `seed`.
   - Гарантія безпеки: редагування конфігурації стосується **лише pre-run стану** і застосовується під час перезапуску. Мутація поточного активного `WorldState` "на льоту" суворо заборонена.

---

## 2. Критерії Прийняття (Acceptance Criteria)

- **AC-1 (UI Workspace Integration):** Перемикач робочих просторів в `AppShell.tsx` активаційно розблоковує вкладку `World Editor` та надає вибір сценаріїв.
- **AC-2 (TOML Editing, Validation & Hashing):** У `WorldEditorWorkspace.tsx` реалізовано TOML редактор, панель виявлення помилок валідації, розрахунок SHA-256 хекшу конфігурації та автозбереження чернетки.
- **AC-3 (Scenario Runner Relaunch Controls):** Реалізовано інтерфейсні кнопки перезапуску симуляції з тим самим або новим seed через `RunnerController`, що підтверджується модульним та компонентним тестуванням (Vitest).

---

## 3. План ТТД (Test-Driven Development)

1. **Фаза 1: Створення `WorldEditorWorkspace.tsx` та контролера валідації**
   - Додати `ui/control-center/src/components/WorldEditorWorkspace.tsx`.
   - Додати парсинг/валідацію TOML конфігурації у `ui/control-center/src/app/worldEditorModel.ts`.
   - Реалізувати розрахунок SHA-256 хекшу та автозбереження у `localStorage`.

2. **Фаза 2: Інтеграція в UI Shell (`AppShell.tsx`)**
   - Розблокувати вкладку `World Editor` у навігаційній панелі `AppShell.tsx`.
   - Додати перемикання між `MonitorWorkspace` та `WorldEditorWorkspace`.

3. **Фаза 3: Інтеграційне Тестування Vitest**
   - Написати `ui/control-center/src/components/WorldEditorWorkspace.test.tsx` та `ui/control-center/src/app/worldEditorModel.test.ts`.
   - Перевірити проходження Vitest та `npm run build`.

---

## 4. Верифікація

- `npx vitest run src/components/WorldEditorWorkspace.test.tsx`
- `npx vitest run src/app/worldEditorModel.test.ts`
- `npm run build`
