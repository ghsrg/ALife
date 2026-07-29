import { describe, expect, it } from 'vitest';
import { screen } from '@testing-library/react';
import { renderApp } from '../test/render';
import { createAppStore } from '../app/appState';
import { BottomDataPanel } from './BottomDataPanel';

describe('BottomDataPanel', () => {
  it('renders the V3 analytics surface without data-panel tabs', () => {
    const store = createAppStore();
    const state = store.getState();

    renderApp(<BottomDataPanel state={state} />);

    expect(screen.queryByRole('navigation', { name: 'Data panel tabs' })).not.toBeInTheDocument();
    expect(screen.getByText('RESOURCE CYCLE (ENERGY & MATTER)')).toBeInTheDocument();
    expect(screen.getByText('ENERGY DISTRIBUTION OVER TIME (100% TOTAL)')).toBeInTheDocument();
    expect(screen.getByText('DOMINANT CELL / BEHAVIOR TYPES')).toBeInTheDocument();
    expect(screen.getByText('CELL SIZE DISTRIBUTION (BY RADIUS)')).toBeInTheDocument();
    expect(screen.getByText('LIVE STREAM')).toBeInTheDocument();
  });
});
