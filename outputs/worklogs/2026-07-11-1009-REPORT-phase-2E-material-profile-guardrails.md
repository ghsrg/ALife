---
tags:
  - alife
  - worklog/report
  - phase/2E
  - material-profiles
---

# Phase 2E Material Profile Guardrails Report

## Статус

Phase 2E material profile coverage залишено у мінімальному, але достатньому для gate стані:

- кожен основний профіль має measurable signal;
- додані low/no negative controls для ключових профілів;
- доданий один tradeoff probe без автоматичного ranking;
- repair і boundary позначені як placeholder/tool-limited, не як повний механізм;
- Phase 2E не претендує на повний material-specific balance.

## Що змінено

- `sweep_analyzer` тепер пише у `material_profile_summary.csv` окремі рядки `material_profile_negative_controls`:
  - `transport_low`, `no_transport`;
  - `metabolic_low`, `no_metabolic`;
  - `storage_low`, `no_storage`;
  - `structural_low`, `no_structural`;
  - `contractile_low`, `no_contractile`;
  - `sensory_low`, `no_sensory`.
- Додано `material_profile_tradeoff_probe` для:
  - `metabolic_rich`: більше `energy_produced`, але більше `heat_generated`/`waste_generated`;
  - `storage_rich`: більше `capacity_free`, але нижчий throughput/energy;
  - `structural_rich`: більше `growth_executed`, але більший `capacity_used`.
- Warning coverage у raw output тепер містить:
  - `SCENARIO_TOO_EASY`;
  - `SCENARIO_TOO_HARD`;
  - `PROFILE_EFFECT_FLAT`;
  - `PROFILE_EFFECT_TOO_SMALL`.
- `material_profile_coverage.csv` для `boundary` і `repair` має статус:
  - `covered_as_placeholder|tool_limited|not_full_mechanism`.

## Output

- `outputs/raw_data/material_profile_summary.csv`
- `outputs/raw_data/material_profile_coverage.csv`
- `outputs/reports/material-profile-coverage-1783753723.md`

## Verification

- `cargo test --test phase2_material_profile_analyzer -- --nocapture`
- `cargo fmt --check`
- `cargo test --test phase2_material_profile_analyzer`
- `cargo test --test phase2_material_profile_effects`
- `cargo test --test phase2_material_profile_gating`
- `cargo run --bin sweep_analyzer -- config/analyzer/material_profile_sweeps.toml`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`

Усі перевірки пройшли.

## Phase Gate

Phase 2E можна вважати закритою для переходу далі:

- матеріальні профілі є mechanism-measurable;
- negative controls захищають від fake PASS через сам сценарій;
- tradeoff probe не дає трактувати профілі як “чим більше, тим краще”;
- repair/boundary не блокують Phase 2E, але явно лишаються `tool_limited`/`not_full_mechanism`.
