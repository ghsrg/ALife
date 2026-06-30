# GLOSSARY.md

> **Єдиний словник термінів Artificial Life Engine**

---

# Призначення

Цей документ містить офіційні визначення всіх термінів, що використовуються в проєкті.

Основні цілі:

* уникнути неоднозначності;
* забезпечити однакову термінологію в документації та коді;
* бути єдиним місцем, де визначаються поняття.

---

# Правила

## Один термін — одне значення

Кожен термін має лише одне офіційне визначення.

---

## Не створювати синонімів

Наприклад

✔️

```text
Resource
```

❌

```text
Ресурс
Речовина
Сировина
```

У документації допускається українська назва, але технічний термін залишається один.

---

## Якщо з'являється новий термін

Спочатку він додається сюди.

Після цього використовується в інших документах.

---

# Основні поняття

---

# World

## World

**Українською:** Світ

Повний простір симуляції.

Містить усі фізичні об'єкти та закони.

Не містить понять "вид" або "організм".

---

## Tick

Найменша неподільна одиниця симуляційного часу.

Усі процеси світу виконуються лише під час Tick.

Не плутати із реальною секундою.

---

## Tick Snapshot

Стабільний read-only стан світу, який клітини читають під час `Cell Decision`.

Зміни, створені в цьому Tick, не стають видимими як новий input до моменту, визначеного `world/tick-semantics.md`.

---

## Delta

Опис запланованої зміни стану світу, створений System або Process до фази commit.

Delta не повинна змінювати read snapshot напряму.

---

## Commit

Детерміноване застосування валідних delta-змін до наступного стану світу.

Commit не повинен залежати від випадкового порядку обходу entity або hash map.

---

## Space

**Українською:** Простір

Структура світу, яка визначає, де існують об'єкти, як поділений світ, де межі, що означає локальність, координати й "поруч".

Space є обмеженням, подібним до ресурсу, але не є `ResourceType`.

---

## Simulation Unit

Умовна одиниця вимірювання симуляції.

У документації може позначатися як `su`, `vu`, `au`, `mu`, `eu` або `tick` залежно від типу величини.

---

## Normalized Unit

Безрозмірне значення у стабільному діапазоні, зазвичай `0.0 .. 1.0`.

Genome output може використовувати діапазон `-1.0 .. +1.0` для пригнічення або пріоритизації процесу.

---

## Field

**Українською:** Поле

Безперервна характеристика середовища.

Приклади:

* світло;
* температура;
* тиск;
* електричне поле;
* хімічний градієнт.

---

## Field Effect

Локальний вплив Field через material capability, reaction, physics або process.

Field не є командою і не створює behavior, Energy, damage або mutation напряму.

Кожен Field має описати origin, propagation/decay, local sampling, effect mechanism, bounds і conserved/abstracted behavior.

---

## Reaction

Локальне перетворення Resources, Materials, Field influence або Heat згідно з визначеним правилом.

Reaction не є повною хімією і не повинна містити прихованих shortcut-ефектів.

---

## Reaction Accounting

Явний material/amount accounting для reaction.

Описує, куди переходять input Resources/Materials:

```text
products
retained/internalized material
residual/waste
configured sink/loss
```

Energy output не є матерією і не пояснює missing matter.

---

## Passive Reaction

Reaction, яка відбувається через умови середовища без Genome-рішення.

---

## Controlled Reaction

Reaction, яка потребує ActionPlan, Feasibility Check, Materials або Capability та локальних умов.

---

# Речовина

## Resource

**Українською:** Ресурс

Рухома речовина світу.

Може:

* дифундувати;
* транспортуватися;
* накопичуватися;
* перетворюватися на матеріали;
* використовуватися для виробництва енергії.

Не визначає функцію клітини.

---

## Material

**Українською:** Матеріал

Внутрішня речовина клітини.

Саме матеріали визначають:

* фізичні властивості;
* функції;
* реакцію на поля;
* міцність;
* провідність;
* скорочення.

Матеріал утворюється з ресурсів.

Після руйнування повертається у ресурси.

Не плутати з Resource.

---

## MaterialFragment

External physical remains with material identity.

MaterialFragment може зберігати `material_id`, amount, location, mass/volume, stability, damage state і decay_rate.

Поза Cell він не дає active cell capabilities.

MaterialFragment стає Resource тільки через explicit degradation, reaction або conversion rule.

---

## Energy

**Українською:** Енергія

Energy — локальний енергетичний буфер клітини.

Energy не є Resource або Material.

Energy Buffer не є transferable substance і не займає internal volume напряму.

Energy виникає через перетворення Resource або Field за участі Material.

Місткість Energy Buffer визначається storage-capable Materials, структурою та internal organization клітини.

Energy не передається між клітинами напряму, але клітини можуть передавати ресурси з `energy_value`, продукти реакцій або Heat.

Розподіл Energy Buffer під час поділу є partition локального стану однієї клітини, а не transport між незалежними клітинами.

---

## Feasibility Check

Універсальна read-only перевірка того, чи може planned action або active process виконатися в поточному Tick.

Перевіряє Energy, Resources, Materials, Space, Boundary, Physics, Joint, lifecycle state і конфлікти.

Feasibility для planned actions використовує post-mandatory state.

---

## Action / Process Registry

Canonical source of process ids, Genome output bindings, Feasibility scope and process duration.

Executable process або Genome output не повинен з'являтися поза `biology/action-process-registry.md`.

---

## ProcessProgress

Накопичений стан довгого процесу.

ProcessProgress не є частковим фінальним результатом і збільшується лише після успішної Feasibility Check та оплати вартості відповідного кроку.

---

## Capability

Нормалізована здатність, яку Materials дають клітині для виконання певного процесу.

