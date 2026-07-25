# TDD Plan: AL-007-S19 Library, Placement, Reporting, And Research Export

## Context
Researchers need a library of saved Cell and Organism templates, a placement command builder that validates explicit coordinates against world boundaries without mutating simulation state directly, and a research export generator providing reproducible Markdown and JSON reports with full metadata.

## Objectives
1. **Library, Placement, & Research Reporting Model (`ui/control-center/src/app/libraryModel.ts`)**:
   - Model `SavedTemplate` (id, name, type, cellCount, roleComposition).
   - Implement `validatePlacementCommand(command, worldSize)` enforcing bounds check (`0 <= x <= width`, `0 <= y <= height`).
   - Implement `generateResearchReport(state)` building reproducible Markdown report with run ID, scenario ID, seed, tick count, population, energy utilization, warnings, and specialization index.
2. **Library & Research Export Workspace (`ui/control-center/src/components/LibraryWorkspace.tsx`)**:
   - Add **Library & Research Export** tab to `AppShell.tsx`.
   - Preset templates list & Placement command form with real-time coordinate validation against active world dimensions.
   - Reproducible Research Report export panel (view formatted Markdown report and copy/export functionality).
   - Explicit provenance notice: "Placement commands require Core execution authority; research reports include full reproducibility metadata."
3. **Automated Verification**:
   - Unit tests in `ui/control-center/src/app/libraryModel.test.ts`.
   - Component tests in `ui/control-center/src/components/LibraryWorkspace.test.tsx`.
   - `npm run build` and `npx vitest run`.

## Verification Plan
- `npx vitest run` in `ui/control-center`
- `npm run build` in `ui/control-center`
