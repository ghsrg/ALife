# UI-1A Application Shell And Deterministic Fixture Viewer Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. REQUIRED SUB-SKILL for every production change: use superpowers:test-driven-development. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first runnable `ALife Control Center` UI slice: a Chromium/Vite React shell, deterministic fixture adapter, PixiJS World Viewer, selection-linked Inspector, Light/Dark themes, and screenshot export.

**Architecture:** Create a new frontend workspace at `ui/control-center` without touching Core simulation behavior. Keep authoritative data in deterministic fixture/projection modules, keep React for shell/panels, and keep PixiJS inside a focused Viewer boundary. UI-1A is fixture-only; live Runner transport is deferred to UI-1B.

**Tech Stack:** React 19, TypeScript, Vite, PixiJS 8, Zustand, CSS Modules/global CSS variables, Vitest, React Testing Library, Playwright, npm lockfile.

---

## Canon And Scope Sources

Read before implementation:

- `docs/PRINCIPLES.md`
- `docs/ui/INDEX.md`
- `docs/ui/architecture.md`
- `docs/ui/navigation.md`
- `docs/ui/visualization.md`
- `docs/ui/presentation.md`
- `docs/ui/interaction.md`
- `docs/ui/quality.md`
- `docs/implementation/implementation-plan-ui.md`
- `docs/implementation/ui-technology-stack.md`
- `docs/observer/projection-contract.md`
- `docs/ui/control-center-monitor-v3.png`

## Scope

Build only `UI-1A`.

In scope:

- Vite React application under `ui/control-center`.
- Monitor workspace only.
- Deterministic fixture frame and fixture adapter.
- Application shell inspired by `docs/ui/control-center-monitor-v3.png`.
- Top navigation with future workspaces disabled or inert.
- Left layer panel.
- Central PixiJS viewer drawing world bounds, resource heatmap, and cells.
- Zoom, pan, reset viewport, full-screen state.
- Single Cell selection and Inspector.
- Light/Dark theme toggle.
- Screenshot export for current viewport.
- Unit, component, Playwright smoke, and basic performance smoke.

Out of scope:

- Live WebSocket/HTTP Runner connection.
- Real ALIF frame decoding.
- OrganismView detail.
- Rich analytics.
- World Editor.
- Experiment runner.
- Remote viewer mode.
- Step N behavior beyond disabled placeholder.
- Any Core/Runner behavior changes.

## File Structure

Create:

```text
ui/control-center/
  package.json
  package-lock.json
  index.html
  tsconfig.json
  tsconfig.node.json
  vite.config.ts
  vitest.setup.ts
  playwright.config.ts
  src/
    main.tsx
    App.tsx
    app/AppShell.tsx
    app/AppShell.test.tsx
    app/appState.ts
    app/appState.test.ts
    components/BottomDataPanel.tsx
    components/InspectorPanel.tsx
    components/LayerPanel.tsx
    components/RunControls.tsx
    components/ThemeToggle.tsx
    fixtures/ui1aFixture.ts
    fixtures/ui1aFixture.test.ts
    projection/types.ts
    projection/fixtureAdapter.ts
    projection/fixtureAdapter.test.ts
    renderer/viewport.ts
    renderer/viewport.test.ts
    renderer/worldRenderer.ts
    renderer/WorldViewer.tsx
    renderer/WorldViewer.test.tsx
    styles/tokens.css
    styles/app.css
    styles/panels.css
    test/render.tsx
  e2e/
    ui1a-monitor.spec.ts
```

Modify:

```text
.gitignore
outputs/worklogs/index.md
```

Do not modify:

```text
src/core/**
src/runner/**
src/viewer_server/**
tests/runner_*.rs
```

## Task 0: Baseline, Branch, And Toolchain Check

**Files:**

- No source changes.

- [ ] **Step 1: Confirm worktree state**

Run:

```powershell
git status --short --branch
```

Expected:

```text
## main
```

If existing modified docs from prior planning are present, record them in the eventual report and do not revert them.

- [ ] **Step 2: Check Node and npm**

Run:

```powershell
node --version
npm --version
```

Expected:

```text
v20.x.x or newer
10.x.x or newer
```

If Node is missing or too old, stop and ask the user to install Node 20+ before implementing UI-1A.

- [ ] **Step 3: Confirm runner-facing docs are synced**

Run:

```powershell
rg -n "WorldFrameProjection v1|ALIF v1" docs\implementation docs\runner
```

Expected:

```text
no output
```

- [ ] **Step 4: Commit only if requested**

Do not commit existing unrelated changes. UI-1A implementation tasks below include explicit commit points for their own changes.

## Task 1: Scaffold Vite React Workspace And Test Harness

**Files:**

- Create: `ui/control-center/package.json`
- Create: `ui/control-center/index.html`
- Create: `ui/control-center/tsconfig.json`
- Create: `ui/control-center/tsconfig.node.json`
- Create: `ui/control-center/vite.config.ts`
- Create: `ui/control-center/vitest.setup.ts`
- Create: `ui/control-center/playwright.config.ts`
- Create: `ui/control-center/src/main.tsx`
- Create: `ui/control-center/src/App.tsx`
- Create: `ui/control-center/src/test/render.tsx`
- Modify: `.gitignore`
- Test: `ui/control-center/src/App.test.tsx`

- [ ] **Step 1: Write the first failing smoke test**

Create `ui/control-center/src/App.test.tsx`:

```tsx
import { screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { renderWithProviders } from './test/render';
import { App } from './App';

describe('App', () => {
  it('renders the ALife Control Center shell title', () => {
    renderWithProviders(<App />);

    expect(
      screen.getByRole('heading', { name: /ALife Control Center/i }),
    ).toBeInTheDocument();
  });
});
```

- [ ] **Step 2: Create minimal package and config files**

Create `ui/control-center/package.json`:

```json
{
  "name": "@alife/control-center",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite --host 127.0.0.1",
    "build": "tsc -b && vite build",
    "test": "vitest run",
    "test:watch": "vitest",
    "e2e": "playwright test",
    "lint": "tsc -b --pretty false"
  },
  "dependencies": {
    "@vitejs/plugin-react": "^4.3.4",
    "pixi.js": "^8.6.6",
    "react": "^19.0.0",
    "react-dom": "^19.0.0",
    "zustand": "^5.0.3"
  },
  "devDependencies": {
    "@playwright/test": "^1.49.1",
    "@testing-library/jest-dom": "^6.6.3",
    "@testing-library/react": "^16.1.0",
    "@testing-library/user-event": "^14.5.2",
    "@types/react": "^19.0.2",
    "@types/react-dom": "^19.0.2",
    "jsdom": "^25.0.1",
    "typescript": "^5.7.2",
    "vite": "^6.0.6",
    "vitest": "^2.1.8"
  }
}
```

