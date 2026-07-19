---
tags:
  - alife
  - delivery/scenarios
---

# Agent Scenario Cards

## AL-007: UI Start Demo, Export, And Acceptance Hardening

### `AL-007-AC01`: Dependency Pre-Check

Plan ID: `AL-007`

Source-of-truth links:

- [[docs/implementation/implementation-plan-ui|UI Implementation Plan]]
- [[docs/implementation/implementation-plan-runner|Runner Implementation Plan]]
- [[docs/observer/projection-contract|Projection Contract]]
- [[docs/observer/observer-layer|Observer Layer]]
- [[docs/delivery/roadmap|Delivery Roadmap]]
- [[docs/delivery/status|Delivery Status]]

Intent: confirm that `UI-1D` can proceed without expanding Runner or Observer
contracts.

Priority: P0

Given:

- Runner exposes live status and `ALIF v2` frame stream.
- Observer projection rules require read-only, data-bound UI rendering.
- UI-1C work has already addressed rendering truthfulness and navigation.

When:

- planning `UI-1D` implementation work;

Then:

- missing Runner or Observer behavior is recorded as a dependency or follow-up,
  not silently implemented inside UI;
- UI work stays inside Start Demo, export, and acceptance hardening scope;
- no UI state is treated as source of truth.

TDD obligation:

- add or update tests only for UI behavior owned by `AL-007`;
- do not add Runner or Observer behavior through UI tests.

Evidence requirements:

- `AL-007-EV01`: dependency pre-check notes in the execution report.

### `AL-007-AC02`: Start Demo Path

Plan ID: `AL-007`

Source-of-truth links:

- [[docs/implementation/implementation-plan-ui#Start Demo Scenario|UI Start Demo Scenario]]
- [[docs/implementation/implementation-plan-ui#Start Acceptance Gate|UI Start Acceptance Gate]]

Intent: make the Start demo path coherent enough to show a live or fixture-backed
world without misleading projection claims.

Priority: P0

Given:

- the Monitor workspace can render fixture and live projection states;
- unavailable projection fields are explicit;

When:

- a user opens the Control Center for a Start demo;

Then:

- demo state, connection state, and projection provenance are visible;
- the Viewer remains usable at Start scope;
- unavailable data is marked unavailable rather than inferred.

TDD obligation:

- add failing UI tests for demo state and provenance before implementation.

Evidence requirements:

- `AL-007-EV02`: `npm.cmd test` result.
- `AL-007-EV03`: Playwright Start demo/e2e result.

### `AL-007-AC03`: Screenshot Export

Plan ID: `AL-007`

Source-of-truth links:

- [[docs/implementation/implementation-plan-ui#Export|UI Export]]
- [[docs/implementation/implementation-plan-ui#Start Acceptance Gate|UI Start Acceptance Gate]]

Intent: provide Start-scope screenshot export without introducing research/debug
export scope.

Priority: P1

Given:

- the Viewer has a current viewport;

When:

- a user exports a Start demo screenshot;

Then:

- the export captures the current visual state;
- export failures are visible;
- CSV, JSON debug, diagnostic, and research exports remain out of scope.

TDD obligation:

- add focused tests for screenshot command availability and failure state.

Evidence requirements:

- `AL-007-EV04`: UI test result for export behavior.

### `AL-007-AC04`: Acceptance Hardening

Plan ID: `AL-007`

Source-of-truth links:

- [[docs/implementation/implementation-plan-ui#Start Acceptance Gate|UI Start Acceptance Gate]]
- [[docs/ui/quality|UI Quality]]

Intent: close Start acceptance risks without expanding into `UI-2 Debug` or
`UI-3 Research`.

Priority: P1

Given:

- Start demo and export behavior exist;

When:

- validating the Start slice;

Then:

- regressions in monitor layout, connection state, viewer navigation, and
  projection truthfulness are covered;
- acceptance evidence is attached to `AL-007`;
- remaining Debug/Research work is deferred explicitly.

TDD obligation:

- preserve existing UI-1C tests and add only Start-scope regression tests.

Evidence requirements:

- `AL-007-EV05`: `npm.cmd run build` result.
- `AL-007-EV06`: selected Playwright e2e result.
