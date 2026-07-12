---
tags:
  - alife
  - worklog/report
  - phase-2
  - chemistry
  - joints
---

# Phase 2 Roadmap Chemistry And Joints Sync Report

## Scope

Synchronized the Phase 2 global roadmap, Phase 3/4 global roadmaps, and canonical implementation phase roadmap after identifying missing Chemistry/Matter Dynamics and persistent Joint prerequisites.

## Findings

- Existing Canon already requires typed Resources, Materials, Reactions, MaterialFragments, local Heat, material-derived Boundary behavior, and persistent Joints.
- Current implementation contains Phase 2F transient contact primitives, but not a ReactionRegistry, typed material/resource registries, MaterialFragmentStore, or JointStore.
- The former roadmap deferred full chemistry without assigning a successor phase and left persistent Joints inconsistent between Phase 2F and later Phase 4 planning.
- Genome Runtime must regulate existing registered mechanisms. It cannot be the phase that invents chemistry, repair, or persistent interaction structures.

## Documentation Changes

- Added Phase 2G, `Chemistry And Matter Dynamics`, to the Phase 2 global roadmap.
- Added Phase 2H, `Persistent Interaction Structures`, to the Phase 2 global roadmap.
- Defined domain boundaries, build scope, invariants, acceptance gates and Rust reachability requirements for both subphases.
- Updated the Phase 3 roadmap: Genome regulates registered controlled reactions, repair and Joint intents, without creating new physical mechanisms.
- Updated the Phase 4 roadmap: it observes, calibrates and analyzes chemistry and persistent multicellular structures rather than introducing missing Joint behavior.
- Updated `docs/implementation/implementation-phases.md` and `docs/ROADMAP.md` with the same ordering and responsibility split.
- Marked Phase 4 as the shared Observer/research layer rather than a second implementation of Core mechanics.
- Fixed the remaining Phase 4/Phase 5 overlap: Phase 5 now owns long-run evolution experiments.

## Verification

```text
git diff --check: passed
roadmap search: no current roadmap statement assigns Joint implementation to Phase 4
roadmap search: full chemical reaction network is explicitly assigned to Phase 2G
historical Phase 4 wording: marked as superseded instead of rewritten
```

## Next Step

Create separate detailed TDD implementation plans for Phase 2G and Phase 2H. Each plan must use the relevant mechanics pre-flight cards and retain deterministic delta/commit accounting boundaries.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
