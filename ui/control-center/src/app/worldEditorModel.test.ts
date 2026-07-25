import { describe, expect, it } from 'vitest';
import {
  PRESET_SCENARIOS,
  computeConfigHash,
  validateScenarioToml
} from './worldEditorModel';

describe('worldEditorModel', () => {
  it('validates valid scenario TOML correctly', () => {
    const validToml = `[world]
size = [100.0, 100.0]
seed = 42

[cell]
radius = 1.2
initial_energy = 80.0`;

    const result = validateScenarioToml(validToml);
    expect(result.isValid).toBe(true);
    expect(result.errors).toHaveLength(0);
  });

  it('catches empty TOML content error', () => {
    const result = validateScenarioToml('   ');
    expect(result.isValid).toBe(false);
    expect(result.errors).toContain('TOML configuration content cannot be empty.');
  });

  it('detects negative energy validation error', () => {
    const invalidToml = `[world]
size = [100.0, 100.0]
[cell]
initial_energy = -10.0`;

    const result = validateScenarioToml(invalidToml);
    expect(result.isValid).toBe(false);
    expect(result.errors).toContain('Initial cell energy cannot be negative.');
  });

  it('computes deterministic config hash', async () => {
    const toml = '[world]\nsize = [100.0, 100.0]';
    const hash1 = await computeConfigHash(toml);
    const hash2 = await computeConfigHash(toml);
    expect(hash1).toBe(hash2);
    expect(hash1.length).toBeGreaterThan(0);
  });

  it('provides default scenario presets', () => {
    expect(PRESET_SCENARIOS.length).toBeGreaterThanOrEqual(3);
    expect(PRESET_SCENARIOS[0].id).toBe('diverse_rich_world');
  });
});