Create `ui/control-center/index.html`:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>ALife Control Center</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
```

Create `ui/control-center/tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "useDefineForClassFields": true,
    "lib": ["DOM", "DOM.Iterable", "ES2022"],
    "allowJs": false,
    "skipLibCheck": true,
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx"
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

Create `ui/control-center/tsconfig.node.json`:

```json
{
  "compilerOptions": {
    "composite": true,
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "allowSyntheticDefaultImports": true,
    "strict": true
  },
  "include": ["vite.config.ts", "playwright.config.ts"]
}
```

Create `ui/control-center/vite.config.ts`:

```ts
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: './vitest.setup.ts',
  },
});
```

Create `ui/control-center/vitest.setup.ts`:

```ts
import '@testing-library/jest-dom/vitest';
```

Create `ui/control-center/playwright.config.ts`:

```ts
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  timeout: 30_000,
  use: {
    baseURL: 'http://127.0.0.1:5173',
    trace: 'retain-on-failure',
  },
  webServer: {
    command: 'npm run dev -- --port 5173',
    url: 'http://127.0.0.1:5173',
    reuseExistingServer: !process.env.CI,
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
});
```

Create `ui/control-center/src/test/render.tsx`:

```tsx
import { ReactElement } from 'react';
import { render } from '@testing-library/react';

export function renderWithProviders(ui: ReactElement) {
  return render(ui);
}
```

Create `ui/control-center/src/App.tsx`:

```tsx
export function App() {
  return (
    <main>
      <h1>ALife Control Center</h1>
    </main>
  );
}
```

Create `ui/control-center/src/main.tsx`:

```tsx
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from './App';

createRoot(document.getElementById('root') as HTMLElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
```

Modify `.gitignore`:

```gitignore
ui/control-center/node_modules/
ui/control-center/dist/
ui/control-center/test-results/
ui/control-center/playwright-report/
```

- [ ] **Step 3: Install dependencies**

Run:

```powershell
Set-Location ui\control-center
npm install
```

Expected:

```text
added ... packages
found 0 vulnerabilities
```

If network is blocked, rerun with sandbox escalation and ask for approval to download npm dependencies.

- [ ] **Step 4: Verify the first test passes**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/App.test.tsx
```

Expected:

```text
1 passed
```

This first test may not have a clean RED phase because it validates the scaffold after package setup. All behavior tasks below must follow strict RED/GREEN.

- [ ] **Step 5: Build check**

Run:

```powershell
Set-Location ui\control-center
npm run build
```

Expected:

```text
vite build ... built
```

- [ ] **Step 6: Commit**

Run:

```powershell
git add .gitignore ui/control-center
git commit -m "chore(ui): scaffold control center workspace"
```

## Task 2: Define Projection Types And Deterministic Fixture

**Files:**

- Create: `ui/control-center/src/projection/types.ts`
- Create: `ui/control-center/src/fixtures/ui1aFixture.ts`
- Test: `ui/control-center/src/fixtures/ui1aFixture.test.ts`

- [ ] **Step 1: Write failing fixture test**

Create `ui/control-center/src/fixtures/ui1aFixture.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import { ui1aFixture } from './ui1aFixture';

describe('ui1aFixture', () => {
  it('contains a deterministic frame with cells, resource samples, and metadata', () => {
    expect(ui1aFixture.frame.schemaVersion).toBe(2);
    expect(ui1aFixture.frame.runId).toBe('fixture-ui-1a');
    expect(ui1aFixture.frame.committedTick).toBe(128);
    expect(ui1aFixture.frame.world.width).toBe(128);
    expect(ui1aFixture.frame.world.height).toBe(72);
    expect(ui1aFixture.frame.cells).toHaveLength(8);
    expect(ui1aFixture.frame.resourceSamples.length).toBeGreaterThan(0);
    expect(ui1aFixture.frame.cells[0]).toMatchObject({
      id: 'cell-0001',
      lifecycle: 'alive',
    });
  });
});
```

- [ ] **Step 2: Run test to verify RED**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/fixtures/ui1aFixture.test.ts
```

Expected:

```text
FAIL src/fixtures/ui1aFixture.test.ts
Cannot find module './ui1aFixture'
```

- [ ] **Step 3: Add projection types**

Create `ui/control-center/src/projection/types.ts`:

```ts
export type LifecycleState = 'alive' | 'stressed' | 'dormant' | 'dead';

export interface WorldBounds {
  width: number;
  height: number;
}

export interface ProjectedCell {
  id: string;
  x: number;
  y: number;
  radius: number;
  energy: number;
  lifecycle: LifecycleState;
}

export interface ResourceSample {
  x: number;
  y: number;
  value: number;
}

export interface WorldFrame {
  schemaVersion: 2;
  runId: string;
  scenarioName: string;
  committedTick: number;
  configHash: string;
  seed: number;
  world: WorldBounds;
  cells: ProjectedCell[];
  resourceSamples: ResourceSample[];
}

export interface UiFixture {
  frame: WorldFrame;
  selectedCellId: string;
}
```

- [ ] **Step 4: Add deterministic fixture**

Create `ui/control-center/src/fixtures/ui1aFixture.ts`:

```ts
import { UiFixture } from '../projection/types';

export const ui1aFixture: UiFixture = {
  selectedCellId: 'cell-0003',
  frame: {
    schemaVersion: 2,
    runId: 'fixture-ui-1a',
    scenarioName: 'UI-1A Deterministic Fixture',
    committedTick: 128,
    configHash: 'scenario_hash_v1:ui1a000000000001',
    seed: 7884221,
    world: { width: 128, height: 72 },
    cells: [
      { id: 'cell-0001', x: 12, y: 16, radius: 2.1, energy: 18.2, lifecycle: 'alive' },
      { id: 'cell-0002', x: 22, y: 43, radius: 1.8, energy: 8.1, lifecycle: 'stressed' },
      { id: 'cell-0003', x: 48, y: 31, radius: 2.6, energy: 24.9, lifecycle: 'alive' },
      { id: 'cell-0004', x: 61, y: 54, radius: 1.5, energy: 3.4, lifecycle: 'dormant' },
      { id: 'cell-0005', x: 82, y: 20, radius: 2.2, energy: 0.0, lifecycle: 'dead' },
      { id: 'cell-0006', x: 94, y: 49, radius: 2.4, energy: 15.6, lifecycle: 'alive' },
      { id: 'cell-0007', x: 112, y: 27, radius: 1.9, energy: 7.3, lifecycle: 'stressed' },
      { id: 'cell-0008', x: 119, y: 61, radius: 2.0, energy: 20.1, lifecycle: 'alive' }
    ],
    resourceSamples: [
      { x: 8, y: 8, value: 0.18 },
      { x: 24, y: 12, value: 0.35 },
      { x: 40, y: 18, value: 0.61 },
      { x: 56, y: 28, value: 0.92 },
      { x: 72, y: 38, value: 0.75 },
      { x: 88, y: 44, value: 0.48 },
      { x: 104, y: 52, value: 0.31 },
      { x: 120, y: 64, value: 0.16 }
    ]
  }
};
```

