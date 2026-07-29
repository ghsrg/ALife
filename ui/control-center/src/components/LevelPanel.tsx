export type AnalysisLevel = 'world' | 'cells' | 'organisms' | 'lineages' | 'evolution' | 'analytics';

const LEVELS = [
  { id: 'world', letter: 'W', name: 'World', available: true },
  { id: 'cells', letter: 'C', name: 'Cells', available: true },
  { id: 'organisms', letter: 'O', name: 'Organisms', available: false },
  { id: 'lineages', letter: 'L', name: 'Lineages', available: false },
  { id: 'evolution', letter: 'E', name: 'Evolution', available: false },
  { id: 'analytics', letter: 'A', name: 'Analytics', available: false },
];

export interface LevelPanelProps {
  activeLevel: AnalysisLevel;
  onLevelChange: (level: AnalysisLevel) => void;
}

export function LevelPanel({ activeLevel, onLevelChange }: LevelPanelProps) {
  return (
    <div className="cc-level-panel" data-testid="monitor-level-track">
      <div className="cc-level-label">LEVEL</div>
      {LEVELS.map(level => (
        <div
          key={level.id}
          className={`cc-level-item ${activeLevel === level.id ? 'active' : ''} ${!level.available ? 'disabled' : ''}`}
          onClick={() => {
            if (level.available) {
              onLevelChange(level.id as AnalysisLevel);
            }
          }}
          title={!level.available ? "Not available" : level.name}
        >
          <div className="cc-level-letter">{level.letter}</div>
          <div className="cc-level-name">{level.name}</div>
        </div>
      ))}
    </div>
  );
}
