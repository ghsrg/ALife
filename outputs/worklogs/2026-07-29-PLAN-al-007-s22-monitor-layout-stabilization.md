# AL-007-S22 Monitor Layout Stabilization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` or `superpowers:subagent-driven-development`. Steps use checkbox syntax.

**Goal:** Make the Monitor grid stable, reference-aligned, and usable at `1920×1080` and `1366×862` CSS viewport without changing Runner/Core behavior.

**Architecture:** Keep the existing React/Pixi components and replace only Monitor composition, CSS track sizing, and visual-state plumbing. Reference geometry and final behavior come from `docs/ui/control-center-design-spec.md` and `docs/ui/control-center-block.md`; unavailable data remains visibly unavailable.

**Tech Stack:** React, TypeScript, Vite, PixiJS, Vitest/RTL, Playwright.

---

## Plan ID and boundaries

**Plan ID:** `AL-007-S22`  
**Status:** TDD plan proposal; not approved for execution.

**In scope:** Grid tracks, viewport behavior, root scroll, fixed Inspector/Data tracks, non-resizable panels, Map fullscreen shell, reference screenshot acceptance.

**Forbidden:** New projections, Runner/Core commands, chart semantics, Level/Layer selection behavior, data contracts, fabricated values, feature work owned by `AL-007-S23`–`S25`.

## Sources

- `docs/ui/control-center-design-spec.md`
- `docs/ui/control-center-block.md`
- `docs/ui/control-center-monitor-v3.png`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/components/MonitorWorkspace.tsx`
- `ui/control-center/src/styles/layout.css`
- `ui/control-center/src/styles/components.css`
- `ui/control-center/src/styles/tokens.css`
- `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

## Agent Scenario Cards

### AL-007-S22-AC01 — Stable reference tracks

**Given** a `1920×1080` CSS viewport, **when** Monitor opens and panels change content, **then** Navigation/Run Bar/Level/Layers/Map/Inspector/Data tracks retain their reference geometry and Map never becomes zero-sized.  
**TDD obligation:** RTL structural test plus Playwright bounding-box assertion and reference screenshot/overlay.  
**Evidence:** `AL-007-S22-EV01`, `EV02`.

### AL-007-S22-AC02 — Compact browser viewport

**Given** `1366×862` CSS viewport, **when** Monitor opens, **then** all fixed tracks remain usable; below the threshold root/page vertical scroll is used instead of collapsing or resizing a panel to zero.  
**TDD obligation:** Playwright viewport tests.  
**Evidence:** `AL-007-S22-EV03`.

### AL-007-S22-AC03 — Explicit Map fullscreen

**Given** a normal Monitor context, **when** Map fullscreen is entered, **then** Map and eligible Focus remain, standard control tracks are hidden, and optional Data Panel overlays at normal height without resizing Map.  
**TDD obligation:** RTL state test and Playwright screenshot.  
**Evidence:** `AL-007-S22-EV04`.

## Files

- Modify: `ui/control-center/src/components/AppShell.tsx`
- Modify: `ui/control-center/src/components/MonitorWorkspace.tsx`
- Modify: `ui/control-center/src/styles/tokens.css`
- Modify: `ui/control-center/src/styles/layout.css`
- Modify: `ui/control-center/src/styles/components.css`
- Modify: `ui/control-center/src/components/MonitorWorkspace.test.tsx`
- Modify: `ui/control-center/tests/e2e/monitor.spec.ts`
- Modify: `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

## Tasks

### AL-007-S22-T01: RED for AC01 grid contract

- [ ] Add `getByTestId` assertions for Navigation, Run Bar, Level, Layers, Map, Inspector, and Data Panel in `MonitorWorkspace.test.tsx`.
- [ ] Add Playwright bounding-box assertions at `1920×1080`; fail if Map width/height is `0` or Inspector/Data tracks move after panel-content state changes.
- [ ] Run `npm.cmd test -- --run src/components/MonitorWorkspace.test.tsx` and `npm.cmd run e2e -- monitor.spec.ts`; capture failures as `EV01`.

### AL-007-S22-T02: GREEN for fixed normal layout

- [ ] Define canonical CSS custom properties in `tokens.css` for `62px` navigation, `82px` run bar, `83px` Level, `262px` Layers, `335px` Inspector, and normal Data height.
- [ ] Recompose `AppShell.tsx`/`MonitorWorkspace.tsx` into fixed Grid areas without collapse handles or draggable dividers.
- [ ] Implement CSS Grid in `layout.css`; panel content changes must not alter grid-template tracks.
- [ ] Re-run T01 commands; capture pass as `EV02`.

### AL-007-S22-T03: RED/GREEN for viewport and root scroll

- [ ] Add Playwright cases at `1366×862` and `1366×861` verifying usable Map/Inspector/Data geometry and root `scrollHeight > clientHeight` only below threshold.
- [ ] Implement bounded vertical track sizing between `862` and `1080` in `layout.css`; below threshold preserve minimum grid and allow root vertical overflow.
- [ ] Assert no Data Panel-only layout scrollbar and no `display:none` Inspector fallback.
- [ ] Run `npm.cmd run e2e -- monitor.spec.ts`; capture `EV03`.

### AL-007-S22-T04: RED/GREEN for fullscreen shell

- [ ] Add RTL test for fullscreen state and Playwright test for Map-only composition.
- [ ] Add explicit fullscreen state in `MonitorWorkspace.tsx`; hide normal tracks, preserve Map viewport, and render Focus/Data as overlays only.
- [ ] Keep Data overlay at normal Data track height; do not introduce Run commands in fullscreen.
- [ ] Run focused Vitest and Playwright; capture `EV04`.

### AL-007-S22-T05: Visual acceptance and refactor

- [ ] Capture `1920×1080` and `1366×862` screenshots at device scale factor `1`.
- [ ] Compare `1920×1080` to `docs/ui/control-center-monitor-v3.png` using alpha overlay; correct the largest geometry mismatch first.
- [ ] Run `npm.cmd test`, `npm.cmd run build`, and `npm.cmd run e2e -- monitor.spec.ts ui-1c-a-visual.spec.ts`.
- [ ] Update `outputs/worklogs/index.md` and create a closure report only after all evidence exists.

## Acceptance gate

- No zero-size Map/panel after any normal panel-content transition.
- No drag-resize/collapse behavior in normal Monitor.
- `1920×1080` screenshot geometry follows reference before decorative polish.
- `1366×862` is the minimum full browser viewport; smaller heights use root scroll.
- All existing run/projection behavior is unchanged.

Reply `OK EXECUTE AL-007-S22` to authorize implementation. Reply `CHANGE AL-007-S22` with corrections to revise this plan.