- [ ] **Step 5: Run test to verify GREEN**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/fixtures/ui1aFixture.test.ts
```

Expected:

```text
1 passed
```

- [ ] **Step 6: Commit**

Run:

```powershell
git add ui/control-center/src/projection/types.ts ui/control-center/src/fixtures
git commit -m "test(ui): add deterministic UI-1A fixture"
```

## Task 3: Add Fixture Adapter And App State

**Files:**

- Create: `ui/control-center/src/projection/fixtureAdapter.ts`
- Test: `ui/control-center/src/projection/fixtureAdapter.test.ts`
- Create: `ui/control-center/src/app/appState.ts`
- Test: `ui/control-center/src/app/appState.test.ts`

- [ ] **Step 1: Write failing fixture adapter test**

Create `ui/control-center/src/projection/fixtureAdapter.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { loadFixtureFrame, selectCell } from './fixtureAdapter';

describe('fixtureAdapter', () => {
  it('loads a frame and resolves selected Cell by canonical id', () => {
    const frame = loadFixtureFrame(ui1aFixture);
    const selected = selectCell(frame, 'cell-0003');

    expect(frame.committedTick).toBe(128);
    expect(selected?.id).toBe('cell-0003');
    expect(selected?.energy).toBe(24.9);
  });
});
```

- [ ] **Step 2: Run adapter test to verify RED**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/projection/fixtureAdapter.test.ts
```

Expected:

```text
FAIL
Cannot find module './fixtureAdapter'
```

- [ ] **Step 3: Implement fixture adapter**

Create `ui/control-center/src/projection/fixtureAdapter.ts`:

```ts
import { ProjectedCell, UiFixture, WorldFrame } from './types';

export function loadFixtureFrame(fixture: UiFixture): WorldFrame {
  return fixture.frame;
}

export function selectCell(frame: WorldFrame, cellId: string): ProjectedCell | undefined {
  return frame.cells.find((cell) => cell.id === cellId);
}
```

- [ ] **Step 4: Run adapter test to verify GREEN**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/projection/fixtureAdapter.test.ts
```

Expected:

```text
1 passed
```

- [ ] **Step 5: Write failing app state test**

Create `ui/control-center/src/app/appState.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import { createInitialAppState, selectCellById, setTheme } from './appState';

describe('appState', () => {
  it('starts from fixture data and supports selection and theme changes', () => {
    const initial = createInitialAppState();
    const selected = selectCellById(initial, 'cell-0006');
    const dark = setTheme(initial, 'dark');

    expect(initial.frame.runId).toBe('fixture-ui-1a');
    expect(selected.selectedCellId).toBe('cell-0006');
    expect(dark.theme).toBe('dark');
    expect(dark.frame).toBe(initial.frame);
  });
});
```

- [ ] **Step 6: Run app state test to verify RED**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/app/appState.test.ts
```

Expected:

```text
FAIL
Cannot find module './appState'
```

- [ ] **Step 7: Implement app state helpers**

Create `ui/control-center/src/app/appState.ts`:

```ts
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { loadFixtureFrame } from '../projection/fixtureAdapter';
import { WorldFrame } from '../projection/types';

export type ThemeMode = 'light' | 'dark';

export interface AppStateSnapshot {
  frame: WorldFrame;
  selectedCellId: string;
  theme: ThemeMode;
  isFullScreen: boolean;
}

export function createInitialAppState(): AppStateSnapshot {
  return {
    frame: loadFixtureFrame(ui1aFixture),
    selectedCellId: ui1aFixture.selectedCellId,
    theme: 'dark',
    isFullScreen: false,
  };
}

export function selectCellById(
  state: AppStateSnapshot,
  selectedCellId: string,
): AppStateSnapshot {
  return { ...state, selectedCellId };
}

export function setTheme(state: AppStateSnapshot, theme: ThemeMode): AppStateSnapshot {
  return { ...state, theme };
}

export function setFullScreen(state: AppStateSnapshot, isFullScreen: boolean): AppStateSnapshot {
  return { ...state, isFullScreen };
}
```

- [ ] **Step 8: Run state tests to verify GREEN**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/app/appState.test.ts src/projection/fixtureAdapter.test.ts
```

Expected:

```text
2 passed
```

- [ ] **Step 9: Commit**

Run:

```powershell
git add ui/control-center/src/app ui/control-center/src/projection
git commit -m "feat(ui): add fixture projection state"
```

## Task 4: Build Application Shell, Panels, Theme Tokens

**Files:**

- Modify: `ui/control-center/src/App.tsx`
- Create: `ui/control-center/src/app/AppShell.tsx`
- Test: `ui/control-center/src/app/AppShell.test.tsx`
- Create: `ui/control-center/src/components/LayerPanel.tsx`
- Create: `ui/control-center/src/components/InspectorPanel.tsx`
- Create: `ui/control-center/src/components/RunControls.tsx`
- Create: `ui/control-center/src/components/ThemeToggle.tsx`
- Create: `ui/control-center/src/components/BottomDataPanel.tsx`
- Create: `ui/control-center/src/styles/tokens.css`
- Create: `ui/control-center/src/styles/app.css`
- Create: `ui/control-center/src/styles/panels.css`

- [ ] **Step 1: Write failing shell component test**

Create `ui/control-center/src/app/AppShell.test.tsx`:

```tsx
import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it } from 'vitest';
import { renderWithProviders } from '../test/render';
import { AppShell } from './AppShell';

describe('AppShell', () => {
  it('renders Monitor layout, fixture metadata, and selected Cell Inspector', async () => {
    renderWithProviders(<AppShell />);

    expect(screen.getByRole('heading', { name: /ALife Control Center/i })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: 'Monitor' })).toHaveAttribute('aria-selected', 'true');
    expect(screen.getByText('UI-1A Deterministic Fixture')).toBeInTheDocument();
    expect(screen.getByText('Tick 128')).toBeInTheDocument();
    expect(screen.getByText('cell-0003')).toBeInTheDocument();
    expect(screen.getByText('Composite Resource Concentration · Smooth')).toBeInTheDocument();

    await userEvent.click(screen.getByRole('button', { name: /switch to light theme/i }));
    expect(document.documentElement.dataset.theme).toBe('light');
  });
});
```

- [ ] **Step 2: Run shell test to verify RED**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/app/AppShell.test.tsx
```

