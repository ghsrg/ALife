import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { App } from './App';
import { renderApp } from './test/render';

vi.mock('./viewer/worldRenderer', () => ({
  mountWorldRenderer: vi.fn(() => Promise.resolve({
    renderFrame: vi.fn(),
    resize: vi.fn(),
    exportPng: vi.fn(() => 'data:image/png;base64,fixture'),
    destroy: vi.fn()
  }))
}));

describe('App', () => {
  it('renders the Monitor layout with fixture data and selected Cell Inspector', async () => {
    renderApp(<App />);

    expect(screen.getByRole('heading', { name: /alife control center/i })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: /monitor/i })).toHaveAttribute('aria-selected', 'true');
    expect(screen.getByText('UI-1A Deterministic Fixture')).toBeInTheDocument();
    expect(screen.getByText('Tick 128')).toBeInTheDocument();
    const inspector = screen.getByLabelText(/cell inspector/i);
    expect(within(inspector).getByRole('heading', { name: /cell inspector/i })).toBeInTheDocument();
    expect(within(inspector).getByText('cell-a')).toBeInTheDocument();
    await waitFor(() => {
      expect(screen.getByLabelText(/world viewer/i)).toHaveAttribute('data-ready', 'true');
    });
  });

  it('exports a viewer PNG from the toolbar', async () => {
    const user = userEvent.setup();
    renderApp(<App />);

    await waitFor(() => {
      expect(screen.getByLabelText(/world viewer/i)).toHaveAttribute('data-ready', 'true');
    });
    await user.click(screen.getByRole('button', { name: /export viewer png/i }));

    expect(screen.getByRole('status')).toHaveTextContent(/png ready/i);
  });
});
