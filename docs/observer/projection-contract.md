---
tags:
  - alife
  - observer
  - projection
  - ui
---

# Projection Contract

Projection is a read-only representation derived from committed Core state and Observer analytics.

Projection is not `WorldState`, not a mutable cache, and not an input channel back into simulation behavior.

## Projection Envelope

Every projection payload should include:

```text
schema_version
projection_kind
run_id
tick
config_hash
engine_version
source
completeness
generated_at
```

`completeness` should distinguish:

```text
full
bounded
sampled
partial
debug_selected
```

## Projection Kinds

Initial planned kinds:

```text
FrameProjection
EntityProjection
InspectorProjection
MetricsProjection
OrganismViewProjection
LineageProjection
CoverageProjection
BehaviorProfileProjection
BalanceFindingProjection
ClassificationProjection
DebugTraceProjection
```

UI may render these projections. It must not infer authority beyond them.

## Live Frame Rules

Live viewer frames should be bounded:

```text
viewport or sampled extent
cell draw data
resource/field summaries
event highlights
optional selected entity details
```

Full world export is an explicit debug/research mode, not the default viewer path.

## Coverage Projection

Coverage projection exposes analyzer results to UI or reports:

```text
mechanism_count
covered_count
partial_count
missing_count
warning_codes
recommended_reruns
candidate_configs
source_report
```

It mirrors files produced by [[docs/observer/mechanism-coverage|Mechanism Coverage Contract]].

## Behavior Balance Projection

Behavior balance projection exposes observer-only survival style findings:

```text
profile_labels
compared_profiles
equal_requirements
result
evidence_metrics
dominance_rate
recommendation
source_report
```

It mirrors [[docs/observer/behavior-profile-balance|Behavior Profile Balance]] outputs.

## Classification Projection

Classification projection exposes observer-only labels with evidence and confidence:

```text
classification_id
dimension_id
entity_type
entity_id
tick_start
tick_end
mode
primary_label
secondary_labels
confidence
classification_status
evidence_summary
registry_version
classifier_version
source_report
```

It mirrors [[docs/observer/classification-contract|Classification Contract]] and [[docs/observer/classification-registry|Classification Registry]].

## Command Boundary

UI commands are not Observer projections.

Any future command path must use explicit approved command APIs and must not mutate the active run by editing projection data.

## Semantic Links

- base contract: [[docs/observer/observer-layer|Observer Layer]]
- UI architecture: [[docs/ui/architecture|UI Architecture]]
- visualization: [[docs/ui/visualization|UI Visualization]]
- rendering: [[docs/engine/rendering|Rendering]]
- storage: [[docs/engine/storage|Storage]]
- classification: [[docs/observer/classification-contract|Classification Contract]]
