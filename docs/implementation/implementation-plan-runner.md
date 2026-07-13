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
docs/implementation/architecture.md
docs/implementation/implementation-phases.md
docs/engine/technology-stack.md
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
HTTP client sends commands to Runner.
Runner validates and applies to run state machine.
Core executes next Tick.
Runner reads new CommittedSnapshot.
viewer-server pushes to WS clients.
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
    state: "idle" | "running" | "paused" | "stopping",
    tick: N,
    scenario_id: "...",
    config_hash: "...",
    seed: N,
    ticks_per_second: N,
    collapse_reason: null | "..."
  }
```

### Run controls

```text
POST /run/start
  body: { scenario_id: "single_cell_survival", seed: 42 }
  → { ok: true, run_id: "...", config_hash: "...", seed: 42 }
  → 409 Conflict якщо run вже активний

POST /run/pause
  → { ok: true, tick: N }

POST /run/resume
  → { ok: true }

POST /run/step
  body: { ticks: 1 }
  → { ok: true, tick_after: N }

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

Мета: чистий поділ `alife-core` від entry-point; state machine; scenario directory.

Build:

```text
виділити alife-runner як окремий crate (або чіткий binary module)
CLI: cargo run --bin runner -- <scenario.toml>
config/scenarios/ directory зі стартовими demo сценаріями
run state machine (Idle / Running / Paused / Stopping)
deterministic replay test (same seed + config → same result)
```

Gate:

```text
headless run стартує і завершується детерміновано
state transitions покриті тестами
scenario TOML знаходиться і валідується при старті
toy simulation → очікуваний tick count
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
serve run: cargo run --bin runner -- --serve <scenario> стартує HTTP + WS
GET /scenarios повертає список з config/scenarios/
GET /scenarios/{id} повертає TOML вміст
POST /run/start запускає Core з коректним seed і config
GET /run/status відображає tick, state, config_hash, seed
POST /run/pause і /run/resume працюють через HTTP
POST /run/step виконує N Tick-ів і повертає tick_after
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