Expected:

```text
FAIL
Cannot find module './AppShell'
```

- [ ] **Step 3: Add theme tokens and global styles**

Create `ui/control-center/src/styles/tokens.css`:

```css
:root {
  color-scheme: dark;
  --font-sans: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  --surface: #061019;
  --surface-elevated: #0a1824;
  --panel: rgba(8, 24, 36, 0.92);
  --panel-strong: rgba(10, 32, 48, 0.96);
  --border: rgba(78, 219, 220, 0.28);
  --text-primary: #ecfeff;
  --text-secondary: #8aa8b7;
  --accent: #11d9d2;
  --accent-strong: #3ff7ef;
  --warning: #f59e0b;
  --critical: #ff4d67;
  --success: #2fd17c;
  --selection: #f2cc4d;
  --dead: #6b7280;
  --shadow: 0 20px 60px rgba(0, 0, 0, 0.35);
}

:root[data-theme='light'] {
  color-scheme: light;
  --surface: #eef7f7;
  --surface-elevated: #ffffff;
  --panel: rgba(255, 255, 255, 0.92);
  --panel-strong: rgba(255, 255, 255, 0.98);
  --border: rgba(13, 109, 124, 0.26);
  --text-primary: #10202a;
  --text-secondary: #4d6471;
  --accent: #057a85;
  --accent-strong: #0795a2;
  --warning: #b45309;
  --critical: #be123c;
  --success: #047857;
  --selection: #9a6b00;
  --dead: #6b7280;
  --shadow: 0 16px 48px rgba(21, 47, 58, 0.18);
}
```

Create `ui/control-center/src/styles/app.css`:

```css
@import './tokens.css';
@import './panels.css';

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  min-width: 1024px;
  min-height: 768px;
  background: var(--surface);
  color: var(--text-primary);
  font-family: var(--font-sans);
}

button {
  font: inherit;
}

.app-shell {
  display: grid;
  grid-template-rows: 64px 80px minmax(420px, 1fr) 220px;
  min-height: 100vh;
  background:
    radial-gradient(circle at 30% 20%, rgba(17, 217, 210, 0.12), transparent 28%),
    linear-gradient(135deg, var(--surface), #06131f 62%, var(--surface));
}

.top-bar,
.run-bar {
  display: flex;
  align-items: center;
  gap: 24px;
  padding: 0 20px;
  border-bottom: 1px solid var(--border);
  background: var(--panel);
}

.brand h1 {
  margin: 0;
  font-size: 18px;
  letter-spacing: 0;
  text-transform: uppercase;
}

.brand span,
.metric-label,
.panel-kicker {
  color: var(--text-secondary);
  font-size: 12px;
}

.workspace-tabs {
  display: flex;
  gap: 8px;
}

.workspace-tabs button,
.icon-button,
.run-button,
.theme-button {
  border: 1px solid var(--border);
  border-radius: 8px;
  background: rgba(17, 217, 210, 0.08);
  color: var(--text-primary);
  min-height: 36px;
  padding: 0 14px;
}

.workspace-tabs button[aria-selected='true'],
.run-button.primary,
.theme-button {
  border-color: var(--accent);
  color: var(--accent-strong);
  box-shadow: inset 0 0 18px rgba(17, 217, 210, 0.16);
}

.monitor-grid {
  display: grid;
  grid-template-columns: 280px minmax(420px, 1fr) 320px;
  gap: 12px;
  padding: 12px;
  min-height: 0;
}

.viewer-card {
  min-width: 0;
  min-height: 0;
}

.viewer-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 14px;
}

.viewer-host {
  height: calc(100% - 50px);
  min-height: 360px;
}

.bottom-grid {
  display: grid;
  grid-template-columns: 1fr 1.25fr 1fr;
  gap: 12px;
  padding: 0 12px 12px;
}
```

Create `ui/control-center/src/styles/panels.css`:

```css
.panel {
  border: 1px solid var(--border);
  border-radius: 8px;
  background: var(--panel);
  box-shadow: var(--shadow);
  overflow: hidden;
}

.panel-header,
.panel-section {
  padding: 14px;
  border-bottom: 1px solid var(--border);
}

.panel-title {
  margin: 0;
  font-size: 14px;
  color: var(--accent-strong);
  text-transform: uppercase;
}

.layer-row,
.metric-row,
.composition-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  min-height: 28px;
}

.layer-dot {
  width: 10px;
  height: 10px;
  border-radius: 999px;
  background: var(--accent);
}

.value {
  font-variant-numeric: tabular-nums;
}

.selected-id {
  color: var(--accent-strong);
  font-family: ui-monospace, SFMono-Regular, Consolas, monospace;
}
```

- [ ] **Step 4: Add panel components**

Create `ui/control-center/src/components/ThemeToggle.tsx`:

```tsx
import { ThemeMode } from '../app/appState';

interface ThemeToggleProps {
  theme: ThemeMode;
  onThemeChange: (theme: ThemeMode) => void;
}

export function ThemeToggle({ theme, onThemeChange }: ThemeToggleProps) {
  const nextTheme = theme === 'dark' ? 'light' : 'dark';
  return (
    <button
      className="theme-button"
      type="button"
      onClick={() => onThemeChange(nextTheme)}
      aria-label={`Switch to ${nextTheme} theme`}
    >
      {theme === 'dark' ? 'Dark' : 'Light'}
    </button>
  );
}
```

Create `ui/control-center/src/components/RunControls.tsx`:

```tsx
export function RunControls() {
  return (
    <div className="run-controls" aria-label="Run controls">
      <button className="run-button" type="button" aria-label="Jump to first frame">|&lt;</button>
      <button className="run-button primary" type="button" aria-label="Play fixture run">▶</button>
      <button className="run-button" type="button" aria-label="Pause fixture run">Ⅱ</button>
      <button className="run-button" type="button" aria-label="Step one Tick">Step 1</button>
      <button className="run-button" type="button" aria-label="Step N placeholder" disabled>Step N</button>
    </div>
  );
}
```

Create `ui/control-center/src/components/LayerPanel.tsx`:

