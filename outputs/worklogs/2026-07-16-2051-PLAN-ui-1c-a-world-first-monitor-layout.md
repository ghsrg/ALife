---
tags:
  - alife
  - worklog/plan
  - ui
  - tdd
  - ui-1c
---

# UI-1C-A World-First Monitor Layout Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Побудувати перший UI-1C slice: world-first Monitor layout, compact bottom stats strip, selected entity focus card і автоматизований visual acceptance harness без зміни Runner/Observer protocol.

**Architecture:** `AppShell` залишається власником Runner connection, run commands і frame state. Нові presentational компоненти читають тільки `WorldFrame` / `CellProjection` і `MonitorDataState`, не створюють simulation semantics і не підміняють missing live data. Playwright acceptance перевіряє композицію з `docs/ui/presentation.md#UI-1C Design Alignment`: top strip, left layers, dominant central World View, right Inspector, bottom stats strip, dark-first і light-usable.

**Tech Stack:** React 19, TypeScript, Vitest, Testing Library, Playwright, PixiJS renderer через існуючий `WorldViewer`.

---

## Scope

`UI-1C-A` закриває тільки structural/design acceptance foundation.

Входить:

- dominant central World View у Monitor;
- компактний bottom stats strip з 3-5 data-bound або unavailable values;
- small selected entity focus card біля World View;
- чесні unavailable/missing стани для live resources і невідомих alive/dead counts;
- dark-first styling, light theme remains usable;
- automated layout acceptance на `1920x1080 dark`, `1366x768 dark`, `1920x1080 light`;
- screenshot artifacts для ручної оцінки WoW/design alignment.

Не входить:

- новий Runner/ALIF payload;
- live resource heatmap, якщо projection не містить `resources`;
- semantic zoom detail renderer;
- cell overlap physics або bootstrap зміни;
- charts, resource cycles, OrganismView internals, warning center;
- full design-system token pass.

## Canonical Inputs

- `docs/PRINCIPLES.md`
- `docs/ui/presentation.md`, section `UI-1C Design Alignment`
- `docs/ui/control-center-design-spec.md`
- `docs/implementation/implementation-plan-ui.md`
- `docs/ui/control-center-monitor-v3.png`
- current UI code under `ui/control-center/src`

## Files

- Create: `ui/control-center/src/components/monitorStats.ts`
- Create: `ui/control-center/src/components/monitorStats.test.ts`
- Create: `ui/control-center/src/components/BottomStatsStrip.tsx`
- Create: `ui/control-center/src/components/BottomStatsStrip.test.tsx`
- Create: `ui/control-center/src/components/SelectedEntityFocusCard.tsx`
- Create: `ui/control-center/src/components/SelectedEntityFocusCard.test.tsx`
- Create: `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`
- Modify: `ui/control-center/src/components/AppShell.tsx`
- Modify: `ui/control-center/src/App.test.tsx`
- Modify: `ui/control-center/src/styles.css`
- Modify after implementation: `outputs/worklogs/index.md`
- Create after implementation: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-a-world-first-monitor-layout.md`

## Success Criteria

Implementation is successful when:

- unit tests prove stats are computed only from available `WorldFrame` values;
- fixture frame shows bottom stats for population, projected cell energy, world size, tick/projection and resource availability;
- live frame with empty `resources` shows `Resources: Missing projection`, not a fake resource layer;
- selected cell focus card shows ID, position, radius, energy, lifecycle/integrity when available, and explicit unavailable state when absent;
- Monitor layout contains stable landmarks for top context, left layers, central world, right inspector, bottom stats;
- Playwright verifies no incoherent overlap at `1920x1080 dark`, `1366x768 dark`, `1920x1080 light`;
- World View remains the dominant center area at desktop and 1366x768;
- `npm.cmd test`, `npm.cmd run build`, and targeted Playwright e2e pass.

## Task 1: World Stats Model

**Files:**

- Create: `ui/control-center/src/components/monitorStats.ts`
- Create: `ui/control-center/src/components/monitorStats.test.ts`

- [ ] **Step 1: Write the failing stats tests**

Create `ui/control-center/src/components/monitorStats.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import type { WorldFrame } from '../projection/types';
import { buildMonitorStats } from './monitorStats';

