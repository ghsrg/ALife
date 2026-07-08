---
tags:
  - alife
  - observer
  - classification
  - analytics
---

# Observer Classification Contract

> Базовий контракт для observer-side класифікацій у `ALife`.
>
> Документ визначає, як із committed simulation data будуються пояснювані аналітичні labels для Cells, OrganismView, lineages, populations та інших derived views.
>
> Цей документ не створює нових simulation entities, не змінює behavior і не має пріоритету над `PRINCIPLES.md`, Biology Canon, Evolution Canon, UI Canon або accepted ADR.

---

# 1. Призначення

Observer classification потрібна для того, щоб описувати emergent patterns у термінах, зрозумілих людині та придатних для:

- debug;
- scientific analysis;
- UI exploration;
- filtering;
- comparison;
- discovery detection;
- lineage and population analysis;
- experiment reports;
- balance analysis;
- export and reproducibility.

Приклади observer-side висновків:

```text
Cell C-17:
  primary Functional Role = transport-like
  secondary Functional Role = boundary-supporting
  confidence = 0.81

OrganismView O-203:
  Organization Archetype = resource-sharing colony-like
  Survival Profile = scarcity-adapted
  confidence = 0.74

Lineage L-9:
  Behavior Profile = opportunistic-growth
  environment context = Resource abundance
  confidence = 0.68
```

Observer classification описує спостережуваний патерн.

Вона не означає, що:

- Cell знає свій тип;
- Genome містить готову роль;
- OrganismView керує клітинами;
- label є частиною simulation state;
- label автоматично доводить біологічну функцію;
- label є fitness input;
- label є species id;
- label є безумовно істинною природною таксономією.

---

# 2. Authority And Boundary

## 2.1 Authority

При конфлікті пріоритет мають:

```text
PRINCIPLES
  -> relevant Biology / World / Genetics / Evolution Canon
  -> accepted ADR
  -> Observer Classification Contract
  -> Classification Registry
  -> classifier implementation
  -> UI presentation
```

## 2.2 Observer Boundary

Canonical flow:

```text
committed simulation state
+ committed events
+ versioned summaries
+ explicit analytics windows
        ↓
observer projection
        ↓
feature extraction
        ↓
classification
        ↓
label + confidence + evidence + provenance
```

Зворотний flow заборонений:

```text
classification label
    -X-> Genome Runtime
    -X-> Feasibility
    -X-> Process selection
    -X-> Cell action priority
    -X-> mutation
    -X-> compatibility
    -X-> reproduction
    -X-> simulation physics
```

## 2.3 Invariant

```text
Classification describes.
Classification does not command.
```

---

# 3. Scope

Цей контракт поширюється на:

```text
Cell Functional Role
Sensory Specialization
Behavior Profile
Survival Profile
Organization Archetype
Morphology Descriptor
Dependency Profile
Lineage / population analytical labels
Species-like cluster metadata
Warning / discovery classifiers
```

Контракт може застосовуватися до:

```text
Cell
OrganismView
Lineage
Genome variant
Population subset
Selection Set
Run
Run interval
Species-like cluster
```

Не кожна classification dimension застосовна до кожного entity type.

---

# 4. Core Terms

## 4.1 Classification Dimension

`Classification Dimension` — незалежна вісь observer analysis.

Приклади:

```text
Functional Cell Role
Sensory Specialization
Behavior Profile
Organization Archetype
Morphology
Dependency
Ecological Niche
```

Одна entity може мати labels у кількох dimensions одночасно.

## 4.2 Label

`Label` — versioned observer-side категорія з явними evidence rules.

Label має:

```text
canonical id
human-readable description
applicable entity types
positive evidence
negative evidence
required data
minimum observation window
benefit metrics
cost metrics
confidence rules
visual encoding
classifier version
```

## 4.3 Classifier

`Classifier` — deterministic або explicitly stochastic analytics component, який:

```text
reads bounded observer data
extracts features
evaluates registered labels
returns classification result
```

Classifier не є simulation system.

## 4.4 Feature

