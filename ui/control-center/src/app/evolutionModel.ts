import type { WorldFrame } from '../projection/types';

export interface GenerationGroup {
  generation: number;
  count: number;
  percentage: number;
  avgEnergy: number;
}

export interface EvolutionSummary {
  totalCells: number;
  maxGeneration: number;
  minGeneration: number;
  avgGeneration: number;
  shannonDiversityIndex: number;
  generationGroups: GenerationGroup[];
}

export interface LineageTreeNode {
  id: string;
  generation: number;
  parentCellId: string | null;
  childrenIds: string[];
  role: string;
  energy: number;
  materials: number[];
}

export interface LineageTreeData {
  nodes: LineageTreeNode[];
  roots: string[];
  maxDepth: number;
  speciationEventsCount: number;
}

export interface SimilarityMatrixData {
  cellIds: string[];
  roles: string[];
  matrix: number[][];
}

export function extractEvolutionSummary(frame: WorldFrame): EvolutionSummary {
  const cells = frame.cells;
  const totalCells = cells.length;

  if (totalCells === 0) {
    return {
      totalCells: 0,
      maxGeneration: 0,
      minGeneration: 0,
      avgGeneration: 0,
      shannonDiversityIndex: 0,
      generationGroups: []
    };
  }

  let maxGeneration = 0;
  let minGeneration = Infinity;
  let sumGeneration = 0;

  const genMap = new Map<number, { count: number; totalEnergy: number }>();

  cells.forEach((cell) => {
    const gen = cell.generation ?? 0;
    if (gen > maxGeneration) maxGeneration = gen;
    if (gen < minGeneration) minGeneration = gen;
    sumGeneration += gen;

    const current = genMap.get(gen) ?? { count: 0, totalEnergy: 0 };
    current.count += 1;
    current.totalEnergy += cell.energy;
    genMap.set(gen, current);
  });

  if (minGeneration === Infinity) minGeneration = 0;
  const avgGeneration = sumGeneration / totalCells;

  // Calculate Shannon Diversity Index over generation distribution
  let shannonDiversityIndex = 0;
  const generationGroups: GenerationGroup[] = [];

  genMap.forEach((val, gen) => {
    const p = val.count / totalCells;
    shannonDiversityIndex -= p * Math.log(p);
    generationGroups.push({
      generation: gen,
      count: val.count,
      percentage: p * 100,
      avgEnergy: val.totalEnergy / val.count
    });
  });

  generationGroups.sort((a, b) => a.generation - b.generation);

  return {
    totalCells,
    maxGeneration,
    minGeneration,
    avgGeneration,
    shannonDiversityIndex,
    generationGroups
  };
}

function to9SlotArray(materials?: Array<{ materialTypeId: number; amount: number }>): number[] {
  const arr = new Array(9).fill(0);
  if (!materials) return arr;
  materials.forEach((m: any, idx: number) => {
    if (typeof m === 'number') {
      if (idx < 9) arr[idx] = m;
    } else if (m && typeof m.amount === 'number') {
      const slot = typeof m.materialTypeId === 'number' ? m.materialTypeId : idx;
      if (slot < 9) arr[slot] = m.amount;
    }
  });
  return arr;
}

export function extractLineageTree(frame: WorldFrame): LineageTreeData {
  const cells = frame.cells;
  if (!cells || cells.length === 0) {
    return { nodes: [], roots: [], maxDepth: 0, speciationEventsCount: 0 };
  }

  let maxDepth = 0;
  const nodesMap = new Map<string, LineageTreeNode>();

  // Initialize nodes
  cells.forEach((cell) => {
    const gen = cell.generation ?? 0;
    if (gen > maxDepth) maxDepth = gen;

    nodesMap.set(cell.id, {
      id: cell.id,
      generation: gen,
      parentCellId: null, // derived by generation grouping or explicit tracking
      childrenIds: [],
      role: cell.roleHint ?? 'Structural',
      energy: cell.energy,
      materials: to9SlotArray(cell.materials)
    });
  });

  // Assign parent-child relationships using generation hierarchy
  const genGroups = new Map<number, LineageTreeNode[]>();
  nodesMap.forEach((node) => {
    const list = genGroups.get(node.generation) ?? [];
    list.push(node);
    genGroups.set(node.generation, list);
  });

  // Link children to closest parent in previous generation
  genGroups.forEach((children, gen) => {
    if (gen > 0) {
      const parents = genGroups.get(gen - 1) ?? [];
      if (parents.length > 0) {
        children.forEach((child, idx) => {
          const parent = parents[idx % parents.length];
          child.parentCellId = parent.id;
          parent.childrenIds.push(child.id);
        });
      }
    }
  });

  const roots: string[] = [];
  let speciationEventsCount = 0;

  nodesMap.forEach((node) => {
    if (!node.parentCellId || node.generation === 0) {
      roots.push(node.id);
    } else {
      const parent = nodesMap.get(node.parentCellId);
      if (parent && parent.role !== node.role) {
        speciationEventsCount += 1;
      }
    }
  });

  return {
    nodes: Array.from(nodesMap.values()),
    roots,
    maxDepth,
    speciationEventsCount
  };
}

export function computeGenomeSimilarityMatrix(frame: WorldFrame): SimilarityMatrixData {
  const cells = frame.cells;
  if (!cells || cells.length === 0) {
    return { cellIds: [], roles: [], matrix: [] };
  }

  const cellIds = cells.map((c) => c.id);
  const roles = cells.map((c) => c.roleHint ?? 'Structural');
  const n = cells.length;
  const matrix: number[][] = Array.from({ length: n }, () => Array(n).fill(1.0));

  const cellMaterials = cells.map((c) => to9SlotArray(c.materials));

  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
      if (i === j) {
        matrix[i][j] = 1.0;
        continue;
      }
      const matA = cellMaterials[i];
      const matB = cellMaterials[j];
      matrix[i][j] = computeMaterialCosineSimilarity(matA, matB, roles[i], roles[j]);
    }
  }

  return { cellIds, roles, matrix };
}

function computeMaterialCosineSimilarity(a: number[], b: number[], roleA: string, roleB: string): number {
  if (a.length === 0 || b.length === 0) {
    return roleA === roleB ? 0.95 : 0.45;
  }
  let dot = 0;
  let normA = 0;
  let normB = 0;
  const len = Math.max(a.length, b.length);
  for (let k = 0; k < len; k++) {
    const valA = a[k] ?? 0;
    const valB = b[k] ?? 0;
    dot += valA * valB;
    normA += valA * valA;
    normB += valB * valB;
  }
  if (normA === 0 || normB === 0) {
    return roleA === roleB ? 0.95 : 0.45;
  }
  const cosSim = dot / (Math.sqrt(normA) * Math.sqrt(normB));
  if (isNaN(cosSim) || !isFinite(cosSim)) {
    return roleA === roleB ? 0.95 : 0.45;
  }
  return Math.min(1.0, Math.max(0.0, cosSim));
}
