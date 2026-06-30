# serialization.md

> Serialization — snapshot стану симуляції.

---

# Призначення

Serialization зберігає й відновлює стан так, щоб симуляцію можна було продовжити відтворювано.

---

# Канонічні правила

- Snapshot має містити config hash, seed, tick і schema version.
- Save/load не повинен змінювати behavior.
- Floating/numeric formats мають бути стабільними.
- Observer data може бути optional і не повинна бути потрібна для simulation continuation.

---

# Мінімальний Snapshot

```text
schema_version
config_hash
seed
tick
world_state
entities
components
rng_state
pending_process_progress
```

---

# Заборонено

Не вводити:

- save/load repair of invalid state without explicit migration;
- dependency on rendering state;
- observer metrics required for behavior.

---

# Пов'язані документи

- `engine/storage.md`
- `world/tick-semantics.md`
- `biology/process-progress.md`
