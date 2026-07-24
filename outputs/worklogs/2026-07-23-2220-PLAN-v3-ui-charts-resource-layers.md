# PLAN: V3 Monitor Layout Alignment, Resource Layer Selectors & Multi-Chart Data Panel

## Context & Objectives

The user requested:
1. **Fix layout overlap & restore bottom panels:** Resolve top header text overlapping with tab controls, and restore the bottom stats/data panel so it doesn't get pushed off screen.
2. **V3 Multi-Chart Data Panel:** Build the 4-card analytics panel at the bottom of the Monitor layout matching `docs/ui/control-center-monitor-v3.png` (Resource Cycle, Energy Distribution Trend, Cell Role Breakdown, Size Distribution Histogram / N/A charts).
3. **Resource & Field Layer Selectors:** Add individual checkboxes in the left Layer Panel to select which resource fields/layers (`Nutrient A`, `Mineral A`, `Energy A`, `Waste A` or layer indices) are active and blended into the map gradient.

## Proposed Changes

### 1. `ui/control-center/src/app/appState.ts` & `runnerController.ts`
- Add state for `activeResourceLayers: number[]` (defaults to all available layers `[0, 1, 2, 3]`).
- Add action `toggleResourceLayer(layerIndex: number)` in appState.

### 2. `ui/control-center/src/components/LayerPanel.tsx`
- Render `FIELD LAYERS` section with individual resource layer toggles (`[x] Nutrient A / Layer 0`, `[x] Mineral A / Layer 1`, etc.).
- Allow user to toggle each resource field on/off with checkboxes and color indicators.

### 3. `ui/control-center/src/viewer/worldRenderer.ts`
- Update `drawResourceLayer` to filter grid sampling by `activeResourceLayers`.
- Only blend and render resource channels/layers selected by the user.

### 4. `ui/control-center/src/components/BottomDataPanel.tsx` [NEW]
- Build V3 bottom panel with tabs: `TIMELINE`, `EVENTS`, `METRICS`, `WARNINGS`.
- Card 1: **Resource Cycle (Energy & Matter)** - Circular flow diagram showing World %, Cells %, Materials %, Waste %.
- Card 2: **Energy Distribution Over Time** - SVG trend area chart over `frameHistory`.
- Card 3: **Dominant Cell & Behavior Types** - Donut / bar chart of cell roles.
- Card 4: **Cell Size Distribution** - Histogram bar chart or N/A placeholder when data is not streamed.

### 5. `ui/control-center/src/components/MonitorWorkspace.tsx` & `layout.css`
- Re-architect flexbox hierarchy inside `.viewer-panel` to ensure clean separation without text overlap.
- Lock bottom panel height to ~220px with internal scrolling, ensuring canvas and charts fit inside `100vh`.

## Verification Plan

- **TDD Tests:**
  - `LayerPanel.test.tsx`: Verify resource layer toggling checkboxes.
  - `worldRenderer.test.ts`: Verify filtering of resource layers.
  - `BottomDataPanel.test.tsx`: Verify rendering of 4 chart cards, tabs, and N/A placeholders.
  - `MonitorWorkspace.test.tsx`: Verify layout composition without text overlap.
- **Build & Vitest:**
  - Run `npx vitest run` (all 35+ test files pass).
  - Run `npm run build` (production build succeeds with 0 errors).
