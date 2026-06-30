# mutation.md

> `Mutation` — спадкова зміна Genome або genetic fragment.

---

# Призначення

Mutation створює варіативність у спадковому регуляторному носії. Вона не є адаптивним рішенням клітини і не повинна знати, що буде корисним.

---

# Канонічні правила

- Mutation змінює Genome або genetic fragment.
- Epigenetic або runtime зміни не є mutation.
- Mutation може бути шкідливою, нейтральною або корисною тільки через selection.
- Mutation має rates, bounds і validation.
- Невалідний Genome може бути rejected, repaired, inert або lethal згідно з rules.
- Mutation не повинна створювати behavior script або global inputs.

---

# Operators

Мінімальні operators:

```text
weight_shift
bias_shift
edge_enable_disable
edge_add
edge_delete
node_parameter_shift
output_binding_shift_with_validation
```

Future operators:

```text
duplication
fragment insertion
fragment deletion
recombination
HGT integration
```

---

# Validation

Після mutation перевіряється:

```text
graph size limits
acyclic/recurrent rules
input/output binding validity
numeric bounds
required outputs
runtime computability
material/capability compatibility
```

Technical validity не означає життєздатність.

---

# Заборонено

Не вводити:

- directed mutation toward need;
- mutation based on future fitness;
- unlimited genome growth;
- invalid output bindings;
- species-specific mutation rules;
- mutation that bypasses Feasibility Check.

---

# Пов'язані документи

- `genetics/genome-representation.md`
- `genetics/regulatory-network.md`
- `genetics/inheritance.md`
- `genetics/recombination.md`
- `genetics/horizontal-transfer.md`
- `docs/examples/genetics-examples.md`
