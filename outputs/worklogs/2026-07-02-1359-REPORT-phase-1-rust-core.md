# REPORT: Phase 1 Rust Core

## Goal

Implemented the first deterministic Phase 1 Rust core vertical slice.

## Scope

- Core module skeleton under `src/core/`.
- Typed IDs (`CellId`, `ResourceTypeId`, `MaterialTypeId`, `EventId`).
- Validated `RuntimeConfig` and custom FNV-based hashing.
- One-cell `WorldState` and SoA `CellStore`.
- Phase 1 mandatory Energy accounting, including dormancy transitions and wake-up checks.
- Heat/waste/capacity lifecycle checks.
- EventBuffer preventing default success-event spam.
- Read-only snapshot (`CommittedSnapshot`) and viewer projection (`ViewerFrame`).
- Deterministic replay check.
- Thin main runner shell.

## Verification

### Formatting
```text
cargo fmt --check
```
*Passed with zero differences.*

### Clippy Checks
```text
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
*Passed with no warnings or errors.*

### Cargo Tests
```text
cargo test
```
*19 tests passed successfully (0 failed).*
- `tests/phase1_core_smoke.rs` (10 tests)
- `tests/phase1_accounting.rs` (8 tests)
- `tests/phase1_determinism.rs` (1 test)

### Python early-stability pytest checks
```text
python -m pytest .\tools\early-stability
```
*93 tests passed successfully.*

## Decisions & Design
- **Single-Cell Dormancy Math:** To match the early stability parameter tuning logic, we implemented the transition from alive/stressed to dormant and from dormant to dead correctly by properly deducting the dormant mandatory upkeep cost and evaluating the post-upkeep state.
- **Uncommitted Status:** All modifications remain uncommitted in the workspace as requested.
- **Pure SoA Structure:** State properties of Cells are organized in contiguous parallel vectors in `CellStore`, keeping memory layouts cache-friendly and avoiding per-entity heap allocations.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
