import { render, screen } from '@testing-library/react';
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
});
