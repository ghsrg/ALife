# Runner Debug Snapshot Cadence Report

## Summary

- Added explicit `SnapshotCadence` with `EveryTick`, `EveryNTicks`, and `OnDemandOnly`.
- Made headless debug use on-demand snapshots through `RunEngineConfig::headless_debug()`.
- Changed CLI debug progress and final summary to sample the latest committed state on demand.
- Fixed `demo_living_world` Genome cadence override from every Tick to scheduled cadence.
- Added scheduler diagnostics to terminal progress:
  - `snapshots`;
  - `genome`;
  - `decay_dt`.
- Updated runner usage and scheduler documentation to separate snapshot cache cadence from UI projection cadence.

## Verification

- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --test runner_snapshot_cadence`
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --test runner_headless_e2e --test runner_http_run_control --test runner_ws_stream --test runner_projection_world_frame`
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --test runner_headless_e2e headless_debug_can_run_ticks_without_building_cached_snapshots`
- `cargo run --release --bin runner -- --debug --progress-interval-ms 2000 bootstrap_minimal_viable_world`
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --test runner_scenario_loader --test scheduler_genome_cadence --test scheduler_determinism`
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --test runner_progress --test runner_headless_e2e`
- `cargo fmt --check`
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --test runner_snapshot_cadence --test runner_progress --test runner_headless_e2e --test runner_scenario_loader --test scheduler_genome_cadence --test scheduler_determinism`
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --workspace`

All listed verification commands passed.

## Manual smoke notes

`bootstrap_minimal_viable_world` release debug smoke completed and reported:

```text
Final tick: 20
```

`demo_living_world` release debug smoke was run with the full 50,000 Tick scenario. The command timed out before scenario completion, but the progress output confirmed the intended behavior:

```text
snapshots: 2, 3, 4...
genome: often 0, sometimes non-zero
decay_dt: 5 on scheduled resource decay ticks, 0 otherwise
```

This confirms terminal debug output can now show whether scheduler cadence is active.

## Notes

- UI projection remains wall-clock limited through `ViewerProjectionSampler`.
- `SnapshotCadence` is cache/debug policy, not simulation authority.
- The `demo_living_world` config edit changing `runtime_interval_ticks` from `1` to `10` existed in the worktree before implementation began; this report intentionally includes it because it is required for the accepted fix.
