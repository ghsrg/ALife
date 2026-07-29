import { uiText } from '../uiText';

export interface GlobalNavigationProps {
  activeWorkspace: string;
  onWorkspaceChange: (ws: string) => void;
  theme: string;
  onToggleTheme: () => void;
  warningsCount?: number;
}

export function GlobalNavigation({
  activeWorkspace,
  onWorkspaceChange,
  theme,
  onToggleTheme,
  warningsCount = 0
}: GlobalNavigationProps) {
  
  const workspaces = [
    { id: 'monitor', label: uiText.workspace.monitor },
    { id: 'organism-view', label: uiText.workspace.organismView },
    { id: 'world-editor', label: uiText.workspace.worldEditor },
    { id: 'experiments', label: 'Experiments' },
    { id: 'evolution', label: 'Evolution' },
    { id: 'specialization', label: 'Specialization' },
    { id: 'library', label: 'Library' },
    { id: 'diagnostics', label: 'Diagnostics' },
  ];

  return (
    <div className="cc-nav" data-testid="monitor-top-context">
      <div className="cc-nav-logo">
        <div className="cc-nav-logo-icon">🔷</div>
        <h1 className="cc-nav-logo-title">ALIFE CONTROL CENTER</h1>
      </div>
      
      <nav className="cc-nav-tabs" aria-label={uiText.app.primaryViews}>
        {workspaces.map(ws => (
          <button
            key={ws.id}
            role="tab"
            className="cc-nav-tab"
            aria-selected={activeWorkspace === ws.id}
            onClick={() => onWorkspaceChange(ws.id)}
          >
            {ws.label}
          </button>
        ))}
      </nav>
      
      <div className="cc-nav-actions" aria-label="Application settings">
        {warningsCount > 0 && (
          <button className="cc-warnings-badge" title="Warnings">
            ⚠ {warningsCount} WARNINGS
          </button>
        )}
        <button className="cc-nav-action-btn">EN</button>
        <button className="cc-nav-action-btn">?</button>
        <button
          className="cc-nav-action-btn"
          onClick={onToggleTheme}
          aria-label={theme === 'dark' ? uiText.controls.switchToLightTheme : uiText.controls.switchToDarkTheme}
          title={theme === 'dark' ? uiText.controls.switchToLightTheme : uiText.controls.switchToDarkTheme}
        >
          {theme === 'dark' ? '◑' : '◐'}
        </button>
        <button className="cc-nav-action-btn">⚙</button>
      </div>
    </div>
  );
}
