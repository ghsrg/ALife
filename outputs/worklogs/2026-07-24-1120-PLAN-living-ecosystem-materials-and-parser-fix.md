# PLAN: Living Ecosystem Material Diversity & TOML Parser Fix

## Context & Objectives

1. **Fix Scenario Parsing Error:** Add required `[environment]` and `[lifecycle]` sections to `living_ecosystem.toml` to fix `TomlError: missing field environment`.
2. **Material Diversity & Trade-off Biases:** Expand material definitions to include 3 material variants per functional role with trade-off biases ("з перекосами"):
   - `boundary`: `boundary_high_integrity`, `boundary_balanced`, `boundary_high_flux`.
   - `transport`: `transport_rapid`, `transport_balanced`, `transport_durable`.
   - `metabolic`: `metabolic_high_efficiency`, `metabolic_balanced`, `metabolic_cool`.
   - `structural`: `structural_heavy`, `structural_standard`, `structural_light`.
   - `catalyst`: `catalyst_fast`, `catalyst_standard`, `catalyst_precise`.
   - `carrier`: `genome_carrier_A`, `genome_carrier_B`.
3. **Complete Simulation Mechanics:** Include `[growth]`, `[division]`, `[genome_copying]`, `[decomposition]`, `[local_interaction]`, `[joints]`, and genome templates (`balanced`, `high_efficiency`, `rapid_replicator`).

## Proposed Changes

### `config/scenarios/demo`

#### [MODIFY] [living_ecosystem.toml](file:///c:/Users/korsr/PycharmProjects/ALife/config/scenarios/demo/living_ecosystem.toml)
- Add required `[environment]` and `[lifecycle]` blocks.
- Add 15+ material definitions with trade-off biases across boundary, transport, metabolic, structural, and catalyst roles.
- Add genome templates and active replication/division parameters.

## Verification Plan

- Run Rust scenario loader test or sweep parser to verify `living_ecosystem.toml` parses with zero errors.
- Run `npm run build` and `npx vitest run`.
