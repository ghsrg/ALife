---
tags:
  - alife
  - implementation
  - runner
  - roadmap
---

# Runner Implementation Plan

> **For agentic workers:** це high-level parent plan, а не готовий task-by-task план. Перед початком кожного implementation slice потрібно створити окремий `PLAN` worklog із checkbox-кроками, файлами, тестами та acceptance gate. Після завершення slice створюється відповідний `REPORT`.

## Призначення

Цей документ визначає архітектуру та послідовність реалізації `alife-runner` — шару між `alife-core` і зовнішнім світом (UI, CLI, тести).

Runner:

- запускає Core симуляцію з TOML конфігу;
- керує lifecycle run-у (start, pause, resume, step, stop);
- транслює committed snapshots у WS clients з обмеженою частотою (≤30 fps);
- в режимі `--serve` надає HTTP API і WebSocket frame stream.

При конфлікті пріоритет мають:

```text
docs/PRINCIPLES.md
docs/runner/INDEX.md
docs/runner/runner.md
docs/runner/execution-modes.md
docs/runner/run-lifecycle.md
docs/runner/command-contract.md
docs/runner/scenario-resolution.md
docs/runner/projections.md
docs/runner/bootstrap.md
docs/implementation/implementation-plan-bootstrap.md
docs/implementation/architecture.md
docs/implementation/implementation-phases.md
docs/engine/technology-stack.md
```

Runner Canon in `docs/runner/` supersedes older implementation-plan wording. If a worklog snippet still shows direct TOML-to-Core startup, simplified `Idle/Running/Paused` state, server-side frame history, or multi-tick `StepRun`, the Canon contract is authoritative.

---

## Canon Alignment Requirements

All Runner phases must implement or preserve these contracts from `docs/runner/`:

```text
Source -> Load -> Parse -> Normalize -> Resolve References
       -> Validate -> Canonicalize -> Hash -> ScenarioDocument
       -> Bootstrap -> PreparedWorld -> Core Start -> Committed Projections
```

Required boundaries:

- CLI, HTTP, UI, tests, and batch tools adapt input into shared Runner commands.
- Adapters must not call Core or Bootstrap directly.
- Runner owns orchestration, lifecycle, command validation, and projections.
- Runner must not define world laws or simulation mechanics.
- Scenario resolution must not generate World state.
- Bootstrap prepares Tick 0 and must not execute a Tick.
- External consumers receive versioned read-only projections, not mutable `WorldState`.

Mandatory shared commands:

```text
ValidateScenario
PrepareScenario
StartRun
PauseRun
ResumeRun
StepRun
StopRun
GetRunStatus
```

`StepRun` contract:

```text
Executes exactly one committed Tick.
Valid only when Active Run is Paused.
Returns Paused state and updated committed_tick.
Multi-tick advancement is intentionally out of scope for this Runner phase.
A future command may add bounded tick advancement, but it must not reuse StepRun semantics.
```

---

## Прийняті архітектурні рішення

### Crate структура — два crate, один процес

```text
alife-core          (pure simulation library, no networking)
  world state SoA
  tick loop
  committed snapshots + event buffer

alife-runner        (binary crate, orchestration)
  CLI entry point
  config/scenario loading and validation
  run state machine
  time-based frame broadcast (≤30 fps)
  command queue (receives from HTTP or CLI)

alife-viewer-server (library crate, HTTP + WS adapter)
  HTTP command API
  WebSocket frame stream
  multiple client support
  binary frame encoding
```

Всі три працюють в **одному процесі**. `alife-core` не залежить від `alife-runner` чи `alife-viewer-server`. `alife-viewer-server` залежить від `alife-runner` тільки через передані snapshots і state.

Runner запускає `alife-viewer-server` опційно через прапорець `--serve`.

### Команди — HTTP для команд, WS для frame stream

Чіткий поділ ролей:

```text
HTTP REST   ← команди з request/response семантикою
WS /stream  ← push-only frame stream (Runner → UI)
```

HTTP дозволяє тестувати команди через curl. WS не змішує типи повідомлень.

### Frame streaming — time-based broadcast, client-side history

```text
Core commits Snapshot[tick N]  (runs at full speed, unbounded)
  ↓
Runner tick loop перевіряє: elapsed ≥ frame_interval (default 33ms = 30fps)
  ↓ (якщо так — encode + broadcast; інакше — skip)
Broadcast channel → всі підключені WS-клієнти отримують frame
```

