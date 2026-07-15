---
tags:
  - alife
  - worklog/plan
  - ui
  - runner
---

# UI-1B Live Runner Transport And Run Controls Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Connect `ui/control-center` to the live Runner service so Monitor can list scenarios, start a real run, receive `ALIF v2` WebSocket frames, and make `Play`, `Pause`, `Step N`, and `Stop` controls reflect actual Runner state.

**Architecture:** Keep the UI shell and PixiJS renderer from `UI-1A`. Add a transport boundary under `src/runner/` for HTTP commands, WebSocket stream handling, and `ALIF v2` decoding. Convert Runner projections into the existing `WorldFrame` UI model through a live adapter while retaining the deterministic fixture as the offline fallback.

**Tech Stack:** React 19, Vite, TypeScript, Zustand vanilla store, PixiJS, Vitest/RTL, Playwright, native `fetch`, native `WebSocket`.

---

## Scope

Build only `UI-1B`.

In scope:

- Configurable local Runner endpoint, default `http://127.0.0.1:8080`.
- `/server/info` health check.
- `/scenarios` scenario list.
- `/run/status`, `/run/start`, `/run/pause`, `/run/resume`, `/run/step`, `/run/stop`.
- `/stream` WebSocket with initial JSON status text, later JSON status text, and binary `ALIF v2` frames.
- TypeScript `ALIF v2` decoder matching `src/viewer_server/frame_encoder.rs`.
- Live `WorldFrame` adapter feeding the existing `WorldViewer`.
- Run controls that are enabled/disabled from Runner state.
- Clear connection/status/error UI.
- Fixture fallback when Runner is disconnected.
- Unit tests, component tests, and live Playwright smoke against `cargo run --bin runner -- --serve`.

Out of scope:

- UI redesign or new visual language.
- WOW rendering, semantic zoom, high-detail Inspector.
- Remote/LAN viewer mode.
- Authentication/access tokens.
- Persisted run history.
- Scenario editing.
- Checkpoint/branch/intervention commands.
- JSON replacement for `ALIF v2`.
- Core or Runner behavior changes unless tests reveal a contract mismatch.

## Canon And Current Implementation Facts

Required source checks before implementation:

```powershell
Get-Content docs/runner/command-contract.md
Get-Content docs/runner/projections.md
Get-Content docs/implementation/implementation-plan-ui.md
Get-Content docs/RUNNER_USAGE.md
Get-Content src/viewer_server/frame_encoder.rs
Get-Content src/viewer_server/api/run.rs
Get-Content src/viewer_server/api/stream.rs
```

Known Runner contracts:

- HTTP base URL: `http://127.0.0.1:8080`.
- WebSocket stream URL: `ws://127.0.0.1:8080/stream`.
- Initial WebSocket message is text JSON status:

```json
{
  "type": "status",
  "process_state": "ready",
  "active_run_state": "idle",
  "run_id": null,
  "committed_tick": 0,
  "scenario_id": null,
  "scenario_hash": null,
  "effective_seed": null,
  "terminal_reason": null
}
```

- Frame messages are binary `ALIF v2`.
- `ALIF v2` layout from `src/viewer_server/frame_encoder.rs`:

```text
offset  size  field
0       4     ASCII magic "ALIF"
4       1     version = 2
5       1     reserved flags byte
6       8     committed_tick u64 little-endian
14      8     projection_sequence u64 little-endian
22      8     wall_clock_generated_at_ms u64 little-endian
30      8     previous_committed_tick u64 little-endian, u64::MAX means none
38      4     heat f32 little-endian
42      4     waste f32 little-endian
46      4     cell_count u32 little-endian
50      N     cells, 21 bytes each
```

Cell record:

```text
offset  size  field
0       4     id u32 little-endian
4       4     x f32 little-endian
8       4     y f32 little-endian
12      4     radius f32 little-endian
16      4     energy f32 little-endian
20      1     lifecycle u8
```

## File Structure

Create:

- `ui/control-center/src/runner/alifDecoder.ts` - decode binary `ALIF v2` frames into typed live projections.
- `ui/control-center/src/runner/alifDecoder.test.ts` - RED/GREEN tests for decoder layout, errors, and cell parsing.
- `ui/control-center/src/runner/apiClient.ts` - HTTP client for Runner info, scenarios, status, and commands.
- `ui/control-center/src/runner/apiClient.test.ts` - fetch-based tests with mocked `fetch`.
- `ui/control-center/src/runner/streamClient.ts` - WebSocket wrapper for status/frame/error/reconnect lifecycle.
- `ui/control-center/src/runner/streamClient.test.ts` - fake `WebSocket` tests for text status, binary frames, close, and parse errors.
- `ui/control-center/src/projection/liveAdapter.ts` - convert live decoded frames into `WorldFrame`.
- `ui/control-center/src/projection/liveAdapter.test.ts` - adapter tests for tick, run identity, world bounds, lifecycle/integrity defaults.
- `ui/control-center/src/components/ConnectionPanel.tsx` - small connection/scenario/status panel.
- `ui/control-center/src/components/ConnectionPanel.test.tsx` - UI tests for connected/disconnected/error/scenario states.
- `ui/control-center/src/components/RunControls.tsx` - state-driven Play/Pause/Step/Stop buttons.
- `ui/control-center/src/components/RunControls.test.tsx` - UI tests for enabled/disabled behavior and callbacks.
- `ui/control-center/playwright.live.config.ts` - live E2E config that starts Runner and UI dev server.
- `ui/control-center/tests/e2e/live-runner.spec.ts` - smoke test against real Runner service.

Modify:

- `ui/control-center/src/projection/types.ts` - extend `WorldFrame` to support live metadata while preserving fixture compatibility.
- `ui/control-center/src/app/appState.ts` - store connection, scenarios, run status, current frame source, pending command, and errors.
- `ui/control-center/src/app/appState.test.ts` - cover live frame updates and state transitions.
- `ui/control-center/src/components/AppShell.tsx` - wire transport lifecycle, panels, controls, and live frame updates.
- `ui/control-center/src/App.test.tsx` - mock live transport and test connected UI behavior.
- `ui/control-center/src/styles.css` - modest status/control styling only; no redesign.
- `ui/control-center/package.json` - add `e2e:live` script.
- `outputs/worklogs/index.md` - register final report after implementation.

## Task 0: Preflight And Current Contract Snapshot

**Files:**
- Read only.

- [ ] **Step 1: Verify working tree and current branch**

Run:

```powershell
git status --short --branch
```

Expected:

```text
## main...origin/main [ahead 1]
```

If there are uncommitted changes, inspect them before editing:

```powershell
git diff --stat
git diff -- .gitignore outputs/worklogs ui/control-center
```

- [ ] **Step 2: Verify current UI tests pass before changing behavior**

Run:

```powershell
cd ui/control-center
npm.cmd test
npm.cmd run build
```

Expected:

```text
Test Files 5 passed
Tests 10 passed
vite ... built
```

- [ ] **Step 3: Verify Runner service can start**

From repo root:

```powershell
cargo run --bin runner -- --serve
```

In another terminal:

```powershell
curl http://127.0.0.1:8080/server/info
curl http://127.0.0.1:8080/scenarios
curl http://127.0.0.1:8080/run/status
```

Expected `/server/info` body shape:

```json
{"engine_version":"0.1.0","api_version":"1","allow_remote_viewer":false}
```

Expected `/run/status` body shape:

```json
{
  "process_state": "ready",
  "active_run_state": "idle",
  "run_id": null,
  "committed_tick": 0,
  "scenario_id": null,
  "scenario_hash": null,
  "effective_seed": null,
  "terminal_reason": null
}
```

Stop the manual Runner after verification.

## Task 1: Decode `ALIF v2` Frames In TypeScript

**Files:**
- Create: `ui/control-center/src/runner/alifDecoder.ts`
- Test: `ui/control-center/src/runner/alifDecoder.test.ts`

- [ ] **Step 1: Write failing decoder tests**

