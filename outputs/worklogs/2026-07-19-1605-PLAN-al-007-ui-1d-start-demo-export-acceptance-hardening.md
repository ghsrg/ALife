# AL-007 UI-1D Start Demo Export Acceptance Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `UI-1D` a Start-scope demo and screenshot-export hardening slice with explicit Runner and Observer dependency boundaries.

**Architecture:** Keep all behavior in the Control Center UI layer. Use existing Runner API/stream adapters and Observer projection contracts as inputs; do not add Runner commands, Observer projection kinds, Genome views, or Debug/Research export scope. Preserve existing UI-1C viewer truthfulness and navigation behavior.

**Tech Stack:** React 19, TypeScript, Vite, Vitest, Testing Library, Playwright, Pixi renderer adapter.

---

## Plan Metadata

- Plan ID: `AL-007`
- Legacy alias: `UI-1D`
- Status: `in-progress`
- Request type: `TDD_PLAN_REQUEST`
- Primary acceptance IDs: `AL-007-AC01`, `AL-007-AC02`, `AL-007-AC03`, `AL-007-AC04`

## Source-Of-Truth Hierarchy

1. `docs/PRINCIPLES.md`
2. `docs/ui/` Canon and `docs/observer/projection-contract.md`
3. `docs/implementation/implementation-plan-ui.md`
4. `docs/implementation/implementation-plan-runner.md`
5. `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`
6. Code and tests as implementation evidence
7. Worklogs as historical evidence only

## Files Read

- `docs/delivery/execution-handoff-al-007.md`
- `docs/delivery/scenario-cards.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `docs/implementation/implementation-plan-ui.md`
- `docs/implementation/implementation-plan-runner.md`
- `docs/observer/projection-contract.md`
- `docs/observer/observer-layer.md`
- `ui/control-center/src/App.tsx`
- `ui/control-center/src/App.test.tsx`
- `ui/control-center/src/app/appState.ts`
- `ui/control-center/src/app/monitorViewModel.ts`
- `ui/control-center/src/app/runnerController.ts`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/components/ConnectionPanel.tsx`
- `ui/control-center/src/components/ConnectionPanel.test.tsx`
- `ui/control-center/src/components/MonitorWorkspace.tsx`
- `ui/control-center/src/components/MonitorWorkspace.test.tsx`
- `ui/control-center/src/components/RunControls.tsx`
- `ui/control-center/src/components/ViewerTruthOverlay.tsx`
- `ui/control-center/src/components/viewerTruth.ts`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/uiText.ts`

## Assumptions

- Existing `Export PNG` behavior is partial evidence for `AL-007-AC03`, but `AL-007` still needs Start-scope failure-state and acceptance evidence.
- Existing fixture/live state tests are reusable regression coverage for `AL-007-AC02`.
- Needs Review: exact user-visible wording may be adjusted during implementation if existing UI copy already has a clearer local convention.

## Forbidden Scope

- Do not add Runner API commands or change Runner command semantics.
- Do not add Observer projection kinds.
- Do not infer missing resource/material/process data in UI.
- Do not add CSV, JSON debug, diagnostic, research, lineage, Genome detail, or OrganismView export.
- Do not rename historical worklogs, legacy `UI-*` labels, or old indexes.

## BDD Agent Scenario Cards

### `AL-007-AC01`: Dependency Pre-Check

Given Runner owns live status and `ALIF v2` frame stream, and Observer owns read-only projection contracts.

When `UI-1D` implementation starts.

Then missing Runner or Observer behavior is recorded as a dependency or follow-up, not implemented inside UI.

### `AL-007-AC02`: Start Demo Path

Given fixture/live/unavailable states already exist in the Monitor.

When a user opens the Control Center for a Start demo.

Then the UI shows demo state, connection state, projection provenance, and unavailable data explicitly.

### `AL-007-AC03`: Screenshot Export

Given the Viewer has a current visual viewport.

When a user exports a Start demo screenshot.

Then the UI reports successful PNG export or a visible unavailable/failure state.

### `AL-007-AC04`: Acceptance Hardening

Given Start demo and export behavior exist.

When the Start slice is validated.

Then monitor layout, connection state, viewer navigation, projection truthfulness, build, and selected e2e checks remain covered.

---

## Task 1: Dependency Pre-Check Note

**Files:**

- Modify: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-007-ui-1d-start-demo-export-acceptance-hardening.md`
- Read-only: `docs/delivery/execution-handoff-al-007.md`
- Read-only: `docs/implementation/implementation-plan-runner.md`
- Read-only: `docs/observer/projection-contract.md`

- [ ] **Step 1: Record Runner boundary**

Add this entry to the eventual report under `Dependency Pre-Check`:

