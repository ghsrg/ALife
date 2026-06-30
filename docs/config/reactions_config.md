# reactions_config.md

> Конфігурація passive і controlled reactions.

---

# Призначення

`reactions_config` описує перетворення Resources/Materials, побічні продукти, Heat і Energy production pathways.

---

# Мінімальна Схема

```yaml
reactions:
  glucose_to_energy:
    mode: controlled
    inputs:
      glucose: 1.0
    required_materials:
      catalyst: 0.2
    outputs:
      waste: 0.5
    energy_output: 3.0
    heat_output: 0.2
    rate: 1.0
    accounting:
      residual:
        waste: 0.5
      configured_sink: 0.0
```

---

# Канонічні правила

- Reactions є джерелом корисності/шкідливості речовин.
- Немає `toxicity`; є реакції, Heat, volume, degradation і capacity effects.
- Controlled reaction потребує process/capability/regulation.
- Passive reaction може відбуватися без Genome, якщо умови виконані.
- Energy output не може з'явитися без визначеної reaction/process.
- Reaction має explicit material/amount accounting.
- `energy_output` не замінює material outputs.
- Configured sink/loss дозволений тільки якщо описаний явно.
- MaterialFragment може стати Resource тільки через explicit degradation/reaction/conversion rule.

---

# Validation

```text
known input resources/materials
non-negative stoichiometry
known outputs
explicit accounting for input matter
warning for unbalanced input/output accounting
warning for input matter without explicit destination
fatal when products exist without inputs
known MaterialFragment conversion source when fragment is consumed
energy_output >= 0
heat_output >= 0
rate >= 0
known mode
```

---

# Пов'язані документи

- `world/reactions.md`
- `world/resources.md`
- `world/materials.md`
- `world/energy.md`
- `engine/chemistry.md`
- `docs/examples/config-examples.md`
