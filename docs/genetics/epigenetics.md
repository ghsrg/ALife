# epigenetics.md

> `Epigenetics` — успадковуваний або довший регуляторний bias без зміни Genome.

---

# Призначення

Epigenetic State змінює те, як Genome Runtime інтерпретує inputs і produces outputs, але не переписує Genome.

---

# Канонічні правила

- Epigenetic State не є mutation.
- Epigenetic State може бути частково inherited during division.
- Epigenetic State може decay-итися або оновлюватися локальними signals, stress, Materials, Heat, damage, lifecycle.
- Epigenetic modifiers впливають на runtime parameters: thresholds, gains, priorities, sensitivity, dormancy/growth/repair bias.
- Epigenetic State має bounds і не може обійти Feasibility Check.

---

# Мінімальна Структура

```text
EpigeneticState
├── process_biases
├── input_sensitivity_modifiers
├── output_gain_modifiers
├── dormancy_bias
├── stress_memory
├── specialization_bias
└── decay_rates
```

---

# Sources

```text
repeated signals
resource scarcity
Heat / Pressure
damage
successful or failed processes
Joint context
lifecycle state
asymmetric inheritance
```

---

# Заборонено

Не вводити:

- epigenetic changes as hidden mutation;
- permanent unbounded modifiers;
- direct creation of capabilities;
- bypass of material basis;
- organism-level role assignment.

---

# Пов'язані документи

- `genetics/genome-runtime.md`
- `genetics/inheritance.md`
- `biology/specialization.md`
- `biology/communication.md`
- `biology/cell-state.md`