`Feature` — нормалізована observer metric, derived from committed data.

Приклади:

```text
fraction of Process executions belonging to Metabolism
Transport Material fraction
Resource uptake success rate
damage repaired per Energy spent
dormant fraction
joint density
resource transfer dependency
fragmentation reproduction frequency
```

## 4.5 Evidence

`Evidence` — конкретне значення, event, trend або relation, що підтримує чи заперечує label.

Evidence повинно бути explainable.

## 4.6 Potential Classification

`Potential` описує capability або структурну можливість.

Приклад:

```text
Cell has Transport-capable Material
+ valid ActiveUptake process path
→ Potential Functional Role: transport-like
```

Potential не доводить, що функція реально виконувалася.

## 4.7 Observed Classification

`Observed` описує стабільний фактичний патерн у заданому interval.

Приклад:

```text
ActiveUptake repeatedly executed
+ measurable Resource capture
+ non-trivial Energy cost
→ Observed Functional Role: transport-like
```

## 4.8 Behavior Profile

`Behavior Profile` — стійкий патерн дій, витрат, результатів і environmental response.

Behavior Profile:

- не є hardcoded strategy;
- не є Action;
- не є одним Process;
- не визначається одним Tick;
- може змінюватися між intervals;
- може бути multi-label.

## 4.9 Organization Archetype

`Organization Archetype` — observer-side опис структури `Cell + Joint`, dependency і coordination patterns.

Archetype не є body plan і не є класом Organism у Core.

---

# 5. Classification Result Schema

Мінімальний результат:

```text
ClassificationResult
├── classification_id
├── dimension_id
├── entity_type
├── entity_id
├── run_id
├── branch_id optional
├── tick_start
├── tick_end
├── classifier_id
├── classifier_version
├── registry_version
├── mode
├── primary_label optional
├── secondary_labels[]
├── label_scores[]
├── confidence
├── classification_status
├── evidence[]
├── counter_evidence[]
├── data_completeness
├── sampling
├── aggregation
├── feature_set_version
├── threshold_profile
├── environment_context
├── created_at
└── provenance
```

## 5.1 Mode

```text
potential
observed
combined
```

`combined` дозволяється лише тоді, коли UI та report окремо показують Potential і Observed evidence.

## 5.2 Classification Status

Canonical statuses:

```text
classified
mixed
unknown
insufficient_data
unstable
not_applicable
classifier_failed
incompatible_version
```

Meaning:

### classified

Один label має достатній score, confidence і dominance margin.

### mixed

Кілька labels мають близькі score або entity стабільно виконує кілька функцій.

### unknown

Даних достатньо, але жоден label не досяг threshold.

### insufficient_data

Немає достатнього observation window, required metrics або data completeness.

### unstable

Профіль суттєво змінювався всередині interval і не має достатньої persistence.

### not_applicable

Dimension не застосовується до entity type або selected context.

### classifier_failed

Помилка обчислення, schema mismatch або invalid numeric input.

### incompatible_version

Projection, feature schema або registry version несумісні з classifier.

---

# 6. Label Definition Schema

Кожен label у registry повинен мати:

```text
ClassificationLabelDefinition
├── id
├── dimension
├── title
├── description
├── biological_analogy optional
├── applicable_to[]
├── mode_support[]
├── required_features[]
├── optional_features[]
├── positive_evidence[]
├── counter_evidence[]
├── exclusion_rules[]
├── minimum_window
├── persistence_requirement
├── benefit_metrics[]
├── cost_metrics[]
├── tradeoff_expectations[]
├── environment_dependencies[]
├── confidence_weights
├── default_thresholds
├── visual_encoding
├── introduced_in_version
├── deprecated_in_version optional
└── notes
```

## 6.1 Biological Analogy

Biological analogy використовується лише для пояснення.

Приклад:

```text
transport-like
  analogy:
    membrane transporter-rich cell,
    absorptive surface,
    exchange-specialized tissue
```

Analogy не означає буквальну наявність органу, тканини або земного біологічного типу.

## 6.2 Exclusion Rules

