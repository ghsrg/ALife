# PLAN: Phase 1 Rust Core Hardening

## Goal
Conduct hardening and refactoring of the Phase 1 Rust core to prepare the codebase for Phase 1B/1C multi-cell scale, eliminating placeholders, and addressing potential performance constraints.

## User Review Required
> [!IMPORTANT]
> **TOML Parsing Integration:** Currently, scenario TOML parameters are hardcoded in Rust integration tests as fixtures. Hardening recommends adding a TOML parsing crate (e.g. `toml` or `serde` via workspace dependencies) to allow loading the `.toml` files directly, eliminating duplicated hardcoded configurations.

## Open Questions
*   **External dependencies restriction:** Is the use of `serde` and `toml` crates allowed in the Rust core workspace, or must we implement a lightweight custom TOML scanner to keep dependencies standard-library-only? We assume standard workspace dependency inclusion is allowed for parsing.

## Proposed Changes

---

### [Component: alife-core]

#### [MODIFY] [units.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/units.rs)
- Add `#[repr(transparent)]` to all amount wrappers to guarantee zero-cost wrapper optimization by LLVM.
- Introduce `pub(crate) const fn new_unchecked(val: f32) -> Self` to bypass branch validation checks in performance-critical execution loops.

#### [MODIFY] [cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs)
- Explicitly add genome capacity placeholders and internal fragments capacity placeholders inside `used_capacity()` to match the Phase 1 data model specification.

#### [MODIFY] [resources.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/resources.rs)
- Implement `decay_or_passive_update` for the `ResourceGrid` and integrate it into `TickExecutor::step` to replace the static resource layer placeholder with active simulation behavior.

#### [MODIFY] [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs)
- Refactor the hardcoded cell index `0` lookup. Change `step()` to iterate dynamically over `cells.iter_indices()` to prepare the tick executor for multi-cell systems.

---

### [Component: alife-runner]

#### [MODIFY] [Cargo.toml](file:///c:/Users/korsr/PycharmProjects/ALife/Cargo.toml)
- Include `serde` (with derive features) and `toml` as dependencies.

#### [NEW] [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config_parser.rs)
- Implement a parser that maps scenario `.toml` files directly into `RuntimeConfig`, matching the Python `config_loader` checks (e.g., initial resource sums exceeding capacity limits).

---

## Verification Plan

### Automated Tests
- Run all Rust tests to ensure 100% regression coverage:
  ```bash
  cargo test
  ```
- Run linter and formatting:
  ```bash
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo fmt --check
  ```

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