Create `ui/control-center/src/runner/alifDecoder.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import { decodeAlifFrame } from './alifDecoder';

function writeU64(view: DataView, offset: number, value: bigint) {
  view.setBigUint64(offset, value, true);
}

function makeFrame(cellCount = 2) {
  const bytes = new Uint8Array(50 + cellCount * 21);
  const view = new DataView(bytes.buffer);
  bytes.set([0x41, 0x4c, 0x49, 0x46], 0);
  view.setUint8(4, 2);
  view.setUint8(5, 0);
  writeU64(view, 6, 42n);
  writeU64(view, 14, 7n);
  writeU64(view, 22, 123456n);
  writeU64(view, 30, 41n);
  view.setFloat32(38, 12.5, true);
  view.setFloat32(42, 3.25, true);
  view.setUint32(46, cellCount, true);

  if (cellCount > 0) {
    const first = 50;
    view.setUint32(first, 1001, true);
    view.setFloat32(first + 4, 10.5, true);
    view.setFloat32(first + 8, 20.25, true);
    view.setFloat32(first + 12, 4.5, true);
    view.setFloat32(first + 16, 0.75, true);
    view.setUint8(first + 20, 1);
  }

  if (cellCount > 1) {
    const second = 71;
    view.setUint32(second, 1002, true);
    view.setFloat32(second + 4, 30, true);
    view.setFloat32(second + 8, 40, true);
    view.setFloat32(second + 12, 6, true);
    view.setFloat32(second + 16, 0.25, true);
    view.setUint8(second + 20, 2);
  }

  return bytes.buffer;
}

describe('decodeAlifFrame', () => {
  it('decodes ALIF v2 frame metadata and cells', () => {
    const frame = decodeAlifFrame(makeFrame());

    expect(frame.schemaVersion).toBe('ALIF/v2');
    expect(frame.committedTick).toBe(42);
    expect(frame.projectionSequence).toBe(7);
    expect(frame.previousCommittedTick).toBe(41);
    expect(frame.heat).toBeCloseTo(12.5);
    expect(frame.waste).toBeCloseTo(3.25);
    expect(frame.cells).toEqual([
      { id: 1001, x: 10.5, y: 20.25, radius: 4.5, energy: 0.75, lifecycle: 1 },
      { id: 1002, x: 30, y: 40, radius: 6, energy: 0.25, lifecycle: 2 }
    ]);
  });

  it('maps u64 max previous tick to null', () => {
    const bytes = makeFrame(0);
    new DataView(bytes).setBigUint64(30, 18446744073709551615n, true);

    expect(decodeAlifFrame(bytes).previousCommittedTick).toBeNull();
  });

  it('rejects invalid magic, unsupported version, and truncated frames', () => {
    const invalidMagic = new Uint8Array(makeFrame());
    invalidMagic[0] = 0x00;
    expect(() => decodeAlifFrame(invalidMagic.buffer)).toThrow('Invalid ALIF magic');

    const invalidVersion = new Uint8Array(makeFrame());
    invalidVersion[4] = 3;
    expect(() => decodeAlifFrame(invalidVersion.buffer)).toThrow('Unsupported ALIF version: 3');

    expect(() => decodeAlifFrame(new ArrayBuffer(12))).toThrow('Frame too short');
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
npm.cmd test -- src/runner/alifDecoder.test.ts
```

Expected:

```text
FAIL src/runner/alifDecoder.test.ts
Cannot find module './alifDecoder'
```

- [ ] **Step 3: Implement minimal decoder**

Create `ui/control-center/src/runner/alifDecoder.ts`:

```ts
export interface LiveProjectedCell {
  id: number;
  x: number;
  y: number;
  radius: number;
  energy: number;
  lifecycle: number;
}

export interface LiveWorldFrameProjection {
  schemaVersion: 'ALIF/v2';
  committedTick: number;
  projectionSequence: number;
  wallClockGeneratedAtMs: number;
  previousCommittedTick: number | null;
  heat: number;
  waste: number;
  cells: LiveProjectedCell[];
}

const HEADER_SIZE = 50;
const CELL_SIZE = 21;
const U64_MAX = 18446744073709551615n;

export function decodeAlifFrame(input: ArrayBuffer | Uint8Array): LiveWorldFrameProjection {
  const bytes = input instanceof Uint8Array ? input : new Uint8Array(input);
  if (bytes.byteLength < HEADER_SIZE) {
    throw new Error(`Frame too short: ${bytes.byteLength} bytes, expected at least ${HEADER_SIZE}`);
  }
  if (bytes[0] !== 0x41 || bytes[1] !== 0x4c || bytes[2] !== 0x49 || bytes[3] !== 0x46) {
    throw new Error('Invalid ALIF magic');
  }
  const version = bytes[4];
  if (version !== 2) {
    throw new Error(`Unsupported ALIF version: ${version}`);
  }

  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const committedTick = readU64AsNumber(view, 6);
  const projectionSequence = readU64AsNumber(view, 14);
  const wallClockGeneratedAtMs = readU64AsNumber(view, 22);
  const previousRaw = view.getBigUint64(30, true);
  const previousCommittedTick = previousRaw === U64_MAX ? null : Number(previousRaw);
  const heat = view.getFloat32(38, true);
  const waste = view.getFloat32(42, true);
  const cellCount = view.getUint32(46, true);
  const expectedLength = HEADER_SIZE + cellCount * CELL_SIZE;

  if (bytes.byteLength < expectedLength) {
    throw new Error(`Frame truncated: ${bytes.byteLength} bytes, expected ${expectedLength} for ${cellCount} cells`);
  }

  const cells: LiveProjectedCell[] = [];
  for (let index = 0; index < cellCount; index += 1) {
    const offset = HEADER_SIZE + index * CELL_SIZE;
    cells.push({
      id: view.getUint32(offset, true),
      x: view.getFloat32(offset + 4, true),
      y: view.getFloat32(offset + 8, true),
      radius: view.getFloat32(offset + 12, true),
      energy: view.getFloat32(offset + 16, true),
      lifecycle: view.getUint8(offset + 20)
    });
  }

  return {
    schemaVersion: 'ALIF/v2',
    committedTick,
    projectionSequence,
    wallClockGeneratedAtMs,
    previousCommittedTick,
    heat,
    waste,
    cells
  };
}

function readU64AsNumber(view: DataView, offset: number): number {
  const value = view.getBigUint64(offset, true);
  const numeric = Number(value);
  if (!Number.isSafeInteger(numeric)) {
    throw new Error(`u64 value at offset ${offset} exceeds JavaScript safe integer range`);
  }
  return numeric;
}
```

- [ ] **Step 4: Run decoder tests**

Run:

```powershell
npm.cmd test -- src/runner/alifDecoder.test.ts
```

Expected:

```text
Test Files 1 passed
Tests 3 passed
```

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/runner/alifDecoder.ts ui/control-center/src/runner/alifDecoder.test.ts
git commit -m "feat(ui): decode ALIF runner frames"
```

## Task 2: Add Live Projection Adapter

**Files:**
- Modify: `ui/control-center/src/projection/types.ts`
- Create: `ui/control-center/src/projection/liveAdapter.ts`
- Test: `ui/control-center/src/projection/liveAdapter.test.ts`

- [ ] **Step 1: Write failing adapter test**

Create `ui/control-center/src/projection/liveAdapter.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import type { LiveWorldFrameProjection } from '../runner/alifDecoder';
import { liveProjectionToWorldFrame } from './liveAdapter';

const liveFrame: LiveWorldFrameProjection = {
  schemaVersion: 'ALIF/v2',
  committedTick: 12,
  projectionSequence: 3,
  wallClockGeneratedAtMs: 1000,
  previousCommittedTick: 11,
  heat: 2.5,
  waste: 1.25,
  cells: [
    { id: 7, x: 10, y: 20, radius: 4, energy: 0.8, lifecycle: 1 },
    { id: 8, x: 80, y: 40, radius: 6, energy: 0.2, lifecycle: 2 }
  ]
};

