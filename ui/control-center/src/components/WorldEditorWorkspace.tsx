import { useEffect, useState } from 'react';
import type { AppState } from '../app/appState';
import {
  PRESET_SCENARIOS,
  clearDraftInLocalStorage,
  computeConfigHash,
  loadDraftFromLocalStorage,
  saveDraftToLocalStorage,
  validateScenarioToml,
  type ValidationResult
} from '../app/worldEditorModel';

export interface WorldEditorWorkspaceProps {
  state: AppState;
  onSelectScenario?: (scenarioId: string) => void;
  onRelaunchRun?: (scenarioId: string, seed: number) => void;
}

export function WorldEditorWorkspace({
  state,
  onSelectScenario,
  onRelaunchRun
}: WorldEditorWorkspaceProps) {
  const [selectedPresetId, setSelectedPresetId] = useState<string>(
    state.selectedScenarioId ?? 'diverse_rich_world'
  );

  const currentPreset =
    PRESET_SCENARIOS.find((p) => p.id === selectedPresetId) ?? PRESET_SCENARIOS[0];

  const [tomlText, setTomlText] = useState<string>(() => {
    return loadDraftFromLocalStorage(selectedPresetId) ?? currentPreset.defaultToml;
  });

  const [seedInput, setSeedInput] = useState<number>(42);
  const [configHash, setConfigHash] = useState<string>('calculating...');
  const [validation, setValidation] = useState<ValidationResult>(() =>
    validateScenarioToml(tomlText)
  );
  const [isDraftSaved, setIsDraftSaved] = useState<boolean>(false);

  // When preset changes, load preset default or saved draft
  const handlePresetChange = (presetId: string) => {
    setSelectedPresetId(presetId);
    if (onSelectScenario) {
      onSelectScenario(presetId);
    }
    const loadedDraft = loadDraftFromLocalStorage(presetId);
    const preset = PRESET_SCENARIOS.find((p) => p.id === presetId) ?? PRESET_SCENARIOS[0];
    const nextText = loadedDraft ?? preset.defaultToml;
    setTomlText(nextText);
    setValidation(validateScenarioToml(nextText));
    setIsDraftSaved(loadedDraft !== null);
  };

  // Live validation and hash computation on text change
  useEffect(() => {
    setValidation(validateScenarioToml(tomlText));
    void computeConfigHash(tomlText).then(setConfigHash);
  }, [tomlText]);

  const handleSaveDraft = () => {
    saveDraftToLocalStorage(selectedPresetId, tomlText);
    setIsDraftSaved(true);
  };

  const handleResetPreset = () => {
    clearDraftInLocalStorage(selectedPresetId);
    setTomlText(currentPreset.defaultToml);
    setIsDraftSaved(false);
  };

  const handleRelaunchSameSeed = () => {
    if (onRelaunchRun) {
      onRelaunchRun(selectedPresetId, seedInput);
    }
  };

  const handleRelaunchNewSeed = () => {
    const newSeed = Math.floor(Math.random() * 1_000_000) + 1;
    setSeedInput(newSeed);
    if (onRelaunchRun) {
      onRelaunchRun(selectedPresetId, newSeed);
    }
  };

  return (
    <div className="world-editor-workspace" data-testid="world-editor-workspace">
      {/* Top Controls Header Card */}
      <div className="v3-chart-card editor-header-card">
        <header className="card-header-v3">
          <span className="card-num">E1</span>
          <h4>WORLD EDITOR & SCENARIO RUNNER</h4>
          <span className="na-badge active" style={{ backgroundColor: '#2a9d8f' }}>
            READ-ONLY LIVE ISOLATION
          </span>
        </header>

        <div className="editor-preset-row" style={{ display: 'flex', gap: '16px', marginTop: '12px', alignItems: 'center' }}>
          <label style={{ fontSize: '13px', fontWeight: 600, color: 'var(--text-color, #fff)' }}>
            Scenario Preset:
            <select
              aria-label="Scenario Preset"
              value={selectedPresetId}
              onChange={(e) => handlePresetChange(e.target.value)}
              style={{
                marginLeft: '8px',
                padding: '6px 12px',
                borderRadius: '4px',
                background: 'var(--bg-input, #1e293b)',
                color: 'var(--text-color, #fff)',
                border: '1px solid var(--border-color, #334155)'
              }}
            >
              {PRESET_SCENARIOS.map((p) => (
                <option key={p.id} value={p.id}>
                  {p.name}
                </option>
              ))}
            </select>
          </label>

          <label style={{ fontSize: '13px', fontWeight: 600, color: 'var(--text-color, #fff)' }}>
            Seed:
            <input
              type="number"
              aria-label="Seed Input"
              value={seedInput}
              onChange={(e) => setSeedInput(parseInt(e.target.value, 10) || 0)}
              style={{
                marginLeft: '8px',
                width: '100px',
                padding: '6px 8px',
                borderRadius: '4px',
                background: 'var(--bg-input, #1e293b)',
                color: 'var(--text-color, #fff)',
                border: '1px solid var(--border-color, #334155)'
              }}
            />
          </label>

          <div style={{ fontSize: '12px', color: '#94a3b8', marginLeft: 'auto' }}>
            Config SHA-256 Hash: <strong style={{ color: '#38bdf8', fontFamily: 'monospace' }}>{configHash}</strong>
          </div>
        </div>
      </div>

      {/* Grid Layout for TOML Editor & Diagnostics */}
      <div className="editor-grid" style={{ display: 'grid', gridTemplateColumns: '2fr 1fr', gap: '16px', marginTop: '16px' }}>
        {/* Left Column: TOML Text Editor */}
        <div className="v3-chart-card">
          <header className="card-header-v3">
            <span className="card-num">E2</span>
            <h4>PRE-RUN TOML CONFIGURATION EDITOR</h4>
            {isDraftSaved && (
              <span className="na-badge" style={{ backgroundColor: '#e76f51' }}>
                LOCAL DRAFT SAVED
              </span>
            )}
          </header>

          <div style={{ marginTop: '12px' }}>
            <textarea
              aria-label="TOML Configuration Editor"
              value={tomlText}
              onChange={(e) => setTomlText(e.target.value)}
              rows={18}
              style={{
                width: '100%',
                fontFamily: 'Consolas, Monaco, monospace',
                fontSize: '13px',
                padding: '12px',
                borderRadius: '6px',
                background: '#0f172a',
                color: '#f8fafc',
                border: validation.isValid ? '1px solid #334155' : '1px solid #ef4444',
                resize: 'vertical'
              }}
            />
          </div>

          <div className="editor-actions" style={{ display: 'flex', gap: '12px', marginTop: '12px' }}>
            <button
              type="button"
              onClick={handleSaveDraft}
              style={{
                padding: '8px 16px',
                borderRadius: '4px',
                background: '#2563eb',
                color: '#fff',
                border: 'none',
                cursor: 'pointer',
                fontWeight: 600
              }}
            >
              Save Local Draft
            </button>
            <button
              type="button"
              onClick={handleResetPreset}
              style={{
                padding: '8px 16px',
                borderRadius: '4px',
                background: '#475569',
                color: '#fff',
                border: 'none',
                cursor: 'pointer'
              }}
            >
              Restore Preset Default
            </button>
          </div>
        </div>

        {/* Right Column: Diagnostics & Scenario Runner Actions */}
        <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
          {/* Validation Diagnostics Card */}
          <div className="v3-chart-card">
            <header className="card-header-v3">
              <span className="card-num">E3</span>
              <h4>VALIDATION DIAGNOSTICS</h4>
            </header>

            <div style={{ marginTop: '12px' }}>
              {validation.isValid ? (
                <div style={{ padding: '8px 12px', borderRadius: '4px', background: '#064e3b', color: '#6ee7b7', fontSize: '13px', fontWeight: 600 }}>
                  ✓ TOML Configuration Valid
                </div>
              ) : (
                <div style={{ padding: '8px 12px', borderRadius: '4px', background: '#7f1d1d', color: '#fca5a5', fontSize: '13px' }}>
                  <strong>Validation Errors ({validation.errors.length}):</strong>
                  <ul style={{ marginTop: '6px', paddingLeft: '20px', margin: 0 }}>
                    {validation.errors.map((err, i) => (
                      <li key={i}>{err}</li>
                    ))}
                  </ul>
                </div>
              )}

              {validation.warnings.length > 0 && (
                <div style={{ marginTop: '8px', padding: '8px 12px', borderRadius: '4px', background: '#78350f', color: '#fde68a', fontSize: '12px' }}>
                  <strong>Warnings:</strong>
                  <ul style={{ marginTop: '4px', paddingLeft: '20px', margin: 0 }}>
                    {validation.warnings.map((w, i) => (
                      <li key={i}>{w}</li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
          </div>

          {/* Scenario Relaunch Controls Card */}
          <div className="v3-chart-card">
            <header className="card-header-v3">
              <span className="card-num">E4</span>
              <h4>RELAUNCH SCENARIO SIMULATION</h4>
            </header>

            <div style={{ marginTop: '12px', display: 'flex', flexDirection: 'column', gap: '10px' }}>
              <button
                type="button"
                aria-label="Relaunch Simulation with Config"
                disabled={!validation.isValid}
                onClick={handleRelaunchSameSeed}
                style={{
                  padding: '10px 16px',
                  borderRadius: '4px',
                  background: validation.isValid ? '#059669' : '#334155',
                  color: '#fff',
                  border: 'none',
                  fontWeight: 700,
                  fontSize: '13px',
                  cursor: validation.isValid ? 'pointer' : 'not-allowed'
                }}
              >
                Relaunch Simulation (Seed {seedInput})
              </button>

              <button
                type="button"
                aria-label="Relaunch with New Random Seed"
                disabled={!validation.isValid}
                onClick={handleRelaunchNewSeed}
                style={{
                  padding: '10px 16px',
                  borderRadius: '4px',
                  background: validation.isValid ? '#0891b2' : '#334155',
                  color: '#fff',
                  border: 'none',
                  fontWeight: 600,
                  fontSize: '13px',
                  cursor: validation.isValid ? 'pointer' : 'not-allowed'
                }}
              >
                Relaunch with New Random Seed
              </button>

              <p style={{ fontSize: '11px', color: '#94a3b8', margin: 0, marginTop: '4px', lineHeight: 1.4 }}>
                <strong>Execution Safety:</strong> Edits apply to pre-run scenario initialization. Active live WorldState is protected and read-only.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
