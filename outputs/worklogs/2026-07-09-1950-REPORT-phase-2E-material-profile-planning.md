# REPORT: Phase 2E Material Profile Planning

## Summary

Зафіксовано нову Phase 2E:

```text
Material Profile Coverage, Material Properties And Sweep Analyzer Calibration
```

Поточну `Phase 2E: Local Cell-Cell Interaction Primitives` зміщено в `Phase 2F`.

## Зміни

- Оновлено [[outputs/worklogs/2026-07-02-1855-PLAN-phase-2-global-roadmap]]:
  - додано Phase 2E як повноцінну фазу доробки core для Materials/Resources;
  - перенесено local cell-cell interaction primitives у Phase 2F;
  - додано material-profile reachability gates;
  - додано acceptance gates для Material proportions, Observer labels і sweep_analyzer reports;
  - зафіксовано root `config/` як канонічний корінь runtime/analyzer/observer config artifacts.

- Створено TDD-план для реалізації:
  - [[outputs/worklogs/2026-07-09-1950-PLAN-phase-2E-material-profile-coverage]]

- Оновлено [[outputs/worklogs/index]].

## Важливі рішення

- Python `tools/early-stability` більше не є authority для Phase 2 behavior.
- Phase 2E має право змінювати Rust core, якщо material properties зараз не впливають на mechanics.
- Для `repair`, `contractile`, `sensory` у Phase 2E дозволені мінімальні placeholder-effects, але без hardcoded behavior або command semantics.
- Material profile scenarios мають бути 10+ Cells, але повноцінний interactive local interaction залишається на Phase 2F.

## Verification

Проведено документаційний update. Код не змінювався і тести не запускались.
