import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { RunnerStreamClient, toStreamUrl } from './streamClient';

function writeU64(view: DataView, offset: number, value: bigint) {
  view.setBigUint64(offset, value, true);
}

function makeFrame() {
  const bytes = new Uint8Array(50 + 21);
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
  view.setUint32(46, 1, true);

  const cell = 50;
  view.setUint32(cell, 1001, true);
  view.setFloat32(cell + 4, 10.5, true);
  view.setFloat32(cell + 8, 20.25, true);
  view.setFloat32(cell + 12, 4.5, true);
  view.setFloat32(cell + 16, 0.75, true);
  view.setUint8(cell + 20, 1);

  return bytes.buffer;
}

class FakeWebSocket {
  static instances: FakeWebSocket[] = [];

  binaryType: BinaryType = 'blob';
  onopen: (() => void) | null = null;
  onmessage: ((event: MessageEvent) => void) | null = null;
  onerror: (() => void) | null = null;
  onclose: (() => void) | null = null;
  close = vi.fn(() => {
    this.onclose?.();
  });

  constructor(readonly url: string) {
    FakeWebSocket.instances.push(this);
  }

  emitOpen() {
    this.onopen?.();
  }

  emitMessage(data: string | ArrayBuffer) {
    this.onmessage?.({ data } as MessageEvent);
  }

  emitError() {
    this.onerror?.();
  }
}

const statusWire = {
  process_state: 'ready',
  active_run_state: 'running',
  run_id: 'run-123',
  committed_tick: 42,
  scenario_id: 'single_cell_survival',
  scenario_hash: 'scenario-hash',
  effective_seed: 1234,
  terminal_reason: null
};

describe('RunnerStreamClient', () => {
  beforeEach(() => {
    FakeWebSocket.instances = [];
    vi.stubGlobal('WebSocket', FakeWebSocket);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('connects to /stream and emits status and frame messages', () => {
    const states: string[] = [];
    const onStatus = vi.fn();
    const onFrame = vi.fn();
    const client = new RunnerStreamClient('http://127.0.0.1:8080', {
      onConnectionState: (state) => states.push(state),
      onStatus,
      onFrame,
      onError: vi.fn()
    });

    expect(toStreamUrl('http://127.0.0.1:8080')).toBe('ws://127.0.0.1:8080/stream');

    client.connect();
    const socket = FakeWebSocket.instances[0];
    expect(socket.url).toBe('ws://127.0.0.1:8080/stream');
    expect(socket.binaryType).toBe('arraybuffer');
    expect(states).toEqual(['connecting']);

    socket.emitOpen();
    socket.emitMessage(JSON.stringify(statusWire));
    socket.emitMessage(makeFrame());

    expect(states).toEqual(['connecting', 'connected']);
    expect(onStatus).toHaveBeenCalledWith({
      processState: 'ready',
      activeRunState: 'running',
      runId: 'run-123',
      committedTick: 42,
      scenarioId: 'single_cell_survival',
      scenarioHash: 'scenario-hash',
      effectiveSeed: 1234,
      terminalReason: null
    });
    expect(onFrame).toHaveBeenCalledWith(
      expect.objectContaining({
        schemaVersion: 'ALIF/v2',
        committedTick: 42,
        cells: [{ id: 1001, x: 10.5, y: 20.25, radius: 4.5, energy: 0.75, lifecycle: 1 }]
      })
    );
  });

  it('reports parse and decode errors without throwing from the message handler', () => {
    const onError = vi.fn();
    const client = new RunnerStreamClient('http://127.0.0.1:8080', {
      onConnectionState: vi.fn(),
      onStatus: vi.fn(),
      onFrame: vi.fn(),
      onError
    });

    client.connect();
    const socket = FakeWebSocket.instances[0];

    expect(() => socket.emitMessage('{')).not.toThrow();
    expect(() => socket.emitMessage(new ArrayBuffer(1))).not.toThrow();

    expect(onError).toHaveBeenCalledTimes(2);
    expect(onError.mock.calls[0][0]).toBeInstanceOf(Error);
    expect(onError.mock.calls[1][0]).toBeInstanceOf(Error);
  });

  it('closes the current socket and reports disconnected when disconnected manually', () => {
    const states: string[] = [];
    const client = new RunnerStreamClient('http://127.0.0.1:8080', {
      onConnectionState: (state) => states.push(state),
      onStatus: vi.fn(),
      onFrame: vi.fn(),
      onError: vi.fn()
    });

    client.connect();
    const socket = FakeWebSocket.instances[0];
    socket.emitOpen();

    client.disconnect();

    expect(socket.close).toHaveBeenCalledTimes(1);
    expect(states).toEqual(['connecting', 'connected', 'disconnected']);
  });

  it('ignores stale socket events after disconnect and reconnect', () => {
    const states: string[] = [];
    const onStatus = vi.fn();
    const onFrame = vi.fn();
    const onError = vi.fn();
    const client = new RunnerStreamClient('http://127.0.0.1:8080', {
      onConnectionState: (state) => states.push(state),
      onStatus,
      onFrame,
      onError
    });

    client.connect();
    const firstSocket = FakeWebSocket.instances[0];
    firstSocket.emitOpen();

    client.disconnect();
    client.connect();
    const secondSocket = FakeWebSocket.instances[1];

    firstSocket.emitOpen();
    firstSocket.emitMessage(JSON.stringify(statusWire));
    firstSocket.emitMessage(makeFrame());
    firstSocket.emitError();

    expect(states).toEqual(['connecting', 'connected', 'disconnected', 'connecting']);
    expect(onStatus).not.toHaveBeenCalled();
    expect(onFrame).not.toHaveBeenCalled();
    expect(onError).not.toHaveBeenCalled();

    secondSocket.emitOpen();
    secondSocket.emitMessage(JSON.stringify(statusWire));
    secondSocket.emitMessage(makeFrame());

    expect(states).toEqual(['connecting', 'connected', 'disconnected', 'connecting', 'connected']);
    expect(onStatus).toHaveBeenCalledTimes(1);
    expect(onFrame).toHaveBeenCalledTimes(1);
    expect(onError).not.toHaveBeenCalled();
  });
});
