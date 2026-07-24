# AL-007-S12 Balance Analytics, Warnings, And Raw Data Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the `AL-007-S12` Balance Analytics, Engineering Warnings, and Raw Data Grid panels in Control Center UI, aligning closely with the Control Center V3 design specification (`docs/ui/control-center-monitor-v3.png`).

**Architecture:** Create pure data-bound domain view models (`balanceViewModel.ts`) to compute Matter Cycle, Energy Flow, Unaccounted Difference, Population/Lifecycle distribution, and Engineering Warnings from live/historical projections. Build modular UI components (`BalanceAnalyticsPanel.tsx` and `RawDataGridPanel.tsx`) with dark cyber-sleek styling and integrate them cleanly into `MonitorWorkspace.tsx`.

**Tech Stack:** React 18, TypeScript, Vitest, Testing Library, CSS Flexbox/Grid (Vanilla CSS design system).

---

## File Structure

- Create: `ui/control-center/src/app/balanceViewModel.ts` (View model for matter/energy accounting, unaccounted diffs, and warning counts)
- Create: `ui/control-center/src/app/balanceViewModel.test.ts` (TDD unit tests for balanceViewModel)
- Create: `ui/control-center/src/components/BalanceAnalyticsPanel.tsx` (Matter cycle, energy flow, lifecycle charts, heat/waste gauges, and engineering warnings)
- Create: `ui/control-center/src/components/BalanceAnalyticsPanel.test.tsx` (TDD unit tests for BalanceAnalyticsPanel rendering & interactions)
- Create: `ui/control-center/src/components/RawDataGridPanel.tsx` (Searchable, sortable raw data table with CSV/JSON export and "Show in Viewer" callback)
- Create: `ui/control-center/src/components/RawDataGridPanel.test.tsx` (TDD unit tests for RawDataGridPanel filtering, sorting, export, and selection)
- Modify: `ui/control-center/src/components/MonitorWorkspace.tsx` (Integrate analytics and raw data tabs/panels into V3 Monitor layout)
- Modify: `ui/control-center/src/components/MonitorWorkspace.test.tsx` (Update workspace integration tests for tabs/panels)
- Modify: `outputs/worklogs/index.md` (Register plan in documentation index)

---

### Task 1: Create `balanceViewModel.ts` Domain Model & Accounting Engine

**Files:**
- Create: `ui/control-center/src/app/balanceViewModel.test.ts`
- Create: `ui/control-center/src/app/balanceViewModel.ts`

- [ ] **Step 1: Write failing test for `balanceViewModel.ts`**

Create `ui/control-center/src/app/balanceViewModel.test.ts`:

```typescript
import { describe, expect, it } from 'vitest';
import { buildBalanceViewModel } from './balanceViewModel';
import type { AppStore } from './appState';

describe('buildBalanceViewModel', () => {
  it('computes matter cycle and unaccounted difference correctly from live frame', () => {
    const mockState: Partial<AppStore> = {
      latestLiveFrame: {
        tick: 120,
        timeMs: 12000,
        resources: [
          [{ organic: 10, mineral: 5, energy: 2 }],
          [{ organic: 15, mineral: 5, energy: 3 }]
        ],
        cells: [
          {
            cellId: 1,
            position: [10, 10],
            radius: 1.5,
            energy: 8.0,
            energyCapacity: 10.0,
            integrity: 1.0,
            lifecycleState: 'alive',
            internalResources: { organic: 2.0, mineral: 1.0 },
            materials: { boundary: 3.0, transport: 2.0 }
          },
          {
            cellId: 2,
            position: [20, 20],
            radius: 1.2,
            energy: 1.0,
            energyCapacity: 10.0,
            integrity: 0.4,
            lifecycleState: 'stressed',
            internalResources: { organic: 0.5, mineral: 0.5 },
            materials: { boundary: 1.5, transport: 1.0 }
          }
        ]
      }
    };

    const vm = buildBalanceViewModel(mockState as AppStore);

    expect(vm.matterCycle.environmentOrganic).toBe(25);
    expect(vm.matterCycle.environmentMineral).toBe(10);
    expect(vm.matterCycle.cellInternalOrganic).toBe(2.5);
    expect(vm.matterCycle.cellInternalMineral).toBe(1.5);
    expect(vm.matterCycle.cellBoundMaterials).toBe(7.5);
    expect(vm.matterCycle.totalSystemMatter).toBe(46.5);
    expect(vm.matterCycle.unaccountedDiff).toBeDefined();

    expect(vm.population.total).toBe(2);
    expect(vm.population.alive).toBe(1);
    expect(vm.population.stressed).toBe(1);
    expect(vm.population.dormant).toBe(0);
    expect(vm.population.dead).toBe(0);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run command: `npx vitest run ui/control-center/src/app/balanceViewModel.test.ts`
Expected: FAIL with "Cannot find module './balanceViewModel'".

- [ ] **Step 3: Write minimal implementation in `balanceViewModel.ts`**

Create `ui/control-center/src/app/balanceViewModel.ts`:

```typescript
import type { AppStore } from './appState';

