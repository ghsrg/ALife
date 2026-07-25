import type { WorldFrame } from '../projection/types';

export interface OrganismCluster {
  organismId: string;
  rootCellId: string;
  cellCount: number;
  jointCount: number;
  totalEnergy: number;
  cellIds: string[];
  roleCounts: Record<string, number>;
}

export function extractOrganismClusters(frame: WorldFrame): OrganismCluster[] {
  const cells = frame.cells;
  const joints = frame.joints ?? [];

  if (cells.length === 0) return [];

  const cellMap = new Map(cells.map((c) => [c.id, c]));
  const adj = new Map<string, Set<string>>();

  cells.forEach((c) => adj.set(c.id, new Set()));

  joints.forEach((j) => {
    if (adj.has(j.sourceCellId) && adj.has(j.targetCellId)) {
      adj.get(j.sourceCellId)!.add(j.targetCellId);
      adj.get(j.targetCellId)!.add(j.sourceCellId);
    }
  });

  const visited = new Set<string>();
  const clusters: OrganismCluster[] = [];

  cells.forEach((cell) => {
    if (visited.has(cell.id)) return;

    const clusterCellIds: string[] = [];
    const queue = [cell.id];
    visited.add(cell.id);

    let totalEnergy = 0;
    let jointCount = 0;
    const roleCounts: Record<string, number> = {};

    while (queue.length > 0) {
      const currentId = queue.shift()!;
      clusterCellIds.push(currentId);

      const currentCell = cellMap.get(currentId);
      if (currentCell) {
        totalEnergy += currentCell.energy;
        const role = currentCell.roleHint || 'unspecified';
        roleCounts[role] = (roleCounts[role] ?? 0) + 1;
      }

      const neighbors = adj.get(currentId);
      if (neighbors) {
        neighbors.forEach((neighborId) => {
          jointCount++;
          if (!visited.has(neighborId)) {
            visited.add(neighborId);
            queue.push(neighborId);
          }
        });
      }
    }

    // Since adjacency graph is undirected, jointCount is counted twice per edge
    jointCount = Math.floor(jointCount / 2);

    clusters.push({
      organismId: `org-${cell.id}`,
      rootCellId: cell.id,
      cellCount: clusterCellIds.length,
      jointCount,
      totalEnergy,
      cellIds: clusterCellIds,
      roleCounts
    });
  });

  clusters.sort((a, b) => b.cellCount - a.cellCount);

  return clusters;
}
