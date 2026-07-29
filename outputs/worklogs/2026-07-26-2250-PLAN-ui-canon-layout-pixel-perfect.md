# Plan: UI Control Center Canon Layout — Pixel-Perfect 1920×1080

**Plan ID:** UI-CANON-LAYOUT  
**Date:** 2026-07-26  
**Scope:** Привести layout UI до канонічних розмірів з `control-center-block.md`.  
**Джерело правди:** `docs/ui/control-center-block.md` (розміри з PNG)

---

## Аналіз gap між spec та поточним станом

### Що реалізовано (уже є)
- AppShell з workspace tabs ✓
- RunControls (Play/Pause/Resume/Step/Stop) ✓
- LayerPanel (layers/overlays basic) ✓
- CellInspector (basic) ✓
- WorldViewer (Pixi.js canvas) ✓
- BottomDataPanel (4 cards, collapsible) ✓
- Connection/live state management ✓

### Що не відповідає spec (gap)

| Компонент | Spec | Поточний стан | Gap |
|---|---|---|---|
| **Загальний layout** | 5-column grid: 83+262+flex+335 px + 62+82+644+281 px | `monitor-grid: 220px 1fr 260px`, top-bar 48px | Неправильні розміри панелей, відсутній LevelPanel, відсутній окремий RunBar |
| **Global Navigation** | height **62px** | height **48px** | +14px |
| **Run & Data Context Bar** | height **82px**, окремий від nav | В nav одним рядком | Відсутній як окремий блок |
| **Level Panel** | width **83px**, вертикальний | Відсутній | Потрібно створити |
| **Layers Panel** | width **262px**, collapsible → 48px strip | `220px`, немає collapse | Розмір + collapse |
| **World View** | width flex (**1202px ref**), height **630px** | flex, OK | Minor |
| **Inspector** | width **335px** | `260px` | +75px |
| **Data Panel** | height **281px**, статичний | collapsible, `auto` висота | Зробити статичним 281px |
| **Rate Slider** | log scale, min=1, max=10000+unlim | `type=range min=1 max=100` linear | Log scale + range |
| **Readback disabled** | slider disabled | не реалізовано | додати disabled state |
| **Level items** | O/L/E/A — disabled dim | відсутні | — |
| **Layers collapse** | icon-only strip 48px | — | — |
| **Inspector default** | placeholder (not hidden) | OK (empty state exists) | Minor |

---

## Завдання (по компонентах)

### Т1 — CSS Layout Variables / Tokens
Оновити `tokens.css` з канонічними CSS змінними для всіх розмірів.

**Файл:** `src/styles/tokens.css`

### Т2 — Global Layout (layout.css + AppShell)
Перебудувати layout:
- `top-bar` → Global Navigation (62px)
- Новий `run-bar` (82px) під nav
- `workspace-area` = 5-column: LevelPanel(83px) + LayersPanel(262px) + Viewer(flex) + Inspector(335px)
- `bottom-panel` = 281px static

**Файли:** `src/styles/layout.css`, `src/components/AppShell.tsx`

### Т3 — LevelPanel (новий компонент)
Вертикальна панель 83×644px з 6 рівнями.
W=World (active), C=Cells, O/L/E/A — disabled dim.
Teal border-left для active.

**Файл:** `src/components/LevelPanel.tsx` (новий)

### Т4 — LayersPanel (оновити LayerPanel.tsx)
- Фіксована ширина 262px
- Collapsible: toggle → 48px icon strip
- RENDERING секція (Semantic zoom + Quality)
- OVERLAYS з правильним списком

**Файл:** `src/components/LayerPanel.tsx`

### Т5 — RunBar (новий компонент)
Окремий 82px бар під Navigation з секціями:
- 2A: Data Context (LIVE badge + RunID + scenario)
- 2B: Scenario name + config hash
- 2C: Run controls (|◄ ▶ ‖ ►| STEP 1 TICK)
- 2D: Simulation Rate (log slider, disabled readback, min=1, max=10000+unlim)
- 2E: Metrics bar (TICK FPS SEED POPULATION TOTAL_ENERGY LATENCY)

**Файл:** `src/components/RunBar.tsx` (новий)

### Т6 — GlobalNavigation (оновити AppShell nav)
- height 62px
- Logo icon + ALIFE CONTROL CENTER
- Workspace tabs center
- Right: EN, ?, ◑, WARNINGS badge (text only → dropdown)

### Т7 — InspectorPanel (оновити CellInspector)
- Ширина 335px
- Placeholder state зі spec текстом
- TRACKING badge
- DATA PROVENANCE секція (disabled до UI-2)
- FREEZE button (disabled до UI-2)
- Footer: LIVE badge + Displayed Tick

### Т8 — BottomDataPanel (оновити)
- Висота 281px статична (не collapsible за замовчуванням)
- Правильний layout 4 секції в рядок

### Т9 — CSS components.css
Всі нові стилі для нових компонентів.

---

## Порядок виконання

1. Т1 — tokens.css (canvas sizes)
2. Т2 — layout.css + AppShell структура
3. Т3 — LevelPanel (новий)
4. Т5 — RunBar (новий)
5. Т4 — LayersPanel (оновлення)
6. Т6 — GlobalNavigation (оновлення AppShell nav)
7. Т7 — InspectorPanel
8. Т8 — BottomDataPanel
9. Т9 — components.css (нові токени)
10. Verify: `npm run build` + `npm test`

---

## Верифікація
- `npm run build` — no TypeScript errors
- `npm test` — no regression failures
- Візуальна перевірка layout в браузері на 1920×1080
