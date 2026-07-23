import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it } from 'vitest';
import { CellInspector } from './CellInspector';

describe('CellInspector', () => {
  it('shows raw energy, materials, internal resources, and local external resources', () => {
    render(
      <CellInspector
        selectedCell={{
          id: '12',
          x: 10,
          y: 20,
          radius: 2,
          energy: 0.5,
          energyRaw: 10,
          energyCapacity: 20,
          integrity: 1,
          generation: 0,
          roleHint: 'Alive',
          materials: [{ materialTypeId: 1, amount: 2.5 }],
          internalResources: [{ resourceTypeId: 0, amount: 1.5 }],
          localExternalResources: [{ resourceTypeId: 0, amount: 3.5 }]
        }}
      />
    );

    expect(screen.getByText('10 / 20 (50%)')).toBeInTheDocument();
    expect(screen.getByText('Material 1')).toBeInTheDocument();
    expect(screen.getByText('2.50')).toBeInTheDocument();
    expect(screen.getByText('Internal resource 0')).toBeInTheDocument();
    expect(screen.getByText('Local external resource 0')).toBeInTheDocument();
    expect(screen.getByText('3.50')).toBeInTheDocument();
  });

  it('bounds long source-backed material and resource sections', () => {
    render(
      <CellInspector
        selectedCell={{
          id: 'bounded',
          x: 10,
          y: 20,
          radius: 2,
          energy: 0.5,
          energyRaw: 10,
          energyCapacity: 20,
          integrity: 1,
          generation: 0,
          roleHint: 'Alive',
          materials: Array.from({ length: 9 }, (_, materialTypeId) => ({ materialTypeId, amount: 1 })),
          internalResources: Array.from({ length: 9 }, (_, resourceTypeId) => ({ resourceTypeId, amount: 2 })),
          localExternalResources: Array.from({ length: 9 }, (_, resourceTypeId) => ({ resourceTypeId, amount: 3 }))
        }}
      />
    );

    expect(screen.getByText('Materials (9)')).toBeInTheDocument();
    expect(screen.getByText('Internal resources (9)')).toBeInTheDocument();
    expect(screen.getByText('Local external resources (9)')).toBeInTheDocument();
    expect(screen.getAllByText('+3 more')).toHaveLength(3);
    expect(screen.queryByText('Material 8')).not.toBeInTheDocument();
    expect(screen.queryByText('Local external resource 8')).not.toBeInTheDocument();
  });

  it('pins a selected Cell and compares it with the current selection', async () => {
    const user = userEvent.setup();
    const pinnedCell = {
      id: 'pinned',
      x: 10,
      y: 20,
      radius: 2,
      energy: 0.5,
      energyRaw: 10,
      energyCapacity: 20,
      integrity: 1,
      generation: 0,
      roleHint: 'Alive',
      materials: [{ materialTypeId: 1, amount: 2.5 }],
      internalResources: [{ resourceTypeId: 0, amount: 1.5 }],
      localExternalResources: [{ resourceTypeId: 0, amount: 3.5 }]
    };
    const currentCell = {
      ...pinnedCell,
      id: 'current',
      x: 30,
      y: 40,
      energyRaw: 14,
      materials: [{ materialTypeId: 1, amount: 4 }]
    };
    const { rerender } = render(<CellInspector selectedCell={pinnedCell} />);

    await user.click(screen.getByRole('button', { name: 'Pin selected Cell' }));
    rerender(<CellInspector selectedCell={currentCell} />);

    expect(screen.getByLabelText('Pinned Cell comparison')).toHaveTextContent('pinned');
    expect(screen.getByLabelText('Pinned Cell comparison')).toHaveTextContent('current');
    expect(screen.getByLabelText('Pinned Cell comparison')).toHaveTextContent('Energy delta 4');
    expect(screen.getByRole('button', { name: 'Clear pinned Cell' })).toBeEnabled();
  });
});
