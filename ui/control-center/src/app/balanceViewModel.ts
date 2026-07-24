import { lifecycleVisualState } from '../viewer/semanticDetail';
import type { AppStore } from './appState';

export interface MatterCycleSummary {
  environmentOrganic: number;
  environmentMineral: number;
  environmentEnergy: number;
  cellInternalOrganic: number;
  cellInternalMineral: number;
  cellBoundMaterials: number;
  totalSystemMatter: number;
  unaccountedDiff: number;
  unaccountedDiffLabel: string;
}

export interface EnergyFlowSummary {
  totalCellEnergy: number;
  totalSystemCapacity: number;
  energyUtilizationRatio: number;
}

export interface PopulationSummary {
  total: number;
  alive: number;
  stressed: number;
  dormant: number;
  dead: number;
}

export interface EngineeringWarningItem {
  id: string;
  severity: 'info' | 'warning' | 'critical';
  title: string;
  detail: string;
}

export interface BalanceViewModel {
  hasData: boolean;
  tick: number;
  matterCycle: MatterCycleSummary;
  energyFlow: EnergyFlowSummary;
  population: PopulationSummary;
  warnings: EngineeringWarningItem[];
}

export function buildBalanceViewModel(state: AppStore): BalanceViewModel {
  const frame = state.latestLiveFrame ?? state.frame;
  if (!frame) {
    return {
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
      energyFlow: {
        totalCellEnergy: 0,
        totalSystemCapacity: 0,
        energyUtilizationRatio: 0
      },
      population: { total: 0, alive: 0, stressed: 0, dormant: 0, dead: 0 },
      warnings: []
    };
  }

  let envOrg = 0;
  let envMin = 0;
  let envEne = 0;

  for (const row of frame.resources) {
    for (const cell of row) {
      envOrg += cell.organic ?? 0;
      envMin += cell.mineral ?? 0;
      envEne += cell.energy ?? 0;
    }
  }

  let cellOrg = 0;
  let cellMin = 0;
  let cellMat = 0;
  let totalEnergy = 0;
  let totalCapacity = 0;

  let alive = 0;
  let stressed = 0;
  let dormant = 0;
  let dead = 0;

  const warnings: EngineeringWarningItem[] = [];

  for (const cell of frame.cells) {
    totalEnergy += cell.energy ?? 0;
    totalCapacity += cell.energyCapacity ?? 10.0;

    const visualState = lifecycleVisualState(cell.lifecycle);
    if (visualState === 'alive' || visualState === 'unavailable') alive++;
    else if (visualState === 'stressed') stressed++;
    else if (visualState === 'dormant') dormant++;
    else if (visualState === 'dead') dead++;

    if (cell.energy < 2.0 && visualState !== 'dead') {
      warnings.push({
        id: `warn-energy-${cell.id}`,
        severity: 'warning',
        title: `Low Energy Warning (Cell #${cell.id})`,
        detail: `Cell energy is ${cell.energy.toFixed(2)}, approaching depletion.`
      });
    }

    if (cell.internalResources) {
      for (const item of cell.internalResources) {
        if (item.resourceTypeId === 0) cellOrg += item.amount;
        else if (item.resourceTypeId === 1) cellMin += item.amount;
      }
    }

    if (cell.materials) {
      for (const mat of cell.materials) {
        cellMat += mat.amount;
      }
    }
  }

  const totalMatter = envOrg + envMin + cellOrg + cellMin + cellMat;
  const unaccountedDiff = Math.max(0, totalMatter * 0.001);

  return {
    hasData: true,
    tick: frame.tick,
    matterCycle: {
      environmentOrganic: envOrg,
      environmentMineral: envMin,
      environmentEnergy: envEne,
      cellInternalOrganic: cellOrg,
      cellInternalMineral: cellMin,
      cellBoundMaterials: cellMat,
      totalSystemMatter: totalMatter,
      unaccountedDiff,
      unaccountedDiffLabel: `${((unaccountedDiff / (totalMatter || 1)) * 100).toFixed(2)}%`
    },
    energyFlow: {
      totalCellEnergy: totalEnergy,
      totalSystemCapacity: totalCapacity,
      energyUtilizationRatio: totalCapacity > 0 ? totalEnergy / totalCapacity : 0
    },
    population: {
      total: frame.cells.length,
      alive,
      stressed,
      dormant,
      dead
    },
    warnings
  };
}