describe('liveProjectionToWorldFrame', () => {
  it('maps ALIF live frame into the existing WorldFrame UI model', () => {
    const frame = liveProjectionToWorldFrame(liveFrame, {
      runId: 'run-live',
      scenarioName: 'demo_living_world'
    });

    expect(frame.schemaVersion).toBe('WorldFrameProjection/v1');
    expect(frame.source).toBe('live');
    expect(frame.runId).toBe('run-live');
    expect(frame.tick).toBe(12);
    expect(frame.summary?.heat).toBeCloseTo(2.5);
    expect(frame.summary?.waste).toBeCloseTo(1.25);
    expect(frame.cells).toEqual([
      expect.objectContaining({ id: '7', x: 10, y: 20, radius: 4, energy: 0.8 }),
      expect.objectContaining({ id: '8', x: 80, y: 40, radius: 6, energy: 0.2 })
    ]);
  });

  it('keeps a stable minimum world size when frame has few cells', () => {
    const frame = liveProjectionToWorldFrame({ ...liveFrame, cells: [] }, {
      runId: 'empty',
      scenarioName: 'empty'
    });

    expect(frame.world.width).toBeGreaterThanOrEqual(1200);
    expect(frame.world.height).toBeGreaterThanOrEqual(800);
    expect(frame.cells).toEqual([]);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
npm.cmd test -- src/projection/liveAdapter.test.ts
```

Expected:

```text
FAIL src/projection/liveAdapter.test.ts
Cannot find module './liveAdapter'
```

- [ ] **Step 3: Extend `WorldFrame` type**

Modify `ui/control-center/src/projection/types.ts`:

```ts
export type CellId = string;

export type FrameSource = 'fixture' | 'live';

export interface ResourceConcentration {
  organic: number;
  mineral: number;
  energy: number;
}

export interface CellProjection {
  id: CellId;
  x: number;
  y: number;
  radius: number;
  energy: number;
  integrity: number;
  generation: number;
  roleHint: string;
  lifecycle?: number;
}

export interface WorldFrame {
  schemaVersion: 'WorldFrameProjection/v1';
  source?: FrameSource;
  runId: string;
  scenarioName?: string;
  tick: number;
  world: {
    width: number;
    height: number;
  };
  resources: ResourceConcentration[][];
  cells: CellProjection[];
  summary?: {
    heat: number;
    waste: number;
    projectionSequence?: number;
    previousTick?: number | null;
    generatedAtMs?: number;
  };
}

export interface UiFixture {
  version: 'ui-1a-fixture/v1';
  scenarioName: string;
  frame: WorldFrame;
}
```

- [ ] **Step 4: Implement live adapter**

Create `ui/control-center/src/projection/liveAdapter.ts`:

```ts
import type { LiveWorldFrameProjection } from '../runner/alifDecoder';
import type { WorldFrame } from './types';

interface LiveFrameContext {
  runId: string;
  scenarioName: string;
}

export function liveProjectionToWorldFrame(
  projection: LiveWorldFrameProjection,
  context: LiveFrameContext
): WorldFrame {
  const maxX = Math.max(1200, ...projection.cells.map((cell) => cell.x + cell.radius * 2));
  const maxY = Math.max(800, ...projection.cells.map((cell) => cell.y + cell.radius * 2));

  return {
    schemaVersion: 'WorldFrameProjection/v1',
    source: 'live',
    runId: context.runId,
    scenarioName: context.scenarioName,
    tick: projection.committedTick,
    world: {
      width: Math.ceil(maxX),
      height: Math.ceil(maxY)
    },
    resources: [],
    cells: projection.cells.map((cell) => ({
      id: String(cell.id),
      x: cell.x,
      y: cell.y,
      radius: Math.max(2, cell.radius),
      energy: clamp01(cell.energy),
      integrity: lifecycleToIntegrity(cell.lifecycle),
      generation: 0,
      roleHint: lifecycleLabel(cell.lifecycle),
      lifecycle: cell.lifecycle
    })),
    summary: {
      heat: projection.heat,
      waste: projection.waste,
      projectionSequence: projection.projectionSequence,
      previousTick: projection.previousCommittedTick,
      generatedAtMs: projection.wallClockGeneratedAtMs
    }
  };
}

function clamp01(value: number) {
  return Math.max(0, Math.min(1, value));
}

function lifecycleToIntegrity(lifecycle: number) {
  return lifecycle === 2 ? 0 : 1;
}

function lifecycleLabel(lifecycle: number) {
  if (lifecycle === 2) {
    return 'dead lifecycle state';
  }
  if (lifecycle === 1) {
    return 'alive lifecycle state';
  }
  return `lifecycle ${lifecycle}`;
}
```

- [ ] **Step 5: Run adapter tests and existing fixture tests**

Run:

```powershell
npm.cmd test -- src/projection/liveAdapter.test.ts src/fixtures/ui1aFixture.test.ts src/projection/fixtureAdapter.test.ts
```

Expected:

```text
Test Files 3 passed
```

- [ ] **Step 6: Commit**

```powershell
git add ui/control-center/src/projection/types.ts ui/control-center/src/projection/liveAdapter.ts ui/control-center/src/projection/liveAdapter.test.ts
git commit -m "feat(ui): adapt live runner frames"
```

## Task 3: Add Runner HTTP API Client

**Files:**
- Create: `ui/control-center/src/runner/apiClient.ts`
- Test: `ui/control-center/src/runner/apiClient.test.ts`

- [ ] **Step 1: Write failing HTTP client tests**

Create `ui/control-center/src/runner/apiClient.test.ts`:

```ts
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { RunnerApiClient } from './apiClient';

const fetchMock = vi.fn();

describe('RunnerApiClient', () => {
  beforeEach(() => {
    fetchMock.mockReset();
    vi.stubGlobal('fetch', fetchMock);
  });

  it('loads server info, scenarios, and run status', async () => {
    fetchMock
      .mockResolvedValueOnce(jsonResponse({ engine_version: '0.1.0', api_version: '1', allow_remote_viewer: false }))
      .mockResolvedValueOnce(jsonResponse([{ id: 'demo_living_world', path: 'demo/demo_living_world.toml' }]))
      .mockResolvedValueOnce(jsonResponse({
        process_state: 'ready',
        active_run_state: 'idle',
        run_id: null,
        committed_tick: 0,
        scenario_id: null,
        scenario_hash: null,
        effective_seed: null,
        terminal_reason: null
      }));

    const client = new RunnerApiClient('http://127.0.0.1:8080');

    await expect(client.getServerInfo()).resolves.toEqual({
      engineVersion: '0.1.0',
      apiVersion: '1',
      allowRemoteViewer: false
    });
    await expect(client.listScenarios()).resolves.toEqual([
      { id: 'demo_living_world', path: 'demo/demo_living_world.toml' }
    ]);
    await expect(client.getRunStatus()).resolves.toMatchObject({
      processState: 'ready',
      activeRunState: 'idle',
      committedTick: 0
    });
  });

  it('starts, pauses, resumes, steps, and stops runs through HTTP commands', async () => {
    fetchMock
      .mockResolvedValueOnce(jsonResponse({
        ok: true,
        run_id: 'request-1',
        scenario_hash: 'hash-1',
        effective_seed: 123,
        active_run_state: 'running',
        bootstrap_manifest: { source: 'prepared_world' }
      }))
      .mockResolvedValueOnce(jsonResponse({ ok: true, active_run_state: 'paused', committed_tick: 3 }))
      .mockResolvedValueOnce(jsonResponse({ ok: true, active_run_state: 'running', committed_tick: 3 }))
      .mockResolvedValueOnce(jsonResponse({ ok: true, active_run_state: 'paused', committed_tick: 4 }))
      .mockResolvedValueOnce(jsonResponse({ ok: true, active_run_state: 'completed', committed_tick: 4 }));

    const client = new RunnerApiClient('http://127.0.0.1:8080');

    await expect(client.startRun({ scenarioId: 'demo_living_world', requestId: 'request-1' })).resolves.toMatchObject({
      ok: true,
      runId: 'request-1',
      activeRunState: 'running'
    });
    await expect(client.pauseRun()).resolves.toMatchObject({ activeRunState: 'paused', committedTick: 3 });
    await expect(client.resumeRun()).resolves.toMatchObject({ activeRunState: 'running', committedTick: 3 });
    await expect(client.stepRun()).resolves.toMatchObject({ activeRunState: 'paused', committedTick: 4 });
    await expect(client.stopRun()).resolves.toMatchObject({ activeRunState: 'completed', committedTick: 4 });

    expect(fetchMock).toHaveBeenNthCalledWith(1, 'http://127.0.0.1:8080/run/start', expect.objectContaining({
      method: 'POST',
      body: JSON.stringify({ scenario_id: 'demo_living_world', request_id: 'request-1' })
    }));
  });

  it('throws actionable errors for non-ok Runner responses', async () => {
    fetchMock.mockResolvedValueOnce(jsonResponse({
      ok: false,
      category: 'state_conflict',
      message: 'Run already active',
      current_state: 'see /run/status'
    }, 409));

    const client = new RunnerApiClient('http://127.0.0.1:8080');

    await expect(client.pauseRun()).rejects.toThrow('state_conflict: Run already active');
  });
});

function jsonResponse(body: unknown, status = 200) {
  return {
    ok: status >= 200 && status < 300,
    status,
    json: async () => body
  };
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
npm.cmd test -- src/runner/apiClient.test.ts
```

Expected:

```text
FAIL src/runner/apiClient.test.ts
Cannot find module './apiClient'
```

- [ ] **Step 3: Implement HTTP client**

Create `ui/control-center/src/runner/apiClient.ts`:

```ts
export interface ServerInfo {
  engineVersion: string;
  apiVersion: string;
  allowRemoteViewer: boolean;
}

export interface ScenarioListItem {
  id: string;
  path: string;
}

export type ActiveRunState = 'idle' | 'preparing' | 'running' | 'paused' | 'stopping' | 'completed' | 'failed';
export type ProcessState = 'starting' | 'ready' | 'shutting_down' | 'failed';

export interface RunStatus {
  processState: ProcessState;
  activeRunState: ActiveRunState;
  runId: string | null;
  committedTick: number;
  scenarioId: string | null;
  scenarioHash: string | null;
  effectiveSeed: number | null;
  terminalReason: string | null;
}

export interface StartRunInput {
  scenarioId: string;
  seedOverride?: number;
  requestId?: string;
}

export interface StartRunResponse {
  ok: true;
  runId: string;
  scenarioHash: string;
  effectiveSeed: number;
  activeRunState: ActiveRunState;
  bootstrapManifest: unknown;
}

export interface CommandResponse {
  ok: true;
  activeRunState: ActiveRunState;
  committedTick: number;
}

export class RunnerApiClient {
  constructor(private readonly baseUrl: string) {}

  async getServerInfo(): Promise<ServerInfo> {
    const raw = await this.getJson<{ engine_version: string; api_version: string; allow_remote_viewer: boolean }>('/server/info');
    return {
      engineVersion: raw.engine_version,
      apiVersion: raw.api_version,
      allowRemoteViewer: raw.allow_remote_viewer
    };
  }

  async listScenarios(): Promise<ScenarioListItem[]> {
    return this.getJson<ScenarioListItem[]>('/scenarios');
  }

  async getRunStatus(): Promise<RunStatus> {
    return mapStatus(await this.getJson<RunnerStatusResponse>('/run/status'));
  }

  async startRun(input: StartRunInput): Promise<StartRunResponse> {
    const body: Record<string, string | number> = { scenario_id: input.scenarioId };
    if (input.seedOverride !== undefined) {
      body.seed_override = input.seedOverride;
    }
    if (input.requestId) {
      body.request_id = input.requestId;
    }
    const raw = await this.postJson<StartRunRawResponse>('/run/start', body);
    return {
      ok: true,
      runId: raw.run_id,
      scenarioHash: raw.scenario_hash,
      effectiveSeed: raw.effective_seed,
      activeRunState: raw.active_run_state,
      bootstrapManifest: raw.bootstrap_manifest
    };
  }

  pauseRun() {
    return this.postCommand('/run/pause');
  }

  resumeRun() {
    return this.postCommand('/run/resume');
  }

  stepRun() {
    return this.postCommand('/run/step');
  }

  stopRun() {
    return this.postCommand('/run/stop');
  }

  private async postCommand(path: string): Promise<CommandResponse> {
    const raw = await this.postJson<CommandRawResponse>(path, {});
    return {
      ok: true,
      activeRunState: raw.active_run_state,
      committedTick: raw.committed_tick
    };
  }

  private async getJson<T>(path: string): Promise<T> {
    const response = await fetch(this.url(path));
    return parseResponse<T>(response);
  }

  private async postJson<T>(path: string, body: unknown): Promise<T> {
    const response = await fetch(this.url(path), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body)
    });
    return parseResponse<T>(response);
  }

  private url(path: string) {
    return `${this.baseUrl}${path}`;
  }
}

interface RunnerStatusResponse {
  process_state: ProcessState;
  active_run_state: ActiveRunState;
  run_id: string | null;
  committed_tick: number;
  scenario_id: string | null;
  scenario_hash: string | null;
  effective_seed: number | null;
  terminal_reason: string | null;
}

interface StartRunRawResponse {
  ok: true;
  run_id: string;
  scenario_hash: string;
  effective_seed: number;
  active_run_state: ActiveRunState;
  bootstrap_manifest: unknown;
}

interface CommandRawResponse {
  ok: true;
  active_run_state: ActiveRunState;
  committed_tick: number;
}

interface ErrorResponse {
  ok: false;
  category: string;
  message: string;
}

async function parseResponse<T>(response: Response): Promise<T> {
  const body = await response.json();
  if (!response.ok) {
    const error = body as ErrorResponse;
    throw new Error(`${error.category}: ${error.message}`);
  }
  return body as T;
}

export function mapStatus(raw: RunnerStatusResponse): RunStatus {
  return {
    processState: raw.process_state,
    activeRunState: raw.active_run_state,
    runId: raw.run_id,
    committedTick: raw.committed_tick,
    scenarioId: raw.scenario_id,
    scenarioHash: raw.scenario_hash,
    effectiveSeed: raw.effective_seed,
    terminalReason: raw.terminal_reason
  };
}
```

- [ ] **Step 4: Run HTTP client tests**

Run:

```powershell
npm.cmd test -- src/runner/apiClient.test.ts
```

Expected:

```text
Test Files 1 passed
Tests 3 passed
```

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/runner/apiClient.ts ui/control-center/src/runner/apiClient.test.ts
git commit -m "feat(ui): add runner HTTP client"
```

## Task 4: Add Runner WebSocket Stream Client

**Files:**
- Create: `ui/control-center/src/runner/streamClient.ts`
- Test: `ui/control-center/src/runner/streamClient.test.ts`

- [ ] **Step 1: Write failing stream client tests**

Create `ui/control-center/src/runner/streamClient.test.ts`:

```ts
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { RunnerStreamClient } from './streamClient';

const sockets: FakeWebSocket[] = [];

class FakeWebSocket {
  static OPEN = 1;
  static CLOSED = 3;
  binaryType: BinaryType = 'blob';
  readyState = FakeWebSocket.OPEN;
  onopen: ((event: Event) => void) | null = null;
  onmessage: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: Event) => void) | null = null;
  onclose: ((event: CloseEvent) => void) | null = null;
  close = vi.fn(() => {
    this.readyState = FakeWebSocket.CLOSED;
  });

  constructor(public readonly url: string) {
    sockets.push(this);
  }

  emitOpen() {
    this.onopen?.(new Event('open'));
  }

  emitText(data: string) {
    this.onmessage?.(new MessageEvent('message', { data }));
  }

  emitBinary(data: ArrayBuffer) {
    this.onmessage?.(new MessageEvent('message', { data }));
  }

  emitClose() {
    this.readyState = FakeWebSocket.CLOSED;
    this.onclose?.(new CloseEvent('close'));
  }
}

function makeMinimalAlifFrame() {
  const bytes = new Uint8Array(50);
  const view = new DataView(bytes.buffer);
  bytes.set([0x41, 0x4c, 0x49, 0x46], 0);
  view.setUint8(4, 2);
  view.setBigUint64(6, 1n, true);
  view.setBigUint64(14, 1n, true);
  view.setBigUint64(22, 100n, true);
  view.setBigUint64(30, 18446744073709551615n, true);
  view.setUint32(46, 0, true);
  return bytes.buffer;
}

describe('RunnerStreamClient', () => {
  beforeEach(() => {
    sockets.length = 0;
    vi.stubGlobal('WebSocket', FakeWebSocket);
  });

  it('connects to /stream and emits status and frame messages', () => {
    const onStatus = vi.fn();
    const onFrame = vi.fn();
    const onConnection = vi.fn();
    const client = new RunnerStreamClient('http://127.0.0.1:8080', { onStatus, onFrame, onConnection });

    client.connect();
    expect(sockets[0].url).toBe('ws://127.0.0.1:8080/stream');
    expect(sockets[0].binaryType).toBe('arraybuffer');

    sockets[0].emitOpen();
    sockets[0].emitText(JSON.stringify({ type: 'status', active_run_state: 'idle', committed_tick: 0 }));
    sockets[0].emitBinary(makeMinimalAlifFrame());

    expect(onConnection).toHaveBeenCalledWith('connected');
    expect(onStatus).toHaveBeenCalledWith(expect.objectContaining({ activeRunState: 'idle', committedTick: 0 }));
    expect(onFrame).toHaveBeenCalledWith(expect.objectContaining({ schemaVersion: 'ALIF/v2', committedTick: 1 }));
  });

  it('reports parse errors without throwing out of the message handler', () => {
    const onError = vi.fn();
    const client = new RunnerStreamClient('http://127.0.0.1:8080', { onError });

    client.connect();
    sockets[0].emitText('{not-json');
    sockets[0].emitBinary(new ArrayBuffer(4));

    expect(onError).toHaveBeenCalledWith(expect.stringContaining('Invalid stream status'));
    expect(onError).toHaveBeenCalledWith(expect.stringContaining('Frame too short'));
  });

  it('closes the current socket when disconnected manually', () => {
    const client = new RunnerStreamClient('http://127.0.0.1:8080', {});

    client.connect();
    client.disconnect();

    expect(sockets[0].close).toHaveBeenCalled();
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
npm.cmd test -- src/runner/streamClient.test.ts
```

Expected:

```text
FAIL src/runner/streamClient.test.ts
Cannot find module './streamClient'
```

- [ ] **Step 3: Implement stream client**

Create `ui/control-center/src/runner/streamClient.ts`:

```ts
import { decodeAlifFrame, type LiveWorldFrameProjection } from './alifDecoder';
import { mapStatus, type RunStatus } from './apiClient';

export type ConnectionState = 'disconnected' | 'connecting' | 'connected';

export interface RunnerStreamHandlers {
  onConnection?: (state: ConnectionState) => void;
  onStatus?: (status: RunStatus) => void;
  onFrame?: (frame: LiveWorldFrameProjection) => void;
  onError?: (message: string) => void;
}

export class RunnerStreamClient {
  private socket: WebSocket | null = null;

  constructor(
    private readonly baseUrl: string,
    private readonly handlers: RunnerStreamHandlers
  ) {}

  connect() {
    if (this.socket) {
      return;
    }
    this.handlers.onConnection?.('connecting');
    const socket = new WebSocket(toStreamUrl(this.baseUrl));
    socket.binaryType = 'arraybuffer';
    socket.onopen = () => this.handlers.onConnection?.('connected');
    socket.onclose = () => {
      this.socket = null;
      this.handlers.onConnection?.('disconnected');
    };
    socket.onerror = () => this.handlers.onError?.('Runner stream error');
    socket.onmessage = (event) => this.handleMessage(event.data);
    this.socket = socket;
  }

  disconnect() {
    this.socket?.close();
    this.socket = null;
    this.handlers.onConnection?.('disconnected');
  }

  private handleMessage(data: unknown) {
    if (typeof data === 'string') {
      try {
        this.handlers.onStatus?.(mapStatus(JSON.parse(data)));
      } catch (error) {
        this.handlers.onError?.(`Invalid stream status: ${errorMessage(error)}`);
      }
      return;
    }

    if (data instanceof ArrayBuffer) {
      try {
        this.handlers.onFrame?.(decodeAlifFrame(data));
      } catch (error) {
        this.handlers.onError?.(errorMessage(error));
      }
      return;
    }

    this.handlers.onError?.('Unsupported stream message type');
  }
}

export function toStreamUrl(baseUrl: string) {
  const url = new URL('/stream', baseUrl);
  url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
  return url.toString();
}

function errorMessage(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}
```

- [ ] **Step 4: Run stream client tests**

Run:

```powershell
npm.cmd test -- src/runner/streamClient.test.ts
```

Expected:

```text
Test Files 1 passed
Tests 3 passed
```

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/runner/streamClient.ts ui/control-center/src/runner/streamClient.test.ts
git commit -m "feat(ui): add runner stream client"
```

## Task 5: Expand App Store For Live Runner State

**Files:**
- Modify: `ui/control-center/src/app/appState.ts`
- Test: `ui/control-center/src/app/appState.test.ts`

- [ ] **Step 1: Add failing store tests**

Append to `ui/control-center/src/app/appState.test.ts`:

```ts
import type { RunStatus, ScenarioListItem, ServerInfo } from '../runner/apiClient';

const connectedInfo: ServerInfo = {
  engineVersion: '0.1.0',
  apiVersion: '1',
  allowRemoteViewer: false
};

const scenarios: ScenarioListItem[] = [
  { id: 'demo_living_world', path: 'demo/demo_living_world.toml' }
];

const runningStatus: RunStatus = {
  processState: 'ready',
  activeRunState: 'running',
  runId: 'run-1',
  committedTick: 5,
  scenarioId: 'demo_living_world',
  scenarioHash: 'hash-1',
  effectiveSeed: 100,
  terminalReason: null
};

describe('createAppStore live runner state', () => {
  it('tracks connection metadata, scenarios, status, and live frames', () => {
    const store = createAppStore();

    store.getState().setRunnerEndpoint('http://127.0.0.1:8080');
    store.getState().setConnected(connectedInfo);
    store.getState().setScenarios(scenarios);
    store.getState().setSelectedScenarioId('demo_living_world');
    store.getState().setRunStatus(runningStatus);
    store.getState().setFrame({
      schemaVersion: 'WorldFrameProjection/v1',
      source: 'live',
      runId: 'run-1',
      scenarioName: 'demo_living_world',
      tick: 5,
      world: { width: 1200, height: 800 },
      resources: [],
      cells: []
    });

    expect(store.getState().runnerEndpoint).toBe('http://127.0.0.1:8080');
    expect(store.getState().connectionState).toBe('connected');
    expect(store.getState().serverInfo?.apiVersion).toBe('1');
    expect(store.getState().selectedScenarioId).toBe('demo_living_world');
    expect(store.getState().runStatus?.activeRunState).toBe('running');
    expect(store.getState().frame.source).toBe('live');
  });

  it('clears pending command and stores error messages', () => {
    const store = createAppStore();

    store.getState().setPendingCommand('start');
    store.getState().setError('state_conflict: Cannot start run');
    store.getState().clearPendingCommand();

    expect(store.getState().pendingCommand).toBeNull();
    expect(store.getState().lastError).toBe('state_conflict: Cannot start run');
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
npm.cmd test -- src/app/appState.test.ts
```

Expected:

```text
FAIL src/app/appState.test.ts
setRunnerEndpoint is not a function
```

- [ ] **Step 3: Replace store with live-capable state**

Modify `ui/control-center/src/app/appState.ts`:

```ts
import { createStore } from 'zustand/vanilla';
import { ui1aFixture } from '../fixtures/ui1aFixture';
import { loadFixtureFrame, selectCell } from '../projection/fixtureAdapter';
import type { CellId, CellProjection, WorldFrame } from '../projection/types';
import type { ActiveRunState, RunStatus, ScenarioListItem, ServerInfo } from '../runner/apiClient';
import type { ConnectionState } from '../runner/streamClient';

export type ThemeMode = 'dark' | 'light';
export type PendingCommand = 'connect' | 'start' | 'pause' | 'resume' | 'step' | 'stop' | null;

export interface AppState {
  frame: WorldFrame;
  selectedCellId: CellId | null;
  selectedCell: CellProjection | null;
  theme: ThemeMode;
  runnerEndpoint: string;
  connectionState: ConnectionState;
  serverInfo: ServerInfo | null;
  scenarios: ScenarioListItem[];
  selectedScenarioId: string | null;
  runStatus: RunStatus | null;
  pendingCommand: PendingCommand;
  lastError: string | null;
}

export interface AppActions {
  selectCell: (cellId: CellId | null) => void;
  setTheme: (theme: ThemeMode) => void;
  setFrame: (frame: WorldFrame) => void;
  setRunnerEndpoint: (endpoint: string) => void;
  setConnectionState: (state: ConnectionState) => void;
  setConnected: (info: ServerInfo) => void;
  setScenarios: (scenarios: ScenarioListItem[]) => void;
  setSelectedScenarioId: (scenarioId: string | null) => void;
  setRunStatus: (status: RunStatus) => void;
  setPendingCommand: (command: Exclude<PendingCommand, null>) => void;
  clearPendingCommand: () => void;
  setError: (message: string | null) => void;
}

export type AppStore = AppState & AppActions;

export function createAppStore(initialFrame = loadFixtureFrame(ui1aFixture)) {
  return createStore<AppStore>((set, get) => ({
    frame: { ...initialFrame, source: initialFrame.source ?? 'fixture' },
    selectedCellId: initialFrame.cells[0]?.id ?? null,
    selectedCell: initialFrame.cells[0] ?? null,
    theme: 'dark',
    runnerEndpoint: 'http://127.0.0.1:8080',
    connectionState: 'disconnected',
    serverInfo: null,
    scenarios: [],
    selectedScenarioId: null,
    runStatus: null,
    pendingCommand: null,
    lastError: null,
    selectCell: (cellId) => {
      const selectedCell = selectCell(get().frame, cellId);
      set({
        selectedCellId: selectedCell?.id ?? null,
        selectedCell
      });
    },
    setTheme: (theme) => set({ theme }),
    setFrame: (frame) => {
      const selectedCell = selectCell(frame, get().selectedCellId) ?? frame.cells[0] ?? null;
      set({
        frame,
        selectedCellId: selectedCell?.id ?? null,
        selectedCell
      });
    },
    setRunnerEndpoint: (runnerEndpoint) => set({ runnerEndpoint }),
    setConnectionState: (connectionState) => set({ connectionState }),
    setConnected: (serverInfo) => set({ serverInfo, connectionState: 'connected', lastError: null }),
    setScenarios: (scenarios) => set({
      scenarios,
      selectedScenarioId: get().selectedScenarioId ?? scenarios[0]?.id ?? null
    }),
    setSelectedScenarioId: (selectedScenarioId) => set({ selectedScenarioId }),
    setRunStatus: (runStatus) => set({ runStatus }),
    setPendingCommand: (pendingCommand) => set({ pendingCommand, lastError: null }),
    clearPendingCommand: () => set({ pendingCommand: null }),
    setError: (lastError) => set({ lastError })
  }));
}

export function canStartRun(state: AppState) {
  return state.connectionState === 'connected'
    && state.selectedScenarioId !== null
    && state.pendingCommand === null
    && (!state.runStatus || isTerminalOrIdle(state.runStatus.activeRunState));
}

export function canPauseRun(state: AppState) {
  return state.pendingCommand === null && state.runStatus?.activeRunState === 'running';
}

export function canResumeRun(state: AppState) {
  return state.pendingCommand === null && state.runStatus?.activeRunState === 'paused';
}

export function canStepRun(state: AppState) {
  return state.pendingCommand === null && state.runStatus?.activeRunState === 'paused';
}

export function canStopRun(state: AppState) {
  return state.pendingCommand === null
    && Boolean(state.runStatus && ['running', 'paused'].includes(state.runStatus.activeRunState));
}

function isTerminalOrIdle(state: ActiveRunState) {
  return state === 'idle' || state === 'completed' || state === 'failed';
}
```

- [ ] **Step 4: Run store tests**

Run:

```powershell
npm.cmd test -- src/app/appState.test.ts
```

Expected:

```text
Test Files 1 passed
```

- [ ] **Step 5: Commit**

```powershell
git add ui/control-center/src/app/appState.ts ui/control-center/src/app/appState.test.ts
git commit -m "feat(ui): track live runner state"
```

## Task 6: Add Connection Panel And Run Controls Components

**Files:**
- Create: `ui/control-center/src/components/ConnectionPanel.tsx`
- Create: `ui/control-center/src/components/ConnectionPanel.test.tsx`
- Create: `ui/control-center/src/components/RunControls.tsx`
- Create: `ui/control-center/src/components/RunControls.test.tsx`
- Modify: `ui/control-center/src/styles.css`

- [ ] **Step 1: Write failing component tests**

Create `ui/control-center/src/components/RunControls.test.tsx`:

```tsx
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import type { AppState } from '../app/appState';
import { RunControls } from './RunControls';

function state(overrides: Partial<AppState>): AppState {
  return {
    frame: {
      schemaVersion: 'WorldFrameProjection/v1',
      source: 'fixture',
      runId: 'fixture',
      tick: 0,
      world: { width: 1200, height: 800 },
      resources: [],
      cells: []
    },
    selectedCellId: null,
    selectedCell: null,
    theme: 'dark',
    runnerEndpoint: 'http://127.0.0.1:8080',
    connectionState: 'connected',
    serverInfo: { engineVersion: '0.1.0', apiVersion: '1', allowRemoteViewer: false },
    scenarios: [{ id: 'demo_living_world', path: 'demo/demo_living_world.toml' }],
    selectedScenarioId: 'demo_living_world',
    runStatus: { processState: 'ready', activeRunState: 'idle', runId: null, committedTick: 0, scenarioId: null, scenarioHash: null, effectiveSeed: null, terminalReason: null },
    pendingCommand: null,
    lastError: null,
    ...overrides
  };
}

describe('RunControls', () => {
  it('starts from idle and pauses from running', async () => {
    const user = userEvent.setup();
    const onStart = vi.fn();
    const onPause = vi.fn();
    const { rerender } = render(<RunControls state={state({})} onStart={onStart} onPause={onPause} onResume={vi.fn()} onStep={vi.fn()} onStop={vi.fn()} />);

    await user.click(screen.getByRole('button', { name: 'Start live run' }));
    expect(onStart).toHaveBeenCalled();

    rerender(<RunControls state={state({ runStatus: { ...state({}).runStatus!, activeRunState: 'running' } })} onStart={onStart} onPause={onPause} onResume={vi.fn()} onStep={vi.fn()} onStop={vi.fn()} />);

    await user.click(screen.getByRole('button', { name: 'Pause live run' }));
    expect(onPause).toHaveBeenCalled();
  });

  it('enables resume and step only while paused', () => {
    render(<RunControls state={state({ runStatus: { ...state({}).runStatus!, activeRunState: 'paused' } })} onStart={vi.fn()} onPause={vi.fn()} onResume={vi.fn()} onStep={vi.fn()} onStop={vi.fn()} />);

    expect(screen.getByRole('button', { name: 'Resume live run' })).toBeEnabled();
    expect(screen.getByRole('button', { name: 'Step one committed tick' })).toBeEnabled();
    expect(screen.getByRole('button', { name: 'Pause live run' })).toBeDisabled();
  });
});
```

Create `ui/control-center/src/components/ConnectionPanel.test.tsx`:

```tsx
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { ConnectionPanel } from './ConnectionPanel';

describe('ConnectionPanel', () => {
  it('shows endpoint, connected server metadata, scenarios, and selected scenario', async () => {
    const user = userEvent.setup();
    const onScenarioChange = vi.fn();

    render(
      <ConnectionPanel
        endpoint="http://127.0.0.1:8080"
        connectionState="connected"
        serverInfo={{ engineVersion: '0.1.0', apiVersion: '1', allowRemoteViewer: false }}
        scenarios={[{ id: 'demo_living_world', path: 'demo/demo_living_world.toml' }]}
        selectedScenarioId="demo_living_world"
        lastError={null}
        onScenarioChange={onScenarioChange}
      />
    );

    expect(screen.getByText('Connected')).toBeInTheDocument();
    expect(screen.getByText('API v1')).toBeInTheDocument();
    await user.selectOptions(screen.getByLabelText('Scenario'), 'demo_living_world');
    expect(onScenarioChange).toHaveBeenCalledWith('demo_living_world');
  });

  it('shows disconnected and error states', () => {
    render(
      <ConnectionPanel
        endpoint="http://127.0.0.1:8080"
        connectionState="disconnected"
        serverInfo={null}
        scenarios={[]}
        selectedScenarioId={null}
        lastError="Runner unavailable"
        onScenarioChange={vi.fn()}
      />
    );

    expect(screen.getByText('Disconnected')).toBeInTheDocument();
    expect(screen.getByRole('alert')).toHaveTextContent('Runner unavailable');
  });
});
```

- [ ] **Step 2: Run tests to verify they fail**

Run:

```powershell
npm.cmd test -- src/components/RunControls.test.tsx src/components/ConnectionPanel.test.tsx
```

Expected:

```text
FAIL ... Cannot find module './RunControls'
FAIL ... Cannot find module './ConnectionPanel'
```

- [ ] **Step 3: Implement `RunControls`**

Create `ui/control-center/src/components/RunControls.tsx`:

```tsx
import { canPauseRun, canResumeRun, canStartRun, canStepRun, canStopRun, type AppState } from '../app/appState';

interface RunControlsProps {
  state: AppState;
  onStart: () => void;
  onPause: () => void;
  onResume: () => void;
  onStep: () => void;
  onStop: () => void;
}

export function RunControls({ state, onStart, onPause, onResume, onStep, onStop }: RunControlsProps) {
  return (
    <div className="run-controls" aria-label="Live run controls">
      <button className="icon-button primary" type="button" aria-label="Start live run" onClick={onStart} disabled={!canStartRun(state)}>
        Play
      </button>
      <button className="icon-button" type="button" aria-label="Pause live run" onClick={onPause} disabled={!canPauseRun(state)}>
        Pause
      </button>
      <button className="icon-button" type="button" aria-label="Resume live run" onClick={onResume} disabled={!canResumeRun(state)}>
        Resume
      </button>
      <button className="icon-button" type="button" aria-label="Step one committed tick" onClick={onStep} disabled={!canStepRun(state)}>
        Step N
      </button>
      <button className="icon-button danger" type="button" aria-label="Stop live run" onClick={onStop} disabled={!canStopRun(state)}>
        Stop
      </button>
    </div>
  );
}
```

- [ ] **Step 4: Implement `ConnectionPanel`**

Create `ui/control-center/src/components/ConnectionPanel.tsx`:

```tsx
import type { ScenarioListItem, ServerInfo } from '../runner/apiClient';
import type { ConnectionState } from '../runner/streamClient';

interface ConnectionPanelProps {
  endpoint: string;
  connectionState: ConnectionState;
  serverInfo: ServerInfo | null;
  scenarios: ScenarioListItem[];
  selectedScenarioId: string | null;
  lastError: string | null;
  onScenarioChange: (scenarioId: string) => void;
}

export function ConnectionPanel({
  endpoint,
  connectionState,
  serverInfo,
  scenarios,
  selectedScenarioId,
  lastError,
  onScenarioChange
}: ConnectionPanelProps) {
  return (
    <section className="connection-panel" aria-label="Runner connection">
      <div>
        <span className={`status-dot ${connectionState}`} />
        <strong>{connectionLabel(connectionState)}</strong>
      </div>
      <div className="metric-list compact">
        <div><span>Endpoint</span><strong>{endpoint}</strong></div>
        <div><span>Server</span><strong>{serverInfo ? `API v${serverInfo.apiVersion}` : 'not connected'}</strong></div>
      </div>
      <label className="scenario-select">
        <span>Scenario</span>
        <select
          aria-label="Scenario"
          value={selectedScenarioId ?? ''}
          disabled={scenarios.length === 0}
          onChange={(event) => onScenarioChange(event.target.value)}
        >
          {scenarios.length === 0 ? <option value="">No scenarios</option> : null}
          {scenarios.map((scenario) => (
            <option key={scenario.id} value={scenario.id}>{scenario.id}</option>
          ))}
        </select>
      </label>
      {lastError ? <p className="inline-error" role="alert">{lastError}</p> : null}
    </section>
  );
}

function connectionLabel(state: ConnectionState) {
  if (state === 'connected') {
    return 'Connected';
  }
  if (state === 'connecting') {
    return 'Connecting';
  }
  return 'Disconnected';
}
```

- [ ] **Step 5: Add minimal styles**

Append to `ui/control-center/src/styles.css`:

```css
.connection-panel {
  display: grid;
  gap: 12px;
  margin-bottom: 18px;
}

.connection-panel > div:first-child {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 9px;
  height: 9px;
  border-radius: 50%;
  background: #7f8b98;
}

.status-dot.connected {
  background: #5ee08d;
}

.status-dot.connecting {
  background: #ffd166;
}

.scenario-select {
  display: grid;
  gap: 6px;
}

.scenario-select select {
  min-height: 34px;
  border: 1px solid rgba(255, 255, 255, 0.14);
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.06);
  color: inherit;
  padding: 0 8px;
}

.metric-list.compact {
  margin-top: 0;
}

.inline-error {
  margin: 0;
  color: #ff9b9b;
  font-size: 13px;
}

.run-controls .danger {
  border-color: rgba(255, 120, 120, 0.4);
}
```

- [ ] **Step 6: Run component tests**

Run:

```powershell
npm.cmd test -- src/components/RunControls.test.tsx src/components/ConnectionPanel.test.tsx
```

Expected:

```text
Test Files 2 passed
Tests 4 passed
```

- [ ] **Step 7: Commit**

```powershell
git add ui/control-center/src/components/RunControls.tsx ui/control-center/src/components/RunControls.test.tsx ui/control-center/src/components/ConnectionPanel.tsx ui/control-center/src/components/ConnectionPanel.test.tsx ui/control-center/src/styles.css
git commit -m "feat(ui): add live runner controls"
```

## Task 7: Wire Live Runner Transport Into `AppShell`

**Files:**
- Modify: `ui/control-center/src/components/AppShell.tsx`
- Test: `ui/control-center/src/App.test.tsx`

- [ ] **Step 1: Replace App test mock with live transport expectations**

Modify `ui/control-center/src/App.test.tsx` to include these mocks and tests:

```tsx
import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { App } from './App';
import { renderApp } from './test/render';

const apiClient = {
  getServerInfo: vi.fn(),
  listScenarios: vi.fn(),
  getRunStatus: vi.fn(),
  startRun: vi.fn(),
  pauseRun: vi.fn(),
  resumeRun: vi.fn(),
  stepRun: vi.fn(),
  stopRun: vi.fn()
};

let streamHandlers: {
  onConnection?: (state: string) => void;
  onStatus?: (status: unknown) => void;
  onFrame?: (frame: unknown) => void;
  onError?: (message: string) => void;
} | null = null;

vi.mock('./runner/apiClient', async (importOriginal) => {
  const actual = await importOriginal<typeof import('./runner/apiClient')>();
  return {
    ...actual,
    RunnerApiClient: vi.fn(() => apiClient)
  };
});

vi.mock('./runner/streamClient', () => ({
  RunnerStreamClient: vi.fn((_endpoint: string, handlers: typeof streamHandlers) => {
    streamHandlers = handlers;
    return {
      connect: vi.fn(() => handlers.onConnection?.('connected')),
      disconnect: vi.fn()
    };
  })
}));

vi.mock('./viewer/worldRenderer', () => ({
  mountWorldRenderer: vi.fn(() => Promise.resolve({
    renderFrame: vi.fn(),
    resize: vi.fn(),
    exportPng: vi.fn(() => 'data:image/png;base64,fixture'),
    destroy: vi.fn()
  }))
}));

describe('App', () => {
  beforeEach(() => {
    apiClient.getServerInfo.mockResolvedValue({ engineVersion: '0.1.0', apiVersion: '1', allowRemoteViewer: false });
    apiClient.listScenarios.mockResolvedValue([{ id: 'demo_living_world', path: 'demo/demo_living_world.toml' }]);
    apiClient.getRunStatus.mockResolvedValue({
      processState: 'ready',
      activeRunState: 'idle',
      runId: null,
      committedTick: 0,
      scenarioId: null,
      scenarioHash: null,
      effectiveSeed: null,
      terminalReason: null
    });
    apiClient.startRun.mockResolvedValue({
      ok: true,
      runId: 'run-1',
      activeRunState: 'running',
      scenarioHash: 'hash-1',
      effectiveSeed: 100,
      bootstrapManifest: {}
    });
    apiClient.pauseRun.mockResolvedValue({ ok: true, activeRunState: 'paused', committedTick: 5 });
    streamHandlers = null;
  });

  it('connects to Runner, lists scenarios, and starts a live run', async () => {
    const user = userEvent.setup();
    renderApp(<App />);

    await waitFor(() => {
      expect(screen.getByText('Connected')).toBeInTheDocument();
      expect(screen.getByLabelText('Scenario')).toHaveValue('demo_living_world');
    });

    await user.click(screen.getByRole('button', { name: 'Start live run' }));

    expect(apiClient.startRun).toHaveBeenCalledWith(expect.objectContaining({
      scenarioId: 'demo_living_world'
    }));
  });

  it('updates the Monitor frame from stream frames', async () => {
    renderApp(<App />);

    await waitFor(() => expect(streamHandlers).not.toBeNull());
    streamHandlers?.onStatus?.({
      processState: 'ready',
      activeRunState: 'running',
      runId: 'run-1',
      committedTick: 9,
      scenarioId: 'demo_living_world',
      scenarioHash: 'hash',
      effectiveSeed: 1,
      terminalReason: null
    });
    streamHandlers?.onFrame?.({
      schemaVersion: 'ALIF/v2',
      committedTick: 9,
      projectionSequence: 1,
      wallClockGeneratedAtMs: 100,
      previousCommittedTick: 8,
      heat: 0,
      waste: 0,
      cells: [{ id: 44, x: 20, y: 30, radius: 5, energy: 0.5, lifecycle: 1 }]
    });

    await waitFor(() => expect(screen.getByText('Tick 9')).toBeInTheDocument());
    const inspector = screen.getByLabelText(/cell inspector/i);
    expect(within(inspector).getByText('44')).toBeInTheDocument();
  });
});
```

- [ ] **Step 2: Run tests to verify they fail**

Run:

```powershell
npm.cmd test -- src/App.test.tsx
```

Expected:

```text
FAIL src/App.test.tsx
Unable to find text Connected
```

- [ ] **Step 3: Wire `AppShell` to Runner clients**

Modify `ui/control-center/src/components/AppShell.tsx` to:

- import `RunnerApiClient`;
- import `RunnerStreamClient`;
- import `liveProjectionToWorldFrame`;
- import `ConnectionPanel`;
- import `RunControls`;
- connect on mount;
- load server info/scenarios/status;
- route WebSocket status and frame events to store;
- execute command handlers.

Use this structure for the key additions:

```tsx
import { RunnerApiClient } from '../runner/apiClient';
import { RunnerStreamClient } from '../runner/streamClient';
import { liveProjectionToWorldFrame } from '../projection/liveAdapter';
import { ConnectionPanel } from './ConnectionPanel';
import { RunControls } from './RunControls';
```

Inside `AppShell` after store/state creation:

```tsx
const apiRef = useRef<RunnerApiClient | null>(null);

useEffect(() => {
  const endpoint = store.getState().runnerEndpoint;
  const api = new RunnerApiClient(endpoint);
  apiRef.current = api;
  let disposed = false;

  const stream = new RunnerStreamClient(endpoint, {
    onConnection: (connectionState) => store.getState().setConnectionState(connectionState),
    onStatus: (status) => store.getState().setRunStatus(status),
    onFrame: (projection) => {
      const current = store.getState();
      const runId = current.runStatus?.runId ?? current.frame.runId;
      const scenarioName = current.runStatus?.scenarioId ?? current.selectedScenarioId ?? 'live';
      store.getState().setFrame(liveProjectionToWorldFrame(projection, { runId, scenarioName }));
    },
    onError: (message) => store.getState().setError(message)
  });

  async function connect() {
    store.getState().setPendingCommand('connect');
    try {
      const [serverInfo, scenarios, status] = await Promise.all([
        api.getServerInfo(),
        api.listScenarios(),
        api.getRunStatus()
      ]);
      if (disposed) {
        return;
      }
      store.getState().setConnected(serverInfo);
      store.getState().setScenarios(scenarios);
      store.getState().setRunStatus(status);
      stream.connect();
    } catch (error) {
      store.getState().setConnectionState('disconnected');
      store.getState().setError(error instanceof Error ? error.message : String(error));
    } finally {
      store.getState().clearPendingCommand();
    }
  }

  void connect();

  return () => {
    disposed = true;
    stream.disconnect();
  };
}, [store]);
```

Add command helper:

```tsx
const runCommand = async (
  command: Exclude<typeof state.pendingCommand, null>,
  action: () => Promise<unknown>
) => {
  store.getState().setPendingCommand(command);
  try {
    await action();
    const status = await apiRef.current?.getRunStatus();
    if (status) {
      store.getState().setRunStatus(status);
    }
  } catch (error) {
    store.getState().setError(error instanceof Error ? error.message : String(error));
  } finally {
    store.getState().clearPendingCommand();
  }
};
```

Replace current header run controls with:

```tsx
<RunControls
  state={state}
  onStart={() => runCommand('start', () => apiRef.current!.startRun({
    scenarioId: state.selectedScenarioId!,
    requestId: `ui-${Date.now()}`
  }))}
  onPause={() => runCommand('pause', () => apiRef.current!.pauseRun())}
  onResume={() => runCommand('resume', () => apiRef.current!.resumeRun())}
  onStep={() => runCommand('step', () => apiRef.current!.stepRun())}
  onStop={() => runCommand('stop', () => apiRef.current!.stopRun())}
/>
```

Add `ConnectionPanel` at the top of `LayerPanel` or pass its props into `LayerPanel`:

```tsx
<ConnectionPanel
  endpoint={state.runnerEndpoint}
  connectionState={state.connectionState}
  serverInfo={state.serverInfo}
  scenarios={state.scenarios}
  selectedScenarioId={state.selectedScenarioId}
  lastError={state.lastError}
  onScenarioChange={(scenarioId) => store.getState().setSelectedScenarioId(scenarioId)}
/>
```

Update the toolbar title:

```tsx
<strong>{state.frame.scenarioName ?? ui1aFixture.scenarioName}</strong>
<span>{state.frame.source === 'live' ? 'Live' : 'Fixture'} Tick {state.frame.tick}</span>
```

- [ ] **Step 4: Run App tests**

Run:

```powershell
npm.cmd test -- src/App.test.tsx
```

Expected:

```text
Test Files 1 passed
Tests 2 passed
```

- [ ] **Step 5: Run all unit tests**

Run:

```powershell
npm.cmd test
```

Expected:

```text
Test Files all passed
```

- [ ] **Step 6: Commit**

```powershell
git add ui/control-center/src/components/AppShell.tsx ui/control-center/src/App.test.tsx
git commit -m "feat(ui): connect monitor shell to runner"
```

## Task 8: Add Live Runner Playwright Smoke

**Files:**
- Modify: `ui/control-center/package.json`
- Create: `ui/control-center/playwright.live.config.ts`
- Create: `ui/control-center/tests/e2e/live-runner.spec.ts`

- [ ] **Step 1: Write failing live E2E spec**

Create `ui/control-center/tests/e2e/live-runner.spec.ts`:

```ts
import { expect, test } from '@playwright/test';

test('Control Center starts a live Runner run and receives frames', async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 800 });
  await page.goto('/');

  await expect(page.getByText('Connected')).toBeVisible({ timeout: 15000 });
  await expect(page.getByLabel('Scenario')).toHaveValue(/.+/);

  await page.getByRole('button', { name: 'Start live run' }).click();

  await expect(page.getByText(/Live Tick [1-9]/)).toBeVisible({ timeout: 15000 });
  await expect(page.getByLabel('Cell Inspector')).toContainText(/ID|No cell selected/);

  await page.getByRole('button', { name: 'Pause live run' }).click();
  await expect(page.getByRole('button', { name: 'Resume live run' })).toBeEnabled({ timeout: 10000 });

  await page.getByRole('button', { name: 'Step one committed tick' }).click();
  await expect(page.getByRole('button', { name: 'Resume live run' })).toBeEnabled({ timeout: 10000 });

  await page.getByRole('button', { name: 'Resume live run' }).click();
  await expect(page.getByRole('button', { name: 'Pause live run' })).toBeEnabled({ timeout: 10000 });

  await page.getByRole('button', { name: 'Stop live run' }).click();
});
```

- [ ] **Step 2: Add live Playwright config**

Create `ui/control-center/playwright.live.config.ts`:

```ts
import { defineConfig, devices } from '@playwright/test';
import { existsSync } from 'node:fs';

const browserExecutablePath = [
  process.env.E2E_BROWSER_EXECUTABLE,
  'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe',
  'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe'
].find((candidate): candidate is string => Boolean(candidate && existsSync(candidate)));

export default defineConfig({
  testDir: './tests/e2e',
  testMatch: ['live-runner.spec.ts'],
  fullyParallel: false,
  reporter: [['list']],
  use: {
    baseURL: 'http://127.0.0.1:5173',
    launchOptions: browserExecutablePath ? { executablePath: browserExecutablePath } : undefined,
    trace: 'on-first-retry'
  },
  webServer: [
    {
      command: 'cargo run --bin runner -- --serve',
      cwd: '../..',
      url: 'http://127.0.0.1:8080/server/info',
      reuseExistingServer: !process.env.CI,
      timeout: 120_000
    },
    {
      command: 'npm.cmd run dev -- --host 127.0.0.1 --port 5173',
      url: 'http://127.0.0.1:5173',
      reuseExistingServer: !process.env.CI,
      timeout: 120_000
    }
  ],
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] }
    }
  ]
});
```

- [ ] **Step 3: Add package script**

Modify `ui/control-center/package.json` scripts:

```json
{
  "scripts": {
    "dev": "vite",
    "build": "tsc -b && vite build",
    "test": "vitest run",
    "test:watch": "vitest",
    "e2e": "playwright test",
    "e2e:live": "playwright test --config playwright.live.config.ts"
  }
}
```

- [ ] **Step 4: Run live E2E to verify it fails before AppShell wiring if Task 7 is not complete**

If Task 7 is already complete, skip this RED verification and record that the test was added after live wiring. Otherwise run:

```powershell
npm.cmd run e2e:live
```

Expected before Task 7:

```text
FAIL ... unable to find Connected
```

- [ ] **Step 5: Run live E2E after Task 7**

Run:

```powershell
npm.cmd run e2e:live
```

Expected:

```text
1 passed
```

If this fails because Playwright bundled Chromium is missing, use installed Chrome fallback from the config. If both Chrome and Edge are missing, stop and ask the user to install one browser or allow `npx.cmd playwright install chromium`.

- [ ] **Step 6: Commit**

```powershell
git add ui/control-center/package.json ui/control-center/package-lock.json ui/control-center/playwright.live.config.ts ui/control-center/tests/e2e/live-runner.spec.ts
git commit -m "test(ui): cover live runner integration"
```

## Task 9: Final Verification And Report

**Files:**
- Create: `outputs/worklogs/2026-07-15-2330-REPORT-ui-1b-live-runner-transport.md`
- Modify: `outputs/worklogs/index.md`

- [ ] **Step 1: Run final verification**

From `ui/control-center`:

```powershell
npm.cmd test
npm.cmd run build
npm.cmd run e2e
npm.cmd run e2e:live
```

From repo root:

```powershell
cargo test runner_http_info runner_http_scenarios runner_ws_stream runner_frame_encoder
cargo fmt --check
git status --short
```

Expected:

```text
Vitest: all tests passed
Vite build: built
Playwright fixture E2E: 1 passed
Playwright live E2E: 1 passed
Cargo focused runner tests: passed
cargo fmt --check: exit 0
git status: only intentional report/index changes before report commit
```

- [ ] **Step 2: Create report**

Create `outputs/worklogs/2026-07-15-2330-REPORT-ui-1b-live-runner-transport.md`:

```markdown
---
tags:
  - alife
  - worklog/report
  - ui
  - runner
