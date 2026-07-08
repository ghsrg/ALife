---
tags:
  - alife
  - observer
  - audience/human
---

# Observer Documentation

`docs/observer/` описує read-only шар спостереження над симуляцією.

Observer потрібен для трьох задач:

- перетворити committed state Core у безпечні projection для UI, звітів і debug;
- рахувати observer-only аналітику: lineage, OrganismView, population metrics, selection interpretation;
- перевіряти, що нові механіки Core мають сценарії, метрики й coverage у stability/reachability інструментах.

Observer не є частиною поведінки клітини. Він не змінює `WorldState`, не впливає на Genome Runtime, Feasibility, Scheduler, selection або lifecycle.

## Як читати

1. [[docs/observer/observer-layer|Observer Layer]] — базові вимоги, межі відповідальності й authority contract.
2. [[docs/observer/mechanism-coverage|Mechanism Coverage Contract]] — як analyzer має бачити зареєстровані механіки та їх покриття.
3. [[docs/observer/behavior-profile-balance|Behavior Profile Balance]] — як з coverage і метрик отримувати висновки про баланс survival styles.
4. [[docs/observer/classification-contract|Classification Contract]] — загальний контракт observer-side labels, evidence, confidence, windows, provenance і UI/storage rules.
5. [[docs/observer/classification-registry|Classification Registry]] — стартовий registry Functional Roles, Survival Profiles, Organization Archetypes та інших labels.
6. [[docs/observer/projection-contract|Projection Contract]] — шаблон для майбутніх UI/debug/analytics projection.
7. [[docs/mechanics/observer-projection|Committed State -> Observer Projection]] — коротка pre-flight card для агентів.

## Semantic Links

- governed by: [[docs/PRINCIPLES|Principles]]
- routed by: [[docs/observer/INDEX|Observer Index]]
- mechanics card: [[docs/mechanics/observer-projection|Observer Projection]]
- compares behavior: [[docs/observer/behavior-profile-balance|Behavior Profile Balance]]
- classifies observed patterns: [[docs/observer/classification-contract|Classification Contract]]
- registers observer labels: [[docs/observer/classification-registry|Classification Registry]]
- feeds UI: [[docs/ui/INDEX|UI Index]]
- supports implementation: [[docs/implementation/INDEX|Implementation Index]]
- analyzes evolution: [[docs/evolution/INDEX|Evolution Index]]
