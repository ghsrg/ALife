---
tags:
  - alife
  - worklog/plan
  - ui
  - docs/audit
---

# PLAN: UI Docs TDD Readiness Audit Fixes

## Summary

UI Canon, UI implementation plan, and UI technology stack are mostly aligned and are sufficient to start detailed TDD planning for `UI-1A: Application Shell And Deterministic Fixture Viewer`.

No blocking architecture contradiction was found. The remaining issues are navigation/synchronization fixes that should be handled before or inside the first UI planning slice.

## P1. Deferred UI stack decisions still listed in implementation plan

Problem: `docs/implementation/ui-technology-stack.md` fixes React 19, TypeScript, Vite, PixiJS 8, Radix UI Primitives, Zustand, Apache ECharts 6, TanStack Table/Virtual, Vitest/RTL/Playwright and future Tauri 2. But `docs/implementation/implementation-plan-ui.md` still lists `frontend framework`, `component library`, `chart library`, and `state-management library` as deferred decisions.

Impact: An agent writing `UI-1A` TDD can treat already-selected stack pieces as open and propose a competing stack or waste planning effort.

Proposal: Update `Deferred Decisions` in `implementation-plan-ui.md`: remove items already fixed by `ui-technology-stack.md`, keep only still-open choices such as binary frame codec, exact WebSocket protocol, package manager/Node pinning, shader/asset pipeline, screenshot tolerances, exact LOD thresholds and Tauri timing.

## P1. UI Technology Stack is not linked from UI index

Problem: `docs/implementation/ui-technology-stack.md` is linked from `docs/implementation/INDEX.md`, but not from `docs/ui/INDEX.md`.

Impact: An agent entering through UI Canon may read UI docs and implementation plan but miss the concrete stack document.

Proposal: Add `UI Technology Stack` to `docs/ui/INDEX.md` under Implementation Links and to `docs/ui/README.md` semantic links if not present.

## P2. Mockup is referenced as plain path, not Obsidian/resource link

Problem: `docs/ui/visualization.md` references `docs/ui/control-center-monitor-v3.png` as plain text.

Impact: The image is findable, but less connected in Obsidian and less explicit as a visual reference for `UI-1A/UI-1C`.

Proposal: Convert it to a Markdown image/link reference and add a short note: visual target for Monitor composition, not exact pixel spec.

## P2. UI-1A acceptance should explicitly point to the stack validation gate

Problem: `implementation-plan-ui.md` has `UI-1A` minimum outcome and `ui-technology-stack.md` has a pre-ADR validation gate. They are consistent but separated.

Impact: A TDD plan may implement fixture viewer without measuring or recording the stack validation evidence.

Proposal: In the future `PLAN-ui-1a-application-shell-fixture-viewer.md`, include both documents as sources and copy the stack validation items into acceptance gates.

## P2. First TDD slice needs fixture/projection contract before UI components

Problem: The docs correctly require one adapter contract for recorded fixtures and live transport, but `UI-1A` can be planned too visually if the fixture projection schema is not defined first.

Impact: Viewer and Inspector may become demo-only and require rework for live Core.

Proposal: In `UI-1A` TDD plan, start with a minimal versioned fixture projection contract: run metadata, world bounds, tick, cells, resource heatmap, selection detail, layer metadata and warnings.

## Non-Blocking Notes

- The mockup `docs/ui/control-center-monitor-v3.png` is structurally aligned with Monitor layout: top shell, left layers, center Viewer, right Inspector, bottom data panels.
- The UI docs consistently preserve observer-only and Core-authoritative boundaries.
- The plan correctly starts with recorded deterministic fixture, then live projection transport.
- `UI-1A` can start as TDD planning after the P1 sync fixes, or with those fixes included as the first checklist items.

## Recommended Next Step

Create detailed TDD plan:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-PLAN-ui-1a-application-shell-fixture-viewer.md
```

Minimum outcome:

```text
React/Vite app shell starts
Monitor workspace opens
versioned deterministic fixture loads
PixiJS/WebGL2 Viewer draws world bounds, cells and one resource heatmap
zoom/pan/full-screen work
cell selection updates read-only Inspector
Light/Dark themes work
screenshot export works
fixture projection contract is tested
no Viewer/UI state mutates simulation state
```

## Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
