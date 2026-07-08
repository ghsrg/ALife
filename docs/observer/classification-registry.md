---
tags:
  - alife
  - observer
  - classification
  - registry
  - biology
registry_id: observer-classification-registry/v1
---

# Observer Classification Registry

> Початковий versioned registry observer-side класифікацій для Cells, OrganismView, lineages та populations.
>
> Registry дає конкретний стартовий набір labels, evidence expectations, benefit/cost metrics і biological analogies.
>
> Labels не є engine classes, species ids, behavior inputs або готовими біологічними органами.

---

# 1. Призначення

Registry відповідає на питання:

```text
Які observer labels існують?
До яких entities вони застосовуються?
Які дані підтримують label?
Які дані його заперечують?
Який benefit він може мати?
Яку cost або trade-off треба вимірювати?
Який minimum window потрібний?
Як label пояснюється в UI?
```

Це базовий registry.

Він:

- не вважається повним;
- може розширюватися;
- має versioning;
- не hardcode-иться в UI;
- не замінює classifiers;
- не змінює simulation semantics;
- не гарантує, що всі labels будуть реалізовані в першій версії.

---

# 2. Registry Dimensions

Initial dimensions:

```text
1. Cell Functional Role
2. Sensory Specialization
3. Behavior / Survival Profile
4. Organization Archetype
5. Organization Morphology
6. Dependency Profile
7. Lineage / Population Profile
8. Species-like Cluster Status
```

Primary implementation priority:

```text
P0:
  Cell Functional Role
  Behavior / Survival Profile
  Organization Archetype

P1:
  Sensory Specialization
  Organization Morphology
  Dependency Profile

P2:
  Lineage / Population Profile
  Species-like Cluster Status
```

---

# 3. Shared Registry Rules

## 3.1 Naming

Canonical ids:

```text
lowercase-kebab-case
```

Examples:

```text
transport-like
dormancy-oriented
resource-sharing-colony-like
```

## 3.2 No Hardcoded Biological Identity

Allowed:

```text
contractile-like
signal-processing-like
filamentous
resource-sharing
```

Avoid as canonical engine-like labels:

```text
muscle-cell
neuron
blood-vessel
brain
predator
plant
animal
bacteria
```

Earth biology terms may appear as analogy only.

## 3.3 Evidence Priority

Starting evidence priority:

```text
observed Process output
observed cost
persistent state
Material composition
capability presence
single event
visual appearance
```

Visual appearance alone never proves function.

## 3.4 Benefit And Cost

Every functional or behavioral label should expose:

```text
benefit metrics
cost metrics
trade-off expectations
```

If cost cannot yet be observed:

```text
cost_status = missing_metric
```

Do not silently assume zero cost.

---

# 4. Cell Functional Role

`Cell Functional Role` describes a persistent function pattern of one Cell.

Applicable to:

```text
Cell
Cell subset
OrganismView role distribution
Lineage role distribution
Population role distribution
```

Potential and Observed modes are supported.

---

## 4.1 boundary-supporting

### Meaning

Cell invests materially or operationally in maintaining a selective, stable and protective Boundary.

Biological analogy:

- membrane maintenance;
- epithelial/barrier-like support;
- protective outer layer;
- exchange boundary.

This is not a hardcoded skin cell.

### Potential Evidence

```text
high Boundary Material fraction
Boundary repair capability
permeability control capability
damage resistance capability
```

### Observed Evidence

```text
Boundary maintenance executions
Boundary repair executions
reduced leakage
stable permeability under stress
damage absorbed or prevented
persistent outer-position context in an OrganismView
```

### Benefit Metrics

```text
resource leakage prevented
boundary damage avoided
survival under pressure/Heat/hazard
internal Resource retention
structural integrity contribution
```

### Cost Metrics

```text
Boundary Material synthesis cost
maintenance Energy
repair Resource cost
occupied capacity
reduced allocation to growth or transport
```

### Counter-Evidence

```text
high Boundary Material but repeated uncontrolled leakage
no measurable protection
Boundary state entirely explained by temporary damage response
```

### Minimum Window

```text
medium
```

### Related Labels

```text
protective
structural-like
repair-focused
transport-like
```

---

## 4.2 transport-like

### Meaning

Cell persistently specializes in controlled Resource uptake, export or local transfer.

Biological analogy:

- absorptive cell;
- membrane transporter-rich cell;
- exchange-specialized interface.

This is not a blood vessel or digestive organ.

### Potential Evidence

```text
Transport-capable Material
ActiveUptake capability
ResourceExport capability
appropriate Boundary permeability
```

