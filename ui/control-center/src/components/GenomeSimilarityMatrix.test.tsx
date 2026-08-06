import { describe, expect, it, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { GenomeSimilarityMatrix } from './GenomeSimilarityMatrix';
import type { SimilarityMatrixData } from '../app/evolutionModel';

describe('GenomeSimilarityMatrix', () => {
  const sampleMatrix: SimilarityMatrixData = {
    cellIds: ['c1', 'c2', 'c3'],
    roles: ['Metabolic', 'Metabolic', 'Transport'],
    matrix: [
      [1.0, 0.95, 0.3],
      [0.95, 1.0, 0.35],
      [0.3, 0.35, 1.0]
    ]
  };

  it('renders heatmap grid with cell IDs and similarity cells', () => {
    render(<GenomeSimilarityMatrix data={sampleMatrix} />);

    expect(screen.getByTestId('genome-similarity-matrix')).toBeInTheDocument();
    expect(screen.getAllByTestId(/matrix-cell-/).length).toBe(9);
  });

  it('triggers onSelectCell pair when matrix cell is clicked', () => {
    const onSelect = vi.fn();
    render(<GenomeSimilarityMatrix data={sampleMatrix} onSelectCell={onSelect} />);

    const cell = screen.getByTestId('matrix-cell-0-1');
    fireEvent.click(cell);

    expect(onSelect).toHaveBeenCalledWith('c1', 'c2');
  });
});
