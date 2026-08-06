import { useState } from 'react';
import type { SimilarityMatrixData } from '../app/evolutionModel';

export interface GenomeSimilarityMatrixProps {
  data: SimilarityMatrixData;
  onSelectCell?: (cellIdA: string, cellIdB: string) => void;
}

export function GenomeSimilarityMatrix({ data, onSelectCell }: GenomeSimilarityMatrixProps) {
  const [hoveredPair, setHoveredPair] = useState<{ i: number; j: number; score: number } | null>(null);

  if (!data.cellIds || data.cellIds.length === 0) {
    return (
      <div
        data-testid="genome-similarity-matrix"
        style={{
          padding: '24px',
          textAlign: 'center',
          color: '#64748b',
          background: '#0d1321',
          borderRadius: '8px',
          border: '1px solid rgba(255,255,255,0.06)'
        }}
      >
        No active cell similarity matrix available.
      </div>
    );
  }

  // Limit display to max 16x16 for visual clarity
  const maxN = Math.min(data.cellIds.length, 16);
  const cellIds = data.cellIds.slice(0, maxN);

  const getHeatmapColor = (score: number) => {
    // Gradient from dark slate (0.0) -> cyan (0.5) -> emerald teal/gold (1.0)
    if (score >= 0.8) {
      const alpha = 0.3 + (score - 0.8) * 3.5;
      return `rgba(0, 200, 150, ${alpha.toFixed(2)})`;
    } else if (score >= 0.4) {
      const alpha = 0.2 + (score - 0.4) * 1.5;
      return `rgba(6, 182, 212, ${alpha.toFixed(2)})`;
    } else {
      return `rgba(30, 41, 59, 0.5)`;
    }
  };

  return (
    <div
      data-testid="genome-similarity-matrix"
      style={{
        position: 'relative',
        background: '#0d1321',
        borderRadius: '8px',
        border: '1px solid rgba(255,255,255,0.08)',
        padding: '16px',
        overflow: 'hidden'
      }}
    >
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '12px' }}>
        <div>
          <span style={{ fontSize: '11px', fontWeight: 700, color: '#00c896', textTransform: 'uppercase', letterSpacing: '0.5px' }}>
            Visual Genome & Material Similarity Matrix
          </span>
          <p style={{ margin: '2px 0 0 0', fontSize: '12px', color: '#7a8a9a' }}>
            Pairwise Material Profile Divergence (Cosine Similarity)
          </p>
        </div>

        {/* Legend */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '10px', color: '#7a8a9a' }}>
          <span>Low (0.0)</span>
          <div
            style={{
              width: '60px',
              height: '8px',
              borderRadius: '4px',
              background: 'linear-gradient(90deg, rgba(30,41,59,0.5) 0%, rgba(6,182,212,0.8) 50%, rgba(0,200,150,1) 100%)'
            }}
          />
          <span style={{ color: '#00c896' }}>High (1.0)</span>
        </div>
      </div>

      <div style={{ overflowX: 'auto' }}>
        <div
          style={{
            display: 'grid',
            gridTemplateColumns: `auto repeat(${maxN}, 1fr)`,
            gap: '2px',
            alignItems: 'center',
            minWidth: '320px'
          }}
        >
          {/* Header row */}
          <div style={{ width: '40px' }} />
          {cellIds.map((id) => (
            <div
              key={`col-${id}`}
              style={{
                fontSize: '9px',
                color: '#7a8a9a',
                textAlign: 'center',
                whiteSpace: 'nowrap',
                overflow: 'hidden',
                textOverflow: 'ellipsis',
                padding: '2px'
              }}
            >
              {id}
            </div>
          ))}

          {/* Matrix Rows */}
          {cellIds.map((rowId, i) => (
            <div key={`row-${rowId}`} style={{ display: 'contents' }}>
              <div
                style={{
                  fontSize: '9px',
                  color: '#7a8a9a',
                  fontWeight: 600,
                  textAlign: 'right',
                  paddingRight: '6px',
                  whiteSpace: 'nowrap'
                }}
              >
                {rowId}
              </div>
              {cellIds.map((colId, j) => {
                const score = data.matrix[i]?.[j] ?? (i === j ? 1.0 : 0.5);
                const isHovered = hoveredPair?.i === i && hoveredPair?.j === j;

                return (
                  <div
                    key={`cell-${i}-${j}`}
                    data-testid={`matrix-cell-${i}-${j}`}
                    style={{
                      aspectRatio: '1',
                      borderRadius: '3px',
                      background: getHeatmapColor(score),
                      border: isHovered ? '1px solid #00c896' : '1px solid transparent',
                      cursor: 'pointer',
                      transition: 'border 0.15s, transform 0.15s',
                      transform: isHovered ? 'scale(1.15)' : 'none',
                      zIndex: isHovered ? 10 : 1
                    }}
                    onClick={() => onSelectCell?.(rowId, colId)}
                    onMouseEnter={() => setHoveredPair({ i, j, score })}
                    onMouseLeave={() => setHoveredPair(null)}
                  />
                );
              })}
            </div>
          ))}
        </div>
      </div>

      {/* Hover Info Tooltip */}
      {hoveredPair && (
        <div
          style={{
            position: 'absolute',
            bottom: '12px',
            right: '16px',
            background: 'rgba(10, 14, 26, 0.95)',
            border: '1px solid #00c896',
            borderRadius: '6px',
            padding: '6px 10px',
            fontSize: '11px',
            color: '#e8edf5',
            pointerEvents: 'none',
            boxShadow: '0 4px 12px rgba(0,0,0,0.5)'
          }}
        >
          <span style={{ color: '#00c896', fontWeight: 700 }}>
            {data.cellIds[hoveredPair.i]} ({data.roles[hoveredPair.i]}) ↔ {data.cellIds[hoveredPair.j]} ({data.roles[hoveredPair.j]})
          </span>
          <div style={{ fontSize: '12px', fontWeight: 700, marginTop: '2px' }}>
            Similarity Score: {(hoveredPair.score * 100).toFixed(1)}%
          </div>
        </div>
      )}
    </div>
  );
}