### Observed Evidence

```text
high ActiveUptake execution count
high uptake success rate
high absorbed Resource per active tick
ResourceExport executions
Resource transfer through valid Joint channels
```

### Benefit Metrics

```text
Resource captured
Resource delivered to connected Cells
capture share
uptake throughput
reduced local Resource shortage
```

### Cost Metrics

```text
uptake Energy cost
Transport Material synthesis cost
maintenance cost
failed uptake attempts
capacity pressure
opportunity cost vs metabolism/storage/growth
```

### Counter-Evidence

```text
Transport capability exists but no uptake occurs
uptake occurs only because all Cells passively absorb
high attempts with near-zero success
```

### Minimum Window

```text
short to medium
```

### Related Labels

```text
resource-sharing
high-throughput
boundary-supporting
metabolic-like
```

---

## 4.3 metabolic-like

### Meaning

Cell persistently converts Resources into usable Energy or other registered metabolic outputs.

Biological analogy:

- metabolically active cell;
- energy-conversion specialization.

This does not imply mitochondria or a specific biochemical pathway.

### Potential Evidence

```text
Metabolic Material
Metabolism capability
required Resource inputs
registered reaction/process path
```

### Observed Evidence

```text
Metabolism execution count
Resource metabolized
Energy produced
conversion efficiency
sustained production across subwindows
```

### Benefit Metrics

```text
Energy produced
Energy supplied to own Processes
survival benefit
support of growth/repair/division
reduced starvation time
```

### Cost Metrics

```text
Resource consumed
Metabolic Material cost
Heat/Waste output
maintenance Energy
capacity use
risk of Resource depletion
```

### Counter-Evidence

```text
high Metabolic Material with no production
production explained only by passive Energy income
large Resource consumption with negligible Energy output
```

### Minimum Window

```text
short to medium
```

### Related Labels

```text
energy-production-like
high-throughput
resource-efficient
opportunistic-growth
```

---

## 4.4 storage-like

### Meaning

Cell persistently allocates capacity and Materials to retain Resources or Energy across changing conditions.

Biological analogy:

- reserve cell;
- storage tissue-like function;
- vacuole/fat-storage analogy.

No specific biological organelle is implied.

### Potential Evidence

```text
Storage Material
increased Resource capacity
increased Energy capacity
retention capability
```

### Observed Evidence

```text
high storage utilization
Resource retained across scarcity interval
overflow prevented
stored amount later consumed
lower depletion rate during Resource gap
```

### Benefit Metrics

```text
survival during scarcity
Resource retained
Energy retained
overflow prevented
wake/recovery success after scarcity
```

### Cost Metrics

```text
Storage Material synthesis cost
maintenance cost
occupied volume/capacity
increased mass
slower growth
reduced transport/metabolism allocation
```

### Counter-Evidence

```text
capacity exists but remains unused
stored Resource never contributes to survival or activity
storage benefit explained by excessive passive income
```

### Minimum Window

```text
medium to long
```

### Related Labels

```text
storage-buffered
scarcity-adapted
dormancy-oriented
conservative-growth
```

---

## 4.5 synthesis-oriented

### Meaning

Cell persistently converts Resources and Energy into functional or structural Materials.

Biological analogy:

- biosynthetic specialization;
- producer/building cell.

This is not a hardcoded factory cell.

### Potential Evidence

```text
Synthesis Material
MaterialSynthesis capability
registered synthesis recipes
available Resources and Energy
```

### Observed Evidence

```text
MaterialSynthesis executions
Material output by type
synthesis throughput
successful completion rate
```

### Benefit Metrics

```text
new functional Material
Boundary/Joint/Structural support
repair substrate
growth readiness
division readiness
```

### Cost Metrics

```text
Resource cost
Energy cost
synthesis time
failed synthesis
capacity pressure
reduced short-term survival reserve
```

### Counter-Evidence

```text
high synthesis attempts with no output
Material increase caused only by inheritance
synthesis occurs as a one-time repair spike
```

### Minimum Window

```text
medium
```

### Related Labels

```text
structural-like
repair-focused
reproduction-supporting
growth-oriented
```

---

## 4.6 structural-like

### Meaning

Cell persistently contributes Material, geometry or mechanical support to its own body or a connected structure.

Biological analogy:

- structural cell;
- support tissue-like function;
- wall/skeleton-like contribution.

No tissue or organ is hardcoded.

### Potential Evidence

```text
Structural Material
high strength/stability
Joint support capability
growth capability
```

### Observed Evidence

