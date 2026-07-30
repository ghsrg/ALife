---
tags:
  - alife
  - worklog/report
  - ui
---

# AL-007-S24-Fix Monitor Visual Regression Hotfix Report

## Summary

`AL-007-S24-Fix` is closed as a UI-only hotfix. The slice restores compact readable Monitor surfaces after `AL-007-S24` without adding Runner/Core/Observer contracts and without changing simulation truth.

Implemented:

- Run/Data Context scenario picker is now a dark custom listbox/popover instead of a native `<select>`.
- Scenario picker listbox is portaled to `document.body` with fixed positioning so options remain clickable outside the fixed Run/Data Context Bar bounds.
- Run metrics use `FRAME AGE` instead of `LATENCY`.
- Layers & Filters uses a compact source-backed presentation model with verbose provenance in secondary `title` metadata.
- Cell-specific layer controls render only for `Cells` and `Organisms` levels.
- Data Panel cards render chart/placeholder content first and provenance as compact chips/footer.
- Level Panel uses deterministic inline SVG icons instead of letter glyphs.
- Map selected/search affordances expose explicit foreground data attributes and stay above resource/field backgrounds.
- Playwright coverage now includes the dense `1280x720` Monitor hotfix regression.

## Scope Boundary

No `AL-007-S25` contracts were implemented. Energy Flow, Material Cycle, lineage, genome, and deeper analytics remain unavailable unless source-backed projections exist.

No Runner/Core/Observer protocol changes were made.

## Changed Files

- `ui/control-center/src/components/ScenarioPicker.tsx`
- `ui/control-center/src/components/ScenarioPicker.test.tsx`
- `ui/control-center/src/components/RunBar.tsx`
- `ui/control-center/src/components/RunBar.test.tsx`
- `ui/control-center/src/app/layerDisplayModel.ts`
- `ui/control-center/src/app/layerDisplayModel.test.ts`
- `ui/control-center/src/components/LayerPanel.tsx`
- `ui/control-center/src/components/LayerPanel.test.tsx`
- `ui/control-center/src/components/BottomDataPanel.tsx`
- `ui/control-center/src/components/BottomDataPanel.test.tsx`
- `ui/control-center/src/components/LevelPanel.tsx`
- `ui/control-center/src/components/LevelPanel.test.tsx`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/components/WorldViewer.test.tsx`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/App.test.tsx`
- `ui/control-center/src/styles/components.css`
- `ui/control-center/tests/e2e/monitor.spec.ts`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/worklog-ledger.md`

## Acceptance Matrix

| Acceptance ID | Result | Evidence |
| --- | --- | --- |
| `AL-007-S24-Fix-AC01` | Pass | `RunBar.test.tsx`, `ScenarioPicker.test.tsx`, selected Playwright hotfix test. |
| `AL-007-S24-Fix-AC02` | Pass | `layerDisplayModel.test.ts`, `LayerPanel.test.tsx`, selected Playwright hotfix test. |
| `AL-007-S24-Fix-AC03` | Pass | `BottomDataPanel.test.tsx`, `monitorSurfaceModel.test.ts`, selected Playwright hotfix test. |
| `AL-007-S24-Fix-AC04` | Pass | `LevelPanel.test.tsx`, `App.test.tsx`, selected Playwright hotfix test. |
| `AL-007-S24-Fix-AC05` | Pass | `WorldViewer.test.tsx`, selected Playwright hotfix test. |
| `AL-007-S24-Fix-AC06` | Pass | Full Vitest, production build, selected Playwright Monitor/UI visual acceptance. |

## Verification Evidence

Commands run from `ui/control-center`:

```powershell
npm.cmd test -- src/components/RunBar.test.tsx src/components/ScenarioPicker.test.tsx --run
```

Result: 2 files passed, 6 tests passed after the portal clickability regression.

```powershell
npm.cmd test -- src/app/layerDisplayModel.test.ts src/components/LayerPanel.test.tsx src/app/appState.test.ts --run
```

Result: 3 files passed, 31 tests passed.

```powershell
npm.cmd test -- src/components/BottomDataPanel.test.tsx src/app/monitorSurfaceModel.test.ts --run
```

Result: 2 files passed, 10 tests passed.

```powershell
npm.cmd test -- src/App.test.tsx src/components/LevelPanel.test.tsx --run
```

Result: 2 files passed, 21 tests passed.

```powershell
npm.cmd test -- src/components/WorldViewer.test.tsx --run
```

Result: 1 file passed, 23 tests passed.

```powershell
npm.cmd exec -- playwright test tests/e2e/monitor.spec.ts
```

Result: 9 tests passed.

```powershell
npm.cmd test -- src/components/RunBar.test.tsx src/components/ScenarioPicker.test.tsx src/components/LayerPanel.test.tsx src/components/BottomDataPanel.test.tsx src/components/LevelPanel.test.tsx src/components/WorldViewer.test.tsx --run
```

Result: 6 files passed, 41 tests passed.

```powershell
npm.cmd test -- --run
```

Result: 56 files passed, 231 tests passed.

```powershell
npm.cmd run build
```

Result: production build passed before closure and again after the ScenarioPicker portal clickability fix. Vite emitted a chunk-size warning for the main bundle; this is non-blocking and pre-existing scale-related debt.

```powershell
npm.cmd exec -- playwright test tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts
```

Result: 17 tests passed.

## Follow-Up

`AL-007-S25` remains the correct next Monitor slice for Runner/Core/Observer contracts: final Frame Age source, speed controls, Energy/Material/Resource accounting, lineage/genome/analytics payloads, and other currently unavailable source-backed surfaces.