```markdown
| Dependency | Decision | Evidence |
| --- | --- | --- |
| `AL-002 Runner` | UI may consume current status and `ALIF v2` frames; UI must not add Runner commands. | `docs/implementation/implementation-plan-runner.md` |
```

- [ ] **Step 2: Record Observer boundary**

Add this entry to the eventual report under `Dependency Pre-Check`:

```markdown
| Dependency | Decision | Evidence |
| --- | --- | --- |
| `AL-005 Observer` | UI may render existing read-only projections; missing fields must stay explicit. | `docs/observer/projection-contract.md` |
```

- [ ] **Step 3: Evidence ID**

Reserve `AL-007-EV01` for the dependency pre-check section in the report.

---

## Task 2: RED For Start Demo Provenance

**Files:**

- Modify: `ui/control-center/src/App.test.tsx`
- Likely modify later: `ui/control-center/src/components/AppShell.tsx`
- Likely modify later: `ui/control-center/src/components/MonitorWorkspace.tsx`
- Likely modify later: `ui/control-center/src/uiText.ts`

- [ ] **Step 1: Add failing test for Start demo fixture provenance**

Add this test inside `describe('App', () => { ... })`:

```tsx
it('shows Start demo provenance without claiming fixture data is live', async () => {
  renderApp(<App />);

  await waitFor(() => {
    expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
  });

  expect(screen.getByText('Start demo')).toBeInTheDocument();
  expect(screen.getByText('Projection source: fixture')).toBeInTheDocument();
  expect(screen.getByText('Runner data: Fixture fallback - idle Runner')).toBeInTheDocument();
  expect(screen.getByText('Unavailable live fields stay unavailable')).toBeInTheDocument();
});
```

- [ ] **Step 2: Run RED**

Run from `ui/control-center/`:

```text
npm.cmd test -- src/App.test.tsx
```

Expected: FAIL because `Start demo`, `Projection source: fixture`, or `Unavailable live fields stay unavailable` is not rendered yet.

Capture this as `AL-007-EV02-RED`.

---

## Task 3: GREEN For Start Demo Provenance

**Files:**

- Modify: `ui/control-center/src/uiText.ts`
- Modify: `ui/control-center/src/components/MonitorWorkspace.tsx`

- [ ] **Step 1: Add Start demo UI text**

Add a `demo` group to `uiText`:

```ts
demo: {
  startDemo: 'Start demo',
  fixtureProjectionSource: 'Projection source: fixture',
  liveProjectionSource: 'Projection source: live',
  unavailableLiveFields: 'Unavailable live fields stay unavailable'
}
```

- [ ] **Step 2: Render provenance in `MonitorWorkspace`**

Inside the viewer toolbar metadata area, render a compact provenance block derived from existing `monitorDataState` and `state.frame.source`:

```tsx
<div className="start-demo-provenance" aria-label="Start demo provenance">
  <span>{uiText.demo.startDemo}</span>
  <span>{state.frame.source === 'live' ? uiText.demo.liveProjectionSource : uiText.demo.fixtureProjectionSource}</span>
  <span>{`Runner data: ${monitorViewModel.subtitle.includes('Runner idle') ? 'Fixture fallback - idle Runner' : monitorViewModel.subtitle}`}</span>
  <span>{uiText.demo.unavailableLiveFields}</span>
</div>
```

If this exact string construction conflicts with existing view-model conventions, move the derived strings into `buildMonitorViewModel` before rendering. Keep the test expectation unchanged unless the source docs require better wording.

- [ ] **Step 3: Run GREEN**

Run from `ui/control-center/`:

```text
npm.cmd test -- src/App.test.tsx
```

Expected: PASS for `shows Start demo provenance without claiming fixture data is live`.

Capture this as `AL-007-EV02-GREEN`.

---

## Task 4: RED For Live Start Demo Provenance

**Files:**

- Modify: `ui/control-center/src/App.test.tsx`

- [ ] **Step 1: Add failing test for live projection provenance**

Add this test inside `describe('App', () => { ... })`:

```tsx
it('shows Start demo live provenance after the first live frame', async () => {
  renderApp(<App />);

  await waitFor(() => {
    expect(mockRunner.streamInstances).toHaveLength(1);
  });

  act(() => {
    mockRunner.streamInstances[0].handlers.onStatus(runningStatus);
    mockRunner.streamInstances[0].handlers.onFrame(liveFrame({ tick: 15, sequence: 5, cellId: 1505 }));
  });

  expect(screen.getByText('Start demo')).toBeInTheDocument();
  expect(screen.getByText('Projection source: live')).toBeInTheDocument();
  expect(screen.getByText('Runner data: Live Tick 15')).toBeInTheDocument();
  expect(screen.getByLabelText('Viewer projection truth')).toHaveTextContent('Missing projection');
});
```

- [ ] **Step 2: Run RED**

Run from `ui/control-center/`:

