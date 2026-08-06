import { useState } from 'react';
import type { LineageTreeData, LineageTreeNode } from '../app/evolutionModel';

export interface LineageTreeDiagramProps {
  tree: LineageTreeData;
  selectedNodeId?: string | null;
  onSelectNode?: (nodeId: string) => void;
}

const ROLE_COLORS: Record<string, string> = {
  Boundary: '#eab308',
  Transport: '#3b82f6',
  Metabolic: '#22c55e',
  Storage: '#a855f7',
  Synthesis: '#ec4899',
  Structural: '#64748b',
  Repair: '#14b8a6',
  Contractile: '#f97316',
  Sensory: '#06b6d4'
};

export function LineageTreeDiagram({ tree, selectedNodeId, onSelectNode }: LineageTreeDiagramProps) {
  const [hoveredNode, setHoveredNode] = useState<LineageTreeNode | null>(null);

  if (!tree.nodes || tree.nodes.length === 0) {
    return (
      <div
        data-testid="lineage-tree-diagram"
        style={{
          padding: '24px',
          textAlign: 'center',
          color: '#64748b',
          background: '#0d1321',
          borderRadius: '8px',
          border: '1px solid rgba(255,255,255,0.06)'
        }}
      >
        No active lineage tree data available in current frame.
      </div>
    );
  }

  // Calculate layout coordinates for SVG rendering
  const maxGen = Math.max(tree.maxDepth, 1);
  const genGroups = new Map<number, LineageTreeNode[]>();

  tree.nodes.forEach((node) => {
    const list = genGroups.get(node.generation) ?? [];
    list.push(node);
    genGroups.set(node.generation, list);
  });

  const width = 800;
  const height = 300;
  const paddingX = 60;
  const paddingY = 40;
  const availableWidth = width - paddingX * 2;
  const availableHeight = height - paddingY * 2;

  const nodePositions = new Map<string, { x: number; y: number }>();

  genGroups.forEach((groupNodes, gen) => {
    const x = paddingX + (gen / maxGen) * availableWidth;
    const count = groupNodes.length;
    groupNodes.forEach((node, idx) => {
      const y = paddingY + ((idx + 0.5) / count) * availableHeight;
      nodePositions.set(node.id, { x, y });
    });
  });

  // Render parent-child connecting paths
  const paths: { key: string; d: string; color: string }[] = [];
  tree.nodes.forEach((node) => {
    if (node.parentCellId) {
      const parentPos = nodePositions.get(node.parentCellId);
      const childPos = nodePositions.get(node.id);
      if (parentPos && childPos) {
        const midX = (parentPos.x + childPos.x) / 2;
        const d = `M ${parentPos.x} ${parentPos.y} C ${midX} ${parentPos.y}, ${midX} ${childPos.y}, ${childPos.x} ${childPos.y}`;
        const color = ROLE_COLORS[node.role] ?? '#00c896';
        paths.push({ key: `${node.parentCellId}->${node.id}`, d, color });
      }
    }
  });

  return (
    <div
      data-testid="lineage-tree-diagram"
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
            Visual Lineage Tree & Speciation Branches
          </span>
          <p style={{ margin: '2px 0 0 0', fontSize: '12px', color: '#7a8a9a' }}>
            Max Depth: Gen {tree.maxDepth} | Nodes: {tree.nodes.length} | Speciation Events: {tree.speciationEventsCount}
          </p>
        </div>

        {/* Legend */}
        <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
          {Object.entries(ROLE_COLORS).slice(0, 5).map(([role, color]) => (
            <div key={role} style={{ display: 'flex', alignItems: 'center', gap: '4px', fontSize: '10px', color: '#c5d3e0' }}>
              <span style={{ width: '8px', height: '8px', borderRadius: '50%', background: color }} />
              {role}
            </div>
          ))}
        </div>
      </div>

      <svg viewBox={`0 0 ${width} ${height}`} style={{ width: '100%', height: 'auto', display: 'block' }}>
        {/* Generation Level Lines */}
        {Array.from({ length: maxGen + 1 }).map((_, gen) => {
          const x = paddingX + (gen / maxGen) * availableWidth;
          return (
            <g key={`gen-${gen}`}>
              <line x1={x} y1={20} x2={x} y2={height - 20} stroke="rgba(255,255,255,0.05)" strokeDasharray="3 3" />
              <text x={x} y={15} fill="#7a8a9a" fontSize="10" textAnchor="middle" fontWeight="600">
                Gen {gen}
              </text>
            </g>
          );
        })}

        {/* Bezier Connecting Curves */}
        {paths.map((path) => (
          <path key={path.key} d={path.d} fill="none" stroke={path.color} strokeWidth="1.5" strokeOpacity="0.4" />
        ))}

        {/* Lineage Tree Nodes */}
        {tree.nodes.map((node) => {
          const pos = nodePositions.get(node.id) ?? { x: 0, y: 0 };
          const color = ROLE_COLORS[node.role] ?? '#00c896';
          const isSelected = selectedNodeId === node.id;
          const isHovered = hoveredNode?.id === node.id;

          return (
            <g
              key={node.id}
              data-testid={`lineage-node-${node.id}`}
              transform={`translate(${pos.x}, ${pos.y})`}
              style={{ cursor: 'pointer' }}
              onClick={() => onSelectNode?.(node.id)}
              onMouseEnter={() => setHoveredNode(node)}
              onMouseLeave={() => setHoveredNode(null)}
            >
              {/* Outer Glow */}
              <circle
                r={isSelected || isHovered ? 10 : 7}
                fill={color}
                fillOpacity={isSelected ? 0.4 : 0.2}
                stroke={color}
                strokeWidth={isSelected ? 2 : 1}
              />

              {/* Inner Node Core */}
              <circle r={isSelected ? 5 : 4} fill={color} />

              {/* Label */}
              <text x={12} y={3} fill={isSelected ? '#ffffff' : '#c5d3e0'} fontSize="9" fontWeight={isSelected ? '700' : '400'}>
                {node.id}
              </text>
            </g>
          );
        })}
      </svg>

      {/* Hover Tooltip Overlay */}
      {hoveredNode && (
        <div
          style={{
            position: 'absolute',
            bottom: '12px',
            right: '16px',
            background: 'rgba(10, 14, 26, 0.95)',
            border: `1px solid ${ROLE_COLORS[hoveredNode.role] ?? '#00c896'}`,
            borderRadius: '6px',
            padding: '8px 12px',
            fontSize: '11px',
            color: '#e8edf5',
            boxShadow: '0 4px 12px rgba(0,0,0,0.5)',
            pointerEvents: 'none'
          }}
        >
          <div style={{ fontWeight: 700, color: ROLE_COLORS[hoveredNode.role] ?? '#00c896' }}>
            Cell {hoveredNode.id} ({hoveredNode.role})
          </div>
          <div>Generation: Gen {hoveredNode.generation}</div>
          <div>Parent: {hoveredNode.parentCellId ?? 'Root'}</div>
          <div>Children: {hoveredNode.childrenIds.length}</div>
        </div>
      )}
    </div>
  );
}
