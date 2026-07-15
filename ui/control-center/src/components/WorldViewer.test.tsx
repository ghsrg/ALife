import { createRef } from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { ui1aFixture } from '../fixtures/ui1aFixture';
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
});