```tsx
export function LayerPanel() {
  const layers = ['Cells', 'World bounds', 'Resource smooth', 'Selection'];
  return (
    <aside className="panel" aria-label="Layers and filters">
      <div className="panel-header">
        <p className="panel-title">Layers & Filters</p>
      </div>
      <div className="panel-section">
        <div className="panel-kicker">Primary color mode</div>
        <strong>Lifecycle state</strong>
      </div>
      <div className="panel-section">
        <div className="panel-kicker">Field layers</div>
        {layers.map((layer) => (
          <div className="layer-row" key={layer}>
            <span><span className="layer-dot" /> {layer}</span>
            <input type="checkbox" checked readOnly aria-label={`${layer} layer enabled`} />
          </div>
        ))}
      </div>
    </aside>
  );
}
```

Create `ui/control-center/src/components/InspectorPanel.tsx`:

```tsx
import { ProjectedCell, WorldFrame } from '../projection/types';

interface InspectorPanelProps {
  frame: WorldFrame;
  selectedCell?: ProjectedCell;
}

export function InspectorPanel({ frame, selectedCell }: InspectorPanelProps) {
  return (
    <aside className="panel" aria-label="Contextual Inspector">
      <div className="panel-header">
        <p className="panel-title">Contextual Inspector</p>
      </div>
      <div className="panel-section">
        <div className="panel-kicker">Selected</div>
        <strong className="selected-id">{selectedCell?.id ?? 'No selection'}</strong>
      </div>
      {selectedCell ? (
        <div className="panel-section">
          <div className="metric-row"><span>Lifecycle</span><strong>{selectedCell.lifecycle}</strong></div>
          <div className="metric-row"><span>Energy</span><strong>{selectedCell.energy.toFixed(1)} eu</strong></div>
          <div className="metric-row"><span>Radius</span><strong>{selectedCell.radius.toFixed(1)} su</strong></div>
          <div className="metric-row"><span>Position</span><strong>{selectedCell.x.toFixed(1)}, {selectedCell.y.toFixed(1)}</strong></div>
        </div>
      ) : null}
      <div className="panel-section">
        <div className="panel-kicker">Data provenance</div>
        <div className="metric-row"><span>Projection</span><span>fixture/v2</span></div>
        <div className="metric-row"><span>Displayed Tick</span><span>{frame.committedTick}</span></div>
      </div>
    </aside>
  );
}
```

Create `ui/control-center/src/components/BottomDataPanel.tsx`:

```tsx
import { WorldFrame } from '../projection/types';

interface BottomDataPanelProps {
  frame: WorldFrame;
}

export function BottomDataPanel({ frame }: BottomDataPanelProps) {
  const alive = frame.cells.filter((cell) => cell.lifecycle === 'alive').length;
  const stressed = frame.cells.filter((cell) => cell.lifecycle === 'stressed').length;
  const dead = frame.cells.filter((cell) => cell.lifecycle === 'dead').length;

  return (
    <section className="bottom-grid" aria-label="Data panel">
      <article className="panel panel-section">
        <p className="panel-title">Resource Cycle</p>
        <div className="metric-row"><span>Total samples</span><strong>{frame.resourceSamples.length}</strong></div>
        <div className="metric-row"><span>Mode</span><strong>Composite · Smooth</strong></div>
      </article>
      <article className="panel panel-section">
        <p className="panel-title">Population</p>
        <div className="metric-row"><span>Alive</span><strong>{alive}</strong></div>
        <div className="metric-row"><span>Stressed</span><strong>{stressed}</strong></div>
        <div className="metric-row"><span>Dead</span><strong>{dead}</strong></div>
      </article>
      <article className="panel panel-section">
        <p className="panel-title">Fixture Metadata</p>
        <div className="metric-row"><span>Seed</span><strong>{frame.seed}</strong></div>
        <div className="metric-row"><span>Config</span><strong>{frame.configHash.slice(0, 24)}</strong></div>
      </article>
    </section>
  );
}
```

- [ ] **Step 5: Add AppShell and App wiring**

Create `ui/control-center/src/app/AppShell.tsx`:

```tsx
import { useEffect, useMemo, useState } from 'react';
import { BottomDataPanel } from '../components/BottomDataPanel';
import { InspectorPanel } from '../components/InspectorPanel';
import { LayerPanel } from '../components/LayerPanel';
import { RunControls } from '../components/RunControls';
import { ThemeToggle } from '../components/ThemeToggle';
import { selectCell } from '../projection/fixtureAdapter';
import { AppStateSnapshot, createInitialAppState, selectCellById, setTheme } from './appState';

export function AppShell() {
  const [state, setState] = useState<AppStateSnapshot>(() => createInitialAppState());
  const selectedCell = useMemo(
    () => selectCell(state.frame, state.selectedCellId),
    [state.frame, state.selectedCellId],
  );

  useEffect(() => {
    document.documentElement.dataset.theme = state.theme;
  }, [state.theme]);

  return (
    <div className="app-shell">
      <header className="top-bar">
        <div className="brand">
          <h1>ALife Control Center</h1>
          <span>Artificial Life Simulation Platform</span>
        </div>
        <nav className="workspace-tabs" aria-label="Workspaces">
          {['Monitor', 'World Editor', 'Experiments', 'Evolution', 'Library', 'Analysis'].map((item) => (
            <button key={item} role="tab" aria-selected={item === 'Monitor'} type="button">
              {item}
            </button>
          ))}
        </nav>
        <ThemeToggle
          theme={state.theme}
          onThemeChange={(theme) => setState((current) => setTheme(current, theme))}
        />
      </header>

      <section className="run-bar" aria-label="Run summary">
        <div>
          <div className="metric-label">Data Context</div>
          <strong>Fixture</strong>
        </div>
        <div>
          <div className="metric-label">Scenario</div>
          <strong>{state.frame.scenarioName}</strong>
        </div>
        <RunControls />
        <div>
          <div className="metric-label">Tick</div>
          <strong>Tick {state.frame.committedTick}</strong>
        </div>
        <div>
          <div className="metric-label">Visual FPS</div>
          <strong>fixture</strong>
        </div>
      </section>

      <main className="monitor-grid">
        <LayerPanel />
        <section className="panel viewer-card" aria-label="World View">
          <div className="viewer-header">
            <strong>World View</strong>
            <span>Composite Resource Concentration · Smooth</span>
          </div>
          <div className="viewer-host" data-testid="viewer-host">
            <button type="button" onClick={() => setState((current) => selectCellById(current, 'cell-0006'))}>
              Select cell-0006
            </button>
          </div>
        </section>
        <InspectorPanel frame={state.frame} selectedCell={selectedCell} />
      </main>

      <BottomDataPanel frame={state.frame} />
    </div>
  );
}
```

