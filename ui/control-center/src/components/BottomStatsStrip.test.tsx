import { screen, within } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { renderApp } from '../test/render';
import { BottomStatsStrip } from './BottomStatsStrip';
import type { MonitorStat } from './monitorStats';

const stats: MonitorStat[] = [
  { id: 'cells', label: 'Cells', value: '192', state: 'available' },
  { id: 'alive-dead', label: 'Alive / Dead', value: '180 / 12', state: 'available' },
  { id: 'cell-energy', label: 'Projected Cell Energy', value: '124.50', state: 'available', note: 'sum of projected cell buffers' },
  { id: 'world', label: 'World', value: '1200 x 800', state: 'available' },
  { id: 'resources', label: 'Resources', value: 'Missing projection', state: 'missing', note: 'Runner ALIF v2 does not include resource grid' }
];

describe('BottomStatsStrip', () => {
  it('renders compact world stats and marks missing projection explicitly', () => {
    renderApp(<BottomStatsStrip stats={stats} />);

    const strip = screen.getByLabelText('World stats');
    expect(within(strip).getByText('Cells')).toBeInTheDocument();
    expect(within(strip).getByText('192')).toBeInTheDocument();
    expect(within(strip).getByText('Resources')).toBeInTheDocument();
    expect(within(strip).getByText('Missing projection')).toBeInTheDocument();
    expect(within(strip).getByText('Runner ALIF v2 does not include resource grid')).toBeInTheDocument();
  });

  it('does not render more than five stat cells', () => {
    renderApp(<BottomStatsStrip stats={[...stats, { id: 'cells', label: 'Extra', value: '1', state: 'available' }]} />);

    expect(screen.getAllByTestId('bottom-stat')).toHaveLength(5);
  });
});
