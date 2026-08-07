import type { DebugField, DebugResourceLayer } from '../projection/types';

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

export function buildFieldLayerDisplay(field: DebugField): LayerDisplayRow {
  const key = field.fieldId.split('.').at(-1) ?? field.fieldId;
  const primaryLabel = FIELD_LABELS[key.toLowerCase()] ?? titleCase(key);

  return {
    primaryLabel,
    secondaryLabel: formatFieldValue(field.value),
    provenance: `${field.sourceMetric.sourceOwner}: ${field.sourceMetric.sourcePath}`
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