Label не може бути присвоєний, якщо:

- required mechanism disabled;
- required data absent;
- observation window too short;
- evidence походить лише з одного випадкового event;
- entity була dead протягом більшості interval, якщо label описує active function;
- показник виник тільки через external intervention, а classifier не врахував intervention context;
- result dominated by missing data;
- label суперечить Canon.

---

# 7. Evidence Model

## 7.1 Evidence Categories

Evidence ділиться на:

```text
composition evidence
capability evidence
process evidence
flow evidence
state evidence
structural evidence
interaction evidence
lifecycle evidence
outcome evidence
environment evidence
historical evidence
comparative evidence
```

### Composition Evidence

```text
Material fractions
Resource stores
Boundary composition
Joint material composition
Genome carrier composition
```

### Capability Evidence

```text
registered capability present
capability strength
enabled Processes
required input availability
```

### Process Evidence

```text
candidate count
accepted count
execution count
rejection reasons
Process time fraction
Process output
Process cost
```

### Flow Evidence

```text
Resource uptake/export
Resource transfer
Energy production/spending
Material synthesis/degradation
Heat flow
Signal flow
```

### State Evidence

```text
Energy level
damage
dormancy
stored signal
fatigue
capacity utilization
lifecycle state
```

### Structural Evidence

```text
cell_count
joint_count
connectedness
geometry
component stability
fragmentation
merge history
dependency
```

### Interaction Evidence

```text
contacts
joint usage
signal exchange
resource sharing
competition
co-location
collective response
```

### Lifecycle Evidence

```text
birth
division
death
decomposition
dormancy entry/exit
age
generation
offspring survival
```

### Outcome Evidence

```text
survival time
division success
resource efficiency
damage avoided
damage repaired
distance reached
Resource patch reached
offspring viability
```

### Environment Evidence

```text
Resource abundance/scarcity
Resource pulses
spatial gradients
Heat
Pressure
hazards
competition
local density
```

## 7.2 Positive And Negative Evidence

Evidence може мати polarity:

```text
positive
negative
neutral/context
```

Приклад:

```text
storage-like:

positive:
  high storage Material fraction
  high retained Resource during scarcity
  high capacity utilization without overflow

negative:
  storage Material present but never used
  no retained Resource benefit
  all apparent benefit explained by passive income
```

## 7.3 Evidence Record

```text
EvidenceRecord
├── evidence_id
├── feature_id
├── observed_value
├── normalized_value
├── unit
├── expected_direction
├── weight
├── polarity
├── tick_range
├── source_projection
├── source_metric
├── data_state
└── explanation
```

---

# 8. Time And Persistence

## 8.1 No Single-Tick Classification

Stable role, profile або archetype не визначаються за одним Tick.

Винятки:

- instantaneous warning;
- explicit event classification;
- `Potential` capability classification;
- debug-only current-state preview.

## 8.2 Observation Windows

Starting canonical windows:

```text
short:
  100 ticks

medium:
  1_000 ticks

long:
  10_000 ticks

whole_run:
  explicit
```

Exact values are versioned analytics configuration.

Window selection залежить від:

- expected Process cadence;
- entity lifetime;
- simulation rate;
- event frequency;
- mechanism activation time;
- selected analysis purpose.

## 8.3 Persistence

```text
persistence =
  fraction of valid subwindows
  in which label score remains above persistence threshold
```

Initial recommendation:

```text
stable role:
  persistence >= 0.60

strong stable role:
  persistence >= 0.80
```

Ці значення є starting defaults, а не universal biological constants.

## 8.4 Window Eligibility

Window невалідне для classification, якщо:

```text
valid_data_fraction < configured minimum
entity absent for most of window
required metrics unavailable
projection version changed incompatibly
sampling cannot support the requested feature
```

---

# 9. Feature Normalization

Classifier не повинен напряму порівнювати несумісні raw units.

Allowed normalization:

```text
fraction of total
per tick
per active tick
per execution
per unit Material
per unit Energy
per unit Resource
per Cell
per OrganismView
per area
relative to local environment
relative to control run
z-score within explicit comparison set
bounded min-max using versioned bounds
```

