import { useMemo, useState } from 'react';

export interface HistogramBin {
  label: string;
  count: number;
  color?: string;
}

export interface HistogramChartProps {
  bins: HistogramBin[];
  height?: number;
  barColor?: string;
  title?: string;
}

export function HistogramChart({
  bins,
  height = 90,
  barColor = '#00c896',
  title
}: HistogramChartProps) {
  const [hoveredIdx, setHoveredIdx] = useState<number | null>(null);

  const maxCount = useMemo(() => {
    if (bins.length === 0) return 1;
    return Math.max(1, ...bins.map((b) => b.count));
  }, [bins]);

  if (bins.length === 0) {
    return (
      <div className="histogram-chart empty" style={{ height }}>
        <span>No distribution data available</span>
      </div>
    );
  }

  return (
    <div className="histogram-chart-container" style={{ width: '100%' }}>
      {title ? <div className="histogram-title">{title}</div> : null}
      <div
        className="histogram-bars-wrapper"
        style={{
          display: 'flex',
          alignItems: 'flex-end',
          gap: '4px',
          height: `${height}px`,
          padding: '8px 0',
          position: 'relative'
        }}
        onMouseLeave={() => setHoveredIdx(null)}
      >
        {bins.map((bin, idx) => {
          const heightPct = (bin.count / maxCount) * 100;
          const isHovered = hoveredIdx === idx;
          const color = bin.color ?? barColor;

          return (
            <div
              key={`bin-${bin.label}-${idx}`}
              className="histogram-bar-col"
              style={{
                flex: 1,
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                height: '100%',
                justifyContent: 'flex-end',
                position: 'relative',
                cursor: 'pointer'
              }}
              onMouseEnter={() => setHoveredIdx(idx)}
            >
              {/* Tooltip on hover */}
              {isHovered ? (
                <div
                  className="histogram-tooltip"
                  style={{
                    position: 'absolute',
                    top: '-24px',
                    background: '#070b18',
                    border: `1px solid ${color}`,
                    borderRadius: '4px',
                    padding: '2px 6px',
                    fontSize: '10px',
                    whiteSpace: 'nowrap',
                    color: '#e8edf5',
                    zIndex: 10
                  }}
                >
                  {bin.label}: {bin.count}
                </div>
              ) : null}

              {/* Bar */}
              <div
                className="histogram-bar"
                style={{
                  width: '100%',
                  height: `${Math.max(4, heightPct)}%`,
                  backgroundColor: color,
                  borderRadius: '3px 3px 0 0',
                  opacity: isHovered ? 1 : 0.75,
                  boxShadow: isHovered ? `0 0 8px ${color}` : 'none',
                  transition: 'height 0.2s ease, opacity 0.15s ease'
                }}
              />
              <span
                className="histogram-label"
                style={{
                  fontSize: '9px',
                  color: isHovered ? '#00c896' : '#7a8a9a',
                  marginTop: '4px',
                  whiteSpace: 'nowrap',
                  overflow: 'hidden',
                  textOverflow: 'ellipsis',
                  maxWidth: '100%'
                }}
              >
                {bin.label}
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
}
