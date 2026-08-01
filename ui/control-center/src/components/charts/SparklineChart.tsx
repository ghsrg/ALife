import { useMemo, useState } from 'react';
import type { RrdMetricHistorySample, RrdSeriesHistorySample } from '../../app/rrdMetricHistory';

export interface SparklineSeriesConfig {
  key: string;
  label: string;
  color: string;
}

export interface SparklineChartProps {
  history: Array<RrdMetricHistorySample | RrdSeriesHistorySample>;
  series: SparklineSeriesConfig[];
  height?: number;
  showArea?: boolean;
  showGrid?: boolean;
  unit?: string;
  title?: string;
}

export function SparklineChart({
  history,
  series,
  height = 100,
  showArea = true,
  showGrid = true,
  unit = '',
  title
}: SparklineChartProps) {
  const [hoveredIndex, setHoveredIndex] = useState<number | null>(null);

  const parsedData = useMemo(() => {
    if (!history || history.length === 0) return [];

    return history.map((sample) => {
      const values: Record<string, number> = {};
      if ('values' in sample && sample.values) {
        Object.assign(values, sample.values);
      } else if ('value' in sample && typeof sample.value === 'number') {
        const defaultKey = series[0]?.key ?? 'value';
        values[defaultKey] = sample.value;
      }
      return {
        tick: sample.tick,
        values
      };
    });
  }, [history, series]);

  const { paths, minTick, maxTick } = useMemo(() => {
    if (parsedData.length < 2) {
      return { paths: [], minTick: 0, maxTick: 1, minValue: 0, maxValue: 1 };
    }

    const ticks = parsedData.map((d) => d.tick);
    const minT = Math.min(...ticks);
    const maxT = Math.max(...ticks);

    let allVals: number[] = [];
    for (const d of parsedData) {
      for (const s of series) {
        const val = d.values[s.key];
        if (typeof val === 'number') allVals.push(val);
      }
    }
    if (allVals.length === 0) allVals = [0, 1];

    let minV = Math.min(0, ...allVals);
    let maxV = Math.max(1, ...allVals);
    if (maxV === minV) maxV = minV + 1;

    const svgWidth = 400;
    const svgHeight = height;
    const padding = 16;
    const chartW = svgWidth - padding * 2;
    const chartH = svgHeight - padding * 2;

    const getX = (tick: number) => {
      const ratio = maxT === minT ? 0.5 : (tick - minT) / (maxT - minT);
      return padding + ratio * chartW;
    };

    const getY = (val: number) => {
      const ratio = (val - minV) / (maxV - minV);
      return svgHeight - padding - ratio * chartH;
    };

    const computedPaths = series.map((s) => {
      const points = parsedData
        .map((d) => {
          const val = d.values[s.key];
          if (typeof val !== 'number') return null;
          return { x: getX(d.tick), y: getY(val), val, tick: d.tick };
        })
        .filter((pt): pt is { x: number; y: number; val: number; tick: number } => pt !== null);

      if (points.length === 0) {
        return { key: s.key, color: s.color, lineD: '', areaD: '', points: [] };
      }

      const lineD = points.reduce((acc, pt, idx) => {
        return idx === 0 ? `M ${pt.x.toFixed(1)} ${pt.y.toFixed(1)}` : `${acc} L ${pt.x.toFixed(1)} ${pt.y.toFixed(1)}`;
      }, '');

      const lastPt = points[points.length - 1];
      const firstPt = points[0];
      const bottomY = svgHeight - padding;
      const areaD = `${lineD} L ${lastPt.x.toFixed(1)} ${bottomY} L ${firstPt.x.toFixed(1)} ${bottomY} Z`;

      return {
        key: s.key,
        color: s.color,
        lineD,
        areaD,
        points
      };
    });

    return { paths: computedPaths, minTick: minT, maxTick: maxT, minValue: minV, maxValue: maxV };
  }, [parsedData, series, height]);

  if (parsedData.length === 0) {
    return (
      <div className="sparkline-chart empty" style={{ height }}>
        <span className="sparkline-empty-text">No time-series data</span>
      </div>
    );
  }

  const hoverSample = hoveredIndex !== null && parsedData[hoveredIndex] ? parsedData[hoveredIndex] : null;

  return (
    <div className="sparkline-chart-container" style={{ position: 'relative', width: '100%' }}>
      {title ? <div className="sparkline-title">{title}</div> : null}
      <svg
        className="sparkline-svg"
        viewBox={`0 0 400 ${height}`}
        preserveAspectRatio="none"
        style={{ width: '100%', height: `${height}px`, overflow: 'visible' }}
        onMouseLeave={() => setHoveredIndex(null)}
      >
        <defs>
          {series.map((s) => (
            <linearGradient key={`grad-${s.key}`} id={`sparkline-grad-${s.key}`} x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor={s.color} stopOpacity={0.35} />
              <stop offset="100%" stopColor={s.color} stopOpacity={0.02} />
            </linearGradient>
          ))}
        </defs>

        {/* Grid lines */}
        {showGrid ? (
          <g className="sparkline-grid" stroke="rgba(255,255,255,0.06)" strokeDasharray="3 3">
            <line x1="16" y1="16" x2="384" y2="16" />
            <line x1="16" y1={height / 2} x2="384" y2={height / 2} />
            <line x1="16" y1={height - 16} x2="384" y2={height - 16} />
          </g>
        ) : null}

        {/* Area paths */}
        {showArea &&
          paths.map((p) => (
            <path key={`area-${p.key}`} d={p.areaD} fill={`url(#sparkline-grad-${p.key})`} pointerEvents="none" />
          ))}

        {/* Line paths */}
        {paths.map((p) => (
          <path
            key={`line-${p.key}`}
            d={p.lineD}
            fill="none"
            stroke={p.color}
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
            style={{ filter: `drop-shadow(0 0 4px ${p.color}80)` }}
          />
        ))}

        {/* Interactive hit areas for ticks */}
        {parsedData.map((d, idx) => {
          const ratio = maxTick === minTick ? 0.5 : (d.tick - minTick) / (maxTick - minTick);
          const x = 16 + ratio * (400 - 32);
          return (
            <rect
              key={`hit-${d.tick}-${idx}`}
              x={x - 6}
              y={0}
              width={12}
              height={height}
              fill="transparent"
              style={{ cursor: 'pointer' }}
              onMouseEnter={() => setHoveredIndex(idx)}
            />
          );
        })}

        {/* Hover indicator crosshair */}
        {hoverSample && hoveredIndex !== null ? (
          <g className="sparkline-hover-group">
            {(() => {
              const ratio = maxTick === minTick ? 0.5 : (hoverSample.tick - minTick) / (maxTick - minTick);
              const x = 16 + ratio * (400 - 32);
              return (
                <line
                  x1={x}
                  y1={8}
                  x2={x}
                  y2={height - 8}
                  stroke="#00c896"
                  strokeWidth="1.5"
                  strokeDasharray="2 2"
                />
              );
            })()}
          </g>
        ) : null}
      </svg>

      {/* Hover tooltip overlay */}
      {hoverSample ? (
        <div className="sparkline-tooltip">
          <div className="sparkline-tooltip-tick">Tick {hoverSample.tick}</div>
          {series.map((s) => {
            const val = hoverSample.values[s.key];
            if (typeof val !== 'number') return null;
            return (
              <div key={s.key} className="sparkline-tooltip-row" style={{ color: s.color }}>
                <span>{s.label}:</span>
                <strong>
                  {val.toFixed(2)} {unit}
                </strong>
              </div>
            );
          })}
        </div>
      ) : null}
    </div>
  );
}
