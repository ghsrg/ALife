# AL-003-S02 Genome Runtime Contract And Registered Output Coverage Report

Plan ID: `AL-003-S02`
Status: `done`
Date: 2026-07-20

## Scope

Implemented the Genome Runtime contract boundary and registered output coverage
slice. This report covers contract/disposition/debug-trace work only.

Out of scope by design:

- mutation;
- inheritance;
- lineage replay;
- scheduler cadence closure for `AL-003-S03`;
- silent enabling of deferred canon outputs.

## Implemented Changes

- Added explicit Genome output dispositions in `src/core/genome.rs`.
- Kept the existing six Phase 3A outputs enabled.
- Marked additional canon `status: now` output names as deferred unless their
  execution path is already defensible.
- Kept unknown/non-registry output names rejected as unsupported.
- Added a minimal normalized local `GenomeRuntimeInputs` value object.
- Added capability mask checks through existing `ProcessSpec` capability
  requirements.
- Added read-only `GenomeRuntimeTrace` and routed it into
  `ProcessDiagnostics`.
- Emitted a debug trace when a Genome ActionPlan refresh occurs.
- Preserved Feasibility as the authority for execution/rejection.
- Applied mechanical clippy fixes required by the workspace gate:
  `contains(&0)`, `.is_multiple_of(...)`, and a boolean `assert!`.

## Acceptance Coverage

| Acceptance ID | Result | Evidence |
| --- | --- | --- |
| `AL-003-S02-AC01` | pass | `GenomeRuntimeInputs` is local, normalized, and contains no global/observer/organism/species inputs. |
| `AL-003-S02-AC02` | pass | `GenomeOutputDisposition` covers current canon outputs and rejects unsupported names. |
| `AL-003-S02-AC03` | pass | Capability masks use existing `ProcessSpec` required capabilities and do not bypass Feasibility. |
| `AL-003-S02-AC04` | pass | `GenomeRuntimeTrace` records tick, cell id, inputs, outputs, action plan, and a read-only feasibility summary. |
| `AL-003-S02-AC05` | pass | Scheduler cadence tests remain regression-only; `AL-003-S03` stays planned. |

## Verification

```text
cargo fmt --check
exit 0
```

```text
cargo test --test phase3b_runtime_contract
5 passed; 0 failed
```

```text
cargo test --test phase3a_genome_bootstrap
11 passed; 0 failed
```

```text
cargo test --test phase3a_genome_config
4 passed; 0 failed
```

```text
cargo test --test phase3a_action_plan
3 passed; 0 failed
```

```text
cargo test --test phase3a_tick_integration
4 passed; 0 failed
```

```text
cargo test --test phase2_process_registry
7 passed; 0 failed
```

```text
cargo test --test phase2_process_smoke
6 passed; 0 failed
```

```text
cargo test --test scheduler_genome_cadence
3 passed; 0 failed
```

```text
cargo clippy --workspace --all-targets -- -D warnings
exit 0
```

## Notes

- `AL-003-S03` should be the next Genome slice if continuing Phase 3 in order.
- Deferred outputs remain explicit by design:
  `resource_export_priority`, `signal_emit_priority`,
  `genome_copying_priority`, `division_partition_priority`, `dormancy_bias`,
  and `internal_rebalance_priority`.
- The runtime trace currently records `not_evaluated` as the feasibility summary
  at ActionPlan refresh time because detailed process rejections are collected
  later by existing `ProcessDiagnostics`.
