# genetics-examples.md

> Приклади генетичних механік без створення нових Canon-правил.

---

# Genome Runtime

## Energy shortage

Inputs: low `energy_level`, available internal Resource, stable Boundary.

Runtime підвищує `produce_energy`, пригнічує `movement` і `division`. Execution залежить від Feasibility і Materials.

## Missing Material

Genome output має `move = high`, але movement-capable Material відсутній.

Результат: output є regulatory intent, але movement action rejected.

## Stateful signal accumulation

Repeated Joint signals можуть накопичуватися в RuntimeState або Stateful Material, якщо це дозволено моделлю.

Накопичення не змінює Genome і не є mutation.

---

# Mutation

## Weight shift

Edge weight змінюється на малу випадкову величину.

Результат може бути корисним, нейтральним або шкідливим; selection оцінює лише наслідки.

## Deleted repair edge

Mutation видаляє edge до `repair_boundary`.

Клітина може втратити здатність ремонтувати Boundary і загинути після damage.

## Silent mutation

Mutation змінює inactive або unavailable binding.

Ефект може проявитися пізніше, якщо середовище або Materials зміняться.

---

# Inheritance

## Normal division inheritance

Genome копіюється, Resources/Materials/Energy partition-яться з noise, Epigenetic State частково передається або reset.

Technical validation не робить daughters життєздатними штучно.

## Daughter without functional Genome

Division error створює daughter з неповним Genome.

Результат допустимий: клітина може жити тимчасово на passive processes, але не має нормальної active regulation.

---

# Recombination and HGT

## Insert fragment

External Genetic Fragment інтегрується як subgraph або fragment container.

Результат може бути silent, harmful або useful залежно від bindings, Materials і Feasibility.

## HGT from dead cell

Dead cell decomposes, Genome fragments залишаються локально, living cell uptake-ить fragment за наявності Boundary/material mechanism.

Fragment стає еволюційно значущим лише якщо зберігається і передається через inheritance.

