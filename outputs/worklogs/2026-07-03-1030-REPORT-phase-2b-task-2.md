# Worklog: Phase 2B Reachability Validation - Task 2

## Goal
Implement reachability validation tests for gated material capabilities by stripping specific capabilities for testing.

## Scope
- Add a bitmask representing disabled capabilities (`disabled_capabilities`) to `CellStore`.
- Implement `CellStore::strip_capability_for_test(CellIndex, MaterialCapability)` to set the corresponding bit in the bitmask.
- Update `CellStore::has_capability()` to query the bitmask and return false if the capability is disabled.
- Expose `TickExecutor::world_mut()` so tests can mutably access the world and strip capabilities.
- Write integration tests checking that cells collapse when metabolism or resource uptake capabilities are missing.

## Decisions
- Used a u8 bitmask to represent disabled capabilities. Each of the `MaterialCapability` variants was mapped to a bit via a clean, zero-overhead private `const fn capability_bit`.
- Implemented `strip_capability_for_test` directly in `CellStore` rather than introducing complex dependency mocking or runtime injection, keeping in line with the "headless core source of truth" principle.

## Files Changed
- `src/core/cell_store.rs` (Added capabilities stripping and bitmask tracking)
- `src/core/tick.rs` (Exposed `world_mut()` on `TickExecutor`)
- `tests/phase2_reachability.rs` (Added tests checking that missing capabilities lead to collapse)

## Verification
- Ran integration tests: `cargo test --test phase2_reachability` (passed)
- Ran all cargo tests: `cargo test` (all 60 tests passed)
- Checked formatting: `cargo fmt --check` (clean)
- Ran clippy: `cargo clippy --workspace --all-targets --all-features -- -D warnings` (clean, 0 warnings)
- Ran Python stability tests: `python -m pytest .\tools\early-stability` (93 passed)

## Open Questions
- None.
