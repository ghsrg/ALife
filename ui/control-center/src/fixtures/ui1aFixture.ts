import type { UiFixture } from '../projection/types';

export const ui1aFixture: UiFixture = {
  version: 'ui-1a-fixture/v1',
  scenarioName: 'UI-1A Deterministic Fixture',
  frame: {
    schemaVersion: 'WorldFrameProjection/v1',
    runId: 'fixture-ui-1a',
    tick: 128,
    world: {
      width: 1200,
      height: 800
    },
    resources: [
      [
        { organic: 0.78, mineral: 0.22, energy: 0.62 },
        { organic: 0.65, mineral: 0.28, energy: 0.58 },
        { organic: 0.42, mineral: 0.49, energy: 0.44 }
      ],
      [
        { organic: 0.48, mineral: 0.31, energy: 0.74 },
        { organic: 0.72, mineral: 0.18, energy: 0.82 },
        { organic: 0.55, mineral: 0.40, energy: 0.36 }
      ],
      [
        { organic: 0.31, mineral: 0.62, energy: 0.29 },
        { organic: 0.38, mineral: 0.52, energy: 0.48 },
        { organic: 0.44, mineral: 0.46, energy: 0.53 }
      ]
    ],
    cells: [
      {
        id: 'cell-a',
        x: 330,
        y: 320,
        radius: 24,
        energy: 0.82,
        integrity: 0.91,
        generation: 3,
        roleHint: 'high-energy cluster member'
      },
      {
        id: 'cell-b',
        x: 520,
        y: 380,
        radius: 19,
        energy: 0.64,
        integrity: 0.78,
        generation: 2,
        roleHint: 'boundary contact'
      },
      {
        id: 'cell-c',
        x: 720,
        y: 470,
        radius: 28,
        energy: 0.71,
        integrity: 0.86,
        generation: 4,
        roleHint: 'resource-rich region'
      }
    ]
  }
};
