## UI/UX: Розподіл клітин за розміром організмів і функціональною класифікацією

### Мета

Показати:

```text
скільки організмів існує у світі
скільки клітин містить кожен організм
які функціональні типи клітин входять до організмів різного розміру
як змінюється спеціалізація клітин зі зростанням складності організму
```

Основна ідея:

```text
розмір організму
×
функціональний тип клітини
×
кількість або частка
```

---

### Основний графік

Використати гістограму з логарифмічними інтервалами розміру організму.

Вісь X:

```text
1 Cell
2 Cells
3-4
5-9
10-19
20-49
50-99
100-199
200-499
500-999
1000+
```

Вісь Y може перемикатися:

```text
кількість Organisms
% від усіх Organisms
кількість Cells
% від усіх Cells
```

Це важливо, бо:

```text
один Organism із 1000 Cells
і
1000 одноклітинних Organisms
```

мають однакову кількість Cells, але різну структуру популяції.

---

### Перемикач типу класифікації

Над графіком додати глобальний перемикач:

```text
Classification by:
  all Cells
  dominant function
  Cell functional role
  sensory specialization
  Material composition
  active process
  Genome-defined specialization
  observed behavior
```

Вибір змінює спосіб сегментації графіка та доступні фільтри.

---

### Функціональні ролі клітин

Базовий перелік може включати:

```text
contractile / muscle-like
neural / signal-processing-like
sensory
structural
transport
metabolic
storage
repair
protective
reproductive
undifferentiated
mixed-function
```

Ці ролі не повинні бути обов’язково жорстко заданими класами.

Вони можуть визначатися як projection на основі:

```text
Material composition
Capabilities
Process activity
Genome regulation
position in Organism
signal connectivity
resource flow
```

Наприклад:

```text
high Contractile Material
+ frequent Contraction process
→ contractile / muscle-like

high signal input/output activity
+ signal-processing capabilities
→ neural-like

high Boundary/Transport activity
→ transport-like
```

---

### Сенсорна спеціалізація

Для `sensory` потрібно окреме вкладене групування:

```text
light-sensitive
temperature-sensitive
pressure-sensitive
chemical-sensitive
resource-gradient-sensitive
damage-sensitive
contact-sensitive
signal-sensitive
mixed sensory
```

UI має дозволяти:

```text
показати всі sensory Cells
показати тільки light-sensitive
показати тільки pressure-sensitive
порівняти кілька sensory types
```

Приклад фільтра:

```text
Cell role = sensory
Sensory capability = pressure
Organism size >= 20 Cells
```

---

### Режими відображення

#### 1. Stacked bars

Кожен стовпчик розміру Organism розбивається за типами Cells.

Наприклад:

```text
20-49 Cells:
  35% contractile
  20% sensory
  15% neural-like
  10% structural
  20% other
```

Це показує склад організмів різного розміру.

#### 2. Grouped bars

Для порівняння окремих функціональних типів:

```text
contractile vs sensory vs neural-like
```

#### 3. Heatmap

Вісь X:

```text
Organism size
```

Вісь Y:

```text
Cell functional role
```

Колір:

```text
Cell count
Cell percentage
average per Organism
```

Це найкращий режим для загального аналізу спеціалізації.

#### 4. Distribution curve

Показати:

```text
яка частка Organisms має не менше N Cells певного типу
```

Наприклад:

```text
% Organisms with >= 10 sensory Cells
% Organisms with >= 50 contractile Cells
```

---

### Фільтри

Ліва або верхня панель фільтрів:

```text
Organism size range
Cell functional role
sensory specialization
Genome / lineage
Species / cluster
Material type
Capability
active process
lifecycle
Environment zone
time range
```

Фільтри повинні комбінуватися.

Приклад:

```text
Organisms:
  size 50-200 Cells
  lineage A12
  containing pressure-sensitive Cells
  in high-temperature zones
```

---

### Перемикач способу підрахунку

Користувач повинен обрати, що саме рахується:

```text
Organisms containing at least one selected Cell type
total selected Cells
average selected Cells per Organism
percentage of selected Cells inside Organism
dominant-role Organisms
```

Це критично, бо різні метрики відповідають на різні питання.

Наприклад:

```text
"скільки Organisms мають sensory Cells?"
не те саме, що
"скільки sensory Cells існує у світі?"
```

---

### Інтерактивність

При наведенні на сегмент:

```text
Organism size bin
Cell role
Cell count
Organism count
percentage
average per Organism
dominant lineages
change over selected time interval
```

При натисканні:

```text
відфільтрувати світ
підсвітити відповідні Organisms
відкрити список Organisms
перейти до Organism Inspector
закріпити вибір для порівняння
```

---

### Зв’язок із головним viewport

Графік і карта світу мають бути двонаправлено пов’язані.

```text
click chart segment
→ highlight matching Organisms in world

select Organism in world
→ highlight its size bin and Cell-role composition
```

При виборі sensory subtype:

```text
pressure-sensitive
```

viewport може:

```text
підсвітити відповідні Cells
показати pressure field heatmap
показати sensory input overlays
```

---

### Timeline

Додати часовий режим:

```text
current snapshot
selected Tick
selected interval
evolution over time
```

Для selected role показувати:

```text
кількість Cells
частку популяції
кількість Organisms, що використовують роль
середню кількість таких Cells на Organism
```

Це дозволить побачити, наприклад:

```text
коли вперше з’явилися neural-like Cells
чи збільшується частка sensory Cells зі зростанням Organisms
чи contractile Cells зникають або домінують
```

---

### Класифікаційна прозорість

Для кожної функціональної ролі UI повинен показувати:

```text
classification source
classification confidence
classification criteria
mixed-role components
```

Наприклад:

```text
Role: contractile-like
Confidence: 0.82

Reason:
  Contractile Material: 48%
  Contraction process share: 36%
  Force generation activity: high
```

Це важливо, щоб label не виглядав як довільний тип.

---

### Mixed Roles

Cell може виконувати кілька функцій.

Тому UI має підтримувати:

```text
primary role
secondary roles
mixed-role classification
```

Режими:

```text
count by primary role
count by all matched roles
fractional contribution
```

Fractional mode:

```text
Cell:
  60% contractile
  30% sensory
  10% structural
```

У stacked chart така Cell додає частки до кількох категорій.

---

### Рекомендоване розташування

```text
top:
  abstraction level
  classification type
  count mode
  time range

left:
  filters

center:
  main histogram / heatmap

right:
  selected role or Organism-size inspector

bottom:
  timeline and trend charts
```

---

### Основні питання, які UI має дозволяти дослідити

```text
Як змінюється спеціалізація зі зростанням Organism size?
Які функціональні ролі з’являються лише у великих Organisms?
Скільки sensory Cells існує і які сигнали вони сприймають?
Які Organisms мають muscle-like або neural-like Cells?
Які lineages першими сформували спеціалізацію?
Чи існує однакова структура в різних Genome clusters?
Чи є функціональна роль корисною або надто дорогою?
```

---

### Ключовий UX-принцип

```text
Organism size показує складність структури.
Cell role показує функціональну спеціалізацію.
Фільтри показують, де і в яких Genome/Environment умовах вона виникла.
```

Класифікація має допомагати аналізувати emergent specialization, але не повинна перетворювати derived labels на hardcoded поведінкові класи.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
