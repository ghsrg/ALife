export interface ServerInfo {
  engineVersion: string;
  apiVersion: string;
  allowRemoteViewer: boolean;
}

export interface ScenarioListItem {
  id: string;
  path: string;
}

export type ActiveRunState =
  | 'idle'
  | 'preparing'
  | 'running'
  | 'paused'
  | 'stopping'
  | 'completed'
  | 'failed';

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
  ticksPerSecond?: number;
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
  bootstrapManifest: unknown;
  effectiveSeed: number;
  activeRunState: ActiveRunState;
}

export interface CommandResponse {
  ok: true;
  activeRunState: ActiveRunState;
  committedTick: number;
}

interface ServerInfoWire {
  engine_version: string;
  api_version: string;
  allow_remote_viewer: boolean;
}

interface ScenarioListItemWire {
  id: string;
  path: string;
}

interface RunStatusWire {
  process_state: ProcessState;
  active_run_state: ActiveRunState;
  run_id: string | null;
  committed_tick: number;
  scenario_id: string | null;
  scenario_hash: string | null;
  effective_seed: number | null;
  terminal_reason: string | null;
  ticks_per_second?: number;
}

interface StartRunResponseWire {
  ok: true;
  run_id: string;
  scenario_hash: string;
  bootstrap_manifest: unknown;
  effective_seed: number;
  active_run_state: ActiveRunState;
}

interface CommandResponseWire {
  ok: true;
  active_run_state: ActiveRunState;
  committed_tick: number;
}

interface ErrorResponseWire {
  ok: false;
  category: string;
  message: string;
}

export function mapStatus(status: RunStatusWire): RunStatus {
  return {
    processState: status.process_state,
    activeRunState: status.active_run_state,
    runId: status.run_id,
    committedTick: status.committed_tick,
    scenarioId: status.scenario_id,
    scenarioHash: status.scenario_hash,
    effectiveSeed: status.effective_seed,
    terminalReason: status.terminal_reason,
    ...(typeof status.ticks_per_second === 'number'
      ? { ticksPerSecond: status.ticks_per_second }
      : {})
  };
}

function mapServerInfo(info: ServerInfoWire): ServerInfo {
  return {
    engineVersion: info.engine_version,
    apiVersion: info.api_version,
    allowRemoteViewer: info.allow_remote_viewer
  };
}

function mapScenarioListItem(item: ScenarioListItemWire): ScenarioListItem {
  return {
    id: item.id,
    path: item.path
  };
}

function mapStartRunResponse(response: StartRunResponseWire): StartRunResponse {
  return {
    ok: true,
    runId: response.run_id,
    scenarioHash: response.scenario_hash,
    bootstrapManifest: response.bootstrap_manifest,
    effectiveSeed: response.effective_seed,
    activeRunState: response.active_run_state
  };
}

function mapCommandResponse(response: CommandResponseWire): CommandResponse {
  return {
    ok: true,
    activeRunState: response.active_run_state,
    committedTick: response.committed_tick
  };
}

function trimTrailingSlash(baseUrl: string) {
  return baseUrl.replace(/\/+$/, '');
}

function isErrorResponse(value: unknown): value is ErrorResponseWire {
  if (typeof value !== 'object' || value === null) {
    return false;
  }

  const candidate = value as Partial<ErrorResponseWire>;
  return (
    candidate.ok === false &&
    typeof candidate.category === 'string' &&
    typeof candidate.message === 'string'
  );
}

export class RunnerApiClient {
  private readonly baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = trimTrailingSlash(baseUrl);
  }

  async getServerInfo(): Promise<ServerInfo> {
    return mapServerInfo(await this.request<ServerInfoWire>('/server/info', { method: 'GET' }));
  }

  async listScenarios(): Promise<ScenarioListItem[]> {
    const scenarios = await this.request<ScenarioListItemWire[]>('/scenarios', { method: 'GET' });
    return scenarios.map(mapScenarioListItem);
  }

  async getRunStatus(): Promise<RunStatus> {
    return mapStatus(await this.request<RunStatusWire>('/run/status', { method: 'GET' }));
  }

  async startRun(input: StartRunInput): Promise<StartRunResponse> {
    const body: Record<string, string | number> = {
      scenario_id: input.scenarioId
    };

    if (input.seedOverride !== undefined) {
      body.seed_override = input.seedOverride;
    }

    if (input.requestId !== undefined) {
      body.request_id = input.requestId;
    }

    return mapStartRunResponse(await this.post<StartRunResponseWire>('/run/start', body));
  }

  async pauseRun(): Promise<CommandResponse> {
    return this.command('/run/pause');
  }

  async resumeRun(): Promise<CommandResponse> {
    return this.command('/run/resume');
  }

  async stepRun(): Promise<CommandResponse> {
    return this.command('/run/step');
  }

  async stopRun(): Promise<CommandResponse> {
    return this.command('/run/stop');
  }

  private async command(path: string): Promise<CommandResponse> {
    return mapCommandResponse(await this.post<CommandResponseWire>(path, {}));
  }

  private post<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>(path, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify(body)
    });
  }

  private async request<T>(path: string, init: RequestInit): Promise<T> {
    const response = await fetch(`${this.baseUrl}${path}`, init);

    if (!response.ok) {
      let body: unknown;
      try {
        body = await response.json();
      } catch {
        throw new Error(`HTTP ${response.status}`);
      }

      if (isErrorResponse(body)) {
        throw new Error(`${body.category}: ${body.message}`);
      }

      throw new Error(`HTTP ${response.status}`);
    }

    const body: unknown = await response.json();
    return body as T;
  }
}
