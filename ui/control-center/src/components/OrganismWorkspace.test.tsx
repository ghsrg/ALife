import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { OrganismWorkspace } from './OrganismWorkspace';
import { createAppStore } from '../app/appState';

describe('OrganismWorkspace', () => {
  it('renders organism observatory header and detected organisms list', () => {
    const store = createAppStore();
    const state = store.getState();

    render(<OrganismWorkspace state={state} />);

    expect(screen.getByText(/Organism Observatory/i)).toBeInTheDocument();
    expect(screen.getByText(/Detected Organisms/i)).toBeInTheDocument();
  });
});
