import type { DebugField, DebugFieldLayer, DebugResourceLayer } from '../projection/types';

export interface LayerDisplayRow {
  primaryLabel: string;
  secondaryLabel: string;
  provenance: string;
}

const FIELD_LABELS: Record<string, string> = {
  heat: 'Heat',
  waste: 'Waste',
  energy: 'Energy',
  organic: 'Organic',
  mineral: 'Mineral'
};

export function buildFieldLayerDisplay(field: DebugField | DebugFieldLayer): LayerDisplayRow {
  const key = field.fieldId.split('.').at(-1) ?? field.fieldId;
  const primaryLabel = FIELD_LABELS[key.toLowerCase()] ?? titleCase(key);
  const value = 'summaryValue' in field ? field.summaryValue : field.value;
  const provenance =
    'sourceMetric' in field
      ? `${field.sourceMetric.sourceOwner}: ${field.sourceMetric.sourcePath}`
      : field.completeness.reason
        ? `VisualWorldProjection.field_layers.${field.fieldId} | ${field.completeness.state}: ${field.completeness.reason}`
        : `VisualWorldProjection.field_layers.${field.fieldId} | ${field.completeness.state}`;

  return {
    primaryLabel,
    secondaryLabel: formatFieldValue(value),
    provenance
  };
}

export function buildResourceLayerDisplay(layer: DebugResourceLayer): LayerDisplayRow {
  const identity = `Layer ${layer.layerIndex} | resource_type_id ${layer.resourceTypeId}`;
  return {
    primaryLabel: layer.resourceId,
    secondaryLabel: `${formatAmount(layer.totalAmount)} total · ${layer.completeness.state}`,
    provenance: layer.completeness.reason
      ? `${identity} | ${layer.completeness.state}: ${layer.completeness.reason}`
      : `${identity} | ${layer.completeness.state}`
  };
}

function formatFieldValue(value: number) {
  return Number.isFinite(value) ? formatAmount(value) : 'unavailable';
}

function formatAmount(value: number) {
  return new Intl.NumberFormat('en-US', { maximumFractionDigits: 2 }).format(value);
}

function titleCase(value: string) {
  return value
    .replace(/[_-]+/g, ' ')
    .replace(/\b\w/g, (match) => match.toUpperCase());
}
