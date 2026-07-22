import { screen, within } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import type { CellProjection } from '../projection/types';
import { renderApp } from '../test/render';
import { SelectedEntityFocusCard } from './SelectedEntityFocusCard';

const selectedCell: CellProjection = {
  id: '42',
  x: 100,
  y: 200,
  radius: 6,
  energy: 0.75,
  integrity: 1,
  generation: 0,
  roleHint: 'alive lifecycle state',
  lifecycle: 0
};

describe('SelectedEntityFocusCard', () => {
  it('renders data-bound selected cell summary', () => {
    renderApp(<SelectedEntityFocusCard selectedCell={selectedCell} />);

    const card = screen.getByLabelText('Selected entity focus');
    expect(within(card).getByText('Cell 42')).toBeInTheDocument();
    expect(within(card).getByText('Position')).toBeInTheDocument();
    expect(within(card).getByText('100, 200')).toBeInTheDocument();
    expect(within(card).getByText('Radius')).toBeInTheDocument();
    expect(within(card).getByText('6')).toBeInTheDocument();
    expect(within(card).getByText('75%')).toBeInTheDocument();
    expect(within(card).getByText('Lifecycle')).toBeInTheDocument();
    expect(within(card).getByText('alive')).toBeInTheDocument();
  });

  it('shows unavailable lifecycle when projection omits lifecycle', () => {
    const { lifecycle: _lifecycle, ...cellWithoutLifecycle } = selectedCell;

    renderApp(<SelectedEntityFocusCard selectedCell={cellWithoutLifecycle} />);

    expect(screen.getByText('Lifecycle')).toBeInTheDocument();
    expect(screen.getByText('Unavailable')).toBeInTheDocument();
  });

  it('renders data-bound energy and integrity bars for selected Cell', () => {
    renderApp(<SelectedEntityFocusCard selectedCell={selectedCell} />);

    expect(screen.getByLabelText('Selected cell energy')).toHaveAttribute('aria-valuenow', '75');
    expect(screen.getByLabelText('Selected cell integrity')).toHaveAttribute('aria-valuenow', '100');
  });

  it('stays out of the layout when no cell is selected', () => {
    const { container } = renderApp(<SelectedEntityFocusCard selectedCell={null} />);

    expect(container).toBeEmptyDOMElement();
  });
});
