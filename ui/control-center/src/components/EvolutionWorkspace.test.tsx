import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { EvolutionWorkspace } from './EvolutionWorkspace';
import { createAppStore } from '../app/appState';

describe('EvolutionWorkspace', () => {
  it('renders evolution observatory header and generation distribution metrics', () => {
    const store = createAppStore();
    const state = store.getState();

    render(<EvolutionWorkspace state={state} />);

    expect(screen.getByText(/Evolution Observatory/i)).toBeInTheDocument();
    expect(screen.getByText(/Diversity Index/i)).toBeInTheDocument();
    expect(screen.getByText(/Lineage & Generation Distribution/i)).toBeInTheDocument();
  });
});
