# fields.md

> `Field` — просторовий вплив або величина світу.

---

# Призначення

Field описує розподілений у просторі вплив: light-like, heat-like, pressure-like або інший scalar/vector стан.

Field не є Resource і не зберігається як речовина в клітині.

---

# Канонічні правила

- Cell може читати Field тільки локально.
- Field стає input для Genome Runtime лише за наявності material sensing basis.
- Field не створює Energy Buffer напряму.
- Field може впливати на reactions, Materials, movement або sensing тільки через описаний mechanism.
- Field semantics мають бути bounded і deterministic.
- Field effects follow `world/field-semantics.md`.
- Кожне Field має описати origin, propagation/decay, local sampling, effect mechanism, bounds і conserved/abstracted behavior.

---

# Мінімальні Властивості

```text
id
kind
value range
spatial representation
diffusion/propagation
decay
interaction rules
```

---

# Заборонено

Не вводити:

- global field read by cell;
- free Energy from Field;
- sensing without Material basis;
- unbounded field values;
- hidden behavior commands through Field.
- direct Energy, damage, mutation or movement without explicit mechanism.

---

# Пов'язані документи

- `world/field-semantics.md`
- `world/space.md`
- `world/energy.md`
- `docs/config/fields_config.md`
