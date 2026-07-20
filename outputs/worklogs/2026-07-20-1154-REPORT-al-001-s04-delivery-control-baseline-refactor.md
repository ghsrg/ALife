# AL-001-S04 Delivery Control Baseline Refactor Report

Plan ID: `AL-001-S04`
Status: `done`
Date: 2026-07-20

## Scope

Refactored delivery-control documentation responsibilities only. No product code,
tests, or implementation behavior was changed.

## Changes

- Moved the historical worklog-derived roadmap table out of
  `docs/delivery/roadmap.md` and into `docs/delivery/worklog-ledger.md`.
- Kept `docs/delivery/roadmap.md` as the canonical delivery plan with a short
  historical evidence pointer.
- Reworked `docs/delivery/status.md` into an operational dashboard for current,
  active, blocked, ready-next, recently closed, and control drift state.
- Added slice-level current/next acceptance rows in `docs/delivery/acceptance.md`
  while preserving non-next legacy `AL-###` rows for backward compatibility.
- Updated `docs/delivery/source-map.md` and `docs/delivery/control.md` to
  distinguish stage IDs (`AL-###`) from executable slice Plan IDs
  (`AL-###-S##`).
- Reviewed `Candidate Next Work` and removed `AL-001-S04` from the next-work
  queue after closure.

## Acceptance Coverage

| Acceptance ID | Result | Evidence |
| --- | --- | --- |
| `AL-001-S04-AC01` | pass | Delivery artifact responsibilities are separated across roadmap/status/ledger/source-map/control/acceptance. |
| `AL-001-S04-AC02` | pass | Historical report rows now live in `docs/delivery/worklog-ledger.md`; roadmap keeps only a pointer. |
| `AL-001-S04-AC03` | pass | `docs/delivery/status.md` is an operational dashboard. |
| `AL-001-S04-AC04` | pass | Current/next acceptance rows use `AL-###-S##-AC##` IDs. |
| `AL-001-S04-AC05` | pass | `Candidate Next Work` was reviewed and updated. |

## Verification

```text
git diff --check -- docs/delivery/roadmap.md docs/delivery/status.md docs/delivery/worklog-ledger.md docs/delivery/acceptance.md docs/delivery/source-map.md docs/delivery/control.md
exit 0
warnings only: LF will be replaced by CRLF
```

```text
roadmap-row-ids=64
duplicate-row-ids=0
```

```text
worklog-files-excluding-index=215
missing-from-ledger=0
```

```text
current/next slice acceptance rows found:
AL-001-S04-AC01..AC05
AL-003-S02-AC01
AL-004-S01-AC01
AL-004-S02-AC01
AL-002-S16-AC01
```

## Notes

Non-next legacy acceptance rows remain in the legacy section by decision. They
must be normalized only when their Plan ID becomes current or Candidate Next
Work.
