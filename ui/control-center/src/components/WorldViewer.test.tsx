import { createRef } from 'react';
import { render, screen, waitFor } from '@testing-library/react';
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
      expect(renderFrame).toHaveBeenCalledWith(ui1aFixture.frame, 'cell-a');
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
      expect(renderFrame).toHaveBeenCalledWith(tinyLiveFrame, 'tiny');
    });

    expect(screen.getByLabelText('Select tiny')).toHaveStyle({ width: '14px', height: '14px' });
    expect(screen.getByLabelText('Viewer projection truth')).toHaveTextContent('Missing projection');
    expect(screen.getByLabelText('Viewer projection truth')).toHaveTextContent('Display minimum applied');
  });
});
