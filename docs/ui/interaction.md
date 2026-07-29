---
tags:
  - alife
  - ui
  - canon
---

# UI Interaction

## Призначення

Цей документ визначає interaction model `ALife Control Center`.

Документ описує:

- run controls;
- simulation speed;
- visualization frame rate;
- live і historical viewing;
- frozen UI snapshots;
- pointer і keyboard interactions;
- selection;
- placement;
- direct manipulation;
- Undo / Redo;
- interventions;
- confirmations;
- pending і error states;
- Command Palette;
- interaction safety.

Interaction layer не є simulation authority.

## Canonical Interaction Contract

```text
Viewer observes.
UI requests.
Core validates.
Core applies.
Core records.
UI observes result.
```

UI не показує simulation-changing action як виконану, доки Core не підтвердив результат.

## Run Controls

Canonical run controls:

```text
Play
Pause
Step
Speed
Stop
```

### Play

Продовжує або запускає Tick execution.

### Pause

Зупиняє виконання нових Ticks після узгодженої safe boundary.

UI показує:

- pause requested;
- pausing;
- paused;
- current committed Tick.

### Step

Default:

```text
1 Tick
```

Додаткові значення:

```text
1
10
100
custom N Ticks
```

Step доступний лише у compatible paused state.

### Speed

Керує target simulation rate:

```text
Ticks per second
```

Default є contract real-time TPS. Finite range: `1…10,000 ticks/s`.
`Unlimited` є окремим explicit command/state, а не значенням finite Speed
control; його вимкнення відновлює попередній finite TPS. Окрема дія
`Real-time` явно повертає finite rate до contract default і вимикає `Unlimited`.
UI використовує logarithmic slider разом з editable точним полем TPS. Speed
керує темпом execution Core, а не visualization FPS.

### Stop

Завершує active run відповідно до lifecycle rules Core.

Stop не є Pause.

## Simulation Rate And Visualization Rate

UI розрізняє дві незалежні величини.

### Simulation Rate

```text
Ticks per second
```

Визначає target rate виконання simulation Core.

### Visualization Frame Rate

```text
Frames per second
```

Визначає, як часто UI оновлює візуальне представлення.

Ці controls не повинні змішуватися.

Приклад:

```text
Simulation:
  100 ticks/s

Visualization:
  30 frames/s
```

UI може пропускати проміжні visual frames, не пропускаючи simulation Ticks у Core.

Зниження visualization FPS не повинно змінювати simulation behavior.

## Live Data Context

У live mode UI показує останній доступний committed Projection або snapshot.

UI може відставати від Core.

У такому разі показуються:

- displayed Tick;
- latest known Core Tick, якщо доступно;
- delayed або catching-up state;
- visualization FPS;
- simulation Tick rate.

UI не повинен називати displayed Tick поточним, якщо він уже застарів.

## Historical Viewing

Перехід до historical view не зупиняє active run автоматично.

Canonical flow:

```text
select historical point
-> freeze selected Data Context in UI
-> run may continue
-> show Historical / Frozen state
-> provide Jump to Live
```

Historical view не є simulation rollback.

## Bounded Historical Data

UI не зобов'язаний зберігати Projection кожного Tick.

Це особливо важливо при десятках або сотнях Ticks за секунду.

Historical data може надходити з двох джерел:

```text
temporary frozen UI snapshot
engine-created keyframe
```

### Temporary Frozen UI Snapshot

Коли користувач:

- ставить run на Pause;
- обирає доступний Tick;
- відкриває historical state;
- фіксує поточний view;

UI може тимчасово заморозити доступний Projection на рівні presentation.

Frozen UI snapshot:

- bounded;
- temporary;
- read-only;
- не є engine checkpoint;
- не є authoritative simulation artifact;
- може бути втрачений після eviction, reload або зміни session;
- повинен мати visible Tick і Data Context.

### Engine Keyframe

Довготривала historical navigation використовує keyframes, створені engine або run recording subsystem.

Keyframe:

- має stable Tick;
- належить run artifact;
- може бути повторно відкритий;
- має data completeness metadata;
- не обов'язково існує для кожного Tick.

