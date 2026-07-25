# TDD Plan: AL-007-S18 Specialization Analytics

## Context
Researchers need observer-only functional role analytics exposing classifier confidence, version, provenance, and role specialization ratios across the cell population without hardcoded biological shortcuts or behavior authority.

## Objectives
1. **Specialization Analytics Data Model (`ui/control-center/src/app/specializationModel.ts`)**:
   - Model `RoleSpecializationGroup` (role name, count, share %, average energy, confidence score `0.0 - 1.0`, classifier version/provenance).
   - Implement `extractSpecializationSummary(frame)` deriving role breakdown, dominant specialized role, and Herfindahl-Hirschman specialization index.
2. **Specialization Analytics Workspace (`ui/control-center/src/components/SpecializationWorkspace.tsx`)**:
   - Add **Specialization Analytics** navigation tab to `AppShell.tsx`.
   - Display summary cards: Total Cells, Dominant Role, Classifier Confidence, Specialization Index.
   - Display Functional Role Classifiers table with role hints, cell counts, energy distribution, and confidence badges.
   - Explicit read-only provenance notice: "Functional role classification generated from observer heuristics without behavior authority."
3. **Automated Verification**:
   - Unit tests in `ui/control-center/src/app/specializationModel.test.ts`.
   - Component tests in `ui/control-center/src/components/SpecializationWorkspace.test.tsx`.
   - `npm run build` and `npx vitest run`.

## Verification Plan
- `npx vitest run` in `ui/control-center`
- `npm run build` in `ui/control-center`
