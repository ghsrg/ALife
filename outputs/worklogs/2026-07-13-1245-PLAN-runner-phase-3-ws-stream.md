# Runner Phase 3 Implementation Plan

> **ADR:** Оновлено 2026-07-13. Scroll-back і seek прибрані з серверної відповідальності. Scroll-back — виключно client-side (browser). Сервер робить time-based broadcast (≤30fps). Причини: усунення mutex contention від `snap.clone()` у tick loop; спрощення WS handler; scroll-back у межах ~5 секунд не дає дослідницької цінності.

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Додати WebSocket endpoint `GET /stream` до viewer-server: time-based бінарний frame stream (≤30fps), незалежні підписники per WS-з'єднання, повільний клієнт пропускає кадри без впливу на Core, JSON status messages при зміні run state.

**Architecture:** `tokio::sync::broadcast::channel` — центральний hub for already-built projection messages. Runner tick loop observes committed Core state, builds `WorldFrameProjection v1`, encodes it to ALIF bytes, then sends through `Sender::send()` without holding the Runner/Core lock. Кожен WS-handler отримує незалежний `Receiver`. Slow client: `RecvError::Lagged(n)` — skip, continue. Status text messages are built from `RunStatusProjection`, not from local `RunState` labels. WS handler: `tokio::select!` між broadcast receiver і вхідними WS повідомленнями. **Немає ring buffer у SharedState. Немає seek на сервері. WS handlers never mutate Runner/Core state.**

**Tech Stack:** `axum 0.8 features=["ws"]`, `tokio::sync::broadcast`, `tokio-tungstenite 0.26` (dev-dep для WS test-клієнта), `futures-util 0.3` (dev-dep для WS stream).

**Передумови:** Runner-2 завершений і Canon-compatible: HTTP endpoints are adapters over shared `RunnerCommand` dispatcher; lifecycle uses `RunnerProcessState` + `ActiveRunState`; status comes from `RunStatusProjection`; snapshots are internal engine data only; `POST /run/step` remains paused-only and exactly one committed Tick.

---

## Canon Sync 2026-07-14

This plan is subordinate to the Runner Canon:

```text
docs/runner/runner.md
docs/runner/execution-modes.md
docs/runner/run-lifecycle.md
docs/runner/command-contract.md
docs/runner/scenario-resolution.md
docs/runner/projections.md
docs/runner/bootstrap.md
```

If any lower task snippet conflicts with these documents, implement the Canon behavior and update the snippet before coding.

Phase 3 must preserve the Runner-2 HTTP command contract:

- HTTP/CLI/UI adapters translate input into shared Runner commands.
- WS is push-only and never accepts control commands.
- `StartRun` still performs Scenario Resolution -> Bootstrap -> PreparedWorld validation -> Core start.
- `StepRun` remains Paused-only and commits exactly one Tick.
- `GetRunStatus` and WS status messages are read-only projections and must not mutate state.
- No server-side frame history, seek, or scroll-back contract is introduced.

Use the existing Bootstrap-compatible scenario fixture unless a later plan adds a new fixture:

```text
bootstrap_minimal_viable_world
config/scenarios/bootstrap/minimal_viable_world.toml
```

Do not use `single_cell_survival` in tests or smoke commands unless that scenario is explicitly added first.

Canonical status payload fields for Runner-3:

```text
type = "status"              # WS envelope only
process_state
active_run_state
run_id
committed_tick
scenario_id
scenario_hash
effective_seed
terminal_reason
```

`ticks_per_second` and richer performance metrics are useful but remain out of scope for this WS-stream slice unless already available as a projection field.

Implementation rule: build WS status text through one shared `RunStatusProjection` -> WS envelope helper. Do not duplicate status-field selection independently in `stream.rs` and `run.rs`. Any `serde_json::json!` snippets below are illustrative and must be replaced or routed through the shared projection helper during implementation.

---

## Binary Frame Format v1 (ALIF)

```
Offset  Size  Type     Description
0       4     bytes    Magic: b"ALIF" (0x41 0x4C 0x49 0x46)
4       1     u8       Version: 0x01
5       1     u8       Reserved: 0x00
6       8     u64 LE   Tick
14      4     f32 LE   Heat
18      4     f32 LE   Waste
22      4     u32 LE   Cell count
26      N*21  struct   CellFrame × cell_count

CellFrame (21 bytes each):
  0     4     u32 LE   Cell ID
  4     4     f32 LE   X position
  8     4     f32 LE   Y position
  12    4     f32 LE   Radius
  16    4     f32 LE   Energy
  20    1     u8       LifecycleState (0=Alive 1=Stressed 2=Dormant 3=Dead)
```

Total header: 26 bytes. Per cell: 21 bytes.

---

## WS Message Protocol (server → client only)

Server → Client (text JSON, serialized `RunStatusProjection` envelope):
```json
{
  "type": "status",
  "process_state": "ready",
  "active_run_state": "running",
  "run_id": "run-...",
  "committed_tick": 42,
  "scenario_id": "bootstrap_minimal_viable_world",
  "scenario_hash": "scenario_hash_v1:...",
  "effective_seed": 42,
  "terminal_reason": null
}
```

Server → Client (binary):
```
<ALIF v1 frame bytes>
```

Client → Server: **нічого** — WS є push-only. Клієнт не надсилає seek чи інші команди по WS. Всі команди (start, pause, stop) — тільки через HTTP.

---

## Scroll-back — відповідальність UI клієнта

```
Browser FrameBuffer (JavaScript):
  live mode:   отримує ≤30fps → зберігає 1fps (кожен 30-й кадр)
               max 60 кадрів (~1 хв) → evict oldest при переповненні
  paused mode: отримує кадри → зберігає всі → max 60 → evict oldest
  seek:        scroll currentIndex у локальному масиві
  "jump to live": currentIndex = buffer.length - 1, live mode
```

Сервер не знає про scroll-back. Клієнт самостійно вирішує що тримати і коли відкидати.

---

## Tick Loop Critical Path (спрощений)

