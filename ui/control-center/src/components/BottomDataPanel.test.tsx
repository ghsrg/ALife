import { describe, expect, it } from 'vitest';
import { screen } from '@testing-library/react';
import { renderApp } from '../test/render';
import { createAppStore } from '../app/appState';
import { BottomDataPanel } from './BottomDataPanel';

describe('BottomDataPanel', () => {
  it('renders the V3 analytics surface without data-panel tabs', () => {
    const store = createAppStore();
    const state = store.getState();

    renderApp(<BottomDataPanel state={state} />);

    expect(screen.queryByRole('navigation', { name: 'Data panel tabs' })).not.toBeInTheDocument();
    expect(screen.getByText('Population Lifecycle')).toBeInTheDocument();
    expect(screen.getByText('Energy Flow')).toBeInTheDocument();
    expect(screen.getByText('Energy Distribution Over Time')).toBeInTheDocument();
    expect(screen.getAllByText('UNAVAILABLE').length).toBeGreaterThan(0);
  });

  it('does not render fake fallback values or mixed accounting title text', () => {
    const store = createAppStore();
    const state = store.getState();

    const { container } = renderApp(<BottomDataPanel state={state} />);
    const text = container.textContent ?? '';

    expect(text).not.toContain('RESOURCE CYCLE (ENERGY & MATTER)');
    expect(text).not.toContain('65.2%');
    expect(text).not.toContain('23.1%');
    expect(text).not.toContain('8.7%');
    expect(text).not.toContain('3.0%');
    expect(text).not.toContain('1270 amu');
  });

  it('renders Cells level without heuristic behavior labels when classifications are unavailable', () => {
    const store = createAppStore();
    const state = store.getState();

    const { container } = renderApp(<BottomDataPanel state={state} activeLevel="cells" />);
    const text = container.textContent ?? '';

    expect(screen.getByText('Observed Primary Roles')).toBeInTheDocument();
    expect(screen.getByText('Cell Radius Distribution')).toBeInTheDocument();
    expect(text).not.toMatch(/Metabolic|Transport|Structural \(Compact\)/);
  });

  it('renders the Cells lifecycle card as a source-backed stacked progress bar', () => {
    const store = createAppStore();
    store.getState().setDebugProjections({
      status: 'available',
      runId: store.getState().frame.runId,
      tick: store.getState().frame.tick,
      visualWorld: {
        projectionKind: 'VisualWorldProjection',
        completeness: { state: 'bounded', missingFields: [], reason: null },
        cells: [
          {
            id: 'cell-a',
            x: 1,
            y: 1,
            radius: 1,
            energy: 1,
            energyCapacity: 2,
            lifecycleState: 'alive',
            materials: [],
            internalResources: [],
            localExternalResources: []
          },
          {
            id: 'cell-b',
            x: 2,
            y: 2,
            radius: 1,
            energy: 1,
            energyCapacity: 2,
            lifecycleState: 'stressed',
            materials: [],
            internalResources: [],
            localExternalResources: []
          },
          {
            id: 'cell-c',
            x: 3,
            y: 3,
            radius: 1,
            energy: 1,
            energyCapacity: 2,
            lifecycleState: 'dead',
            materials: [],
            internalResources: [],
            localExternalResources: []
          }
        ],
        resourceLayers: [],
        fields: [],
        sourceMetrics: []
      },
      coverage: { projectionKind: 'CoverageProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, mechanisms: [] },
      warnings: { projectionKind: 'WarningProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, warnings: [] },
      classifications: { projectionKind: 'ClassificationProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, classifications: [] },
      balanceFindings: { projectionKind: 'BalanceFindingProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, findings: [] }
    });

    const { container } = renderApp(<BottomDataPanel state={store.getState()} activeLevel="cells" />);
    const lifecycleCard = container.querySelector('[data-card-id="cells-population-lifecycle"]');

    expect(lifecycleCard).toBeInTheDocument();
    expect(lifecycleCard).toHaveTextContent('Total: 3');
    expect(lifecycleCard?.querySelector('.monitor-card-rows')).not.toBeInTheDocument();
    expect(lifecycleCard?.querySelector('.lifecycle-bar')).toHaveAttribute(
      'aria-label',
      'Population lifecycle distribution'
    );
    expect(lifecycleCard?.querySelector('.segment.alive')).toHaveStyle({ width: '33.3%' });
    expect(lifecycleCard?.querySelector('.segment.stressed')).toHaveStyle({ width: '33.3%' });
    expect(lifecycleCard?.querySelector('.segment.dead')).toHaveStyle({ width: '33.3%' });
    expect(lifecycleCard).toHaveTextContent('Alive: 1');
    expect(lifecycleCard).toHaveTextContent('Stressed: 1');
    expect(lifecycleCard).toHaveTextContent('Dormant: 0');
    expect(lifecycleCard).toHaveTextContent('Dead: 1');
  });

  it('renders World accounting selector and compact provenance chips', () => {
    const store = createAppStore();
    const state = store.getState();

    const { container } = renderApp(<BottomDataPanel state={state} />);

    expect(screen.getByRole('button', { name: 'Energy' })).toHaveClass('active');
    expect(screen.getByRole('button', { name: 'Resource' })).toBeInTheDocument();
    expect(container.querySelectorAll('.monitor-card-provenance-chip').length).toBeGreaterThan(0);
    expect(container.querySelector('dl.monitor-card-provenance')).not.toBeInTheDocument();
    expect(screen.queryByText('Source')).not.toBeInTheDocument();
    expect(screen.queryByText('Completeness')).not.toBeInTheDocument();
  });

  it('renders unavailable cards as compact placeholders before provenance metadata', () => {
    const store = createAppStore();
    const state = store.getState();

    const { container } = renderApp(<BottomDataPanel state={state} />);
    const energyFlowCard = container.querySelector('[data-card-id="world-energy-flow"]');

    expect(energyFlowCard?.querySelector('.monitor-card-placeholder')).toHaveTextContent('Unavailable');
    expect(energyFlowCard?.querySelector('.monitor-card-placeholder')).toHaveTextContent('EnergyAccountingProjection');
    expect(energyFlowCard?.querySelector('.monitor-card-provenance')).toHaveClass('compact');
  });
});
