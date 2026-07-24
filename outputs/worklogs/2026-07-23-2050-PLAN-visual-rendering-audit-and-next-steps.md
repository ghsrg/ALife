---
tags:
  - alife
  - worklog/plan
  - ui
  - audit
  - rendering
---

# Visual Rendering Audit And Next Steps Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Bridge the gap between green unit tests and the visual target in `docs/ui/control-center-monitor-v3.png` by auditing current implementation vs documentation, and defining execution slices for multi-channel resource heatmaps, detailed cell organelles, and joint connection rendering.

---

## Audit Findings: Current State vs Canon & Reference Design

### 1. Resource Layer Rendering Gap
- **Current Behavior:** `worldRenderer.ts` (`drawResourceLayer`) calculates coarse grid tiles (`cols x rows`), computes `dominantResourceColor` (choosing one solid color: green, blue, or amber), and draws flat filled rectangles (`layer.rect`).
- **Reference Target (`control-center-monitor-v3.png`):** Smooth, continuous multi-spectral atmospheric background with organic glow, gradient falloffs, and multi-channel blending (organic green, mineral azure, energy amber overlaid simultaneously).
- **Root Cause:** Current rendering uses coarse tile rasterization without bilinear/radial smoothing or multi-channel blend modes.

### 2. Cell Visual Quality Gap
- **Current Behavior:** `worldRenderer.ts` (`renderFrame`) draws cells as flat 2D circles (`cellGraphic.circle`) with simple 1-2 stroke outline rings.
- **Reference Target (`control-center-monitor-v3.png`):** Rich biological entities with visible internal nucleus/organelles, energy core, structural membrane layers, cytoplasm gradients, and stress/dormancy visual states.
- **Root Cause:** `semanticDetail.ts` and `worldRenderPlan.ts` provide numeric ratios (`energyRatio`, `integrityRatio`), but `worldRenderer.ts` only maps them to simple circle radius and stroke opacity.

### 3. Missing Joint & Connection Visuals
- **Current Behavior:** Joints between cells (`AL-002-S08` in Core) are not rendered on the canvas at all.
- **Reference Target (`control-center-monitor-v3.png`):** Visible structural tendrils/lines connecting bonded cells with signal pulse indicators and mechanical tension styling.
- **Root Cause:** `WorldFrame` projection and `worldRenderer.ts` do not parse or draw joint channel vectors.

### 4. Status of Delivery Roadmap vs Reality
- **Roadmap Claim:** Slices `AL-007-S01` through `AL-007-S11` and `AL-007-S21` are marked `done`.
- **Reality:** Data contracts, state machine, HTTP/WS transport, inspectors, search, filters, and state history are fully implemented and passing TDD tests. However, the PixiJS renderer was kept at a minimal placeholder level ("Start-level atmospheric clarity").

---

## Proposed Next Steps & Execution Plan

### Slice 1: `AL-007-S22` — Atmospheric Multi-Channel Resource Heatmap & Interpolation
- **Goal:** Replace blocky flat rectangles with smooth multi-spectral gradient rendering for resource layers.
- **Tasks:**
  - [ ] Implement smooth bilinear/radial falloff interpolation for resource grids in `drawResourceLayer`.
  - [ ] Support simultaneous multi-channel rendering (organic green, mineral blue, energy amber) with additive/alpha blending instead of single dominant color selection.
  - [ ] Add atmospheric background glow for high-density resource hotspots.

### Slice 2: `AL-007-S23` — High-Detail Cell Organelles & Visual Depth
- **Goal:** Upgrade cell rendering to match the organic biological visual style of `control-center-monitor-v3.png`.
- **Tasks:**
  - [ ] Render internal energy core/nucleus inside cell graphics based on `energyRatio`.
  - [ ] Add double-layered cell membrane graphics with integrity-based degradation/gaps.
  - [ ] Add visual pulse/glow effects for active/stressed/dormant lifecycle states.

### Slice 3: `AL-007-S24` — Joint Connections & Signal Channel Canvas Overlay
- **Goal:** Draw physical joints and signal channels between connected cells.
- **Tasks:**
  - [ ] Extend `WorldFrame` / `debugProjectionAdapter.ts` to include joint endpoints and channel types.
  - [ ] Draw joint lines/tendrils between cells with stress-color coding (mechanical, resource, signal).
  - [ ] Add subtle animated pulse indicators on active signal joints.

### Slice 4: `AL-007-S12` — Balance Analytics, Warnings, And Raw Data (Next Planned Roadmap Slice)
- **Goal:** Implement Debug analytics overview, Matter Cycle & Energy Flow accounting, and engineering control grid as planned in `docs/delivery/roadmap.md`.

---

## Verification Plan

### Automated Tests
- `cd ui/control-center && npm test` (Vitest unit tests for renderer plan and joint projections).
- `cd ui/control-center && npm run build` (TypeScript compilation & Vite production build).
- `cd ui/control-center && npm run e2e` (Playwright E2E visual & interaction tests).

### Visual Verification
- Run `npm run dev` and test against `config/scenarios/bootstrap/rich_patchy_world.toml` and `demo_world_resource.toml`.
- Compare live canvas output directly against [docs/ui/control-center-monitor-v3.png](file:///c:/Users/korsr/PycharmProjects/ALife/docs/ui/control-center-monitor-v3.png).
