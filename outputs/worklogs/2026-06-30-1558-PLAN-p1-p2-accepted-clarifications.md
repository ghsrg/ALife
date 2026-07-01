---
tags:
  - alife
  - worklog/plan
---

# PLAN: p1-p2-accepted-clarifications

Дата: 2026-06-30 15:58

Джерело:

- `outputs/worklogs/2026-06-30-1405-PLAN-physics-logic-repeat-audit.md`
- user clarification for P1 items
- user decision: P2 proposals accepted

Мета: зафіксувати прийняті P1/P2-рішення перед внесенням правок у Canon.

---

# Scope

Виправляти:

1. P1 Reaction accounting.
2. P1 Field Effect / Heat-temperature accounting.
3. P1 2D Space radius/volume/capacity relation.
4. P1 Tick document simplification.
5. P1 Action / Process Registry.
6. P1 MaterialFragment vs Resource boundary.
7. P2 Organism wording in principles.
8. P2 GLOSSARY/ROADMAP sync.
9. P2 Stability bounds hard invalid/warning rules.

P0 вже винесені окремо в:

```text
outputs/worklogs/2026-06-30-1457-PLAN-p0-accepted-clarifications.md
```

---

# P1. Reaction Accounting Contract

## Проблема

Reaction accounting не має достатньо явного contract для збереження матерії в simulation amount units.

## Вплив

Реакції можуть мовчки створювати або знищувати Resources/Materials. `energy_output` може бути помилково сприйнятий як пояснення missing matter.

## Пропозиція

Оновити:

- `docs/world/reactions.md`
- `docs/config/reactions_config.md`
- `docs/engine/chemistry.md`
- за потреби `docs/world/laws.md`

Зафіксувати `Reaction Accounting Contract`:

```text
Reactions may simplify chemistry, but they must not silently create or destroy matter.
Resources and Materials are accounted in simulation amount units.
Every reaction must explicitly describe where input matter goes:

inputs
  -> products
  -> retained/internalized material
  -> residual/waste
  -> configured sink/loss
```

Energy rule:

```text
energy_output represents released or captured energy potential from input Resources/Materials.
It does not replace material outputs and does not explain missing matter.
```

Configured loss:

```text
Loss is allowed only when explicitly modeled as outflow, degradation sink,
evaporation-like removal, radiation-like escape, or scenario-defined sink.
Hidden disappearance is not allowed.
```

Validation:

```text
Warn when input/output accounting is not balanced.
Warn when part of input matter has no explicit destination.
Fatal when reaction creates products without inputs.
Fatal when unknown Resources/Materials are used.
Fatal when core invariants are violated.
```

Invariant:

```text
Energy is not matter.
Reaction products must have material sources.
Configured loss must be explicit.
Unaccounted Resources or Materials are invalid or at least a validation warning.
```

Формулювати як `material/amount accounting`, не як strict SI `mass conservation`.

---

# P1. Field Effect Contract + Heat as concrete profile

## Проблема

Heat/temperature має локальну модель, але той самий принцип стосується всіх Fields: Field не має бути command або shortcut effect.

## Вплив

Без спільного contract кожне Field може отримати власну приховану магію: Light створює Energy напряму, Pressure шкодить без Material threshold, Radiation мутує без damage rule тощо.

## Пропозиція

Не створювати вузький `docs/world/heat.md`.

Натомість:

- розширити `docs/world/field-semantics.md` до повноцінного `Field Effect Contract`;
- або створити `docs/world/field-effects.md` і оновити посилання.

Рекомендовано: розширити `docs/world/field-semantics.md`, бо файл уже існує і має правильний scope.

Оновити також:

- `docs/world/fields.md`
- `docs/world/physics.md`
- `docs/world/energy.md`
- `docs/world/reactions.md`
- `docs/config/fields_config.md`
- `docs/config/stability_bounds.md`
- `docs/biology/joint.md`
- `docs/engine/physics.md`

Загальне правило:

