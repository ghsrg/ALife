# resources_config.md

> **Resources Config — конфігурація речовин, які можуть рухатися, накопичуватися, реагувати і ставати сировиною для життя**

---

# Призначення

`resources_config.md` описує, як задавати типи Resources без зміни рушія.

Resource — це рухома або накопичувана речовина світу.

Resources можуть:

* існувати в середовищі;
* потрапляти в клітину;
* займати об’єм;
* дифундувати;
* реагувати;
* деградувати;
* бути сировиною для Materials;
* мати `energy_value`;
* бути waste;
* бути продуктом розпаду клітин;
* створювати gradients і traces.

Resource не є функцією.

Resource не є Material.

Resource не є Energy Buffer.

---

# Основна ідея

Новий світ повинен мати змогу визначити власний набір Resources.

Наприклад:

```text id="1p8sdm"
simple_nutrient
mineral_A
reactive_compound
inert_waste
energy_rich_resource
boundary_precursor
signal_trace_resource
```

Рушій не повинен знати наперед, що таке “цукор”, “кисень”, “вода” або “токсин”.

Він має працювати з універсальними параметрами.

---

# Що Resource Config НЕ є

Resource Config не повинен описувати:

* готову їжу;
* отруту як окремий магічний тип;
* ліки;
* органи;
* поведінку клітин;
* fitness;
* hardcoded корисність;
* пряме створення Energy;
* пряме пошкодження HP.

Якщо Resource шкідливий, це має виникати через:

* реактивність;
* зайнятий об’єм;
* blocking;
* degradation products;
* Heat;
* Material damage;
* interference with reactions.

---

# Загальна структура

Приклад:

```yaml id="l0un77"
resources:
  - id: "nutrient_A"
    name: "Simple Nutrient A"
    phase: "dissolved"
    density: 1.0
    volume_per_unit: 1.0
    diffusion_rate: 0.30
    stability: 0.80
    decay_rate: 0.001
    energy_value: 0.60
    reactivity_class: "organic_like"
    tags:
      - "nutrient"
      - "energy_source"

  - id: "mineral_A"
    name: "Mineral A"
    phase: "solid_particle"
    density: 2.5
    volume_per_unit: 1.4
    diffusion_rate: 0.05
    stability: 0.95
    decay_rate: 0.0001
    energy_value: 0.0
    reactivity_class: "mineral_like"
    tags:
      - "material_precursor"
```

---

# Resource Fields

## id

Унікальний технічний ідентифікатор.

```yaml id="lwgt3n"
id: "nutrient_A"
```

Повинен бути стабільним, бо на нього посилаються:

* world_config;
* materials_config;
* reactions;
* cell processes;
* debug logs.

---

## name

Людська назва для UI і документації.

```yaml id="tjzuyg"
name: "Simple Nutrient A"
```

Не повинна використовуватись у логіці.

---

## phase

Фаза або форма існування Resource.

```yaml id="huvrkq"
phase: "dissolved"
```

Можливі значення:

```text id="l71j9r"
dissolved
solid_particle
gas_like
liquid_like
gel_like
fragment
trace
```

Це не обов’язково реальна фізика.

Це спрощена категорія для симуляції.

---

## density

Впливає на масу Resource.

```yaml id="4jwsdl"
density: 1.0
```

Маса може обчислюватися так:

```text id="wolb1e"
mass = amount × density
```

---

## volume_per_unit

Визначає, скільки внутрішнього або зовнішнього об’єму займає одиниця Resource.

```yaml id="0t4lor"
volume_per_unit: 1.0
```

Це важливо для:

* clogging;
* free_capacity;
* cell growth;
* storage;
* internal pressure.

---

## diffusion_rate

Швидкість пасивного поширення Resource у середовищі.

```yaml id="6y9x84"
diffusion_rate: 0.30
```

Більше значення — Resource швидше розмивається.

Менше значення — Resource довше лишається локальним.

---

## stability

Стійкість Resource до розпаду.

```yaml id="36mph4"
stability: 0.80
```

Висока stability означає, що Resource довго існує.

Низька stability означає швидкий decay або реакції.

---

## decay_rate

Базова швидкість природного розпаду за Tick.

```yaml id="xbedbw"
decay_rate: 0.001
```

Decay може створювати інші Resources.

```yaml id="ipwqox"
decay_products:
  - resource_id: "inert_waste"
    ratio: 0.7
```

---

## energy_value

Потенційна енергетична цінність Resource.

```yaml id="ecji2i"
energy_value: 0.60
```

