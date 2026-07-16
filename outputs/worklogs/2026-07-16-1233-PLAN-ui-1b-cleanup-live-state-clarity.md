---
tags:
  - alife
  - plan
  - ui
  - tdd
  - control-center
  - ui-1b-cleanup
---

# UI-1B Cleanup Live State Clarity Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Close the carry-over UX and state clarity debt from `UI-1B` without renumbering the completed `UI-1B Live Projection Transport And Run Controls` slice.

**Architecture:** Keep the existing `AppShell`, Zustand vanilla store, Runner HTTP client, and WebSocket stream client. Add small derived-state helpers and explicit UI labels so connected/idle/live/fixture/stale states cannot be confused. Do not add new simulation data or extend `ALIF`; live resources remain unavailable until `UI-1C` or a runner projection contract plan.

**Tech Stack:** React, TypeScript, Vitest, Testing Library, PixiJS renderer, Runner HTTP and WebSocket APIs.

---

## Context And Scope

Canonical parent:

- `docs/implementation/implementation-plan-ui.md`

Relevant completed worklogs:

- `outputs/worklogs/2026-07-15-2025-PLAN-ui-1a-application-shell-fixture-viewer.md`
- `outputs/worklogs/2026-07-15-2215-PLAN-ui-1b-live-runner-transport.md`
- `outputs/worklogs/2026-07-15-2330-REPORT-ui-1b-live-runner-transport.md`
- `outputs/worklogs/2026-07-16-1233-REPORT-ui-implementation-roadmap-sync.md`

This cleanup exists because:

- `Connected` currently means HTTP bootstrap succeeded and stream is connected, not that live frames are visible.
- Idle connected state still shows the UI-1A fixture without enough explanation.
- Live resources are not streamed, while the layer checkbox says `Composite Resource Concentration`.
- `Step N` visually suggests bounded multi-step stepping, while Runner currently exposes exactly one committed Tick per step command.
- Failed initial bootstrap has no explicit reconnect path.

## Non-Goals

- Do not extend `ALIF v2`.
- Do not implement live resource grid projection.
- Do not implement semantic zoom.
- Do not redesign the Monitor layout.
- Do not add full design tokens or a design system session.
- Do not add speed/FPS controls in this cleanup slice.

## File Map

- Modify: `ui/control-center/src/app/appState.ts`
  - Add derived monitor data state helpers.
  - Keep authoritative simulation state unchanged.
- Modify: `ui/control-center/src/app/appState.test.ts`
  - Cover idle fixture, live waiting, live, and stale states.
- Modify: `ui/control-center/src/components/ConnectionPanel.tsx`
  - Display connection and monitor data state separately.
  - Display unavailable live resource projection state.
  - Add reconnect button hook.
- Modify: `ui/control-center/src/components/ConnectionPanel.test.tsx`
  - Cover state labels, resource unavailability, and reconnect action.
- Modify: `ui/control-center/src/components/RunControls.tsx`
  - Rename misleading `Step N` control to `Step 1`.
  - Keep command semantics unchanged.
- Modify: `ui/control-center/src/components/RunControls.test.tsx`
  - Update accessible names and disabled/enabled assertions.
- Modify: `ui/control-center/src/components/AppShell.tsx`
  - Wire reconnect action.
  - Use derived monitor data state in the toolbar and side panel.
  - Mark missing live resources explicitly.
- Modify: `ui/control-center/src/App.test.tsx`
  - Cover failed bootstrap -> reconnect -> connected.
  - Cover connected idle fixture explanation.
  - Cover running status before first frame as waiting for live frame.
  - Cover live frame switching to live label.
- Modify: `ui/control-center/src/styles.css`
  - Add compact status rows and warning text styles.
- Modify: `docs/implementation/implementation-plan-ui.md`
  - Mark `UI-1B-Cleanup` as the bridge cleanup before `UI-1C`.
- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1b-cleanup-live-state-clarity.md`
  - Implementation report after verification.

## Acceptance Gate

`UI-1B-Cleanup` is complete when:

```text
Connected idle state explicitly says fixture fallback is being shown.
Running/paused state before the first live frame says waiting for live frame.
Live frame state says Live Tick and live/v1 projection.
Disconnected after a live frame says stale live data, not normal live.
Resource layer says live resource projection is unavailable.
Reconnect button retries bootstrap and stream connection without repeating run commands.
Step control says Step 1 and maps to exactly one Runner StepRun command.
No browser-native alert/confirm is used.
App, state, controls, and connection tests pass.
Build passes.
docs/implementation/implementation-plan-ui.md records the cleanup bridge.
```

## Verification Commands

Use `npm.cmd`, not `npm`, on Windows.

```powershell
cd C:\Users\korsr\PycharmProjects\ALife\ui\control-center
npm.cmd test -- src/app/appState.test.ts src/components/ConnectionPanel.test.tsx src/components/RunControls.test.tsx src/App.test.tsx
npm.cmd test
npm.cmd run build
```

If Vitest fails with sandbox access to `vite.config.ts`, rerun the same command with escalation from the parent session.

---

## Task 1: Add Derived Monitor Data State

**Files:**

- Modify: `ui/control-center/src/app/appState.ts`
- Modify: `ui/control-center/src/app/appState.test.ts`

### Behavior

Add a derived helper that distinguishes connection state from displayed frame state.

Expected type:

```ts
export type MonitorDataState =
  | 'fixture-offline'
  | 'fixture-idle'
  | 'live-waiting'
  | 'live'
  | 'stale-live';
```

Expected helper:

```ts
export function getMonitorDataState(
  state: Pick<AppState, 'connectionState' | 'runStatus' | 'frame'>
): MonitorDataState {
  if (state.frame.source === 'live' && state.connectionState === 'disconnected') {
    return 'stale-live';
  }

  if (state.frame.source === 'live') {
    return 'live';
  }

  if (state.connectionState !== 'connected') {
    return 'fixture-offline';
  }

  if (
    state.runStatus?.activeRunState === 'running' ||
    state.runStatus?.activeRunState === 'paused'
  ) {
    return 'live-waiting';
  }

  return 'fixture-idle';
}
```

- [ ] **Step 1: Write failing state tests**

Add to `ui/control-center/src/app/appState.test.ts`:

```ts
import { createAppStore, getMonitorDataState } from './appState';

it('describes disconnected fixture data as offline fixture fallback', () => {
  const store = createAppStore();

  expect(getMonitorDataState(store.getState())).toBe('fixture-offline');
});

it('describes connected idle fixture data as idle fixture fallback', () => {
  const store = createAppStore();
  store.getState().setConnected(connectedInfo);
  store.getState().setRunStatus({
    processState: 'ready',
    activeRunState: 'idle',
    runId: null,
    committedTick: 0,
    scenarioId: null,
    scenarioHash: null,
    effectiveSeed: null,
    terminalReason: null
  });

  expect(getMonitorDataState(store.getState())).toBe('fixture-idle');
});

it('describes running status without a live frame as waiting for live data', () => {
  const store = createAppStore();
  store.getState().setConnected(connectedInfo);
  store.getState().setRunStatus({
    processState: 'ready',
    activeRunState: 'running',
    runId: 'run-live',
    committedTick: 9,
    scenarioId: 'demo_living_world',
    scenarioHash: 'sha256:demo',
    effectiveSeed: 42,
    terminalReason: null
  });

  expect(getMonitorDataState(store.getState())).toBe('live-waiting');
});

it('describes live frame data as live and disconnected live data as stale', () => {
  const store = createAppStore();
  store.getState().setConnected(connectedInfo);
  store.getState().setFrame({
    ...store.getState().frame,
    source: 'live',
    runId: 'run-live',
    tick: 12
  });

  expect(getMonitorDataState(store.getState())).toBe('live');

  store.getState().setConnectionState('disconnected');
  expect(getMonitorDataState(store.getState())).toBe('stale-live');
});
```

- [ ] **Step 2: Run state tests and verify RED**

Run:

```powershell
cd C:\Users\korsr\PycharmProjects\ALife\ui\control-center
npm.cmd test -- src/app/appState.test.ts
```

Expected:

```text
FAIL src/app/appState.test.ts
getMonitorDataState is not exported
```

- [ ] **Step 3: Implement derived state helper**

Modify `ui/control-center/src/app/appState.ts` with the `MonitorDataState` type and `getMonitorDataState` helper shown above.

- [ ] **Step 4: Run state tests and verify GREEN**

Run:

```powershell
npm.cmd test -- src/app/appState.test.ts
```

Expected:

```text
PASS src/app/appState.test.ts
```

- [ ] **Step 5: Commit Task 1**

```powershell
git add ui/control-center/src/app/appState.ts ui/control-center/src/app/appState.test.ts
git commit -m "test(ui): classify monitor data state"
```

---

## Task 2: Show Explicit Connection, Data, And Resource Projection State

**Files:**

- Modify: `ui/control-center/src/components/ConnectionPanel.tsx`
- Modify: `ui/control-center/src/components/ConnectionPanel.test.tsx`
- Modify: `ui/control-center/src/styles.css`

### Behavior

Connection state and displayed data state must be separate.

Labels:

```text
Runner: Connected
Data: Fixture fallback - idle Runner
Data: Waiting for first live frame
Data: Live stream
Data: Stale live frame - disconnected
Resources: Not streamed in ALIF v2
```

Add props:

```ts
import type { MonitorDataState } from '../app/appState';