```text
Structural Material synthesis
radius growth
high mechanical load
stable Joint anchoring
low deformation/damage under load
persistent position in load-bearing part of OrganismView
```

### Benefit Metrics

```text
mechanical stability
component lifetime
Joint survival
damage resistance
growth/division preparation
shape retention
```

### Cost Metrics

```text
Material synthesis cost
Energy cost
mass
mobility penalty
maintenance cost
reduced flexible capacity
```

### Counter-Evidence

```text
high structural fraction but no load or stability effect
temporary growth state only
```

### Minimum Window

```text
medium
```

### Related Labels

```text
mechanical-support-like
boundary-supporting
joint-supporting
growth-oriented
```

---

## 4.7 repair-focused

### Meaning

Cell persistently allocates Processes and Materials to restore damage or integrity.

Biological analogy:

- regenerative/repair specialization;
- maintenance-focused cell.

This is not an immune cell unless future mechanisms explicitly support such interpretation.

### Potential Evidence

```text
Repair Material
repair capability
damage sensing
repair recipes
```

### Observed Evidence

```text
Repair executions
damage repaired
Boundary/Material integrity restored
repeated repair after stress
```

### Benefit Metrics

```text
damage repaired
survival benefit
component lifetime extension
Joint/Boundary preservation
recovery time
```

### Cost Metrics

```text
Energy spent on repair
Material spent
Resource spent
growth/division opportunity cost
failed repair attempts
```

### Counter-Evidence

```text
repair capability exists but no damage occurred
damage repaired only once after artificial intervention
high cost with negligible integrity recovery
```

### Minimum Window

```text
medium
```

### Related Labels

```text
stress-tolerant
protective
conservative-growth
resource-sharing
```

---

## 4.8 contractile-like

### Meaning

Cell persistently generates force or contraction through Contractile Material.

Biological analogy:

- muscle-like contraction;
- contractile cytoskeleton;
- force-generating cell.

This is not a hardcoded muscle cell.

### Potential Evidence

```text
Contractile Material
Contraction capability
valid mechanical context
sufficient Energy
```

### Observed Evidence

```text
Contraction executions
force generated
active displacement
Joint tension changes
movement against local resistance
```

### Benefit Metrics

```text
distance travelled
Resource patch reached
escape success
position improvement
collective movement contribution
mechanical work
```

### Cost Metrics

```text
Energy spent
Contractile Material maintenance
damage/fatigue
lost growth opportunity
ineffective movement
```

### Counter-Evidence

```text
displacement caused only by collision correction or passive physics
force generated without useful movement
one isolated contraction event
```

### Minimum Window

```text
short to medium
```

### Related Labels

```text
mobile-foraging
gradient-following
mechanical-support-like
signal-responsive
```

---

## 4.9 sensory

### Meaning

Cell persistently detects local stimuli through a physical Material basis and produces differentiated internal or behavioral responses.

Biological analogy:

- receptor-rich cell;
- sensory specialization.

This is not a hardcoded eye, ear or receptor organ.

### Potential Evidence

```text
Sensory Material
registered sensitivity
local stimulus available
signal path into RuntimeState
```

### Observed Evidence

```text
stimuli detected
signal state changes
candidate actions generated after stimulus
response differs from no-stimulus baseline
useful response outcome
```

### Benefit Metrics

```text
Resource patch reached
hazard avoided
damage reduced
uptake increased
response latency
useful response rate
```

### Cost Metrics

```text
Sensory Material cost
maintenance Energy
false response cost
ineffective action cost
signal fatigue
```

### Counter-Evidence

```text
stimulus exists but no detection
detection occurs but never changes candidate/action pattern
all responses are random or ineffective
```

### Minimum Window

```text
medium
```

### Related Labels

```text
signal-processing-like
gradient-following
contractile-like
damage-sensitive
```

---

## 4.10 signal-processing-like

### Meaning

Cell persistently stores, transforms, conducts or integrates scalar signals through signal-plastic Materials and state.

Biological analogy:

- excitable/signalling cell;
- neural-like processing analogy.

This is explicitly not a hardcoded neuron.

### Potential Evidence

```text
signal sensitivity
signal storage
signal conductivity
signal-plastic MaterialState
Joint signal channel
```

### Observed Evidence

```text
repeated signal receipt
stored_signal dynamics
signal propagation
conductivity adaptation
response threshold/cooldown pattern
signal-conditioned Process bias
```

### Benefit Metrics

```text
improved response coordination
reduced response latency
higher useful response rate
successful propagation
collective benefit
```

### Cost Metrics

