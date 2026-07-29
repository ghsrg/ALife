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
    monitorWorkspace: 'Monitor workspace',
    unavailable: 'Unavailable',
    organismViewUnavailable: 'OrganismView unavailable - observer projection not ready',
    worldEditorUnavailable: 'World Editor unavailable - debug scenario editor not ready',
    unavailableSummary: 'Unavailable workspaces stay visible and disabled'
  },
  controls: {
    exportPng: 'Export PNG',
    exportViewerPng: 'Export viewer PNG',
    startScreenshotReady: 'Start screenshot PNG ready',
    startScreenshotUnavailable: 'Start screenshot PNG export unavailable',
    fullScreen: 'Full screen',
    exitFullScreen: 'Exit full screen',
    enterStartFullScreen: 'Enter Start full screen',
    switchToLightTheme: 'Switch to light theme',
    switchToDarkTheme: 'Switch to dark theme'
  },
  demo: {
    startDemo: 'Start demo',
    fixtureProjectionSource: 'Projection source: fixture',
    liveProjectionSource: 'Projection source: live',
    runnerDataPrefix: 'Runner data',
    unavailableLiveFields: 'Unavailable live fields stay unavailable'
  },
  dataContext: {
    title: 'Data Context',
    freezeFrame: 'Freeze current frame',
    jumpToLive: 'Jump to Live',
    boundedHistory: 'Bounded history',
    inspectTick: 'Inspect Tick'
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
    fitButton: 'Fit'
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
