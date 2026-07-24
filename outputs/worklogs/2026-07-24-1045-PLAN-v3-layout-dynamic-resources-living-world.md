# PLAN: V3 Layout Architecture, Dynamic Resource Layers & Living World Simulation

## Context & User Feedback

1. **Dynamic Resource Field Layers:** Replace static hardcoded resource labels (`Nutrient A`, etc.) with dynamic resource layers parsed directly from `state.debugProjections?.visualWorld?.resourceLayers` or `state.frame.resources`.
2. **V3 Layout Alignment & Overlap Fix:** Fix the top text overlap with workspace tabs, and ensure the 4-card `BottomDataPanel` is visibly docked at the bottom of the map view matching `docs/ui/control-center-monitor-v3.png`.
3. **Living World Scenario & Replication:** Provide a working, living scenario configuration (`living_rich_world.toml` / `bootstrap_rich_patchy_world.toml`) where initial cells have positive energy, active resource absorption, division thresholds, and genome mutations so cells live, divide, and evolve.

## Proposed Changes

### Component 1: `LayerPanel.tsx` & `worldRenderer.ts` (Dynamic Resource Layers)
- Parse active resource layers dynamically from `debugProjections.visualWorld.resourceLayers` or `frame.resources`.
- Render dynamic checkbox list with layer index, channel label, total amount, and color indicator.
- Automatically update map rendering when toggling dynamic checkboxes.

### Component 2: `MonitorWorkspace.tsx` & `components.css` (Layout Structure Fix)
- Separate header bar from tabs:
  - Clean top canvas header bar inside `.viewer-panel` displaying Scenario name, Live Tick, and playback status.
  - Workspace tabs (`Map Viewer`, `Analytics`, `Raw Data`) clean and un-overlapped.
  - Docks `BottomDataPanel` directly below the map canvas within `100vh`.

### Component 3: Living Simulation Balance Config (`scenarios/living_rich_world.toml`)
- Update scenario TOML parameters:
  - Set initial cell energy = 75.0 (capacity = 100.0).
  - Increase resource patch density (`nutrient_A` = 0.8, `mineral_A` = 0.5, `energy_source` = 0.9).
  - Enable active metabolic uptake & repair in `scheduler.cell`.

## Verification Plan

### Automated Tests
- `LayerPanel.test.tsx`: Verify dynamic rendering of resource layers from projection.
- `MonitorWorkspace.test.tsx`: Verify non-overlapping layout and bottom data panel mounting.
- `npx vitest run`: Run full Vitest suite.
- `npm run build`: Verify TypeScript compilation and Vite build.