Core ніколи не чекає на WS-клієнтів. Повільний клієнт отримує `Lagged` помилку і продовжує з найновішого кадру. Це впливає виключно на плавність перегляду, не на Core.

**Немає серверного ring buffer.** Scroll-back — виключно відповідальність UI клієнта (browser зберігає останню хвилину кадрів у пам'яті; при паузі зберігає всі кадри до memory limit).

### Scenario discovery — HTTP API

UI не читає filesystem напряму. Він запитує Runner:

```text
GET /scenarios         → список сценаріїв з метаданими
GET /scenarios/{id}    → повний TOML вміст конфігу
```

Runner сканує `config/scenarios/` і повертає список через HTTP. При запуску надсилає `scenario_id`, Runner сам читає файл і валідує.

### Headless vs serve mode

```bash
# Headless — без HTTP/WS
cargo run --bin runner -- config/scenarios/my_world.toml

# З UI підтримкою — стартує HTTP + WS сервер
cargo run --bin runner -- --serve config/scenarios/my_world.toml
```

HTTP server налаштовується у `config/server.toml` або через `[server]` блок.

---

## Core Contract

```text
Runner observes CommittedSnapshot.
Runner does not mutate WorldState.
CLI/HTTP/UI adapters translate input into shared Runner commands.
Runner resolves ScenarioDocument, validates it, invokes Bootstrap, and starts Core only from PreparedWorld.
Core executes Tick only after Runner reaches Running.
Runner reads committed Core outputs and builds versioned projections.
viewer-server pushes projections to WS clients.
```

Runner не є частиною simulation hot path.

---

## Scenario Directory

UI-facing scenarios живуть у:

```text
config/scenarios/
  single_cell_survival.toml
  division_test.toml
  ...
```

Файли з інших директорій (`config/analyzer/`, `config/observer/`) не відображаються через `/scenarios` API — вони для headless tools.

---

Scenario files are sources, not runtime input. Every run must resolve source input into an immutable `ScenarioDocument` before Bootstrap:

```text
local path | scenario id | inline document
  -> ScenarioDocument
  -> scenario_hash
  -> Bootstrap
```

`scenario_hash` is computed from the canonical normalized document, not from filesystem path, request id, UI state, or raw TOML bytes.

## HTTP API

### Server info

```text
GET /server/info
→ {
    engine_version: "...",
    api_version: "1",
    server_uptime_ticks: N,
    allow_remote_viewer: false
  }
```

### Scenario discovery

```text
GET /scenarios
→ [
    { id: "single_cell_survival", name: "...", description: "...", path: "..." },
    ...
  ]

GET /scenarios/{id}
→ { id: "...", config_toml: "<full TOML content>" }
```

### Run status

```text
GET /run/status
→ {
    process_state: "starting" | "ready" | "shutting_down" | "failed",
    active_run_state: "idle" | "preparing" | "running" | "paused" | "stopping" | "completed" | "failed",
    run_id: "...",
    committed_tick: N,
    scenario_hash: "...",
    seed: N,
    ticks_per_second: N,
    collapse_reason: null | "..."
  }
```

### Run controls

```text
POST /run/start
  body: { scenario_id: "single_cell_survival", seed_override: 42, request_id: "optional-id" }
  -> shared command: StartRun
  -> Scenario Resolution -> Bootstrap -> PreparedWorld validation -> Core start
  → { ok: true, run_id: "...", scenario_hash: "...", bootstrap_manifest: {...}, seed: 42, active_run_state: "running" }
  → 409 Conflict якщо run вже активний

POST /run/pause
  → { ok: true, tick: N }

POST /run/resume
  → { ok: true }

POST /run/step
  body: {}
  valid only when Active Run is Paused
  executes exactly one committed Tick
  → { ok: true, active_run_state: "paused", committed_tick: N }

POST /run/stop
  → { ok: true }
```

Всі команди повертають явний error body у разі проблеми.

---

## WebSocket Frame Stream

### Підключення

```text
WS /stream
```

При підключенні сервер одразу надсилає поточний статус і останній доступний frame (якщо є).

### Push від сервера

Два типи повідомлень:

```text
{ type: "status", state: "running", tick: N }
{ type: "frame",  tick: N, data: <binary CommittedSnapshot projection> }
```

Frame format: бінарний, версіонований. Точна схема визначається при реалізації Runner-3.

Wire frames encode `WorldFrameProjection`, not internal `WorldState`. The allowed pipeline is:

```text
Committed Core State -> Projection Builder -> WorldFrameProjection v1 -> ALIF frame bytes
```

### Кілька клієнтів

Кожне WS-з'єднання є незалежним підписником. Runner пушить кожному клієнту паралельно. Повільний клієнт пропускає кадри без впливу на Core або інших клієнтів.

### Scroll-back і seek — відповідальність клієнта

Сервер не зберігає history і не реалізує seek. Scroll-back реалізується на стороні UI:

```text
UI (browser) — стратегія утримання кадрів:
  live mode:   зберігати 1 кадр/сек (відкидати решту з 30fps потоку)
  paused mode: зберігати всі кадри що приходять (≤60 кадрів = ~1 хв)
  memory limit: evict oldest при переповненні

Seek: scroll по локальному масиву кадрів у браузері
"Jump to live": reset scroll position → стежити за live кадрами
```

Сервер надсилає лише `{ type: "status" }` text і binary frames. Клієнт вирішує що тримати.

---

## Run State Machine

Canon state model from `docs/runner/run-lifecycle.md` is authoritative. Any older simplified diagrams in this file or worklogs are legacy sketches and must not be implemented when they conflict with this model.

```text
Runner Process:
Starting -> Ready | Failed
Ready -> ShuttingDown

Active Run:
Idle -> Preparing
Preparing -> Running | Failed | Idle
Running -> Paused | Stopping | Completed | Failed
Paused -> Running | Stopping | Completed | Failed
Stopping -> Completed | Failed
Completed -> Idle
Failed -> Idle
```

Command validity:

```text
StartRun: Idle
PauseRun: Running
ResumeRun: Paused
StepRun: Paused only; exactly one committed Tick; returns Paused + committed_tick
StopRun: Preparing, Running, or Paused
GetRunStatus: every non-process-failed state
```

No Tick may execute in `Preparing`. Failed preparation must not expose a partial World. Invalid commands return a stable state-conflict error without changing state.

Legacy pre-Canon sketch below is retained only as historical context until detailed worklogs are regenerated:

```text
Idle
  → [POST /run/start]     → Running
  → [POST /run/step]      → Paused (1 step executed)

Running
  → [POST /run/pause]     → Paused
  → [POST /run/stop]      → Idle
  → [max_ticks reached]   → Idle
  → [collapse detected]   → Idle (collapse_reason saved)

Paused
  → [POST /run/resume]    → Running
  → [POST /run/step]      → Paused (N steps executed)
  → [POST /run/stop]      → Idle

Stopping (transitional)
  → Core finishes current Tick → Idle
```

Неможливі команди повертають 409 Conflict з description.

---

## Frame Rate Config

```toml
[server]
bind_host = "127.0.0.1"
port = 8080
target_broadcast_fps = 30    # max кадрів/сек до WS клієнтів (time-based)
```

`target_broadcast_fps = 30` → frame_interval = 33ms. Core тікає необмежено швидко; broadcast відбувається не частіше ніж раз на frame_interval.

Сервер не зберігає history у пам'яті. Клієнт тримає власну ring queue кадрів (browser-side).

---

## Server Configuration

```toml
# config/server.toml (або вбудований у scenario config як [server] блок)

[server]
bind_host = "127.0.0.1"       # local-only за замовчуванням
port = 8080
allow_remote_viewer = false
target_broadcast_fps = 30     # max WS frame push rate (time-based, не tick-based)

# Remote viewer mode (opt-in):
# bind_host = "0.0.0.0"
# allow_remote_viewer = true
# allowed_origins = ["http://192.168.1.51:5173"]
```

Local mode (`127.0.0.1`): тільки з тієї ж машини.
Remote mode (`0.0.0.0`): явно opt-in, для trusted LAN.

---

## Headless Mode

Debug progress output:

- `--debug` enables a terminal status table while the headless runner is active.
- Default progress interval: `2000 ms`.
- `--progress-interval-ms <N>` overrides the debug progress interval.
- First debug status is printed after the first committed tick, then repeated by interval.
- Debug output is observer-only: it must not mutate simulation state, change random seeds, or change deterministic replay results.
- Minimum table fields: elapsed time, current tick / max ticks, ticks per second, total cells, alive/dead cells when available, heat, waste, runner state/collapse reason when available.

Без `--serve` Runner:

- завантажує конфіг;
- запускає Core Tick loop;
- пише logs у stdout;
- завершується по `max_ticks` або collapse.

З `--serve`:

- те саме + стартує HTTP + WS сервер;
- чекає на команди;
- не завершується автоматично при collapse (залишається Idle, чекає нового start).

---

## Implementation Slices

### Runner-1: Headless Run Loop And State Machine

Prerequisite:

```text
Bootstrap-1 Foundation complete:
outputs/worklogs/2026-07-14-1635-PLAN-bootstrap-1-foundation.md
```

Runner-1 must start Core only from `PreparedWorld`. It must not reimplement Bootstrap generation and must not keep a direct TOML-to-`RuntimeConfig`-to-Core startup path except as an explicitly superseded pre-Canon snippet.

Мета: чистий поділ `alife-core` від entry-point; state machine; scenario directory.

Build:

```text
виділити alife-runner як окремий crate (або чіткий binary module)
CLI: cargo run --bin runner -- <scenario.toml>
CLI debug: cargo run --bin runner -- --debug --progress-interval-ms 2000 <scenario.toml>
config/scenarios/ directory зі стартовими demo сценаріями
RunnerProcessState + ActiveRunState from docs/runner/run-lifecycle.md
ScenarioDocument resolution and canonical scenario_hash
Bootstrap boundary: ScenarioDocument -> PreparedWorld + BootstrapManifest
shared RunnerCommand enum and command dispatcher
RunStatusProjection for CLI/debug output
deterministic replay test uses same seed + canonical ScenarioDocument -> same result
deterministic replay test (same seed + ScenarioDocument + PreparedWorld -> same result)
```

Gate:

```text
headless run стартує і завершується детерміновано
state transitions покриті тестами
scenario TOML знаходиться і валідується при старті
toy simulation → очікуваний tick count
--debug prints a terminal status table every 2000 ms by default
--progress-interval-ms <N> changes the debug table refresh interval
debug progress output does not change deterministic final snapshots
```

Canon Gate Addendum:

```text
scenario source resolves to immutable ScenarioDocument before Bootstrap
Bootstrap prepares Tick 0 and executes no Tick
Bootstrap-1 deterministic constrained generation acceptance gate passes before Runner-1 implementation starts
StartRun goes Idle -> Preparing -> Running atomically
failed resolution/validation/bootstrap leaves no partial active World
StepRun is not part of Runner-1 headless fast path unless the run is explicitly Paused
```

---

### Runner-2: HTTP Command API

Мета: `--serve` прапорець; HTTP сервер; всі command endpoints.

Build:

```text
alife-viewer-server crate skeleton
HTTP server (tokio + axum)
config/server.toml + [server] block support
endpoints:
  GET /server/info
  GET /scenarios
  GET /scenarios/{id}
  GET /run/status
  POST /run/start
  POST /run/pause
  POST /run/resume
  POST /run/step
  POST /run/stop
shared RunnerCommand dispatcher behind every endpoint
HTTP handlers do not call Core or Bootstrap directly
POST /run/step maps to StepRun: exactly one committed Tick, Paused only
integration tests через reqwest або curl
```

Gate:

```text
cargo run --bin runner -- --serve config/scenarios/demo.toml стартує
GET /scenarios повертає список
POST /run/start з валідним scenario_id запускає симуляцію
GET /run/status відображає поточний стан
POST /run/pause зупиняє Tick loop
POST /run/resume відновлює
всі команди повертають 409 при неправильному стані
конфіг hash і seed присутні у /run/status
```

---

### Runner-3: WebSocket Frame Stream

Мета: WS /stream; binary frame encoding; multiple clients; time-based broadcast.

Build:

```text
WS /stream endpoint (axum WebSocket)
CommittedSnapshot -> WorldFrameProjection v1 -> binary frame encoder
CommittedSnapshot → binary frame encoder (версіонований формат ALIF v1)
time-based broadcast: push не частіше target_broadcast_fps (default 30fps)
незалежний підписник per connection (tokio::sync::broadcast)
slow client → RecvError::Lagged → skip, continue (не блокує Core)
status JSON messages при зміні RunState (start/pause/resume/stop)
initial status message при підключенні клієнта
```

Gate:

```text
два browser tabs отримують незалежні frame streams
повільний клієнт пропускає кадри без впливу на Core
frame містить tick і бінарний payload (ALIF magic bytes)
binary frame decoder може відновити Cell positions і lifecycle states
нові підключення отримують initial status без seek запитів
tick loop не тримає mutex під час broadcast
```

---

### Runner-4: Remote Viewer And Acceptance Hardening

Мета: remote viewer mode; повна валідація; error handling.

Build:

```text
allow_remote_viewer = true mode (0.0.0.0 + CORS)
allowed_origins validation
canonical ScenarioDocument hashing, not raw TOML hashing
Scenario Resolution / Bootstrap errors use stable Runner error categories
config validation errors через HTTP (не тільки CLI)
graceful shutdown (Ctrl+C зупиняє Core і закриває WS з'єднання)
reconnect handling (клієнт перепідключається — отримує статус + останній frame)
```

Gate:

```text
local mode блокує запити з інших IP
remote mode дозволяє з allowed_origins
reconnect отримує поточний стан без повтору команд
graceful shutdown закриває всі WS
config validation error відображається у POST /run/start response
```

---

## Core / Architecture Invariants

```text
alife-core не залежить від alife-runner або alife-viewer-server.
alife-viewer-server не змінює WorldState.
HTTP відповідь ніколи не містить uncommitted simulation state.
Core Tick ніколи не чекає на WS clients або HTTP responses.
Runner state machine є єдиним авторитетом для run state.
Команди від UI проходять Runner validation перед впливом на Core.
Сервер не зберігає frame history; scroll-back — відповідальність UI клієнта.
```

---

## Non-Goals

Не входять у цей план:

```text
alife-storage (SQLite, Parquet, binary event logs) → Phase 5
checkpoint / branch API → UI-3 Research
placement API → UI-3 Research
experiment queue (кілька sequential runs) → UI-2 Debug
Genome / lineage projections → Phase 3+
advanced viewport-filtered frames → Runner-3+ extension
Python / DuckDB analysis adapter → Phase 5
```

---

## Acceptance Gate

Runner план вважається завершеним, коли:

```text
headless run: cargo run --bin runner -- <scenario> стартує і завершується детерміновано
debug headless run: cargo run --bin runner -- --debug --progress-interval-ms 2000 <scenario> prints periodic status table while running
Scenario source resolves to immutable ScenarioDocument before any World generation
Bootstrap-1 Foundation acceptance gate passes before Runner-1 implementation starts
Bootstrap produces PreparedWorld + BootstrapManifest and executes no Tick
StartRun failure during resolution/validation/bootstrap leaves active run Idle/Failed without partial World
serve run: cargo run --bin runner -- --serve <scenario> стартує HTTP + WS
GET /scenarios повертає список з config/scenarios/
GET /scenarios/{id} повертає TOML вміст
POST /run/start запускає Core з коректним seed і config
GET /run/status відображає process_state, active_run_state, run_id, committed_tick, scenario_hash, seed
POST /run/pause і /run/resume працюють через HTTP
POST /run/step виконує рівно один committed Tick тільки з Paused і повертає Paused + committed_tick
POST /run/stop повертає в Idle
WS /stream отримує frame ≤target_broadcast_fps разів на секунду
два підключені клієнти отримують незалежні stream-и
Core ніколи не чекає на WS клієнтів
new WS connect → initial status message (idle/running/paused)
reconnect клієнта не повторює команди
той самий seed + той самий config → той самий результат
collapse зупиняє run з collapse_reason у /run/status
```

---

## Semantic Links

- implements adapter over: [[docs/implementation/architecture|Architecture]]
- exposes Core state through: [[docs/observer/observer-layer|Observer Layer]]
- serves data to: [[docs/implementation/implementation-plan-ui|UI Implementation Plan]]
- bounded by: [[docs/world/laws|World Laws]]
- uses config from: [[docs/config/INDEX|Config Index]]

## Пов'язані документи

- `docs/implementation/architecture.md`
- `docs/implementation/implementation-phases.md`
- `docs/implementation/implementation-plan-ui.md`
- `docs/engine/storage.md`
- `docs/engine/rendering.md`
- `docs/observer/observer-layer.md`
- `docs/config/INDEX.md`
