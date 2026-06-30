---
tags:
  - alife
  - canon
  - area/genetics
---

# horizontal-transfer.md

> `Horizontal Transfer` — future-compatible передача genetic fragments між незалежними клітинами.

---

# Призначення

Horizontal Transfer описує потенційний шлях, де genetic fragment може перейти не через parent-child inheritance, а через середовище, contact, Joint або decomposition remains.

Це future-compatible механіка, не обов'язкова для першої реалізації.

---

# Канонічні правила

- Genetic fragment має фізичний carrier і degradation.
- Transfer потребує локального контакту, Joint, uptake або environment fragment.
- Uptake не дорівнює integration.
- Integration проходить validation.
- HGT може бути harmful, neutral або useful тільки через selection.
- HGT не є signal і не є command.

---

# Pipeline

```text
fragment source
  -> local carrier/environment/Joint
  -> uptake feasibility
  -> internal fragment state
  -> integration attempt
  -> genome validation
  -> inheritance if retained
```

---

# Заборонено

Не вводити:

- distance-free gene transfer;
- guaranteed beneficial integration;
- species-based whitelist;
- fragment without material carrier;
- integration without validation;
- HGT as ordinary signal.

---

# Semantic Links

- moves fragments of: [[docs/biology/genome|Genome]]
- can use: [[docs/biology/joint|Joint]]
- can use: [[docs/biology/communication|Communication]]
- affects: [[docs/evolution/population-dynamics|Population Dynamics]]

# Пов'язані документи

- `genetics/genome-representation.md`
- `genetics/recombination.md`
- `genetics/mutation.md`
- `biology/joint.md`
- `biology/lifecycle.md`
- `world/materials.md`
