# cell-state.md

> **Cell State — функціональна безперервність життя без hidden HP**

---

# Призначення

`cell-state.md` описує функціональні стани клітини без abstract viability score.

Клітина не має HP і не помирає через перетин штучного viability threshold.

---

# Living Function Continuity

There is no abstract viability threshold.

A cell stops being alive when its living functions can no longer continue through material, energetic, regulatory, or boundary mechanisms.

Оцінюються функції:

- чи оплачено mandatory existence cost;
- чи працює Genome Runtime;
- чи Boundary утримує внутрішній стан;
- чи є Energy/Resources для active work;
- чи Materials ще дають потрібні capabilities;
- чи структура стала inert material structure.

---

# Functional States

## active

Genome Runtime працює, mandatory cost оплачено, planned actions можливі.

## stalled

Energy або Resources недостатні для planned actions. Passive degradation триває.

## stressed

Клітина має пошкодження, нестачу або конфлікт станів, але ще може відновитися.

## dormant

Активність мінімізована. Degradation може бути сповільнена, якщо Materials це дозволяють.

## damaged

Boundary, Materials, Genome carrier або Joint context пошкоджені.

## inert

Living processes зупинені, але структура фізично існує.

## decomposing

Materials розпадаються в Resources, fragments або inert remains.

## persistent_remains

Стабільні Materials лишаються як shell, scaffold, obstacle або local source.

---

# Physical Remains

Мертва клітина може стати:

- organic waste;
- mineral residue;
- stable shell;
- toxic-like local source;
- physical obstacle;
- resource patch;
- surface for attachment;
- protective scaffold.

---

# Rules

## Rule 1. No hidden HP

Cell state is functional, not score-based.

## Rule 2. Death is loss of continuity

Death is consequence of material, energetic, regulatory or boundary failure.

## Rule 3. Remains are physical

Dead/inert cells do not disappear magically.

## Rule 4. Observer may summarize

Observer metrics may label state, but cells and Genome Runtime cannot read viability labels or scores.

---

# Пов'язані документи

- `biology/cell.md`
- `biology/lifecycle.md`
- `world/energy.md`
- `evolution/selection.md`

