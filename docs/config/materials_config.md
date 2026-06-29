# materials_config.md

> **Materials Config — конфігурація функціональних речовин клітини**

---

# Призначення

`materials_config.md` описує, як задавати типи Materials без зміни рушія.

Material — це функціональна речовина, з якої клітина будує свої можливості.

Materials можуть впливати на:

* Boundary;
* міцність;
* еластичність;
* проникність;
* repair;
* synthesis;
* Energy conversion;
* storage;
* signal sensing;
* signal conduction;
* Joint formation;
* movement;
* heat resistance;
* pressure resistance;
* learning-like state.

Material не є клітиною.

Material не є органом.

Material не є поведінкою.

Material — це властивості, через які клітина отримує функціональність.

---

# Основна ідея

Рушій не повинен мати hardcoded матеріали:

```text id="5yv5po"
membrane_material
muscle_material
neuron_material
skin_material
bone_material
```

Замість цього будь-який світ може задати власні Materials через config:

```text id="m5ogov"
flexible_boundary_gel
strong_structural_fiber
signal_sensitive_compound
energy_conversion_pigment
storage_matrix
adhesive_joint_polymer
```

Функція виникає не з назви Material, а з його параметрів.

---

# Що Material Config НЕ є

Material Config не повинен описувати:

* готові органи;
* готові тканини;
* типи клітин;
* поведінкові ролі;
* species-specific materials;
* neuron/muscle/skin як hardcoded класи;
* пряме створення Energy;
* direct HP;
* toxicity;
* healing magic.

Якщо Material допомагає клітині, це має бути через конкретні властивості.

Якщо Material шкодить, це має бути через нестабільність, реакції, Heat, volume, degradation або blocking.

---

# Загальна структура

Приклад:

```yaml id="d5wxwa"
materials:
  - id: "boundary_gel_A"
    name: "Flexible Boundary Gel A"
    category: "boundary"
    density: 1.0
    volume_per_unit: 1.0
    stability: 0.80
    repairability: 0.70
    synthesis_cost:
      resources:
        nutrient_A: 1.0
        mineral_A: 0.2
      energy: 0.3

    physical:
      strength: 0.40
      elasticity: 0.70
      permeability: 0.30
      heat_resistance: 0.50
      pressure_resistance: 0.40

    functional:
      boundary_support: 0.80
      storage_capacity: 0.10
      energy_conversion_efficiency: 0.0
      signal_conductivity: 0.05
      joint_affinity: 0.30
```

Це приклад формату, а не остаточна схема.

---

# Material Fields

## id

Унікальний технічний ідентифікатор.

```yaml id="xjqf88"
id: "boundary_gel_A"
```

На нього можуть посилатися:

* Genome outputs;
* processes;
* synthesis recipes;
* Joint creation;
* Boundary;
* debug logs;
* save files.

---

## name

Людська назва для UI.

```yaml id="4o94xe"
name: "Flexible Boundary Gel A"
```

Не повинна використовуватись у логіці.

---

## category

Категорія для документації й UI.

```yaml id="v300qf"
category: "boundary"
```

Можливі категорії:

```text id="zkz4rz"
boundary
structural
storage
signal
energy_conversion
joint
movement
repair
generic
```

`category` не повинна бути hardcoded поведінкою.

Логіка має йти через числові властивості.

---

# Physical Properties

Фізичні властивості визначають, як Material поводиться у світі.

```yaml id="edm6q8"
physical:
  strength: 0.40
  elasticity: 0.70
  permeability: 0.30
  heat_resistance: 0.50
  pressure_resistance: 0.40
  density: 1.0
  volume_per_unit: 1.0
```

---

## strength

Міцність Material.

Впливає на:

* Boundary integrity;
* Joint strength;
* pressure resistance;
* damage resistance;
* structural support.

```yaml id="svkr13"
strength: 0.75
```

---

## elasticity

Еластичність Material.

Впливає на:

* deformation;
* movement;
* Joint flexibility;
* collision response;
* shape stability.

```yaml id="3vypnf"
elasticity: 0.60
```

---

## permeability

Проникність Material.

Впливає на:

