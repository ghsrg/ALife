---
tags:
  - alife
  - worklog/report
  - plan/AL-003-S04
---

# REPORT: AL-003-S04 Genome Copying, Mutation, And Repair

## Status

done

## Summary

Implemented material-backed Genome copying as a registered Core process with deterministic bounded mutation and explicit division gating for Genome-bearing Cells.

## Changes

- Added `ProcessId::GenomeCopying` runtime execution path with feasibility gates for config enablement, Genome presence, material capability, energy, resource carrier cost, capacity, and completed-copy state.
- Added `GenomeCopyingConfig` and TOML parser support, including `scheduler.cell.genome_copying_ticks`.
- Added CellStore-owned transient copy state: copied Genome id, copy progress, and copied carrier amount counted in used capacity.
- Added deterministic World-owned Genome copy creation with bounded mutation during copy completion.
- Updated division so Genome-bearing Cells require a completed Genome copy and transfer the copied Genome carrier to daughter B while resetting transient copy state.
- Added conservative sweeper-ready scenario config at `config/scenarios/genome/phase3c_genome_copying_conservative.toml`.

## Verification

- `cargo test --test phase3c_genome_copying --no-run`
- `cargo test --test phase3c_genome_copying`
- `cargo test --test phase3b_runtime_contract --test phase2_process_registry --test phase3a_action_plan --test scheduler_config --test scheduler_process_cadence --test runner_scenario_loader`

## Notes

- Repair is represented as explicit copying feasibility/rejection and deterministic mutation bounds in this slice. Rich damaged-carrier repair remains `Needs Review` until Canon defines damaged Genome carrier state beyond current integrity metadata.
- Lineage event log/replay is not implemented here and remains `AL-003-S05`.
- Balance values are conservative defaults intended for sweeper calibration, not final ecosystem tuning.
