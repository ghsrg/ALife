---
tags:
  - alife
  - docs/index
  - audience/agent
---

# Agent Index

> Compact navigation map for agents. For human reading order use [[docs/README|Docs README]].

## Required First Reads

- [[docs/PRINCIPLES|Principles]] `#principles` `#canon/root` `#constraints`
- [[docs/GLOSSARY|Glossary]] `#terms` `#vocabulary`
- [[docs/ROADMAP|Roadmap]] `#status` `#current-state`
- [[docs/STYLE_GUIDE|Style Guide]] `#docs` `#writing`
- [[outputs/worklogs/index|Worklogs]] `#plans` `#reports`

## Source-Of-Truth Layers

- [[docs/PRINCIPLES|Principles]] `#highest-authority`
- Canon: `world/`, `biology/`, `genetics/`, `evolution/`, `config/`, `engine/`, `ui/`
- [[docs/implementation/README|Implementation]] `#implementation-plan`
- [[docs/decisions/README|ADR Index]] `#adr`
- `research/` `#hypotheses` `#future-options`
- `examples/` `#examples` `#non-authoritative`

## World

- [[docs/world/laws|World Laws]] `#world` `#laws`
- [[docs/world/philosophy|World Philosophy]] `#world` `#principles`
- [[docs/world/tick|Tick]] `#time` `#tick`
- [[docs/world/tick-semantics|Tick Semantics]] `#scheduler` `#determinism`
- [[docs/world/space|Space]] `#space` `#locality`
- [[docs/world/units|Units]] `#units`
- [[docs/world/fields|Fields]] `#fields`
- [[docs/world/field-semantics|Field Semantics]] `#fields` `#semantics`
- [[docs/world/resources|Resources]] `#resources`
- [[docs/world/reactions|Reactions]] `#chemistry`
- [[docs/world/materials|Materials]] `#materials`
- [[docs/world/energy|Energy]] `#energy`
- [[docs/world/physics|Physics]] `#physics`

## Biology

- [[docs/biology/cell|Cell]] `#cell`
- [[docs/biology/cell-state|Cell State]] `#runtime-state`
- [[docs/biology/membrane|Membrane]] `#boundary`
- [[docs/biology/processes|Processes]] `#processes`
- [[docs/biology/action-process-registry|Action Process Registry]] `#process-registry`
- [[docs/biology/process-capabilities|Process Capabilities]] `#capabilities`
- [[docs/biology/process-progress|Process Progress]] `#long-running`
- [[docs/biology/feasibility|Feasibility]] `#feasibility`
- [[docs/biology/lifecycle|Lifecycle]] `#lifecycle`
- [[docs/biology/division-partition|Division Partition]] `#division`
- [[docs/biology/joint|Joint]] `#joint`
- [[docs/biology/communication|Communication]] `#signal`
- [[docs/biology/organism|Organism View]] `#observer` `#organism-view`
- [[docs/biology/specialization|Specialization]] `#specialization`
- [[docs/biology/genome|Genome Carrier]] `#genome` `#carrier`

## Genetics

- [[docs/genetics/genome-representation|Genome Representation]] `#genome` `#representation`
- [[docs/genetics/genome-runtime|Genome Runtime]] `#genome-runtime`
- [[docs/genetics/regulatory-interface|Regulatory Interface]] `#regulation`
- [[docs/genetics/regulatory-network|Regulatory Network]] `#direct-regulatory-graph`
- [[docs/genetics/heredity|Heredity]] `#heredity`
- [[docs/genetics/inheritance|Inheritance]] `#inheritance`
- [[docs/genetics/mutation|Mutation]] `#mutation`
- [[docs/genetics/recombination|Recombination]] `#recombination`
- [[docs/genetics/horizontal-transfer|Horizontal Transfer]] `#hgt`
- [[docs/genetics/epigenetics|Epigenetics]] `#epigenetics`

## Evolution

- [[docs/evolution/adaptation|Adaptation]] `#adaptation` `#observer-only`
- [[docs/evolution/population-dynamics|Population Dynamics]] `#population`
- [[docs/evolution/selection|Selection]] `#selection` `#observer-interpretation`
- [[docs/evolution/species-like-clusters|Species-like Clusters]] `#clusters` `#observer-only`

## Config

- [[docs/config/world_config|World Config]] `#config` `#world`
- [[docs/config/resources_config|Resources Config]] `#config` `#resources`
- [[docs/config/materials_config|Materials Config]] `#config` `#materials`
- [[docs/config/reactions_config|Reactions Config]] `#config` `#reactions`
- [[docs/config/fields_config|Fields Config]] `#config` `#fields`
- [[docs/config/stability_bounds|Stability Bounds]] `#stability` `#bounds`

