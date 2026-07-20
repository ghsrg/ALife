---
tags:
  - alife
  - delivery/control
---

# Delivery Control

## Purpose

This directory is the delivery-control layer for ALife. It does not replace Canon,
implementation plans, old worklog indexes, or historical worklog names.

## Authority

Authority order for delivery planning:

1. `docs/PRINCIPLES.md`
2. Canon documents under `docs/world/`, `docs/biology/`, `docs/genetics/`,
   `docs/evolution/`, `docs/config/`, `docs/engine/`, `docs/runner/`,
   `docs/observer/`, and `docs/ui/`
3. Accepted ADRs under `docs/decisions/`
4. Implementation plans under `docs/implementation/`
5. Code and tests as implementation evidence
6. Worklogs as historical evidence only

`outputs/worklogs/` must not be treated as canonical requirements.

## Stable IDs

Delivery stage IDs use the compact `AL-###` format. Executable TDD Plan IDs use
`AL-###-S##` child slices under those stages.

Related IDs:

- Acceptance IDs: `AL-###-S##-AC##`
- Evidence IDs: `AL-###-S##-EV##`

Legacy top-level acceptance and evidence IDs such as `AL-###-AC##` may remain in
older delivery-control artifacts for backward compatibility, but new current and
Candidate Next Work acceptance should use slice-level IDs.

Legacy labels such as `Phase 2G`, `Runner-3`, `UI-1C-D`, and `Bootstrap-1`
remain valid aliases for historical compatibility. They are not replaced and
must not be renamed in old worklogs.

## Status Values

Use these values in `docs/delivery/status.md` and `docs/delivery/roadmap.md`:

- `done-evidenced`: docs, code/tests, and worklog evidence agree.
- `done-weak-evidence`: work appears complete, but closure mapping is weak.
- `in-progress`: active or recent work exists, but closure is not settled.
- `planned`: source-backed work not yet implemented.
- `blocked`: a named decision, source, command, or approval is missing.
- `Needs Review`: evidence is incomplete, ambiguous, or ownership is unclear.

Do not mark a slice `done-evidenced` from a worklog alone.

## Stream Ownership

Initial delivery streams:

- `Core`: simulation mechanics and deterministic world state under `src/core/`.
- `Bootstrap`: Tick 0 preparation under `src/bootstrap/`.
- `Runner`: orchestration, lifecycle, command validation, and projections under
  `src/runner/`.
- `Viewer Server`: HTTP and WebSocket adapter under `src/viewer_server/`.
- `Observer`: read-only projections, classification, and coverage under
  `src/observer/`.
- `Genome`: genome model, runtime, action planning, mutation, and inheritance.
- `UI`: ALife Control Center under `ui/control-center/`.
- `Stability Tools`: early stability and reachability tooling under
  `tools/early-stability/`.

Top-level modules `src/world`, `src/simulation`, `src/renderer`, `src/physics`,
`src/organism`, and `src/cell` are currently `Needs Review` ownership areas
because they contain placeholders while implemented behavior lives elsewhere.

## Backward Compatibility

Do not rename historical worklogs, legacy phase labels, or old worklog indexes
during delivery initialization. New delivery artifacts should link back to old
labels and worklogs instead.