```text
Field is not a command.
Field does not create effects directly.
Cell/Material/Resource reacts to Field only through material capability, reaction, physics or process.
Each Field needs minimum accounting rules:
  origin
  propagation/decay
  local sampling
  effect mechanism
  bounds
  conserved or abstracted behavior
```

Concrete profiles:

```text
Heat:
  temperature
  heat_capacity
  heat_generated
  heat_transfer_rate
  heat_dissipation_rate
  material_heat_tolerance

Light:
  intensity
  propagation/occlusion
  absorption
  conversion through photosensitive Material
  no direct Energy Buffer

Pressure:
  collision/crowding/flow source
  material strength/elasticity/Boundary tolerance

Radiation:
  explicit damage/mutation/degradation rules only

Chemical gradient:
  Resource distribution or derived Field
  local sample through sensing Material

Flow:
  movement/resource transport through physics rules
```

Heat base model:

```text
Heat is local physical effect represented through local temperature state and explicit transfer/dissipation rules.
Heat is not Energy Buffer and not global command field.
temperature is local state of Cell, Material, Resource patch or environment patch.
reaction heat_output changes local temperature through heat_capacity.
heat transfer is local: contact, nearby patch, Joint with heat capability.
local dissipation is allowed as explicit sink if global HeatField is not modeled.
Heat damage works only through Material tolerance/degradation thresholds.
```

Invariant:

```text
Heat is not Energy Buffer.
Heat damage is Material degradation.
Heat transfer is local and explicit.
Heat dissipation must be configured, not hidden.
Reaction heat_output changes local temperature through heat_capacity.
Field effects require explicit mechanism and material/process/physics/reaction mediation.
```

---

# P1. 2D Space radius/volume/capacity relation

## Проблема

2D model використовує radius, footprint, volume_capacity, mass і capacity, але їхній зв'язок не зафіксований.

## Вплив

Маленька клітина може отримати необмежену capacity або mass без stress rule; growth/division/collision можуть розійтися між physics і biology.

## Пропозиція

Оновити:

- `docs/world/space.md`
- `docs/world/units.md`
- `docs/config/world_config.md`
- `docs/biology/cell.md`
- `docs/world/physics.md`
- `docs/config/stability_bounds.md`

Зафіксувати:

```text
In the 2D base model, volume_capacity is an internal simulation capacity unit, not SI volume.
It must be bounded by physical footprint and storage-capable Materials.

2D radius -> physical footprint
Materials -> storage structure
volume_capacity -> bounded internal capacity
```

Starting formula:

```text
cell_area = π * radius²

volume_capacity =
  base_capacity_per_area
  * cell_area
  * storage_material_modifier
```

Це placeholder contract, не фінальна фізика.

Rules:

```text
radius defines collision/locality footprint.
cell_area is not automatically equal to volume_capacity.
storage-capable Materials may increase effective capacity.
non-storage Materials may occupy capacity without increasing storage.
mass is derived from contained Resources, Materials and configured density values.
Energy Buffer does not add separate mass or volume.
```

Overflow outcomes:

```text
growth increases radius if boundary/material rules allow it
excess is rejected or exported
internal pressure/stress increases
cell becomes unstable/degrading
division preparation becomes possible
```

Validation:

```text
radius too small for configured minimum capacity
capacity too high for radius/materials
stored amount exceeds capacity
mass too high for footprint without stress rule
division creates daughters below minimum viable footprint/capacity
```

Invariant:

```text
In 2D, volume_capacity is abstract internal capacity.
It is not literal SI volume.
It must be bounded by radius, footprint and storage-capable Materials.
A small Cell cannot contain unlimited matter.
```

---

# P1. Simplify `docs/world/tick.md`

## Проблема

`tick.md` досі схожий на literal scheduler і дублює/відстає від `tick-semantics.md` та `engine/scheduler.md`.

## Вплив

Майбутня реалізація може взяти phase list з `tick.md` як canonical implementation order і порушити Tick Semantics.

## Пропозиція

Оновити `docs/world/tick.md` як conceptual time document.

Зафіксувати:

```text
Tick = logical simulation step
simulation time = configured interpretation of ticks
wall-clock time = real execution speed
```

Default calibration:

