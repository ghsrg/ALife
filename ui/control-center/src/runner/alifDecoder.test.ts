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
