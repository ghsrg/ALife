export const uiText = {
  app: {
    title: 'ALife Control Center',
    eyebrow: 'ALife Control Center',
    primaryViews: 'Primary views'
  },
  workspace: {
    monitor: 'Monitor',
    organismView: 'OrganismView',
    worldEditor: 'World Editor',
    monitorWorkspace: 'Monitor workspace'
  },
  controls: {
    exportPng: 'Export PNG',
    exportViewerPng: 'Export viewer PNG',
    switchToLightTheme: 'Switch to light theme',
    switchToDarkTheme: 'Switch to dark theme'
  },
  layers: {
    title: 'Layers',
    ariaLabel: 'Layer controls',
    cells: 'Cells',
    resources: 'Composite Resource Concentration',
    joints: 'Joints',
    run: 'Run',
    world: 'World',
    projection: 'Projection'
  },
  viewer: {
    ariaLabel: 'World Viewer',
    navigationAriaLabel: 'World Viewer navigation',
    hitTargetsAriaLabel: 'World cell hit targets',
    zoomLabel: 'World Viewer zoom',
    zoomIn: 'Zoom in World Viewer',
    zoomOut: 'Zoom out World Viewer',
    fit: 'Fit World Viewer',
    fitButton: 'Fit',
    reset: 'Reset World Viewer navigation',
    resetButton: 'Reset'
  },
  inspector: {
    title: 'Cell Inspector',
    emptyCell: 'No cell selected.',
    id: 'ID',
    energy: 'Energy',
    integrity: 'Integrity',
    generation: 'Generation',
    roleHint: 'Role hint'
  }
} as const;
