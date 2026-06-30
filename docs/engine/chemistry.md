# chemistry.md

> Chemistry system — виконання reactions і material/resource transformations.

---

# Призначення

Chemistry реалізує `world/reactions.md` і `docs/config/reactions_config.md`.

Вона не вирішує поведінку клітини; вона застосовує passive reactions і controlled reactions, які пройшли process/feasibility pipeline.

---

# Канонічні правила

- Немає окремої `toxicity`.
- Шкідливість виникає через reactions, Heat, volume, Material degradation або capacity.
- Passive reactions можуть відбуватися без Genome.
- Controlled reactions потребують process/capability/regulation.
- Energy output і Heat output мають бути визначені reaction/process.
- Material/amount accounting має бути явним.
- `energy_output` не пояснює missing matter.
- Configured sink/loss має бути явним.

---

# Мінімальні Обов'язки

```text
match reaction inputs
apply rates
consume inputs
produce outputs
update Heat/Energy where allowed
apply Material degradation
validate configured sink/loss
emit trace/debug data
```

---

# Заборонено

Не вводити:

- magic detox;
- poison damage shortcut;
- free Energy;
- reaction without configured mechanism;
- hidden resource deletion.
- products without material source.

---

# Пов'язані документи

- `world/reactions.md`
- `world/resources.md`
- `world/materials.md`
- `world/energy.md`
- `docs/config/reactions_config.md`