export interface ConnectionPanelProps {
  endpoint: string;
  connectionState: ConnectionState;
  monitorDataState: MonitorDataState;
  serverInfo: ServerInfo | null;
  scenarios: ScenarioListItem[];
  selectedScenarioId: string | null;
  lastError: string | null;
  onScenarioChange: (scenarioId: string) => void;
  onReconnect: () => void;
}
```

Add mapping:

```ts
const dataStateLabels: Record<MonitorDataState, string> = {
  'fixture-offline': 'Fixture fallback - Runner offline',
  'fixture-idle': 'Fixture fallback - idle Runner',
  'live-waiting': 'Waiting for first live frame',
  live: 'Live stream',
  'stale-live': 'Stale live frame - disconnected'
};
```

- [ ] **Step 1: Write failing ConnectionPanel tests**

Add to `ui/control-center/src/components/ConnectionPanel.test.tsx`:

```tsx
it('separates runner connection state from displayed data state', () => {
  render(
    <ConnectionPanel
      endpoint="http://127.0.0.1:8080"
      connectionState="connected"
      monitorDataState="fixture-idle"
      serverInfo={{ engineVersion: '0.1.0', apiVersion: '1', allowRemoteViewer: false }}
      scenarios={scenarios}
      selectedScenarioId="demo"
      lastError={null}
      onScenarioChange={vi.fn()}
      onReconnect={vi.fn()}
    />
  );

  expect(screen.getByText('Runner: Connected')).toBeInTheDocument();
  expect(screen.getByText('Data: Fixture fallback - idle Runner')).toBeInTheDocument();
  expect(screen.getByText('Resources: Not streamed in ALIF v2')).toBeInTheDocument();
});

it('calls reconnect from the connection panel', async () => {
  const user = userEvent.setup();
  const onReconnect = vi.fn();

  render(
    <ConnectionPanel
      endpoint="http://127.0.0.1:8080"
      connectionState="disconnected"
      monitorDataState="fixture-offline"
      serverInfo={null}
      scenarios={[]}
      selectedScenarioId={null}
      lastError="Failed to fetch"
      onScenarioChange={vi.fn()}
      onReconnect={onReconnect}
    />
  );

  await user.click(screen.getByRole('button', { name: 'Reconnect to Runner' }));

  expect(onReconnect).toHaveBeenCalledTimes(1);
});
```

- [ ] **Step 2: Run panel tests and verify RED**

Run:

```powershell
npm.cmd test -- src/components/ConnectionPanel.test.tsx
```

Expected:

```text
FAIL src/components/ConnectionPanel.test.tsx
Property 'monitorDataState' does not exist on type 'ConnectionPanelProps'
```

- [ ] **Step 3: Implement panel labels and reconnect button**

Update the panel body to include:

```tsx
<div className="connection-status">
  <span className={`status-dot status-dot-${connectionState}`} aria-hidden="true" />
  <strong>{`Runner: ${connectionLabels[connectionState]}`}</strong>
</div>

<div className="connection-meta">
  <span>{endpoint}</span>
  <span>{serverInfo ? `API v${serverInfo.apiVersion}` : 'not connected'}</span>
  <span>{`Data: ${dataStateLabels[monitorDataState]}`}</span>
  <span>Resources: Not streamed in ALIF v2</span>
</div>

<button type="button" className="secondary-action" onClick={onReconnect} aria-label="Reconnect to Runner">
  Reconnect
