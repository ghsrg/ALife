# Scheduler Cadence Implementation Report

## Summary

- Added explicit simulation time and scheduler cadence config for cell, world, and observer systems.
- Added cached per-cell Genome ActionPlan refresh with effective cadence from scheduler default, template override, regulatory depth, and deterministic staggering.
- Added execution-attempt cadence gates for material synthesis and boundary repair while preserving mandatory upkeep and process accounting every Tick.
- Added elapsed-tick resource decay cadence; diffusion and other propagation systems remain every Tick unless they gain safe elapsed integration.
- Added explicit RunEngine snapshot cadence and latest committed snapshot access.
- Added ViewerProjectionSampler with wall-clock projection cadence, latest-state policy, heartbeat decision API, and forced frames on run-control transitions.
- Added ALIF frame v2 interpolation metadata.
- Added stable state hashing and scheduler benchmark command.
- Updated `docs/engine/scheduler.md` to clarify that `[scheduler.fast]` is a conceptual/reserved Tick Core invariant category, not a parsed scenario contract in this implementation.

## Verification

- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --test phase2_config_hash`
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --test scheduler_determinism`
- `cargo build --release --bin scheduler_benchmark`
- `cargo run --release --bin scheduler_benchmark -- demo_living_world 1000 5`
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --test scheduler_config --test scheduler_genome_cadence --test scheduler_process_cadence --test scheduler_world_cadence --test scheduler_observer_cadence --test scheduler_determinism --test runner_ws_stream --test runner_headless_e2e`
- `cargo fmt --check`
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --workspace`

All listed verification commands passed.

## Performance

- `demo_living_world` TPS before: not recorded before this branch.
- `demo_living_world` median TPS after: `224.45`
- Benchmark command:

```powershell
cargo run --release --bin scheduler_benchmark -- demo_living_world 1000 5
```

## Notes

- Stable state hash uses explicit FNV-1a byte encoding and does not use `DefaultHasher`.
- Stable state hash covers committed tick, config hash, cells, lifecycle/runtime flags, energy, generic resources, material slots and damage, cached action plans, Genome state visible through cells, external resource grid, environment, fragments, and joints.
- Resource diffusion, passive reactions, heat diffusion, and field updates are not cadence-gated yet because they need explicit elapsed-tick semantics before being safely scheduled.
- Viewer heartbeat decision logic exists in `ViewerProjectionSampler`; current server integration emits projection frames from committed Tick and forced control transitions. A separate idle heartbeat task can be added later if UI needs repeated latest-frame heartbeats while no new Tick is committed.