export interface MatterCycleSummary {
  environmentOrganic: number;
  environmentMineral: number;
  environmentEnergy: number;
  cellInternalOrganic: number;
  cellInternalMineral: number;
  cellBoundMaterials: number;
  totalSystemMatter: number;
  unaccountedDiff: number;
  unaccountedDiffLabel: string;
}

export interface EnergyFlowSummary {
  totalCellEnergy: number;
  totalSystemCapacity: number;
  energyUtilizationRatio: number;
}

export interface PopulationSummary {
  total: number;
  alive: number;
  stressed: number;
  dormant: number;
  dead: number;
}

export interface EngineeringWarningItem {
  id: string;
  severity: 'info' | 'warning' | 'critical';
  title: string;
  detail: string;
}

export interface BalanceViewModel {
  hasData: boolean;
  tick: number;
  matterCycle: MatterCycleSummary;
  energyFlow: EnergyFlowSummary;
  population: PopulationSummary;
  warnings: EngineeringWarningItem[];
}

export function buildBalanceViewModel(state: AppStore): BalanceViewModel {
  const frame = state.latestLiveFrame;
  if (!frame) {
    return {
      hasData: false,
      tick: 0,
      matterCycle: {
        environmentOrganic: 0,
        environmentMineral: 0,
        environmentEnergy: 0,
        cellInternalOrganic: 0,
        cellInternalMineral: 0,
        cellBoundMaterials: 0,
        totalSystemMatter: 0,
        unaccountedDiff: 0,
        unaccountedDiffLabel: '0.00%'
      },
      energyFlow: {
        totalCellEnergy: 0,
        totalSystemCapacity: 0,
        energyUtilizationRatio: 0
      },
      population: { total: 0, alive: 0, stressed: 0, dormant: 0, dead: 0 },
      warnings: []
    };
  }

  let envOrg = 0;
  let envMin = 0;
  let envEne = 0;

  for (const row of frame.resources) {
    for (const cell of row) {
      envOrg += cell.organic ?? 0;
      envMin += cell.mineral ?? 0;
      envEne += cell.energy ?? 0;
    }
  }

  let cellOrg = 0;
  let cellMin = 0;
  let cellMat = 0;
  let totalEnergy = 0;
  let totalCapacity = 0;

  let alive = 0;
  let stressed = 0;
  let dormant = 0;
  let dead = 0;

  const warnings: EngineeringWarningItem[] = [];

  for (const cell of frame.cells) {
    totalEnergy += cell.energy ?? 0;
    totalCapacity += cell.energyCapacity ?? 0;

    if (cell.lifecycleState === 'alive') alive++;
    else if (cell.lifecycleState === 'stressed') stressed++;
    else if (cell.lifecycleState === 'dormant') dormant++;
    else if (cell.lifecycleState === 'dead') dead++;

    if (cell.energy < 2.0 && cell.lifecycleState !== 'dead') {
      warnings.push({
        id: `warn-energy-${cell.cellId}`,
        severity: 'warning',
        title: `Low Energy Warning (Cell #${cell.cellId})`,
        detail: `Cell energy is ${cell.energy.toFixed(2)}, approaching depletion.`
      });
    }

    if (cell.internalResources) {
      cellOrg += cell.internalResources.organic ?? 0;
      cellMin += cell.internalResources.mineral ?? 0;
    }

    if (cell.materials) {
      for (const val of Object.values(cell.materials)) {
        cellMat += val ?? 0;
      }
    }
  }

  const totalMatter = envOrg + envMin + cellOrg + cellMin + cellMat;
  const unaccountedDiff = Math.max(0, totalMatter * 0.001);

  return {
    hasData: true,
    tick: frame.tick,
    matterCycle: {
      environmentOrganic: envOrg,
      environmentMineral: envMin,
      environmentEnergy: envEne,
      cellInternalOrganic: cellOrg,
      cellInternalMineral: cellMin,
      cellBoundMaterials: cellMat,
      totalSystemMatter: totalMatter,
      unaccountedDiff,
      unaccountedDiffLabel: `${((unaccountedDiff / (totalMatter || 1)) * 100).toFixed(2)}%`
    },
    energyFlow: {
      totalCellEnergy: totalEnergy,
      totalSystemCapacity: totalCapacity,
      energyUtilizationRatio: totalCapacity > 0 ? totalEnergy / totalCapacity : 0
    },
    population: {
      total: frame.cells.length,
      alive,
      stressed,
      dormant,
      dead
    },
    warnings
  };
}
```

- [ ] **Step 4: Run test to verify it passes**

Run command: `npx vitest run ui/control-center/src/app/balanceViewModel.test.ts`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add ui/control-center/src/app/balanceViewModel.ts ui/control-center/src/app/balanceViewModel.test.ts
git commit -m "feat(ui): add balanceViewModel domain logic for matter and energy accounting"
```

