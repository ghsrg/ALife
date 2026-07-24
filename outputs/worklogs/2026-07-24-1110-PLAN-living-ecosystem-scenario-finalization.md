# PLAN: Living Ecosystem Scenario Finalization & Balance Synthesis

## Context & Objectives

The user noted:
1. Clarification: `rich_patchy_world.toml` has `scenario_id = "bootstrap_rich_patchy_world"`.
2. Requirement: Create a comprehensive, fully balanced scenario TOML (`living_ecosystem.toml` with `scenario_id = "living_ecosystem"`) synthesizing all mechanics developed across project phases:
   - 4 resource fields: `nutrient_A` (green), `mineral_A` (blue), `energy_A` (gold), `waste_A` (purple).
   - Environmental fields: `temperature` and `pressure`.
   - Genome-driven cells: multi-layer genome execution (`template = "balanced"`), active uptake, synthesis, repair, and division thresholds.
   - Viable energy balance: `initial_energy = 85.0`, `energy_capacity = 120.0`, `mandatory_cost_per_tick = 0.005` to ensure cells live, metabolize, divide, and evolve over thousands of ticks.

## Proposed Changes

### 1. New Scenario: `config/scenarios/demo/living_ecosystem.toml` [NEW]
- `scenario_id = "living_ecosystem"`
- World size: `320.0 x 240.0`
- Resources: `nutrient_A`, `mineral_A`, `energy_A`, `waste_A` with patchy generation.
- Environmental fields: `temperature` (band 18–32°C), `pressure` (gradient 1.0–2.5 atm).
- 16 initial cells in 4 spatial clusters, loaded with `initial_energy = 85.0`, capacity = 120.0, metabolic cost = 0.005.
- Active genome runtime enabled for metabolic synthesis, membrane repair, and division.

### 2. Scenario Update: `config/scenarios/bootstrap/living_patchy_world.toml` & `rich_patchy_world.toml`
- Ensure all bootstrap scenarios use viable energy parameters so cells do not starve on tick 150.

### 3. Verification & UI Alignment
- Verify that `living_ecosystem` appears in the UI scenario dropdown.
- Verify that selecting `living_ecosystem` streams all 4 resource layers dynamically into `FIELD LAYERS` in `LayerPanel.tsx`.

## Verification Plan

- **Vitest & Build:**
  - Run `npx vitest run` (36 test files pass).
  - Run `npm run build` (0 TypeScript / Vite errors).