Modify `ui/control-center/src/App.tsx`:

```tsx
import { AppShell } from './app/AppShell';
import './styles/app.css';

export function App() {
  return <AppShell />;
}
```

- [ ] **Step 6: Run shell test to verify GREEN**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/app/AppShell.test.tsx
```

Expected:

```text
1 passed
```

- [ ] **Step 7: Commit**

Run:

```powershell
git add ui/control-center/src
git commit -m "feat(ui): add monitor application shell"
```

## Task 5: Add Viewport Math And PixiJS World Viewer

**Files:**

- Create: `ui/control-center/src/renderer/viewport.ts`
- Test: `ui/control-center/src/renderer/viewport.test.ts`
- Create: `ui/control-center/src/renderer/worldRenderer.ts`
- Create: `ui/control-center/src/renderer/WorldViewer.tsx`
- Test: `ui/control-center/src/renderer/WorldViewer.test.tsx`
- Modify: `ui/control-center/src/app/AppShell.tsx`

- [ ] **Step 1: Write failing viewport math test**

Create `ui/control-center/src/renderer/viewport.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import { fitWorldToViewport, screenToWorld, worldToScreen } from './viewport';

describe('viewport', () => {
  it('fits world while preserving aspect ratio and round-trips coordinates', () => {
    const transform = fitWorldToViewport({ width: 128, height: 72 }, { width: 640, height: 360 });
    const screen = worldToScreen({ x: 64, y: 36 }, transform);
    const world = screenToWorld(screen, transform);

    expect(transform.scale).toBe(5);
    expect(screen.x).toBe(320);
    expect(screen.y).toBe(180);
    expect(world.x).toBeCloseTo(64);
    expect(world.y).toBeCloseTo(36);
  });
});
```

- [ ] **Step 2: Run viewport test to verify RED**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/renderer/viewport.test.ts
```

Expected:

```text
FAIL
Cannot find module './viewport'
```

- [ ] **Step 3: Implement viewport math**

Create `ui/control-center/src/renderer/viewport.ts`:

```ts
import { WorldBounds } from '../projection/types';

export interface Size {
  width: number;
  height: number;
}

export interface Point {
  x: number;
  y: number;
}

export interface ViewTransform {
  scale: number;
  offsetX: number;
  offsetY: number;
}

export function fitWorldToViewport(world: WorldBounds, viewport: Size): ViewTransform {
  const scale = Math.min(viewport.width / world.width, viewport.height / world.height);
  const offsetX = (viewport.width - world.width * scale) / 2;
  const offsetY = (viewport.height - world.height * scale) / 2;
  return { scale, offsetX, offsetY };
}

export function worldToScreen(point: Point, transform: ViewTransform): Point {
  return {
    x: point.x * transform.scale + transform.offsetX,
    y: point.y * transform.scale + transform.offsetY,
  };
}

export function screenToWorld(point: Point, transform: ViewTransform): Point {
  return {
    x: (point.x - transform.offsetX) / transform.scale,
    y: (point.y - transform.offsetY) / transform.scale,
  };
}
```

- [ ] **Step 4: Run viewport test to verify GREEN**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/renderer/viewport.test.ts
```

Expected:

```text
1 passed
```

- [ ] **Step 5: Write failing WorldViewer component test**

Create `ui/control-center/src/renderer/WorldViewer.test.tsx`:

```tsx
import { screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { renderWithProviders } from '../test/render';
import { WorldViewer } from './WorldViewer';

vi.mock('./worldRenderer', () => ({
  mountWorldRenderer: vi.fn(() => ({
    destroy: vi.fn(),
    resize: vi.fn(),
    renderFrame: vi.fn(),
  })),
}));

describe('WorldViewer', () => {
  it('renders a canvas host and exposes viewer controls', () => {
    renderWithProviders(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-0003"
        onSelectCell={() => undefined}
      />,
    );

    expect(screen.getByTestId('world-viewer')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /reset viewport/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /toggle full screen/i })).toBeInTheDocument();
  });
});
```

- [ ] **Step 6: Run WorldViewer test to verify RED**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/renderer/WorldViewer.test.tsx
```

Expected:

```text
FAIL
Cannot find module './WorldViewer'
```

- [ ] **Step 7: Implement PixiJS renderer boundary**

Create `ui/control-center/src/renderer/worldRenderer.ts`:

```ts
import { Application, Container, Graphics } from 'pixi.js';
import { ProjectedCell, WorldFrame } from '../projection/types';
import { fitWorldToViewport, worldToScreen } from './viewport';

export interface MountedWorldRenderer {
  renderFrame: (frame: WorldFrame, selectedCellId: string | undefined) => void;
  resize: () => void;
  destroy: () => void;
}

const lifecycleColors: Record<ProjectedCell['lifecycle'], number> = {
  alive: 0x20e6a8,
  stressed: 0xf2cc4d,
  dormant: 0x8b5cf6,
  dead: 0x6b7280,
};

export async function mountWorldRenderer(host: HTMLElement): Promise<MountedWorldRenderer> {
  const app = new Application();
  await app.init({
    resizeTo: host,
    backgroundAlpha: 0,
    antialias: true,
  });
  host.appendChild(app.canvas);

  const scene = new Container();
  app.stage.addChild(scene);

  function renderFrame(frame: WorldFrame, selectedCellId: string | undefined) {
    scene.removeChildren();
    const bounds = host.getBoundingClientRect();
    const transform = fitWorldToViewport(frame.world, {
      width: Math.max(bounds.width, 1),
      height: Math.max(bounds.height, 1),
    });

    const worldBounds = new Graphics();
    worldBounds.rect(
      transform.offsetX,
      transform.offsetY,
      frame.world.width * transform.scale,
      frame.world.height * transform.scale,
    );
    worldBounds.stroke({ color: 0x11d9d2, width: 1, alpha: 0.45 });
    scene.addChild(worldBounds);

    for (const sample of frame.resourceSamples) {
      const point = worldToScreen(sample, transform);
      const radius = Math.max(18 * sample.value, 4);
      const g = new Graphics();
      g.circle(point.x, point.y, radius);
      g.fill({ color: 0x11d9d2, alpha: 0.08 + sample.value * 0.22 });
      scene.addChild(g);
    }

    for (const cell of frame.cells) {
      const point = worldToScreen(cell, transform);
      const radius = Math.max(cell.radius * transform.scale, 4);
      const g = new Graphics();
      g.circle(point.x, point.y, radius);
      g.fill({ color: lifecycleColors[cell.lifecycle], alpha: cell.lifecycle === 'dead' ? 0.45 : 0.85 });
      g.stroke({
        color: cell.id === selectedCellId ? 0xf2cc4d : 0x061019,
        width: cell.id === selectedCellId ? 3 : 1,
        alpha: 0.95,
      });
      scene.addChild(g);
    }
  }

  return {
    renderFrame,
    resize: () => app.renderer.resize(host.clientWidth, host.clientHeight),
    destroy: () => app.destroy(true),
  };
}
```