Normalization metadata має бути частиною provenance.

Заборонено:

- прихований min-max по поточному screen subset;
- зміна normalization між views без indication;
- порівняння labels, розрахованих різними feature versions, як еквівалентних;
- використання future data для live classification без позначення.

---

# 10. Potential vs Observed

## 10.1 Potential Role

Potential classification спирається на:

```text
Material capabilities
available Process definitions
required structural context
reachable Action path
required sensing basis
```

Potential score не повинен використовувати actual benefit як обов'язкову умову.

## 10.2 Observed Role

Observed classification вимагає:

```text
mechanism activation
repeated execution or persistent state
measurable output
measurable cost or resource use
sufficient observation window
```

## 10.3 Potential Without Observation

Example:

```text
Potential:
  contractile-like = 0.88

Observed:
  contractile-like = unknown

Reason:
  Contractile Material exists,
  but no pressure/gradient stimulus occurred.
```

## 10.4 Observation Without Strong Potential

Це може означати:

- incomplete capability projection;
- classifier bug;
- hidden mechanism;
- incorrect registry mapping;
- direct state mutation outside expected Process pipeline.

Warning:

```text
OBSERVED_FUNCTION_WITHOUT_EXPLAINED_CAPABILITY
```

---

# 11. Multi-Label Classification

## 11.1 Supported Modes

```text
Primary only
All matched labels
Fractional contribution
```

## 11.2 Initial Thresholds

Starting defaults:

```text
minimum secondary score:
  0.45

minimum primary score:
  0.65

minimum primary confidence:
  0.60

dominance margin:
  0.10

mixed margin:
  0.10
```

Interpretation:

```text
top score >= 0.65
and
top score - second score >= 0.10
  -> classified

multiple scores >= 0.45
and
difference <= 0.10
  -> mixed

no score >= 0.45
  -> unknown
```

Thresholds must be:

- versioned;
- dimension-specific;
- visible in classifier metadata;
- configurable without changing Canon;
- validated on deterministic fixtures.

## 11.3 Fractional Contribution

Fractional contribution може бути:

```text
normalized matched label scores
```

але UI має пояснити normalization.

Сума може дорівнювати `1.0`, тільки якщо classifier explicitly normalizes contributions.

---

# 12. Confidence Model

## 12.1 Confidence Is Not Score

`label_score` показує відповідність label evidence.

`confidence` показує, наскільки надійним є висновок.

## 12.2 Confidence Factors

Initial factors:

```text
evidence_match
data_completeness
persistence
temporal_consistency
feature_reliability
classifier_calibration
```

Starting formula:

```text
confidence =
  evidence_match
  × data_completeness
  × persistence
  × temporal_consistency
  × feature_reliability
```

Усі factors bounded:

```text
0.0 .. 1.0
```

Для уникнення надмірного падіння confidence implementation може використовувати weighted geometric mean, але formula повинна бути versioned.

## 12.3 Data Completeness

```text
data_completeness =
  available required evidence weight
  /
  total required evidence weight
```

Starting status rule:

```text
data_completeness < 0.60
  -> insufficient_data
```

## 12.4 Temporal Consistency

Temporal consistency враховує:

- score variance between subwindows;
- abrupt unexplained role flips;
- event-driven temporary spikes;
- entity presence;
- lifecycle changes.

## 12.5 Confidence Bands

Suggested UI bands:

```text
0.00–0.39  low
0.40–0.69  medium
0.70–0.84  high
0.85–1.00  very high
```

UI не повинно приховувати exact value.

---

# 13. Benefit, Cost And Trade-Off

Кожен функціональний або поведінковий label повинен мати:

```text
at least one benefit metric
at least one cost metric
or explicit reason why cost is not observable yet
```

Example:

```text
storage-like

benefit:
  Resource retained during scarcity
  overflow prevented
  survival during Resource gap

cost:
  Material synthesis cost
  maintenance cost
  occupied capacity
  slower growth
```

Classification не повинна автоматично припускати, що function beneficial.

