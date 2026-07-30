import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { LevelPanel } from './LevelPanel';

describe('LevelPanel', () => {
  it('renders canonical level icons instead of letter glyphs', () => {
    render(<LevelPanel activeLevel="world" onLevelChange={vi.fn()} />);

    const levels = screen.getAllByRole('button');
    expect(levels.map((level) => level.getAttribute('aria-label'))).toEqual([
      'World level',
      'Cells level',
      'Organisms level',
      'Lineages level',
      'Evolution level',
      'Analytics level'
    ]);

    expect(screen.getByTestId('level-icon-world')).toBeInTheDocument();
    expect(screen.getByTestId('level-icon-cells')).toBeInTheDocument();
    expect(screen.getByTestId('level-icon-organisms')).toBeInTheDocument();
    expect(screen.getByTestId('level-icon-lineages')).toBeInTheDocument();
    expect(screen.getByTestId('level-icon-evolution')).toBeInTheDocument();
    expect(screen.getByTestId('level-icon-analytics')).toBeInTheDocument();
    expect(screen.queryByText('W')).not.toBeInTheDocument();
    expect(screen.queryByText('C')).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'World level' })).toHaveAttribute('aria-pressed', 'true');
  });
});
