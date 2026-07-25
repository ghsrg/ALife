import { describe, expect, it } from 'vitest';
import { extractOrganismClusters } from './organismModel';
import type { WorldFrame } from '../projection/types';

describe('organismModel', () => {
  it('groups joint-connected cells into organism graph clusters', () => {
    const frame: WorldFrame = {
      schemaVersion: 'WorldFrameProjection/v1',
      runId: 'run-1',
      tick: 100,
      world: { width: 100, height: 100 },
      resources: [],
      cells: [
        { id: 'c1', x: 10, y: 10, radius: 2, energy: 0.5, integrity: 1, generation: 0, roleHint: 'transport' },
        { id: 'c2', x: 12, y: 10, radius: 2, energy: 0.8, integrity: 1, generation: 0, roleHint: 'feeder' },
        { id: 'c3', x: 50, y: 50, radius: 2, energy: 0.4, integrity: 1, generation: 0, roleHint: 'sensor' }
      ],
      joints: [
        { id: 'j1', sourceCellId: 'c1', targetCellId: 'c2', channelType: 'mechanical' }
      ]
    };

    const clusters = extractOrganismClusters(frame);

    expect(clusters.length).toBe(2);
    // Cluster 1 (multi-cell organism)
    expect(clusters[0].cellCount).toBe(2);
    expect(clusters[0].jointCount).toBe(1);
    expect(clusters[0].roleCounts['transport']).toBe(1);
    expect(clusters[0].roleCounts['feeder']).toBe(1);

    // Cluster 2 (single-cell organism)
    expect(clusters[1].cellCount).toBe(1);
    expect(clusters[1].cellIds).toEqual(['c3']);
  });
});
