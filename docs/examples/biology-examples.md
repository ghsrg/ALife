---
tags:
  - alife
  - examples
  - area/examples
---

# biology-examples.md

> Приклади біологічних механік без створення нових Canon-правил.

---

# Lifecycle

## Нормальний ріст і поділ

Клітина поглинає ресурс, виробляє Energy, синтезує Boundary/Structural Materials, копіює Genome і проходить Feasibility Check для division.

Результат: parent state partition-иться у дві дочірні клітини за правилами `biology/division-partition.md`.

## Занадто ранній поділ

Genome Runtime підвищує `divide_priority`, але клітина має недостатньо Boundary Material або Energy.

Результат: division action rejected by Feasibility Check. ProcessProgress може зберегтися, деградувати або скинутися за правилами конкретного процесу.

## Смерть від Boundary failure

Collision або reaction пошкоджує Boundary Material. Resources витікають, unwanted Resources потрапляють всередину, Energy production падає, repair не проходить Feasibility.

Результат: клітина переходить у `dead` або `decomposing` не через HP, а через втрату Living Function Continuity.

## Мертва клітина як ресурс

Після death Materials і Resources залишаються у світі, можуть деградувати, вступати в reactions або бути поглинуті іншими клітинами.

---

# Joint

## Проста колонія

Дві клітини контактують, мають compatible Boundary Materials і принаймні одна формує regulatory intent для Joint creation.

Feasibility проходить: створюється один Joint object з mechanical/resource/signal/heat channels.

## Resource sharing

Cell A має більше Resource A, Cell B менше. Joint має permeability/resource channel.

Результат: passive або active transfer можливий, але Energy Buffer напряму не передається.

## Signal chain

Cell A створює scalar signal через Joint. Cell B читає його в наступній stable reading phase, якщо має signal-sensitive Material.

Результат: Cell B змінює власні process priorities; Cell A не командує Cell B.

## Division with Joint

Parent cell має external Joint і ділиться.

Результат: Joint не дублюється. Він reassigned до просторово валідної daughter або breaks.

---

# Communication

## Trace following

Cell A лишає Material/Resource trace у середовищі. Trace decay/diffusion створює локальний gradient.

Cell B може реагувати лише якщо має material basis для sensing. Це не pathfinding і не species marker.

## Heat warning

Cell A має високу local temperature. Heat передається через contact або Joint.

Cell B може збільшити repair/dormancy priorities, якщо має heat-sensitive Material і відповідні Genome inputs.

---

# Organism-Like Structures

## Colony-like structure

Connected component має weak Joints, слабкий Resource sharing і більшість клітин може вижити окремо.

Аналітично це colony-like, але клітини не читають label.

## Organism-like dependency

Outer cells підтримують Boundary-like layer, inner cells отримують Resources через transport-like paths, signal-conducting cells передають stress signals.

Якщо transport-like cells гинуть, залежні клітини втрачають ресурси. Це dependency, не organism HP.

## Fragmentation reproduction

Joint network розривається або перебудовується так, що відокремлений cluster виживає.

Якщо такий pattern повторюється і передає lineage, selection може підтримати organism-level reproduction-like strategy.

# Semantic Links

- illustrates: [[docs/biology/cell|Cell]]
- illustrates: [[docs/biology/joint|Joint]]
- illustrates: [[docs/biology/lifecycle|Lifecycle]]

