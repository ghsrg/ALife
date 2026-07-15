---
tags:
  - alife
  - engine
  - area/engine
  - scheduler
  - cadence
---

# Scheduler

> Scheduler defines how simulation systems are paced, ordered, sampled, and committed without changing Tick semantics.

---

# Призначення

`Tick` є дискретним часом світу. Scheduler є технічною організацією обчислень усередині та навколо Tick.

Scheduler може оптимізувати частоту запуску різних systems, але не може змінювати фундаментальне правило:

```text
World state changes only through deterministic committed Tick boundaries.
```

Фази scheduler не зобов'язані один-в-один збігатися з концептуальним описом Tick, але мають зберігати його інваріанти.

---

# Чотири рівні cadence

Scheduler розділяє чотири різні речі, які не можна змішувати:

```text
1. Simulation Tick Core
2. Genome Decision Cadence
3. World Propagation Cadence
4. Projection / Observer Cadence
```

## 1. Simulation Tick Core

Simulation Tick Core виконує системи, які мають змінювати world state з Tick-level причинністю.

Ці системи за замовчуванням працюють кожен Tick:

```text
mandatory upkeep
lifecycle check
process progress accounting
physics and movement
contact detection
resource uptake/export execution
energy conversion progress
local heat/contact transfer
joint resource/signal/heat transfer
critical damage checks
```

Правила:

- mandatory upkeep списується кожен Tick;
- lifecycle check виконується кожен Tick, щоб death, stress і dormancy не мали штучної затримки;
- physics, movement і contact detection виконуються кожен Tick;
- process execution/progress є окремим від Genome decision;
- committed Tick є єдиною межею зміни authoritative world state.

## 2. Genome Decision Cadence

Genome Runtime не повинен виконуватись кожен Tick за замовчуванням.

Genome Runtime:

- читає committed local inputs;
- оновлює regulatory outputs або `ActionPlan`;
- не виконує процеси напряму;
- не мутує world state напряму.

Між запусками Genome Runtime клітина використовує останній committed decision state:

```text
Committed local state
    -> Genome Runtime at configured cadence
    -> committed ActionPlan / priority state
    -> process execution/progress over following Ticks
```

Рекомендований default для нового scheduler config:

```text
genome_runtime_base_ticks = 10
```

Tests and compatibility scenarios may explicitly set Genome cadence to `1` when they need every-Tick decision refresh.

Genome cadence may vary by template, regulatory layer, or future epigenetic state, but the effective cadence must be deterministic and visible in config/projections.

## 3. World Propagation Cadence

World propagation systems may run less often than every Tick if elapsed time is integrated explicitly.

Examples:

```text
resource diffusion
resource decay
passive reactions
material degradation
environmental heat diffusion
field updates
chemical gradients
background exposure
```

If a system runs every `N` ticks, it must not simply skip `N - 1` units of physical effect. It must use one of:

```text
elapsed_tick integration
incremental accumulator
dirty-region update with deterministic catch-up
```

World propagation cadence must define:

```text
cadence_ticks
elapsed_tick handling
input snapshot visibility
commit boundary
deterministic partition order
deterministic merge order
```

## 4. Projection / Observer Cadence

Projection and Observer cadence is not simulation cadence.

Observer and viewer systems read committed state and produce read-only outputs:

```text
Committed World State
    -> Projection / Observer Sampler
    -> Viewer frame, status, metrics, report, trace
```

These outputs must not affect:

```text
Genome Runtime
Feasibility
process execution
lifecycle
selection
world state
```

Observer metrics, graph/organism analysis, resource totals, debug traces, and viewer projections should not be computed every Tick unless explicitly required.

---

# Мінімальні Tick фази

The exact implementation may split these phases into smaller systems, but the ordering contract is:

```text
1. Read stable committed state.
2. Apply scheduled world propagation due for this Tick.
3. Rebuild spatial/contact structures required for this Tick.
4. Collect local inputs and contact/field samples.
5. Refresh Genome decision state only when due.
6. Assemble or reuse committed ActionPlan / process priorities.
7. Pay mandatory costs.
8. Commit post-mandatory state.
9. Run Feasibility for executable process steps.
10. Execute allowed process progress for this Tick.
11. Apply physics/contact/joint transfer corrections.
12. Apply lifecycle, death, decomposition and cleanup.
13. Commit authoritative Tick state.
14. Emit events and lightweight per-Tick summaries.
15. Sample projections/observer outputs only if due or forced.
```