```text
signal production Energy
signal Material maintenance
fatigue
false activation
propagation loss
opportunity cost
```

### Counter-Evidence

```text
one transient signal
signal state with no downstream effect
debug trace only
```

### Minimum Window

```text
medium to long
```

### Related Labels

```text
sensory
signal-coordinated
contractile-like
resource-sharing
```

---

## 4.11 reproductive-supporting

### Meaning

Cell persistently contributes to division preparation, offspring viability or viable fragment formation.

Biological analogy:

- reproductive support;
- germline/support analogy.

No germ cell class is implied.

### Potential Evidence

```text
DivisionPreparation capability
Material synthesis for division
Energy storage for division
Joint context supporting viable fragment
```

### Observed Evidence

```text
division preparation time
division executions
offspring survival
Material/Energy allocation to division
viable fragment formation
```

### Benefit Metrics

```text
offspring count
offspring viability
generation reached
lineage persistence
successful fragmentation reproduction
```

### Cost Metrics

```text
Energy spent
Material spent
Resource spent
temporary vulnerability
growth/repair opportunity cost
```

### Counter-Evidence

```text
division readiness never reached
division caused non-viable offspring
apparent allocation explained by general growth
```

### Minimum Window

```text
medium to long
```

### Related Labels

```text
rapid-reproduction
growth-oriented
synthesis-oriented
fragmenting-structure
```

---

## 4.12 joint-supporting

### Meaning

Cell persistently creates, anchors, maintains or repairs material Joints.

Biological analogy:

- adhesion/support interface;
- connective specialization.

This is not a hardcoded tissue class.

### Potential Evidence

```text
Joint-compatible Material
Joint creation capability
Joint repair capability
```

### Observed Evidence

```text
Joint creation
Joint maintenance
Joint repair
stable anchoring
high local Joint degree with persistent integrity
```

### Benefit Metrics

```text
component stability
resource/signal channel availability
reduced fragmentation
mechanical integration
```

### Cost Metrics

```text
Joint Material
creation Energy
maintenance cost
repair cost
reduced independent mobility
```

### Counter-Evidence

```text
many Joints caused only by crowding
Joints repeatedly fail immediately
no measurable structural or flow benefit
```

### Minimum Window

```text
medium
```

### Related Labels

```text
structural-like
resource-sharing
signal-coordinated
mechanically-integrated
```

---

## 4.13 undifferentiated

### Meaning

Cell має capabilities, але не демонструє стійкого dominance жодної спеціалізованої ролі.

Evidence:

```text
low role-score dominance
broad but weak activity
short age
recent division
unstable material/process composition
```

`undifferentiated` не означає безфункціональність.

Status may later become another role.

---

## 4.14 mixed-function

### Meaning

Кілька ролей стабільно мають близькі scores і measurable outputs.

Requirements:

```text
at least two observed labels above threshold
difference within mixed margin
sufficient persistence
```

UI повинно показувати склад, а не приховувати secondary roles.

---

# 5. Sensory Specialization

Applied only when:

```text
Functional Role includes sensory
or
Potential sensory capability exists
```

Initial subtypes:

---

## 5.1 resource-gradient-sensitive

Evidence:

```text
local Resource gradient sampled
signal/input changes with gradient
movement/uptake response correlates with gradient direction
```

Benefit:

```text
Resource found
uptake increased
search Energy reduced
```

Cost:

```text
sensing upkeep
movement cost
false-gradient response
```

---

## 5.2 chemical-sensitive

Used for Resource-like trace substances with explicit physical properties.

Evidence:

```text
trace Resource sampled
signal-sensitive Material response
repeatable Process/Action response
```

Do not use for semantic commands such as `food` or `danger`.

---

## 5.3 temperature-sensitive

Evidence:

```text
local Heat/temperature sampled
response changes with bounded temperature variation
useful avoidance or regulation
```

---

## 5.4 pressure-sensitive

Evidence:

```text
pressure/contact stimulus
repeatable signal or ActionCandidate response
```

---

## 5.5 damage-sensitive

Evidence:

```text
damage state change detected
repair/withdrawal/protective response follows
```

---

## 5.6 contact-sensitive

Evidence:

```text
contact event
local state change
repeatable response
```

---

## 5.7 signal-sensitive

Evidence:

```text
external scalar signal
valid physical carrier
receiver Material basis
repeatable downstream response
```

---

## 5.8 light-sensitive

Reserved for explicit light-like Field with:

```text
origin
propagation
sampling
Material sensing basis
effect mechanism
```

Do not infer from display brightness.

---

## 5.9 mixed-sensory

