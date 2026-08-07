import { createRef } from 'react';
import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import type { WorldFrame } from '../projection/types';
import type { DebugProjectionState } from '../projection/types';
import { WorldViewer, type WorldViewerHandle } from './WorldViewer';
import { createCellSelection } from '../app/selectionModel';

const renderFrame = vi.fn();
const destroy = vi.fn();
const exportPng = vi.fn(() => 'data:image/png;base64,fixture');

vi.mock('../viewer/worldRenderer', () => ({
  mountWorldRenderer: vi.fn(() => Promise.resolve({
    renderFrame,
    resize: vi.fn(),
    exportPng,
    destroy
  }))
}));

const debugProjections: DebugProjectionState = {
  status: 'available',
  runId: 'run-1',
  tick: 213,
  visualWorld: {
    projectionKind: 'VisualWorldProjection',
    completeness: {
      state: 'partial',
      missingFields: ['cells.materials'],
      reason: 'CommittedSnapshot lacks per-cell material data'
    },
    cells: [],
    resourceLayers: [
      {
        layerIndex: 0,
        resourceTypeId: 0,
        resourceId: 'nucleotide_precursor',
        width: 2,
        height: 2,
        totalAmount: 4,
        cells: [
          { x: 0, y: 0, amount: 1 },
          { x: 1, y: 0, amount: 1 },
          { x: 0, y: 1, amount: 1 },
          { x: 1, y: 1, amount: 1 }
        ],
        completeness: {
          state: 'bounded',
          missingFields: [],
          reason: 'Totals only'
        }
      }
    ],
    fields: [
      {
        fieldId: 'heat',
        value: 2.5,
        sourceMetric: {
          fieldId: 'heat',
          sourceOwner: 'CoreCommittedSnapshot',
          sourcePath: 'CommittedSnapshot.heat'
        }
      }
    ],
    sourceMetrics: []
  },
  coverage: {
    projectionKind: 'CoverageProjection',
    completeness: { state: 'bounded', missingFields: [], reason: 'No rows' },
    mechanisms: []
  },
  warnings: {
    projectionKind: 'WarningProjection',
    completeness: { state: 'bounded', missingFields: [], reason: 'No rows' },
    warnings: []
  },
  classifications: {
    projectionKind: 'ClassificationProjection',
    completeness: { state: 'bounded', missingFields: [], reason: 'No rows' },
    classifications: []
  },
  balanceFindings: {
    projectionKind: 'BalanceFindingProjection',
    completeness: { state: 'bounded', missingFields: [], reason: 'No rows' },
    findings: []
  }
};

