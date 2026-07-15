import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { RunnerApiClient, mapStatus } from './apiClient';

const fetchMock = vi.fn<typeof fetch>();

function jsonResponse(body: unknown, init: ResponseInit = {}) {
  return new Response(JSON.stringify(body), {
    status: init.status ?? 200,
    headers: { 'content-type': 'application/json' },
    ...init
  });
}

describe('RunnerApiClient', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', fetchMock);
    fetchMock.mockReset();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('loads server info and scenarios', async () => {
    const client = new RunnerApiClient('http://127.0.0.1:8080/');
    fetchMock
      .mockResolvedValueOnce(
        jsonResponse({
          engine_version: '0.1.0',
          api_version: '1',
          allow_remote_viewer: false
        })
      )
      .mockResolvedValueOnce(
        jsonResponse([
          {
            id: 'single_cell_survival',
            name: 'Single Cell Survival',
            description: 'Demo scenario',
            path: 'config/scenarios/single_cell_survival.toml'
          }
        ])
      );

    await expect(client.getServerInfo()).resolves.toEqual({
      engineVersion: '0.1.0',
      apiVersion: '1',
      allowRemoteViewer: false
    });
    await expect(client.listScenarios()).resolves.toEqual([
      { id: 'single_cell_survival', path: 'config/scenarios/single_cell_survival.toml' }
    ]);

    expect(fetchMock).toHaveBeenNthCalledWith(1, 'http://127.0.0.1:8080/server/info', {
      method: 'GET'
    });
    expect(fetchMock).toHaveBeenNthCalledWith(2, 'http://127.0.0.1:8080/scenarios', {
      method: 'GET'
    });
  });

  it('loads run status with camelCase fields', async () => {
    const client = new RunnerApiClient('http://runner.local');
    fetchMock.mockResolvedValueOnce(
      jsonResponse({
        process_state: 'ready',
        active_run_state: 'running',
        run_id: 'run-123',
        committed_tick: 42,
        scenario_id: 'single_cell_survival',
        scenario_hash: 'scenario-hash',
        effective_seed: 1234,
        terminal_reason: null
      })
    );

    await expect(client.getRunStatus()).resolves.toEqual({
      processState: 'ready',
      activeRunState: 'running',
      runId: 'run-123',
      committedTick: 42,
      scenarioId: 'single_cell_survival',
      scenarioHash: 'scenario-hash',
      effectiveSeed: 1234,
      terminalReason: null
    });
  });

  it('maps status payloads for stream clients', () => {
    expect(
      mapStatus({
        process_state: 'ready',
        active_run_state: 'paused',
        run_id: null,
        committed_tick: 7,
        scenario_id: null,
        scenario_hash: null,
        effective_seed: null,
        terminal_reason: 'completed'
      })
    ).toEqual({
      processState: 'ready',
      activeRunState: 'paused',
      runId: null,
      committedTick: 7,
      scenarioId: null,
      scenarioHash: null,
      effectiveSeed: null,
      terminalReason: 'completed'
    });
  });

  it('starts a run with snake_case POST body and maps response', async () => {
    const client = new RunnerApiClient('http://127.0.0.1:8080');
    fetchMock.mockResolvedValueOnce(
      jsonResponse({
        ok: true,
        run_id: 'run-1',
        scenario_hash: 'hash-1',
        bootstrap_manifest: { prepared_tick: 0 },
        effective_seed: 99,
        active_run_state: 'running'
      })
    );

    await expect(
      client.startRun({
        scenarioId: 'single_cell_survival',
        seedOverride: 99,
        requestId: 'request-1'
      })
    ).resolves.toEqual({
      ok: true,
      runId: 'run-1',
      scenarioHash: 'hash-1',
      bootstrapManifest: { prepared_tick: 0 },
      effectiveSeed: 99,
      activeRunState: 'running'
    });

    expect(fetchMock).toHaveBeenCalledWith('http://127.0.0.1:8080/run/start', {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({
        scenario_id: 'single_cell_survival',
        seed_override: 99,
        request_id: 'request-1'
      })
    });
  });

  it.each([
    ['pauseRun', '/run/pause', 'paused'],
    ['resumeRun', '/run/resume', 'running'],
    ['stepRun', '/run/step', 'paused'],
    ['stopRun', '/run/stop', 'completed']
  ] as const)('posts %s command and maps response', async (methodName, path, activeRunState) => {
    const client = new RunnerApiClient('http://127.0.0.1:8080');
    fetchMock.mockResolvedValueOnce(
      jsonResponse({
        ok: true,
        active_run_state: activeRunState,
        committed_tick: 12
      })
    );

    await expect(client[methodName]()).resolves.toEqual({
      ok: true,
      activeRunState,
      committedTick: 12
    });
    expect(fetchMock).toHaveBeenCalledWith(`http://127.0.0.1:8080${path}`, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: '{}'
    });
  });

  it('throws category and message for non-ok JSON error responses', async () => {
    const client = new RunnerApiClient('http://127.0.0.1:8080');
    fetchMock.mockResolvedValueOnce(
      jsonResponse(
        {
          ok: false,
          category: 'state_conflict',
          message: 'Run already active'
        },
        { status: 409 }
      )
    );

    await expect(client.startRun({ scenarioId: 'single_cell_survival' })).rejects.toThrow(
      'state_conflict: Run already active'
    );
  });

  it('throws HTTP status for non-ok responses with non-JSON bodies', async () => {
    const client = new RunnerApiClient('http://127.0.0.1:8080');
    fetchMock.mockResolvedValueOnce({
      ok: false,
      status: 502,
      json: vi.fn().mockRejectedValue(new SyntaxError('Unexpected token <'))
    } as unknown as Response);

    await expect(client.getRunStatus()).rejects.toThrow('HTTP 502');
  });
});