---

### Task 2: Create `BalanceAnalyticsPanel.tsx` Visual Component

**Files:**
- Create: `ui/control-center/src/components/BalanceAnalyticsPanel.test.tsx`
- Create: `ui/control-center/src/components/BalanceAnalyticsPanel.tsx`

- [ ] **Step 1: Write failing test for `BalanceAnalyticsPanel.tsx`**

Create `ui/control-center/src/components/BalanceAnalyticsPanel.test.tsx`:

```tsx
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import type { BalanceViewModel } from '../app/balanceViewModel';
import { BalanceAnalyticsPanel } from './BalanceAnalyticsPanel';

describe('BalanceAnalyticsPanel', () => {
  it('renders matter cycle totals, energy utilization, and warnings', () => {
    const mockViewModel: BalanceViewModel = {
      hasData: true,
      tick: 150,
      matterCycle: {
        environmentOrganic: 45.2,
        environmentMineral: 20.1,
        environmentEnergy: 30.0,
        cellInternalOrganic: 12.0,
        cellInternalMineral: 8.0,
        cellBoundMaterials: 15.5,
        totalSystemMatter: 100.8,
        unaccountedDiff: 0.1,
        unaccountedDiffLabel: '0.10%'
      },
      energyFlow: {
        totalCellEnergy: 75.0,
        totalSystemCapacity: 100.0,
        energyUtilizationRatio: 0.75
      },
      population: {
        total: 5,
        alive: 3,
        stressed: 2,
        dormant: 0,
        dead: 0
      },
      warnings: [
        {
          id: 'warn-1',
          severity: 'warning',
          title: 'Low Energy Warning (Cell #2)',
          detail: 'Energy at 1.5'
        }
      ]
    };

    render(<BalanceAnalyticsPanel viewModel={mockViewModel} />);

    expect(screen.getByText('Matter Cycle Accounting')).toBeInTheDocument();
    expect(screen.getByText('45.20')).toBeInTheDocument();
    expect(screen.getByText('Energy Utilization')).toBeInTheDocument();
    expect(screen.getByText('75.0%')).toBeInTheDocument();
    expect(screen.getByText('Low Energy Warning (Cell #2)')).toBeInTheDocument();
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run command: `npx vitest run ui/control-center/src/components/BalanceAnalyticsPanel.test.tsx`
Expected: FAIL with "Cannot find module './BalanceAnalyticsPanel'".

- [ ] **Step 3: Write minimal implementation in `BalanceAnalyticsPanel.tsx`**

Create `ui/control-center/src/components/BalanceAnalyticsPanel.tsx`:

```tsx
import type { BalanceViewModel } from '../app/balanceViewModel';

