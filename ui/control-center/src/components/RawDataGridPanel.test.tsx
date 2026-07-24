import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import type { WorldFrame } from '../projection/types';
import { RawDataGridPanel } from './RawDataGridPanel';

describe('RawDataGridPanel', () => {
  const mockFrame: WorldFrame = {
    schemaVersion: 'WorldFrameProjection/v1',
    runId: 'run-1',
    tick: 100,
    world: { width: 100, height: 100 },
    resources: [],
    cells: [
      {
        id: '101',
        x: 12.5,
        y: 34.0,
        radius: 1.4,
        energy: 9.5,
        energyCapacity: 12.0,
        integrity: 0.95,
        generation: 0,
        roleHint: 'stem',
        internalResources: [{ resourceTypeId: 0, amount: 2.0 }],
        materials: [{ materialTypeId: 0, amount: 3.0 }]
      },
      {
        id: '102',
        x: 45.0,
        y: 50.0,
        radius: 1.1,
        energy: 1.2,
        energyCapacity: 10.0,
        integrity: 0.3,
        generation: 0,
        roleHint: 'stem',
        internalResources: [{ resourceTypeId: 0, amount: 0.1 }],
        materials: [{ materialTypeId: 0, amount: 1.0 }]
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
    expect(onSelectCell).toHaveBeenCalledWith('102');
  });
});