Mandatory costs are paid before planned action Feasibility. Feasibility uses post-mandatory state.

---

# Cadence categories

## Fast Tick systems

These normally run every Tick:

```toml
[scheduler.fast]
mandatory_upkeep_ticks = 1
lifecycle_ticks = 1
movement_physics_ticks = 1
contact_ticks = 1
resource_uptake_export_ticks = 1
energy_conversion_ticks = 1
process_progress_ticks = 1
local_heat_transfer_ticks = 1
joint_transfer_ticks = 1
```

## Cell decision systems

These may run less often than every Tick:

```toml
[scheduler.cell]
genome_runtime_base_ticks = 10
genome_runtime_ticks_per_layer = 10
signal_emit_ticks = 2
controlled_reaction_ticks = 2
simple_synthesis_ticks = 5
basic_repair_ticks = 10
internal_rebalance_ticks = 5
```

Decision cadence affects when a new decision is made. It does not prevent ongoing process progress from advancing each Tick when a process is active.

## World propagation systems

These can be scheduled when they integrate elapsed time explicitly:

```toml
[scheduler.world]
resource_diffusion_ticks = 2
resource_decay_ticks = 5
passive_reactions_ticks = 2
background_material_degradation_ticks = 5
environment_heat_diffusion_ticks = 2
field_update_ticks = 5
```

## Observer systems

These are read-only and may be sampled:

```toml
[scheduler.observer]
observer_metrics_ticks = 10
resource_totals_ticks = 10
graph_analysis_ticks = 50
debug_trace_ticks = 10
```

Resource totals may alternatively be maintained incrementally. If they are recalculated, cadence must be explicit.

## Long-running process defaults

Long-running process duration is different from scheduler cadence.

```toml
[scheduler.process_duration]
large_synthesis_ticks = 50
basic_repair_progress_ticks = 50
large_repair_ticks = 200
joint_strengthening_ticks = 100
division_preparation_ticks = 200
genome_copying_ticks = 150
```

Example distinction:

```text
Genome decision: every 10 Tick
Division preparation progress: every Tick while active
Division partition: atomic completion at committed Tick after requirements are satisfied
```

---

# Viewer Projection Logic

Viewer projection працює незалежно від simulation Tick cadence.

```text
Simulation Tick
    -> Committed World State
    -> Viewer Projection Sampler
    -> Viewer Frame
    -> UI interpolation/rendering
```

## Основні правила

- Viewer projection не формується після кожного Tick за замовчуванням.
- Maximum projection frequency is wall-clock limited.
- При високій simulation speed проміжні Tick пропускаються для viewer.
- Projection завжди будується з останнього повністю committed state.
- Повільний Viewer не блокує simulation.
- Якщо продуктивність падає, projection можна пропускати.
- Під час active Run має передаватися щонайменше один frame на секунду, якщо Runner і projection generation здорові.
- UI може рендерити частіше за надходження projection через interpolation.
- Pause, StepRun, Completed і Failed примусово створюють projection незалежно від cadence.

## Рекомендована конфігурація

```toml
[viewer_projection]
target_frames_per_second = 10
minimum_frames_per_second = 1
render_target_frames_per_second = 30
maximum_frame_age_ms = 1000

drop_intermediate_frames = true
latest_frame_only = true
force_frame_on_start = true
force_frame_on_pause = true
force_frame_on_step = true
force_frame_on_resume_if_stale = true
force_frame_on_terminal_state = true
```

## Нормальна робота

```text
simulation speed: 10 TPS
projection rate: 10 FPS
UI rendering: 30 FPS
```

Кожен Tick може створити новий Viewer frame, але це не є вимогою simulation semantics.

## Прискорена simulation

```text
simulation speed: 100 TPS
projection rate: 10 FPS
```

Viewer отримує приблизно один frame на кожні 10 Tick:

```text
Tick 10  -> projection
Tick 20  -> projection
Tick 30  -> projection
```

Точна кількість пропущених Tick визначається wall-clock cadence, а не жорстким `projection_every_n_ticks`, оскільки фактичний TPS може змінюватися.

## Падіння продуктивності

Якщо simulation або projection не встигає підтримувати `target_frames_per_second`:

```text
target: 10 FPS
minimum: 1 FPS
```

Scheduler пропускає проміжні projection і передає лише найновіший доступний committed state.

```text
старі непередані frames не накопичуються
черга не росте
Viewer отримує latest state
simulation не очікує Viewer
```

Projection нижче `minimum_frames_per_second` допускається лише коли:

```text
Run is Paused
Runner is Stopping
Runner process is unavailable
projection generation failed
```

## Forced projections

Projection створюється негайно при:

```text
StartRun completed preparation
Pause committed
StepRun committed exactly one Tick
Resume, if last frame is stale
Completed
Failed
```

Forced projection still reads only committed state.

## Viewer frame fields

Viewer frame metadata should include:

```text
committed_tick
simulation_time
projection_sequence
wall_clock_generated_at
previous_committed_tick
```

UI interpolation uses the last two frames:

```text
position_display = interpolate(previous_position, current_position, render_alpha)
```

Interpolation affects rendering only and creates no simulation state.

## Summary behavior

```text
10 TPS   -> up to 10 projections/sec
50 TPS   -> up to 10 projections/sec
100 TPS  -> up to 10 projections/sec
5 TPS    -> up to 5 projections/sec
1 TPS    -> 1 projection/sec
<1 TPS   -> projection after each committed Tick
```

Viewer projection has wall-clock limit `10 FPS`, heartbeat `1 FPS`, and no authority over simulation speed.

---

# Time configuration

Time config distinguishes playback pacing from simulation semantics:

```toml
[time]
tick_duration_ms = 100
realtime_target_tps = 10
headless_target_tps = 50
```

`tick_duration_ms` is presentation/playback calibration, not a reason to mutate state between Ticks.

---

# Determinism rules

- Same seed + same config + same scheduler config => same committed world state.
- Scheduled systems must define deterministic due checks.
- Entity staggering must use deterministic partitioning.
- Parallel execution must preserve deterministic reduction order.
- Skipped projections and observer outputs must not change committed simulation state.
- Wall-clock decisions may control only projection/pacing, never simulation mechanics.
- Any cadence that changes semantic visibility must be explicit in config and projections.

---

# Заборонено

Не вводити:

- order-dependent priority from entity iteration;
- same-tick infinite feedback;
- optimization that changes Canon behavior silently;
- hidden retry/repair of invalid actions;
- organism-level control phase;
- observer metrics as behavior input;
- viewer projection as simulation authority;
- wall-clock-dependent simulation mechanics;
- cadence optimization that changes Tick visibility without explicit rule.

---

# Implementation notes

Current implementation status:

- Runner already has WS frame throttling through `target_broadcast_fps`.
- `TickExecutor::step()` currently computes many systems every Tick.
- `GenomeTemplate.runtime_interval_ticks` exists in config parsing and hashing, but the Core still needs explicit ActionPlan refresh cadence.
- `RunEngine` currently stores a committed snapshot after every Tick.
- A future implementation should move viewer frame selection into an explicit `ViewerProjectionSampler` and keep Core commit boundaries deterministic.

---

# Semantic Links

- implements: [[docs/world/tick|Tick]]
- orders: [[docs/world/tick-semantics|Tick Semantics]]
- constrained by: [[docs/engine/technology-stack|Technology Stack]]
- invokes: [[docs/genetics/genome-runtime|Genome Runtime]]
- invokes: [[docs/biology/processes|Processes]]
- read-only projection: [[docs/observer/observer-layer|Observer Layer]]
- runner projection: [[docs/runner/projections|Runner Projections]]

# Пов'язані документи

- `world/tick.md`
- `world/tick-semantics.md`
- `engine/technology-stack.md`
- `biology/feasibility.md`
- `biology/process-progress.md`
- `genetics/genome-runtime.md`
- `observer/observer-layer.md`
- `runner/projections.md`
