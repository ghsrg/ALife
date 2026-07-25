# TDD Report: AL-007-S15 Experiments And Run Comparison

## Context
Implemented `AL-007-S15` (UI-3A) Experiments and Run Comparison workspace in the Control Center frontend UI.

## Work Accomplished
1. **Experiment Comparison Model (`ui/control-center/src/app/experimentModel.ts`)**:
   - Defined `ExperimentRunSnapshot` and preset experiment runs.
   - Built `compareExperimentRuns(runA, runB)` computing deltas for cell count, percentage change, joint connections, average energy, and warning count.
2. **Experiment & Comparison Workspace UI (`ui/control-center/src/components/ExperimentWorkspace.tsx`)**:
   - Created side-by-side run comparison interface with dropdown selectors for baseline (Run A) and target (Run B).
   - Rendered metrics table with colored delta badges (+ / -).
3. **AppShell Integration (`ui/control-center/src/components/AppShell.tsx`)**:
   - Added **Experiments & Comparison** top navigation tab to switch seamlessly to the comparison view.
4. **Automated Verification**:
   - Created `experimentModel.test.ts` (100% pass).
   - Created `ExperimentWorkspace.test.tsx` (100% pass).
   - `npm run build` in `ui/control-center`: **PASSED** (`built in 22.14s`).
   - `npx vitest run` in `ui/control-center`: **PASSED** (40/40 test files passed, 189/189 tests passed).
