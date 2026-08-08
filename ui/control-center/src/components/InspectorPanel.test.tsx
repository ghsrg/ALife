import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { createWorldBlockSelection } from '../app/selectionModel';
import { createAppStore } from '../app/appState';
import { InspectorPanel } from './InspectorPanel';

describe('InspectorPanel', () => {
  it('renders selected World cell resources and configured scalar fields', () => {
    const store = createAppStore();
    const frame = {
      ...store.getState().frame,
      runId: 'run-world-cell',
      tick: 42,
      resources: [
        [{ organic: 0, mineral: 0, energy: 0 }, { organic: 0, mineral: 0, energy: 0 }]
      ]
    };
    const selection = createWorldBlockSelection({
      runId: frame.runId,
      tick: frame.tick,
      blockX: 1,
      blockY: 0,
      bounds: { x: 10, y: 0, width: 10, height: 10 },
      completeness: 'bounded'
    });

    render(
      <InspectorPanel
        frame={frame}
        debugProjections={{
          status: 'available',
          runId: frame.runId,
          tick: frame.tick,
          visualWorld: {
            projectionKind: 'VisualWorldProjection',
            completeness: { state: 'bounded', missingFields: [], reason: null },
            cells: [],
            resourceLayers: [
              {
                layerIndex: 0,
                resourceTypeId: 0,
                resourceId: 'amino_acid',
                width: 2,
                height: 1,
                totalAmount: 7,
                cells: [{ x: 1, y: 0, amount: 7 }],
                completeness: { state: 'bounded', missingFields: [], reason: null }
              }
            ],
            fieldLayers: [
              {
                fieldId: 'temperature',
                width: 2,
                height: 1,
                summaryValue: 21,
                cells: [{ x: 1, y: 0, value: 21 }],
                completeness: { state: 'bounded', missingFields: [], reason: null }
              }
            ],
            fields: [],
            sourceMetrics: []
          },
          coverage: { projectionKind: 'CoverageProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, mechanisms: [] },
          warnings: { projectionKind: 'WarningProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, warnings: [] },
          classifications: { projectionKind: 'ClassificationProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, classifications: [] },
          balanceFindings: { projectionKind: 'BalanceFindingProjection', completeness: { state: 'bounded', missingFields: [], reason: null }, findings: [] }
        }}
        currentSelection={selection}
        selectedCell={null}
        displayedTick={frame.tick}
      />
    );

    expect(screen.getByLabelText('Inspector')).toHaveTextContent('World cell 1, 0');
    expect(screen.getByLabelText('Inspector')).toHaveTextContent('amino_acid');
    expect(screen.getByLabelText('Inspector')).toHaveTextContent('Temperature');
    expect(screen.getByLabelText('Inspector')).toHaveTextContent('21');
  });
});