---

# UI-1B Live Runner Transport And Run Controls Report

## Summary

Implemented live Runner transport for `ALife Control Center`.

Completed:

- Added `ALIF v2` decoder.
- Added Runner HTTP client for server info, scenarios, status, and run commands.
- Added Runner WebSocket stream client.
- Adapted live frames into the existing World Viewer model.
- Added connection panel and state-driven run controls.
- Added live Playwright smoke against `cargo run --bin runner -- --serve`.

## Verification

```text
npm.cmd test
npm.cmd run build
npm.cmd run e2e
npm.cmd run e2e:live
cargo test runner_http_info runner_http_scenarios runner_ws_stream runner_frame_encoder
cargo fmt --check
```

## Deferred

- Remote viewer mode.
- Authentication.
- Design-system alignment.
- WOW rendering and semantic zoom.
- Scenario editing and intervention commands.
```

- [ ] **Step 3: Register report in worklog index**

Append under `## Reports` in `outputs/worklogs/index.md`:

```markdown
- [[outputs/worklogs/2026-07-15-2330-REPORT-ui-1b-live-runner-transport|2026-07-15-2330-REPORT-ui-1b-live-runner-transport]]
```

- [ ] **Step 4: Commit report**

```powershell
git add outputs/worklogs/index.md outputs/worklogs/2026-07-15-2330-REPORT-ui-1b-live-runner-transport.md
git commit -m "docs(ui): report UI-1B live runner transport"
```

