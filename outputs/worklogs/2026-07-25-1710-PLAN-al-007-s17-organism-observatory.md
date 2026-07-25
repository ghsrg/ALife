# TDD Plan: AL-007-S17 Organism Observatory

## Context
Researchers need observer-only visibility into multi-cell organism graph structures, cell-joint connectivity clusters, and role compositions without granting UI behavior or physics authority.

## Objectives
1. **Organism Graph & Cluster Model (`ui/control-center/src/app/organismModel.ts`)**:
   - Model `OrganismCluster` (organism ID, cell count, joint count, total energy, root cell ID, role breakdown).
   - Implement `extractOrganismClusters(frame)` grouping cells connected by joints via breadth-first graph search.
2. **Organism Observatory Workspace (`ui/control-center/src/components/OrganismWorkspace.tsx`)**:
   - Enable the **Organism View** tab in `AppShell.tsx`.
   - Display Organism Clusters list and detail panel with role distribution badges (e.g., `2 Transport`, `1 Feeder`).
   - Display explicit read-only provenance notice: "Observer-only projections. No physics or behavior authority."
3. **Automated Verification**:
   - Unit tests in `ui/control-center/src/app/organismModel.test.ts`.
   - Component tests in `ui/control-center/src/components/OrganismWorkspace.test.tsx`.
   - `npm run build` and `npx vitest run`.

## Verification Plan
- `npx vitest run` in `ui/control-center`
- `npm run build` in `ui/control-center`