## Engine

- [[docs/engine/technology-stack|Technology Stack]] `#rust` `#webgl2` `#storage`
- [[docs/engine/performance|Performance]] `#performance`
- [[docs/engine/scheduler|Scheduler]] `#scheduler`
- [[docs/engine/storage|Storage]] `#storage`
- [[docs/engine/serialization|Serialization]] `#serialization`
- [[docs/engine/rendering|Rendering]] `#viewer` `#rendering`
- [[docs/engine/physics|Engine Physics]] `#physics-engine`
- [[docs/engine/chemistry|Engine Chemistry]] `#chemistry-engine`
- [[docs/engine/ecs|ECS]] `#soa` `#ecs`

## UI

- [[docs/ui/README|UI Layer]] `#ui` `#canon`
- [[docs/ui/principles|UI Principles]] `#ui` `#principles`
- [[docs/ui/architecture|UI Architecture]] `#ui` `#architecture`
- [[docs/ui/navigation|UI Navigation]] `#ui` `#navigation`
- [[docs/ui/visualization|UI Visualization]] `#ui` `#viewer` `#webgl2`
- [[docs/ui/analytics|UI Analytics]] `#ui` `#analytics`
- [[docs/ui/exploration|UI Exploration]] `#ui` `#inspector` `#selection`
- [[docs/ui/presentation|UI Presentation]] `#ui` `#theme` `#localization`
- [[docs/ui/interaction|UI Interaction]] `#ui` `#commands`
- [[docs/ui/quality|UI Quality]] `#ui` `#quality`

## Implementation

- [[docs/implementation/README|Implementation Index]] `#implementation`
- [[docs/implementation/architecture|Architecture]] `#architecture`
- [[docs/implementation/implementation-phases|Implementation Phases]] `#phases`
- [[docs/implementation/phase-1-design|Phase 1 Design]] `#phase1`
- [[docs/implementation/phase-1-data-model|Phase 1 Data Model]] `#rust` `#data-model`
- [[docs/implementation/phase-1-module-api|Phase 1 Module API]] `#api`
- [[docs/implementation/optimization-paths|Optimization Paths]] `#optimization`
- [[docs/implementation/early-stability-tool|Early Stability Tool]] `#tools` `#stability`
- [[docs/implementation/early-stability-parameter-tuning|Early Stability Parameter Tuning]] `#tuning`
- [[docs/implementation/mechanism-reachability|Mechanism Reachability]] `#reachability`
- [[docs/implementation/implementation-plan-ui|UI Implementation Plan]] `#ui` `#implementation`

## Decisions

- [[docs/decisions/README|ADR Index]] `#adr`
- [[docs/decisions/ADR-0001-tech-stack|ADR-0001 Technology Stack]] `#tech-stack`

## Examples

- [[docs/examples/README|Examples Index]] `#examples`
- [[docs/examples/biology-examples|Biology Examples]] `#biology`
- [[docs/examples/config-examples|Config Examples]] `#config`
- [[docs/examples/engine-examples|Engine Examples]] `#engine`
- [[docs/examples/genetics-examples|Genetics Examples]] `#genetics`

## Research

- [[docs/research/genome-representation-options|Genome Representation Options]] `#research` `#genome`
- [[docs/research/graph-recombination-options|Graph Recombination Options]] `#research` `#recombination`
- [[docs/research/mobile-genetic-elements|Mobile Genetic Elements]] `#research` `#hgt`
- [[docs/research/reproduction-strategy-options|Reproduction Strategy Options]] `#research` `#reproduction`
- [[docs/research/rejected-ideas|Rejected Ideas]] `#research` `#rejected`

## Work Process

- Before editing Canon: read [[docs/PRINCIPLES|Principles]], [[docs/GLOSSARY|Glossary]], relevant Canon, and relevant ADR.
- Before implementation: read [[docs/implementation/README|Implementation]], relevant phase plan, and related engine/config docs.
- Before UI work: read [[docs/ui/README|UI Layer]], [[docs/ui/principles|UI Principles]], [[docs/ui/architecture|UI Architecture]], [[docs/implementation/implementation-plan-ui|UI Implementation Plan]], and [[docs/engine/technology-stack|Technology Stack]].
- Save plans and reports in [[outputs/worklogs/index|Worklogs]].

## Semantic Links

- human entry: [[docs/README|Docs README]]
- governed by: [[docs/PRINCIPLES|Principles]]
- terms: [[docs/GLOSSARY|Glossary]]
- status: [[docs/ROADMAP|Roadmap]]
- implementation: [[docs/implementation/README|Implementation]]
- UI entry: [[docs/ui/README|UI Layer]]
- worklogs: [[outputs/worklogs/index|Worklogs]]
