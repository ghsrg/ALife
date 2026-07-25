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
