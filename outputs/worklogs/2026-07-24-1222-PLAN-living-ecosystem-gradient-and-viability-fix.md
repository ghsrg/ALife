# PLAN: Living Ecosystem Gradient Enhancement & Long-Term Viability Fix

## Context & User Feedback

1. **Mass Extinction at Ticks 4000–5000:** Drained nutrient pool without ambient regeneration caused total population starvation and heat/waste death.
2. **Lack of Spatial Gradients (Uniform Saturation):** High diffusion frequency (`resource_diffusion_ticks = 2`) rapidly blurred patchy resource oases into uniform background noise.

## Proposed Changes

### `config/scenarios/demo/living_ecosystem.toml`

1. **Spatial Gradients & Visual Contrast:**
   - Lower diffusion rates (`diffusion_rate = 0.005` for `nutrient_A`, `0.002` for `mineral_A`) and set `resource_diffusion_ticks = 30` to preserve sharp, visually striking patchy oases and gradients.
   - Set high contrast patch parameters in `bootstrap.resources`: `min_amount = 0.0`, `max_amount = 45.0`, `falloff = 0.65`.

2. **Long-Term Ecosystem Viability (>100,000 Ticks):**
   - Add ambient solar/environmental energy inflow: `passive_energy_income_placeholder = 0.02`.
   - Add ecological nutrient recycling reaction: `waste_A` + ambient heat -> `nutrient_A` at low rate (`0.002`), closing the matter cycle.
   - Adjust cell mandatory cost to `0.002` to allow natural selection around resource hotspots.

## Verification Plan

- Run `npm run build` and `npx vitest run`.
- Verify scenario parsing and field layer visual contrast.
