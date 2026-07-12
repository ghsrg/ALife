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
- збирає committed snapshots у ring buffer;
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
  ring buffer management
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

### Frame streaming — full snapshot, ring buffer

```text
Core commits Snapshot[tick N]
  ↓ (in-memory, no wait)
Runner записує у ring_buffer[N % capacity]
  ↓ (async, non-blocking)
viewer-server зчитує і пушить всім WS-клієнтам
```

Core ніколи не чекає на WS-клієнтів. Повільний клієнт пропускає кадри і отримує найновіший. Це впливає виключно на плавність перегляду, не на Core.

Ring buffer дозволяє UI "прокрутити назад" у межах `snapshot_buffer_size` останніх Tick-ів.

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

### Scroll back

Клієнт може запросити frame за tick-ом у межах ring buffer:

```text
{ type: "seek", tick: N }
→ сервер відповідає frame для tick N (або найближчий доступний)
→ сервер продовжує push live frames
```

Якщо tick не в буфері — відповідь `{ type: "seek_error", reason: "not_in_buffer", oldest_available_tick: M }`.

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

## Ring Buffer

```toml
[server]
snapshot_buffer_size = 300   # кількість Tick-ів у пам'яті (scroll back window)
stream_frame_interval = 3    # пушити кожні N Tick-ів до WS клієнтів
```

`snapshot_buffer_size = 300` при 30 ticks/sec = 10 секунд scroll back.

Ring buffer живе виключно у пам'яті. Він не є Storage — для довгострокового replay використовується `alife-storage` (майбутній crate).

---

## Server Configuration

```toml
# config/server.toml (або вбудований у scenario config як [server] блок)

[server]
bind_host = "127.0.0.1"       # local-only за замовчуванням
port = 8080
allow_remote_viewer = false
snapshot_buffer_size = 300
stream_frame_interval = 3

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

Мета: чистий поділ `alife-core` від entry-point; state machine; ring buffer; scenario directory.

Build:

```text
виділити alife-runner як окремий crate (або чіткий binary module)
CLI: cargo run --bin runner -- <scenario.toml>
config/scenarios/ directory зі стартовими demo сценаріями
run state machine (Idle / Running / Paused / Stopping)
ring buffer (CommittedSnapshot, configurable size)
deterministic replay test через ring buffer
```

Gate:

```text
headless run стартує і завершується детерміновано
state transitions покриті тестами
ring buffer зберігає N останніх snapshot-ів
scenario TOML знаходиться і валідується при старті
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

Мета: WS /stream; binary frame encoding; multiple clients; scroll back.

Build:

```text
WS /stream endpoint (axum WebSocket або tokio-tungstenite)
CommittedSnapshot → binary frame encoder (версіонований формат)
push до всіх connected clients кожні stream_frame_interval Tick-ів
незалежний підписник per connection (slow client не блокує Core)
status messages при зміні state
seek by tick (scroll back в межах ring buffer)
seek_error коли tick поза буфером
```

Gate:

```text
два browser tabs отримують незалежні frame streams
повільний клієнт пропускає кадри без впливу на Core
seek до tick у ring buffer повертає правильний frame
seek поза буфером повертає seek_error з oldest_available_tick
frame містить tick і бінарний payload
binary frame decoder може відновити Cell positions і lifecycle states
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
Ring buffer — тільки in-memory; не є заміною Storage.
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
WS /stream отримує frame кожні stream_frame_interval Tick-ів
два підключені клієнти отримують незалежні stream-и
Core ніколи не чекає на WS клієнтів
seek у ring buffer повертає frame або seek_error
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
