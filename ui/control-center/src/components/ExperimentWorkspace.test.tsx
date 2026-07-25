import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { ExperimentWorkspace } from './ExperimentWorkspace';
import { createAppStore } from '../app/appState';

describe('ExperimentWorkspace', () => {
  it('renders experiment workspace header and side-by-side comparison table', async () => {
    const store = createAppStore();
    const state = store.getState();
    const user = userEvent.setup();

    render(<ExperimentWorkspace state={state} />);

    expect(screen.getByText(/Experiments & Run Comparison/i)).toBeInTheDocument();
    expect(screen.getByText(/Side-by-Side Metrics Comparison/i)).toBeInTheDocument();

    const selects = screen.getAllByRole('combobox');
    expect(selects.length).toBe(2);

    await user.selectOptions(selects[1], 'preset-run-diverse-rich');
    expect(screen.getAllByText(/diverse_rich_world/i).length).toBeGreaterThan(0);
  });
});