</button>
```

Add styles:

```css
.secondary-action {
  margin-top: 10px;
  width: 100%;
  justify-content: center;
}
```

- [ ] **Step 4: Run panel tests and verify GREEN**

Run:

```powershell
npm.cmd test -- src/components/ConnectionPanel.test.tsx
```

Expected:

```text
PASS src/components/ConnectionPanel.test.tsx
```

- [ ] **Step 5: Commit Task 2**

```powershell
git add ui/control-center/src/components/ConnectionPanel.tsx ui/control-center/src/components/ConnectionPanel.test.tsx ui/control-center/src/styles.css
git commit -m "feat(ui): clarify runner data state"
```

---

## Task 3: Rename Misleading Step N Control To Step 1

**Files:**

- Modify: `ui/control-center/src/components/RunControls.tsx`
- Modify: `ui/control-center/src/components/RunControls.test.tsx`
- Modify: `ui/control-center/src/App.test.tsx`

### Behavior

Runner currently supports exactly one committed Tick per step command. The UI must not label that command as `Step N`.

Expected visible label:

```text
Step 1
```

Expected accessible name:

```text
Step one committed tick
```

- [ ] **Step 1: Write failing RunControls test changes**

Replace existing `Step N: one committed tick` lookups in `RunControls.test.tsx` with:

```tsx
screen.getByRole('button', { name: 'Step one committed tick' })
```

Add:

```tsx
expect(screen.getByRole('button', { name: 'Step one committed tick' })).toHaveTextContent('Step 1');
```

- [ ] **Step 2: Run controls tests and verify RED**

Run:

```powershell
npm.cmd test -- src/components/RunControls.test.tsx
```

Expected:

```text
FAIL src/components/RunControls.test.tsx
Unable to find role="button" and name "Step one committed tick"
```

- [ ] **Step 3: Implement label change**

Update `RunControls.tsx`:

```tsx
<button
  className="icon-button"
  type="button"
  aria-label="Step one committed tick"
  disabled={!canStepRun(state)}
  onClick={onStep}
>
  Step 1
