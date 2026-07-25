# TDD Plan: AL-007-S15 Experiments And Run Comparison

## Context
Researchers need to compare different simulation runs side-by-side by Tick, simulation time, cell population dynamics, energy efficiency, and warning counts without mutating active simulation state.

## Objectives
1. **Experiment Comparison Model (`ui/control-center/src/app/experimentModel.ts`)**:
   - Model `ExperimentRunSnapshot` (run ID, scenario ID, seed, tick count, duration, cell count, energy efficiency, warnings).
   - Implement `compareExperimentRuns(runA, runB)` to calculate relative deltas (population change, energy efficiency difference, warning count delta).
2. **Experiment & Run Comparison Workspace (`ui/control-center/src/components/ExperimentWorkspace.tsx`)**:
   - Build side-by-side comparison table for selected runs.
   - Display delta badges with color coding (`+15 Cells (+30%)`, `-0.04 Energy Usage`, etc.).
   - Support run selection between current active run and saved run presets/history snapshots.
3. **Automated Verification**:
   - Unit tests in `ui/control-center/src/app/experimentModel.test.ts`.
   - Component tests in `ui/control-center/src/components/ExperimentWorkspace.test.tsx`.
   - `npm run build` and `npx vitest run`.

## Verification Plan
- `npx vitest run` in `ui/control-center`
- `npm run build` in `ui/control-center`
