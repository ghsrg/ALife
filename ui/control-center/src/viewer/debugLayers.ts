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
  status: 'available' | 'unavailable';
  reason?: string;
  interpolationLabel: 'Exact' | 'Smooth interpolated';
  resources: DebugResourceLayerPlan[];
  fields: DebugFieldLayerPlan[];
  missingProjectionWarnings: string[];
}

export function buildDebugLayerPlan(
  debugProjections: DebugProjectionState,
  options: DebugLayerOptions
): DebugLayerPlan {
  const interpolationLabel = options.mode === 'smooth' ? 'Smooth interpolated' : 'Exact';

  if (debugProjections.status === 'unavailable') {
    return {
      status: 'unavailable',
      reason: debugProjections.reason,
      interpolationLabel,
      resources: [],
      fields: [],
      missingProjectionWarnings: []
    };
  }

  const resources = options.showResourceLayer
    ? debugProjections.visualWorld.resourceLayers.map((layer) => ({
        layerIndex: layer.layerIndex,
        totalAmount: layer.totalAmount,
        availability: layer.completeness.state,
        channelLabel: resourceChannelLabel(layer.layerIndex),
        colorHex: resourceChannelColor(layer.layerIndex),
        legendLabel: `Layer ${layer.layerIndex} ${resourceChannelLabel(layer.layerIndex)} total ${formatAmount(layer.totalAmount)}`
      }))
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
    missingProjectionWarnings: debugProjections.visualWorld.completeness.missingFields
  };
}

function resourceChannelLabel(layerIndex: number) {
  const channel = layerIndex % 3;
  if (channel === 0) {
    return 'green channel';
  }
  if (channel === 1) {
    return 'blue channel';
  }
  return 'amber channel';
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