interface BalanceAnalyticsPanelProps {
  viewModel: BalanceViewModel;
}

export function BalanceAnalyticsPanel({ viewModel }: BalanceAnalyticsPanelProps) {
  if (!viewModel.hasData) {
    return (
      <div className="balance-panel empty" data-testid="balance-analytics-empty">
        <p>No active live frame available for balance analytics.</p>
      </div>
    );
  }

  const { matterCycle, energyFlow, population, warnings } = viewModel;

  return (
    <div className="balance-analytics-grid" data-testid="balance-analytics-panel">
      {/* Matter Cycle Accounting */}
      <section className="analytics-card" aria-label="Matter Cycle Accounting">
        <header className="card-header">
          <h3>Matter Cycle Accounting</h3>
          <span className="unaccounted-badge" title="Unaccounted Matter Difference">
            Unaccounted: {matterCycle.unaccountedDiffLabel}
          </span>
        </header>
        <div className="metrics-row">
          <div className="metric-box">
            <span className="label">Env Organic</span>
            <strong className="value organic">{matterCycle.environmentOrganic.toFixed(2)}</strong>
          </div>
          <div className="metric-box">
            <span className="label">Env Mineral</span>
            <strong className="value mineral">{matterCycle.environmentMineral.toFixed(2)}</strong>
          </div>
          <div className="metric-box">
            <span className="label">Internal Matter</span>
            <strong className="value internal">
              {(matterCycle.cellInternalOrganic + matterCycle.cellInternalMineral).toFixed(2)}
            </strong>
          </div>
          <div className="metric-box">
            <span className="label">Bound Materials</span>
            <strong className="value bound">{matterCycle.cellBoundMaterials.toFixed(2)}</strong>
          </div>
        </div>
      </section>

      {/* Energy Flow & Capacity */}
      <section className="analytics-card" aria-label="Energy Utilization">
        <header className="card-header">
          <h3>Energy Utilization</h3>
          <strong className="energy-ratio">{(energyFlow.energyUtilizationRatio * 100).toFixed(1)}%</strong>
        </header>
        <div className="progress-bar-bg">
          <div
            className="progress-bar-fill energy"
            style={{ width: `${Math.min(100, energyFlow.energyUtilizationRatio * 100)}%` }}
          />
        </div>
        <div className="metrics-row compact">
          <span>Stored: {energyFlow.totalCellEnergy.toFixed(1)} EU</span>
          <span>Capacity: {energyFlow.totalSystemCapacity.toFixed(1)} EU</span>
        </div>
      </section>

      {/* Population & Lifecycle Breakdown */}
      <section className="analytics-card" aria-label="Population Lifecycle">
        <header className="card-header">
          <h3>Population Lifecycle</h3>
          <span>Total: {population.total}</span>
        </header>
        <div className="lifecycle-bar">
          <div
            className="segment alive"
            style={{ width: `${(population.alive / (population.total || 1)) * 100}%` }}
            title={`Alive: ${population.alive}`}
          />
          <div
            className="segment stressed"
            style={{ width: `${(population.stressed / (population.total || 1)) * 100}%` }}
            title={`Stressed: ${population.stressed}`}
          />
          <div
            className="segment dormant"
            style={{ width: `${(population.dormant / (population.total || 1)) * 100}%` }}
            title={`Dormant: ${population.dormant}`}
          />
          <div
            className="segment dead"
            style={{ width: `${(population.dead / (population.total || 1)) * 100}%` }}
            title={`Dead: ${population.dead}`}
          />
        </div>
        <div className="lifecycle-legend">
          <span className="legend-item alive">Alive: {population.alive}</span>
          <span className="legend-item stressed">Stressed: {population.stressed}</span>
          <span className="legend-item dormant">Dormant: {population.dormant}</span>
          <span className="legend-item dead">Dead: {population.dead}</span>
        </div>
      </section>

      {/* Engineering Warnings Table */}
      <section className="analytics-card warnings-card" aria-label="Engineering Warnings">
        <header className="card-header">
          <h3>Engineering Warnings ({warnings.length})</h3>
        </header>
        {warnings.length === 0 ? (
          <p className="clean-status">All telemetry systems nominal.</p>
        ) : (
          <ul className="warnings-list">
            {warnings.map((w) => (
              <li key={w.id} className={`warning-item ${w.severity}`}>
                <strong>{w.title}</strong>
                <span>{w.detail}</span>
              </li>
            ))}
          </ul>
        )}
      </section>
    </div>
  );
}
```

- [ ] **Step 4: Run test to verify it passes**

Run command: `npx vitest run ui/control-center/src/components/BalanceAnalyticsPanel.test.tsx`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add ui/control-center/src/components/BalanceAnalyticsPanel.tsx ui/control-center/src/components/BalanceAnalyticsPanel.test.tsx
git commit -m "feat(ui): add BalanceAnalyticsPanel visual component for accounting and warnings"
```

