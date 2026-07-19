---
tags:
  - alife
  - docs/index
  - audience/agent
---

# Documentation Index

> Головний agent-facing router. Не дублює повний список документів; веде до локальних індексів і mechanics cards. Для людського порядку читання дивись [[docs/README|Docs README]].

## Before Work

1. Read [[docs/PRINCIPLES|Principles]].
2. Use this index to choose the relevant area.
3. Select relevant [[docs/mechanics/INDEX|mechanics cards]].
4. Read the Must Read sources listed in those cards.
5. Check relevant [[docs/decisions/INDEX|ADR]] or [[docs/implementation/INDEX|implementation]] documents.

Read [[docs/ROADMAP|Roadmap]] only for project status, documentation priorities, or phase planning.

## Authority Order

[[docs/PRINCIPLES|Principles]] -> Canon documents -> accepted [[docs/decisions/INDEX|ADR]] -> [[docs/implementation/INDEX|Implementation]] -> [[docs/mechanics/INDEX|Mechanics cards]].

Mechanics cards are routing checklists, not sources of truth.

## Area Indexes

- [[docs/world/INDEX|World Index]] `#world` `#physics` `#matter` `#energy`
- [[docs/biology/INDEX|Biology Index]] `#cell` `#processes` `#lifecycle`
- [[docs/genetics/INDEX|Genetics Index]] `#genome` `#inheritance` `#mutation`
- [[docs/evolution/INDEX|Evolution Index]] `#observer` `#population` `#selection`
- [[docs/observer/INDEX|Observer Index]] `#observer` `#projection` `#coverage`
- [[docs/config/INDEX|Config Index]] `#config` `#bounds`
- [[docs/engine/INDEX|Engine Index]] `#runtime` `#performance` `#storage`
- [[docs/runner/INDEX|Runner Index]] `#runner` `#orchestration` `#bootstrap`
- [[docs/ui/INDEX|UI Index]] `#viewer` `#control-center`
- [[docs/implementation/INDEX|Implementation Index]] `#phases` `#api` `#data-model`
- [[docs/delivery/INDEX|Delivery Index]] `#delivery` `#roadmap` `#status`
- [[docs/decisions/INDEX|Decisions Index]] `#adr`
- [[docs/examples/INDEX|Examples Index]] `#examples`
- [[docs/research/INDEX|Research Index]] `#research`
- [[docs/mechanics/INDEX|Mechanics Index]] `#interaction-map` `#tdd-preflight`

## Common Agent Routes

- Implementing behavior or writing plans -> [[docs/mechanics/INDEX]] -> relevant card -> Must Read sources.
- Changing Canon rule -> [[docs/PRINCIPLES]] -> relevant area index -> Canon document -> ADR check.
- Planning phase work -> [[docs/ROADMAP]] + [[docs/implementation/INDEX]] + relevant mechanics cards.
- Working with configs -> [[docs/config/INDEX]] + [[docs/mechanics/config-to-runtime|Config -> Runtime]].
- Working with runner orchestration, execution modes, Scenario resolution, Bootstrap, commands, or projections -> [[docs/runner/INDEX]] + [[docs/implementation/implementation-plan-runner|Runner Implementation Plan]].
- Running Runner manually from terminal -> [[docs/RUNNER_USAGE|Runner Usage Guide]].
- Working with viewer/UI -> [[docs/ui/INDEX]] + [[docs/implementation/implementation-plan-ui|UI Implementation Plan]].
- Working with observer/projections/coverage -> [[docs/observer/INDEX]] + [[docs/mechanics/observer-projection|Observer Projection]].
- Looking for saved plans/reports -> [[outputs/worklogs/index|Worklogs Index]].