* Resource uptake;
* leakage;
* transport;
* Boundary control;
* exposure to harmful reactions.

```yaml id="0vt28a"
permeability: 0.25
```

Висока permeability може бути корисною для uptake, але небезпечною для втрати внутрішніх Resources.

---

## heat_resistance

Стійкість до Heat.

```yaml id="fnc8xi"
heat_resistance: 0.70
```

Висока heat resistance зменшує damage від перегріву.

---

## pressure_resistance

Стійкість до Pressure.

```yaml id="v8mqf2"
pressure_resistance: 0.60
```

Впливає на виживання в середовищах з тиском, стисканням або collision.

---

# Functional Properties

Functional properties визначають, яку роль Material може виконувати.

```yaml id="9nt52f"
functional:
  boundary_support: 0.80
  storage_capacity: 0.10
  repair_support: 0.30
  energy_conversion_efficiency: 0.00
  signal_sensitivity: 0.05
  signal_conductivity: 0.05
  joint_affinity: 0.30
  movement_support: 0.00
```

---

## boundary_support

Наскільки Material корисний для Boundary.

```yaml id="qlan74"
boundary_support: 0.80
```

Високе значення може допомагати:

* формувати Boundary;
* зменшувати leakage;
* підвищувати integrity;
* контролювати permeability.

---

## storage_capacity

Наскільки Material підтримує зберігання Resources.

```yaml id="6li6x8"
storage_capacity: 0.60
```

Storage Material може збільшувати внутрішню capacity, але має volume і cost.

---

## repair_support

Наскільки Material допомагає repair.

```yaml id="24cm9z"
repair_support: 0.50
```

Це може впливати на швидкість або ефективність ремонту Boundary, Joint або інших Materials.

---

## energy_conversion_efficiency

Наскільки Material допомагає перетворювати Resources з `energy_value` в Energy Buffer.

```yaml id="r6wsig"
energy_conversion_efficiency: 0.65
```

Material сам не створює Energy.

Потрібні:

```text id="i0ueph"
Resource with energy_value
+
conversion-capable Material
+
process
+
conditions
    ↓
Energy Buffer
```

---

## field_sensitivity

Чутливість Material до Fields.

```yaml id="x0e1cb"
field_sensitivity:
  light: 0.70
  heat: 0.20
  pressure: 0.10
```

Наприклад, light-sensitive Material може дозволити клітині використовувати Light Field як джерело Energy або signal input.

---

## signal_sensitivity

Наскільки Material дозволяє клітині сприймати сигнали.

```yaml id="qls93c"
signal_sensitivity: 0.80
```

Signal sensitivity може впливати на Genome Runtime inputs.

---

## signal_conductivity

Наскільки Material проводить сигнали.

```yaml id="nwqe0d"
signal_conductivity: 0.75
```

Це важливо для:

* Joint signals;
* communication;
* neural-like behavior;
* impulse-like pathways.

---

## joint_affinity

Наскільки Material допомагає створювати Joint.

```yaml id="p95t3r"
joint_affinity: 0.60
```

Висока `joint_affinity` не створює Joint автоматично.

Потрібні контакт, Energy, Resources і процес створення Joint.

---

## movement_support

Наскільки Material підтримує movement або force generation.

```yaml id="8mrkhj"
movement_support: 0.40
```

Це не hardcoded muscle.

Muscle-like behavior може виникнути лише через Material properties, Joint, Energy і selection.

---

# Stateful Properties

Materials можуть мати стан.

Це потрібно для learning-like behavior, адаптації до сигналів і memory-like effects.

```yaml id="ba7xu4"
stateful:
  enabled: true
  accumulation_rate: 0.10
  decay_rate: 0.02
  plasticity: 0.30
  memory_stability: 0.40
  threshold_modifier: 0.20
  gain_modifier: 0.10
```

Stateful Material може:

* накопичувати signal;
* змінювати gain;
* змінювати threshold;
* підтримувати epigenetic effect;
* зберігати коротку історію стану клітини.

Це не окремий learning module.

---

# Degradation

Material може деградувати.

