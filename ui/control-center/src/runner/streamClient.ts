import { decodeAlifFrame, type LiveWorldFrameProjection } from './alifDecoder';
import { mapStatus, type RunStatus } from './apiClient';

export type RunnerStreamConnectionState = 'connecting' | 'connected' | 'disconnected';

export interface RunnerStreamHandlers {
  onConnectionState: (state: RunnerStreamConnectionState) => void;
  onStatus: (status: RunStatus) => void;
  onFrame: (frame: LiveWorldFrameProjection) => void;
  onError: (error: Error) => void;
}

type StatusWire = Parameters<typeof mapStatus>[0];

export function toStreamUrl(baseUrl: string): string {
  const url = new URL('/stream', baseUrl);
  if (url.protocol === 'http:') {
    url.protocol = 'ws:';
  } else if (url.protocol === 'https:') {
    url.protocol = 'wss:';
  }
  return url.toString();
}

export class RunnerStreamClient {
  private socket: WebSocket | null = null;

  constructor(
    private readonly baseUrl: string,
    private readonly handlers: RunnerStreamHandlers
  ) {}

  connect(): void {
    this.disconnect();

    this.handlers.onConnectionState('connecting');

    const socket = new WebSocket(toStreamUrl(this.baseUrl));
    socket.binaryType = 'arraybuffer';
    socket.onopen = () => {
      this.handlers.onConnectionState('connected');
    };
    socket.onmessage = (event) => {
      this.handleMessage(event.data);
    };
    socket.onerror = () => {
      this.handlers.onError(new Error('Runner stream socket error'));
    };
    socket.onclose = () => {
      if (this.socket === socket) {
        this.socket = null;
        this.handlers.onConnectionState('disconnected');
      }
    };

    this.socket = socket;
  }

  disconnect(): void {
    const socket = this.socket;
    if (socket === null) {
      return;
    }

    this.socket = null;
    socket.onclose = null;
    socket.close();
    this.handlers.onConnectionState('disconnected');
  }

  private handleMessage(data: unknown): void {
    try {
      if (typeof data === 'string') {
        this.handlers.onStatus(mapStatus(JSON.parse(data) as StatusWire));
        return;
      }

      if (data instanceof ArrayBuffer) {
        this.handlers.onFrame(decodeAlifFrame(data));
      }
    } catch (error) {
      this.handlers.onError(toError(error));
    }
  }
}

function toError(error: unknown): Error {
  return error instanceof Error ? error : new Error(String(error));
}