```text
npm.cmd test -- src/App.test.tsx
```

Expected: FAIL because live-specific Start provenance is not rendered correctly yet.

Capture this as `AL-007-EV03-RED`.

---

## Task 5: GREEN For Live Start Demo Provenance

**Files:**

- Modify: `ui/control-center/src/app/monitorViewModel.ts`
- Modify: `ui/control-center/src/components/MonitorWorkspace.tsx`

- [ ] **Step 1: Add explicit Start provenance to `MonitorViewModel`**

Extend `MonitorViewModel`:

```ts
startDemo: {
  projectionSource: 'fixture' | 'live';
  runnerDataLabel: string;
  unavailableFieldsLabel: string;
}
```

Return this from `buildMonitorViewModel`:

```ts
startDemo: {
  projectionSource: state.frame.source,
  runnerDataLabel: buildFrameSubtitle(state, dataState),
  unavailableFieldsLabel: 'Unavailable live fields stay unavailable'
}
```

- [ ] **Step 2: Render from the view model**

Replace ad hoc provenance strings in `MonitorWorkspace` with:

```tsx
<div className="start-demo-provenance" aria-label="Start demo provenance">
  <span>{uiText.demo.startDemo}</span>
  <span>{`Projection source: ${monitorViewModel.startDemo.projectionSource}`}</span>
  <span>{`Runner data: ${monitorViewModel.startDemo.runnerDataLabel}`}</span>
  <span>{monitorViewModel.startDemo.unavailableFieldsLabel}</span>
</div>
```

- [ ] **Step 3: Run GREEN**

Run from `ui/control-center/`:

```text
npm.cmd test -- src/App.test.tsx
```

Expected: PASS for fixture and live Start provenance tests.

Capture this as `AL-007-EV03-GREEN`.

---

## Task 6: RED For Screenshot Export Failure State

**Files:**

- Modify: `ui/control-center/src/components/MonitorWorkspace.test.tsx`

- [ ] **Step 1: Add a focused failure-state test**

Update the existing `vi.mock('../viewer/worldRenderer', ...)` in this test file so one test can override `exportPng` to return `null`, then add:

```tsx
it('reports screenshot export as unavailable when the viewer cannot provide a PNG', async () => {
  const onExportScreenshot = vi.fn();
  const store = createAppStore();
  const state = store.getState();

  renderApp(
    <MonitorWorkspace
      state={state}
      monitorStats={buildMonitorStats(state.frame, 'fixture-offline')}
      onScenarioChange={vi.fn()}
      onReconnect={vi.fn()}
      onSelectCell={vi.fn()}
      onToggleTheme={vi.fn()}
      onExportScreenshot={onExportScreenshot}
      exportStatus={null}
    />
  );

  await waitFor(() => {
    expect(screen.getByLabelText('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');
  });

  screen.getByRole('button', { name: /export viewer png/i }).click();

  expect(onExportScreenshot).toHaveBeenCalledWith(null);
});
```

- [ ] **Step 2: Run RED**

Run from `ui/control-center/`:

```text
npm.cmd test -- src/components/MonitorWorkspace.test.tsx
```

Expected: FAIL until the mock and export path can exercise the unavailable case correctly.

Capture this as `AL-007-EV04-RED`.

---

## Task 7: GREEN For Screenshot Export Failure State

**Files:**

- Modify: `ui/control-center/src/components/MonitorWorkspace.test.tsx`
- Modify only if needed: `ui/control-center/src/components/MonitorWorkspace.tsx`

- [ ] **Step 1: Make the renderer mock configurable**

Use a hoisted renderer mock:

```tsx
const mockRenderer = vi.hoisted(() => ({
  exportPng: vi.fn(() => 'data:image/png;base64,fixture')
}));

vi.mock('../viewer/worldRenderer', () => ({
  mountWorldRenderer: vi.fn(() => Promise.resolve({
    renderFrame: vi.fn(),
    resize: vi.fn(),
    exportPng: mockRenderer.exportPng,
    destroy: vi.fn()
  }))
}));
```

Before each test, reset it:

```tsx
beforeEach(() => {
  mockRenderer.exportPng.mockReset();
  mockRenderer.exportPng.mockReturnValue('data:image/png;base64,fixture');
});
```

In the failure-state test, set:

```tsx
mockRenderer.exportPng.mockReturnValueOnce(null);
```

- [ ] **Step 2: Keep implementation minimal**

If `MonitorWorkspace` already calls `onExportScreenshot(viewerRef.current?.exportPng() ?? null)`, do not change production code for this task.

- [ ] **Step 3: Run GREEN**

Run from `ui/control-center/`:

```text
npm.cmd test -- src/components/MonitorWorkspace.test.tsx
```

Expected: PASS.

Capture this as `AL-007-EV04-GREEN`.