```rust
// В std::thread (без tokio):
let mut last_broadcast = Instant::now();
let frame_interval = Duration::from_millis(1000 / target_fps as u64);

loop {
    if signal.is_stop_requested() { break; }
    if signal.is_pause_requested() { sleep(10ms); continue; }

    // Крок симуляції — lock тільки на час committed Tick + projection build.
    let maybe_frame: Option<Vec<u8>> = {
        let mut locked = state.lock().unwrap();
        locked.engine.as_mut()?.run_one_tick()?;
        locked.committed_tick = engine.current_tick();
        let should_broadcast = last_broadcast.elapsed() >= frame_interval;
        should_broadcast.then(|| {
            let projection = WorldFrameProjection::from_committed_state(engine.snapshots().newest()?);
            encode_world_frame(&projection)
        })
    }; // unlock

    if let Some(bytes) = maybe_frame {
        broadcast_sender.send(WsMessage::Frame(bytes)).ok(); // без lock
        last_broadcast = Instant::now();
    }
}
```

Ключові гарантії:
- Public wire schema is `WorldFrameProjection v1`; `CommittedSnapshot` is internal input only
- `Vec<CellSnapshot>` **не клонується як server history** — projection builder writes a bounded frame projection for immediate broadcast
- Mutex відпущено до виклику `broadcast_sender.send()`
- Slow клієнт ніколи не затримує tick loop

---

## Canon Supersession: Runner-3 Projections

This phase must use `docs/runner/projections.md` as the public data contract.

Required projection pipeline:

```text
Committed Core State -> Projection Builder -> WorldFrameProjection v1 -> ALIF frame bytes
```

The binary frame encoder must not expose internal `WorldState` and should not treat `CommittedSnapshot` as the public wire schema. `CommittedSnapshot` may be the input to the projection builder, but the externally documented payload is `WorldFrameProjection v1`.

Required projection modules:

```text
src/runner/projections.rs              [MODIFY] RunStatusProjection, WorldFrameProjection
src/viewer_server/frame_encoder.rs     [NEW] encode WorldFrameProjection v1 -> ALIF bytes
tests/runner_projection_world_frame.rs [NEW] projection schema tests
```

Status messages use `RunStatusProjection` fields:

```text
process_state
active_run_state
run_id
committed_tick
scenario_id
scenario_hash
effective_seed
terminal_reason
```

No WS handler may mutate Runner or Core state. WS remains push-only; commands stay HTTP/CLI adapters over the shared Runner command dispatcher.

The existing no-server-scroll-back decision remains valid:

```text
No server seek.
No server frame history contract.
Client-side scroll-back only.
```

Additional acceptance:

```text
WorldFrameProjection v1 declares schema version and committed Tick
ALIF encoder accepts WorldFrameProjection v1, not mutable WorldState
WS initial status uses RunStatusProjection
slow clients cannot block Tick execution or projection building
```

---

## File Structure

```
src/
  viewer_server/
    frame_encoder.rs   [NEW] — encode WorldFrameProjection v1 → Vec<u8> (ALIF v1)
    broadcaster.rs     [NEW] — WsMessage enum, Broadcaster (broadcast::Sender wrapper)
    api/
      stream.rs        [NEW] — GET /stream WS handler + pump loop
      mod.rs           [MODIFY] — register /stream route
      run.rs           [MODIFY] — broadcast status on state changes
    state.rs           [MODIFY] — add Broadcaster + target_fps; remove public frame history; update Runner-owned tick loop integration
    mod.rs             [MODIFY] — pub mod frame_encoder, broadcaster
  bin/
    runner.rs          [MODIFY] — pass target_broadcast_fps to new_app_state
  lib.rs               [no change]
Cargo.toml             [MODIFY] — axum ws feature; tokio-tungstenite + futures-util dev-dep
tests/
  runner_projection_world_frame.rs [NEW] — projection schema tests
  runner_frame_encoder.rs  [NEW] — WorldFrameProjection encode/decode unit tests (sync)
  runner_ws_stream.rs      [NEW] — WS integration tests (real port + tokio-tungstenite)
```

---

## Task 1: WorldFrameProjection and Binary Frame Encoder

**Files:**
- Modify: `src/runner/projections.rs`
- Create: `src/viewer_server/frame_encoder.rs`
- Modify: `src/viewer_server/mod.rs` — pub mod frame_encoder
- Test: `tests/runner_projection_world_frame.rs`
- Test: `tests/runner_frame_encoder.rs`

- [ ] **Step 1: Write the failing tests**

```rust
// tests/runner_frame_encoder.rs
use alife::runner::projections::{ProjectedCell, WorldFrameProjection};
use alife::viewer_server::frame_encoder::{decode_frame, encode_world_frame, MAGIC, VERSION};

fn make_projection(tick: u64, cell_count: usize) -> WorldFrameProjection {
    WorldFrameProjection {
        schema_version: 1,
        committed_tick: tick,
        heat: 1.5,
        waste: 0.25,
        cells: (0..cell_count)
            .map(|i| ProjectedCell {
                id: i as u32,
                x: i as f32 * 10.0,
                y: i as f32 * 5.0,
                radius: 4.0 + i as f32,
                energy: 50.0 + i as f32,
                lifecycle: 0,
            })
            .collect(),
    }
}

#[test]
fn encoded_frame_starts_with_magic_bytes() {
    let frame = make_projection(1, 0);
    let bytes = encode_world_frame(&frame);
    assert_eq!(&bytes[0..4], MAGIC, "First 4 bytes must be ALIF magic");
}

#[test]
fn encoded_frame_has_correct_version() {
    let frame = make_projection(1, 0);
    let bytes = encode_world_frame(&frame);
    assert_eq!(bytes[4], VERSION);
}

#[test]
fn encoded_frame_encodes_tick_correctly() {
    let frame = make_projection(999, 0);
    let bytes = encode_world_frame(&frame);
    let tick = u64::from_le_bytes(bytes[6..14].try_into().unwrap());
    assert_eq!(tick, 999);
}

#[test]
fn encoded_frame_encodes_heat_and_waste() {
    let frame = make_projection(1, 0);
    let bytes = encode_world_frame(&frame);
    let heat = f32::from_le_bytes(bytes[14..18].try_into().unwrap());
    let waste = f32::from_le_bytes(bytes[18..22].try_into().unwrap());
    assert!((heat - 1.5).abs() < 1e-5);
    assert!((waste - 0.25).abs() < 1e-5);
}

#[test]
fn encoded_frame_with_zero_cells_has_26_bytes() {
    let frame = make_projection(1, 0);
    let bytes = encode_world_frame(&frame);
    assert_eq!(bytes.len(), 26);
}

#[test]
fn encoded_frame_cell_count_and_size_match() {
    let frame = make_projection(1, 3);
    let bytes = encode_world_frame(&frame);
    let count = u32::from_le_bytes(bytes[22..26].try_into().unwrap());
    assert_eq!(count, 3);
    assert_eq!(bytes.len(), 26 + 3 * 21);
}

#[test]
fn encode_decode_roundtrip_preserves_tick_and_cell_count() {
    let frame = make_projection(42, 2);
    let bytes = encode_world_frame(&frame);
    let decoded = decode_frame(&bytes).expect("decode must succeed");
    assert_eq!(decoded.tick, 42);
    assert_eq!(decoded.cells.len(), 2);
}

#[test]
fn encode_decode_roundtrip_preserves_cell_fields() {
    let frame = make_projection(10, 1);
    let bytes = encode_world_frame(&frame);
    let decoded = decode_frame(&bytes).expect("decode must succeed");
    let cell = &decoded.cells[0];
    assert_eq!(cell.id, 0);
    assert!((cell.x - 0.0).abs() < 1e-4);
    assert!((cell.y - 0.0).abs() < 1e-4);
    assert!((cell.radius - 4.0).abs() < 1e-4);
    assert!((cell.energy - 50.0).abs() < 1e-4);
    assert_eq!(cell.lifecycle, 0u8);
}

#[test]
fn lifecycle_states_encode_correctly() {
    fn state_byte(lifecycle: u8) -> u8 {
        let mut frame = make_projection(0, 1);
        frame.cells[0].lifecycle = lifecycle;
        let bytes = encode_world_frame(&frame);
        bytes[26 + 20] // lifecycle byte in first cell
    }
    assert_eq!(state_byte(0), 0);
    assert_eq!(state_byte(1), 1);
    assert_eq!(state_byte(2), 2);
    assert_eq!(state_byte(3), 3);
}

#[test]
fn decode_returns_error_on_wrong_magic() {
    let mut bytes = encode_world_frame(&make_projection(1, 0));
    bytes[0] = 0xFF;
    assert!(decode_frame(&bytes).is_err());
}

#[test]
fn decode_returns_error_on_truncated_header() {
    assert!(decode_frame(&vec![0u8; 10]).is_err());
}
```

