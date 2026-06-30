# division-partition.md

> **Division Partition — фізичний розподіл локального стану однієї клітини**

---

# Призначення

`division-partition.md` описує, що відбувається зі станом клітини під час division.

Division не гарантує дві здорові клітини.

---

# Core Rule

```text
Division = physical split of one living cell state into two cell states.
Nothing is copied except what is explicitly defined as copied.
Everything else is partitioned, degraded, duplicated with cost, or lost.
```

---

# Starting Partition Model

Базова модель: noisy proportional split.

Future-compatible options:

- spatial split;
- regulated asymmetric split.

---

# Partition Table

| State type | Starting partition rule | Conservation |
| --- | --- | --- |
| Energy Buffer | noisy proportional split | total conserved minus division cost |
| Resources | noisy proportional by target volume | conserved |
| Materials | noisy proportional by target volume | conserved unless configured loss |
| Boundary material | split into two boundary states | conserved, but may be insufficient |
| Genome | copied, not partitioned | copy cost paid before completion |
| Genome mutation | applied during copying | deterministic with seed |
| EpigeneticState | partial inherited / attenuated | not necessarily conserved |
| RuntimeState | usually reset | explicit exceptions only |
| MaterialState | partitioned with Materials | physical state follows material |
| Damage | split or inherited by affected materials | conserved as material condition |
| Joints | broken or reassigned if still valid | no magic preservation |
| ProcessProgress | normally not inherited | no hidden copy |

---

# Failed Before Partition

Feasibility fails before physical split.

Result:

- no daughter created;
- no partition;
- no partial physical result;
- ProcessProgress may pause or decay by explicit rule.

Examples:

- not enough Energy for division execution;
- genome copy not complete;
- no minimum material structure for split.

---

# Failed After Partition

Physical split happened and partition committed.

One or both resulting cells may be weak, leaking, inert or doomed.

Selection and lifecycle handle consequences.

---

# Joint Handling

Existing Joints are not duplicated.

Default:

- reassign to spatially valid daughter;
- otherwise break;
- create new Joint only through explicit process.

---

# Rules

## Rule 1. Division is deterministic

Same seed and same state produce same partition.

## Rule 2. Matter and Energy are conservative

Matter and Energy are conserved unless explicit loss/damage rule is configured.

## Rule 3. Offspring viability is not guaranteed

Division can produce weak, inert, leaking or dead descendants.

## Rule 4. No hidden copy

Only explicitly copied state is copied.

---

# Пов'язані документи

- `biology/lifecycle.md`
- `biology/cell-state.md`
- `genetics/inheritance.md`
- `biology/joint.md`
- `world/energy.md`

