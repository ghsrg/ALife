---
tags:
  - alife
  - worklog/report
---

# REPORT: Obsidian links and tags

## Goal

Зробити Markdown-документацію зв'язною для Obsidian: жоден `.md` не має лишатися окремим вузлом, а документи мають отримати корисні теги для графа.

## Scope

- Кореневі навігатори: `README.md`, `AGENTS.MD`.
- Основний навігатор документації: `docs/README.md`.
- Усі Markdown-файли в `docs/`.
- Робочі плани й звіти в `outputs/worklogs/`.

## Changes

- Додано кореневий `MAP.md` як Obsidian graph hub для всіх Markdown-документів.
- Додано `outputs/worklogs/README.md` як окремий хаб для планів і звітів.
- Додано посилання з `README.md` на `MAP.md`, `AGENTS.MD`, `outputs/worklogs/README.md` і `LICENSE`.
- Додано посилання з `docs/README.md` на `MAP.md` і worklogs.
- Додано службові Obsidian-посилання в `AGENTS.MD`.
- Додано YAML frontmatter `tags` до Markdown-файлів за типом і областю: `area/world`, `area/biology`, `area/genetics`, `area/evolution`, `area/config`, `area/engine`, `area/research`, `worklog/plan`, `worklog/report` тощо.

## Verification

- Усі Markdown-файли включені в `MAP.md`.
- Усі Markdown-файли мають YAML `tags`.
- Очевидних битих локальних Markdown або Obsidian-посилань не знайдено.

## Open Questions

- Немає.