```yaml id="cx83lx"
degradation:
  base_decay_rate: 0.001
  heat_multiplier: 2.0
  pressure_multiplier: 1.5
  reaction_multiplier: 1.2
  damage_products:
    - resource_id: "inert_waste"
      ratio: 0.5
```

Degradation означає втрату матеріальної структури.

Це не HP damage.

---

# Repair

Material може мати repair properties.

```yaml id="jq154d"
repair:
  repairability: 0.70
  required_resources:
    nutrient_A: 0.5
    mineral_A: 0.1
  energy_cost: 0.2
```

Repair не повинен магічно відновлювати клітину.

Він відновлює конкретний Material за наявності Resources і Energy.

---

# Synthesis Cost

Material синтезується з Resources.

```yaml id="ykc5td"
synthesis_cost:
  resources:
    nutrient_A: 1.0
    mineral_A: 0.2
  energy: 0.3
  time_ticks: 5
```

Синтез не повинен бути безкоштовним.

Більш складні або сильні Materials мають мати більший cost.

---

# Reaction Profile

Material може реагувати з Resources або іншими Materials.

```yaml id="ctujd9"
reaction_profile:
  reacts_with_resources:
    - resource_id: "oxidizer_A"
      damage_rate: 0.05
      heat_release: 0.10
      product_resources:
        inert_waste: 0.4
```

Так можна моделювати harmful effects без поля `toxicity`.

---

# Boundary Materials

Boundary-like Material має високі:

```text id="oez8lu"
boundary_support
strength
permeability control
repairability
```

Приклад:

```yaml id="lbv7c3"
id: "boundary_gel_A"
category: "boundary"
physical:
  strength: 0.45
  elasticity: 0.70
  permeability: 0.30
  heat_resistance: 0.50
functional:
  boundary_support: 0.85
  joint_affinity: 0.25
```

Це не hardcoded membrane.

Це Material, який добре працює в Boundary.

---

# Structural Materials

Structural-like Material має високі:

```text id="s6qcb9"
strength
pressure_resistance
stability
```

Приклад:

```yaml id="zcus7a"
id: "structural_fiber_A"
category: "structural"
physical:
  strength: 0.85
  elasticity: 0.35
  pressure_resistance: 0.80
  heat_resistance: 0.50
functional:
  boundary_support: 0.30
  joint_affinity: 0.60
  movement_support: 0.10
```

Це може підтримувати shell-like або support-like структури.

---

# Signal Materials

Signal-like Material має високі:

```text id="mdd9ao"
signal_sensitivity
signal_conductivity
statefulness
plasticity
```

Приклад:

```yaml id="j4o5wt"
id: "signal_polymer_A"
category: "signal"
physical:
  strength: 0.20
  elasticity: 0.50
  permeability: 0.20
functional:
  signal_sensitivity: 0.80
  signal_conductivity: 0.75
  joint_affinity: 0.30
stateful:
  enabled: true
  accumulation_rate: 0.15
  decay_rate: 0.04
  plasticity: 0.30
```

Це може підтримувати neural-like поведінку без hardcoded neurons.

---

# Energy-conversion Materials

Energy-conversion-like Material має високі:

```text id="6sf4bn"
energy_conversion_efficiency
field_sensitivity
heat_resistance
```

Приклад:

```yaml id="krd4oa"
id: "light_pigment_A"
category: "energy_conversion"
physical:
  strength: 0.20
  heat_resistance: 0.60
functional:
  energy_conversion_efficiency: 0.60
  field_sensitivity:
    light: 0.80
```

Material сам не виробляє Energy.

Він дозволяє процесу перетворення за наявності Field або Resource.

---

# Joint Materials

Joint-like Material має високі:

```text id="393oqu"
joint_affinity
strength
elasticity
signal_conductivity
resource_transport_support
```

Приклад:

```yaml id="e4gs99"
id: "adhesive_polymer_A"
category: "joint"
physical:
  strength: 0.60
  elasticity: 0.65
  permeability: 0.20
functional:
  joint_affinity: 0.85
  signal_conductivity: 0.20
  resource_transport_support: 0.40
```

Це не hardcoded Joint.

Це Material, який допомагає створити Joint.

---

# Storage Materials

Storage-like Material має високі:

