import { render, screen, fireEvent } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { createAppStore } from '../app/appState';
import { WorldEditorWorkspace } from './WorldEditorWorkspace';

describe('WorldEditorWorkspace', () => {
  it('renders World Editor header and preset dropdown', () => {
    const store = createAppStore();
    render(<WorldEditorWorkspace state={store.getState()} />);

    expect(screen.getByText('WORLD EDITOR & SCENARIO RUNNER')).toBeInTheDocument();
    expect(screen.getByLabelText('Scenario Preset')).toBeInTheDocument();
    expect(screen.getByLabelText('TOML Configuration Editor')).toBeInTheDocument();
    expect(screen.getByText('✓ TOML Configuration Valid')).toBeInTheDocument();
  });

  it('allows changing preset and editing TOML text', () => {
    const store = createAppStore();
    render(<WorldEditorWorkspace state={store.getState()} />);

    const textarea = screen.getByLabelText('TOML Configuration Editor') as HTMLTextAreaElement;
    fireEvent.change(textarea, { target: { value: '[world]\nsize = [200.0, 200.0]' } });

    expect(textarea.value).toBe('[world]\nsize = [200.0, 200.0]');
  });

  it('triggers relaunch on Relaunch Simulation click', () => {
    const store = createAppStore();
    const handleRelaunch = vi.fn();
    render(<WorldEditorWorkspace state={store.getState()} onRelaunchRun={handleRelaunch} />);

    const relaunchBtn = screen.getByRole('button', { name: /Relaunch Simulation/i });
    fireEvent.click(relaunchBtn);

    expect(handleRelaunch).toHaveBeenCalledTimes(1);
    expect(handleRelaunch).toHaveBeenCalledWith('diverse_rich_world', 42);
  });
});
