# TDD Plan: AL-007-S16 Evolution Observatory

## Context
Researchers need observer-only visibility into cell generations, genome hash diversity, mutation counts, and spatial lineage clusters without introducing biological shortcuts or selection authority in the UI.

## Objectives
1. **Evolution & Lineage Data Model (`ui/control-center/src/app/evolutionModel.ts`)**:
   - Model `LineageSummary` (generation distribution, active lineage count, estimated Shannon diversity index, mutation load).
   - Implement `extractEvolutionSummary(frame)` deriving generation metrics and lineage clusters directly from `WorldFrame.cells`.
2. **Evolution Observatory Workspace (`ui/control-center/src/components/EvolutionWorkspace.tsx`)**:
   - Display key metrics cards: Max Generation, Active Lineages, Diversity Index, Avg Mutations.
   - Render Lineage Generation Breakdown table and spatial lineage distribution overview.
   - Explicit read-only provenance notice: "Observer-only projections. No Genome Runtime or selection authority."
3. **Automated Verification**:
   - Unit tests in `ui/control-center/src/app/evolutionModel.test.ts`.
   - Component tests in `ui/control-center/src/components/EvolutionWorkspace.test.tsx`.
   - `npm run build` and `npx vitest run`.

## Verification Plan
- `npx vitest run` in `ui/control-center`
- `npm run build` in `ui/control-center`