Possible result:

```text
Observed Role:
  storage-like

Outcome:
  no measurable survival benefit
  high upkeep cost
```

Це валідний observer result.

---

# 14. Environment Context

Behavior і survival profiles мають environmental context.

Required fields:

```text
Resource state
Field state
hazard state
local density
competition level
time interval
scenario id
scenario version
config hash
seed
```

Один profile може бути:

```text
beneficial in scarcity
neutral in pulses
costly in abundance
```

Не допускається global conclusion:

```text
strategy X is best
```

без explicit tested environment set.

---

# 15. Organization-Level Classification

OrganismView classification використовує derived structure.

Minimum sources:

```text
Cell-Joint component
component age
cell count
joint count
connectedness
lineage composition
Genome composition
resource flow summary
signal flow summary
fragmentation / merge / collapse events
```

Optional future sources:

```text
mechanical dependency
repair dependency
reproduction dependency
survival without structure probability
role distribution
spatial morphology
```

Archetype classification не змінює:

- OrganismView detection;
- component membership;
- Joint behavior;
- Cell behavior;
- lineage;
- compatibility.

---

# 16. Species-Like Cluster Boundary

Species-like clusters:

- є dynamic analytics groups;
- можуть мати fuzzy membership;
- використовують lineage/genome/compatibility/niche evidence;
- не є fixed labels registry на кшталт `species_A`;
- мають dynamic cluster ids;
- не є behavior inputs.

Registry може визначати лише status labels:

```text
unclassified
stable_cluster
emerging_cluster
merging_clusters
diverging_clusters
mixed_cluster
```

Actual cluster identity генерується analytics result:

```text
cluster_id = observer artifact
```

---

# 17. Provenance And Explainability

Кожна classification має пояснювати:

```text
what was classified
for which interval
using which classifier version
using which registry version
which features contributed
which features contradicted
which thresholds were applied
how complete the data was
which environment context applied
```

Minimum explanation example:

```text
Observed transport-like: 0.78

Supporting evidence:
- ActiveUptake executions: 421
- uptake success rate: 0.84
- absorbed Resource per active tick: 0.31
- Transport Material fraction: 0.26
- uptake Energy cost: 14.2

Counter-evidence:
- ResourceExport executions: 0
- high rejection rate during final 20% of interval

Window:
- Tick 20_000–21_000

Confidence:
- 0.76

Classifier:
- cell-role/v1.0.0
```

---

# 18. Versioning

Version independently:

```text
classification registry
classifier implementation
feature set
threshold profile
visual encoding
projection schema
```

Changing only UI color does not change semantic classifier version.

Changing evidence weights, label meaning or threshold semantics requires version update.

Suggested ids:

```text
registry:
  observer-classification-registry/v1

classifier:
  cell-functional-role/v1.0.0
  behavior-profile/v1.0.0
  organism-archetype/v1.0.0

feature set:
  cell-role-features/v1

threshold profile:
  baseline/v1
```

## 18.1 Historical Reproducibility

Saved classification artifact must preserve enough metadata to recalculate or explain old results.

Do not silently reinterpret old labels using a new registry version.

---

# 19. Storage And Cadence

## 19.1 Cadence

Suggested categories:

```text
live lightweight:
  current state preview
  every rendered frame or frequent observer interval

rolling classification:
  every 100–1_000 ticks

research classification:
  on demand
  selected interval
  whole run
  multi-run comparison
```

## 19.2 Persistence

Persist:

- classification summaries;
- evidence references;
- classifier metadata;
- selected detailed artifacts;
- discoveries and role transitions.

Do not persist full per-Cell feature vectors every Tick by default.

## 19.3 Transition Events

Observer may emit:

```text
classification_appeared
classification_changed
classification_became_stable
classification_became_unstable
classification_disappeared
new_role_discovered
new_archetype_discovered
```

These are observer events and cannot enter simulation behavior.

---

# 20. UI Contract

UI must show:

```text
classification dimension
primary label
secondary labels
Potential / Observed
confidence
time interval
classifier version
data completeness
evidence summary
```