```text
1 Tick = 1 simulation second
```

але це calibration default, not biological truth.

References:

```text
docs/world/tick-semantics.md defines visibility, phase commits and causality.
docs/engine/scheduler.md defines scheduler implementation.
```

Conceptual flow only:

```text
environment preparation
cell decision
planned action resolution
physics / lifecycle resolution
statistics / observer update
```

No literal implementation order in `tick.md`.

Base exclusions:

```text
Base Tick model does not require HGT, recombination, advanced learning systems or specialized lifecycle phases for future genetic extensions.
```

Invariant:

```text
tick.md defines conceptual time.
tick-semantics.md defines visibility and causality.
engine/scheduler.md defines implementation.
No literal phase list in tick.md may override the Tick Semantics contract.
```

---

# P1. Action / Process Registry

## Проблема

Process ids, Genome output bindings, feasibility scope, capabilities and duration are spread across several documents.

## Вплив

Перша реалізація не матиме єдиного списку того, що Genome може output-ити, що Feasibility приймає, які actions atomic/long-running і які capabilities потрібні.

## Пропозиція

Створити:

```text
docs/biology/action-process-registry.md
```

Other documents may explain concepts, but must not define independent executable process/output lists. They should reference the registry.

Registry fields:

```text
process_id
kind: passive | planned_action | controlled_reaction
implementation_level: mvp | future_compatible | post_mvp
duration: atomic | long_running
required_capabilities
allowed_genome_output
feasibility_scope
resource_cost
energy_cost
material_cost
result_state
failure_behavior
```

Initial MVP process set:

```text
mandatory_upkeep
resource_uptake
resource_export
energy_conversion
material_synthesis
basic_repair
signal_emit
movement_request
division_preparation
genome_copying
division_partition
dormancy_shift
internal_rebalance
```

Future-compatible:

```text
joint_creation
joint_repair
joint_strengthening
hgt_fragment_uptake
genome_integration
recombination
advanced_learning_plasticity
specialized_structure_growth
fast_signal_conduction
```

Rules:

```text
Genome Runtime may output only priorities listed in this registry.
Feasibility Check accepts only registered planned actions or controlled reactions.
Passive processes do not require Genome output, but still require explicit rules.
A process may be long-running only if registry marks it as long_running.
```

Invariant:

```text
The Action / Process Registry is the canonical source of process ids,
Genome output bindings, feasibility scope and process duration.
No document may introduce a new executable process or Genome output outside this registry.
```

---

# P1. MaterialFragment vs Resource boundary

## Проблема

External Material remains, Cell-internal functional Materials and Resources are not separated enough.

## Вплив

Dead remains may be consumed, activated or reused without explicit process; Material capabilities may work outside living context.

## Пропозиція

Оновити:

- `docs/world/materials.md`
- `docs/world/resources.md`
- `docs/biology/lifecycle.md`
- `docs/engine/ecs.md`
- `docs/config/materials_config.md`
- `docs/config/reactions_config.md`

Define:

```text
Material inside Cell      -> functional cell material
MaterialFragment outside  -> physical remains / structure / debris
Resource                  -> movable or consumable substance
```

MaterialFragment may retain:

```text
material_id
amount
location
mass / volume
stability
damage state
decay_rate
remaining structural properties
```

Rules:

```text
Outside a Cell, MaterialFragment does not provide active cell capabilities.
Active capabilities require Cell context, Energy Buffer, Genome Runtime and process execution.
Dead cells may produce MaterialFragments.
MaterialFragments degrade/react into Resources only through explicit rules.
Living Cell cannot silently absorb MaterialFragment as ordinary Resource.
Uptake requires fragment breakdown, external digestion, surface degradation, material uptake capability, or conversion into Resource.
```

Context-specific capability interpretation:

```text
inside living Cell  -> may enable processes
inside Joint        -> may enable joint behavior
outside Cell        -> passive physical/material properties only
```

Invariant:

```text
External MaterialFragment is matter with material identity, not an active cell component.
Material capabilities require the proper biological context.
A MaterialFragment becomes Resource only through explicit degradation, reaction or conversion rules.
No dead remains may be consumed, activated or reused without an explicit process.
```

