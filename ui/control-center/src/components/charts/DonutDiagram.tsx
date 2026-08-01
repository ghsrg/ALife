import { useMemo, useState } from 'react';

export interface DonutSegment {
  label: string;
  value: number;
  color: string;
}

export interface DonutDiagramProps {
  segments: DonutSegment[];
  size?: number;
  thickness?: number;
  centerText?: string;
  centerSubtext?: string;
  showLegend?: boolean;
}

export function DonutDiagram({
  segments,
  size = 110,
  thickness = 14,
  centerText,
  centerSubtext,
  showLegend = true
}: DonutDiagramProps) {
  const [hoveredIdx, setHoveredIdx] = useState<number | null>(null);

  const total = useMemo(() => {
    return segments.reduce((sum, seg) => sum + Math.max(0, seg.value), 0);
  }, [segments]);

  const arcs = useMemo(() => {
    if (total <= 0) return [];

    const radius = size / 2 - thickness / 2 - 2;
    const center = size / 2;
    const circumference = 2 * Math.PI * radius;

    let accumulatedAngle = -90; // Start at top

    return segments.map((seg, idx) => {
      const pct = Math.max(0, seg.value) / total;
      const strokeDasharray = `${pct * circumference} ${circumference}`;
      const angle = accumulatedAngle;
      accumulatedAngle += pct * 360;

      return {
        ...seg,
        pct,
        radius,
        center,
        strokeDasharray,
        angle,
        idx
      };
    });
  }, [segments, total, size, thickness]);

  if (total <= 0) {
    return (
      <div className="donut-diagram empty" style={{ width: size, height: size }}>
        <span className="donut-empty-text">No data</span>
      </div>
    );
  }

  const activeSegment = hoveredIdx !== null ? segments[hoveredIdx] : null;

  return (
    <div className="donut-diagram-wrapper" style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
      <div className="donut-svg-container" style={{ position: 'relative', width: `${size}px`, height: `${size}px` }}>
        <svg
          width={size}
          height={size}
          viewBox={`0 0 ${size} ${size}`}
          className="donut-svg"
          onMouseLeave={() => setHoveredIdx(null)}
        >
          {arcs.map((arc) => {
            const isHovered = hoveredIdx === arc.idx;
            const currentThickness = isHovered ? thickness + 4 : thickness;

            return (
              <circle
                key={`arc-${arc.label}-${arc.idx}`}
                cx={arc.center}
                cy={arc.center}
                r={arc.radius}
                fill="none"
                stroke={arc.color}
                strokeWidth={currentThickness}
                strokeDasharray={arc.strokeDasharray}
                transform={`rotate(${arc.angle} ${arc.center} ${arc.center})`}
                style={{
                  transition: 'stroke-width 0.2s ease, filter 0.2s ease',
                  cursor: 'pointer',
                  filter: isHovered ? `drop-shadow(0 0 6px ${arc.color})` : undefined
                }}
                onMouseEnter={() => setHoveredIdx(arc.idx)}
              />
            );
          })}
        </svg>

        {/* Center label */}
        <div
          className="donut-center-label"
          style={{
            position: 'absolute',
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            justifyContent: 'center',
            pointerEvents: 'none'
          }}
        >
          {activeSegment ? (
            <>
              <strong style={{ fontSize: '12px', color: activeSegment.color }}>
                {activeSegment.value.toLocaleString()}
              </strong>
              <span style={{ fontSize: '9px', color: '#7a8a9a' }}>
                {((activeSegment.value / total) * 100).toFixed(0)}%
              </span>
            </>
          ) : (
            <>
              <strong style={{ fontSize: '13px', color: '#e8edf5' }}>
                {centerText ?? total.toLocaleString()}
              </strong>
              {centerSubtext ? <span style={{ fontSize: '9px', color: '#7a8a9a' }}>{centerSubtext}</span> : null}
            </>
          )}
        </div>
      </div>

      {showLegend ? (
        <div className="donut-legend" style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
          {segments.map((seg, idx) => {
            const pct = total > 0 ? (seg.value / total) * 100 : 0;
            const isHovered = hoveredIdx === idx;
            return (
              <div
                key={`legend-${seg.label}`}
                className={`donut-legend-item ${isHovered ? 'active' : ''}`}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  fontSize: '11px',
                  color: isHovered ? '#e8edf5' : '#c5d3e0',
                  cursor: 'pointer',
                  fontWeight: isHovered ? 700 : 400
                }}
                onMouseEnter={() => setHoveredIdx(idx)}
                onMouseLeave={() => setHoveredIdx(null)}
              >
                <span
                  style={{
                    width: '8px',
                    height: '8px',
                    borderRadius: '50%',
                    backgroundColor: seg.color,
                    boxShadow: isHovered ? `0 0 6px ${seg.color}` : 'none'
                  }}
                />
                <span>{seg.label}:</span>
                <strong style={{ marginLeft: 'auto' }}>{pct.toFixed(0)}%</strong>
              </div>
            );
          })}
        </div>
      ) : null}
    </div>
  );
}
