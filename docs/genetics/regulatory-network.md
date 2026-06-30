# regulatory-network.md

> `Regulatory Network` — графова структура регуляції процесів клітини.

---

# Призначення

Regulatory Network описує внутрішню структуру Direct Regulatory Graph: nodes, edges, weights, activation і output bindings.

Він визначає, як normalized local inputs перетворюються на process priorities.

---

# Мінімальна структура

```text
RegulatoryGraph
├── input_nodes
├── internal_nodes
├── output_nodes
├── edges
├── activation_functions
└── topology_limits
```

Edge:

```text
source
target
weight
bias
enabled
```

---

# Канонічні правила

- Graph працює тільки з локальними inputs.
- Output не є action; це priority/request.
- Feasibility Check завжди сильніший за output.
- Missing material basis робить відповідний input `0.0` або `unavailable` згідно з contract.
- Topology має limits, щоб mutation не створювала нескінченний або нерозраховний graph.
- Cycles/recurrent behavior не є базовою вимогою; якщо додаються, потрібні bounded runtime rules.

---

# Output Bindings

Output nodes мають бути прив'язані до allowed Genome outputs з `biology/action-process-registry.md`.

Genome output names must not be invented in genetics documents without adding a registry entry.

---

# Заборонено

Не вводити:

- hidden global inputs;
- direct action execution from graph;
- organism-level outputs;
- species-specific outputs;
- unbounded graph growth;
- hardcoded cell roles.

---

# Пов'язані документи

- `genetics/genome-representation.md`
- `genetics/genome-runtime.md`
- `genetics/regulatory-interface.md`
- `biology/processes.md`
- `biology/action-process-registry.md`
- `biology/process-capabilities.md`
