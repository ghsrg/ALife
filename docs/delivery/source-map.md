---
tags:
  - alife
  - delivery/source-map
---

# Delivery Source Map

## Source Types

| Source type | Paths | Delivery role |
| --- | --- | --- |
| Operating rules | `AGENTS.MD` | Agent workflow and project guardrails |
| Principles | `docs/PRINCIPLES.md` | Highest-level project requirements |
| Canon | `docs/world/`, `docs/biology/`, `docs/genetics/`, `docs/evolution/`, `docs/config/`, `docs/engine/`, `docs/runner/`, `docs/observer/`, `docs/ui/` | Canonical requirements and contracts |
| ADR | `docs/decisions/` | Accepted architecture decisions |
| Implementation plans | `docs/implementation/` | Planned realization of Canon |
| Documentation status | `docs/ROADMAP.md` | Documentation roadmap only |
| Code evidence | `src/`, `ui/control-center/src/`, `tools/early-stability/src/` | Implemented behavior |
| Test evidence | `tests/`, `ui/control-center/src/**/*.test.*`, `ui/control-center/tests/`, `tools/early-stability/tests/` | Verification evidence |
| Historical evidence | `outputs/worklogs/` | Plans and reports; not source of truth |

## Current Interpretation

- `docs/ROADMAP.md` remains the documentation roadmap and must not be used as
  the delivery roadmap.
- `docs/delivery/roadmap.md` is the delivery roadmap initialized from Canon,
  implementation plans, code/test evidence, and historical worklogs.
- `AL-###` IDs are large delivery stages. Executable TDD Plan IDs use
  `AL-###-S##` child slices.
- `docs/delivery/status.md` is an operational dashboard over current, active,
  blocked, ready-next, and recently closed work. It is not the canonical
  roadmap.
- `outputs/worklogs/index.md` remains the old worklog index for backward
  compatibility.
- `docs/delivery/worklog-ledger.md` is the delivery-control ledger over
  historical worklogs and worklog-derived historical rows.

## Drift Notes

| Gap | Handling |
| --- | --- |
| `G01` documentation roadmap drift | Delivery status is separated into `docs/delivery/status.md`. |
| `G02` missing delivery-control layer | Initialized under `docs/delivery/` and `.agent/active-context.md`. |
| `G03` missing stable IDs | Stage IDs use `AL-###`; executable TDD Plan IDs use `AL-###-S##`; legacy labels are aliases. |
| `G04` UI next-slice ambiguity | `UI-1D` may be the next UI slice, but global next work must be selected from the delivery roadmap. |
| `G05` ALIF protocol drift | Current protocol baseline is `ALIF v2`; older worklogs are historical evidence. |
| `G06` closure gap | Historical completion claims require acceptance/evidence mapping before `done-evidenced`. |
| `G07` ownership gap | Placeholder top-level modules are `Needs Review`. |
| `G08` stale historical evidence | Old worklogs are not rewritten during initialization. |
