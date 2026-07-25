import type { WorldFrame } from '../projection/types';

export interface RoleSpecializationGroup {
  role: string;
  count: number;
  percentage: number;
  avgEnergy: number;
  confidenceScore: number;
  provenance: string;
}

export interface SpecializationSummary {
  totalCells: number;
  dominantRole: string;
  specializationIndex: number;
  overallConfidence: number;
  roles: RoleSpecializationGroup[];
}

export function extractSpecializationSummary(frame: WorldFrame): SpecializationSummary {
  const cells = frame.cells;
  const totalCells = cells.length;

  if (totalCells === 0) {
    return {
      totalCells: 0,
      dominantRole: 'none',
      specializationIndex: 0,
      overallConfidence: 1.0,
      roles: []
    };
  }

  const roleMap = new Map<string, { count: number; totalEnergy: number }>();

  cells.forEach((cell) => {
    const role = cell.roleHint || 'unspecified';
    const current = roleMap.get(role) ?? { count: 0, totalEnergy: 0 };
    current.count += 1;
    current.totalEnergy += cell.energy;
    roleMap.set(role, current);
  });

  const roles: RoleSpecializationGroup[] = [];
  let maxCount = 0;
  let dominantRole = 'unspecified';
  let specializationIndex = 0; // HHI = sum((count/totalCells)^2)

  roleMap.forEach((val, role) => {
    const p = val.count / totalCells;
    specializationIndex += p * p;

    if (val.count > maxCount) {
      maxCount = val.count;
      dominantRole = role;
    }

    const confidenceScore = role === 'unspecified' ? 0.6 : 0.95;
    const provenance = role === 'unspecified' ? 'Observer unclassified' : 'Observer heuristic / v1.0';

    roles.push({
      role,
      count: val.count,
      percentage: p * 100,
      avgEnergy: val.totalEnergy / val.count,
      confidenceScore,
      provenance
    });
  });

  roles.sort((a, b) => b.count - a.count);

  const overallConfidence =
    roles.reduce((acc, r) => acc + r.confidenceScore * (r.count / totalCells), 0) || 1.0;

  return {
    totalCells,
    dominantRole,
    specializationIndex,
    overallConfidence,
    roles
  };
}
