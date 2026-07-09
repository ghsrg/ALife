---
tags:
  - alife
  - docs/index
  - area/observer
  - audience/agent
---

# Observer Index

> Agent-facing router for Observer Layer, projections, analytics and mechanism coverage.

## Before Work

1. Read [[docs/PRINCIPLES|Principles]].
2. Read [[docs/mechanics/observer-projection|Committed State -> Observer Projection]].
3. For analyzer or coverage work, read [[docs/observer/mechanism-coverage|Mechanism Coverage Contract]].
4. For UI/viewer work, read [[docs/observer/projection-contract|Projection Contract]] and [[docs/ui/INDEX|UI Index]].
5. Check implementation context in [[docs/implementation/INDEX|Implementation Index]].

## Observer Docs

- [[docs/observer/README|Observer README]] `#human-summary`
- [[docs/observer/observer-layer|Observer Layer]] `#contract` `#authority`
- [[docs/observer/mechanism-coverage|Mechanism Coverage Contract]] `#coverage` `#analyzer`
- [[docs/observer/behavior-profile-balance|Behavior Profile Balance]] `#balance` `#survival-style`
- [[docs/observer/classification-contract|Classification Contract]] `#classification` `#analytics`
- [[docs/observer/classification-registry|Classification Registry]] `#classification` `#registry`
- [[docs/observer/projection-contract|Projection Contract]] `#projection` `#ui`

### Observer Configs
- [classification-registry.toml](file:///c:/Users/korsr/PycharmProjects/ALife/config/observer/classification-registry.toml) `#config` `#registry`
- [cell-functional-role-classifier.toml](file:///c:/Users/korsr/PycharmProjects/ALife/config/observer/cell-functional-role-classifier.toml) `#config` `#classifier` `#role`
- [behavior-profile-classifier.toml](file:///c:/Users/korsr/PycharmProjects/ALife/config/observer/behavior-profile-classifier.toml) `#config` `#classifier` `#profile`
- [organism-archetype-classifier.toml](file:///c:/Users/korsr/PycharmProjects/ALife/config/observer/organism-archetype-classifier.toml) `#config` `#classifier` `#archetype`


## Related Canon

- [[docs/biology/organism|Organism View]]
- [[docs/evolution/population-dynamics|Population Dynamics]]
- [[docs/evolution/selection|Selection]]
- [[docs/evolution/species-like-clusters|Species-like Clusters]]
- [[docs/engine/storage|Storage]]
- [[docs/engine/rendering|Rendering]]

## Related Implementation

- [[docs/implementation/architecture|Architecture]]
- [[docs/implementation/mechanism-reachability|Mechanism Reachability]]
- [[docs/implementation/early-stability-tool|Early Stability Tool]]
- [[docs/implementation/implementation-plan-ui|UI Implementation Plan]]

## Classification Routing

- Classification rules and result schema -> [[docs/observer/classification-contract|Classification Contract]]
- Initial labels and priorities -> [[docs/observer/classification-registry|Classification Registry]]
- Survival-style balance conclusions -> [[docs/observer/behavior-profile-balance|Behavior Profile Balance]]
- UI display of labels -> [[docs/ui/exploration|UI Exploration]] + [[docs/ui/analytics|UI Analytics]]

## Core Rule

Observer reads committed simulation state and registries. It may produce projections, summaries, coverage reports and recommendations.

Observer must not mutate simulation state, feed hidden inputs into Core, or become a shortcut for behavior.
