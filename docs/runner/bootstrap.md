---
tags: [alife, canon, area/runner, area/bootstrap, audience/agent]
---
# Bootstrap
> Deterministic application module that prepares the complete initial World before Tick 0.

## Role
Bootstrap belongs to the Runner subsystem conceptually and is implemented under `src/bootstrap/`.

Runner, CLI bootstrap mode, tests, batch tools, and future preview tools use one shared contract.

## Contract
```text
Resolved Scenario -> Bootstrap -> PreparedWorld
                             -> BootstrapManifest
                             -> Warnings
```
Bootstrap must not start Core or execute a Tick.

## Input
Bootstrap receives an immutable resolved Scenario with:
- effective root seed;
- World geometry and limits;
- Resource, Material, Field, and Genome definitions;
- initialization specification;
- validated identifiers, schema, and Scenario hash.

Adapter paths, HTTP payloads, and UI state are forbidden inputs.

## Output
`PreparedWorld` contains concrete Tick 0 data:
- Cells, positions, lifecycle, Energy, Resources, and Materials;
- assigned or instantiated Genomes;
- Resource and Field layers;
- optional Material fragments and Joints;
- normalized runtime configuration.

No unresolved generator instruction may remain in `PreparedWorld`.

## Pipeline
```text
Validate Input
  -> Derive Seed Domains
  -> Prepare Spatial Layers
  -> Place Cells
  -> Assign Cell State And Genomes
  -> Prepare Optional Entities
  -> Validate Cross-State Invariants
  -> Hash Prepared State
```

## Determinism
The same Scenario, root seed, and generator versions must produce the same `PreparedWorld`.

Independent seed domains are required for Cell placement, Cell properties, Genome variation, each Resource layer, each Field layer, and each optional entity class.

Changing one generator must not perturb unrelated generated data.

## Spatial Initialization
Each ResourceType and FieldType uses its own spatial layer.

Multiple Resource types may coexist at one coordinate.

Generators must be bounded, deterministic, versioned, and validated. Totals and ranges are recorded in the manifest.

## Cell And Genome Initialization
Cell placement respects World bounds, radius, capacity, and configured spacing.

Genome assignment uses known templates and deterministic variation.

Runtime mutation, copying, inheritance, and expression belong to Core mechanics.

## Validation
Bootstrap rejects:
- entities outside World bounds or prohibited overlaps;
- unknown ids or templates;
- negative or non-finite quantities;
- capacity violations;
- inconsistent layer dimensions;
- invalid totals or ranges.

## Manifest
`BootstrapManifest` contains Scenario hash, effective seed, Bootstrap and generator versions, entity counts, Resource totals, Field ranges, prepared-state hash, and warnings.

## Invariants
```text
Bootstrap prepares Tick 0 but executes no Tick.
PreparedWorld contains no unresolved generators.
Generation is deterministic and versioned.
Bootstrap contains no runtime world mechanics.
All adapters use the same contract.
Failure produces no partial active World.
```

## Semantic Links
- [[docs/runner/runner|Runner]]
- [[docs/runner/scenario-resolution|Scenario Resolution]]
- [[docs/runner/execution-modes|Execution Modes]]
- [[docs/runner/projections|Runner Projections]]
- [[docs/config/INDEX|Config Index]]
