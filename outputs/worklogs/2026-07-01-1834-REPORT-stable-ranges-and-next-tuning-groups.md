# REPORT: Stable Ranges And Next Tuning Groups

Date: 2026-07-01 18:34

## Goal

Accept useful ranges from the first `early-stability` tuning pass and define the next tuning groups for external agents.

## Decisions

Accepted as experimental ranges:

| Parameter | Current Baseline | Stable Min | Stable Max | Tool Recommended | Decision |
| --- | ---: | ---: | ---: | ---: | --- |
| `cell.initial_energy` | `50.0` | `10.0` | `50.0` | `30.0` | Keep current baseline; accept range. |
| `cell.mandatory_cost_per_tick` | `2.0` | `1.0` | `5.0` | `3.0` | Keep `2.0`; accept range. |
| `environment.heat_dissipation_rate` | `0.2` | `0.05` | `0.5` | `0.275` | Keep current baseline; accept range. |

Source scenario configs were not changed.

## Files Changed

- `docs/config/stability_bounds.md`
  - Added accepted experimental ranges from `outputs/stability/single_cell_tune/ranges.json`.
  - Documented that tool midpoint recommendations are not automatic baseline changes.
  - Listed parameters not covered by the first sweep.
- `docs/implementation/early-stability-parameter-tuning.md`
  - Added next tuning groups:
    - Energy Budget Sweep
    - Heat And Waste Sweep
    - Capacity And Material Sweep
    - Growth And Division Estimate Sweep
    - Joint Upkeep Estimate Sweep
  - Added per-group scenarios, parameters, signals to catch, expected outputs and stop rules.

## Verification

- No source scenarios were modified.
- No generated `outputs/stability/` artifacts were changed by this documentation update.

## Notes

An attempted direct patch to `docs/implementation/README.md` was skipped because the file currently displays mojibake in PowerShell and exact-line patching was unsafe in this task.
