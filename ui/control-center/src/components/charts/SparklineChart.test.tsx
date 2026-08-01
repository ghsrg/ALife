import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { SparklineChart } from './SparklineChart';

describe('SparklineChart', () => {
  it('renders empty message when no history is provided', () => {
    render(<SparklineChart history={[]} series={[{ key: 'val', label: 'Value', color: '#00c896' }]} />);
    expect(screen.getByText('No time-series data')).toBeInTheDocument();
  });

  it('renders SVG lines when metric history is present', () => {
    const history = [
      { tick: 1, startTick: 1, endTick: 1, value: 10, count: 1, kind: 'raw' as const },
      { tick: 2, startTick: 2, endTick: 2, value: 20, count: 1, kind: 'raw' as const },
      { tick: 3, startTick: 3, endTick: 3, value: 15, count: 1, kind: 'raw' as const }
    ];

    render(
      <SparklineChart
        history={history}
        series={[{ key: 'value', label: 'Energy', color: '#00c896' }]}
        title="Energy Trend"
      />
    );

    expect(screen.getByText('Energy Trend')).toBeInTheDocument();
    expect(document.querySelector('.sparkline-svg')).toBeInTheDocument();
    expect(document.querySelectorAll('path').length).toBeGreaterThan(0);
  });
});
