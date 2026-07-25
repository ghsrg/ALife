export interface ValidationResult {
  isValid: boolean;
  errors: string[];
  warnings: string[];
}

export interface ScenarioPreset {
  id: string;
  name: string;
  description: string;
  defaultToml: string;
}

export const PRESET_SCENARIOS: ScenarioPreset[] = [
  {
    id: 'diverse_rich_world',
    name: 'Diverse Rich World (8 Resource Layers, 5 Cell Archetypes)',
    description: 'Diverse ecosystem with glucose, amino, iron, silica, photons, geothermal heat, waste CO2, and heavy metals.',
    defaultToml: `[world]
size = [160.0, 160.0]
seed = 42

[space]
spatial_grid_size = 10.0
physics_solver_iterations = 2

[environment]
ambient_temperature = 25.0
light_intensity = 1.0

[[resources.layers]]
name = "glucose_nutrient"
initial_amount = 100.0

[[resources.layers]]
name = "amino_building_block"
initial_amount = 80.0

[[resources.layers]]
name = "mineral_catalyst_iron"
initial_amount = 40.0

[cell]
initial_position = [30.0, 30.0]
radius = 1.2
initial_energy = 100.0
energy_capacity = 150.0
mandatory_cost_per_tick = 0.005
capacity_limit = 150.0
`
  },
  {
    id: 'demo_world_resource',
    name: 'Demo World Resource (2 Resource Layers)',
    description: 'Balanced demo scenario with ambient energy and organic nutrients.',
    defaultToml: `[world]
size = [100.0, 100.0]
seed = 1001

[space]
spatial_grid_size = 10.0

[[resources.layers]]
name = "organic_nutrient"
initial_amount = 50.0

[cell]
initial_position = [50.0, 50.0]
radius = 1.3
initial_energy = 80.0
energy_capacity = 100.0
mandatory_cost_per_tick = 0.01
capacity_limit = 100.0
`
  },
  {
    id: 'scale_20k_cells',
    name: 'Scale 20k Cells (Performance Benchmark)',
    description: 'Large-scale stress benchmark with 20,000 cells.',
    defaultToml: `[world]
size = [1000.0, 1000.0]
seed = 20000

[space]
spatial_grid_size = 10.0

[cell]
initial_position = [500.0, 500.0]
radius = 1.0
initial_energy = 50.0
energy_capacity = 100.0
mandatory_cost_per_tick = 0.001
capacity_limit = 100.0
`
  }
];

export function validateScenarioToml(tomlText: string): ValidationResult {
  const errors: string[] = [];
  const warnings: string[] = [];

  if (!tomlText || tomlText.trim().length === 0) {
    errors.push('TOML configuration content cannot be empty.');
    return { isValid: false, errors, warnings };
  }

  // Basic structural validation checks
  if (!tomlText.includes('[world]') && !tomlText.includes('size')) {
    warnings.push('Missing explicit [world] header section.');
  }

  const widthMatch = tomlText.match(/size\s*=\s*\[\s*([\d.]+)\s*,\s*([\d.]+)\s*\]/);
  if (widthMatch) {
    const width = parseFloat(widthMatch[1]);
    const height = parseFloat(widthMatch[2]);
    if (isNaN(width) || width <= 0 || isNaN(height) || height <= 0) {
      errors.push('World size dimensions must be positive numbers.');
    }
  }

  const radiusMatch = tomlText.match(/radius\s*=\s*([\d.]+)/);
  if (radiusMatch) {
    const radius = parseFloat(radiusMatch[1]);
    if (isNaN(radius) || radius <= 0) {
      errors.push('Cell radius must be greater than 0.');
    }
  }

  const energyMatch = tomlText.match(/initial_energy\s*=\s*([-\d.]+)/);
  if (energyMatch) {
    const energy = parseFloat(energyMatch[1]);
    if (isNaN(energy) || energy < 0) {
      errors.push('Initial cell energy cannot be negative.');
    }
  }

  return {
    isValid: errors.length === 0,
    errors,
    warnings
  };
}

export async function computeConfigHash(tomlText: string): Promise<string> {
  if (typeof crypto !== 'undefined' && crypto.subtle) {
    try {
      const encoder = new TextEncoder();
      const data = encoder.encode(tomlText);
      const hashBuffer = await crypto.subtle.digest('SHA-256', data);
      const hashArray = Array.from(new Uint8Array(hashBuffer));
      const hexHash = hashArray.map((b) => b.toString(16).padStart(2, '0')).join('');
      return hexHash.substring(0, 16);
    } catch {
      // Fallback below
    }
  }

  // Fallback simple hash for non-crypto environments
  let hash = 0;
  for (let i = 0; i < tomlText.length; i++) {
    const char = tomlText.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash |= 0;
  }
  return Math.abs(hash).toString(16).padStart(8, '0');
}

const DRAFT_KEY_PREFIX = 'alife_world_editor_draft_';

export function saveDraftToLocalStorage(scenarioId: string, tomlText: string): void {
  if (typeof localStorage !== 'undefined') {
    try {
      localStorage.setItem(`${DRAFT_KEY_PREFIX}${scenarioId}`, tomlText);
    } catch {
      // Ignore storage errors
    }
  }
}

export function loadDraftFromLocalStorage(scenarioId: string): string | null {
  if (typeof localStorage !== 'undefined') {
    try {
      return localStorage.getItem(`${DRAFT_KEY_PREFIX}${scenarioId}`);
    } catch {
      return null;
    }
  }
  return null;
}

export function clearDraftInLocalStorage(scenarioId: string): void {
  if (typeof localStorage !== 'undefined') {
    try {
      localStorage.removeItem(`${DRAFT_KEY_PREFIX}${scenarioId}`);
    } catch {
      // Ignore storage errors
    }
  }
}
