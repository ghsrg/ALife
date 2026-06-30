# docs/research/mobile-genetic-elements.md

> **Mobile Genetic Elements — мобільні genetic fragments, plasmid-like і HGT-compatible ідеї**

---

# Research Note

Research / future design.

Цей файл не є поточною реалізацією.

Канонічні базові документи:

```text
docs/genetics/horizontal-transfer.md
docs/genetics/recombination.md
docs/biology/genome.md
```

---

# Призначення

Цей файл призначений для майбутніх ідей про mobile genetic elements.

Сюди можуть входити:

* plasmid-like fragments;
* mobile genome packets;
* persistent internal fragments;
* external genetic fragments;
* parasitic fragments;
* symbiotic fragments;
* virus-like behavior без hardcoded Virus class.

---

# Поточна позиція

Для базової моделі не потрібно реалізовувати plasmid-like genome system.

Достатньо, щоб архітектура не блокувала майбутні:

```text
genetic fragments
HGT
fragment uptake
fragment degradation
fragment integration
```

---

# Що може бути mobile genetic element

Можливий mobile element — це фізичний genetic fragment, який може:

* існувати поза клітиною;
* потрапити в клітину;
* деградувати;
* тимчасово зберігатися;
* інтегруватися в Genome;
* частково успадковуватися;
* впливати на Genome Runtime після інтеграції.

---

# Що це НЕ є

Mobile genetic element не є:

* магічним апгрейдом;
* корисною мутацією за замовчуванням;
* вірусом як hardcoded class;
* species marker;
* готовим органом;
* поведінковим скриптом;
* прямим способом обійти mutation і selection.

---

# Майбутні напрямки

Можливі теми для дослідження:

```text
plasmid-like persistence
fragment copy stability
fragment compatibility
integration cost
fragment rejection
fragment degradation
parasitic fragments
symbiotic fragments
virus-like emergent dynamics
```

---

# Коли доповнювати файл

Файл варто доповнювати, коли:

* буде реалізовано базовий Genome;
* з'явиться HGT як механіка;
* genetic fragments стануть фізичними objects або resource-like packets;
* потрібно буде вирішити, чи fragment може жити окремо від основного Genome;
* з'явиться ADR щодо plasmid-like або virus-like системи.

---

# Що НЕ робити зараз

Не потрібно зараз:

* реалізовувати plasmids;
* створювати Virus class;
* вводити infection system;
* робити fragment always beneficial;
* додавати species-specific genetic exchange;
* ускладнювати базова модель.

---

# Пов'язані документи

* `docs/genetics/horizontal-transfer.md`
* `docs/genetics/recombination.md`
* `docs/genetics/heredity.md`
* `docs/biology/genome.md`
* `docs/research/genome-representation-options.md`
* `docs/research/rejected-ideas.md`

---

# Notes for Agents

Цей файл навмисно короткий.

Він існує, щоб зафіксувати майбутній research напрямок і не дозволити агенту випадково впровадити plasmid або virus-like behavior У базовій моделі.

---



