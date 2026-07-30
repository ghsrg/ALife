import type {
  DebugBalanceFindingProjection,
  DebugClassificationProjection,
  DebugCoverageProjection,
  DebugProjectionCompleteness,
  DebugProjectionSourceMetric,
  DebugProjectionState,
  DebugVisualWorldProjection,
  DebugWarningProjection
} from './types';
import { normalizeMonitorProjection } from './monitorProjectionAdapter';

interface WireCompleteness {
  state: DebugProjectionCompleteness['state'];
  missing_fields?: string[];
  reason?: string | null;
}

interface WireSourceMetric {
  field_id: string;
  source_owner: string;
  source_path: string;
}

function completeness(value: WireCompleteness): DebugProjectionCompleteness {
  return {
    state: value.state,
    missingFields: value.missing_fields ?? [],
    reason: value.reason ?? null
  };
}

function sourceMetric(value: WireSourceMetric): DebugProjectionSourceMetric {
  return {
    fieldId: value.field_id,
    sourceOwner: value.source_owner,
    sourcePath: value.source_path
  };
}

export function normalizeDebugProjectionBundle(value: any): DebugProjectionState {
  const visualWorld = normalizeVisualWorld(value.visual_world);

  return {
    status: 'available',
    runId: value.run_id,
    tick: value.tick,
    ...(value.monitor ? { monitor: normalizeMonitorProjection(value.monitor) } : {}),
    visualWorld,
    coverage: normalizeCoverage(value.coverage),
    warnings: normalizeWarnings(value.warnings),
    classifications: normalizeClassifications(value.classifications),
    balanceFindings: normalizeBalanceFindings(value.balance_findings)
  };
}

export function normalizeUnavailableDebugProjection(value: any): DebugProjectionState {
  return {
    status: 'unavailable',
    reason: value.message ?? 'Projection unavailable'
  };
}

function normalizeVisualWorld(value: any): DebugVisualWorldProjection {
  return {
    projectionKind: 'VisualWorldProjection',
    completeness: completeness(value.completeness),
    cells: (value.payload?.cells ?? []).map((cell: any) => ({
      id: String(cell.id),
      x: cell.x,
      y: cell.y,
      radius: cell.radius,
      energy: cell.energy,
      energyCapacity: cell.energy_capacity ?? cell.energy,
      lifecycleState: cell.lifecycle_state,
      materials: (cell.materials ?? []).map((material: any) => ({
        materialTypeId: material.material_type_id,
        amount: material.amount
      })),
      internalResources: (cell.internal_resources ?? []).map((resource: any) => ({
        resourceTypeId: resource.resource_type_id,
        amount: resource.amount
      })),
      localExternalResources: (cell.local_external_resources ?? []).map((resource: any) => ({
        resourceTypeId: resource.resource_type_id,
        amount: resource.amount
      }))
    })),
    resourceLayers: (value.payload?.resource_layers ?? []).map((layer: any) => ({
      layerIndex: layer.layer_index,
      width: layer.width ?? 0,
      height: layer.height ?? 0,
      totalAmount: layer.total_amount,
      cells: (layer.cells ?? []).map((cell: any) => ({
        x: cell.x,
        y: cell.y,
        amount: cell.amount
      })),
      completeness: completeness(layer.completeness)
    })),
    fields: (value.payload?.fields ?? []).map((field: any) => ({
      fieldId: field.field_id,
      value: field.value,
      sourceMetric: sourceMetric(field.source_metric)
    })),
    sourceMetrics: (value.payload?.source_metrics ?? []).map(sourceMetric)
  };
}

function normalizeCoverage(value: any): DebugCoverageProjection {
  return {
    projectionKind: 'CoverageProjection',
    completeness: completeness(value.completeness),
    mechanisms: (value.payload?.mechanisms ?? []).map((mechanism: any) => ({
      mechanismId: mechanism.mechanism_id,
      statusId: mechanism.status_id,
      sourceReport: mechanism.source_report
    }))
  };
}

function normalizeWarnings(value: any): DebugWarningProjection {
  return {
    projectionKind: 'WarningProjection',
    completeness: completeness(value.completeness),
    warnings: (value.payload?.warnings ?? []).map((warning: any) => ({
      code: warning.code,
      disposition: warning.disposition,
      affectedScope: warning.affected_scope,
      sourceReport: warning.source_report,
      recommendedReruns: warning.recommended_reruns ?? []
    }))
  };
}

function normalizeClassifications(value: any): DebugClassificationProjection {
  return {
    projectionKind: 'ClassificationProjection',
    completeness: completeness(value.completeness),
    classifications: value.payload?.classifications ?? []
  };
}

function normalizeBalanceFindings(value: any): DebugBalanceFindingProjection {
  return {
    projectionKind: 'BalanceFindingProjection',
    completeness: completeness(value.completeness),
    findings: value.payload?.findings ?? []
  };
}