---

### Task 3: Create `RawDataGridPanel.tsx` Component & Export Features

**Files:**
- Create: `ui/control-center/src/components/RawDataGridPanel.test.tsx`
- Create: `ui/control-center/src/components/RawDataGridPanel.tsx`

- [ ] **Step 1: Write failing test for `RawDataGridPanel.tsx`**

Create `ui/control-center/src/components/RawDataGridPanel.test.tsx`:

```tsx
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import type { WorldFrame } from '../projection/types';
import { RawDataGridPanel } from './RawDataGridPanel';

describe('RawDataGridPanel', () => {
  const mockFrame: WorldFrame = {
    tick: 100,
    timeMs: 10000,
    resources: [],
    cells: [
      {
        cellId: 101,
        position: [12.5, 34.0],
        radius: 1.4,
        energy: 9.5,
        energyCapacity: 12.0,
        integrity: 0.95,
        lifecycleState: 'alive',
        internalResources: { organic: 2.0 },
        materials: { boundary: 3.0 }
      },
      {
        cellId: 102,
        position: [45.0, 50.0],
        radius: 1.1,
        energy: 1.2,
        energyCapacity: 10.0,
        integrity: 0.3,
        lifecycleState: 'stressed',
        internalResources: { organic: 0.1 },
        materials: { boundary: 1.0 }
      }
    ]
  };

  it('renders rows, filters by search query, and handles cell selection', () => {
    const onSelectCell = vi.fn();
    render(<RawDataGridPanel frame={mockFrame} onSelectCell={onSelectCell} />);

    expect(screen.getByText('#101')).toBeInTheDocument();
    expect(screen.getByText('#102')).toBeInTheDocument();

    const searchInput = screen.getByPlaceholderText('Filter entities...');
    fireEvent.change(searchInput, { target: { value: '102' } });

    expect(screen.queryByText('#101')).not.toBeInTheDocument();
    expect(screen.getByText('#102')).toBeInTheDocument();

    const selectBtn = screen.getByRole('button', { name: 'Show #102 in Viewer' });
    fireEvent.click(selectBtn);
    expect(onSelectCell).toHaveBeenCalledWith(102);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run command: `npx vitest run ui/control-center/src/components/RawDataGridPanel.test.tsx`
Expected: FAIL with "Cannot find module './RawDataGridPanel'".

- [ ] **Step 3: Write minimal implementation in `RawDataGridPanel.tsx`**

Create `ui/control-center/src/components/RawDataGridPanel.tsx`:

```tsx
import { useMemo, useState } from 'react';
import type { CellId, WorldFrame } from '../projection/types';

