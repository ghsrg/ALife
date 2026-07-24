# PLAN: V3 UI Control Center Alignment, Charts Docking & Ecosystem Viability Balance (`AL-007-S13`)

## Context & Objectives

Based on human feedback and screenshots:
1. **Top Bar Header Overlap Fix:** `.map-context-strip` text (`living_ecosystem Live Tick 5858 demo Projection source: live...`) is overflowing directly on top of the main top navigation tabs (`Monitor`, `OrganismView`, `World Editor`). We will clean up header layout CSS and remove text overlap.
2. **V3 Charts Docking under Map View:** `BottomDataPanel` (the 4 charts: Resource Cycle, Energy Distribution Trend, Dominant Behaviors, Cell Size Histogram) is currently placed inside the `Analytics` tab. Matching `docs/ui/control-center-monitor-v3.png`, `BottomDataPanel` must be docked directly below the Map Viewer canvas in `Map Viewer` tab.
3. **Live Stream Telemetry & Charts Data Binding:** Replace fallback `N/A STREAM` badges on `BottomDataPanel` chart cards with dynamic real-time telemetry parsed from `state.frame.cells`, `state.frame.resources`, and `state.debugProjections`.
4. **Ecosystem Population Viability Balance:** Fix `living_ecosystem.toml` balance parameters so cell population grows smoothly, self-sustains above 100+ cells, and doesn't die off at tick 5800:
   - `mandatory_cost_per_tick = 0.0005` (low background maintenance cost).
   - `max_uptake_per_tick = 3.5`, `energy_per_resource = 3.5` (high metabolic conversion efficiency).
   - `division.energy_cost = 12.0` (accessible replication threshold).
   - `passive_energy_income_placeholder = 0.05` (ambient solar energy inflow).

## BDD Agent Scenario Cards

### Scenario Card 1: `AL-007-S13-AC01` (Top Bar Layout Isolation)
- **Given:** The Control Center UI is connected to a live or offline simulation run.
- **When:** The user views the top header bar and workspace tabs.
- **Then:** `.map-context-strip` metadata displays cleanly in a dedicated single-line row without overlapping `.workspace-tab-nav` or header buttons.

### Scenario Card 2: `AL-007-S13-AC02` (V3 Bottom Charts Docking)
- **Given:** The user opens the `Map Viewer` tab.
- **When:** The map workspace is rendered inside the 100vh viewport.
- **Then:** The 4 V3 analytics chart cards (`BottomDataPanel`) are docked below the map canvas within `Map Viewer` view, strictly matching `docs/ui/control-center-monitor-v3.png`.

### Scenario Card 3: `AL-007-S13-AC03` (Live Telemetry Chart Streams)
- **Given:** A live frame or projection is received by the application store.
- **When:** `BottomDataPanel` renders the 4 chart cards.
- **Then:** Resource Cycle Flow, Energy Distribution Trend (SVG curve), Dominant Behaviors, and Cell Size Histogram reflect real-time population metrics instead of `N/A STREAM`.

### Scenario Card 4: `AL-007-S13-AC04` (Sustained Ecosystem Growth)
- **Given:** Scenario `living_ecosystem.toml` is executed for >50,000 ticks.
- **When:** Cells absorb resources from patchy oases.
- **Then:** Cell population grows beyond 50+ cells, self-sustains, and forms active evolving colonies without mass extinction.

## Proposed Changes

### UI Control Center

#### [MODIFY] [MonitorWorkspace.tsx](file:///c:/Users/korsr/PycharmProjects/ALife/ui/control-center/src/components/MonitorWorkspace.tsx)
- Re-dock `BottomDataPanel` into `activeTab === 'viewer'` workspace underneath `WorldViewer`.
- Clean up `.map-context-strip` header elements to eliminate top bar text overlap.

#### [MODIFY] [BottomDataPanel.tsx](file:///c:/Users/korsr/PycharmProjects/ALife/ui/control-center/src/components/BottomDataPanel.tsx)
- Bind live chart metrics to `state.frame.cells` (energy distribution array, cell radius histogram, behavior classification counts) and `state.frame.resources`.

#### [MODIFY] [components.css](file:///c:/Users/korsr/PycharmProjects/ALife/ui/control-center/src/styles/components.css)
- Add CSS layout rules for docked V3 bottom charts panel and header text truncation.

### Simulation Scenarios

#### [MODIFY] [living_ecosystem.toml](file:///c:/Users/korsr/PycharmProjects/ALife/config/scenarios/demo/living_ecosystem.toml)
- Optimize metabolic parameters for long-term population growth and self-sustained division.

## Verification Plan

### Automated Tests
- `BottomDataPanel.test.tsx`: Test live telemetry data binding for SVG energy area curve, cell size histogram, and behavior counts.
- `MonitorWorkspace.test.tsx`: Test V3 layout composition and non-overlapping header bar.
- `npx vitest run`: Run full Vitest suite (36 test files).
- `npm run build`: Verify TypeScript compilation and Vite build.