- [ ] **Step 8: Implement WorldViewer React component**

Create `ui/control-center/src/renderer/WorldViewer.tsx`:

```tsx
import { useEffect, useRef, useState } from 'react';
import { WorldFrame } from '../projection/types';
import { mountWorldRenderer, MountedWorldRenderer } from './worldRenderer';

interface WorldViewerProps {
  frame: WorldFrame;
  selectedCellId?: string;
  onSelectCell: (cellId: string) => void;
}

export function WorldViewer({ frame, selectedCellId, onSelectCell }: WorldViewerProps) {
  const hostRef = useRef<HTMLDivElement | null>(null);
  const rendererRef = useRef<MountedWorldRenderer | null>(null);
  const [isFullScreen, setIsFullScreen] = useState(false);

  useEffect(() => {
    let disposed = false;
    const host = hostRef.current;
    if (!host) return;

    mountWorldRenderer(host).then((renderer) => {
      if (disposed) {
        renderer.destroy();
        return;
      }
      rendererRef.current = renderer;
      renderer.renderFrame(frame, selectedCellId);
    });

    return () => {
      disposed = true;
      rendererRef.current?.destroy();
      rendererRef.current = null;
    };
  }, []);

  useEffect(() => {
    rendererRef.current?.renderFrame(frame, selectedCellId);
  }, [frame, selectedCellId]);

  return (
    <div className={isFullScreen ? 'viewer-host is-full-screen' : 'viewer-host'}>
      <div className="viewer-toolbar" aria-label="Viewer controls">
        <button type="button" onClick={() => rendererRef.current?.renderFrame(frame, selectedCellId)}>
          Reset viewport
        </button>
        <button type="button" onClick={() => setIsFullScreen((value) => !value)}>
          Toggle full screen
        </button>
        <button type="button" onClick={() => onSelectCell(frame.cells[0]?.id ?? '')}>
          Select first Cell
        </button>
      </div>
      <div ref={hostRef} data-testid="world-viewer" className="viewer-canvas-host" />
    </div>
  );
}
```

Append to `ui/control-center/src/styles/app.css`:

```css
.viewer-toolbar {
  display: flex;
  gap: 8px;
  padding: 0 14px 10px;
}

.viewer-toolbar button {
  border: 1px solid var(--border);
  border-radius: 8px;
  background: rgba(17, 217, 210, 0.08);
  color: var(--text-primary);
  min-height: 32px;
}

.viewer-canvas-host {
  height: calc(100% - 48px);
  min-height: 320px;
}

.viewer-canvas-host canvas {
  display: block;
  width: 100%;
  height: 100%;
}

.viewer-host.is-full-screen {
  position: fixed;
  inset: 0;
  z-index: 20;
  background: var(--surface);
  padding: 12px;
}
```

- [ ] **Step 9: Replace placeholder viewer in AppShell**

Modify the imports in `ui/control-center/src/app/AppShell.tsx`:

```tsx
import { WorldViewer } from '../renderer/WorldViewer';
```

Replace the placeholder `<div className="viewer-host" data-testid="viewer-host">...</div>` with:

```tsx
<WorldViewer
  frame={state.frame}
  selectedCellId={state.selectedCellId}
  onSelectCell={(cellId) => setState((current) => selectCellById(current, cellId))}
/>
```

- [ ] **Step 10: Run renderer tests to verify GREEN**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/renderer/viewport.test.ts src/renderer/WorldViewer.test.tsx src/app/AppShell.test.tsx
```

Expected:

```text
3 passed
```

- [ ] **Step 11: Commit**

Run:

```powershell
git add ui/control-center/src
git commit -m "feat(ui): render fixture world with PixiJS"
```

## Task 6: Add Screenshot Export

**Files:**

- Modify: `ui/control-center/src/renderer/worldRenderer.ts`
- Modify: `ui/control-center/src/renderer/WorldViewer.tsx`
- Test: `ui/control-center/src/renderer/WorldViewer.test.tsx`

- [ ] **Step 1: Extend failing WorldViewer test for export control**

Modify `ui/control-center/src/renderer/WorldViewer.test.tsx`:

```tsx
import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { renderWithProviders } from '../test/render';
import { WorldViewer } from './WorldViewer';

const exportPng = vi.fn(() => 'data:image/png;base64,fixture');

vi.mock('./worldRenderer', () => ({
  mountWorldRenderer: vi.fn(() => ({
    destroy: vi.fn(),
    resize: vi.fn(),
    renderFrame: vi.fn(),
    exportPng,
  })),
}));

describe('WorldViewer', () => {
  it('renders a canvas host and exposes viewer controls', () => {
    renderWithProviders(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-0003"
        onSelectCell={() => undefined}
      />,
    );

    expect(screen.getByTestId('world-viewer')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /reset viewport/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /toggle full screen/i })).toBeInTheDocument();
  });

  it('exports the current viewport as a PNG data URL', async () => {
    renderWithProviders(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-0003"
        onSelectCell={() => undefined}
      />,
    );

    await userEvent.click(screen.getByRole('button', { name: /export screenshot/i }));

    expect(exportPng).toHaveBeenCalledOnce();
    expect(screen.getByText(/screenshot ready/i)).toBeInTheDocument();
  });
});
```

- [ ] **Step 2: Run screenshot test to verify RED**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/renderer/WorldViewer.test.tsx
```

Expected:

```text
FAIL
Unable to find role="button" and name /export screenshot/i
```

- [ ] **Step 3: Add export method to renderer**

Modify `MountedWorldRenderer` in `ui/control-center/src/renderer/worldRenderer.ts`:

```ts
export interface MountedWorldRenderer {
  renderFrame: (frame: WorldFrame, selectedCellId: string | undefined) => void;
  resize: () => void;
  exportPng: () => string;
  destroy: () => void;
}
```

Return object:

```ts
return {
  renderFrame,
  resize: () => app.renderer.resize(host.clientWidth, host.clientHeight),
  exportPng: () => app.canvas.toDataURL('image/png'),
  destroy: () => app.destroy(true),
};
```

- [ ] **Step 4: Add export UI**

Modify `ui/control-center/src/renderer/WorldViewer.tsx`:

