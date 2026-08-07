import type { DebugProjectionState } from '../projection/types';

export type DebugLayerMode = 'exact' | 'smooth';

export interface DebugLayerOptions {
  mode: DebugLayerMode;
  showResourceLayer: boolean;
  showFieldLayer: boolean;
}

export interface DebugResourceLayerPlan {
  layerIndex: number;
  totalAmount: number;
  availability: string;
  channelLabel: string;
  colorHex: string;
  legendLabel: string;
}

export interface DebugFieldLayerPlan {
  fieldId: string;
  value: number;
  sourceOwner: string;
  legendLabel: string;
  sampledValueLabel: string;
}

export interface DebugLayerPlan {
  status: 'available' | 'loading' | 'stale' | 'unavailable';
  reason?: string;
  interpolationLabel: 'Exact' | 'Smooth interpolated';
  resources: DebugResourceLayerPlan[];
  fields: DebugFieldLayerPlan[];
  totalResourceLayerCount: number;
  hiddenResourceLayerCount: number;
  missingProjectionWarnings: string[];
}

const DEBUG_RESOURCE_LEGEND_LIMIT = 8;

export function buildDebugLayerPlan(
  debugProjections: DebugProjectionState,
  options: DebugLayerOptions
): DebugLayerPlan {
  const interpolationLabel = options.mode === 'smooth' ? 'Smooth interpolated' : 'Exact';

  if (debugProjections.status !== 'available') {
    return {
      status: debugProjections.status,
      reason: debugProjections.reason,
      interpolationLabel,
      resources: [],
      fields: [],
      totalResourceLayerCount: 0,
      hiddenResourceLayerCount: 0,
      missingProjectionWarnings: []
    };
  }

  const allResources = debugProjections.visualWorld.resourceLayers.map((layer) => ({
    layerIndex: layer.layerIndex,
    totalAmount: layer.totalAmount,
    availability: layer.completeness.state,
    channelLabel: layer.resourceId,
    colorHex: resourceChannelColor(layer.layerIndex),
    legendLabel: `${layer.resourceId} total ${formatAmount(layer.totalAmount)}`
  }));
  const resources = options.showResourceLayer
    ? allResources.slice(0, DEBUG_RESOURCE_LEGEND_LIMIT)
    : [];

  const fields = options.showFieldLayer
    ? debugProjections.visualWorld.fields.map((field) => ({
        fieldId: field.fieldId,
        value: field.value,
        sourceOwner: field.sourceMetric.sourceOwner,
        legendLabel: `${field.fieldId} ${field.value}`,
        sampledValueLabel: `sampled ${field.fieldId}: ${field.value}`
      }))
    : [];

  return {
    status: 'available',
    interpolationLabel,
    resources,
    fields,
    totalResourceLayerCount: options.showResourceLayer ? allResources.length : 0,
    hiddenResourceLayerCount: options.showResourceLayer
      ? Math.max(0, allResources.length - resources.length)
      : 0,
    missingProjectionWarnings: debugProjections.visualWorld.completeness.missingFields
  };
}

function resourceChannelColor(layerIndex: number) {
  const channel = layerIndex % 3;
  if (channel === 0) {
    return '#27b582';
  }
  if (channel === 1) {
    return '#2f80ed';
  }
  return '#ffd166';
}

function formatAmount(value: number) {
  return Number.isInteger(value) ? String(value) : value.toFixed(2);
}
