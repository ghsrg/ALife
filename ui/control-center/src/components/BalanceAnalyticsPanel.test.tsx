import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import type { BalanceViewModel } from '../app/balanceViewModel';
import { BalanceAnalyticsPanel } from './BalanceAnalyticsPanel';

describe('BalanceAnalyticsPanel', () => {
  it('renders empty message when no data is available', () => {
    const emptyVm: BalanceViewModel = {
      hasData: false,
      tick: 0,
      matterCycle: {
        environmentOrganic: 0,
        environmentMineral: 0,
        environmentEnergy: 0,
        cellInternalOrganic: 0,
        cellInternalMineral: 0,
        cellBoundMaterials: 0,
        totalSystemMatter: 0,
        unaccountedDiff: 0,
        unaccountedDiffLabel: '0.00%'
      },
      energyFlow: { totalCellEnergy: 0, totalSystemCapacity: 0, energyUtilizationRatio: 0 },
      population: { total: 0, alive: 0, stressed: 0, dormant: 0, dead: 0 },
      warnings: []
    };

    render(<BalanceAnalyticsPanel viewModel={emptyVm} />);
    expect(screen.getByTestId('balance-analytics-empty')).toBeInTheDocument();
  });

  it('renders matter cycle totals, energy utilization, and warnings', () => {
    const mockViewModel: BalanceViewModel = {
      hasData: true,
      tick: 150,
      matterCycle: {
        environmentOrganic: 45.2,
        environmentMineral: 20.1,
        environmentEnergy: 30.0,
        cellInternalOrganic: 12.0,
        cellInternalMineral: 8.0,
        cellBoundMaterials: 15.5,
        totalSystemMatter: 100.8,
        unaccountedDiff: 0.1,
        unaccountedDiffLabel: '0.10%'
      },
      energyFlow: {
        totalCellEnergy: 75.0,
        totalSystemCapacity: 100.0,
        energyUtilizationRatio: 0.75
      },
      population: {
        total: 5,
        alive: 3,
        stressed: 2,
        dormant: 0,
        dead: 0
      },
      warnings: [
        {
          id: 'warn-1',
          severity: 'warning',
          title: 'Low Energy Warning (Cell #2)',
          detail: 'Energy at 1.5'
        }
      ]
    };

    render(<BalanceAnalyticsPanel viewModel={mockViewModel} />);

    expect(screen.getByText('Matter Cycle Accounting')).toBeInTheDocument();
    expect(screen.getByText('45.20')).toBeInTheDocument();
    expect(screen.getByText('Energy Utilization')).toBeInTheDocument();
    expect(screen.getByText('75.0%')).toBeInTheDocument();
    expect(screen.getByText('Low Energy Warning (Cell #2)')).toBeInTheDocument();
  });
});