UI не повинно створювати враження, що будь-який Tick доступний історично.

## Timeline Availability

Timeline явно розрізняє:

```text
live range
temporarily frozen frame
available engine keyframe
unavailable Tick
event marker
checkpoint
```

Unavailable Tick:

- не відкривається як повний state;
- може мати aggregated metrics або events;
- не показується як accessible snapshot.

При переході між keyframes UI не вигадує exact intermediate state.

## Jump To Live

У historical або frozen state UI показує явну дію:

```text
Jump to Live
```

Повернення до live:

- змінює Data Context;
- не змінює simulation state;
- може зробити frozen frame недоступним для Redo;
- повинно показувати latest displayed Tick.

## Viewer Pointer Interaction

Canonical desktop interactions:

```text
single click entity:
  select

double click entity:
  focus or zoom to entity

Enter or explicit Open:
  open full detail View

click empty World:
  clear primary selection

drag empty World:
  pan

wheel or trackpad gesture:
  zoom
```

Double click не відкриває full detail View.

## Selection Interaction

### Single Selection

```text
click
-> select entity
-> update Inspector
-> preserve Data Context
```

### Multi-Selection

```text
Ctrl + click
-> add or remove entity
```

Platform-equivalent modifier може використовуватися на інших operating systems.

### Rectangle Selection

```text
Shift + drag
-> rectangular spatial selection
```

Rectangle selection:

- не змінює World;
- не виконує simulation command;
- показує selection count;
- враховує active filters;
- може створити temporary selection subset.

Lasso і radius selection не є canonical requirement.

## Pan And Zoom

Pan і zoom:

- змінюють лише Viewport;
- не змінюють simulation coordinates;
- не змінюють selection автоматично;
- не впливають на Tick execution;
- можуть бути reset;
- можуть бути restored як UI state.

Zoom може змінювати Semantic Zoom і LOD presentation.

## Hover

Hover може:

- підсвічувати entity;
- показувати short label;
- показувати доступність Contextual Help;
- показувати маленьку напівпрозору `i`.

Hover не може:

- змінювати selection;
- відкривати Inspector;
- запускати command;
- вмикати tracking;
- відкривати Expanded Hint автоматично лише через рух pointer.

## Touch Interaction

Desktop-first architecture не вимагає повного touch parity.

Мінімально:

```text
one-finger drag:
  pan

pinch:
  zoom

tap:
  select
```

Rectangle selection на touch не є вимогою.

Contextual Help на touch реалізується лише для critical або реально потрібних components.

## Keyboard Interaction

Canonical defaults:

```text
Space:
  Play / Pause

.:
  Step 1 Tick

F:
  Focus selected

T:
  Track selected

Enter:
  Open full View

Escape:
  cancel mode, close overlay or close transient state

Ctrl+F:
  entity search

Ctrl+K:
  Command Palette

Ctrl+Z:
  Undo reversible UI state

Ctrl+Y:
  Redo reversible UI state
```

Shortcuts:

- не спрацьовують у text input без відповідного modifier;
- мають бути доступні у Help;
- повинні мати accessible alternatives;
- можуть бути configurable у майбутньому;
- не обходять validation або confirmation.

## Placement Workflow

Canonical placement flow:

```text
select asset
-> enter Placement Mode
-> show ghost preview
-> choose position
-> validate continuously
-> show valid or invalid state
-> confirm
-> Core validates again
-> Core applies and records
-> Viewer observes result
```

Placement Mode:

- має чіткий visible state;
- показує asset identity;
- показує target run;
- показує target Tick;
- показує validation result;
- завершується після одного placement;
- скасовується через `Escape`;
- не підтримує canonical `Place Multiple` mode.

Invalid position:

```text
Confirm disabled
```

UI-side validation не замінює Core validation.

## Placement Result

Після confirmation UI показує:

```text
Submitting
Validated
Applied
Rejected
Failed
```

Ghost preview не перетворюється на authoritative entity до Core confirmation.

При rejection UI показує:

- RejectionReason;
- target position;
- asset id;
- current Data Context;
- recovery action.

## Direct Manipulation

### World Editor