---

## Task 8: RED For App-Level Export Status Copy

**Files:**

- Modify: `ui/control-center/src/App.test.tsx`
- Likely modify later: `ui/control-center/src/components/AppShell.tsx`
- Likely modify later: `ui/control-center/src/uiText.ts`

- [ ] **Step 1: Add failing test for Start-scope export status**

Add this test near the existing PNG export test:

```tsx
it('uses Start-scope screenshot export status copy', async () => {
  const user = userEvent.setup();
  renderApp(<App />);

  await waitFor(() => {
    expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
  });

  await user.click(screen.getByRole('button', { name: /export viewer png/i }));

  expect(screen.getByRole('status')).toHaveTextContent('Start screenshot PNG ready');
});
```

- [ ] **Step 2: Run RED**

Run from `ui/control-center/`:

```text
npm.cmd test -- src/App.test.tsx
```

Expected: FAIL because current copy is `PNG ready (...)`.

Capture this as `AL-007-EV05-RED`.

---

## Task 9: GREEN For App-Level Export Status Copy

**Files:**

- Modify: `ui/control-center/src/uiText.ts`
- Modify: `ui/control-center/src/components/AppShell.tsx`

- [ ] **Step 1: Add export status text**

Add to `uiText.controls`:

```ts
startScreenshotReady: 'Start screenshot PNG ready',
startScreenshotUnavailable: 'Start screenshot export unavailable'
```

- [ ] **Step 2: Use Start-scope export status in `AppShell`**

Change `exportScreenshot`:

```tsx
const exportScreenshot = (png: string | null) => {
  setExportStatus(png ? `${uiText.controls.startScreenshotReady} (${png.length} bytes)` : uiText.controls.startScreenshotUnavailable);
};
```

- [ ] **Step 3: Run GREEN**

Run from `ui/control-center/`:

```text
npm.cmd test -- src/App.test.tsx
```

Expected: PASS for the Start-scope export status test and existing export test.

Capture this as `AL-007-EV05-GREEN`.

---

## Task 10: Acceptance Hardening And Delivery Evidence

**Files:**

- Modify: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-007-ui-1d-start-demo-export-acceptance-hardening.md`
- Modify: `docs/delivery/status.md`
- Modify: `docs/delivery/worklog-ledger.md`
- Modify: `outputs/worklogs/index.md`

- [ ] **Step 1: Run full UI unit/component tests**

Run from `ui/control-center/`:

```text
npm.cmd test
```

Expected: PASS.

Capture this as `AL-007-EV02`.

- [ ] **Step 2: Run production build**

Run from `ui/control-center/`:

```text
npm.cmd run build
```

Expected: PASS.

Capture this as `AL-007-EV05`.

- [ ] **Step 3: Run selected Playwright e2e**

Run from `ui/control-center/`:

```text
npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts
```

Expected: PASS.

Capture this as `AL-007-EV03` and `AL-007-EV06`.

- [ ] **Step 4: Run Runner tests only if Runner behavior was touched**

If implementation modified Runner code or live Runner protocol assumptions, run from repo root:

```text
cargo test --test runner_ws_stream --test runner_http_run_control --test runner_frame_encoder
```

Expected: PASS.

If no Runner code or protocol behavior changed, record `not run: Runner behavior not touched`.

- [ ] **Step 5: Create REPORT worklog**

Create:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-007-ui-1d-start-demo-export-acceptance-hardening.md
```

Required sections:

```markdown
# REPORT: AL-007 UI-1D Start Demo Export Acceptance Hardening

## Purpose

## Plan ID

`AL-007`

## Source Documents Read

## Changed Files Summary

## Verification Commands And Results

## Coverage Matrix

| Plan ID | Requirement | Scenario ID | Task IDs | Evidence IDs | Evidence | Status |
| --- | --- | --- | --- | --- | --- | --- |

## Deferred Scope

## Status Update Recommendation
```

- [ ] **Step 6: Update delivery status after closure verification**

Do not mark `AL-007` `done-evidenced` in `docs/delivery/status.md` until `closure-verification` confirms all P0/P1 scenarios are covered.

- [ ] **Step 7: Update indexes**

Add the REPORT worklog to:

- `outputs/worklogs/index.md`
- `docs/delivery/worklog-ledger.md`

Do not rename old worklogs or old legacy labels.

## Verification Summary

Required commands:

```text
cd ui/control-center
npm.cmd test
npm.cmd run build
npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts
```

Conditional command:

```text
cargo test --test runner_ws_stream --test runner_http_run_control --test runner_frame_encoder
```

## Approval Gate

Reply `OK EXECUTE AL-007` to authorize execution of this TDD plan.

Reply `CHANGE AL-007` with corrections to revise the plan.

Generic `OK` approves the plan content only and does not authorize execution.