`energy_value` не означає, що Resource автоматично стає Energy.

Для перетворення потрібні:

* Material;
* reaction;
* умови;
* Energy Buffer capacity;
* cell process.

```text id="7d24p9"
Resource with energy_value
+
Energy-conversion Material
+
Process
    ↓
Energy Buffer
```

---

## reactivity_class

Узагальнений клас реактивності.

```yaml id="llpxtn"
reactivity_class: "oxidizer_like"
```

Можливі приклади:

```text id="ebq11n"
inert
organic_like
mineral_like
oxidizer_like
reducer_like
acid_like
base_like
reactive
unstable
genetic_fragment_like
```

Це не повинно перетворюватися на hardcoded chemistry.

Це лише простий спосіб описати реакції.

---

## tags

Tags потрібні для людини, debug UI або простого групування.

```yaml id="zowzsw"
tags:
  - "nutrient"
  - "energy_source"
```

Tags не повинні напряму визначати поведінку клітин.

Клітина не повинна читати:

```text id="6kwput"
tag == nutrient
```

Вона реагує на Resource через Materials, Genome inputs і reactions.

---

# Reaction Profile

Resource може мати `reaction_profile`.

```yaml id="vuj4zs"
reaction_profile:
  reacts_with:
    - resource_id: "oxidizer_A"
      probability: 0.4
      products:
        - resource_id: "waste_A"
          ratio: 0.5
      energy_release: 0.3
      heat_release: 0.1
      material_damage:
        target_property: "stability"
        amount: 0.05
```

Reaction Profile описує потенційні реакції.

Реакція не повинна автоматично відбуватися завжди.

Вона залежить від:

* концентрації;
* proximity;
* Materials;
* temperature / Heat;
* Field conditions;
* process rules;
* stochastic seed.

---

# Resource as Material Precursor

Resource може бути сировиною для Material.

Наприклад:

```yaml id="8k0v2n"
material_precursor:
  can_build:
    - material_id: "boundary_material_A"
      efficiency: 0.7
    - material_id: "storage_material_A"
      efficiency: 0.4
```

Але сам Resource не виконує функцію Material.

Він лише може бути перетворений.

---

# Resource Sources

Джерела Resources задаються переважно у `world_config.md`.

Але Resource Config може задавати default source behavior.

```yaml id="z7yaql"
default_source:
  enabled: false
  spawn_rate: 0.0
```

Приклад world-specific source:

```yaml id="76ldj3"
resource_sources:
  - resource_id: "nutrient_A"
    area:
      x: 100
      y: 200
      radius: 50
    rate: 0.2
```

---

# Resource Sinks

Resource може зникати через:

* decay;
* reactions;
* uptake by cells;
* transformation into Materials;
* leaving open boundary;
* degradation into inert remains.

```yaml id="0jd1vk"
sinks:
  decay: true
  uptake: true
  reactions: true
  open_boundary_loss: false
```

---

# Waste Resources

Waste — це не окремий магічний клас.

Waste Resource — це Resource, який має низьку корисність або створює проблему.

Приклади причин:

* займає volume;
* погано дифундує;
* блокує free_capacity;
* має низький energy_value;
* має reactive products;
* заважає Material repair;
* має високу stability і накопичується.

```yaml id="zd99z7"
id: "inert_waste"
energy_value: 0.0
diffusion_rate: 0.05
stability: 0.95
volume_per_unit: 1.2
reactivity_class: "inert"
```

---

# Signal-like Resources

Деякі Resources можуть працювати як сліди або сигнали.

```yaml id="i3v6si"
id: "trace_signal_A"
phase: "trace"
diffusion_rate: 0.20
stability: 0.40
decay_rate: 0.01
energy_value: 0.0
tags:
  - "trace"
  - "signal_like"
```

Це не hardcoded pheromone.

Це Resource або trace, який інші клітини можуть відчувати, якщо мають відповідні Materials.

---

# Genetic Fragment Resource

Для HGT у майбутньому genetic fragments можуть бути окремими physical objects або resource-like packets.

```yaml id="40j0am"
id: "genetic_fragment"
phase: "fragment"
density: 1.2
volume_per_unit: 0.5
diffusion_rate: 0.02
stability: 0.30
decay_rate: 0.02
energy_value: 0.0
reactivity_class: "genetic_fragment_like"
tags:
  - "genetic_fragment"
  - "hgt"
```

Це future-compatible варіант.

У базовій моделі можна не використовувати.

---

# Climate Modifiers

Resource може реагувати на клімат або Fields.