World Editor може дозволяти:

- drag initial objects;
- move Resource zones;
- resize configured areas;
- change pre-run parameters;
- preview configuration.

Це редагує draft configuration, а не active WorldState.

### Monitor

Monitor не дозволяє напряму:

- drag existing Cell;
- edit Energy;
- edit Materials;
- edit Genome;
- change lifecycle;
- alter Process state.

Monitor дозволяє:

- select;
- focus;
- track;
- inspect;
- place approved asset;
- submit explicit intervention command.

## Undo And Redo

Canonical Undo / Redo стосується лише reversible UI state.

Приклади:

- selection;
- filter state;
- Viewport state;
- panel state;
- local navigation context;
- temporary frozen frame navigation, якщо frame ще доступний.

Undo / Redo не змінює:

- Core state;
- completed Tick;
- applied intervention;
- run artifact;
- checkpoint;
- authoritative configuration;
- saved asset.

Draft configuration Undo не є обов'язковою вимогою цього Canon.

## Redo Availability

Redo є best-effort для transient UI state.

Redo може бути недоступним, якщо:

- користувач повернувся до live mode;
- frozen UI snapshot було evicted;
- Projection більше не доступна;
- session було reloaded;
- Data Context змінився;
- engine keyframe не існує.

UI не повинно показувати unavailable Redo як гарантований history restore.

## Simulation Time Travel

Для повернення до попереднього simulation state використовуються:

- checkpoint;
- branch;
- replay;
- new run;
- recorded keyframe, якщо доступний.

Це не називається Undo.

## Intervention Workflow

Canonical intervention flow:

```text
select intervention
-> configure parameters
-> show command summary
-> validate request
-> evaluate risk policy
-> optionally require checkpoint
-> confirm
-> submit command
-> Core validates
-> Core applies and records
-> UI observes result
```

UI не застосовує optimistic simulation state.

## Checkpoint Policy

Checkpoint перед intervention визначається risk policy.

### Low Risk

Може не вимагати checkpoint.

### Medium Risk

Checkpoint strongly recommended або запитується.

### High Risk

Checkpoint required, якщо Core та artifact subsystem це підтримують.

Risk policy враховує:

- scope;
- reversibility;
- affected entities;
- expected resource impact;
- run importance;
- destructive potential;
- branch state.

UI показує:

- command summary;
- affected run;
- current Tick;
- validation result;
- risk level;
- checkpoint policy.

## Confirmation Levels

### No Confirmation

Для UI-only reversible actions:

- selection;
- filter;
- zoom;
- layout;
- theme;
- density;
- panel state.

### Standard Confirmation

Для simulation-changing або run-changing actions:

- placement;
- intervention;
- Stop;
- branch creation;
- checkpoint creation, якщо має cost;
- apply World configuration.

### Strong Confirmation

Для destructive або hard-to-recover actions:

- delete run artifact;
- delete saved asset;
- discard unsaved configuration;
- bulk deletion;
- overwrite protected artifact.

Typed confirmation використовується лише для особливо небезпечних bulk або irreversible actions.

## In-App Confirmation

Confirmations реалізуються компонентами application UI.

Заборонено використовувати browser-native:

- `alert`;
- `confirm`;
- `prompt`;

як canonical interaction mechanism.

In-app confirmation повинна:

- відповідати theme;
- підтримувати keyboard;
- мати accessible semantics;
- показувати action summary;
- виділяти destructive action;
- не змінювати wording непередбачувано.

## Pending State

Для asynchronous action UI показує visible pending state.

Приклади:

```text
Validating
Submitting
Applying
Calculating
Loading keyframe
Exporting
```

Pending state:

- блокує duplicate submit;
- не блокує весь UI без потреби;
- має progress, якщо він відомий;
- може мати Cancel для cancellable operation;
- показує target Data Context.

## Duplicate Command Protection

UI повинно захищати від повторної submission одного command.

Механізми можуть включати:

- disabled submit;
- request id;
- idempotency key;
- visible pending state;
- Core-side duplicate rejection.

UI-side disable не замінює Core-side protection.

## Cancellation