describe('WorldViewer', () => {
  beforeEach(() => {
    renderFrame.mockClear();
    destroy.mockClear();
    exportPng.mockClear();
  });

  it('mounts the renderer and renders fixture cells', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await waitFor(() => {
      expect(renderFrame).toHaveBeenLastCalledWith(ui1aFixture.frame, 'cell-a', { x: 0, y: 0, scale: 1 });
    });
    expect(screen.getByLabelText('Select cell-a')).toBeInTheDocument();
    expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
  });

  it('opens with the full world fitted as map scale 1:2600', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
    });

    expect(screen.getByLabelText('World Viewer zoom')).toHaveTextContent('1:2600');
  });

  it('selects cells through accessible hit targets', async () => {
    const onSelectCell = vi.fn();
    const user = userEvent.setup();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={onSelectCell}
      />
    );

    await user.click(screen.getByLabelText('Select cell-c'));

    expect(onSelectCell).toHaveBeenCalledWith('cell-c');
  });

  it('selects a World block instead of a Cell when active Level is World', async () => {
    const onSelectCell = vi.fn();
    const onSelectTarget = vi.fn();
    const user = userEvent.setup();

    render(
      <WorldViewer
        frame={{
          ...ui1aFixture.frame,
          resources: [
            [{ organic: 1, mineral: 0, energy: 0 }, { organic: 2, mineral: 0, energy: 0 }],
            [{ organic: 3, mineral: 0, energy: 0 }, { organic: 4, mineral: 0, energy: 0 }]
          ]
        }}
        selectedCellId="cell-a"
        activeLevel="world"
        onSelectCell={onSelectCell}
        onSelectTarget={onSelectTarget}
      />
    );

    await user.click(screen.getByLabelText('Select cell-a'));

    expect(onSelectCell).not.toHaveBeenCalled();
    expect(onSelectTarget).toHaveBeenCalledWith(expect.objectContaining({ kind: 'world-block' }));
  });

  it('selects a World block from empty Map clicks at World Level', () => {
    const onSelectCell = vi.fn();
    const onSelectTarget = vi.fn();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        activeLevel="world"
        onSelectCell={onSelectCell}
        onSelectTarget={onSelectTarget}
      />
    );

    act(() => {
      fireEvent.click(screen.getByLabelText('World Viewer'), { clientX: 100, clientY: 120 });
    });

    expect(onSelectCell).not.toHaveBeenCalled();
    expect(onSelectTarget).toHaveBeenCalledWith(expect.objectContaining({ kind: 'world-block' }));
  });

  it('uses Shift click to emit a compatible Cell selection set instead of replacing selection', async () => {
    const onSelectTarget = vi.fn();
    const user = userEvent.setup();
    const currentSelection = createCellSelection({
      cellId: 'cell-a',
      runId: ui1aFixture.frame.runId,
      tick: ui1aFixture.frame.tick
    });

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        activeLevel="cells"
        currentSelection={currentSelection}
        onSelectCell={vi.fn()}
        onSelectTarget={onSelectTarget}
      />
    );

    await user.keyboard('{Shift>}');
    await user.click(screen.getByLabelText('Select cell-b'));
    await user.keyboard('{/Shift}');

    expect(onSelectTarget).toHaveBeenCalledWith(expect.objectContaining({
      kind: 'selection-set',
      targetKind: 'cell'
    }));
  });

  it('uses Shift drag to emit a Cell selection set instead of replacing selection', async () => {
    const onSelectTarget = vi.fn();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId={null}
        activeLevel="cells"
        onSelectCell={vi.fn()}
        onSelectTarget={onSelectTarget}
      />
    );

    const viewer = screen.getByLabelText('World Viewer');
    act(() => {
      fireEvent.mouseDown(viewer, { button: 0, shiftKey: true, clientX: -1000, clientY: -1000 });
      fireEvent.mouseMove(viewer, { shiftKey: true, clientX: 2000, clientY: 2000 });
      fireEvent.mouseUp(viewer, { shiftKey: true, clientX: 2000, clientY: 2000 });
    });

    expect(onSelectTarget).toHaveBeenCalledWith(expect.objectContaining({
      kind: 'selection-set',
      targetKind: 'cell'
    }));
  });

  it('does not draw a second selected ring from the DOM hit target', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
    });

    expect(screen.getByLabelText('Select cell-a')).not.toHaveClass('selected');
  });

  it('exposes explicit foreground affordances for selected and search-matched map elements', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
        debugProjections={debugProjections}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
    });

    const selectedLabel = screen.getByLabelText('Selected cell detail label');
    expect(selectedLabel).toHaveAttribute('data-map-affordance', 'selected-foreground');

    await userEvent.click(screen.getByRole('button', { name: 'Expand debug overlay' }));
    fireEvent.change(screen.getByLabelText('Search cells or resource layers'), {
      target: { value: 'cell-c' }
    });

    const searchMatch = screen.getByLabelText('Select cell-c');
    expect(searchMatch).toHaveClass('search-match');
    expect(searchMatch).toHaveAttribute('data-map-affordance', 'search-match');
  });

  it('exposes PNG export through its imperative handle', async () => {
    const ref = createRef<WorldViewerHandle>();

    render(
      <WorldViewer
        ref={ref}
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await waitFor(() => {
      expect(ref.current?.exportPng()).toBe('data:image/png;base64,fixture');
    });
  });

  it('uses display radius for hit targets and exposes truth overlay for tiny live cells', async () => {
    const tinyLiveFrame: WorldFrame = {
      ...ui1aFixture.frame,
      source: 'live',
      resources: [],
      cells: [
        {
          id: 'tiny',
          x: 100,
          y: 100,
          radius: 2,
          energy: 0.8,
          integrity: 1,
          generation: 0,
          roleHint: 'alive lifecycle state',
          lifecycle: 1
        }
      ]
    };

    render(
      <WorldViewer
        frame={tinyLiveFrame}
        selectedCellId="tiny"
        onSelectCell={vi.fn()}
      />
    );

    await waitFor(() => {
      expect(renderFrame).toHaveBeenLastCalledWith(tinyLiveFrame, 'tiny', { x: 0, y: 0, scale: 1 });
    });

    expect(screen.getByLabelText('Select tiny')).toHaveStyle({ width: '36px', height: '36px' });
    expect(screen.getByLabelText('Viewer projection truth')).toHaveTextContent('Missing projection');
    expect(screen.getByLabelText('Viewer projection truth')).toHaveTextContent('Display minimum applied');
  });

  it('zooms with visible controls and sends the camera to the renderer', async () => {
    const user = userEvent.setup();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await user.click(screen.getByRole('button', { name: 'Zoom in World Viewer' }));

    await waitFor(() => {
      expect(renderFrame).toHaveBeenLastCalledWith(ui1aFixture.frame, 'cell-a', {
        x: -120,
        y: -80,
        scale: 1.2
      });
    });
    expect(screen.getByLabelText('World Viewer zoom')).toHaveTextContent(/^1:\d+$/);
  });

  it('renders optional map utility controls next to navigation tools', async () => {
    const onExport = vi.fn();
    const onFullScreen = vi.fn();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
        onExportScreenshot={onExport}
        onToggleFullScreen={onFullScreen}
        isFullScreen={false}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
    });

    await userEvent.click(screen.getByRole('button', { name: 'Export viewer PNG' }));
    await userEvent.click(screen.getByRole('button', { name: 'Enter Start full screen' }));

    expect(onExport).toHaveBeenCalledTimes(1);
    expect(onFullScreen).toHaveBeenCalledTimes(1);
  });

  it('shows data-bound Debug Visualization Mode controls and disabled future overlays', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
        debugProjections={debugProjections}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
    });

    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveClass('collapsed');
    await userEvent.click(screen.getByRole('button', { name: 'Expand debug overlay' }));

    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('Debug');
    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('Exact');
    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('VisualWorldProjection');
    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('Tick 213');
    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('cells.materials');
    expect(screen.getByRole('button', { name: 'Switch debug layers to Smooth interpolation' })).toBeEnabled();
    expect(screen.getByRole('checkbox', { name: 'Spatial index overlay unavailable' })).toBeDisabled();
    expect(screen.queryByText('Missing live projection')).not.toBeInTheDocument();
  });

  it('bounds large debug resource legends so they cannot cover the map', async () => {
    const resourceLayers = Array.from({ length: 27 }, (_, layerIndex) => ({
      layerIndex,
      resourceTypeId: layerIndex,
      resourceId: `resource_${layerIndex}`,
      width: 1,
      height: 1,
      totalAmount: layerIndex + 1,
      cells: [{ x: 0, y: 0, amount: layerIndex + 1 }],
      completeness: {
        state: 'bounded' as const,
        missingFields: [],
        reason: null
      }
    }));

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
        debugProjections={{
          ...debugProjections,
          visualWorld: {
            ...debugProjections.visualWorld,
            resourceLayers
          }
        }}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
    });

    await userEvent.click(screen.getByRole('button', { name: 'Expand debug overlay' }));

    const debugOverlay = screen.getByLabelText('Debug Visualization Mode');
    expect(debugOverlay).toHaveTextContent('Resource layers 8 of 27');
    expect(debugOverlay).toHaveTextContent('+19 resource layers hidden');
    expect(debugOverlay).toHaveTextContent('resource_0 total 1');
    expect(debugOverlay).not.toHaveTextContent('Layer 26');
  });

  it('shows loading debug projections without marking resources missing', async () => {
    render(
      <WorldViewer
        frame={{ ...ui1aFixture.frame, source: 'live', resources: [] }}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
        debugProjections={{
          status: 'loading',
          runId: ui1aFixture.frame.runId,
          requestedTick: ui1aFixture.frame.tick,
          reason: 'Waiting for Observer debug projection'
        } as unknown as DebugProjectionState}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
    });

    await userEvent.click(screen.getByRole('button', { name: 'Expand debug overlay' }));

    expect(screen.getByLabelText('Viewer projection truth')).toHaveTextContent('Loading projection');
    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('Waiting for Observer debug projection');
    expect(screen.queryByText('Missing live projection')).not.toBeInTheDocument();
  });

  it('filters source-backed cells and resource layers from the map search box', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
        debugProjections={debugProjections}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
    });

    await userEvent.click(screen.getByRole('button', { name: 'Expand debug overlay' }));

    fireEvent.change(screen.getByLabelText('Search cells or resource layers'), {
      target: { value: 'cell-c' }
    });

    expect(screen.getByLabelText('Select cell-c')).toBeVisible();
    expect(screen.getByLabelText('Select cell-a')).toHaveAttribute('aria-hidden', 'true');
    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('Search match: cell-c');
    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('Unsupported: process/contact/history search');

    fireEvent.change(screen.getByLabelText('Search cells or resource layers'), {
      target: { value: 'nucleotide' }
    });

    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('nucleotide_precursor');
  });

  it('offers Fit World without a separate Reset navigation control', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
    });

    expect(screen.getByRole('button', { name: 'Fit World Viewer' })).toBeVisible();
    expect(screen.queryByRole('button', { name: 'Reset World Viewer navigation' })).not.toBeInTheDocument();
  });

  it('keeps hit targets aligned with the navigation camera', async () => {
    const user = userEvent.setup();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await user.click(screen.getByRole('button', { name: 'Zoom in World Viewer' }));

    expect(screen.getByLabelText('Select cell-a')).toHaveStyle({
      left: '276px',
      top: '304px',
      width: '57.599999999999994px',
      height: '57.599999999999994px'
    });
  });

  it('pans by dragging the World Viewer surface', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    const viewer = screen.getByLabelText('World Viewer');
    await waitFor(() => {
      expect(viewer).toHaveAttribute('data-ready', 'true');
    });

    fireEvent.mouseDown(viewer, { button: 0, clientX: 100, clientY: 100 });
    fireEvent.mouseMove(viewer, { clientX: 130, clientY: 80 });
    fireEvent.mouseUp(viewer, { clientX: 130, clientY: 80 });

    await waitFor(() => {
      expect(renderFrame).toHaveBeenLastCalledWith(ui1aFixture.frame, 'cell-a', {
        x: 30,
        y: -20,
        scale: 1
      });
    });
  });

  it('aligns hit targets to the measured canvas viewport instead of the world size', async () => {
    const clientWidth = vi.spyOn(HTMLElement.prototype, 'clientWidth', 'get').mockImplementation(function getClientWidth(this: HTMLElement) {
      return this.classList.contains('world-canvas-host') ? 900 : 0;
    });
    const clientHeight = vi.spyOn(HTMLElement.prototype, 'clientHeight', 'get').mockImplementation(function getClientHeight(this: HTMLElement) {
      return this.classList.contains('world-canvas-host') ? 560 : 0;
    });

    try {
      render(
        <WorldViewer
          frame={ui1aFixture.frame}
          selectedCellId="cell-a"
          onSelectCell={vi.fn()}
        />
      );

      await waitFor(() => {
        expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
      });

      expect(screen.getByLabelText('Select cell-a')).toHaveStyle({
        left: '247.5px',
        top: '224px',
        width: '36px',
        height: '36px'
      });
    } finally {
      clientWidth.mockRestore();
      clientHeight.mockRestore();
    }
  });

  it('cancels browser text selection while panning the World Viewer surface', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    const viewer = screen.getByLabelText('World Viewer');
    await waitFor(() => {
      expect(viewer).toHaveAttribute('data-ready', 'true');
    });

    expect(fireEvent.mouseDown(viewer, { button: 0, clientX: 100, clientY: 100 })).toBe(false);
  });

  it('dismisses projection notices when the empty viewer surface is clicked', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
    });
    expect(screen.getByLabelText('Viewer projection truth')).toBeInTheDocument();

    fireEvent.click(screen.getByLabelText('World cell hit targets'));

    expect(screen.queryByLabelText('Viewer projection truth')).not.toBeInTheDocument();
  });

  it('clears selected Cell state when the empty viewer surface is clicked', async () => {
    const onSelectCell = vi.fn();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={onSelectCell}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
    });

    fireEvent.click(screen.getByLabelText('World cell hit targets'));

    expect(onSelectCell).toHaveBeenCalledWith(null);
  });

  it('keeps cell selection available after panning the World Viewer surface', async () => {
    const onSelectCell = vi.fn();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={onSelectCell}
      />
    );

    const viewer = screen.getByLabelText('World Viewer');
    await waitFor(() => {
      expect(viewer).toHaveAttribute('data-ready', 'true');
    });

    fireEvent.pointerDown(viewer, { button: 0, pointerId: 1, clientX: 100, clientY: 100 });
    fireEvent.pointerMove(viewer, { pointerId: 1, clientX: 130, clientY: 80 });
    fireEvent.pointerUp(viewer, { pointerId: 1, clientX: 130, clientY: 80 });
    fireEvent.click(screen.getByLabelText('Select cell-c'));

    expect(onSelectCell).toHaveBeenCalledWith('cell-c');
  });

  it('shows data-bound selected Cell detail label without changing selection behavior', async () => {
    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await waitFor(() => {
      expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
    });

    expect(screen.getByLabelText('Selected cell detail label')).toHaveTextContent('cell-a · E82 · I91');
    expect(screen.getByLabelText('Select cell-a')).toHaveAttribute('data-semantic-level', 'structure');
  });

  it('shows zoomed semantic labels for sufficiently large visible Cells', async () => {
    const user = userEvent.setup();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await user.click(screen.getByRole('button', { name: 'Zoom in World Viewer' }));
    await user.click(screen.getByRole('button', { name: 'Zoom in World Viewer' }));

    expect(screen.getByLabelText('Selected cell detail label')).toHaveTextContent('cell-a · E82 · I91');
  });
});
