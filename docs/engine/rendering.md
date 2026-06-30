# rendering.md

> Rendering — read-only visualization of simulation state.

---

# Призначення

Rendering показує стан світу, клітин, Joint, Resources, Materials, Fields і observer views.

Rendering не керує симуляцією.

---

# Канонічні правила

- Renderer читає snapshots або observer data.
- Visualization labels не є simulation inputs.
- Debug overlays мають бути відтворюваними для одного snapshot.
- Heavy rendering не повинно змінювати Tick timing semantics.

---

# Мінімальні Шари

```text
cells
joints
resources
fields
traces
genome/runtime debug
organism-like connected components
metrics overlays
```

---

# Заборонено

Не вводити:

- behavior changes from UI state;
- hidden simulation updates during rendering;
- organism labels that feed back into cells.

---

# Пов'язані документи

- `engine/storage.md`
- `engine/serialization.md`
- `biology/organism.md`
