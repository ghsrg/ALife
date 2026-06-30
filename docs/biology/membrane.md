---
tags:
  - alife
  - canon
  - area/biology
---

# membrane.md

> `Boundary` — матеріальна межа клітини.

---

# Призначення

Boundary відокремлює внутрішній стан клітини від середовища і визначає permeability, protection, contact, sensing і Joint formation.

Термін `Boundary` використовується як нейтральніший за реальну біологічну `Membrane`.

У базовій моделі `Boundary` не є окремою сутністю рушія або component. Це назва для агрегованої властивості Materials клітини, яка потрібна, щоб описувати межу клітини без введення спеціального hardcoded об'єкта.

---

# Канонічні правила

- Boundary виникає з Materials клітини.
- Boundary є derived aggregate від Materials, а не окремий component.
- Boundary не є магічним фільтром або окремим органом.
- Boundary не приймає рішень; Genome Runtime регулює процеси, а Boundary фізично дозволяє або обмежує їх.
- Boundary integrity є частиною cell viability.
- Boundary damage є material/physical state change, а не HP damage.
- Permeability залежить від Materials, damage, gradients, pumps, Joint і regulation.
- Pump behavior є process + material capability, а не окрема магічна сутність.
- No Resource crosses Boundary by default.
- Resource exchange requires Boundary permeability rule: `blocked`, `passive` або `active_required`.
- Boundary damage increases leakage and reduces control, but does not make all Resources freely pass.

---

# Мінімальні Властивості

```text
integrity
default_permeability: blocked
permeability_by_resource_class
permeability_by_resource_id
leakage_rate
uptake_modifier
export_modifier
failure_thresholds
strength
heat_tolerance
pressure_tolerance
repairability
joint_affinity
sensing_capabilities
```

У першій реалізації Boundary є агрегованою властивістю Materials клітини.

Permeability визначається не користю Resource для клітини, а фізико-матеріальним правилом:

```text
Resource physical traits + Boundary Material -> permeability rule
```

Базові правила:

- `blocked`: Resource не проходить через нормальну Boundary.
- `passive`: Resource може проходити за gradient/diffusion без Genome decision.
- `active_required`: Resource може пройти тільки через uptake/export process, якщо є Material capability, Energy і Feasibility.

Boundary Material має `default_permeability: blocked`. Override за конкретним Resource id має пріоритет над правилом resource class.

```text
boundary_state
├── integrity: 0.0..1.0
├── default_permeability: blocked
├── permeability_by_resource_class
├── permeability_by_resource_id
├── leakage_rate
├── uptake_modifier
├── export_modifier
└── failure_thresholds
    ├── leakage_threshold
    ├── uncontrolled_exchange_threshold
    └── death_threshold
```

Boundary damage:

- знижує `integrity`;
- підвищує `leakage_rate`;
- погіршує `uptake_modifier` і `export_modifier`;
- може збільшити passive leakage для tiny/small physically compatible Resources;
- не робить великі fragments, genetic fragments або несумісні Resources вільно прохідними;
- reactive/corrosive Resources можуть пошкоджувати Boundary ззовні через reaction/material degradation rules.

Якщо `integrity < leakage_threshold`, починається leakage для фізично сумісних passive Resources.

Якщо `integrity < uncontrolled_exchange_threshold`, частина Resources, які раніше були `blocked` або `active_required`, може неконтрольовано просочуватись тільки в межах фізично можливих damage classes.

Якщо `integrity < death_threshold`, Lifecycle може перевести клітину в `inert`, `death` або `decomposing` згідно з `biology/lifecycle.md`.

---

# Заборонено

Не вводити:

- absolute barrier;
- free selective transport;
- HP repair;
- species marker;
- contact sensing without material basis;
- Joint formation without Boundary/Material basis.

# Semantic Links

- bounds: [[docs/biology/cell|Cell]]
- made from: [[docs/world/materials|Materials]]
- controls uptake of: [[docs/world/resources|Resources]]
- enables contact with: [[docs/biology/joint|Joint]]
- affects: [[docs/biology/lifecycle|Lifecycle]]

# Пов'язані документи

- `biology/cell.md`
- `biology/processes.md`
- `biology/joint.md`
- `world/materials.md`
- `world/resources.md`
- `biology/process-capabilities.md`