Genome може лише пріоритизувати процеси, доступні через Capabilities.

---

## Living Function Continuity

Здатність клітини продовжувати мінімальні функції життя через Boundary, Energy, Materials, Genome Runtime, repair і контроль внутрішнього стану.

Не є HP або прихованим viability score.

---

## Division Partition

Фізичний розподіл локального стану parent cell між дочірніми клітинами під час reproduction.

Partition застосовується до Resources, Materials, Energy Buffer, Genome, Epigenetic State, Heat, damage і Joint context згідно з явними правилами.

---

# Біологія

## Cell

**Українською:** Клітина

Базова фізична одиниця життя.

Містить:

* матеріали;
* ресурси;
* енергію;
* геном;
* локальний стан.

---

## Joint

Зв'язок між двома клітинами.

Може:

* передавати сили;
* транспортувати ресурси;
* передавати сигнали;
* руйнуватися;
* створюватися.

Не є окремою клітиною.

---

## Organism

**Українською:** Організм

Observer-side organism-like graph view над Cells і Joints.

Connected component є кандидатом, але не автоматично Organism.

Не є активною сутністю світу, яка керує клітинами.

Observer або analytics layer може будувати `OrganismView` для lineage tracking, genome tracking і досліджень.

---

# Генетика

## Genome

**Українською:** Геном

Фізична спадкова структура.

Керує регуляцією процесів клітини.

Не містить готових поведінкових сценаріїв.

Genome information не є речовиною, але Genome має physical genome carrier.

Functional Genome copy потребує physical carrier, створений з Resources, Materials або internal precursor fragments.

---

## Regulatory Intent

Вихід Genome Runtime, який означає пріоритет, пригнічення або модуляцію процесу.

Regulatory Intent не є дією і не гарантує виконання.

---

## Runtime State

Поточний стан виконання регуляторної мережі або signal accumulation у клітині.

Runtime State не є Genome і зазвичай скидається або передається лише частково під час division.

---

## Material State

Поточний стан конкретних Materials: damage, coefficients, accumulated signal, heat response або інші фізично обґрунтовані змінні.

---

## Genetic Fragment

Фізичний фрагмент спадкової інформації, який може бути частиною Genome, mobile fragment або залишком після decomposition.

Genetic Fragment має об'єм, stability, damage і cost.

---

## Gene

Логічний функціональний елемент геному.

Точне представлення залежить від моделі геному.

Не є окремим класом рушія.

---

## Regulatory Node

Внутрішній вузол регуляторної мережі.

Поєднує сигнали.

Не є біологічною клітиною.

---

## Mutation

Спадкова випадкова зміна геному.

Не має визначеної мети.

---

## Recombination

Об'єднання генетичного матеріалу двох геномів.

Не гарантує життєздатності потомства.

---

## Horizontal Gene Transfer (HGT)

Передача генетичного матеріалу між організмами без статевого розмноження.

---

## Observer Layer

Read-only шар аналізу, статистики, lineage tracking, organism-like grouping і візуалізації.

Observer Layer не повинен змінювати поведінку клітин або бути input для Genome Runtime.

---

## Epigenetic State

Тимчасовий спадковий стан регуляції.

Не змінює геном.

Може частково передаватися дочірнім клітинам.

---

# Процеси

## Synthesis

Перетворення ресурсів на матеріали.

Потребує енергії.

---

## Degradation

Руйнування матеріалу.

Результатом є ресурси або простіші матеріали.

---

## Repair

Відновлення матеріалу за рахунок ресурсів та енергії.

---

## Growth

Збільшення кількості матеріалу в клітині.

---

## Division

Поділ клітини.

Утворює дві нові клітини.

---

## Death

Незворотна втрата функціональності клітини.

Після смерті:

* матеріали деградують;
* ресурси повертаються у світ;
* генетичний матеріал може зберегтися або деградувати.

---

# Фізика

## Collision

Фізичний контакт двох тіл.

Може спричиняти:

* деформацію;
* руйнування матеріалів;
* передачу імпульсу.

---

## Gradient

Просторова зміна концентрації або інтенсивності поля.

Саме градієнт, а не абсолютне значення, часто використовується для орієнтації клітин.

---

## Diffusion

Самовільне поширення ресурсів або сигналів відповідно до градієнта концентрації.

---

# Поведінка

## Learning

Зміна параметрів сигнально-пластичних клітин або Joint під час життя.

Не змінює геном.

---

## Adaptation

Будь-яка зміна, що підвищує або знижує пристосованість.

Може відбуватися на рівні:

* регуляції;
* навчання;
* епігенетики;
* еволюції.

---

## Fitness

Відносна здатність організму виживати та залишати потомство.

Не є параметром рушія.

Є статистичною характеристикою популяції.

---

## Stability Bound

Діапазон параметрів, у якому сценарій не руйнується через очевидний дисбаланс ресурсів, енергії, простору, реакцій або вартості процесів.

Stability Bound є емпіричним результатом калібрування, а не законом світу.

---

## Seed Config

Конфігурація для перевірки конкретного сценарію стабільності або меж життєздатності.

Приклади: `smoke_world`, `single_cell_survival`, `single_cell_division`, `multicellular_stability`.

---

# Документація

## Canon

Офіційні правила світу.

---

## Research

Дослідження та гіпотези.

---

## ADR

Architecture Decision Record.

Документує причини прийнятого рішення.

---

# Пов'язані документи

* `README.md`
* `PRINCIPLES.md`
* `ROADMAP.md`

---

# Open Questions

Після стабілізації моделі геному цей словник буде доповнений термінами:

* Plasmid;
* Regulatory Fragment;
* Genome Module;
* Reproduction Strategy;
* Signal Material;
* Pump Strategy;
* Development Program.


