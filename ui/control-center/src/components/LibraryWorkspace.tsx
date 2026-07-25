import { useState } from 'react';
import type { AppState } from '../app/appState';
import {
  PRESET_LIBRARY_TEMPLATES,
  validatePlacementCommand,
  generateResearchReport,
  type SavedTemplate
} from '../app/libraryModel';

export interface LibraryWorkspaceProps {
  state: AppState;
}

export function LibraryWorkspace({ state }: LibraryWorkspaceProps) {
  const [selectedTemplate, setSelectedTemplate] = useState<SavedTemplate>(PRESET_LIBRARY_TEMPLATES[0]);
  const [posX, setPosX] = useState<string>('50');
  const [posY, setPosY] = useState<string>('50');
  const [placementNotice, setPlacementNotice] = useState<string | null>(null);

  const worldWidth = state.frame.world.width;
  const worldHeight = state.frame.world.height;

  const numX = parseFloat(posX);
  const numY = parseFloat(posY);

  const validation = validatePlacementCommand(numX, numY, worldWidth, worldHeight);
  const reportMarkdown = generateResearchReport(state);

  const handleEmitPlacement = () => {
    if (!validation.valid) return;
    setPlacementNotice(
      `Placement Command Emitted: Template "${selectedTemplate.name}" at (${numX}, ${numY}). Awaiting Core Execution.`
    );
  };

  return (
    <section className="library-workspace" aria-label="Library, Placement & Research Export" style={{ color: '#dce6f1' }}>
      <header style={{ marginBottom: '20px', display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
        <div>
          <h2 style={{ margin: 0, fontSize: '18px', fontWeight: 600 }}>Library, Placement & Research Export (AL-007-S19)</h2>
          <p style={{ margin: '4px 0 0 0', fontSize: '13px', color: '#9bb0c1' }}>
            Saved templates, Core-approved placement commands, and reproducible research report export.
          </p>
        </div>
        <span
          style={{
            fontSize: '11px',
            background: 'rgba(56, 139, 253, 0.15)',
            color: '#58a6ff',
            border: '1px solid rgba(56, 139, 253, 0.4)',
            padding: '4px 8px',
            borderRadius: '4px'
          }}
        >
          Core-Approved Commands & Reproducible Metadata
        </span>
      </header>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '24px', marginBottom: '24px' }}>
        {/* Templates Library & Placement Form */}
        <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
          <h3 style={{ margin: '0 0 12px 0', fontSize: '15px' }}>Template Library & Placement Command Builder</h3>

          <div style={{ marginBottom: '16px' }}>
            <label style={{ fontSize: '13px', display: 'block', marginBottom: '6px' }}>
              <strong>Select Saved Template:</strong>
            </label>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
              {PRESET_LIBRARY_TEMPLATES.map((tpl) => {
                const isSelected = selectedTemplate.id === tpl.id;
                return (
                  <button
                    key={tpl.id}
                    type="button"
                    onClick={() => setSelectedTemplate(tpl)}
                    style={{
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center',
                      padding: '8px 12px',
                      borderRadius: '6px',
                      background: isSelected ? 'rgba(56, 139, 253, 0.2)' : '#232931',
                      border: isSelected ? '1px solid #58a6ff' : '1px solid rgba(255,255,255,0.05)',
                      color: '#fff',
                      cursor: 'pointer',
                      textAlign: 'left'
                    }}
                  >
                    <div>
                      <strong style={{ fontSize: '13px' }}>{tpl.name}</strong>
                      <span style={{ fontSize: '11px', color: '#9bb0c1', display: 'block' }}>
                        Roles: {tpl.roles.join(', ')}
                      </span>
                    </div>
                    <span style={{ fontSize: '12px', background: 'rgba(255,255,255,0.1)', padding: '2px 6px', borderRadius: '4px' }}>
                      {tpl.cellCount} cell(s)
                    </span>
                  </button>
                );
              })}
            </div>
          </div>

          <div style={{ borderTop: '1px solid rgba(255,255,255,0.1)', paddingTop: '16px' }}>
            <h4 style={{ margin: '0 0 10px 0', fontSize: '14px' }}>Placement Coordinates (World Size: {worldWidth} x {worldHeight})</h4>
            <div style={{ display: 'flex', gap: '12px', marginBottom: '12px' }}>
              <label style={{ flex: 1, fontSize: '12px' }}>
                X Position:
                <input
                  type="number"
                  value={posX}
                  onChange={(e) => setPosX(e.target.value)}
                  style={{ width: '100%', padding: '6px', marginTop: '4px', background: '#232931', color: '#fff', border: '1px solid rgba(255,255,255,0.2)', borderRadius: '4px' }}
                />
              </label>
              <label style={{ flex: 1, fontSize: '12px' }}>
                Y Position:
                <input
                  type="number"
                  value={posY}
                  onChange={(e) => setPosY(e.target.value)}
                  style={{ width: '100%', padding: '6px', marginTop: '4px', background: '#232931', color: '#fff', border: '1px solid rgba(255,255,255,0.2)', borderRadius: '4px' }}
                />
              </label>
            </div>

            {!validation.valid ? (
              <p style={{ color: '#f85149', fontSize: '12px', margin: '0 0 12px 0' }}>{validation.reason}</p>
            ) : null}

            <button
              type="button"
              disabled={!validation.valid}
              onClick={handleEmitPlacement}
              style={{
                width: '100%',
                padding: '10px',
                borderRadius: '6px',
                background: validation.valid ? '#2ea44f' : '#30363d',
                color: '#fff',
                fontWeight: 600,
                border: 'none',
                cursor: validation.valid ? 'pointer' : 'not-allowed'
              }}
            >
              Emit Placement Command
            </button>

            {placementNotice ? (
              <p style={{ color: '#3fb950', fontSize: '12px', marginTop: '10px' }}>{placementNotice}</p>
            ) : null}
          </div>
        </div>

        {/* Reproducible Research Report Export */}
        <div style={{ background: '#1a1e24', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
          <h3 style={{ margin: '0 0 12px 0', fontSize: '15px' }}>Reproducible Research Report</h3>
          <textarea
            readOnly
            value={reportMarkdown}
            style={{
              width: '100%',
              height: '240px',
              fontFamily: 'monospace',
              fontSize: '12px',
              background: '#0d1117',
              color: '#c9d1d9',
              border: '1px solid rgba(255,255,255,0.15)',
              borderRadius: '6px',
              padding: '10px',
              resize: 'vertical'
            }}
          />
        </div>
      </div>
    </section>
  );
}