---

# P2. Organism wording in `PRINCIPLES.md`

## Проблема

`PRINCIPLES.md` says organism is a connected graph of cells. Current `biology/organism.md` is more precise: connected component is candidate; organism-like status depends on dependency/integration; `OrganismView` is observer-side.

## Вплив

Any connected cluster may be treated as organism too early.

## Пропозиція

Оновити `docs/PRINCIPLES.md`:

```text
Organism is observer-side organism-like graph view.
Connected component is a candidate, not automatic organism.
Organism identity is not a behavior input.
```

Уточнити:

```text
decomposing cells may be part of collapse/remains tracking,
but do not make a structure a living organism-like system.
```

---

# P2. GLOSSARY / ROADMAP sync

## Проблема

`docs/GLOSSARY.md` і `docs/ROADMAP.md` відстають після чистки й нових P0/P1-рішень.

## Вплив

Required reading може містити застарілі визначення.

## Пропозиція

Після P0/P1 правок синхронізувати:

- `docs/GLOSSARY.md`
- `docs/ROADMAP.md`
- за потреби `README.md`
- за потреби `docs/README.md`

Терміни для звірки:

```text
Energy Buffer / Energy capacity
Genome carrier
Reaction Accounting
Field Effect
Heat / temperature
volume_capacity
MaterialFragment
Action / Process Registry
OrganismView
Stability Bound / Seed Config
```

---

# P2. Stability bounds hard invalid / warning rules

## Проблема

`docs/config/stability_bounds.md` має категорії меж, але не має мінімальних invalid/warning criteria для очевидно небезпечних значень.

## Вплив

Seed configs for boundary testing будуть важко валідувати.

## Пропозиція

Оновити `docs/config/stability_bounds.md`.

Додати:

```text
Hard invalid examples
Warning ranges
Scenario-specific experimental ranges
```

Покрити щонайменше:

```text
negative amount/rate
diffusion/decay above safe per-Tick bound
reaction rate too high for dt
unbounded heat/field values
mutation rate beyond graph validation limits
cell density/radius impossible for world size
capacity too high for radius/materials
stored amount exceeds capacity
missing explicit sink/loss in reactions
```

Не фіксувати фінальні числа там, де потрібна симуляційна калібровка.

---

# Verification After Implementation

Після внесення правок перевірити:

```powershell
rg -n "Reaction Accounting|configured sink|material/amount accounting|energy_output" docs/world/reactions.md docs/config/reactions_config.md docs/engine/chemistry.md
rg -n "Field Effect|Heat|temperature|heat_capacity|material_heat_tolerance|Field is not a command" docs/world/field-semantics.md docs/world/fields.md docs/world/physics.md docs/config/fields_config.md
rg -n "volume_capacity|cell_area|base_capacity_per_area|storage_material_modifier|π \\* radius" docs/world/space.md docs/world/units.md docs/config/world_config.md docs/biology/cell.md
rg -n "conceptual time|tick-semantics.md defines|No literal phase list|Tick = logical simulation step" docs/world/tick.md
rg -n "action-process-registry|Action / Process Registry|process_id|Genome Runtime may output only" docs/biology/action-process-registry.md docs/biology/processes.md docs/genetics/regulatory-network.md docs/genetics/regulatory-interface.md
rg -n "MaterialFragment|physical remains|outside a Cell|becomes Resource only" docs/world/materials.md docs/world/resources.md docs/biology/lifecycle.md docs/engine/ecs.md
rg -n "OrganismView|connected component is a candidate|observer-side" docs/PRINCIPLES.md docs/biology/organism.md docs/GLOSSARY.md
rg -n "Hard invalid|Warning ranges|Scenario-specific" docs/config/stability_bounds.md
```

Перевірити, що не з'явились:

```text
Field as command
Light creates Energy directly
Heat damage as HP
reaction products without material source
MaterialFragment as active cell material outside Cell
unregistered Genome output
literal scheduler order in tick.md
unbounded capacity for small Cell
```
