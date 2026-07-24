import { describe, expect, it } from 'vitest';
import { screen } from '@testing-library/react';
import { renderApp } from '../test/render';
import { createAppStore } from '../app/appState';
import { BottomDataPanel } from './BottomDataPanel';

describe('BottomDataPanel', () => {
  it('renders V3 4-card analytics panel with tabs and N/A badges', () => {
    const store = createAppStore();
    const state = store.getState();

    renderApp(<BottomDataPanel state={state} />);

    expect(screen.getByRole('navigation', { name: 'Data panel tabs' })).toBeInTheDocument();
    expect(screen.getByText('TIMELINE')).toBeInTheDocument();
    expect(screen.getByText('RESOURCE CYCLE (ENERGY & MATTER)')).toBeInTheDocument();
    expect(screen.getByText('ENERGY DISTRIBUTION OVER TIME (100% TOTAL)')).toBeInTheDocument();
    expect(screen.getByText('DOMINANT CELL / BEHAVIOR TYPES')).toBeInTheDocument();
    expect(screen.getByText('CELL SIZE DISTRIBUTION (BY RADIUS)')).toBeInTheDocument();
    expect(screen.getByText('LIVE STREAM')).toBeInTheDocument();
  });
});
