import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { ScenarioPicker } from './ScenarioPicker';

describe('ScenarioPicker', () => {
  const scenarios = [
    { id: 'bootstrap_minimal_viable_world' },
    { id: 'living_ecosystem' },
    { id: 'very_long_scenario_name_that_must_not_force_the_run_bar_to_expand' }
  ];

  it('opens a dark readable listbox and marks the selected scenario', async () => {
    const user = userEvent.setup();
    render(
      <ScenarioPicker
        scenarios={scenarios}
        selectedScenarioId="living_ecosystem"
        onScenarioChange={vi.fn()}
      />
    );

    await user.click(screen.getByRole('button', { name: /scenario/i }));

    const listbox = screen.getByRole('listbox', { name: /scenario/i });
    expect(listbox).toHaveClass('cc-scenario-picker-listbox');
    expect(listbox).toHaveAttribute('data-theme', 'dark');
    expect(screen.getByRole('option', { name: 'living_ecosystem' })).toHaveAttribute('aria-selected', 'true');
  });

  it('uses bounded trigger text while preserving full scenario id in title', () => {
    render(
      <ScenarioPicker
        scenarios={scenarios}
        selectedScenarioId="very_long_scenario_name_that_must_not_force_the_run_bar_to_expand"
        onScenarioChange={vi.fn()}
      />
    );

    const trigger = screen.getByRole('button', { name: /scenario/i });
    expect(trigger).toHaveClass('cc-scenario-picker-trigger');
    expect(trigger).toHaveAttribute('title', 'very_long_scenario_name_that_must_not_force_the_run_bar_to_expand');
  });

  it('renders the open listbox outside the run bar so options remain clickable beyond the bar bounds', async () => {
    const user = userEvent.setup();
    const onScenarioChange = vi.fn();

    render(
      <div data-testid="run-bar-shell">
        <ScenarioPicker
          scenarios={scenarios}
          selectedScenarioId="living_ecosystem"
          onScenarioChange={onScenarioChange}
        />
      </div>
    );

    await user.click(screen.getByRole('button', { name: /scenario/i }));

    const listbox = screen.getByRole('listbox', { name: /scenario/i });
    expect(listbox.parentElement).toBe(document.body);

    await user.click(screen.getByRole('option', { name: 'bootstrap_minimal_viable_world' }));
    expect(onScenarioChange).toHaveBeenCalledWith('bootstrap_minimal_viable_world');
  });
});