Multiple sensory subtypes persist with similar scores.

---

# 6. Behavior And Survival Profiles

Behavior Profiles may apply to:

```text
Cell
OrganismView
Lineage
Population subset
```

They require Observed mode by default.

Potential profile may be shown only as experimental analytics.

---

## 6.1 dormancy-oriented

### Meaning

Entity repeatedly uses dormancy as a major survival response to low Energy or Resource shortage.

### Positive Evidence

```text
high dormant fraction during scarcity
repeated dormancy entry
survival through Resource gaps
successful wake transitions
reduced dormant upkeep
```

### Benefit Metrics

```text
survival extension
scarcity interval survived
wake success
Resource/Energy preserved
```

### Cost Metrics

```text
reduced active fraction
reduced growth
reduced division
reduced Resource capture
recovery delay
```

### Counter-Evidence

```text
dormancy never activated
dormancy occurs only immediately before death
high dormancy in abundance with no compensating benefit
```

### Trade-Off Expectation

```text
beneficial in temporary scarcity
inferior to active growth/reproduction in stable abundance
```

---

## 6.2 storage-buffered

### Meaning

Survival and activity are stabilized by retained Resources or Energy reserves.

Evidence:

```text
stores fill in abundance
stores decline during scarcity
survival exceeds comparable low-storage entities
recovery follows stored reserve use
```

Trade-off:

```text
capacity and survival benefit
vs
Material/upkeep/mass/growth cost
```

---

## 6.3 resource-efficient

### Meaning

Entity produces survival, activity, growth or reproduction with low Resource expenditure.

Suggested metrics:

```text
survival_ticks per Resource consumed
Energy produced per Resource metabolized
division count per Resource consumed
useful work per Resource consumed
```

Counter-evidence:

```text
low consumption caused only by inactivity
apparent efficiency caused by passive Energy income
```

Efficiency must be paired with output.

---

## 6.4 energy-efficient

### Meaning

Entity produces useful outputs with relatively low Energy spending.

Suggested metrics:

```text
useful output per Energy spent
movement benefit per Energy
damage repaired per Energy
division success per Energy
```

Do not classify inactive entities as efficient solely because they spend little.

---

## 6.5 high-throughput

### Meaning

Entity rapidly captures, converts or processes Resources.

Evidence:

```text
high uptake rate
high metabolism rate
high synthesis rate
high output rate
```

Trade-off:

```text
fast growth or Energy production
vs
Resource depletion, Heat/Waste, maintenance and overflow risk
```

---

## 6.6 opportunistic-growth

### Meaning

Entity rapidly increases growth/division activity during short periods of abundance.

Evidence:

```text
growth spike after Resource increase
division readiness reached quickly
high abundance response
activity decreases when abundance ends
```

Trade-off:

```text
rapid expansion
vs
poor scarcity survival or reserve depletion
```

---

## 6.7 conservative-growth

### Meaning

Entity maintains lower growth and reproduction in exchange for stability or resilience.

Evidence:

```text
low but persistent growth
high survival
stable Energy
low collapse frequency
```

Trade-off:

```text
resilience
vs
lower expansion and reproduction
```

---

## 6.8 rapid-reproduction

### Meaning

Entity prioritizes division rate and offspring count.

Evidence:

```text
short division interval
high division count
high offspring count
```

Required outcome context:

```text
offspring viability
parent survival
Resource cost
lineage persistence
```

High birth count with immediate offspring death is not automatically successful.

---

## 6.9 mobile-foraging

### Meaning

Entity uses active movement to improve Resource access.

Evidence:

```text
active displacement
Resource patch reached
uptake increase after movement
movement direction linked to local conditions
```

Trade-off:

```text
Resource benefit
vs
movement Energy, damage and lost processing time
```

---

## 6.10 gradient-following

### Meaning

Movement or orientation systematically follows a local Resource/Field gradient.

Evidence:

```text
gradient exists
sensor input changes
directional response
movement alignment above chance/baseline
benefit after movement
```

This is not conscious navigation.

---

## 6.11 sessile-resource-capture

### Meaning

Entity remains relatively stationary and relies on passive or local Resource capture.

Biological analogy:

- filter-feeding/sessile absorption;
- rooted or attached resource capture analogy.

Evidence:

```text
low active displacement
persistent local uptake
stable position near Resource flow
```

Trade-off:

```text
low movement cost
vs
dependence on local Resource availability
```

---

## 6.12 repair-oriented

### Meaning

Large and persistent allocation to repair improves resilience.

Evidence:

```text
high repair fraction
damage recovered
survival under repeated stress
```