</button>
```

Update `App.test.tsx` lookups if any test references the old accessible name.

- [ ] **Step 4: Run controls and app tests and verify GREEN**

Run:

```powershell
npm.cmd test -- src/components/RunControls.test.tsx src/App.test.tsx
```

Expected:

```text
PASS src/components/RunControls.test.tsx
PASS src/App.test.tsx
```

- [ ] **Step 5: Commit Task 3**

```powershell
git add ui/control-center/src/components/RunControls.tsx ui/control-center/src/components/RunControls.test.tsx ui/control-center/src/App.test.tsx
git commit -m "fix(ui): label single tick step control"
```

---

## Task 4: Wire Reconnect Without Repeating Run Commands

**Files:**

- Modify: `ui/control-center/src/components/AppShell.tsx`
- Modify: `ui/control-center/src/App.test.tsx`

### Behavior

Reconnect retries bootstrap and stream connection. It must not call `startRun`,
`pauseRun`, `resumeRun`, `stepRun`, or `stopRun`.

Implementation approach:

- Extract bootstrap logic inside `AppShell` into a local `connectRunner` callback.
- `connectRunner`:
  - creates fresh `RunnerApiClient` and `RunnerStreamClient`;
  - sets pending command to `connect`;
  - loads server info, scenarios, and run status;
  - connects the stream on success;
  - clears pending command in success and failure paths;
  - disconnects any previous stream before replacing it.
- `onReconnect` calls `connectRunner`.
- Mount effect calls `connectRunner` once and disconnects on unmount.

- [ ] **Step 1: Write failing reconnect App test**

Add to `ui/control-center/src/App.test.tsx`:

```tsx
it('reconnects after failed bootstrap without repeating run commands', async () => {
  const user = userEvent.setup();
  mockRunner.apiInstance.getServerInfo
    .mockRejectedValueOnce(new Error('Failed to fetch'))
    .mockResolvedValueOnce({
      engineVersion: '0.1.0',
      apiVersion: '1',
      allowRemoteViewer: false
    });
  mockRunner.apiInstance.listScenarios.mockResolvedValue([
    { id: 'demo-scenario', path: 'scenarios/demo.toml' }
  ]);
  mockRunner.apiInstance.getRunStatus.mockResolvedValue(idleStatus);

  renderApp(<App />);

  await waitFor(() => {
    expect(screen.getByText('Failed to fetch')).toBeInTheDocument();
  });

  await user.click(screen.getByRole('button', { name: 'Reconnect to Runner' }));

  await waitFor(() => {
    expect(screen.getByText('Runner: Connected')).toBeInTheDocument();
  });
  expect(mockRunner.streamInstances.at(-1)?.connect).toHaveBeenCalledTimes(1);
  expect(mockRunner.apiInstance.startRun).not.toHaveBeenCalled();
  expect(mockRunner.apiInstance.pauseRun).not.toHaveBeenCalled();
  expect(mockRunner.apiInstance.resumeRun).not.toHaveBeenCalled();
  expect(mockRunner.apiInstance.stepRun).not.toHaveBeenCalled();
  expect(mockRunner.apiInstance.stopRun).not.toHaveBeenCalled();
});
```

- [ ] **Step 2: Run App test and verify RED**

Run:

```powershell
npm.cmd test -- src/App.test.tsx
```

Expected:

```text
FAIL src/App.test.tsx
Unable to find role="button" and name "Reconnect to Runner"
```

- [ ] **Step 3: Implement reconnect wiring**

In `AppShell.tsx`:

- import `useCallback`;
- create `const connectRunner = useCallback(() => { ... }, [store]);`;
- move current bootstrap body from `useEffect` into `connectRunner`;
- pass `onReconnect={connectRunner}` to `ConnectionPanel`;
- keep unmount cleanup disconnecting `streamClientRef.current`.

Minimal structure:

```tsx
const connectRunner = useCallback(() => {
  const endpoint = store.getState().runnerEndpoint;
  const apiClient = new RunnerApiClient(endpoint);
  const streamClient = new RunnerStreamClient(endpoint, handlers);

  streamClientRef.current?.disconnect();
  apiClientRef.current = apiClient;
  streamClientRef.current = streamClient;
  store.getState().setPendingCommand('connect');

  void Promise.all([
    apiClient.getServerInfo(),
    apiClient.listScenarios(),
    apiClient.getRunStatus()
  ])
    .then(([serverInfo, scenarios, runStatus]) => {
      if (streamClientRef.current !== streamClient) {
        return;
      }
      const actions = store.getState();
      actions.setConnected(serverInfo);
      actions.setScenarios(scenarios);
      actions.setRunStatus(runStatus);
      actions.clearPendingCommand();
      streamClient.connect();
    })
    .catch((error: unknown) => {
      if (streamClientRef.current !== streamClient) {
        return;
      }
      const actions = store.getState();
      actions.setError(toErrorMessage(error));
      actions.setConnectionState('disconnected');
      actions.clearPendingCommand();
    });
}, [store]);
```

Keep the existing stale callback guards when moving handler code.

- [ ] **Step 4: Run App test and verify GREEN**

Run:

```powershell
npm.cmd test -- src/App.test.tsx
```

Expected:

```text
PASS src/App.test.tsx
```

- [ ] **Step 5: Commit Task 4**

```powershell
git add ui/control-center/src/components/AppShell.tsx ui/control-center/src/App.test.tsx
git commit -m "feat(ui): retry runner bootstrap"
```

---

## Task 5: Surface Waiting, Fixture, Live, And Stale States In Monitor

**Files:**

- Modify: `ui/control-center/src/components/AppShell.tsx`
- Modify: `ui/control-center/src/App.test.tsx`
- Modify: `ui/control-center/src/styles.css`

### Behavior

The viewer toolbar must not only show `Fixture Tick` or `Live Tick`; it must also explain why fixture is shown while connected.

Expected toolbar secondary labels:

```text
Fixture Tick 128 - Runner idle
Waiting for live frame - Fixture Tick 128
Live Tick 9
Stale Live Tick 9 - disconnected
```

- [ ] **Step 1: Write failing App tests for state labels**

Add to `ui/control-center/src/App.test.tsx`:

```tsx
it('explains connected idle fixture fallback', async () => {
  renderApp(<App />);

  await waitFor(() => {
    expect(screen.getByText('Data: Fixture fallback - idle Runner')).toBeInTheDocument();
  });
  expect(screen.getByText('Fixture Tick 128 - Runner idle')).toBeInTheDocument();
});