```yaml id="ny8yqr"
environment_modifiers:
  heat:
    decay_multiplier: 2.0
    reaction_multiplier: 1.5

  light:
    decay_multiplier: 1.2

  pressure:
    diffusion_multiplier: 0.8
```

Це дозволяє сценаріям погоди, клімату і катастроф змінювати хімію світу.

---

# Example: Early World Resources

Приклад набору для простого early-world сценарію:

```yaml id="7mdnul"
resources:
  - id: "nutrient_A"
    phase: "dissolved"
    density: 1.0
    volume_per_unit: 1.0
    diffusion_rate: 0.30
    stability: 0.70
    decay_rate: 0.002
    energy_value: 0.50
    reactivity_class: "organic_like"

  - id: "mineral_A"
    phase: "solid_particle"
    density: 2.0
    volume_per_unit: 1.2
    diffusion_rate: 0.04
    stability: 0.95
    decay_rate: 0.0001
    energy_value: 0.0
    reactivity_class: "mineral_like"

  - id: "oxidizer_A"
    phase: "dissolved"
    density: 1.1
    volume_per_unit: 1.0
    diffusion_rate: 0.25
    stability: 0.65
    decay_rate: 0.003
    energy_value: 0.0
    reactivity_class: "oxidizer_like"

  - id: "inert_waste"
    phase: "dissolved"
    density: 1.0
    volume_per_unit: 1.1
    diffusion_rate: 0.10
    stability: 0.95
    decay_rate: 0.0002
    energy_value: 0.0
    reactivity_class: "inert"
```

---

# базова модель Resource Fields

Для базової моделі достатньо:

```yaml id="5hmgn2"
id: "nutrient_A"
density: 1.0
volume_per_unit: 1.0
diffusion_rate: 0.3
stability: 0.8
decay_rate: 0.001
energy_value: 0.5
reactivity_class: "organic_like"
```

Обов’язкові поля:

```text id="v0fdev"
id
density
volume_per_unit
diffusion_rate
stability
decay_rate
energy_value
```

Опційні:

```text id="xd6ws0"
name
phase
reactivity_class
reaction_profile
decay_products
material_precursor
environment_modifiers
tags
```

---

# Validation

Resource Config має проходити validation.

Перевірки:

* `id` унікальний;
* `density >= 0`;
* `volume_per_unit > 0`;
* `diffusion_rate >= 0`;
* `stability` у межах `0.0..1.0`;
* `decay_rate >= 0`;
* `energy_value >= 0`;
* `reaction_profile` посилається на існуючі resource_id;
* `material_precursor` посилається на існуючі material_id;
* немає NaN або infinite values.

---

# Правила

## Rule 1. Resources are config-defined

Нові Resources додаються через config, а не через зміну engine.

## Rule 2. Resource is not Material

Resource може стати сировиною для Material, але не виконує функцію Material напряму.

## Rule 3. Resource is not Energy

`energy_value` — це потенціал, а не Energy Buffer.

## Rule 4. No toxicity field

Не вводити `toxicity`.

Шкідливість має виникати через реакції, об’єм, blocking, Heat, Material damage або decay products.

## Rule 5. Resource behavior is physical

Diffusion, decay, reactions і uptake повинні йти через універсальні правила.

## Rule 6. Tags are analytical

Tags не повинні бути поведінковою логікою клітини.

---

# Заборонено

Не вводити:

* toxicity;
* poison_damage;
* healing_resource;
* food as hardcoded category;
* direct Energy creation;
* direct HP damage;
* species-specific resource;
* plant-only або animal-only resource;
* behavior based on tag;
* resource that bypasses Materials and Processes.

---

# Пов'язані документи

* `config/world_config.md`
* `config/materials_config.md`
* `config/fields_config.md`
* `world/resources.md`
* `world/materials.md`
* `world/energy.md`
* `world/fields.md`
* `biology/cell.md`
* `biology/processes.md`

---

# Open Questions

## Reaction model

Потрібно вирішити, наскільки складними будуть reactions У базовій моделі.

## Resource phase

Потрібно визначити, чи `phase` впливає на physics У базовій моделі, чи лише на diffusion.

## Genetic fragments

Потрібно вирішити, чи genetic fragments будуть Resource-like, Material-like або окремою сутністю.

## Signal traces

Потрібно визначити, чи pheromone-like traces будуть Resources, Materials або окремий trace layer.

## Resource source config

Потрібно вирішити, чи джерела Resources повністю описуються у `world_config.md`, чи частково в `resources_config.md`.

## Units

Потрібно визначити умовні одиниці для amount, volume і density.


