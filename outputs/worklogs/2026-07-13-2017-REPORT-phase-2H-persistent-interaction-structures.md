# REPORT: Phase 2H Persistent Interaction Structures

## Summary

Implemented Phase 2H persistent local Joints with World-owned `JointStore`, typed `JointId`, material-backed creation, mechanical constraint, passive Resource channel, delayed scalar Signal channel, Heat channel, deterministic degradation/break, death/division lifecycle handling, observer projection, parser support, and `sweep_analyzer` raw-data coverage.

Also fixed a Phase 2G repair accounting blocker found during full analyzer verification: repair commit no longer panics when the configured repair resource is unavailable or numerically short at commit time; it rolls back the attempted consumption and records a repair rejection instead.

## Implemented

- Added `src/core/joints.rs` with stable IDs, ordered endpoints, lifecycle state, channel config, signal buffering, degradation, inert and break helpers.
- Added `RuntimeConfig::joints` and TOML parser support for `[joints]`.
- Added `ProcessId::JointCreate` and `ProcessId::JointRepair` plus joint rejection reasons.
- Integrated joint creation into Tick execution through local contact eligibility and material/resource/energy costs.
- Added mechanical correction, Resource transfer, scalar Signal delay, Heat transfer, degradation/break metrics, and lifecycle cleanup.
- Added observer-only connected-component projection for organism view features; no organism controller or behavior authority was introduced.
- Extended `sweep_analyzer` full and smoke configs with Phase 2H scenarios and joint raw-data columns.
- Added canonical fixture `config/scenarios/joints/phase2h.toml`.

## Acceptance Gates

- Stable connected cell structure exists: covered by `phase2h_joint_creation`, `phase2h_joint_store`, and analyzer `joint_creation_viability`.
- Joint costs matter: creation consumes structural material/resource/energy and rejects material-free or non-local cases.
- Joint breaks/degrades: covered by `phase2h_joint_lifecycle` and analyzer `joint_degradation_break`.
- Resource/Signal/Heat channels remain local: covered by `phase2h_joint_channels` and analyzer raw columns.
- No direct Energy transfer: explicitly covered by resource and heat channel tests.
- No organism controller: observer projection is derived-only; connected components are not fed back into simulation.

## Verification

- `cargo fmt --check` passed.
- `cargo test --workspace --all-targets` passed.
- `cargo test --test phase2h_reachability` passed and generated Phase 2H smoke CSVs under `outputs/raw_data/smoke/`.
- `cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml` reached and wrote all Phase 2H full sweep CSVs under `outputs/raw_data/`, then timed out during the final `resource_abundance` matrix/report section at 15 minutes.

Observed Phase 2H raw-data files:

- `outputs/raw_data/joint_creation_viability.csv`
- `outputs/raw_data/joint_resource_channel.csv`
- `outputs/raw_data/joint_signal_delay.csv`
- `outputs/raw_data/joint_heat_channel.csv`
- `outputs/raw_data/joint_degradation_break.csv`
- `outputs/raw_data/joint_lifecycle_division.csv`

## Known Limits

- `JointRepair` is registered but remains disabled as an explicit future process.
- Analyzer `joint_lifecycle_division` proves reachability through raw joint metrics; stricter division-break semantics are covered by core tests.
- Full analyzer runtime is now too long for a 15-minute command timeout because it completes all sweeps and then starts the matrix section. Smoke analyzer is the fast CI/sanity path.