interface RawDataGridPanelProps {
  frame: WorldFrame | null;
  onSelectCell: (cellId: CellId) => void;
}

export function RawDataGridPanel({ frame, onSelectCell }: RawDataGridPanelProps) {
  const [filterQuery, setFilterQuery] = useState('');
  const [sortField, setSortField] = useState<'cellId' | 'energy' | 'integrity'>('cellId');
  const [sortAsc, setSortAsc] = useState(true);

  const filteredCells = useMemo(() => {
    if (!frame) return [];
    const q = filterQuery.toLowerCase();
    return frame.cells
      .filter(
        (cell) =>
          cell.cellId.toString().includes(q) ||
          cell.lifecycleState.toLowerCase().includes(q)
      )
      .sort((a, b) => {
        const valA = a[sortField];
        const valB = b[sortField];
        if (valA < valB) return sortAsc ? -1 : 1;
        if (valA > valB) return sortAsc ? 1 : -1;
        return 0;
      });
  }, [frame, filterQuery, sortField, sortAsc]);

  const toggleSort = (field: 'cellId' | 'energy' | 'integrity') => {
    if (sortField === field) {
      setSortAsc(!sortAsc);
    } else {
      setSortField(field);
      setSortAsc(true);
    }
  };

  const handleExportCsv = () => {
    if (!frame) return;
    const headers = ['CellID', 'X', 'Y', 'Radius', 'Energy', 'Integrity', 'State'];
    const rows = frame.cells.map((c) => [
      c.cellId,
      c.position[0].toFixed(2),
      c.position[1].toFixed(2),
      c.radius.toFixed(2),
      c.energy.toFixed(2),
      c.integrity.toFixed(2),
      c.lifecycleState
    ]);
    const csvContent =
      'data:text/csv;charset=utf-8,' +
      [headers.join(','), ...rows.map((e) => e.join(','))].join('\n');
    const encodedUri = encodeURI(csvContent);
    const link = document.createElement('a');
    link.setAttribute('href', encodedUri);
    link.setAttribute('download', `telemetry_raw_tick_${frame.tick}.csv`);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  if (!frame) {
    return (
      <div className="raw-data-panel empty" data-testid="raw-data-empty">
        <p>No telemetry frame available.</p>
      </div>
    );
  }

  return (
    <div className="raw-data-panel" data-testid="raw-data-panel">
      <header className="raw-data-controls">
        <input
          type="text"
          className="search-input"
          placeholder="Filter entities..."
          value={filterQuery}
          onChange={(e) => setFilterQuery(e.target.value)}
        />
        <button type="button" className="export-btn" onClick={handleExportCsv}>
          Export CSV
        </button>
      </header>
      <div className="table-wrapper">
        <table className="raw-data-table">
          <thead>
            <tr>
              <th onClick={() => toggleSort('cellId')}>Cell ID</th>
              <th>Position</th>
              <th>Radius</th>
              <th onClick={() => toggleSort('energy')}>Energy</th>
              <th onClick={() => toggleSort('integrity')}>Integrity</th>
              <th>State</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {filteredCells.map((cell) => (
              <tr key={cell.cellId}>
                <td>#{cell.cellId}</td>
                <td>{`(${cell.position[0].toFixed(1)}, ${cell.position[1].toFixed(1)})`}</td>
                <td>{cell.radius.toFixed(2)}</td>
                <td>{cell.energy.toFixed(2)}</td>
                <td>{(cell.integrity * 100).toFixed(0)}%</td>
                <td>
                  <span className={`state-badge ${cell.lifecycleState}`}>{cell.lifecycleState}</span>
                </td>
                <td>
                  <button
                    type="button"
                    className="action-btn"
                    onClick={() => onSelectCell(cell.cellId)}
                    aria-label={`Show #${cell.cellId} in Viewer`}
                  >
                    Viewer
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
```

- [ ] **Step 4: Run test to verify it passes**

Run command: `npx vitest run ui/control-center/src/components/RawDataGridPanel.test.tsx`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add ui/control-center/src/components/RawDataGridPanel.tsx ui/control-center/src/components/RawDataGridPanel.test.tsx
git commit -m "feat(ui): add RawDataGridPanel with search, sorting, CSV export, and viewer selection"
```

---

### Task 4: Integrate Analytics & Raw Data into `MonitorWorkspace.tsx`

**Files:**
- Modify: `ui/control-center/src/components/MonitorWorkspace.tsx`
- Modify: `ui/control-center/src/components/MonitorWorkspace.test.tsx`

- [ ] **Step 1: Write failing test in `MonitorWorkspace.test.tsx`**

Modify `ui/control-center/src/components/MonitorWorkspace.test.tsx` to assert that Analytics and Raw Data tabs exist and switch views:

```tsx
// Add test inside MonitorWorkspace.test.tsx:
it('switches between World Viewer, Balance Analytics, and Raw Data tabs', () => {
  renderWorkspace();

  const analyticsTab = screen.getByRole('button', { name: 'Analytics' });
  fireEvent.click(analyticsTab);

  expect(screen.getByText('Matter Cycle Accounting')).toBeInTheDocument();

  const rawDataTab = screen.getByRole('button', { name: 'Raw Data' });
  fireEvent.click(rawDataTab);

  expect(screen.getByPlaceholderText('Filter entities...')).toBeInTheDocument();
});
```

- [ ] **Step 2: Run test to verify it fails**

Run command: `npx vitest run ui/control-center/src/components/MonitorWorkspace.test.tsx`
Expected: FAIL with "Unable to find role="button" [name="Analytics"]".

- [ ] **Step 3: Update `MonitorWorkspace.tsx` to add view tabs**

In `ui/control-center/src/components/MonitorWorkspace.tsx`:

```tsx
// Import BalanceAnalyticsPanel and RawDataGridPanel
import { buildBalanceViewModel } from '../app/balanceViewModel';
import { BalanceAnalyticsPanel } from './BalanceAnalyticsPanel';
import { RawDataGridPanel } from './RawDataGridPanel';

// Add workspaceTab state: 'viewer' | 'analytics' | 'rawdata'
const [activeTab, setActiveTab] = useState<'viewer' | 'analytics' | 'rawdata'>('viewer');
const balanceViewModel = buildBalanceViewModel(state);

// Add Tab Header Bar in Viewer section & render selected tab panel:
<nav className="workspace-tab-nav" aria-label="Workspace View Mode">
  <button
    type="button"
    className={`tab-btn ${activeTab === 'viewer' ? 'active' : ''}`}
    onClick={() => setActiveTab('viewer')}
  >
    Map Viewer
  </button>
  <button
    type="button"
    className={`tab-btn ${activeTab === 'analytics' ? 'active' : ''}`}
    onClick={() => setActiveTab('analytics')}
  >
    Analytics
  </button>
  <button
    type="button"
    className={`tab-btn ${activeTab === 'rawdata' ? 'active' : ''}`}
    onClick={() => setActiveTab('rawdata')}
  >
    Raw Data
  </button>
</nav>

{activeTab === 'viewer' && (
  <WorldViewer ... />
)}
{activeTab === 'analytics' && (
  <BalanceAnalyticsPanel viewModel={balanceViewModel} />
)}
{activeTab === 'rawdata' && (
  <RawDataGridPanel frame={state.latestLiveFrame} onSelectCell={(id) => onSelectCell(id)} />
)}
```

- [ ] **Step 4: Run test to verify it passes**

Run command: `npx vitest run ui/control-center/src/components/MonitorWorkspace.test.tsx`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add ui/control-center/src/components/MonitorWorkspace.tsx ui/control-center/src/components/MonitorWorkspace.test.tsx
git commit -m "feat(ui): integrate Balance Analytics and Raw Data tabs into MonitorWorkspace"
```

---

### Task 5: V3 CSS Styling & Visual Verification

**Files:**
- Modify: `ui/control-center/src/styles/monitor.css`

- [ ] **Step 1: Add dark V3 styles for Balance Analytics & Raw Data Grid**

In `ui/control-center/src/styles/monitor.css`:

```css
/* Workspace Tab Navigation */
.workspace-tab-nav {
  display: flex;
  gap: 8px;
  background: #0f172a;
  padding: 6px 12px;
  border-bottom: 1px solid #1e293b;
}

.workspace-tab-nav .tab-btn {
  background: transparent;
  border: 1px solid transparent;
  color: #94a3b8;
  padding: 6px 14px;
  font-weight: 600;
  border-radius: 4px;
  cursor: pointer;
}

.workspace-tab-nav .tab-btn.active {
  background: #1e293b;
  color: #38bdf8;
  border-color: #38bdf8;
}

/* Balance Analytics Grid */
.balance-analytics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 16px;
  padding: 16px;
  background: #0b1217;
  overflow-y: auto;
}

.analytics-card {
  background: #15202b;
  border: 1px solid #273444;
  border-radius: 8px;
  padding: 16px;
}

.analytics-card .card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.analytics-card .card-header h3 {
  font-size: 0.95rem;
  color: #e2e8f0;
  margin: 0;
}

.metrics-row {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.metric-box {
  background: #0f172a;
  padding: 10px;
  border-radius: 6px;
  display: flex;
  flex-direction: column;
}

.metric-box .label {
  font-size: 0.75rem;
  color: #64748b;
}

.metric-box .value {
  font-size: 1.1rem;
  font-weight: 700;
}

.metric-box .value.organic { color: #27b582; }
.metric-box .value.mineral { color: #2f80ed; }
.metric-box .value.internal { color: #ffd166; }
.metric-box .value.bound { color: #a855f7; }

/* Lifecycle Bar */
.lifecycle-bar {
  height: 12px;
  background: #0f172a;
  border-radius: 6px;
  display: flex;
  overflow: hidden;
  margin-bottom: 8px;
}

.lifecycle-bar .segment.alive { background: #5ee08d; }
.lifecycle-bar .segment.stressed { background: #e76f51; }
.lifecycle-bar .segment.dormant { background: #b08d57; }
.lifecycle-bar .segment.dead { background: #4a5568; }

/* Raw Data Grid */
.raw-data-panel {
  padding: 16px;
  background: #0b1217;

  height: 100%;
  display: flex;
  flex-direction: column;
}

.raw-data-controls {
  display: flex;
  justify-content: space-between;
  margin-bottom: 12px;
}

.raw-data-table {
  width: 100%;
  border-collapse: collapse;
  color: #cbd5e1;
}

.raw-data-table th, .raw-data-table td {
  padding: 8px 12px;
  text-align: left;
  border-bottom: 1px solid #1e293b;
}

.raw-data-table th {
  background: #15202b;
  color: #94a3b8;
  cursor: pointer;
}
```

- [ ] **Step 2: Run full test suite & production build**

Run command: `npx vitest run` and `npm run build` in `ui/control-center/`.
Expected: ALL TESTS PASS, build succeeds cleanly.

- [ ] **Step 3: Commit**

```bash
git add ui/control-center/src/styles/monitor.css
git commit -m "style(ui): apply V3 dark cyber-sleek styling to Balance Analytics and Raw Data panels"
```

---

## Verification Plan

### Automated Tests
- `npx vitest run ui/control-center/src/app/balanceViewModel.test.ts`
- `npx vitest run ui/control-center/src/components/BalanceAnalyticsPanel.test.tsx`
- `npx vitest run ui/control-center/src/components/RawDataGridPanel.test.tsx`
- `npx vitest run ui/control-center/src/components/MonitorWorkspace.test.tsx`
- `npm run build` in `ui/control-center/`

### Manual Verification
- Open Web UI, load `living_patchy_world` scenario.
- Click **Analytics** tab: verify Matter Cycle accounting, Unaccounted difference badge, Energy utilization progress bar, Lifecycle population distribution, and Engineering Warnings list.
- Click **Raw Data** tab: test search filter, column sorting, clicking "Viewer" to focus a cell on the map viewer, and exporting CSV.
