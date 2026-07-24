import { describe, expect, it } from 'vitest';
import { buildBalanceViewModel } from './balanceViewModel';
import type { AppStore } from './appState';

describe('buildBalanceViewModel', () => {
  it('returns empty balance view model when no live frame exists', () => {
    const mockState: Partial<AppStore> = {
      latestLiveFrame: null
    };

    const vm = buildBalanceViewModel(mockState as AppStore);
    expect(vm.hasData).toBe(false);
    expect(vm.matterCycle.totalSystemMatter).toBe(0);
    expect(vm.population.total).toBe(0);
  });

  it('computes matter cycle and unaccounted difference correctly from live frame', () => {
    const mockState: Partial<AppStore> = {
      latestLiveFrame: {
        schemaVersion: 'WorldFrameProjection/v1',
        runId: 'run-1',
        tick: 120,
        world: { width: 100, height: 100 },
        resources: [
          [{ organic: 10, mineral: 5, energy: 2 }],
          [{ organic: 15, mineral: 5, energy: 3 }]
        ],
        cells: [
          {
            id: '1',
            x: 10,
            y: 10,
            radius: 1.5,
            energy: 8.0,
            energyCapacity: 10.0,
            integrity: 1.0,
            lifecycle: 0,
            generation: 0,
            roleHint: 'stem',
            internalResources: [{ resourceTypeId: 0, amount: 2.0 }, { resourceTypeId: 1, amount: 1.0 }],
            materials: [{ materialTypeId: 0, amount: 3.0 }, { materialTypeId: 1, amount: 2.0 }]
          },
          {
            id: '2',
            x: 20,
            y: 20,
            radius: 1.2,
            energy: 1.0,
            energyCapacity: 10.0,
            integrity: 0.4,
            lifecycle: 1,
            generation: 0,
            roleHint: 'stem',
            internalResources: [{ resourceTypeId: 0, amount: 0.5 }, { resourceTypeId: 1, amount: 0.5 }],
            materials: [{ materialTypeId: 0, amount: 1.5 }, { materialTypeId: 1, amount: 1.0 }]
          }
        ]
      }
    };

    const vm = buildBalanceViewModel(mockState as AppStore);

    expect(vm.hasData).toBe(true);
    expect(vm.tick).toBe(120);
    expect(vm.matterCycle.environmentOrganic).toBe(25);
    expect(vm.matterCycle.environmentMineral).toBe(10);
    expect(vm.matterCycle.cellInternalOrganic).toBe(2.5);
    expect(vm.matterCycle.cellInternalMineral).toBe(1.5);
    expect(vm.matterCycle.cellBoundMaterials).toBe(7.5);
    expect(vm.matterCycle.totalSystemMatter).toBe(46.5);
    expect(vm.matterCycle.unaccountedDiff).toBeDefined();

    expect(vm.population.total).toBe(2);
    expect(vm.population.alive).toBe(1);
    expect(vm.population.stressed).toBe(1);
    expect(vm.population.dormant).toBe(0);
    expect(vm.population.dead).toBe(0);

    expect(vm.warnings.length).toBe(1);
    expect(vm.warnings[0].title).toContain('Low Energy Warning (Cell #2)');
  });
});