Trade-off:

```text
resilience
vs
Energy/Material cost and slower growth/reproduction
```

---

## 6.13 stress-tolerant

### Meaning

Entity maintains viability and function under explicit environmental stress.

Stress context must be named:

```text
Heat
Pressure
damage
Resource scarcity
hazard
crowding
```

Do not use one global stress tolerance score without context.

---

## 6.14 pulse-adapted

### Meaning

Entity performs well under alternating abundance and scarcity.

Evidence:

```text
captures Resource during pulses
stores or converts efficiently
survives gaps
recovers after pulse
```

Trade-off:

```text
pulse performance
vs
possible lower efficiency in constant conditions
```

---

## 6.15 scarcity-adapted

### Meaning

Entity has relatively strong survival or reproduction under persistent low Resource conditions.

Evidence must be comparative across:

```text
same scenario
multiple profiles/lineages
multiple seeds where needed
```

Possible mechanisms:

```text
efficiency
dormancy
storage
low upkeep
repair
```

Label describes outcome, not one mechanism.

---

## 6.16 abundance-adapted

### Meaning

Entity converts stable abundance into growth, division or offspring success.

Trade-off expectation:

```text
high abundance performance
vs
possible poor scarcity performance
```

---

## 6.17 resource-sharing

### Meaning

Connected Cells or OrganismView members exchange Resources such that recipients or the structure gain measurable benefit.

Evidence:

```text
Joint Resource transfer
donor/receiver flow
reduced local shortage
improved component survival or growth
```

Cost:

```text
donor Resource loss
Joint cost
transport cost
dependency risk
```

Resource-sharing is not altruism by definition.

---

## 6.18 signal-coordinated

### Meaning

Connected Cells show temporally linked signal propagation and differentiated local responses.

Evidence:

```text
physical signal carrier
signal emitted and received with correct delay
response correlation
improved collective outcome
```

Do not call any correlated activity coordination without signal or shared-cause analysis.

---

## 6.19 generalist

### Meaning

Entity has moderate performance across several environments/functions without extreme specialization.

Evidence:

```text
broad capability use
moderate role distribution
low environment-specific regret
no single dominant role/profile
```

Generalist is comparative and requires environment set metadata.

---

## 6.20 specialist

### Meaning

Entity has strong performance in a narrow niche or function and clear cost outside it.

Evidence:

```text
high niche-specific rank
large performance drop outside niche
strong role dominance
```

Specialist must name the specialization:

```text
scarcity specialist
pressure specialist
resource-A specialist
repair specialist
```

---

## 6.21 boom-bust

### Meaning

Entity or lineage rapidly expands during favorable conditions and then sharply collapses.

Evidence:

```text
high expansion rate
Resource depletion or cost accumulation
high subsequent death rate
repeated cycles or clear run pattern
```

This is descriptive, not necessarily adaptive.

---

# 7. Organization Archetypes

Applied to:

```text
OrganismView
stable Cell-Joint component
component trajectory
```

Base detection remains:

```text
connected component of Cell-Joint graph
```

Archetype does not change membership.

---

## 7.1 transient-cluster

### Meaning

Short-lived connected component without persistent dependency or stable structure.

Evidence:

```text
short component lifetime
frequent merge/split
low Joint persistence
low shared flow
```

Biological analogy:

- temporary aggregation.

---

## 7.2 loose-colony-like

### Meaning

Stable co-located or connected group where most Cells remain individually viable and dependency is low.

Evidence:

```text
persistent component
low resource/signal dependency
low repair/reproduction coupling
high survival after separation
```

---

## 7.3 stable-colony-like

### Meaning

Persistent multi-Cell component with repeatable structure and interactions, but limited functional dependency.

Evidence:

```text
long component lifetime
stable Cell count range
persistent Joints
some shared flow
moderate specialization
```

---

## 7.4 resource-sharing-colony-like

### Meaning

Component stability or member performance is measurably supported by Resource transfer.

Evidence:

```text
Resource transfer edges
recipient benefit
flow persistence
reduced shortage
```

Trade-off:

```text
Joint/transport cost
donor cost
dependency
```

---

## 7.5 signal-coordinated-structure

### Meaning

Component exhibits persistent signal propagation and local response coordination.

Evidence:

```text
signal edges
correct Tick delay
response propagation
improved collective outcome
```

This is not a nervous system unless future analysis explicitly supports that analogy.

---

## 7.6 mechanically-integrated-structure

### Meaning

Component behaves as a mechanically coupled structure.

Evidence:

