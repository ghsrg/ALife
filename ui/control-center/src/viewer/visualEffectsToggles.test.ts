import { describe, expect, it } from 'vitest';
import { createAppStore, DEFAULT_VISUAL_EFFECTS } from '../app/appState';
import type { WorldFrame } from '../projection/types';
import { createWorldRenderPlan } from './worldRenderPlan';

describe('Visual Effects Toggles & Default Resource Layers', () => {
  it('initializes activeResourceLayers to [0, 1] by default', () => {
    const store = createAppStore();
    expect(store.getState().activeResourceLayers).toEqual([0, 1]);
  });

  it('initializes visualEffects with DEFAULT_VISUAL_EFFECTS', () => {
    const store = createAppStore();
    expect(store.getState().visualEffects).toEqual(DEFAULT_VISUAL_EFFECTS);
  });

  it('initializes visualEffects disabled by default', () => {
    const store = createAppStore();
    expect(Object.values(store.getState().visualEffects).every((value) => value === false)).toBe(true);
  });

  it('toggles visualEffects when toggleVisualEffect action is called', () => {
    const store = createAppStore();
    expect(store.getState().visualEffects.showNebula).toBe(false);

    store.getState().toggleVisualEffect('showNebula');
    expect(store.getState().visualEffects.showNebula).toBe(true);

    store.getState().toggleVisualEffect('showNebula');
    expect(store.getState().visualEffects.showNebula).toBe(false);
  });

  it('toggles all visual effects as a group', () => {
    const store = createAppStore();

    store.getState().setVisualEffectsEnabled(true);
    expect(Object.values(store.getState().visualEffects).every((value) => value === true)).toBe(true);

    store.getState().setVisualEffectsEnabled(false);
    expect(Object.values(store.getState().visualEffects).every((value) => value === false)).toBe(true);
  });

  it('passes visualEffects to createWorldRenderPlan', () => {
    const frame: WorldFrame = {
      schemaVersion: 'WorldFrameProjection/v1',
      runId: 'test',
      tick: 1,
      world: { width: 100, height: 100 },
      resources: [],
      cells: []
    };

    const visualEffects = { ...DEFAULT_VISUAL_EFFECTS, showNebula: true };
    const plan = createWorldRenderPlan(frame, null, { width: 800, height: 600 }, undefined, visualEffects);

    expect(plan.visualEffects?.showNebula).toBe(true);
  });
});
