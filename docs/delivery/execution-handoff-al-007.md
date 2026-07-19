---
tags:
  - alife
  - delivery/handoff
---

# Execution Handoff: AL-007

## Selected Slice

Plan ID: `AL-007`

Title: UI Start Demo, Export, And Acceptance Hardening

Legacy alias: `UI-1D`

Request type: `ROADMAP_DELIVERY`

## Dependency Pre-Check

| Dependency | Plan ID | Status | Decision |
| --- | --- | --- | --- |
| Runner live status and frame stream | `AL-002` | `done-weak-evidence` | May proceed, but do not add Runner behavior inside UI. |
| Observer projection contract | `AL-005` | `in-progress` | May proceed using existing projection contract; missing fields must be explicit. |
| UI-1C rendering/navigation | `AL-007` alias history | historical evidence | Preserve existing tests and behavior. |

## Scope

In scope:

- Start demo path and acceptance hardening.
- Screenshot export.
- Clear live/fixture/unavailable projection states.
- Dependency notes for Runner and Observer gaps.

Out of scope:

- New Runner commands.
- New Observer projection kinds.
- Debug CSV/JSON/diagnostic export.
- Research reports or experiment export.
- Genome, lineage, or OrganismView deep UI.

## Task Plan

- `AL-007-T01`: dependency pre-check against Runner and Observer contracts.
- `AL-007-T02`: add failing UI tests for Start demo state and projection provenance.
- `AL-007-T03`: implement minimal Start demo behavior.
- `AL-007-T04`: add failing UI tests for screenshot export availability and failure state.
- `AL-007-T05`: implement screenshot export within Start scope.
- `AL-007-T06`: run acceptance hardening over existing monitor/viewer/connection tests.
- `AL-007-T07`: update delivery evidence and final report.

## Verification Commands

Run from `ui/control-center/`:

```text
npm.cmd test
npm.cmd run build
npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts
```

If live Runner behavior is touched or verified, also run the smallest relevant
Runner tests from the repository root:

```text
cargo test --test runner_ws_stream --test runner_http_run_control --test runner_frame_encoder
```

## Required Closure Evidence

- `AL-007-EV01`: dependency pre-check notes.
- `AL-007-EV02`: UI unit/component test result.
- `AL-007-EV03`: Playwright Start demo/e2e result.
- `AL-007-EV04`: screenshot export test result.
- `AL-007-EV05`: production build result.
- `AL-007-EV06`: deferred Debug/Research scope list.