```text id="x1ujkr"
storage_capacity
stability
low permeability
```

Приклад:

```yaml id="97tfne"
id: "storage_matrix_A"
category: "storage"
physical:
  strength: 0.30
  permeability: 0.10
functional:
  storage_capacity: 0.90
  boundary_support: 0.10
```

Storage має cost і volume.

---

# Movement Materials

Movement-like Material може підтримувати force generation або deformation.

```yaml id="238f2w"
id: "contractile_fiber_A"
category: "movement"
physical:
  strength: 0.50
  elasticity: 0.80
functional:
  movement_support: 0.75
  joint_affinity: 0.30
```

Це не muscle class.

Movement виникає через:

* Material;
* Energy;
* Joint;
* physics;
* Genome Runtime;
* selection.

---

# MVP Material Fields

Для MVP достатньо:

```yaml id="2ox1nj"
id: "boundary_gel_A"
density: 1.0
volume_per_unit: 1.0
stability: 0.8

synthesis_cost:
  resources:
    nutrient_A: 1.0
  energy: 0.2

physical:
  strength: 0.4
  elasticity: 0.6
  permeability: 0.3
  heat_resistance: 0.5

functional:
  boundary_support: 0.8
  storage_capacity: 0.1
  energy_conversion_efficiency: 0.0
  signal_sensitivity: 0.1
  signal_conductivity: 0.1
  joint_affinity: 0.2
```

Обов’язкові поля MVP:

```text id="gqwmyk"
id
density
volume_per_unit
stability
synthesis_cost
physical.strength
physical.elasticity
physical.permeability
physical.heat_resistance
functional.boundary_support
functional.storage_capacity
functional.energy_conversion_efficiency
functional.signal_sensitivity
functional.signal_conductivity
functional.joint_affinity
```

---

# Example Minimal Material Set

Для early-world MVP можна почати з 5–6 Materials.

```yaml id="mhiqre"
materials:
  - id: "boundary_gel_A"
    category: "boundary"
    density: 1.0
    volume_per_unit: 1.0
    stability: 0.80
    synthesis_cost:
      resources:
        nutrient_A: 1.0
      energy: 0.2
    physical:
      strength: 0.45
      elasticity: 0.70
      permeability: 0.30
      heat_resistance: 0.50
    functional:
      boundary_support: 0.85
      storage_capacity: 0.10
      energy_conversion_efficiency: 0.00
      signal_sensitivity: 0.10
      signal_conductivity: 0.05
      joint_affinity: 0.25

  - id: "energy_pigment_A"
    category: "energy_conversion"
    density: 0.8
    volume_per_unit: 0.7
    stability: 0.60
    synthesis_cost:
      resources:
        nutrient_A: 0.8
        mineral_A: 0.3
      energy: 0.3
    physical:
      strength: 0.20
      elasticity: 0.30
      permeability: 0.10
      heat_resistance: 0.45
    functional:
      boundary_support: 0.05
      storage_capacity: 0.00
      energy_conversion_efficiency: 0.70
      signal_sensitivity: 0.10
      signal_conductivity: 0.05
      joint_affinity: 0.05
      field_sensitivity:
        light: 0.70

  - id: "signal_polymer_A"
    category: "signal"
    density: 1.0
    volume_per_unit: 0.8
    stability: 0.65
    synthesis_cost:
      resources:
        nutrient_A: 0.6
        mineral_A: 0.2
      energy: 0.25
    physical:
      strength: 0.25
      elasticity: 0.50
      permeability: 0.20
      heat_resistance: 0.40
    functional:
      boundary_support: 0.10
      storage_capacity: 0.05
      energy_conversion_efficiency: 0.00
      signal_sensitivity: 0.80
      signal_conductivity: 0.70
      joint_affinity: 0.25
    stateful:
      enabled: true
      accumulation_rate: 0.12
      decay_rate: 0.04
      plasticity: 0.25

  - id: "adhesive_polymer_A"
    category: "joint"
    density: 1.1
    volume_per_unit: 1.0
    stability: 0.75
    synthesis_cost:
      resources:
        nutrient_A: 0.8
        mineral_A: 0.4
      energy: 0.25
    physical:
      strength: 0.60
      elasticity: 0.65
      permeability: 0.25
      heat_resistance: 0.45
    functional:
      boundary_support: 0.20
      storage_capacity: 0.05
      energy_conversion_efficiency: 0.00
      signal_sensitivity: 0.10
      signal_conductivity: 0.20
      joint_affinity: 0.85

  - id: "storage_matrix_A"
    category: "storage"
    density: 1.2
    volume_per_unit: 1.3
    stability: 0.90
    synthesis_cost:
      resources:
        nutrient_A: 1.2
      energy: 0.2
    physical:
      strength: 0.30
      elasticity: 0.20
      permeability: 0.05
      heat_resistance: 0.55
    functional:
      boundary_support: 0.05
      storage_capacity: 0.90
      energy_conversion_efficiency: 0.00
      signal_sensitivity: 0.05
      signal_conductivity: 0.05
      joint_affinity: 0.05
```

