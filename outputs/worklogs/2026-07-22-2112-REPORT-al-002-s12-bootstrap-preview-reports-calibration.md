---
tags:
  - alife
  - worklog/report
  - delivery/al-002-s12
---

# REPORT: AL-002-S12 Bootstrap Preview, Reports, And Calibration

## Result

Status: done

Implemented a compact Bootstrap-4 preview/report path for human validation and batch calibration without starting Core or executing ticks.

## Scope Closed

- Added `src/bootstrap/preview.rs` with:
  - `build_bootstrap_preview`;
  - bounded resource layer preview cells;
  - manifest/seed-domain/generator-version summaries;
  - explicit manifest-only field summaries;
  - viability checks and warnings;
  - deterministic seed-sweep rows.
- Added `runner --bootstrap-preview <scenario-id-or-path>` as a JSON CLI path over the shared Bootstrap preview API.
- Documented the CLI preview command in `docs/RUNNER_USAGE.md`.
- Preserved the existing warning boundary: `BOOTSTRAP_FIELD_LAYER_NOT_CORE_INTEGRATED` means Bootstrap field generators are manifest summaries only until Core owns spatial field grids.

## Out Of Scope

- No Core spatial `FieldGrid`.
- No runtime field mechanics.
- No UI World Editor.
- No large preview artifacts written to disk.
- No preview/calibration output can become simulation input.

## Verification

Passed:

```text
cargo test --test bootstrap_preview --test runner_bootstrap_preview_cli --test bootstrap_rich_generators --test bootstrap_integration --test runner_scenario_loader
```

Setup note:

```text
cargo clean
```

was required before verification because the local `target/` directory exhausted disk space during Windows linking. It removed Cargo build artifacts only.

## Candidate Next Work Review

`AL-002-S12` is removed from Candidate Next Work. The next highest-value paths are:

- `AL-007-S11` if the priority is to continue visible UI inspection/search/filtering over the now source-backed resource/material data.
- `AL-002-S17` if the priority is to close remaining AL-002-owned material/repair/boundary/joint-repair debts before the final `AL-002-S18` closure matrix.
