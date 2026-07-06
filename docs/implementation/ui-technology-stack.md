---
tags:
  - alife
  - implementation
  - ui
  - technology-stack
status: draft
---

# UI Technology Stack

> Draft implementation stack for `ALife Control Center`.
> This document does not override UI Canon, accepted ADR, or `docs/engine/technology-stack.md`.

## Purpose

Define the practical web stack for implementing the application shell, shared Viewer, inspectors, charts, tables, transport adapters and future desktop packaging.

The stack must support:

- local Chromium-based application;
- scientific-instrument UX with game-like presentation;
- 2D World with pseudo-3D visual quality;
- `20k` Cells and `20k-40k` Joints target scenarios;
- binary WebSocket projections;
- Light/Dark themes, compact density and UI scaling;
- read-only visualization of authoritative Core state;
- future packaging as a desktop application without rewriting the frontend.

## Authority And Status

Authority order:

```text
UI Canon
  -> accepted ADR
  -> Engine Technology Stack
  -> this draft
  -> detailed UI worklogs
```

Exact package versions are pinned in the lockfile when `UI-1A` starts. Major-version changes require compatibility and performance verification.

## Stack Summary

```text
Application shell:
  React 19
  TypeScript
  Vite

World Viewer:
  PixiJS 8
  WebGL2 baseline
  custom batched geometry and shaders where required
  RenderTexture for Fields, heatmaps and composed layers

UI primitives and styling:
  Radix UI Primitives
  semantic CSS variables
  CSS Modules
  CSS Grid / Flexbox

Application state:
  Zustand

Charts and large data views:
  Apache ECharts 6
  TanStack Table
  TanStack Virtual

Transport and frame processing:
  binary WebSocket
  ArrayBuffer / TypedArray projections
  Web Worker for frame decoding and heavy UI-side transforms

Testing:
  Vitest
  React Testing Library
  Playwright

Desktop packaging, later:
  Tauri 2
```

## Application Shell

Use `React + TypeScript` for:

- global shell and navigation;
- workspaces;
- panels and inspectors;
- dialogs, menus, tooltips and controls;
- settings and localization;
- tables and analytical views;
- command request state;
- loading, error, empty and partial states.

Use `Vite` for development, production builds and deterministic fixture-based UI work.

Do not use React component trees to represent individual Cells, Joints, Resource particles or Field samples.

## World Viewer

Use one shared `PixiJS` Viewer inside the React application.

```text
React application
  -> WorldViewerHost
      -> one canvas
          -> PixiJS renderer
```

The Viewer must be reusable for:

- live World;
- recorded World;
- World Editor preview;
- Evolution spatial view;
- Analysis spatial results;
- placement preview;
- debug visualization.

### Renderer Layers

Recommended base order:

```text
1. background
2. Field and Resource textures
3. Joints
4. Cells
5. signals, traces and movement trails
6. selection and debug overlays
7. labels at sufficient zoom
8. bounded composite/post-processing pass
```

### Rendering Rules

- WebGL2 is the production baseline.
- Use batched geometry, instancing or equivalent GPU batching for Cells and Joints.
- Use sprites/particles for lightweight decorative or sparse elements.
- Use custom shaders only behind explicit renderer interfaces.
- Use zoom-dependent detail and LOD.
- Keep visual interpolation separate from committed simulation values.
- Decorative animation may be disabled without losing scientific information.
- Expensive filters, masks and glow passes must be bounded and measured.
- Canvas 2D may be used for limited overlays or export helpers, not as the primary World renderer.

## Visual Quality Strategy

Pseudo-3D presentation should be produced through:

- layered sprites or geometry;
- gradient and emissive shading;
- material-dependent outlines;
- bounded glow and bloom-like composition;
- animated internal particles;
- Resource and Field heatmaps;
- trails and local flow visualization;
- high-detail selected-entity scene;
- zoom-dependent labels and internal detail.

The selected Cell or OrganismView detail may use a separate high-detail PixiJS scene, but it must consume the same projection contracts as the main Viewer.

True 3D is not a baseline dependency.

## UI Primitives And Design System

Use `Radix UI Primitives` for accessible behavior such as:

- Dialog;
- Dropdown Menu;
- Tooltip;
- Popover;
- Select;
- Slider;
- Tabs;
- Scroll Area;
- Toggle and Switch;
- focus and keyboard management.

Radix supplies behavior, not the visual identity.

The ALife design system uses:

```text
semantic CSS variables
+ CSS Modules
+ reusable ALife components
```

Required token groups:

- color roles;
- typography;
- spacing;
- panel elevation;
- border and focus states;
- status severity;
- chart colors;
- World layer colors;
- compact/comfortable density;
- UI scale;
- animation duration and reduced-motion behavior.

Do not adopt a fully styled administration UI kit as the visual foundation.

## State Separation

Use `Zustand` for bounded application state:

- navigation;
- selection;
- filters;
- presentation preferences;
- connection status;
- pending commands;
- temporary historical context;
- inspector summaries.

Do not store the complete World Frame as normal React state.

```text
WebSocket ArrayBuffer
  -> Web Worker decode
  -> TypedArray projection buffers
  -> PixiJS renderer
  -> sampled summaries to Zustand / React
```

Required separation:

```text
Simulation Projection State
Navigation State
Selection State
Presentation State
Temporary Historical State
Pending Command State
```

UI preference, simulation config and authoritative WorldState must remain separate.

## Charts And Tables

Use `Apache ECharts` for:

