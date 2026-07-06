# REPORT: Over-Capacity Scenario Resolution Hardening

## Goal
Resolve conflict between the TOML parser preflight rejection of over-capacity scenario configs and the runtime collapse expectations. Over-capacity scenarios must successfully parse and resolve dynamically as a collapse in runtime.

## Scope
- **Parser Modification:** Removed the hard reject of `used_capacity > capacity_limit` from the TOML scenario parser inside `src/runner/config_parser.rs`.
- **Test Adjustment:** Renamed `native_toml_parser_rejects_over_capacity_scenario` to `native_toml_parser_loads_over_capacity_scenario` inside `tests/phase1_config_validation.rs` to assert successful configuration loading.
- **Integration Test:** Added `parsed_over_capacity_toml_collapses_in_runtime` inside `tests/phase1_config_validation.rs` to verify that parsed over-capacity configuration correctly drives a `CapacityExceeded` collapse on tick 1 when run via the `TickExecutor`.

## Verification
- **Rust Formatting:** `cargo fmt --check` completed successfully with no style issues.
- **Linter:** `cargo clippy --workspace --all-targets --all-features -- -D warnings` completed with no warnings.
- **Rust Tests:** `cargo test` executes successfully, with all 28 integration and unit tests passing.
- **Python Tests:** `python -m pytest .\tools\early-stability` completes with all 93 tests passing.

## Status
All modifications remain uncommitted in the workspace as requested.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
