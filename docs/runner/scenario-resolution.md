---
tags: [alife, canon, area/runner, audience/agent]
---
# Scenario Resolution
> Converts adapter-specific Scenario input into one normalized document for validation and Bootstrap.
## Inputs
Runner may receive a Scenario from:
- local filesystem path;
- registered Scenario id;
- inline Scenario document;
- persisted prepared-state or checkpoint reference; future-compatible.

Adapters resolve transport-specific input before shared Runner logic.

```text
CLI path -----> File Adapter -----\
Scenario id --> Registry Adapter ---> ScenarioDocument
UI document --> HTTP Adapter -----/
```

## Normalized Contract
`ScenarioDocument` is the transport-independent input to validation and Bootstrap.

It contains:
- schema version;
- Scenario identity and metadata;
- world and simulation configuration;
- type registries and references;
- initialization specification;
- run options defined by Scenario;
- canonical serialized representation.

Paths, URLs, request bodies, and UI state must not remain inside `ScenarioDocument`.

## Resolution Pipeline
```text
Source -> Load -> Parse -> Normalize -> Resolve References
       -> Validate -> Canonicalize -> Hash -> ScenarioDocument
```

Resolution must not generate Cells, Resource maps, Fields, or Genome instances.

## Overrides
Command overrides take precedence over Scenario defaults only for fields allowed by the command contract.

Initial supported override:
- root seed.

The effective value must be recorded in run metadata without silently rewriting the source document.

Unknown or forbidden overrides must be rejected.

## References
All referenced ids must resolve against normalized Scenario registries.

Unknown, duplicate, ambiguous, or cyclic references are errors unless another Canon explicitly permits them.

Resolution order must be deterministic and independent of map iteration order.

## Canonicalization And Hash
The Scenario hash is computed from the canonical normalized document before Bootstrap.

The hash excludes:
- filesystem location;
- transport metadata;
- UI presentation state;
- request id.

It includes every value that can affect Bootstrap or simulation behavior.

## Immutability
Each run receives an immutable resolved Scenario snapshot.

Later file edits, registry changes, or UI edits must not alter an active or prepared run.

## Errors
Stable categories:
```text
source_not_found
parse_error
unsupported_schema
invalid_reference
invalid_override
validation_error
```

## Invariants
```text
All adapters produce the same ScenarioDocument contract.
Resolution does not create World state.
Scenario hash precedes Bootstrap.
Behavior-affecting values are included in the hash.
Active runs use immutable resolved Scenarios.
```

## Semantic Links
- [[docs/runner/runner|Runner]]
- [[docs/runner/command-contract|Command Contract]]
- [[docs/runner/bootstrap|Bootstrap]]
- [[docs/config/INDEX|Config Index]]
