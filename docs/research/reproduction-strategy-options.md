# docs/research/reproduction-strategy-options.md

> **Reproduction Strategy Options — майбутні варіанти стратегій розмноження**

---

# Статус

Research / future design.

Канонічні базові документи:

```text id="aazwp3"
docs/biology/lifecycle.md
docs/genetics/inheritance.md
docs/genetics/heredity.md
docs/biology/organism.md
```

---

# Призначення

Цей файл призначений для майбутніх варіантів reproduction strategy.

Він не описує MVP реалізацію.

MVP має починатися з простого cell division і inheritance.

---

# Поточна позиція

Для MVP достатньо:

```text id="9unvnt"
one parent cell
    ↓
division
    ↓
two daughter cells
```

Під час division:

* Genome копіюється;
* mutation може відбутися;
* Resources діляться;
* Materials діляться;
* Energy Buffer ділиться локально;
* Epigenetic State частково передається або скидається;
* Runtime State здебільшого не успадковується.

---

# Майбутні reproduction strategies

Можливі варіанти для дослідження:

```text id="n6oj7l"
simple cell division
asymmetric division
fragmentation
budding-like reproduction
spore-like reproductive cell
cluster reproduction
organism-like fragmentation
fusion-like reproduction
sexual-like recombination
multi-parent recombination
```

Це не означає, що всі вони будуть реалізовані.

---

# Що НЕ вводити зараз

Не потрібно зараз:

* male/female;
* fixed sexes;
* hardcoded species compatibility;
* guaranteed viable offspring;
* reproduction fitness function;
* organism-level spawn command;
* full sexual reproduction system.

---

# Organism-level reproduction

Organism-like reproduction може виникати, коли частина Cell-Joint graph відокремлюється і здатна продовжувати lineage.

```text id="6c5wcz"
organism-like structure
    ↓
fragmentation / budding-like split
    ↓
viable descendant structure
```

Це має бути фізичним процесом, а не magic spawn.

---

# Коли доповнювати файл

Файл варто доповнювати, коли:

* MVP cell division працює стабільно;
* з'явиться organism-like structure;
* fragmentation стане реальною подією;
* буде потрібна recombination between lineages;
* буде обрано перший non-trivial reproduction strategy;
* буде створюватися ADR щодо reproduction.

---

# Що перевіряти перед реалізацією нової strategy

Перед реалізацією нової strategy треба відповісти:

```text id="w6t07m"
Який фізичний носій reproduction?
Що саме успадковується?
Де виникає mutation?
Чи є recombination?
Який Energy/Resource cost?
Чи потрібні Joint?
Чи може result бути non-viable?
Чи не вводимо ми species_id?
Чи не hardcode-имо sex?
```

---

# Пов'язані документи

* `docs/biology/lifecycle.md`
* `docs/biology/organism.md`
* `docs/genetics/inheritance.md`
* `docs/genetics/heredity.md`
* `docs/genetics/recombination.md`
* `docs/evolution/selection.md`
* `docs/research/graph-recombination-options.md`
* `docs/research/rejected-ideas.md`

---

# Notes for Agents

Цей файл є research placeholder.

Не розширювати його в MVP-реалізацію без окремого завдання або ADR.
