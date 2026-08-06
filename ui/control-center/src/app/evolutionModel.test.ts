import { describe, expect, it } from 'vitest';
import { extractEvolutionSummary, extractLineageTree, computeGenomeSimilarityMatrix } from './evolutionModel';
import type { WorldFrame } from '../projection/types';

describe('evolutionModel', () => {
  const sampleFrame: WorldFrame = {
    schemaVersion: 'WorldFrameProjection/v1',
    runId: 'run-1',
    tick: 50,
    world: { width: 100, height: 100 },
    resources: [],
    cells: [
      {
        id: 'c1',
        x: 10,
        y: 10,
        radius: 2,
        energy: 0.8,
        integrity: 1,
        generation: 0,
        roleHint: 'Metabolic',
        materials: [{ materialTypeId: 2, amount: 15 }, { materialTypeId: 5, amount: 2 }]
      },
      {
        id: 'c2',
        x: 20,
        y: 20,
        radius: 2,
        energy: 0.6,
        integrity: 1,
        generation: 1,
        roleHint: 'Metabolic',
        materials: [{ materialTypeId: 2, amount: 14 }, { materialTypeId: 5, amount: 2 }]
      },
      {
        id: 'c3',
        x: 30,
        y: 30,
        radius: 2,
        energy: 0.4,
        integrity: 1,
        generation: 1,
        roleHint: 'Transport',
        materials: [{ materialTypeId: 1, amount: 12 }, { materialTypeId: 5, amount: 2 }]
      },
      {
        id: 'c4',
        x: 40,
        y: 40,
        radius: 2,
        energy: 0.9,
        integrity: 1,
        generation: 2,
        roleHint: 'Transport',
        materials: [{ materialTypeId: 1, amount: 14 }, { materialTypeId: 5, amount: 2 }]
      }
    ]
  };

  it('extracts evolution summary metrics and Shannon diversity from cell generations', () => {
    const summary = extractEvolutionSummary(sampleFrame);

    expect(summary.totalCells).toBe(4);
    expect(summary.minGeneration).toBe(0);
    expect(summary.maxGeneration).toBe(2);
    expect(summary.avgGeneration).toBe(1);
    expect(summary.shannonDiversityIndex).toBeGreaterThan(0);
    expect(summary.generationGroups.length).toBe(3);
    expect(summary.generationGroups[1].count).toBe(2); // Gen 1 has 2 cells
  });

  it('extracts lineage tree hierarchy and generation depth', () => {
    const tree = extractLineageTree(sampleFrame);

    expect(tree.nodes.length).toBe(4);
    expect(tree.maxDepth).toBe(2);
    expect(tree.roots.length).toBe(1); // c1 is Gen 0 root
    expect(tree.nodes.find((n) => n.id === 'c1')?.role).toBe('Metabolic');
  });

  it('computes pairwise genome/material similarity matrix', () => {
    const matrixData = computeGenomeSimilarityMatrix(sampleFrame);

    expect(matrixData.cellIds).toEqual(['c1', 'c2', 'c3', 'c4']);
    expect(matrixData.matrix.length).toBe(4);
    expect(matrixData.matrix[0][0]).toBeCloseTo(1.0); // Self-similarity is 1.0
    expect(matrixData.matrix[0][1]).toBeGreaterThan(0.9); // c1 & c2 both Metabolic (high similarity)
    expect(matrixData.matrix[0][2]).toBeLessThan(0.8); // c1 (Metabolic) vs c3 (Transport) lower similarity
  });
});
