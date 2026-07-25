import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { LibraryWorkspace } from './LibraryWorkspace';
import { createAppStore } from '../app/appState';

describe('LibraryWorkspace', () => {
  it('renders library templates and research report export panel', () => {
    const store = createAppStore();
    const state = store.getState();

    render(<LibraryWorkspace state={state} />);

    expect(screen.getByText(/Library, Placement & Research Export/i)).toBeInTheDocument();
    expect(screen.getByText(/Single Transport Cell/i)).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: /Reproducible Research Report/i })).toBeInTheDocument();
  });
});