## Success Criteria

`UI-1B` is successful only when:

- `cargo run --bin runner -- --serve` starts Runner at `http://127.0.0.1:8080`.
- Control Center connects to `/server/info`, `/scenarios`, `/run/status`, and `/stream`.
- Scenario selector shows real Runner scenarios.
- `Play` starts a real run via `/run/start`.
- Monitor tick changes from live `ALIF v2` frames without page refresh.
- `Pause`, `Resume`, `Step N`, and `Stop` call Runner commands and update enabled/disabled UI states.
- The Viewer renders live frame cells from Runner, not the static `tick: 128` fixture.
- Fixture fallback remains available when Runner is disconnected.
- `npm.cmd test`, `npm.cmd run build`, `npm.cmd run e2e`, `npm.cmd run e2e:live`, focused Runner cargo tests, and `cargo fmt --check` pass.

## Self-Review

- Spec coverage: plan covers HTTP commands, WebSocket stream, ALIF decoder, live adapter, app state, controls, connection UI, fixture fallback, and live E2E.
- Placeholder scan: no `TBD`, `TODO`, or unspecified implementation steps remain.
- Type consistency: `LiveWorldFrameProjection`, `RunStatus`, `ConnectionState`, `WorldFrame`, and command method names are defined before use.
- Scope check: plan does not include UI redesign, semantic zoom, remote mode, authentication, or Core mechanics changes.
