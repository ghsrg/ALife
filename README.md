---
tags:
  - alife
  - index/root
---

# ALife Simulation

**ALife Simulation** — documentation-first проєкт Artificial Life Engine: симулятора штучного життя, де складні форми поведінки, клітинна спеціалізація, організми та екосистеми мають виникати з універсальних законів світу, а не з жорстко запрограмованих біологічних shortcut-механік.

## Поточний стан

Зараз репозиторій містить документацію вимог, принципів, законів світу, біології, генетики, еволюції, конфігурацій і майбутнього рушія.

Код реалізації ще не є основним джерелом істини. Перед розробкою документація проходить аудит на:

* розбіжності та нестикування;
* сірі зони;
* дублювання;
* відповідність фізичним, хімічним і біологічним обмеженням;
* придатність до базової моделі.

## Ключові принципи

* Документація має пріоритет над майбутнім кодом.
* `docs/PRINCIPLES.md` є верхнім рівнем правил.
* Біологічні структури не задаються напряму: вони мають виникати з фізики, матеріалів, ресурсів, енергії, геному, Joint-зв'язків та добору.
* `Organism`, тканини, органи, навчання й поведінка є емерджентними або аналітичними поняттями, а не окремими hardcoded класами рушія.
* Research-документи не є специфікацією, доки рішення не перенесене в Canon і, за потреби, не зафіксоване ADR.

## Документація

Основний навігатор:

* [docs/README.md](docs/README.md)
* [docs/INDEX.md](docs/INDEX.md) — ієрархічний агентський індекс документації
* [docs/mechanics/INDEX.md](docs/mechanics/INDEX.md) — карта взаємодій для агентського pre-flight перед TDD/реалізацією
* [LICENSE](LICENSE)

Рекомендований маршрут читання:

1. [docs/PRINCIPLES.md](docs/PRINCIPLES.md)
2. [docs/GLOSSARY.md](docs/GLOSSARY.md)
3. [docs/ROADMAP.md](docs/ROADMAP.md)
4. [docs/world/INDEX.md](docs/world/INDEX.md)
5. [docs/biology/INDEX.md](docs/biology/INDEX.md)
6. [docs/genetics/INDEX.md](docs/genetics/INDEX.md)
7. [docs/evolution/INDEX.md](docs/evolution/INDEX.md)
8. [docs/config/INDEX.md](docs/config/INDEX.md)
9. [docs/engine/INDEX.md](docs/engine/INDEX.md)
10. [docs/ui/INDEX.md](docs/ui/INDEX.md)
11. [docs/implementation/INDEX.md](docs/implementation/INDEX.md)
12. `docs/research/`
13. [docs/decisions/INDEX.md](docs/decisions/INDEX.md)

Поточні статуси, пріоритети та етапи розвитку ведуться тільки в [docs/ROADMAP.md](docs/ROADMAP.md).

Службові правила для агентів описані в [[AGENTS|AGENTS.MD]], швидка навігація для агентів зібрана в [[docs/INDEX|docs/INDEX]], а робочі плани й звіти зібрані в [[outputs/worklogs/index|outputs/worklogs]].