- [ ] **Step 2: Run to verify failure**

```bash
cargo test --test runner_frame_encoder
```

Expected: compile error — `alife::viewer_server::frame_encoder` does not exist.

- [ ] **Step 3: Create `src/viewer_server/frame_encoder.rs`**

```rust
//! Binary frame encoder: WorldFrameProjection v1 → ALIF v1 wire format.
//!
//! Header (26 bytes):
//!   [0..4]   b"ALIF"
//!   [4]      version 0x01
//!   [5]      reserved 0x00
//!   [6..14]  tick u64 LE
//!   [14..18] heat f32 LE
//!   [18..22] waste f32 LE
//!   [22..26] cell_count u32 LE
//!
//! CellFrame (21 bytes):
//!   [0..4]   id u32 LE
//!   [4..8]   x f32 LE
//!   [8..12]  y f32 LE
//!   [12..16] radius f32 LE
//!   [16..20] energy f32 LE
//!   [20]     lifecycle u8 (0=Alive 1=Stressed 2=Dormant 3=Dead)

use crate::runner::projections::WorldFrameProjection;

pub const MAGIC: &[u8; 4] = b"ALIF";
pub const VERSION: u8 = 1;

const HEADER_SIZE: usize = 26;
const CELL_SIZE: usize = 21;

/// Encode a WorldFrameProjection into ALIF v1 binary format.
/// No heap allocations beyond the output Vec.
pub fn encode_world_frame(frame: &WorldFrameProjection) -> Vec<u8> {
    let cell_count = frame.cells.len();
    let mut buf = Vec::with_capacity(HEADER_SIZE + cell_count * CELL_SIZE);

    buf.extend_from_slice(MAGIC);
    buf.push(VERSION);
    buf.push(0x00);
    buf.extend_from_slice(&frame.committed_tick.to_le_bytes());
    buf.extend_from_slice(&frame.heat.to_le_bytes());
    buf.extend_from_slice(&frame.waste.to_le_bytes());
    buf.extend_from_slice(&(cell_count as u32).to_le_bytes());

    for cell in &frame.cells {
        buf.extend_from_slice(&cell.id.to_le_bytes());
        buf.extend_from_slice(&cell.x.to_le_bytes());
        buf.extend_from_slice(&cell.y.to_le_bytes());
        buf.extend_from_slice(&cell.radius.to_le_bytes());
        buf.extend_from_slice(&cell.energy.to_le_bytes());
        buf.push(cell.lifecycle);
    }

    buf
}

/// Decoded cell — used in tests and future UI TypeScript adapter generation.
#[derive(Debug, PartialEq)]
pub struct DecodedCell {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub energy: f32,
    pub lifecycle: u8,
}

/// Decoded frame — used in tests and future UI TypeScript adapter generation.
#[derive(Debug)]
pub struct DecodedFrame {
    pub tick: u64,
    pub heat: f32,
    pub waste: f32,
    pub cells: Vec<DecodedCell>,
}

/// Decode ALIF v1 bytes. Returns Err on wrong magic, truncation, or bad version.
pub fn decode_frame(bytes: &[u8]) -> Result<DecodedFrame, String> {
    if bytes.len() < HEADER_SIZE {
        return Err(format!(
            "Frame too short: {} bytes (min {})",
            bytes.len(), HEADER_SIZE
        ));
    }
    if &bytes[0..4] != MAGIC {
        return Err(format!("Invalid magic: {:?}", &bytes[0..4]));
    }

    let tick       = u64::from_le_bytes(bytes[6..14].try_into().unwrap());
    let heat       = f32::from_le_bytes(bytes[14..18].try_into().unwrap());
    let waste      = f32::from_le_bytes(bytes[18..22].try_into().unwrap());
    let cell_count = u32::from_le_bytes(bytes[22..26].try_into().unwrap()) as usize;

    let expected = HEADER_SIZE + cell_count * CELL_SIZE;
    if bytes.len() < expected {
        return Err(format!(
            "Truncated: {} bytes for {} cells (need {})",
            bytes.len(), cell_count, expected
        ));
    }

    let cells = (0..cell_count)
        .map(|i| {
            let b = HEADER_SIZE + i * CELL_SIZE;
            DecodedCell {
                id:        u32::from_le_bytes(bytes[b..b+4].try_into().unwrap()),
                x:         f32::from_le_bytes(bytes[b+4..b+8].try_into().unwrap()),
                y:         f32::from_le_bytes(bytes[b+8..b+12].try_into().unwrap()),
                radius:    f32::from_le_bytes(bytes[b+12..b+16].try_into().unwrap()),
                energy:    f32::from_le_bytes(bytes[b+16..b+20].try_into().unwrap()),
                lifecycle: bytes[b+20],
            }
        })
        .collect();

    Ok(DecodedFrame { tick, heat, waste, cells })
}
```

