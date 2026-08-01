import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { HistogramChart } from './HistogramChart';

describe('HistogramChart', () => {
  it('renders empty message when no bins provided', () => {
    render(<HistogramChart bins={[]} />);
    expect(screen.getByText('No distribution data available')).toBeInTheDocument();
  });

  it('renders histogram bars for provided bins', () => {
    const bins = [
      { label: '<10px', count: 5 },
      { label: '10-20px', count: 18 },
      { label: '>20px', count: 7 }
    ];

    render(<HistogramChart bins={bins} title="Size Distribution" />);

    expect(screen.getByText('Size Distribution')).toBeInTheDocument();
    expect(screen.getByText('<10px')).toBeInTheDocument();
    expect(screen.getByText('10-20px')).toBeInTheDocument();
    expect(screen.getByText('>20px')).toBeInTheDocument();
    expect(document.querySelectorAll('.histogram-bar').length).toBe(3);
  });
});
