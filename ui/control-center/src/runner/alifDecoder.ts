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
  const previousCommittedTick = previousRaw === U64_MAX ? null : readU64AsNumber(view, 30);
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
