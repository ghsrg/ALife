# PLAN: Refining Core Boundaries and Executor Summary Hardening

## Goal
Address key feedback on Phase 1 core boundaries and executor scaling risks:
1.  **Move the TOML parser** from `src/core/` to `src/runner/config_parser.rs` to keep `alife-core` clean of parsing dependencies.
2.  **Remove hot-path Vec allocation** inside `TickExecutor::step()` by utilizing a simple integer-based loop.
3.  **Aggregate RunSummary deterministically** across all simulated cells rather than hardcoding to cell `0`.

---

## Proposed Changes

### [Component: alife-core]

#### [DELETE] [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config_parser.rs)
- Remove config_parser from `src/core/`.

#### [MODIFY] [mod.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/mod.rs)
- Remove `pub mod config_parser;` from the core module tree.

#### [MODIFY] [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs)
- Refactor `step()` to loop over `0..self.world.cells().len()` using raw integer indices, eliminating the `Vec<CellIndex>` heap allocation.
- Aggregate `overall_lifecycle` (Collapse if *any* cell is dead, Fragile if any cell is stressed/dormant, Stable otherwise), `collapse_reason` (first non-None reason), and `final_energy` (sum of energies of all cells).

---

### [Component: alife-runner]

#### [NEW] [mod.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/mod.rs)
- Declare runner module exposing `config_parser`.

#### [NEW] [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs)
- Re-create the TOML parser code under `runner` module.

---

### [Component: alife-workspace]

#### [MODIFY] [lib.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/lib.rs)
- Register `pub mod runner;`.

#### [MODIFY] [tests/phase1_config_validation.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase1_config_validation.rs)
- Update parser imports to reference `alife::runner::config_parser::RawScenarioConfig`.

---

## Verification Plan

### Automated Tests
- Run all tests to verify correct execution:
  ```bash
  cargo test
  ```
- Run linter and formatter:
  ```bash
  cargo fmt --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  ```