```text
persistent load-bearing Joints
collective displacement
mechanical dependency
shape retention
fragmentation after Joint failure
```

---

## 7.7 specialized-multicellular-structure

### Meaning

Component contains persistent differentiated Cell roles whose combination produces measurable structure-level benefit.

Requirements:

```text
at least two stable role groups
spatial or relational organization
persistent role distribution
measurable combined benefit
```

Counter-evidence:

```text
role diversity caused only by random temporary states
no structure-level benefit
```

---

## 7.8 high-dependency-organism-like

### Meaning

Member survival or function strongly depends on remaining within the component.

Evidence may include:

```text
resource_dependency
signal_dependency
mechanical_dependency
repair_dependency
reproduction_dependency
low survival after separation
```

This is the strongest organism-like observer label, but still not a Core entity.

---

## 7.9 modular-organism-like

### Meaning

Structure consists of semi-independent repeated modules.

Evidence:

```text
repeated subgraphs
local role combinations
partial survival after fragmentation
module-level reproduction or regrowth
```

Future classifier may use graph motifs.

---

## 7.10 fragmenting-reproductive-structure

### Meaning

Component repeatedly produces viable daughter components or founder fragments.

Evidence:

```text
fragmentation event
child component viability
lineage continuation
repeatability
```

Not every accidental split is reproduction-like.

---

## 7.11 clonal-structure

### Meaning

Component has high shared-lineage or Genome similarity.

Evidence:

```text
shared_lineage_ratio
Genome distribution
low mixedness
```

Clonal does not imply cooperation or organism-level integration.

---

## 7.12 mixed-lineage-structure

### Meaning

Component contains substantial contributions from multiple lineages or Genome groups.

Evidence:

```text
lineage distribution
Genome distribution
mixedness score
```

Mixed-lineage does not imply symbiosis unless benefit and persistence are shown.

---

## 7.13 symbiosis-like-structure

### Meaning

Different lineage/functional groups persist together with reciprocal measurable benefit.

Minimum evidence:

```text
mixed-lineage composition
persistent association
benefit to at least two groups
cost or dependency
better outcome together than relevant separated comparison
```

This label should remain P2 until comparison methodology exists.

---

# 8. Organization Morphology

Morphology describes geometry, not function.

One OrganismView may have several descriptors.

---

## 8.1 compact

```text
low perimeter-to-area ratio
high local density
short average graph distance
```

## 8.2 filamentous

```text
elongated component
low branching
high aspect ratio
chain-like graph
```

Biological analogy:

- filament;
- hypha-like chain;
- colonial chain.

## 8.3 branching

```text
multiple persistent graph branches
branch points above threshold
```

## 8.4 sheet-like

```text
approximately planar dense layer
local neighborhood resembles surface mesh
```

## 8.5 reticulate-network

```text
many cycles
multiple alternative paths
high graph redundancy
```

## 8.6 modular

```text
repeated or weakly connected subcomponents
community structure
```

## 8.7 polarized

```text
persistent asymmetry in roles, flows or geometry
```

## 8.8 layered

```text
spatially separated role/composition bands
```

## 8.9 amorphous

```text
no stable morphology descriptor above threshold
```

Morphology must not imply a function without process/flow evidence.

---

# 9. Dependency Profile

Dependency dimensions are multi-label numeric summaries.

```text
resource_dependency
signal_dependency
mechanical_dependency
repair_dependency
reproduction_dependency
environment_dependency
```

Suggested categorical bands:

```text
low
moderate
high
critical
unknown
```

Initial generic thresholds may begin at:

```text
low:
  0.00–0.24

moderate:
  0.25–0.49

high:
  0.50–0.74

critical:
  0.75–1.00
```

But each dependency metric requires its own validated definition.

Dependency is not automatically good or bad.

---

# 10. Lineage And Population Profiles

---

## 10.1 expanding

```text
positive population growth
birth/division exceeds death
sustained across window
```

## 10.2 stable

```text
population fluctuates within bounded range
no strong trend
```

## 10.3 declining

```text
persistent negative population trend
```

## 10.4 bottlenecked

```text
sharp population reduction
diversity reduction
few survivors
```

## 10.5 recovering

```text
growth after bottleneck or collapse interval
```

## 10.6 extinct

```text
no living members remain
```

## 10.7 diversifying

```text
increasing Genome/lineage/material profile diversity
```

## 10.8 converging

```text
decreasing profile diversity
increasing dominance of one variant/profile
```

## 10.9 niche-specialized

```text
performance advantage tied to explicit environment context
```

## 10.10 dominant

```text
high frequency or rank across explicit comparison set
```