UI must distinguish:

```text
engine state
observer classification
user-defined tag
saved filter
species-like cluster
warning
discovery
```

Recommended visual markers:

```text
Potential:
  outlined badge

Observed:
  filled badge

Mixed:
  segmented badge

Unknown:
  neutral badge

Insufficient data:
  muted badge with explanation
```

Classification filters must preserve:

- threshold;
- mode;
- classifier version;
- time context.

---

# 21. Warnings

Canonical observer classification warnings:

```text
CLASSIFIER_FAILED
INCOMPATIBLE_CLASSIFIER_VERSION
INSUFFICIENT_CLASSIFICATION_DATA
UNSTABLE_CLASSIFICATION
POTENTIAL_NOT_ACTIVATED
OBSERVED_FUNCTION_WITHOUT_EXPLAINED_CAPABILITY
CLASSIFICATION_THRESHOLD_NOT_CALIBRATED
CLASSIFICATION_DATA_GAP
CLASSIFICATION_CONFLICT
UNKNOWN_REGISTERED_ROLE
UNTESTED_CLASSIFICATION_LABEL
```

Warnings are analytics results, not simulation state.

---

# 22. Testing Contract

## 22.1 Unit Tests

```text
feature extraction
normalization
threshold application
multi-label selection
confidence calculation
status selection
version compatibility
missing-data handling
Potential vs Observed separation
```

## 22.2 Deterministic Fixture Tests

For each label:

```text
positive fixture
negative fixture
mixed fixture
insufficient-data fixture
temporary-spike fixture
```

## 22.3 Directional Tests

Example:

```text
higher repeated uptake
→ transport-like score does not decrease

higher repeated repair output
→ repair-focused score does not decrease

one temporary signal
→ signal-processing-like does not become stable
```

## 22.4 Integration Tests

```text
committed state -> projection -> features -> classification
same run + same interval + same versions -> same result
sampling does not change simulation
UI filters do not change classification source data
historical context does not silently use live state
```

## 22.5 Registry Coverage

Every enabled label requires:

```text
definition
required features
classifier mapping
positive fixture
negative fixture
UI metadata
report explanation
```

Missing coverage warning:

```text
UNTESTED_CLASSIFICATION_LABEL
```

---

# 23. Initial Acceptance Criteria

This contract is implemented when:

```text
classifications are observer-only
Potential and Observed are separated
results include confidence and evidence
results include interval and provenance
multi-label classification is supported
unknown and insufficient_data are honest states
single-Tick spikes do not create stable roles
classifiers are versioned
threshold profiles are versioned
UI can explain every displayed label
same input and versions reproduce the same classification
classification cannot affect simulation behavior
registry coverage is testable
```

---

# 24. Semantic Links

- derives from: [[docs/biology/organism|Organism View]]
- derives from: [[docs/biology/specialization|Specialization]]
- interprets: [[docs/evolution/adaptation|Adaptation]]
- interprets: [[docs/evolution/selection|Selection]]
- interprets: [[docs/evolution/population-dynamics|Population Dynamics]]
- constrains: [[docs/ui/exploration|UI Exploration]]
- constrains: [[docs/ui/analytics|UI Analytics]]
- follows: [[docs/mechanics/observer-projection|Observer Projection]]
- registry: [[docs/observer/classification-registry|Classification Registry]]
- configuration registry: [classification-registry.toml](file:///c:/Users/korsr/PycharmProjects/ALife/docs/config/observer/classification-registry.toml)
- cell functional role config: [cell-functional-role-classifier.toml](file:///c:/Users/korsr/PycharmProjects/ALife/docs/config/observer/cell-functional-role-classifier.toml)
- behavior profile config: [behavior-profile-classifier.toml](file:///c:/Users/korsr/PycharmProjects/ALife/docs/config/observer/behavior-profile-classifier.toml)
- organism archetype config: [organism-archetype-classifier.toml](file:///c:/Users/korsr/PycharmProjects/ALife/docs/config/observer/organism-archetype-classifier.toml)

