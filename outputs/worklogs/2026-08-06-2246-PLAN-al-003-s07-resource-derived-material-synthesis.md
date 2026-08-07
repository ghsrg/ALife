---
plan_id: AL-003-S07
status: proposed
date: 2026-08-06
scope: Resource-Derived Material Synthesis
---

# AL-003-S07 Resource-Derived Material Synthesis

## Goal

Replace the current fixed Cell material-role inventory as the material-synthesis result with deterministic `MaterialInstance` records derived from configured precursor Resources. A synthesis reaction must debit local Resources and Energy only after feasibility succeeds, create material with explicit profile/provenance, emit configured Heat and waste, and preserve accounting through degradation and fragments.

`AL-003-S08` owns physical nucleotide requirements for Genome copying/recombination. `AL-003-S09` owns active local Field grids. This slice may parse their manifest declarations for the canonical test scenario, but must not execute either behavior.

## Canonical Design

1. Add a config and Core value type `MaterialProfile` with six scalar axes: `volume`, `stability`, `strength`, `energy_capacity`, `permeability`, and `durability`. Runtime converts `durability` to the existing decay representation only at the boundary; it must not use an inverted, unnamed coefficient in configuration.
2. Add a bounded `MaterialCapabilityProfile` to Resources and MaterialInstances. Each capability contribution is an explicit coefficient keyed by the existing canonical capability identifiers. The output value is the volume-weighted sum of consumed precursor contributions, clamped to the declared domain. This is necessary because the six physical axes do not determine synthesis, copying, transport, or recombination capability.
3. Add `MaterialInstanceId`, `MaterialInstance`, and Cell-owned compositional material inventory. An instance contains its derived physical profile, capability profile, current amount/state, and deterministic recipe/provenance reference. It is not a static `MaterialTypeId` with a role slot.
4. Extend controlled reaction configuration with a material-output recipe: ordered precursor requirements, output amount rule, profile derivation rule, Energy/Heat/waste terms, and optional allowed output tags. Parser validation rejects unknown resources, duplicate recipe keys, negative amounts, profiles outside bounds, and undeclared waste outputs.
5. Replace the material-synthesis execution shortcut with Feasibility plus an atomic debit/create transaction. Every reject preserves Resource, Energy, material, and fragment state. Every success produces a single deterministic accounting event.
6. Update degradation to create a profile-preserving `MaterialFragment`. A later explicit reaction may convert a fragment to Resources; degradation itself cannot mint Resources.
7. Migrate `config/scenarios/demo/canonical_living_world.toml` away from static role-material synthesis and add `config/scenarios/demo/canonical_test_world.toml`. The latter defines the agreed precursor catalogue, synthesis recipes, passive resource decay, metabolism by-products, material degradation, nucleotide-like precursor declarations, and bounded Field manifests.

## Compatibility And Cutover

- Remove the fixed role-slot inventory from synthesis, capacity, damage, decomposition, initialization, stable hashing, and observer projections together. Keeping a hidden slot-to-instance bridge would preserve the shortcut and invalidate the slice goal.
- Static configured `MaterialType` entries may remain only as immutable recipe/template definitions while any caller that represents cell-owned matter uses `MaterialInstance`.
- Existing non-synthesis processes must query aggregate material capability and physical properties from instance inventory. They must not select an implicit boundary/structure/storage role.
- No species IDs, cell classes, resource-specific process shortcuts, direct Energy creation, or special Genome material are introduced.

## Test-Driven Steps

1. Add failing Core tests in `tests/phase3f_resource_material_synthesis.rs` for deterministic volume-weighted physical/capability derivation, declaration-order invariance, and stable-hash/replay consistency.
2. Add failing transaction tests for success accounting, missing precursor, insufficient Energy, full capacity, invalid recipe, and explicit Heat/waste outputs. Assert rejected actions change no state.
3. Add failing lifecycle tests proving degradation creates profile-preserving fragments and Resource recovery occurs only through a configured conversion reaction.
4. Add failing parser/config tests for resource profile bounds, capability-profile keys, material-output recipes, and invalid references.
5. Implement the smallest Core inventory, registry, parser, feasibility, execution, degradation, fragment, hashing, and projection changes needed to make steps 1–4 pass.
6. Add `canonical_test_world.toml` and `tests/phase3f_canonical_test_world.rs`; prove it resolves and that the declared resource/material/reaction/decay/fragment/manifest surface is accepted without role-material aliases.
7. Run focused Core, parser, scenario-loader, determinism, accounting, and full workspace checks. Record command results in the implementation report before moving `AL-003-S07` to done.

## Acceptance Mapping

| Acceptance | Test evidence |
| --- | --- |
| `AL-003-S07-AC01` | Deterministic derivation and transaction-accounting tests. |
| `AL-003-S07-AC02` | Feasibility, degradation, fragment, and explicit conversion tests. |
| `AL-003-S07-AC03` | Canonical test scenario loader/resolution test. |

## Risks And Decisions

- Full cutover touches Cell storage, tick execution, parser, fragments, stable hashes, observer contracts, and scenario fixtures. This is medium confidence until compile-driven inventory migration is complete.
- Capability coefficients are explicit data rather than inferred from material names or hard-coded roles. This is the only design here that keeps both physical material composition and the process registry coherent.
- The proposed initial canonical scenario has declarative nucleotide precursors and Field manifests only. Executing those semantics before `S08`/`S09` would conflate delivery acceptance and make failure accounting unclear.

## Execution Gate

Implementation has not started. Execute this plan only after the explicit approval: `OK EXECUTE AL-003-S07`.