- time series;
- stacked distributions;
- histograms;
- scatter plots;
- heatmaps;
- linked selections;
- data zoom;
- custom analytical series;
- exportable chart images and data.

Use `TanStack Table` for headless table behavior and `TanStack Virtual` for long lists, event streams, entity catalogs and large result sets.

Charts and tables consume bounded projections or analytical results. They do not read or mutate Core state directly.

## Projection And Command Boundaries

Canonical data flow:

```text
Core / Runner
  -> committed snapshots
  -> versioned binary projections
  -> WebSocket / recorded fixture adapter
  -> UI projection model
  -> Viewer / Inspector / Charts
```

Canonical command flow:

```text
UI request
  -> Command Gateway
  -> Core validation
  -> Core apply and record
  -> committed projection
  -> UI observes result
```

Rules:

- Recorded fixtures and live transport implement the same adapter contract.
- Projection versions are checked before decoding.
- Stale or incompatible frames are rejected explicitly.
- UI-side validation never replaces Core validation.
- Commands are not repeated without Core acknowledgement and explicit retry policy.

## Web Workers

Use Web Workers for work that should not block rendering or interaction:

- binary frame decoding;
- buffer preparation;
- large filtering or aggregation;
- export preparation;
- optional chart preprocessing.

Worker output must remain deterministic for the same input projection where deterministic behavior is expected.

Simulation logic does not run in frontend workers.

## Testing

### Unit And Component

Use `Vitest` and `React Testing Library` for:

- state stores;
- projection adapters;
- protocol validation;
- component behavior;
- formatting and localization;
- command request lifecycle;
- accessibility-critical interactions.

### End-To-End

Use `Playwright` for:

- Chromium smoke tests;
- primary demo flow;
- run controls;
- selection and Inspector;
- Light/Dark themes;
- 1024x768 baseline;
- full-screen and screenshot flow;
- recorded fixture and live adapter compatibility;
- critical warning visibility;
- stale-data and transport-error states.

### Renderer Tests

Include:

- deterministic fixture screenshots with controlled tolerances;
- frame/projection compatibility tests;
- entity selection hit-testing;
- LOD transition tests;
- performance smoke scenarios;
- context-loss and resize recovery where supported.

## Performance Rules

- React renders the Control Center; PixiJS renders the World.
- Simulation frames do not trigger full application rerenders.
- Inspector and metrics updates use selectors and bounded sampling.
- Frame decoding is measured separately from rendering.
- Simulation Tick rate and visualization FPS remain separate.
- Viewer may adapt FPS or visual detail without changing Core behavior.
- No per-entity DOM elements for World objects.
- No unbounded UI history, chart series or event lists.

Initial Viewer acceptance targets follow the engine technology stack:

```text
20k Cells target
20k-40k Joints
4-8 Resource types
3-5 Field layers
30+ visualization FPS target on the reference scenario
```

## Desktop Packaging

Start as a local browser application served by the project tooling.

Use `Tauri 2` only when native packaging is required for:

- Windows installer;
- runner process lifecycle;
- approved local file access;
- native dialogs;
- application updates;
- desktop shortcuts and integration.

The browser application must remain independently runnable for development and testing.

## Not Selected As Baseline

### Next.js

Not required for a local client application without SSR or public content routing.

### Electron

Not selected while Tauri can package the existing frontend with lower duplication and direct Rust integration.

### Three.js

Not required for the 2D baseline. Evaluate only when true 3D becomes an accepted requirement.

### WebGPU

Not a production baseline. It may be evaluated later behind renderer boundaries after browser, driver and fallback testing.

### Canvas 2D Primary Renderer

Not suitable as the main renderer for the target entity count and visual layering.

### Fully Styled UI Kits

MUI, Ant Design and similar systems are not the visual foundation because ALife requires a custom scientific/game-like design language.

## Version Policy

- Pin exact dependencies in the lockfile.
- Use the current stable major at the start of the implementation slice unless this document names a required major.
- Do not upgrade renderer, framework or chart major versions during an active slice without a separate compatibility task.
- Record browser/runtime minimums in the UI workspace.
- Keep fixture, protocol and screenshot tests across dependency upgrades.

## Validation Gate Before Final ADR

Before promoting this draft into an accepted ADR, `UI-1A` must prove:

```text
React application shell starts
PixiJS Viewer renders deterministic fixture
one Resource/Field heatmap is visible
20k Cell performance smoke is measured
zoom, pan, selection and full-screen work
Inspector shows fixture data
Light/Dark and UI scale work
screenshot export works
Chromium and WebGL2 requirements are documented
no Viewer state affects simulation behavior
```

## Open Decisions

Resolve in detailed UI worklogs or the final UI technology ADR:

- exact binary frame codec;
- exact PixiJS batch/mesh strategy;
- package manager and Node runtime pinning;
- shader and asset pipeline;
- exact chart import strategy;
- screenshot comparison tolerances;
- Tauri packaging boundary and timing.

## Semantic Links

- constrained by: [[docs/engine/technology-stack|Engine Technology Stack]]
- implements: [[docs/ui/architecture|UI Architecture]]
- implements visualization for: [[docs/ui/visualization|UI Visualization]]
- implements quality requirements from: [[docs/ui/quality|UI Quality]]
- planned by: [[docs/implementation/implementation-plan-ui|UI Implementation Plan]]
- uses projection rules from: [[docs/mechanics/observer-projection|Observer Projection]]
- future decision: [[docs/decisions/INDEX|Decisions Index]]
