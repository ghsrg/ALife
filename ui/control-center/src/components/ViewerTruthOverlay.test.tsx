import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import type { ViewerTruthState } from './viewerTruth';
import { ViewerTruthOverlay } from './ViewerTruthOverlay';

const truthState: ViewerTruthState = {
  resourceLayer: {
    state: 'missing',
    label: 'Resources',
    value: 'Missing projection',
    note: 'Runner ALIF v2 does not include resource grid'
  },
  cellScale: {
    state: 'presentation-minimum',
    label: 'Cell size',
    value: 'Display minimum applied',
    note: '2 of 5 cells enlarged for visibility'
  }
};

describe('ViewerTruthOverlay', () => {
  it('renders projection truth labels and notes', () => {
    render(<ViewerTruthOverlay truthState={truthState} />);

    const overlay = screen.getByLabelText('Viewer projection truth');
    expect(overlay).toHaveTextContent('Resources');
    expect(overlay).toHaveTextContent('Missing projection');
    expect(overlay).toHaveTextContent('Cell size');
    expect(overlay).toHaveTextContent('Display minimum applied');
  });

  it('can be dismissed through an explicit close button', async () => {
    const onDismiss = vi.fn();
    const user = userEvent.setup();

    render(<ViewerTruthOverlay truthState={truthState} onDismiss={onDismiss} />);

    await user.click(screen.getByRole('button', { name: 'Dismiss projection notices' }));

    expect(onDismiss).toHaveBeenCalledTimes(1);
  });
});
