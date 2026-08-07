---
plan_id: AL-003-S10
status: done
date: 2026-08-07
---

# Scalar Field Capability Boundary And Profile Semantics Report

## Purpose

Close `AL-003-S10` by making the current Field runtime boundary explicit and testable:

- supported Field runtime kind is scalar only;
- supported profile names are `temperature`, `light`, `pressure`, `radiation`, `chemical_gradient`, and `flow`;
- profile names are non-command metadata and do not directly execute Energy, movement, mutation, damage, Resource transport, or Genome behavior.

## Source Documents Read

- `docs/PRINCIPLES.md`
- `docs/GLOSSARY.md`
- `docs/world/field-semantics.md`
- `docs/config/fields_config.md`
- `docs/mechanics/field-local-effect.md`
- `outputs/worklogs/2026-08-07-1226-PLAN-al-003-s10-scalar-field-capability-boundary.md`

## Changed Files

- `src/core/fields.rs`: added read-only `FieldProfileSemantics` value object and `FieldEffectProfile::semantics()`.
- `src/runner/config_parser.rs`: changed unsupported Field kind validation to explicitly report scalar-only runtime support.
- `tests/phase3h_local_fields.rs`: added scalar profile parser coverage, profile semantics coverage, and direct-behavior negative controls.
- `tests/phase3f_canonical_test_world.rs`: added canonical manifest disclosure assertions.
- `config/scenarios/demo/canonical_test_world.toml`: added `[canonical_manifests.fields]` scalar-only profile/support disclosure.
- `docs/config/fields_config.md`: documented current scalar-only Field runtime support.
- `docs/world/field-semantics.md`: documented current implementation status for profile semantics.
- `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `outputs/worklogs/index.md`: updated closure evidence and next-work routing.

## Verification

| Evidence ID | Command | Result |
| --- | --- | --- |
| `AL-003-S10-EV01` | `$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields field_runtime_accepts_declared_scalar_profiles_only` before parser change | FAIL as expected: `ValidationError("Unknown field kind: vector")` did not mention scalar-only runtime. |
| `AL-003-S10-EV02` | `$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields field_runtime_accepts_declared_scalar_profiles_only` | PASS: 1 passed. |
| `AL-003-S10-EV03` | `$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields field_profile_semantics_are_non_command_metadata` before semantics API | FAIL as expected: no method named `semantics` for `FieldEffectProfile`. |
| `AL-003-S10-EV04` | `$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields field_profile_semantics_are_non_command_metadata` | PASS: 1 passed. |
| `AL-003-S10-EV05` | `$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields field_profiles_do_not_execute_direct_behavior` | PASS: 1 passed; characterization confirmed no direct profile behavior. |
| `AL-003-S10-EV06` | Production direct-effect fix | Not needed; `AL-003-S10-EV05` passed without production fix. |
| `AL-003-S10-EV07` | `$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3f_canonical_test_world canonical_test_world_resolves_resource_derived_material_synthesis_surface` before manifest update | FAIL as expected: canonical source did not contain `runtime_kind`. |
| `AL-003-S10-EV08` | `$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3f_canonical_test_world canonical_test_world_resolves_resource_derived_material_synthesis_surface` | PASS: 1 passed. Note: assertions use canonicalized `key=value` source format. |
| `AL-003-S10-EV09` | `git diff --check -- docs/config/fields_config.md docs/world/field-semantics.md` | PASS. |
| `AL-003-S10-EV10` | `$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields --test phase3f_canonical_test_world --test runner_scenario_loader --test scheduler_world_cadence` | PASS: 21 tests passed. |
| `AL-003-S10-EV11` | `rustfmt --edition 2024 --check src/core/fields.rs src/runner/config_parser.rs tests/phase3h_local_fields.rs tests/phase3f_canonical_test_world.rs` | PASS. Tool emitted non-fatal warning: `could not canonicalize path C:\Users\korsr`. |
| `AL-003-S10-EV12` | `$env:RUSTFLAGS='-C debuginfo=0'; cargo test --workspace --all-targets` | FAILED on unrelated existing runner serve expectation: `tests/runner_binary_serve.rs::serve_flag_starts_http_server` expected `active_run_state == "idle"` but got `"completed"`. Isolated rerun reproduced the same failure. Targeted AL-003-S10 acceptance tests passed. |
| `AL-003-S10-EV13` | `git diff --check -- docs/delivery/roadmap.md docs/delivery/status.md docs/delivery/acceptance.md outputs/worklogs/index.md` | PASS after delivery closure updates. |

## Acceptance Coverage

| Acceptance ID | Coverage |
| --- | --- |
| `AL-003-S10-AC01` | Covered by `field_runtime_accepts_declared_scalar_profiles_only`: all six profile names parse as scalar, non-scalar kind is rejected with scalar-only runtime wording. |
| `AL-003-S10-AC02` | Covered by `field_profile_semantics_are_non_command_metadata` and `field_profiles_do_not_execute_direct_behavior`: profile semantics report no direct behavior and runtime negative controls preserve Energy, position, Genome id, Resource amount, and Material amount. |
| `AL-003-S10-AC03` | Covered by `canonical_test_world_resolves_resource_derived_material_synthesis_surface`: canonical test world exposes scalar-only Field runtime status, supported scalar profiles, and unsupported direct effects. |

## Notes

- Worklogs are evidence, not source of truth.
- The source of truth remains `docs/PRINCIPLES.md`, Canon docs, and delivery roadmap/status/acceptance files.
- This slice intentionally does not add vector Field storage, flow movement, radiation mutation, light-to-Energy conversion, pressure damage, or direct profile-name behavior.
