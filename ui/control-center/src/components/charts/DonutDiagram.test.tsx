import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { DonutDiagram } from './DonutDiagram';

describe('DonutDiagram', () => {
  it('renders empty state when total value is 0', () => {
    render(<DonutDiagram segments={[{ label: 'Alive', value: 0, color: '#4ade80' }]} />);
    expect(screen.getByText('No data')).toBeInTheDocument();
  });

  it('renders arcs and legend items for valid segment data', () => {
    const segments = [
      { label: 'Alive', value: 80, color: '#4ade80' },
      { label: 'Stressed', value: 20, color: '#fb7185' }
    ];

    render(<DonutDiagram segments={segments} centerText="100" centerSubtext="Cells" />);

    expect(screen.getByText('100')).toBeInTheDocument();
    expect(screen.getByText('Cells')).toBeInTheDocument();
    expect(screen.getByText('Alive:')).toBeInTheDocument();
    expect(screen.getByText('Stressed:')).toBeInTheDocument();
    expect(document.querySelectorAll('circle').length).toBe(2);
  });
});
