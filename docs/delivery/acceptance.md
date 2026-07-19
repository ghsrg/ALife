---
tags:
  - alife
  - delivery/acceptance
---

# Delivery Acceptance

## Acceptance Matrix

| Acceptance ID | Plan ID | Acceptance outcome | Required evidence |
| --- | --- | --- | --- |
| `AL-001-AC01` | `AL-001` | Every historical worklog has one ledger row. | `docs/delivery/worklog-ledger.md` |
| `AL-001-AC02` | `AL-001` | Delivery streams have explicit owners and status classes. | `docs/delivery/status.md` |
| `AL-001-AC03` | `AL-001` | Legacy labels remain backward-compatible aliases. | `docs/delivery/control.md` |
| `AL-002-AC01` | `AL-002` | Runner phases have source, test, and worklog evidence mapped. | Runner docs, `src/runner/`, `src/viewer_server/`, `tests/runner_*.rs` |
| `AL-003-AC01` | `AL-003` | Core Phase 2 and integrated world status is separated from later Genome work. | Phase 2 docs, `src/core/`, Phase 2 tests |
| `AL-004-AC01` | `AL-004` | Genome Phase 3A evidence is mapped without implying full Phase 3 completion. | Genetics docs, genome source/tests, Phase 3A worklogs |
| `AL-005-AC01` | `AL-005` | Observer contracts, analyzer evidence, and UI projection needs are reconciled. | Observer docs, observer source/tests, worklogs |
| `AL-006-AC01` | `AL-006` | Bootstrap foundation status is mapped to Runner startup constraints. | Bootstrap docs/source/tests/worklogs |
| `AL-007-AC01` | `AL-007` | UI-1D dependency pre-check respects Runner, Observer, and projection boundaries. | UI plan, Runner plan, Observer projection contract |
| `AL-007-AC02` | `AL-007` | Start demo path is coherent and does not infer unavailable projection data. | UI tests and e2e evidence |
| `AL-007-AC03` | `AL-007` | Screenshot export works within Start scope. | UI tests and manual/e2e evidence |
| `AL-007-AC04` | `AL-007` | Start acceptance hardening preserves UI-1C behavior and defers Debug/Research scope. | UI tests, build, e2e evidence |
| `AL-008-AC01` | `AL-008` | Stability tools and reachability evidence are indexed without entering the simulation hot path. | Early stability docs/tool/tests/worklogs |

## Evidence Rules

- Worklogs can support an evidence row, but cannot be the only reason to mark a
  delivery item `done-evidenced`.
- A completed report with verification commands but no delivery coverage mapping
  starts as `done-weak-evidence`.
- A stream with code and tests but unclear Canon or ownership starts as
  `Needs Review`.
