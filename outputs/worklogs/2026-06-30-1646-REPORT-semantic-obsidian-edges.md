---
tags:
  - alife
  - worklog/report
---

# REPORT: Semantic Obsidian edges

## Goal

Замінити штучний центральний граф-хаб на семантичні ребра між поняттями в документації.

## Scope

- Кореневі навігатори: `README.md`, `AGENTS.MD`.
- Навігатор документації: `docs/README.md`.
- Canon, Config, Engine, Genetics, Evolution, Research, Examples і ADR README.
- Worklog index: `outputs/worklogs/README.md`.

## Changes

- Видалено `MAP.md`, бо він створював штучний центр Obsidian-графа.
- Прибрано активні Obsidian-посилання на `MAP`.
- Додано блоки `# Semantic Links` у документах, де ребра описують зміст зв'язку:
  - `Resource -> Cell/Reactions/Materials/Energy`;
  - `Cell -> Resources/Materials/Energy/Membrane/Genome/Joint/Lifecycle`;
  - `Material -> Cell/Boundary/Joint/Process Capabilities/Genome carrier`;
  - `Genome -> Processes/Genome Runtime/Regulatory Interface/Inheritance`;
  - `Joint -> Cells/Materials/Resources/Heat/Communication/Physics`;
  - `Config -> configured Canon entity`;
  - `Engine -> implemented Canon entity`.
- Worklogs лишені окремим службовим індексом, щоб вони не ставали центром предметного графа.

## Verification

- `MAP.md` відсутній.
- Markdown-файлів перевірено: `94`.
- `# Semantic Links` секцій: `71`.
- Битих локальних Markdown/Obsidian-посилань: `0`.
- Граф Markdown-документів лишається зв'язним без `MAP.md`.

## Open Questions

- Немає.
