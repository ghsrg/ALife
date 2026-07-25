import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { SpecializationWorkspace } from './SpecializationWorkspace';
import { createAppStore } from '../app/appState';

describe('SpecializationWorkspace', () => {
  it('renders specialization analytics header and role classifiers breakdown table', () => {
    const store = createAppStore();
    const state = store.getState();

    render(<SpecializationWorkspace state={state} />);

    expect(screen.getByText(/Specialization Analytics/i)).toBeInTheDocument();
    expect(screen.getByText(/Dominant Role/i)).toBeInTheDocument();
    expect(screen.getByText(/Functional Role Classifiers & Energy Distribution/i)).toBeInTheDocument();
  });
});