---

# Climate Modifiers

Materials можуть змінювати властивості під впливом середовища.

```yaml id="uysl88"
environment_modifiers:
  heat:
    stability_multiplier: 0.6
    elasticity_multiplier: 1.2

  pressure:
    damage_multiplier: 1.5

  radiation:
    degradation_multiplier: 2.0
```

Це дозволяє клімату, погоді й катастрофам впливати на еволюцію через Materials.

---

# Material Compatibility

Деякі Materials можуть краще або гірше працювати разом.

```yaml id="6hwunh"
compatibility:
  with_materials:
    - material_id: "boundary_gel_A"
      compatibility: 0.8
    - material_id: "oxidized_polymer_A"
      compatibility: 0.2
```

У MVP це можна не реалізовувати.

Але future compatibility важлива для складних клітин і Joint.

---

# Validation

Material Config має проходити validation.

Перевірки:

* `id` унікальний;
* `density >= 0`;
* `volume_per_unit > 0`;
* `stability` у межах `0.0..1.0`;
* усі functional properties у межах `0.0..1.0`;
* усі physical properties у межах `0.0..1.0`, якщо нормалізовані;
* `synthesis_cost.resources` посилається на існуючі resource_id;
* `damage_products` посилається на існуючі resource_id;
* немає NaN або infinite values;
* Material не має direct behavior script.

---

# Правила

## Rule 1. Materials are config-defined

Нові Materials додаються через config, а не через нові класи рушія.

## Rule 2. Material properties create function

Функція виникає з властивостей Material, а не з його назви.

## Rule 3. Material is not a cell type

Не можна створювати NeuronMaterial як готову роль.

Можна створити signal-conductive, stateful Material.

## Rule 4. Material has cost

Синтез, repair і підтримка Material мають коштувати Resources і Energy.

## Rule 5. Material can degrade

Damage — це втрата або зміна Materials, а не HP.

## Rule 6. Materials enable emergence

Boundary, Joint, signal, storage, movement і energy conversion повинні виникати з Material properties.

---

# Заборонено

Не вводити:

* hardcoded membrane class;
* hardcoded neuron material;
* hardcoded muscle material;
* hardcoded skin material;
* hardcoded organ material;
* healing material;
* poison material with toxicity field;
* direct Energy material;
* direct HP repair;
* species-specific materials;
* behavior script inside Material config.

---

# Пов'язані документи

* `config/world_config.md`
* `config/resources_config.md`
* `config/fields_config.md`
* `world/materials.md`
* `world/resources.md`
* `world/energy.md`
* `world/fields.md`
* `biology/cell.md`
* `biology/membrane.md`
* `biology/joint.md`
* `biology/communication.md`
* `biology/specialization.md`

---

# Open Questions

## MVP property set

Потрібно остаточно затвердити мінімальний набір Material properties для MVP.

## Stateful Materials

Потрібно вирішити, чи stateful Materials входять у MVP або post-MVP.

## Movement Materials

Потрібно визначити, чи movement_support входить у MVP.

## Compatibility

Потрібно вирішити, чи Material compatibility потрібна одразу.

## Units

Потрібно визначити умовні одиниці для density, volume, amount і synthesis cost.

## Config schema

Потрібно створити формальну schema validation для Materials.