const frame: WorldFrame = {
  schemaVersion: 'WorldFrameProjection/v1',
  source: 'live',
  runId: 'run-live',
  scenarioName: 'demo_living_world',
  tick: 42,
  world: { width: 1200, height: 800 },
  resources: [],
  cells: [
    { id: '1', x: 10, y: 20, radius: 4, energy: 0.8, integrity: 1, generation: 0, roleHint: 'alive lifecycle state', lifecycle: 1 },
    { id: '2', x: 30, y: 40, radius: 6, energy: 0.2, integrity: 0, generation: 0, roleHint: 'dead lifecycle state', lifecycle: 2 },
    { id: '3', x: 50, y: 60, radius: 8, energy: 0.5, integrity: 1, generation: 0, roleHint: 'lifecycle unknown' }
  ],
  summary: { heat: 2.5, waste: 1.25, projectionSequence: 7, previousTick: 41, generatedAtMs: 1000 }
};

describe('buildMonitorStats', () => {
  it('summarizes only values that exist in the current WorldFrame', () => {
    const stats = buildMonitorStats(frame, 'live');

    expect(stats).toEqual([
      { id: 'cells', label: 'Cells', value: '3', state: 'available' },
      { id: 'alive-dead', label: 'Alive / Dead', value: '1 / 1', state: 'partial', note: '1 unknown' },
      { id: 'cell-energy', label: 'Projected Cell Energy', value: '1.50', state: 'available', note: 'sum of projected cell buffers' },
      { id: 'world', label: 'World', value: '1200 x 800', state: 'available' },
      { id: 'resources', label: 'Resources', value: 'Missing projection', state: 'missing', note: 'Runner ALIF v2 does not include resource grid' }
    ]);
  });

  it('does not invent alive/dead counts when lifecycle is absent', () => {
    const stats = buildMonitorStats({
      ...frame,
      source: 'fixture',
      resources: [[{ organic: 0.1, mineral: 0.2, energy: 0.3 }]],
      cells: frame.cells.map(({ lifecycle: _lifecycle, ...cell }) => cell)
    }, 'fixture-idle');

    expect(stats.find((stat) => stat.id === 'alive-dead')).toEqual({
      id: 'alive-dead',
      label: 'Alive / Dead',
      value: 'Unavailable',
      state: 'missing',
      note: 'lifecycle projection unavailable'
    });
    expect(stats.find((stat) => stat.id === 'resources')).toEqual({
      id: 'resources',
      label: 'Resources',
      value: '1 cells',
      state: 'available',
      note: 'fixture grid'
    });
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/monitorStats.test.ts
```

Expected: FAIL with module resolution error for `./monitorStats`.

- [ ] **Step 3: Implement minimal stats model**

Create `ui/control-center/src/components/monitorStats.ts`:

```ts
import type { MonitorDataState } from '../app/appState';
import type { WorldFrame } from '../projection/types';

export type MonitorStatState = 'available' | 'partial' | 'missing';

export interface MonitorStat {
  id: 'cells' | 'alive-dead' | 'cell-energy' | 'world' | 'resources';
  label: string;
  value: string;
  state: MonitorStatState;
  note?: string;
}

export function buildMonitorStats(frame: WorldFrame, _dataState: MonitorDataState): MonitorStat[] {
  const lifecycleValues = frame.cells
    .map((cell) => cell.lifecycle)
    .filter((value): value is number => typeof value === 'number');
  const alive = lifecycleValues.filter((value) => value === 1).length;
  const dead = lifecycleValues.filter((value) => value === 2).length;
  const unknown = frame.cells.length - lifecycleValues.length;
  const energy = frame.cells.reduce((sum, cell) => sum + cell.energy, 0);
  const resourceCells = frame.resources.reduce((sum, row) => sum + row.length, 0);

  return [
    { id: 'cells', label: 'Cells', value: String(frame.cells.length), state: 'available' },
    lifecycleValues.length === 0
      ? {
          id: 'alive-dead',
          label: 'Alive / Dead',
          value: 'Unavailable',
          state: 'missing',
          note: 'lifecycle projection unavailable'
        }
      : {
          id: 'alive-dead',
          label: 'Alive / Dead',
          value: `${alive} / ${dead}`,
          state: unknown > 0 ? 'partial' : 'available',
          ...(unknown > 0 ? { note: `${unknown} unknown` } : {})
        },
    {
      id: 'cell-energy',
      label: 'Projected Cell Energy',
      value: energy.toFixed(2),
      state: 'available',
      note: 'sum of projected cell buffers'
    },
    {
      id: 'world',
      label: 'World',
      value: `${frame.world.width} x ${frame.world.height}`,
      state: 'available'
    },
    resourceCells === 0
      ? {
          id: 'resources',
          label: 'Resources',
          value: 'Missing projection',
          state: 'missing',
          note: 'Runner ALIF v2 does not include resource grid'
        }
      : {
          id: 'resources',
          label: 'Resources',
          value: `${resourceCells} cells`,
          state: 'available',
          note: frame.source === 'live' ? 'live grid' : 'fixture grid'
        }
  ];
}
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/monitorStats.test.ts
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/components/monitorStats.ts ui/control-center/src/components/monitorStats.test.ts
git commit -m "feat(ui): add monitor stats model"
```

## Task 2: Bottom Stats Strip

**Files:**

- Create: `ui/control-center/src/components/BottomStatsStrip.tsx`
- Create: `ui/control-center/src/components/BottomStatsStrip.test.tsx`

- [ ] **Step 1: Write the failing component tests**

Create `ui/control-center/src/components/BottomStatsStrip.test.tsx`:

```tsx
import { screen, within } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { renderApp } from '../test/render';
import { BottomStatsStrip } from './BottomStatsStrip';
import type { MonitorStat } from './monitorStats';

const stats: MonitorStat[] = [
  { id: 'cells', label: 'Cells', value: '192', state: 'available' },
  { id: 'alive-dead', label: 'Alive / Dead', value: '180 / 12', state: 'available' },
  { id: 'cell-energy', label: 'Projected Cell Energy', value: '124.50', state: 'available', note: 'sum of projected cell buffers' },
  { id: 'world', label: 'World', value: '1200 x 800', state: 'available' },
  { id: 'resources', label: 'Resources', value: 'Missing projection', state: 'missing', note: 'Runner ALIF v2 does not include resource grid' }
];

describe('BottomStatsStrip', () => {
  it('renders compact world stats and marks missing projection explicitly', () => {
    renderApp(<BottomStatsStrip stats={stats} />);

    const strip = screen.getByLabelText('World stats');
    expect(within(strip).getByText('Cells')).toBeInTheDocument();
    expect(within(strip).getByText('192')).toBeInTheDocument();
    expect(within(strip).getByText('Resources')).toBeInTheDocument();
    expect(within(strip).getByText('Missing projection')).toBeInTheDocument();
    expect(within(strip).getByText('Runner ALIF v2 does not include resource grid')).toBeInTheDocument();
  });

  it('does not render more than five stat cells', () => {
    renderApp(<BottomStatsStrip stats={[...stats, { id: 'cells', label: 'Extra', value: '1', state: 'available' }]} />);

    expect(screen.getAllByTestId('bottom-stat')).toHaveLength(5);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/BottomStatsStrip.test.tsx
```

Expected: FAIL with module resolution error for `./BottomStatsStrip`.

- [ ] **Step 3: Implement minimal strip component**

Create `ui/control-center/src/components/BottomStatsStrip.tsx`:

```tsx
import type { MonitorStat } from './monitorStats';

interface BottomStatsStripProps {
  stats: MonitorStat[];
}

export function BottomStatsStrip({ stats }: BottomStatsStripProps) {
  return (
    <section className="bottom-stats-strip" aria-label="World stats" data-testid="bottom-stats-strip">
      {stats.slice(0, 5).map((stat) => (
        <article
          key={`${stat.id}-${stat.label}`}
          className={`bottom-stat bottom-stat-${stat.state}`}
          data-testid="bottom-stat"
        >
          <span>{stat.label}</span>
          <strong>{stat.value}</strong>
          {stat.note ? <small>{stat.note}</small> : null}
        </article>
      ))}
    </section>
  );
}
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/BottomStatsStrip.test.tsx
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/components/BottomStatsStrip.tsx ui/control-center/src/components/BottomStatsStrip.test.tsx
git commit -m "feat(ui): add bottom world stats strip"
```

## Task 3: Selected Entity Focus Card

**Files:**

- Create: `ui/control-center/src/components/SelectedEntityFocusCard.tsx`
- Create: `ui/control-center/src/components/SelectedEntityFocusCard.test.tsx`

- [ ] **Step 1: Write the failing focus card tests**

Create `ui/control-center/src/components/SelectedEntityFocusCard.test.tsx`:

```tsx
import { screen, within } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import type { CellProjection } from '../projection/types';
import { renderApp } from '../test/render';
import { SelectedEntityFocusCard } from './SelectedEntityFocusCard';

const selectedCell: CellProjection = {
  id: '42',
  x: 100,
  y: 200,
  radius: 6,
  energy: 0.75,
  integrity: 1,
  generation: 0,
  roleHint: 'alive lifecycle state',
  lifecycle: 1
};

describe('SelectedEntityFocusCard', () => {
  it('renders data-bound selected cell summary', () => {
    renderApp(<SelectedEntityFocusCard selectedCell={selectedCell} />);

    const card = screen.getByLabelText('Selected entity focus');
    expect(within(card).getByText('Cell 42')).toBeInTheDocument();
    expect(within(card).getByText('Position')).toBeInTheDocument();
    expect(within(card).getByText('100, 200')).toBeInTheDocument();
    expect(within(card).getByText('Radius')).toBeInTheDocument();
    expect(within(card).getByText('6')).toBeInTheDocument();
    expect(within(card).getByText('Energy')).toBeInTheDocument();
    expect(within(card).getByText('75%')).toBeInTheDocument();
    expect(within(card).getByText('Lifecycle')).toBeInTheDocument();
    expect(within(card).getByText('alive')).toBeInTheDocument();
  });

  it('shows unavailable lifecycle when projection omits lifecycle', () => {
    const { lifecycle: _lifecycle, ...cellWithoutLifecycle } = selectedCell;

    renderApp(<SelectedEntityFocusCard selectedCell={cellWithoutLifecycle} />);

    expect(screen.getByText('Lifecycle')).toBeInTheDocument();
    expect(screen.getByText('Unavailable')).toBeInTheDocument();
  });

  it('stays out of the layout when no cell is selected', () => {
    const { container } = renderApp(<SelectedEntityFocusCard selectedCell={null} />);

    expect(container).toBeEmptyDOMElement();
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/SelectedEntityFocusCard.test.tsx
```

Expected: FAIL with module resolution error for `./SelectedEntityFocusCard`.

- [ ] **Step 3: Implement minimal focus card**

Create `ui/control-center/src/components/SelectedEntityFocusCard.tsx`:

```tsx
import type { CellProjection } from '../projection/types';

interface SelectedEntityFocusCardProps {
  selectedCell: CellProjection | null;
}

export function SelectedEntityFocusCard({ selectedCell }: SelectedEntityFocusCardProps) {
  if (selectedCell === null) {
    return null;
  }

  return (
    <aside className="selected-focus-card" aria-label="Selected entity focus" data-testid="selected-focus-card">
      <div>
        <span>Selected</span>
        <strong>Cell {selectedCell.id}</strong>
      </div>
      <dl>
        <div>
          <dt>Position</dt>
          <dd>{Math.round(selectedCell.x)}, {Math.round(selectedCell.y)}</dd>
        </div>
        <div>
          <dt>Radius</dt>
          <dd>{Math.round(selectedCell.radius)}</dd>
        </div>
        <div>
          <dt>Energy</dt>
          <dd>{formatRatio(selectedCell.energy)}</dd>
        </div>
        <div>
          <dt>Lifecycle</dt>
          <dd>{formatLifecycle(selectedCell.lifecycle)}</dd>
        </div>
      </dl>
    </aside>
  );
}

function formatRatio(value: number) {
  return `${Math.round(value * 100)}%`;
}

function formatLifecycle(lifecycle: number | undefined) {
  if (lifecycle === 1) {
    return 'alive';
  }
  if (lifecycle === 2) {
    return 'dead';
  }
  return 'Unavailable';
}
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/SelectedEntityFocusCard.test.tsx
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/components/SelectedEntityFocusCard.tsx ui/control-center/src/components/SelectedEntityFocusCard.test.tsx
git commit -m "feat(ui): add selected entity focus card"
```

## Task 4: Compose UI-1C-A Monitor Shell

**Files:**

- Modify: `ui/control-center/src/components/AppShell.tsx`
- Modify: `ui/control-center/src/App.test.tsx`

- [ ] **Step 1: Write failing AppShell tests for the new composition**

Modify `ui/control-center/src/App.test.tsx` by adding this test inside `describe('App', () => { ... })`:

```tsx
it('renders UI-1C-A world-first Monitor landmarks with bottom stats and focus card', async () => {
  renderApp(<App />);

  await waitFor(() => {
    expect(screen.getByLabelText(/world viewer/i)).toHaveAttribute('data-ready', 'true');
  });

  expect(screen.getByTestId('monitor-top-context')).toBeInTheDocument();
  expect(screen.getByLabelText('Layer controls')).toBeInTheDocument();
  expect(screen.getByLabelText('Monitor workspace')).toBeInTheDocument();
  expect(screen.getByLabelText('Cell Inspector')).toBeInTheDocument();
  expect(screen.getByLabelText('World stats')).toBeInTheDocument();
  expect(screen.getByLabelText('Selected entity focus')).toHaveTextContent('Cell cell-a');
  expect(screen.getByText('Projected Cell Energy')).toBeInTheDocument();
  expect(screen.getByText('Resources')).toBeInTheDocument();
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/App.test.tsx -t "world-first Monitor landmarks"
```

Expected: FAIL because `monitor-top-context`, `World stats`, and `Selected entity focus` are not rendered.

- [ ] **Step 3: Wire stats strip and focus card into AppShell**

Modify imports in `ui/control-center/src/components/AppShell.tsx`:

```tsx
import { BottomStatsStrip } from './BottomStatsStrip';
import { SelectedEntityFocusCard } from './SelectedEntityFocusCard';
import { buildMonitorStats } from './monitorStats';
```

Add this inside `AppShell`, after `toggleTheme`:

```tsx
  const monitorDataState = getMonitorDataState(state);
  const monitorStats = buildMonitorStats(state.frame, monitorDataState);
```

Change the header root:

```tsx
      <header className="top-bar" data-testid="monitor-top-context">
```

Change `viewer-panel` content by inserting the focus card after `WorldViewer` and the stats strip before `exportStatus`:

```tsx
          <WorldViewer
            ref={viewerRef}
            frame={state.frame}
            selectedCellId={state.selectedCellId}
            onSelectCell={(cellId) => store.getState().selectCell(cellId)}
          />
          <SelectedEntityFocusCard selectedCell={state.selectedCell} />
          <BottomStatsStrip stats={monitorStats} />
          {exportStatus ? <p className="export-status" role="status">{exportStatus}</p> : null}
```

Change `LayerPanel` calls to reuse the local `monitorDataState`:

```tsx
          monitorDataState={monitorDataState}
```

Update `LayerPanel` signature:

```tsx
  monitorDataState,
  onScenarioChange,
  onReconnect
}: {
  state: AppStore;
  monitorDataState: ReturnType<typeof getMonitorDataState>;
  onScenarioChange: (scenarioId: string) => void;
  onReconnect: () => void;
}) {
```

Update `ConnectionPanel` prop:

```tsx
        monitorDataState={monitorDataState}
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/App.test.tsx -t "world-first Monitor landmarks"
```

Expected: PASS.

- [ ] **Step 5: Run targeted component tests**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/components/monitorStats.test.ts src/components/BottomStatsStrip.test.tsx src/components/SelectedEntityFocusCard.test.tsx src/App.test.tsx
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add ui/control-center/src/components/AppShell.tsx ui/control-center/src/App.test.tsx
git commit -m "feat(ui): compose world-first monitor shell"
```

## Task 5: Dark-First Visual Styling

**Files:**

- Modify: `ui/control-center/src/styles.css`

- [ ] **Step 1: Write failing AppShell class expectation**

Modify the `world-first Monitor landmarks` test from Task 4 by adding:

```tsx
  expect(screen.getByTestId('bottom-stats-strip')).toHaveClass('bottom-stats-strip');
  expect(screen.getByTestId('selected-focus-card')).toHaveClass('selected-focus-card');
```

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/App.test.tsx -t "world-first Monitor landmarks"
```

Expected: PASS for classes, because classes exist. This step anchors selectors before CSS changes; it is not the visual proof.

- [ ] **Step 2: Add the CSS for world-first density**

Modify `ui/control-center/src/styles.css`:

```css
.viewer-panel {
  position: relative;
  display: grid;
  grid-template-rows: auto minmax(520px, 1fr) auto auto;
  min-width: 0;
  overflow: hidden;
}

.world-viewer {
  position: relative;
  min-height: 520px;
  overflow: hidden;
}

.world-canvas-host {
  position: absolute;
  inset: 10px;
  border: 1px solid rgba(85, 184, 212, 0.24);
  border-radius: 6px;
  overflow: hidden;
  background:
    radial-gradient(circle at 18% 18%, rgba(70, 211, 191, 0.1), transparent 28%),
    radial-gradient(circle at 70% 62%, rgba(69, 139, 255, 0.12), transparent 34%),
    #061115;
  box-shadow: inset 0 0 34px rgba(69, 139, 255, 0.12);
}

.selected-focus-card {
  position: absolute;
  left: 28px;
  bottom: 92px;
  z-index: 4;
  width: min(260px, calc(100% - 56px));
  border: 1px solid rgba(116, 222, 210, 0.32);
  border-radius: 8px;
  padding: 12px;
  background: rgba(8, 19, 24, 0.86);
  box-shadow: 0 18px 42px rgba(0, 0, 0, 0.28);
  backdrop-filter: blur(10px);
}

.selected-focus-card span,
.selected-focus-card dt,
.bottom-stat span,
.bottom-stat small {
  color: #9fadb9;
  font-size: 12px;
}

.selected-focus-card strong {
  display: block;
  margin-top: 2px;
}

.selected-focus-card dl {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 8px 12px;
  margin: 10px 0 0;
}

.selected-focus-card div {
  min-width: 0;
}

.selected-focus-card dd {
  margin: 2px 0 0;
  font-weight: 700;
  overflow-wrap: anywhere;
}

.bottom-stats-strip {
  display: grid;
  grid-template-columns: repeat(5, minmax(0, 1fr));
  gap: 8px;
  padding: 10px 12px 12px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(6, 13, 17, 0.58);
}

.bottom-stat {
  min-width: 0;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 6px;
  padding: 9px 10px;
  background: rgba(255, 255, 255, 0.035);
}

.bottom-stat strong {
  display: block;
  margin-top: 3px;
  font-size: 15px;
  overflow-wrap: anywhere;
}

.bottom-stat small {
  display: block;
  margin-top: 3px;
  line-height: 1.25;
}

.bottom-stat-missing {
  border-color: rgba(240, 184, 79, 0.34);
}

:root[data-theme='light'] .world-canvas-host {
  background:
    radial-gradient(circle at 18% 18%, rgba(70, 211, 191, 0.14), transparent 28%),
    radial-gradient(circle at 70% 62%, rgba(69, 139, 255, 0.12), transparent 34%),
    #eaf2f5;
}

:root[data-theme='light'] .selected-focus-card {
  background: rgba(255, 255, 255, 0.92);
  border-color: rgba(35, 115, 140, 0.3);
}

:root[data-theme='light'] .bottom-stats-strip {
  background: #eef3f6;
  border-top-color: #d9e0e7;
}

:root[data-theme='light'] .bottom-stat {
  background: #ffffff;
  border-color: #d9e0e7;
}

@media (max-width: 1366px) {
  .top-bar {
    gap: 12px;
    padding: 10px 16px;
  }

  .monitor-grid {
    grid-template-columns: 230px minmax(520px, 1fr) 260px;
    gap: 12px;
    padding: 12px;
  }

  .viewer-panel {
    grid-template-rows: auto minmax(430px, 1fr) auto auto;
  }

  .world-viewer {
    min-height: 430px;
  }

  .bottom-stats-strip {
    grid-template-columns: repeat(5, minmax(96px, 1fr));
    overflow-x: auto;
  }
}
```

- [ ] **Step 3: Run unit tests after CSS changes**

Run:

```powershell
cd ui\control-center
npm.cmd test -- src/App.test.tsx src/components/BottomStatsStrip.test.tsx src/components/SelectedEntityFocusCard.test.tsx
```

Expected: PASS.

- [ ] **Step 4: Commit**

```powershell
git add ui/control-center/src/styles.css ui/control-center/src/App.test.tsx
git commit -m "style(ui): align monitor shell with UI-1C world-first layout"
```

## Task 6: Playwright Visual Acceptance Harness

**Files:**

- Create: `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

- [ ] **Step 1: Write failing layout acceptance tests**

Create `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`:

```ts
import { expect, test, type Page } from '@playwright/test';
import { mkdirSync } from 'node:fs';
import { join } from 'node:path';

const screenshotDir = join(process.cwd(), 'test-results', 'ui-1c-a');

test.describe('UI-1C-A visual acceptance', () => {
  test.beforeAll(() => {
    mkdirSync(screenshotDir, { recursive: true });
  });

  test('1920x1080 dark keeps World View dominant and captures acceptance screenshot', async ({ page }) => {
    await openMonitor(page, { width: 1920, height: 1080 });

    await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
    await assertWorldFirstLayout(page);
    await page.screenshot({ path: join(screenshotDir, '1920x1080-dark.png'), fullPage: true });
  });

  test('1366x768 dark keeps controls usable without incoherent overlap', async ({ page }) => {
    await openMonitor(page, { width: 1366, height: 768 });

    await assertWorldFirstLayout(page);
    await expect(page.getByRole('button', { name: 'Play live run' })).toBeVisible();
    await expect(page.getByLabel('World stats')).toBeVisible();
    await page.screenshot({ path: join(screenshotDir, '1366x768-dark.png'), fullPage: true });
  });

  test('1920x1080 light remains usable', async ({ page }) => {
    await openMonitor(page, { width: 1920, height: 1080 });

    await page.getByRole('button', { name: 'Switch to light theme' }).click();

    await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
    await assertWorldFirstLayout(page);
    await expect(page.getByLabel('Cell Inspector')).toBeVisible();
    await page.screenshot({ path: join(screenshotDir, '1920x1080-light.png'), fullPage: true });
  });
});

async function openMonitor(page: Page, viewport: { width: number; height: number }) {
  await page.setViewportSize(viewport);
  await page.goto('/');
  await expect(page.getByRole('heading', { name: 'ALife Control Center' })).toBeVisible();
  await expect(page.getByLabel('World Viewer')).toHaveAttribute('data-ready', 'true');
}

async function assertWorldFirstLayout(page: Page) {
  const layers = await page.getByLabel('Layer controls').boundingBox();
  const viewer = await page.getByLabel('Monitor workspace').boundingBox();
  const world = await page.getByLabel('World Viewer').boundingBox();
  const inspector = await page.getByLabel('Cell Inspector').boundingBox();
  const stats = await page.getByLabel('World stats').boundingBox();

  expect(layers).not.toBeNull();
  expect(viewer).not.toBeNull();
  expect(world).not.toBeNull();
  expect(inspector).not.toBeNull();
  expect(stats).not.toBeNull();

  const l = layers!;
  const v = viewer!;
  const w = world!;
  const i = inspector!;
  const s = stats!;

  expect(w.width).toBeGreaterThan(l.width);
  expect(w.width).toBeGreaterThan(i.width);
  expect(w.height).toBeGreaterThan(360);
  expect(v.x).toBeGreaterThan(l.x + l.width - 1);
  expect(i.x).toBeGreaterThan(v.x + v.width - 1);
  expect(s.y).toBeGreaterThan(w.y + w.height - 8);
}
```

- [ ] **Step 2: Run test to verify it fails on current layout**

Run:

```powershell
cd ui\control-center
npm.cmd run e2e -- tests/e2e/ui-1c-a-visual.spec.ts
```

Expected: FAIL before Task 5 is complete because `World stats` and/or new dominance constraints are absent.

- [ ] **Step 3: Run test after Task 5**

Run:

```powershell
cd ui\control-center
npm.cmd run e2e -- tests/e2e/ui-1c-a-visual.spec.ts
```

Expected: PASS and screenshots written under `ui/control-center/test-results/ui-1c-a/`.

- [ ] **Step 4: Commit**

```powershell
git add ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts
git commit -m "test(ui): add UI-1C-A visual acceptance harness"
```

## Task 7: Full Verification And Report

**Files:**

- Modify: `outputs/worklogs/index.md`
- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-a-world-first-monitor-layout.md`

- [ ] **Step 1: Run full UI verification**

Run:

```powershell
cd ui\control-center
npm.cmd test
npm.cmd run build
npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts
```

Expected: all commands PASS.

- [ ] **Step 2: Run repository diff check**

Run:

```powershell
git diff --check
```

Expected: no output.

- [ ] **Step 3: Create implementation report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-a-world-first-monitor-layout.md` with:

```md
---
tags:
  - alife
  - worklog/report
  - ui
  - ui-1c
---

# UI-1C-A World-First Monitor Layout Report

## Summary

Implemented world-first Monitor layout, compact bottom stats, selected entity focus card and visual acceptance harness.

## Changed Files

- `ui/control-center/src/components/monitorStats.ts`
- `ui/control-center/src/components/monitorStats.test.ts`
- `ui/control-center/src/components/BottomStatsStrip.tsx`
- `ui/control-center/src/components/BottomStatsStrip.test.tsx`
- `ui/control-center/src/components/SelectedEntityFocusCard.tsx`
- `ui/control-center/src/components/SelectedEntityFocusCard.test.tsx`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/App.test.tsx`
- `ui/control-center/src/styles.css`
- `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

## Verification

- `npm.cmd test`
- `npm.cmd run build`
- `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts`
- `git diff --check`

## Screenshots

- `ui/control-center/test-results/ui-1c-a/1920x1080-dark.png`
- `ui/control-center/test-results/ui-1c-a/1366x768-dark.png`
- `ui/control-center/test-results/ui-1c-a/1920x1080-light.png`

## Unresolved Issues

- Live resource grid remains unavailable until Runner/Observer projection includes it.
- Semantic zoom and renderer detail remain in the next UI-1C slice.

## Next Recommended Slice

`UI-1C-B`: projection truthfulness and renderer scale cleanup for live cells/resources.
```

- [ ] **Step 4: Register the report in worklog index**

Add the final report link under `## Reports` in `outputs/worklogs/index.md`.

- [ ] **Step 5: Commit**

```powershell
git add outputs/worklogs/index.md outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-1c-a-world-first-monitor-layout.md
git commit -m "docs(ui): report UI-1C-A monitor layout"
```

## Final Acceptance Gate

`UI-1C-A` can be marked complete only if:

- every RED test was observed failing before implementation;
- all GREEN verification commands pass;
- visual acceptance screenshots exist and are reviewed for:
  - no overlap at `1366x768`;
  - World View remains dominant;
  - dark theme is the primary WOW view;
  - light theme remains usable;
- no UI text claims live resources exist when `frame.resources` is empty;
- no new simulation behavior, Runner command, or Observer projection is introduced by this slice.

## Next Slice Recommendation

After `UI-1C-A`, move to:

```text
UI-1C-B:
Projection Truthfulness, Resource Missing State And Cell Render Scale
```

Rationale:

- `UI-1C-A` makes the shell/design measurable.
- `UI-1C-B` should address the current visible confusion: overlapping live cells, resource layer absence, and render radius truthfulness.
- Semantic zoom and richer renderer detail should wait until the scale/truthfulness rules are stable.
