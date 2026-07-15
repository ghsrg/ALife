---
tags:
  - alife
  - worklog/report
  - ui
---

# UI-1A Application Shell And Deterministic Fixture Viewer Report

## Summary

Implemented the first runnable `ALife Control Center` UI slice in `ui/control-center`.

Completed:

- Added Vite React TypeScript workspace with npm lockfile.
- Added deterministic `WorldFrameProjection/v1` fixture and fixture adapter.
- Added Zustand-backed fixture application state.
- Added Monitor shell with disabled non-UI-1A placeholders.
- Added Layer panel, PixiJS World Viewer, selection-linked Cell Inspector, Light/Dark theme toggle, and PNG export status.
- Added Vitest/RTL unit coverage and Playwright 1024x768 smoke coverage.

## Scope Notes

Still deferred:

- live Runner transport;
- ALIF decoding;
- OrganismView detail;
- World Editor;
- rich analytics;
- experiment runner;
- production design alignment session;
- functional Step N execution.

## Deviations From Plan

- `@vitejs/plugin-react` was placed in `devDependencies`, not runtime `dependencies`.
- Playwright browser bundle download repeatedly timed out in this environment. The E2E config now falls back to installed Chrome or Edge when present, while still supporting the standard bundled browser path.
- Vite/Vitest commands require elevated filesystem read permissions in this sandbox because esbuild probes parent directories while loading config.
- npm install used a local `.npm-cache` because the global npm cache returned `EPERM`.

## Verification

Passed during implementation:

```text
npm.cmd test
npm.cmd run build
npm.cmd run e2e
```

Additional project-level checks are run in the final verification step.

## Result

`UI-1A` is implemented as a fixture-only monitor slice. It is ready for follow-up `UI-1B` Runner transport work after the planned interface design checkpoint.
