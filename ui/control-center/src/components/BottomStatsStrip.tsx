import type { MonitorStat } from './monitorStats';

interface BottomStatsStripProps {
  stats: MonitorStat[];
}

export function BottomStatsStrip({ stats }: BottomStatsStripProps) {
  return (
    <section className="bottom-stats-strip" aria-label="World stats" data-testid="bottom-stats-strip">
      {stats.slice(0, 5).map((stat) => (
        <article
          key={`${stat.id}-${stat.label}`}
          className={`bottom-stat bottom-stat-${stat.state}`}
          data-testid="bottom-stat"
        >
          <span>{stat.label}</span>
          <strong>{stat.value}</strong>
          {stat.note ? <small>{stat.note}</small> : null}
        </article>
      ))}
    </section>
  );
}