- [ ] **Step 4: Expose in `src/viewer_server/mod.rs`** (додати `pub mod frame_encoder;`)

- [ ] **Step 5: Run tests**

```bash
cargo test --test runner_frame_encoder
```

Expected: всі 10 тестів `PASS`.

- [ ] **Step 6: Commit**

```bash
git add src/viewer_server/frame_encoder.rs src/viewer_server/mod.rs \
        tests/runner_frame_encoder.rs
git commit -m "feat(viewer-server): add ALIF v1 binary frame encoder/decoder"
```

---

## Task 2: WsMessage and Broadcaster

**Files:**
- Create: `src/viewer_server/broadcaster.rs`
- Modify: `src/viewer_server/mod.rs` — pub mod broadcaster
- Modify: `Cargo.toml` — axum ws feature

- [ ] **Step 1: Update `Cargo.toml`**

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
serde_json = "1.0"
axum = { version = "0.8", features = ["ws"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
tower = { version = "0.5", features = ["util"] }
http-body-util = "0.1"
reqwest = { version = "0.12", features = ["json"] }
tokio-tungstenite = "0.26"
futures-util = "0.3"
```

- [ ] **Step 2: Create `src/viewer_server/broadcaster.rs`**

```rust
//! Broadcast hub for WebSocket frame streaming.
//!
//! The tick loop (std::thread) calls send_frame() or send_status() synchronously.
//! Each WS handler gets an independent Receiver via subscribe().
//! Slow subscribers get RecvError::Lagged(n) — they skip missed frames and continue.

use tokio::sync::broadcast;

/// Messages pushed from tick loop / HTTP handlers to all WS subscribers.
#[derive(Clone, Debug)]
pub enum WsMessage {
    /// ALIF v1 binary frame.
    Frame(Vec<u8>),
    /// JSON status, e.g. {"type":"status","active_run_state":"running","committed_tick":42,"scenario_id":"bootstrap_minimal_viable_world","effective_seed":42}
    Status(String),
}

/// Thread-safe broadcast hub. Sender can be cloned and used from std::thread.
pub struct Broadcaster {
    sender: broadcast::Sender<WsMessage>,
}

impl Broadcaster {
    /// `capacity` = max messages buffered per subscriber before Lagged.
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    /// Subscribe a new WS client. Returns an independent async receiver.
    pub fn subscribe(&self) -> broadcast::Receiver<WsMessage> {
        self.sender.subscribe()
    }

    /// Clone the underlying sender for passing into std::thread.
    pub fn sender(&self) -> broadcast::Sender<WsMessage> {
        self.sender.clone()
    }

    /// Broadcast a binary frame (sync — safe from std::thread).
    pub fn send_frame(&self, bytes: Vec<u8>) {
        self.sender.send(WsMessage::Frame(bytes)).ok();
    }

    /// Broadcast a JSON status string (sync — safe from std::thread).
    pub fn send_status(&self, text: String) {
        self.sender.send(WsMessage::Status(text)).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sends_frame_to_single_subscriber() {
        let b = Broadcaster::new(16);
        let mut rx = b.subscribe();
        b.send_frame(vec![0x41, 0x4C, 0x49, 0x46]);
        assert!(matches!(rx.recv().await.unwrap(), WsMessage::Frame(_)));
    }

    #[tokio::test]
    async fn sends_status_to_subscriber() {
        let b = Broadcaster::new(16);
        let mut rx = b.subscribe();
        b.send_status(r#"{"type":"status"}"#.to_string());
        assert!(matches!(rx.recv().await.unwrap(), WsMessage::Status(_)));
    }

    #[tokio::test]
    async fn delivers_to_multiple_subscribers_independently() {
        let b = Broadcaster::new(16);
        let mut rx1 = b.subscribe();
        let mut rx2 = b.subscribe();
        b.send_frame(vec![1, 2, 3]);
        assert!(matches!(rx1.recv().await.unwrap(), WsMessage::Frame(_)));
        assert!(matches!(rx2.recv().await.unwrap(), WsMessage::Frame(_)));
    }

    #[tokio::test]
    async fn slow_subscriber_gets_lagged_and_can_recover() {
        let b = Broadcaster::new(2);
        let mut rx = b.subscribe();
        b.send_frame(vec![1]);
        b.send_frame(vec![2]);
        b.send_frame(vec![3]); // causes lag
        use tokio::sync::broadcast::error::RecvError;
        loop {
            match rx.recv().await {
                Ok(_) => break,
                Err(RecvError::Lagged(_)) => continue,
                Err(e) => panic!("unexpected: {:?}", e),
            }
        }
    }

    #[test]
    fn sender_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<broadcast::Sender<WsMessage>>();
    }
}
```

- [ ] **Step 3: Expose in `src/viewer_server/mod.rs`** (додати `pub mod broadcaster;`)

- [ ] **Step 4: Run tests**

```bash
cargo test -p alife viewer_server::broadcaster
```

Expected: всі 5 internal tests `PASS`.

- [ ] **Step 5: Commit**

```bash
git add src/viewer_server/broadcaster.rs src/viewer_server/mod.rs Cargo.toml
git commit -m "feat(viewer-server): add Broadcaster over tokio::broadcast with WsMessage"
```

---

## Task 3: Integrate Broadcaster into SharedState, update tick loop

**Files:**
- Modify: `src/viewer_server/state.rs`
- Modify: `src/bin/runner.rs`

> **Canon patch:** do not rewrite `SharedState` into a simplified WS-only state. Extend the Runner-2 state in place. The state must keep `process_state`, `active_run_state`, `run_id`, `scenario_id`, `scenario_hash`, `effective_seed`, `committed_tick`, `terminal_reason`, `engine`, `tick_signal`, `scenarios_dir`, and `engine_snapshot_buffer_size`. Add only WS-specific fields such as `target_broadcast_fps` and `broadcaster`.
>
> Any code block below that omits existing Runner-2 lifecycle/status fields is illustrative only and must be adapted before implementation.

Ключові зміни vs Runner-2:
- Не додавати server-side frame history або public scroll-back state
- Замінити legacy `stream_frame_interval: u32` на `target_broadcast_fps: u32`
- Додати `broadcaster: Broadcaster`
- `spawn_tick_loop`: time-based broadcast, жодного `snap.clone()`

- [ ] **Step 1: Rewrite `src/viewer_server/state.rs`**

```rust
use crate::runner::engine::RunEngine;
use crate::runner::lifecycle::ActiveRunState;
use crate::runner::projections::WorldFrameProjection;
use crate::viewer_server::broadcaster::{Broadcaster, WsMessage};
use crate::viewer_server::frame_encoder::encode_world_frame;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct TickLoopSignal {
    stop:  Arc<AtomicBool>,
    pause: Arc<AtomicBool>,
}

impl TickLoopSignal {
    pub fn new() -> Self {
        Self {
            stop:  Arc::new(AtomicBool::new(false)),
            pause: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn request_stop(&self)   { self.stop.store(true, Ordering::Relaxed); }
    pub fn request_pause(&self)  { self.pause.store(true, Ordering::Relaxed); }
    pub fn request_resume(&self) { self.pause.store(false, Ordering::Relaxed); }
    pub fn is_stop_requested(&self)  -> bool { self.stop.load(Ordering::Relaxed) }
    pub fn is_pause_requested(&self) -> bool { self.pause.load(Ordering::Relaxed) }
}

pub struct SharedState {
    pub engine:               Option<RunEngine>,
    pub active_run_state:     ActiveRunState,
    pub scenario_id:          Option<String>,
    pub committed_tick:       u64,
    pub terminal_reason:      Option<String>,
    pub scenarios_dir:        PathBuf,
    pub tick_signal:          Option<Arc<TickLoopSignal>>,
    pub engine_snapshot_buffer_size: usize,  // internal RunEngine config only; not scroll-back
    /// Max frames per second pushed to WS clients.
    pub target_broadcast_fps: u32,
    /// Broadcast hub — tick loop sends here; WS handlers subscribe.
    pub broadcaster:          Broadcaster,
}

impl SharedState {
    pub fn new(
        scenarios_dir: PathBuf,
        snapshot_buffer_size: usize,
        target_broadcast_fps: u32,
    ) -> Self {
        Self {
            engine: None,
            active_run_state: ActiveRunState::Idle,
            scenario_id: None,
            committed_tick: 0,
            terminal_reason: None,
            scenarios_dir,
            tick_signal: None,
            engine_snapshot_buffer_size: snapshot_buffer_size,
            target_broadcast_fps,
            broadcaster: Broadcaster::new(128), // 128 messages buffered per subscriber
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.active_run_state, ActiveRunState::Running | ActiveRunState::Paused)
    }
}

pub type AppState = Arc<Mutex<SharedState>>;

pub fn new_app_state(
    scenarios_dir: PathBuf,
    snapshot_buffer_size: usize,
    target_broadcast_fps: u32,
) -> AppState {
    Arc::new(Mutex::new(SharedState::new(
        scenarios_dir,
        snapshot_buffer_size,
        target_broadcast_fps,
    )))
}

/// Spawn the background tick loop.
/// Assumes engine is initialised and active_run_state == Running.
pub fn spawn_tick_loop(state: AppState) {
    let signal = Arc::new(TickLoopSignal::new());

    // Clone sender + fps BEFORE spawning — no borrow into thread
    let (broadcast_sender, fps) = {
        let locked = state.lock().unwrap();
        (locked.broadcaster.sender(), locked.target_broadcast_fps)
    };

    {
        let mut locked = state.lock().unwrap();
        locked.tick_signal = Some(Arc::clone(&signal));
    }

    std::thread::spawn(move || {
        let frame_interval = Duration::from_millis(1000 / fps.max(1) as u64);
        let mut last_broadcast = Instant::now() - frame_interval; // broadcast on first tick

        loop {
            if signal.is_stop_requested() { break; }

            if signal.is_pause_requested() {
                std::thread::sleep(Duration::from_millis(10));
                continue;
            }

            // Step engine + optionally capture frame bytes — all under one lock
            let maybe_frame: Option<Vec<u8>> = {
                let mut locked = state.lock().unwrap();
                let should_broadcast = last_broadcast.elapsed() >= frame_interval;

                let engine = match locked.engine.as_mut() {
                    Some(e) => e,
                    None => break,
                };

                if engine.run_one_tick().is_err() {
                    locked.active_run_state = ActiveRunState::Failed;
                    locked.terminal_reason = Some("core_error".to_string());
                    break;
                }

                locked.committed_tick = engine.current_tick();

                if should_broadcast {
                    engine.snapshots().newest().map(|snap| {
                        let projection = WorldFrameProjection::from_committed_state(snap);
                        encode_world_frame(&projection)
                    })
                } else {
                    None
                }
            }; // mutex released here

            if let Some(bytes) = maybe_frame {
                broadcast_sender.send(WsMessage::Frame(bytes)).ok();
                last_broadcast = Instant::now();
            }
        }

        let mut locked = state.lock().unwrap();
        if matches!(locked.active_run_state, ActiveRunState::Running) {
            locked.active_run_state = ActiveRunState::Completed;
        }
        locked.tick_signal = None;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make() -> AppState {
        new_app_state(std::path::PathBuf::from("config/scenarios"), 20, 30)
    }

    #[test]
    fn broadcaster_is_ready_after_creation() {
        let state = make();
        let locked = state.lock().unwrap();
        let _rx = locked.broadcaster.subscribe(); // must not panic
    }

    #[test]
    fn tick_signal_initially_none() {
        let state = make();
        assert!(state.lock().unwrap().tick_signal.is_none());
    }
}
```

- [ ] **Step 2: Update `src/bin/runner.rs`**

Замінити всі виклики `new_app_state(dir, size, interval)` на `new_app_state(dir, size, fps)`:

```rust
let state = new_app_state(scenarios_dir, 300, cfg.target_broadcast_fps);
```

- [ ] **Step 3: Update `src/runner/server_config.rs`**

Замінити `stream_frame_interval: u32` на `target_broadcast_fps: u32`:

```rust
pub struct ServerConfig {
    pub bind_host: String,
    pub port: u16,
    pub allow_remote_viewer: bool,
    pub snapshot_buffer_size: usize,
    pub target_broadcast_fps: u32,    // ← замість stream_frame_interval
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_host: "127.0.0.1".to_string(),
            port: 8080,
            allow_remote_viewer: false,
            snapshot_buffer_size: 300,
            target_broadcast_fps: 30,
        }
    }
}
```

Також оновити `config/server.toml`:

```toml
[server]
bind_host = "127.0.0.1"
port = 8080
allow_remote_viewer = false
snapshot_buffer_size = 300
target_broadcast_fps = 30
```

- [ ] **Step 4: Fix callers in test files**

В `tests/runner_http_*.rs` виклики `new_app_state(path, 10)` → `new_app_state(path, 10, 30)`.

- [ ] **Step 5: Build**

```bash
cargo build
```

Expected: без помилок.

- [ ] **Step 6: Run full suite**

```bash
cargo test --workspace
```

Expected: всі попередні тести `PASS` (server_config, http_info, http_scenarios, http_run_control, frame_encoder, broadcaster).

- [ ] **Step 7: Commit**

```bash
git add src/viewer_server/state.rs src/runner/server_config.rs \
        src/bin/runner.rs config/server.toml tests/
git commit -m "feat(viewer-server): integrate Broadcaster + time-based fps into tick loop; remove ring buffer"
```

---

## Task 4: WS /stream endpoint — connect, initial status, pump loop

**Files:**
- Create: `src/viewer_server/api/stream.rs`
- Modify: `src/viewer_server/api/mod.rs` — register /stream
- Test: `tests/runner_ws_stream.rs`

> **Canon/test patch:** use `bootstrap_minimal_viable_world` for all start requests. Because the scenario is intentionally short, tests that poll `/run/status` after start may observe either `running` or `completed`. Tests that need to prove a `running` status broadcast should listen to the WS status message emitted immediately by `StartRun`, not infer it from a later poll.

- [ ] **Step 1: Write the failing WS tests**

```rust
// tests/runner_ws_stream.rs
use alife::viewer_server::{create_app, state::new_app_state};
use futures_util::StreamExt;
use std::path::PathBuf;
use tokio_tungstenite::{connect_async, tungstenite::Message};

fn scenarios_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios")
}

async fn spawn_test_server() -> (String, alife::viewer_server::state::AppState) {
    let state = new_app_state(scenarios_dir(), 50, 30); // 30fps
    let app = create_app(state.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.ok(); });
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    (format!("http://{}", addr), state)
}

// ── Task 4: Connect and initial status ───────────────────────────────────────

#[tokio::test]
async fn ws_connect_receives_initial_status_idle() {
    let (base, _) = spawn_test_server().await;
    let ws_url = base.replace("http://", "ws://") + "/stream";
    let (mut ws, _) = connect_async(&ws_url).await.unwrap();

    let msg = tokio::time::timeout(
        std::time::Duration::from_millis(500),
        ws.next(),
    ).await.expect("timeout").unwrap().unwrap();

    assert!(matches!(msg, Message::Text(_)), "Initial must be text JSON");
    let text = match msg { Message::Text(t) => t, _ => unreachable!() };
    let json: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(json["type"].as_str().unwrap(), "status");
    assert_eq!(json["active_run_state"].as_str().unwrap(), "idle");
}

#[tokio::test]
async fn two_ws_clients_both_receive_initial_status() {
    let (base, _) = spawn_test_server().await;
    let ws_url = base.replace("http://", "ws://") + "/stream";
    let (mut ws1, _) = connect_async(&ws_url).await.unwrap();
    let (mut ws2, _) = connect_async(&ws_url).await.unwrap();
    let t = std::time::Duration::from_millis(500);
    let m1 = tokio::time::timeout(t, ws1.next()).await.unwrap().unwrap().unwrap();
    let m2 = tokio::time::timeout(t, ws2.next()).await.unwrap().unwrap().unwrap();
    assert!(matches!(m1, Message::Text(_)));
    assert!(matches!(m2, Message::Text(_)));
}

// ── Task 5: Binary frames from tick loop ─────────────────────────────────────

#[tokio::test]
async fn ws_receives_binary_alif_frame_after_start() {
    let (base, _) = spawn_test_server().await;
    let ws_url = base.replace("http://", "ws://") + "/stream";
    let (mut ws, _) = connect_async(&ws_url).await.unwrap();

    // Drain initial status
    let _ = tokio::time::timeout(std::time::Duration::from_millis(300), ws.next())
        .await.unwrap().unwrap().unwrap();

    // Start simulation
    let client = reqwest::Client::new();
    client.post(format!("{}/run/start", base))
        .json(&serde_json::json!({ "scenario_id": "bootstrap_minimal_viable_world" }))
        .send().await.unwrap();

    // Wait for binary frame (up to 1 sec)
    let binary = tokio::time::timeout(std::time::Duration::from_secs(1), async {
        loop {
            let msg = ws.next().await.unwrap().unwrap();
            if let Message::Binary(b) = msg { return b; }
        }
    }).await.expect("timed out waiting for binary frame");

    assert_eq!(&binary[0..4], b"ALIF", "Must start with ALIF magic");
    assert_eq!(binary[4], 1, "Version must be 1");
    assert!(binary.len() >= 26);

    client.post(format!("{}/run/stop", base)).send().await.unwrap();
}

#[tokio::test]
async fn slow_ws_client_does_not_block_simulation() {
    let (base, state) = spawn_test_server().await;
    let ws_url = base.replace("http://", "ws://") + "/stream";
    let (_ws, _) = connect_async(&ws_url).await.unwrap(); // connected but not reading

    let client = reqwest::Client::new();
    client.post(format!("{}/run/start", base))
        .json(&serde_json::json!({ "scenario_id": "bootstrap_minimal_viable_world" }))
        .send().await.unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let tick = state.lock().unwrap().committed_tick;
    assert!(tick > 0, "Simulation must advance despite slow WS client");

    client.post(format!("{}/run/stop", base)).send().await.unwrap();
}

// ── Task 6: Status broadcasts on state changes ────────────────────────────────

#[tokio::test]
async fn ws_receives_running_status_after_http_start() {
    let (base, _) = spawn_test_server().await;
    let ws_url = base.replace("http://", "ws://") + "/stream";
    let (mut ws, _) = connect_async(&ws_url).await.unwrap();

    // Drain initial idle status
    let _ = tokio::time::timeout(std::time::Duration::from_millis(300), ws.next())
        .await.unwrap().unwrap().unwrap();

    let client = reqwest::Client::new();
    client.post(format!("{}/run/start", base))
        .json(&serde_json::json!({ "scenario_id": "bootstrap_minimal_viable_world" }))
        .send().await.unwrap();

    // Wait for "running" status
    let json = tokio::time::timeout(std::time::Duration::from_millis(800), async {
        loop {
            let msg = ws.next().await.unwrap().unwrap();
            if let Message::Text(t) = msg {
                let j: serde_json::Value = serde_json::from_str(&t).unwrap_or_default();
                if j["type"] == "status" && j["active_run_state"] == "running" { return j; }
            }
        }
    }).await.expect("timed out waiting for running status");

    assert_eq!(json["active_run_state"].as_str().unwrap(), "running");
    client.post(format!("{}/run/stop", base)).send().await.unwrap();
}

#[tokio::test]
async fn ws_receives_idle_status_after_http_stop() {
    let (base, _) = spawn_test_server().await;
    let ws_url = base.replace("http://", "ws://") + "/stream";
    let (mut ws, _) = connect_async(&ws_url).await.unwrap();

    let client = reqwest::Client::new();
    client.post(format!("{}/run/start", base))
        .json(&serde_json::json!({ "scenario_id": "bootstrap_minimal_viable_world" }))
        .send().await.unwrap();

    // Drain initial + running status
    for _ in 0..3 {
        let _ = tokio::time::timeout(std::time::Duration::from_millis(500), ws.next())
            .await.ok();
    }

    client.post(format!("{}/run/stop", base)).send().await.unwrap();

    let json = tokio::time::timeout(std::time::Duration::from_millis(500), async {
        loop {
            let msg = ws.next().await.unwrap().unwrap();
            if let Message::Text(t) = msg {
                let j: serde_json::Value = serde_json::from_str(&t).unwrap_or_default();
                if j["type"] == "status" && j["active_run_state"] == "completed" { return j; }
            }
        }
    }).await.expect("timed out waiting for idle status");

    assert_eq!(json["active_run_state"].as_str().unwrap(), "completed");
}
```

- [ ] **Step 2: Run to verify failure**

```bash
cargo test --test runner_ws_stream
```

Expected: compile error — `/stream` route doesn't exist.

- [ ] **Step 3: Create `src/viewer_server/api/stream.rs`**

```rust
//! WebSocket /stream handler.
//!
//! On connect:
//!   1. Subscribe to Broadcaster.
//!   2. Send current JSON status text.
//!   3. Pump loop via tokio::select!:
//!      - From broadcast: send Binary (frame) or Text (status) to WS client.
//!      - From WS client: ignore (WS is push-only; commands go via HTTP).
//!      - Lagged: skip missed frames, log, continue.
//!      - Disconnect or channel closed: exit.

use axum::{
    Router,
    extract::{State, ws::{Message, WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
    routing::get,
};
use tokio::sync::broadcast::error::RecvError;

use crate::runner::lifecycle::ActiveRunState;
use crate::runner::projections::RunStatusProjection;
use crate::viewer_server::broadcaster::WsMessage;
use crate::viewer_server::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/stream", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws_client(socket, state))
}

async fn handle_ws_client(mut socket: WebSocket, state: AppState) {
    // Subscribe to broadcast channel
    let mut rx = state.lock().unwrap().broadcaster.subscribe();

    // Send initial status
    let status = {
        let locked = state.lock().unwrap();
        // Build the same public status shape used by HTTP /run/status.
        serde_json::json!({
            "type": "status",
            "process_state": format!("{:?}", locked.process_state).to_lowercase(),
            "active_run_state": active_state_label(locked.active_run_state),
            "run_id": locked.run_id,
            "committed_tick": locked.committed_tick,
            "scenario_id": locked.scenario_id,
            "scenario_hash": locked.scenario_hash,
            "effective_seed": locked.effective_seed,
            "terminal_reason": locked.terminal_reason
        }).to_string()
    };
    if socket.send(Message::Text(status.into())).await.is_err() {
        return;
    }

    // Pump loop — WS is push-only
    loop {
        tokio::select! {
            result = rx.recv() => {
                match result {
                    Ok(WsMessage::Frame(bytes)) => {
                        if socket.send(Message::Binary(bytes.into())).await.is_err() {
                            break;
                        }
                    }
                    Ok(WsMessage::Status(text)) => {
                        if socket.send(Message::Text(text.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(RecvError::Lagged(n)) => {
                        eprintln!("[ws] Client lagged by {} frames — skipping", n);
                        continue;
                    }
                    Err(RecvError::Closed) => break,
                }
            }
            incoming = socket.recv() => {
                match incoming {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(_)) => break,
                    _ => {} // ignore text/binary from client; commands go via HTTP
                }
            }
        }
    }
}

fn active_state_label(state: ActiveRunState) -> &'static str {
    match state {
        ActiveRunState::Idle => "idle",
        ActiveRunState::Preparing => "preparing",
        ActiveRunState::Running => "running",
        ActiveRunState::Paused => "paused",
        ActiveRunState::Stopping => "stopping",
        ActiveRunState::Completed => "completed",
        ActiveRunState::Failed => "failed",
    }
}
```

- [ ] **Step 4: Register /stream у `src/viewer_server/api/mod.rs`**

```rust
pub mod info;
pub mod run;
pub mod scenarios;
pub mod stream;

use axum::Router;
use crate::viewer_server::state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(info::router())
        .merge(scenarios::router(state.clone()))
        .merge(run::router(state.clone()))
        .merge(stream::router(state))
}
```

- [ ] **Step 5: Run Task 4 tests (перші 2)**

```bash
cargo test --test runner_ws_stream ws_connect_receives_initial_status_idle
cargo test --test runner_ws_stream two_ws_clients_both_receive_initial_status
```

Expected: `PASS`.

- [ ] **Step 6: Run Task 5 tests (binary frames)**

```bash
cargo test --test runner_ws_stream ws_receives_binary_alif_frame_after_start
cargo test --test runner_ws_stream slow_ws_client_does_not_block_simulation
```

Expected: `PASS`. Якщо slow client test флакить — збільш sleep до 500ms.

- [ ] **Step 7: Commit**

```bash
git add src/viewer_server/api/stream.rs src/viewer_server/api/mod.rs
git commit -m "feat(viewer-server): add push-only WS /stream with initial status and pump loop"
```

---

## Task 5: Status broadcasts on state changes

**Files:**
- Modify: `src/viewer_server/api/run.rs`

- [ ] **Step 1: Run Task 6 tests to verify they fail**

```bash
cargo test --test runner_ws_stream ws_receives_running_status_after_http_start
cargo test --test runner_ws_stream ws_receives_idle_status_after_http_stop
```

Expected: `FAIL` — status not broadcasted yet.

- [ ] **Step 2: Add status broadcast після кожної зміни стану у `run.rs`**

Створи helper функцію і виклич після кожного endpoint:

```rust
/// Broadcast a RunStatusProjection JSON to all WS subscribers.
/// Call after any canonical lifecycle change. Lock must NOT be held when calling.
fn broadcast_status(state: &AppState) {
    let locked = state.lock().unwrap();
    let msg = serde_json::json!({
        "type": "status",
        "process_state": format!("{:?}", locked.process_state).to_lowercase(),
        "active_run_state": active_state_label(locked.active_run_state),
        "run_id": locked.run_id,
        "committed_tick": locked.committed_tick,
        "scenario_id": locked.scenario_id,
        "scenario_hash": locked.scenario_hash,
        "effective_seed": locked.effective_seed,
        "terminal_reason": locked.terminal_reason
    }).to_string();
    locked.broadcaster.send_status(msg);
}
```

Додати `broadcast_status(&state);` після:
- `spawn_tick_loop(state.clone())` у `handle_run_start`
- `PauseRun` command success у `handle_run_pause`
- `ResumeRun` command success у `handle_run_resume`
- `locked.engine = None` у `handle_run_stop`

- [ ] **Step 3: Run status tests**

```bash
cargo test --test runner_ws_stream ws_receives_running_status_after_http_start
cargo test --test runner_ws_stream ws_receives_idle_status_after_http_stop
```

Expected: обидва `PASS`.

- [ ] **Step 4: Run full suite**

```bash
cargo test --workspace
```

Expected: всі тести `PASS`.

- [ ] **Step 5: Commit**

```bash
git add src/viewer_server/api/run.rs
git commit -m "feat(viewer-server): broadcast JSON status to WS on run state changes"
```

---

## Task 6: Smoke test — serve mode з WS

- [ ] **Step 1: Build binary**

```bash
cargo build --bin runner
```

- [ ] **Step 2: Smoke test (якщо є wscat)**

```bash
# Terminal 1:
cargo run --bin runner -- --serve

# Terminal 2:
wscat -c ws://127.0.0.1:8080/stream
# → {"type":"status","active_run_state":"idle","committed_tick":0,...}

# Terminal 3:
curl -s -X POST http://127.0.0.1:8080/run/start \
     -H "Content-Type: application/json" \
     -d '{"scenario_id":"bootstrap_minimal_viable_world"}'
# → Terminal 2 отримує: {"type":"status","active_run_state":"running","committed_tick":0,...}
# → Terminal 2 отримує: binary frames (ALIF, ≤30 per sec)

curl -s -X POST http://127.0.0.1:8080/run/pause
# → Terminal 2: {"type":"status","active_run_state":"paused",...}

curl -s -X POST http://127.0.0.1:8080/run/stop
# → Terminal 2: {"type":"status","active_run_state":"completed",...}
```

- [ ] **Step 3: Final workspace check**

```bash
cargo test --workspace
```

Expected: всі тести `PASS`.

- [ ] **Step 4: Commit**

```bash
git add .
git commit -m "test(runner-3): smoke verify --serve WS push-only stream end-to-end"
```

---

## Self-Review

### Spec coverage

| Вимога | Реалізована? |
|---|---|
| WS `/stream` endpoint | ✅ Task 4 |
| ALIF v1 binary frame encoder | ✅ Task 1 |
| Time-based broadcast ≤30fps | ✅ Task 3 (spawn_tick_loop) |
| Незалежний підписник per connection | ✅ Task 4 (broadcast::Receiver) |
| Slow client не блокує Core | ✅ Task 4 (Lagged) + test |
| Status JSON при зміні стану | ✅ Task 5 |
| Initial status при підключенні | ✅ Task 4 |
| Scroll-back на сервері | ❌ Свідомо прибрано — client-side |
| Seek по WS | ❌ Свідомо прибрано — client-side |
| Ring buffer у SharedState | ❌ Свідомо прибрано |
| snap.clone() у tick loop | ❌ Свідомо прибрано — encode пряму в buf |

### Type consistency

- `WsMessage::Frame(Vec<u8>)` і `WsMessage::Status(String)` — однаково скрізь
- `Broadcaster::subscribe()` → `broadcast::Receiver<WsMessage>` — однаково у state.rs і stream.rs
- `encode_world_frame(&WorldFrameProjection) -> Vec<u8>` — однаково у frame_encoder.rs і state.rs
- `new_app_state(path, size, fps)` — 3 аргументи скрізь
- `target_broadcast_fps: u32` — замінює `stream_frame_interval` у всіх файлах
- `WorldFrameProjection v1` — public WS payload source; `CommittedSnapshot` is internal input only
- `RunStatusProjection` — public status text payload source

---

## Acceptance Gate

```
cargo test --test runner_frame_encoder     → 11 PASS
cargo test -p alife viewer_server::broadcaster → 5 PASS
cargo test --test runner_ws_stream         → 6 PASS
cargo test --workspace                     → без регресій

cargo run --bin runner -- --serve
wscat -c ws://127.0.0.1:8080/stream
  → {"type":"status","active_run_state":"idle","committed_tick":0,...}

POST /run/start bootstrap_minimal_viable_world
  → WS: {"type":"status","active_run_state":"running",...}
  → WS: <ALIF binary frames at ≤30fps>

POST /run/pause
  → WS: {"type":"status","active_run_state":"paused",...}

POST /run/resume
  → WS: {"type":"status","active_run_state":"running",...}

POST /run/stop
  → WS: {"type":"status","active_run_state":"completed",...}

два WS клієнти → незалежні stream-и
повільний клієнт → Lagged помилка → skip → Core не стоїть
новий клієнт → initial status (без будь-яких seek запитів)
```