Dominant is descriptive.

It does not imply universally fit.

---

# 11. Species-Like Cluster Status

Dynamic cluster identities are not registry labels.

Allowed status labels:

```text
unclassified
emerging-cluster
stable-cluster
diverging-clusters
merging-clusters
mixed-cluster
isolated-cluster
```

Minimum sources:

```text
lineage distance
Genome similarity optional
exchange patterns optional
compatibility optional
niche overlap optional
```

All labels must be displayed as:

```text
observer-only inferred cluster
```

---

# 12. Initial Cross-Dimension Combinations

Useful combinations:

```text
transport-like
+ metabolic-like
+ high-throughput

storage-like
+ dormancy-oriented
+ scarcity-adapted

sensory
+ contractile-like
+ gradient-following
+ mobile-foraging

repair-focused
+ stress-tolerant
+ conservative-growth

resource-sharing-colony-like
+ specialized-multicellular-structure
+ high resource dependency

signal-coordinated-structure
+ sensory role distribution
+ contractile role distribution

fragmenting-reproductive-structure
+ rapid-reproduction
+ modular morphology
```

Combination does not create a new Core type.

It may become a future composite observer label after validation.

---

# 13. Initial Visual Encoding Guidance

Visual encoding belongs to versioned UI configuration.

Suggested semantic families:

```text
boundary/protection:
  cyan-blue

transport:
  turquoise

metabolism/energy:
  amber

storage:
  yellow-green

synthesis/growth:
  green

structural:
  blue-gray

repair:
  mint

contractile:
  red-orange

sensory:
  violet

signal-processing:
  magenta-violet

reproduction:
  warm pink

unknown:
  neutral gray

mixed:
  segmented or multi-color
```

Rules:

- color does not define classification;
- Light/Dark variants must preserve meaning;
- color-blind-safe alternatives are required;
- confidence is not encoded only by saturation;
- Potential and Observed must differ by shape/fill, not only color.

---

# 14. Initial Implementation Order

## P0. Cell Roles

Implement first:

```text
boundary-supporting
transport-like
metabolic-like
storage-like
synthesis-oriented
structural-like
repair-focused
contractile-like
sensory
signal-processing-like
reproductive-supporting
joint-supporting
undifferentiated
mixed-function
```

## P0. Behavior Profiles

Implement first:

```text
dormancy-oriented
storage-buffered
resource-efficient
high-throughput
opportunistic-growth
conservative-growth
rapid-reproduction
mobile-foraging
repair-oriented
stress-tolerant
resource-sharing
generalist
specialist
```

## P0. Organization Archetypes

Implement first:

```text
transient-cluster
loose-colony-like
stable-colony-like
resource-sharing-colony-like
mechanically-integrated-structure
specialized-multicellular-structure
high-dependency-organism-like
fragmenting-reproductive-structure
clonal-structure
mixed-lineage-structure
```

---

# 15. Registry Coverage Requirements

Every enabled label requires:

```text
feature mapping
positive fixture
negative fixture
mixed/ambiguous fixture
minimum window test
confidence test
UI metadata
report explanation
```

Coverage output:

```text
label_id
dimension
registered
classifier_mapping
required_features_available
positive_fixture
negative_fixture
integration_test
ui_metadata
status
warning_codes
```

Statuses:

```text
covered
partially_covered
registered_but_disabled
missing_features
missing_classifier
missing_tests
```

Warning:

```text
UNTESTED_CLASSIFICATION_LABEL
```

---

# 16. Initial Acceptance Criteria

Registry v1 is usable when:

```text
all labels have canonical ids
all labels identify applicable entity types
all functional labels define Potential and Observed evidence
all functional/behavior labels define benefit and cost metrics
all stable labels require a time window
unknown and mixed states are supported
classification is multi-label
classification remains observer-only
UI loads labels from registry metadata
new labels can be added without changing chart implementations
saved results preserve registry version
```

---

# 17. Semantic Links

- contract: [[docs/observer/classification-contract|Classification Contract]]
- source: [[docs/biology/organism|Organism View]]
- source: [[docs/biology/specialization|Specialization]]
- source: [[docs/biology/communication|Communication]]
- source: [[docs/evolution/adaptation|Adaptation]]
- source: [[docs/evolution/selection|Selection]]
- source: [[docs/evolution/population-dynamics|Population Dynamics]]
- source: [[docs/evolution/species-like-clusters|Species-like Clusters]]
- presentation: [[docs/ui/exploration|UI Exploration]]
- analytics: [[docs/ui/analytics|UI Analytics]]
