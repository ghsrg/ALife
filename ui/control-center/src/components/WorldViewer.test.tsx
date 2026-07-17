import { createRef } from 'react';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import type { WorldFrame } from '../projection/types';
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
      expect(renderFrame).toHaveBeenCalledWith(ui1aFixture.frame, 'cell-a', { x: 0, y: 0, scale: 1 });
    });
    expect(screen.getByLabelText('Select cell-a')).toBeInTheDocument();
    expect(screen.getByLabelText('World Viewer')).toHaveAttribute('data-ready', 'true');
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
      expect(renderFrame).toHaveBeenCalledWith(tinyLiveFrame, 'tiny', { x: 0, y: 0, scale: 1 });
    });

    expect(screen.getByLabelText('Select tiny')).toHaveStyle({ width: '14px', height: '14px' });
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
    expect(screen.getByText('120%')).toBeInTheDocument();
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
