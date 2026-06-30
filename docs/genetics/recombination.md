# recombination.md

> `Recombination` — future-compatible обмін або перебудова genetic fragments.

---

# Призначення

Recombination описує можливу майбутню механіку, де genetic fragments комбінуються у новий Genome або variant.

Це не обов'язкова частина першої реалізації, але поточна Genome-модель має не блокувати її.

---

# Канонічні правила

- Recombination працює з genetic fragments або Genome segments.
- Recombination має physical carrier, cost і validation.
- Result може бути valid, inert, harmful або lethal.
- Recombination не гарантує корисності.
- Recombination не обминає mutation validation і Feasibility Check.

---

# Future Operators

```text
fragment swap
fragment insertion
fragment deletion
fragment duplication
edge/node merge
output binding conflict resolution
```

---

# Заборонено

Не вводити:

- magic compatibility;
- directed beneficial recombination;
- species-based automatic compatibility;
- unlimited fragment size;
- behavior scripts through fragments.

---

# Пов'язані документи

- `genetics/genome-representation.md`
- `genetics/mutation.md`
- `genetics/horizontal-transfer.md`
- `genetics/inheritance.md`