Cancel доступний для operations, які реально можна безпечно скасувати.

Приклади:

- analytics query;
- export preparation;
- keyframe loading;
- search;
- report generation.

Applied Core command не показується як cancellable після application boundary.

## Errors And Rejections

UI розрізняє:

```text
UI validation error
Core rejection
transport error
timeout
partial result
unavailable historical frame
permission or capability restriction
```

Message містить:

- what failed;
- reason;
- affected Data Context;
- recovery action;
- technical details on demand;
- stable error id, якщо доступний.

## Command Palette

`Command Palette` відкривається через:

```text
Ctrl+K
```

Вона може шукати:

- workspaces;
- entities;
- runs;
- saved assets;
- navigation actions;
- safe UI actions;
- available commands.

Simulation-changing command із Command Palette:

- не обходить validation;
- не обходить risk policy;
- не обходить confirmation;
- показує target run і Tick;
- не виконується лише через search selection без явної activation.

## Entity Search

`Ctrl+F` відкриває entity search.

Search може:

- select;
- open;
- focus;
- compare;
- add to pinned set.

Search не запускає simulation command.

## Mode Visibility

Temporary interaction mode повинен бути явно видимим.

Приклади:

- Placement Mode;
- Rectangle Selection;
- Tracking;
- Historical View;
- Debug Visualization;
- Full-Screen Viewer.

Mode indicator показує:

- active mode;
- exit action;
- relevant target;
- conflicts with other modes.

## Escape Behavior

`Escape` закриває або скасовує найглибший transient context у predictable order.

Приклад:

```text
1. close Expanded Hint
2. close overlay Inspector
3. cancel Placement Mode
4. cancel Rectangle Selection
5. exit Full-Screen
```

`Escape` не зупиняє run і не виконує destructive action.

## Focus And Tracking

### Focus

```text
F
```

Центрує або fit-ить selected entity у Viewport.

### Track

```text
T
```

Утримує selected entity видимою або центрованою при оновленні simulation.

Tracking:

- presentation-only;
- не змінює entity priority;
- не впливає на physics;
- припиняється явно або при incompatible Data Context;
- показується visible indicator.

## Interaction With Aggregation

Для cluster підтримуються дві canonical actions:

```text
inspect cluster
zoom to cluster
```

Concrete gesture може бути:

```text
single click:
  inspect

double click:
  zoom
```

Gesture має залишатися consistent із entity interaction.

## Архітектурні обмеження

Заборонено:

- змінювати simulation state через UI без Core command;
- показувати ghost preview як applied entity;
- використовувати browser-native alert як canonical confirmation;
- зберігати Projection кожного Tick як обов'язкову UI вимогу;
- показувати unavailable Tick як повний historical snapshot;
- гарантувати Redo для evicted frozen frame;
- називати checkpoint або replay Undo;
- змішувати Ticks per second і Frames per second;
- відкривати Expanded Hint автоматично через pointer movement;
- робити optimistic update authoritative simulation state;
- дозволяти duplicate command submission;
- приховувати pending або rejection state;
- використовувати visibility як simulation priority;
- підтримувати `Place Multiple` як canonical base workflow.

## Пов'язані документи

- `GLOSSARY.md`
- `docs/ui/README.md`
- `docs/ui/principles.md`
- `docs/ui/architecture.md`
- `docs/ui/navigation.md`
- `docs/ui/visualization.md`
- `docs/ui/analytics.md`
- `docs/ui/exploration.md`
- `docs/ui/presentation.md`
- `docs/ui/quality.md`

# Semantic Links

- indexed by: [[docs/ui/README|UI Layer]]
- governed by: [[docs/ui/principles|UI Principles]]
- acts within: [[docs/ui/architecture|UI Architecture]]
- implements navigation from: [[docs/ui/navigation|UI Navigation]]
- controls views from: [[docs/ui/visualization|UI Visualization]]
- interacts with: [[docs/ui/analytics|UI Analytics]]
- interacts with: [[docs/ui/exploration|UI Exploration]]
- uses presentation from: [[docs/ui/presentation|UI Presentation]]
- validated by: [[docs/ui/quality|UI Quality]]
