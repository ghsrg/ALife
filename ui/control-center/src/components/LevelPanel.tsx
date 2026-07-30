export type AnalysisLevel = 'world' | 'cells' | 'organisms' | 'lineages' | 'evolution' | 'analytics';

const LEVELS: Array<{ id: AnalysisLevel; name: string; available: boolean }> = [
  { id: 'world', name: 'World', available: true },
  { id: 'cells', name: 'Cells', available: true },
  { id: 'organisms', name: 'Organisms', available: false },
  { id: 'lineages', name: 'Lineages', available: false },
  { id: 'evolution', name: 'Evolution', available: false },
  { id: 'analytics', name: 'Analytics', available: false }
];

export interface LevelPanelProps {
  activeLevel: AnalysisLevel;
  onLevelChange: (level: AnalysisLevel) => void;
}

export function LevelPanel({ activeLevel, onLevelChange }: LevelPanelProps) {
  return (
    <div className="cc-level-panel" data-testid="monitor-level-track">
      <div className="cc-level-label">LEVEL</div>
      {LEVELS.map((level) => {
        const isActive = activeLevel === level.id;
        return (
          <button
            key={level.id}
            type="button"
            className={`cc-level-item ${isActive ? 'active' : ''}`}
            aria-label={`${level.name} level`}
            aria-pressed={isActive}
            disabled={!level.available}
            title={!level.available ? `${level.name} level is not available yet` : `${level.name} level`}
            onClick={() => onLevelChange(level.id)}
          >
            <LevelIcon id={level.id} />
            <span className="cc-level-name">{level.name}</span>
          </button>
        );
      })}
    </div>
  );
}

function LevelIcon({ id }: { id: AnalysisLevel }) {
  return (
    <svg
      className="cc-level-icon"
      data-testid={`level-icon-${id}`}
      aria-hidden="true"
      viewBox="0 0 24 24"
      focusable="false"
    >
      {iconPath(id)}
    </svg>
  );
}

function iconPath(id: AnalysisLevel) {
  switch (id) {
    case 'world':
      return (
        <>
          <circle cx="12" cy="12" r="8" />
          <path d="M4 12h16M12 4a13 13 0 0 1 0 16M12 4a13 13 0 0 0 0 16" />
        </>
      );
    case 'cells':
      return (
        <>
          <circle cx="12" cy="12" r="2" />
          <circle cx="7" cy="10" r="1.4" />
          <circle cx="16.5" cy="9" r="1.2" />
          <circle cx="9" cy="16" r="1.1" />
          <circle cx="17" cy="15" r="1.5" />
          <path d="M5 6a10 10 0 0 1 14 0M5 18a10 10 0 0 0 14 0" />
        </>
      );
    case 'organisms':
      return (
        <>
          <circle cx="6" cy="15" r="3" />
          <circle cx="12" cy="8" r="3" />
          <circle cx="18" cy="15" r="3" />
          <path d="M8.4 12.8 10 10.5M14 10.5l1.6 2.3M9 15h6" />
        </>
      );
    case 'lineages':
      return (
        <>
          <circle cx="5" cy="18" r="1.5" />
          <circle cx="12" cy="12" r="1.5" />
          <circle cx="19" cy="6" r="1.5" />
          <circle cx="19" cy="18" r="1.5" />
          <path d="M6.2 17 10.8 13M13.2 11 17.8 7M13.4 13.1 17.6 17" />
        </>
      );
    case 'evolution':
      return (
        <>
          <path d="M7 4c8 2 2 14 10 16M17 4C9 6 15 18 7 20" />
          <path d="M9 8h6M8 12h8M9 16h6" />
        </>
      );
    case 'analytics':
    default:
      return (
        <>
          <path d="M4 20h16M6 17V9M12 17V5M18 17v-7" />
          <path d="M5 8l5-3 4 5 5-4" />
        </>
      );
  }
}
