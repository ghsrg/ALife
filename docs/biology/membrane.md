# membrane.md

> `Boundary` — матеріальна межа клітини.

---

# Призначення

Boundary відокремлює внутрішній стан клітини від середовища і визначає permeability, protection, contact, sensing і Joint formation.

Термін `Boundary` використовується як нейтральніший за реальну біологічну `Membrane`.

---

# Канонічні правила

- Boundary виникає з Materials клітини.
- Boundary не є магічним фільтром або окремим органом.
- Boundary не приймає рішень; Genome Runtime регулює процеси, а Boundary фізично дозволяє або обмежує їх.
- Boundary integrity є частиною cell viability.
- Boundary damage є material/physical state change, а не HP damage.
- Permeability залежить від Materials, damage, gradients, pumps, Joint і regulation.
- Pump behavior є process + material capability, а не окрема магічна сутність.

---

# Мінімальні Властивості

```text
integrity
permeability_by_resource
strength
heat_tolerance
pressure_tolerance
repairability
joint_affinity
sensing_capabilities
```

У першій реалізації Boundary може бути агрегованою властивістю Materials клітини.

---

# Заборонено

Не вводити:

- absolute barrier;
- free selective transport;
- HP repair;
- species marker;
- contact sensing without material basis;
- Joint formation without Boundary/Material basis.

---

# Open Questions

- Чи Boundary буде окремим component або derived aggregate.
- Мінімальна модель permeability для першої реалізації.
- Як damage Boundary впливає на leakage, uptake і death threshold.

---

# Пов'язані документи

- `biology/cell.md`
- `biology/processes.md`
- `biology/joint.md`
- `world/materials.md`
- `world/resources.md`
- `biology/process-capabilities.md`
