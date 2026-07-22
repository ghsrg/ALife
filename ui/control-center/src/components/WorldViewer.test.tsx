import { createRef } from 'react';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import type { WorldFrame } from '../projection/types';
import type { DebugProjectionState } from '../projection/types';
import { WorldViewer, type WorldViewerHandle } from './WorldViewer';

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
      expect(renderFrame).toHaveBeenCalledWith(ui1aFixture.frame, 'cell-a', { x: 36, y: 24, scale: 0.94 });
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
      expect(renderFrame).toHaveBeenCalledWith(tinyLiveFrame, 'tiny', { x: 36, y: 24, scale: 0.94 });
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
        x: -76.79999999999995,
        y: -51.19999999999993,
        scale: 1.128
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

    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('Debug');
    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('Exact');
    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('VisualWorldProjection');
    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('Tick 213');
    expect(screen.getByLabelText('Debug Visualization Mode')).toHaveTextContent('cells.materials');
    expect(screen.getByRole('button', { name: 'Switch debug layers to Smooth interpolation' })).toBeEnabled();
    expect(screen.getByRole('checkbox', { name: 'Spatial index overlay unavailable' })).toBeDisabled();
    expect(screen.getByText('Missing live projection')).toBeVisible();
  });

  it('resets navigation to the default camera', async () => {
    const user = userEvent.setup();

    render(
      <WorldViewer
        frame={ui1aFixture.frame}
        selectedCellId="cell-a"
        onSelectCell={vi.fn()}
      />
    );

    await user.click(screen.getByRole('button', { name: 'Zoom in World Viewer' }));
    await user.click(screen.getByRole('button', { name: 'Reset World Viewer navigation' }));

    await waitFor(() => {
      expect(renderFrame).toHaveBeenLastCalledWith(ui1aFixture.frame, 'cell-a', {
        x: 0,
        y: 0,
        scale: 1
      });
    });
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
      left: '295.44px',
      top: '309.76000000000005px',
      width: '54.14399999999999px',
      height: '54.14399999999999px'
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
        x: 66,
        y: 4,
        scale: 0.94
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
        left: '224.4px',
        top: '167.36px',
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