```tsx
const [screenshotStatus, setScreenshotStatus] = useState<string>('');
```

Add to toolbar:

```tsx
<button
  type="button"
  onClick={() => {
    const dataUrl = rendererRef.current?.exportPng();
    setScreenshotStatus(dataUrl ? 'Screenshot ready' : 'Screenshot unavailable');
  }}
>
  Export screenshot
</button>
```

Add below toolbar:

```tsx
{screenshotStatus ? <div role="status">{screenshotStatus}</div> : null}
```

- [ ] **Step 5: Run screenshot test to verify GREEN**

Run:

```powershell
Set-Location ui\control-center
npm test -- src/renderer/WorldViewer.test.tsx
```

Expected:

```text
2 passed
```

- [ ] **Step 6: Commit**

Run:

```powershell
git add ui/control-center/src/renderer
git commit -m "feat(ui): add viewer screenshot export"
```

## Task 7: Add E2E Smoke, 1024x768 Check, And Build Verification

**Files:**

- Create: `ui/control-center/e2e/ui1a-monitor.spec.ts`
- Modify: `ui/control-center/package.json`

- [ ] **Step 1: Write Playwright smoke test**

Create `ui/control-center/e2e/ui1a-monitor.spec.ts`:

```ts
import { expect, test } from '@playwright/test';

test('UI-1A Monitor opens at 1024x768 with fixture viewer and Inspector', async ({ page }) => {
  await page.setViewportSize({ width: 1024, height: 768 });
  await page.goto('/');

  await expect(page.getByRole('heading', { name: /ALife Control Center/i })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Monitor' })).toHaveAttribute('aria-selected', 'true');
  await expect(page.getByText('UI-1A Deterministic Fixture')).toBeVisible();
  await expect(page.getByTestId('world-viewer')).toBeVisible();
  await expect(page.getByText('cell-0003')).toBeVisible();

  await page.getByRole('button', { name: /switch to light theme/i }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
});
```

- [ ] **Step 2: Install Playwright browser if needed**

Run:

```powershell
Set-Location ui\control-center
npx playwright install chromium
```

Expected:

```text
Chromium ... downloaded
```

If browser download is blocked by network sandbox, request escalation for this command.

- [ ] **Step 3: Run full UI test suite**

Run:

```powershell
Set-Location ui\control-center
npm test
```

Expected:

```text
all Vitest tests passed
```

- [ ] **Step 4: Run build**

Run:

```powershell
Set-Location ui\control-center
npm run build
```

Expected:

```text
vite build ... built
```

- [ ] **Step 5: Run Playwright**

Run:

```powershell
Set-Location ui\control-center
npm run e2e
```

Expected:

```text
1 passed
```

- [ ] **Step 6: Commit**

Run:

```powershell
git add ui/control-center
git commit -m "test(ui): cover UI-1A monitor smoke"
```

## Task 8: Documentation, Worklog Report, And Final Acceptance

**Files:**

- Modify: `outputs/worklogs/index.md`
- Create: `outputs/worklogs/2026-07-15-2130-REPORT-ui-1a-application-shell-fixture-viewer.md`

- [ ] **Step 1: Add report file**

Create `outputs/worklogs/2026-07-15-2130-REPORT-ui-1a-application-shell-fixture-viewer.md`:

```markdown
# UI-1A Application Shell And Deterministic Fixture Viewer Report

## Summary

- Created `ui/control-center` Vite React workspace.
- Added deterministic UI-1A fixture projection.
- Built Monitor application shell.
- Added PixiJS fixture World Viewer.
- Added Cell selection and Inspector.
- Added Light/Dark theme support.
- Added screenshot export.
- Added Vitest and Playwright coverage.

## Changed files

- `ui/control-center/**`
- `.gitignore`
- `outputs/worklogs/index.md`

## Verification

- `npm test`
- `npm run build`
- `npm run e2e`
- `cargo fmt --check`

## Screenshots

- Attach or reference the Playwright screenshot artifact if captured during execution.

## Deviations

- List any deviation from the plan.

## Next recommended slice

- `UI-1B: Live Projection Transport And Run Controls`
- Run the Interface Design Alignment Session before `UI-1C`.
```

- [ ] **Step 2: Register report in worklog index**

Modify `outputs/worklogs/index.md` under `## Reports`:

```markdown
- [[outputs/worklogs/2026-07-15-2130-REPORT-ui-1a-application-shell-fixture-viewer|2026-07-15-2130-REPORT-ui-1a-application-shell-fixture-viewer]]
```

- [ ] **Step 3: Run final checks**

Run:

```powershell
Set-Location ui\control-center
npm test
npm run build
npm run e2e
Set-Location ..\..
cargo fmt --check
git status --short
```

Expected:

```text
Vitest passes
Vite build passes
Playwright passes
cargo fmt --check passes
git status shows only intentional UI-1A/report changes before final commit
```

- [ ] **Step 4: Commit report**

Run:

```powershell
git add outputs/worklogs/index.md outputs/worklogs/2026-07-15-2130-REPORT-ui-1a-application-shell-fixture-viewer.md
git commit -m "docs(ui): report UI-1A fixture viewer"
```

## Acceptance Gate

UI-1A is complete only when:

- `ui/control-center` starts with `npm run dev`.
- `npm test` passes.
- `npm run build` passes.
- `npm run e2e` passes in Chromium.
- Monitor opens as first workspace.
- Deterministic fixture loads without live Runner.
- Viewer draws world bounds, resource heatmap, and Cells.
- Selected Cell updates Inspector.
- Light and Dark themes work.
- 1024x768 viewport remains usable.
- Screenshot export returns a PNG data URL.
- `Step N` is disabled or visibly placeholder-only.
- No browser-native `alert`, `confirm`, or `prompt` is used.
- No Core, Runner, or simulation behavior files changed.
- Worklog report is registered in `outputs/worklogs/index.md`.

## Self-Review

- Spec coverage: Tasks cover scaffold, deterministic fixture, shell, layer panel, viewer, Inspector, themes, screenshot export, 1024x768 E2E, and report.
- TDD coverage: Production behavior tasks include RED/GREEN steps before implementation. Scaffold has an explicit exception note because initial package setup cannot run before dependencies exist.
- Deferred scope is explicit: live Runner, ALIF decoding, OrganismView detail, rich analytics, and design alignment are not part of UI-1A.
- Type consistency: `WorldFrame`, `ProjectedCell`, `UiFixture`, `AppStateSnapshot`, `WorldViewer`, and renderer method names are consistent across tasks.
- Placeholder scan: `Step N placeholder` is intentional user-approved scope behavior, not an implementation omission.