it('shows waiting for live frame when run is active before first frame', async () => {
  mockRunner.apiInstance.getRunStatus.mockResolvedValue(runningStatus);

  renderApp(<App />);

  await waitFor(() => {
    expect(screen.getByText('Data: Waiting for first live frame')).toBeInTheDocument();
  });
  expect(screen.getByText('Waiting for live frame - Fixture Tick 128')).toBeInTheDocument();
});
```

- [ ] **Step 2: Run App tests and verify RED**

Run:

```powershell
npm.cmd test -- src/App.test.tsx
```

Expected:

```text
FAIL src/App.test.tsx
Unable to find text "Fixture Tick 128 - Runner idle"
```

- [ ] **Step 3: Implement monitor label helper**

In `AppShell.tsx`, add:

```tsx
function frameSubtitle(state: AppStore) {
  const dataState = getMonitorDataState(state);
  if (dataState === 'fixture-idle') {
    return `Fixture Tick ${state.frame.tick} - Runner idle`;
  }
  if (dataState === 'live-waiting') {
    return `Waiting for live frame - Fixture Tick ${state.frame.tick}`;
  }
  if (dataState === 'stale-live') {
    return `Stale Live Tick ${state.frame.tick} - disconnected`;
  }
  return `${state.frame.source === 'live' ? 'Live' : 'Fixture'} Tick ${state.frame.tick}`;
}
```

Use it in the viewer toolbar:

```tsx
<span>{frameSubtitle(state)}</span>
```

Pass `monitorDataState={getMonitorDataState(state)}` and `onReconnect={connectRunner}` to `ConnectionPanel`.

- [ ] **Step 4: Run App tests and verify GREEN**

Run:

```powershell
npm.cmd test -- src/App.test.tsx
```

Expected:

```text
PASS src/App.test.tsx
```

- [ ] **Step 5: Commit Task 5**

```powershell
git add ui/control-center/src/components/AppShell.tsx ui/control-center/src/App.test.tsx ui/control-center/src/styles.css
git commit -m "fix(ui): explain fixture and live monitor states"
```

---

## Task 6: Document Cleanup Bridge And Produce Report

**Files:**

- Modify: `docs/implementation/implementation-plan-ui.md`
- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1b-cleanup-live-state-clarity.md`

### Behavior

Documentation must make clear that this cleanup does not reopen canonical `UI-1B`.

- [ ] **Step 1: Update implementation plan bridge note**

In `docs/implementation/implementation-plan-ui.md`, under `Current Start Slice Status`, add:

```md
`UI-1B-Cleanup` is allowed as a bridge worklog before `UI-1C`. It may fix
state clarity, reconnect behavior, and misleading labels from the completed
`UI-1B` slice, but it must not expand `ALIF`, resource projection, semantic
zoom, or Inspector scope. Those belong to `UI-1C` or later.
```

- [ ] **Step 2: Run final verification**

Run:

```powershell
cd C:\Users\korsr\PycharmProjects\ALife\ui\control-center
npm.cmd test -- src/app/appState.test.ts src/components/ConnectionPanel.test.tsx src/components/RunControls.test.tsx src/App.test.tsx
npm.cmd test
npm.cmd run build
cd C:\Users\korsr\PycharmProjects\ALife
git diff --check
```

Expected:

```text
targeted tests pass
full test suite passes
build passes
git diff --check exits 0
```

- [ ] **Step 3: Create implementation report**

Create:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1b-cleanup-live-state-clarity.md
```

Report sections:

```md
# REPORT: UI-1B Cleanup Live State Clarity

## Summary
## Changed Files
## Behavior Changes
## Tests
## Verification
## Deviations
## Remaining Work Before UI-1C
```

- [ ] **Step 4: Commit Task 6**

```powershell
git add docs/implementation/implementation-plan-ui.md outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1b-cleanup-live-state-clarity.md
git commit -m "docs(ui): report UI-1B cleanup"
```

---

## Final Review Checklist

- [ ] No production code was changed before its failing test.
- [ ] `getMonitorDataState` has direct unit coverage.
- [ ] Connection panel separates Runner connection from displayed data state.
- [ ] Resource layer unavailability is explicit.
- [ ] Reconnect retries bootstrap and does not repeat commands.
- [ ] `Step 1` label matches Runner's one-tick command.
- [ ] Fixture/live/stale toolbar labels are covered by App tests.
- [ ] Targeted tests pass.
- [ ] Full UI test suite passes.
- [ ] UI build passes.
- [ ] `docs/implementation/implementation-plan-ui.md` remains canonical.

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
- roadmap sync report: [[outputs/worklogs/2026-07-16-1233-REPORT-ui-implementation-roadmap-sync|UI Implementation Roadmap Sync]]
- canonical UI plan: [[docs/implementation/implementation-plan-ui|UI Implementation Plan]]
