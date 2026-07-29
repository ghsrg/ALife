---
plan_id: AL-007-S23
status: done
created: 2026-07-29
---

# AL-007-S23 Monitor Interaction State Report

Worklogs are evidence, not source of truth. Canonical Monitor behavior remains in `docs/ui/control-center-design-spec.md` and `docs/ui/control-center-block.md`; delivery state remains in `docs/delivery/roadmap.md` and `docs/delivery/status.md`.

## Purpose

Execute `AL-007-S23` after Monitor layout stabilization: compact the bottom Data Panel at the `1280x720` CSS baseline, make Stop visually unambiguous, move Runner connection state out of Layers & Filters, keep layers as Map presentation controls only, and implement a view-only Map fullscreen shell.

## Source documents read

- `docs/PRINCIPLES.md`
- `docs/INDEX.md`
- `docs/ui/control-center-design-spec.md`
- `docs/ui/control-center-block.md`
- `docs/implementation/implementation-plan-ui.md`
- `docs/implementation/implementation-plan-runner.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `outputs/worklogs/2026-07-29-1507-PLAN-al-007-s23-monitor-interaction-state.md`

## Changed files

- `ui/control-center/src/components/AppShell.tsx`
  - lifted Map fullscreen state to the app shell;
  - renders a fullscreen Monitor-only shell with optional Data Panel overlay;
  - routes scenario selection and reconnect into `RunBar`.
- `ui/control-center/src/components/RunBar.tsx`
  - added compact Runner connection summary, endpoint/API/data status, scenario selector, reconnect action, and last-error display;
  - replaced Stop glyph with a square stop command.
- `ui/control-center/src/components/LayerPanel.tsx`
  - removed embedded `ConnectionPanel`/Runner details;
  - kept the panel focused on map presentation controls.
- `ui/control-center/src/components/MonitorWorkspace.tsx`
  - accepts app-owned fullscreen state and exits through the same state path.
- `ui/control-center/src/components/WorldViewer.tsx`
  - changes fullscreen button aria label between enter and exit.
- `ui/control-center/src/styles/components.css`
  - compacted Data Panel card density;
  - styled RunBar Runner summary/error and scenario selector.
- `ui/control-center/src/styles/layout.css`
  - added Map-only fullscreen shell and Data Panel overlay layout.
- Tests:
  - `ui/control-center/src/components/RunBar.test.tsx`
  - `ui/control-center/src/components/LayerPanel.test.tsx`
  - `ui/control-center/tests/e2e/monitor.spec.ts`

## TDD evidence

| Evidence ID | Command / observation | Result |
| --- | --- | --- |
| `AL-007-S23-EV01` | `npm.cmd run e2e -- monitor.spec.ts --grep "AL-007-S23 keeps Data Panel compact"` before CSS fix | Failed: Data Panel had `scrollHeight 187` vs `clientHeight 186`. |
| `AL-007-S23-EV02` | Same focused e2e after Data Panel CSS fix | Passed: 1/1. |
| `AL-007-S23-EV03` | `npm.cmd test -- src/components/RunBar.test.tsx --run` before Stop glyph fix | Failed: expected `■`, received `◄|`. |
| `AL-007-S23-EV04` | Same focused RunBar test after Stop glyph fix | Passed: 1/1. |
| `AL-007-S23-EV05` | `npm.cmd test -- src/components/RunBar.test.tsx src/components/LayerPanel.test.tsx --run` before relocation | Failed: RunBar lacked `Runner: Connected`; LayerPanel still contained `Runner:` and `Reconnect`. |
| `AL-007-S23-EV06` | Same focused tests plus `npm.cmd run e2e -- monitor.spec.ts --grep "runner status"` after relocation | Passed: component 3/3, e2e 1/1. |
| `AL-007-S23-EV07` | Layer presentation regression after relocation | Component tests passed immediately after Runner cleanup; e2e initially exposed test selector assumptions, then validated geometry/selection invariants. |
| `AL-007-S23-EV08` | `npm.cmd test -- src/components/LayerPanel.test.tsx src/app/appState.test.ts --run` and `npm.cmd run e2e -- monitor.spec.ts --grep "layer toggles keep monitor geometry"` | Passed: component 22/22, e2e 1/1. |
| `AL-007-S23-EV09` | `npm.cmd run e2e -- monitor.spec.ts --grep "Map-only fullscreen"` before app-level fullscreen shell | Failed: navigation track remained visible. |
| `AL-007-S23-EV10` | Same fullscreen e2e after app-level fullscreen shell | Passed: 1/1. |
| `AL-007-S23-EV11` | Full verification commands | Passed: Vitest 51 files / 204 tests; build exit 0; selected e2e 16/16. |
| `AL-007-S23-EV12` | Delivery docs updated and deterministic traceability checked by Plan ID/AC/Evidence presence plus `git diff --check` | Passed, with only LF-to-CRLF warnings. |

## Coverage matrix

| Plan ID | Requirement | Scenario ID | Task ID(s) | Evidence ID(s) | Status |
| --- | --- | --- | --- | --- | --- |
| `AL-007-S23` | Data Panel compact at `1280x720` without panel-only scroll or Map loss | `AL-007-S23-AC01` | `AL-007-S23-T01`, `AL-007-S23-T02` | `AL-007-S23-EV01`, `AL-007-S23-EV02`, `AL-007-S23-EV11` | covered |
| `AL-007-S23` | Stop control uses square Stop glyph and existing command path | `AL-007-S23-AC02` | `AL-007-S23-T03`, `AL-007-S23-T04` | `AL-007-S23-EV03`, `AL-007-S23-EV04`, `AL-007-S23-EV11` | covered |
| `AL-007-S23` | Runner status belongs in Run/Data Context, not Layers | `AL-007-S23-AC03` | `AL-007-S23-T05`, `AL-007-S23-T06` | `AL-007-S23-EV05`, `AL-007-S23-EV06`, `AL-007-S23-EV11` | covered |
| `AL-007-S23` | Layers & Filters changes only Map presentation | `AL-007-S23-AC04` | `AL-007-S23-T07`, `AL-007-S23-T08` | `AL-007-S23-EV07`, `AL-007-S23-EV08`, `AL-007-S23-EV11` | covered |
| `AL-007-S23` | Map fullscreen is view-only and restores shell state | `AL-007-S23-AC05` | `AL-007-S23-T09`, `AL-007-S23-T10` | `AL-007-S23-EV09`, `AL-007-S23-EV10`, `AL-007-S23-EV11` | covered |

## Verification commands

```powershell
Set-Location ui/control-center
npm.cmd test -- --run
```

Result: pass, 51 test files, 204 tests.

```powershell
Set-Location ui/control-center
npm.cmd run build
```

Result: pass. Vite still reports the existing chunk-size warning for the main bundle above 500 kB.

```powershell
Set-Location ui/control-center
npm.cmd run e2e -- monitor.spec.ts ui-1c-a-visual.spec.ts
```

Result: pass, 16 Playwright tests.

## Notes

- No Core, Runner lifecycle, ALIF, or Observer contract changes were made.
- `ConnectionPanel` remains available as a standalone component and for Diagnostics coverage; it is no longer embedded in Layers & Filters.
- The optional live Runner smoke was not run because this slice is UI shell behavior and the required non-live verification passed.

## Status recommendation

Mark `AL-007-S23` as `done` with high confidence.

Next recommended delivery slice: `AL-007-S24` Source-Backed Monitor Surfaces.
