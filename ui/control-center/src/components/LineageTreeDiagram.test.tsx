import { describe, expect, it, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { LineageTreeDiagram } from './LineageTreeDiagram';
import type { LineageTreeData } from '../app/evolutionModel';

describe('LineageTreeDiagram', () => {
  const sampleTree: LineageTreeData = {
    nodes: [
      { id: 'c1', generation: 0, parentCellId: null, childrenIds: ['c2', 'c3'], role: 'Metabolic', energy: 0.8, materials: [0,0,15,0,0,0,0,0,0] },
      { id: 'c2', generation: 1, parentCellId: 'c1', childrenIds: ['c4'], role: 'Metabolic', energy: 0.6, materials: [0,0,14,0,0,0,0,0,0] },
      { id: 'c3', generation: 1, parentCellId: 'c1', childrenIds: [], role: 'Transport', energy: 0.4, materials: [0,12,0,0,0,0,0,0,0] },
      { id: 'c4', generation: 2, parentCellId: 'c2', childrenIds: [], role: 'Transport', energy: 0.9, materials: [0,14,0,0,0,0,0,0,0] }
    ],
    roots: ['c1'],
    maxDepth: 2,
    speciationEventsCount: 1
  };

  it('renders SVG diagram with generation nodes and connecting paths', () => {
    render(<LineageTreeDiagram tree={sampleTree} />);

    expect(screen.getByTestId('lineage-tree-diagram')).toBeInTheDocument();
    expect(screen.getAllByTestId(/lineage-node-/).length).toBe(4);
    expect(screen.getAllByText(/Gen 0/).length).toBeGreaterThan(0);
    expect(screen.getAllByText(/Gen 2/).length).toBeGreaterThan(0);
  });

  it('triggers onSelectNode callback when a lineage node is clicked', () => {
    const onSelect = vi.fn();
    render(<LineageTreeDiagram tree={sampleTree} onSelectNode={onSelect} />);

    const nodeC1 = screen.getByTestId('lineage-node-c1');
    fireEvent.click(nodeC1);

    expect(onSelect).toHaveBeenCalledWith('c1');
  });
});
