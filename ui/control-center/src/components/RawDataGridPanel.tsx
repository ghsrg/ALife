import { useMemo, useState } from 'react';
import type { CellId, WorldFrame } from '../projection/types';
import { lifecycleVisualState } from '../viewer/semanticDetail';

interface RawDataGridPanelProps {
  frame: WorldFrame | null;
  onSelectCell: (cellId: CellId) => void;
}

export function RawDataGridPanel({ frame, onSelectCell }: RawDataGridPanelProps) {
  const [filterQuery, setFilterQuery] = useState('');
  const [sortField, setSortField] = useState<'id' | 'energy' | 'integrity'>('id');
  const [sortAsc, setSortAsc] = useState(true);

  const filteredCells = useMemo(() => {
    if (!frame) return [];
    const q = filterQuery.toLowerCase();
    return frame.cells
      .filter((cell) => {
        const stateStr = lifecycleVisualState(cell.lifecycle).toLowerCase();
        return cell.id.toLowerCase().includes(q) || stateStr.includes(q) || cell.roleHint.toLowerCase().includes(q);
      })
      .sort((a, b) => {
        const valA = a[sortField] ?? 0;
        const valB = b[sortField] ?? 0;
        if (valA < valB) return sortAsc ? -1 : 1;
        if (valA > valB) return sortAsc ? 1 : -1;
        return 0;
      });
  }, [frame, filterQuery, sortField, sortAsc]);

  const toggleSort = (field: 'id' | 'energy' | 'integrity') => {
    if (sortField === field) {
      setSortAsc(!sortAsc);
    } else {
      setSortField(field);
      setSortAsc(true);
    }
  };

  const handleExportCsv = () => {
    if (!frame) return;
    const headers = ['CellID', 'X', 'Y', 'Radius', 'Energy', 'Integrity', 'Role', 'State'];
    const rows = frame.cells.map((c) => [
      c.id,
      c.x.toFixed(2),
      c.y.toFixed(2),
      c.radius.toFixed(2),
      c.energy.toFixed(2),
      c.integrity.toFixed(2),
      c.roleHint,
      lifecycleVisualState(c.lifecycle)
    ]);
    const csvContent =
      'data:text/csv;charset=utf-8,' +
      [headers.join(','), ...rows.map((e) => e.join(','))].join('\n');
    const encodedUri = encodeURI(csvContent);
    const link = document.createElement('a');
    link.setAttribute('href', encodedUri);
    link.setAttribute('download', `telemetry_raw_tick_${frame.tick}.csv`);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  if (!frame) {
    return (
      <div className="raw-data-panel empty" data-testid="raw-data-empty">
        <p>No telemetry frame available.</p>
      </div>
    );
  }

  return (
    <div className="raw-data-panel" data-testid="raw-data-panel">
      <header className="raw-data-controls">
        <input
          type="text"
          className="search-input"
          placeholder="Filter entities..."
          value={filterQuery}
          onChange={(e) => setFilterQuery(e.target.value)}
        />
        <button type="button" className="export-btn" onClick={handleExportCsv}>
          Export CSV
        </button>
      </header>
      <div className="table-wrapper">
        <table className="raw-data-table">
          <thead>
            <tr>
              <th onClick={() => toggleSort('id')}>Cell ID</th>
              <th>Position</th>
              <th>Radius</th>
              <th onClick={() => toggleSort('energy')}>Energy</th>
              <th onClick={() => toggleSort('integrity')}>Integrity</th>
              <th>State</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {filteredCells.map((cell) => {
              const stateStr = lifecycleVisualState(cell.lifecycle);
              return (
                <tr key={cell.id}>
                  <td>#{cell.id}</td>
                  <td>{`(${cell.x.toFixed(1)}, ${cell.y.toFixed(1)})`}</td>
                  <td>{cell.radius.toFixed(2)}</td>
                  <td>{cell.energy.toFixed(2)}</td>
                  <td>{(cell.integrity * 100).toFixed(0)}%</td>
                  <td>
                    <span className={`state-badge ${stateStr}`}>{stateStr}</span>
                  </td>
                  <td>
                    <button
                      type="button"
                      className="action-btn"
                      onClick={() => onSelectCell(cell.id)}
                      aria-label={`Show #${cell.id} in Viewer`}
                    >
                      Viewer
                    </button>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}
