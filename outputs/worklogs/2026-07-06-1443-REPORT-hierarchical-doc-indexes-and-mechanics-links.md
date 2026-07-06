---
tags:
  - alife
  - worklog/report
  - docs/navigation
  - mechanics/index
---

# REPORT: Hierarchical Doc Indexes And Mechanics Links

## Goal

Connect `docs/mechanics/` into agent navigation and replace the flat root `docs/INDEX.md` with a hierarchical router.

## Scope

- Created local `INDEX.md` files for `world`, `biology`, `genetics`, `evolution`, `config`, `engine`, `implementation`, `ui`, `decisions`, and `examples`.
- Updated `docs/INDEX.md` to route to local indexes and `docs/mechanics/INDEX.md` instead of listing every document.
- Updated `AGENTS.MD` with the new before-work flow and authority order.
- Linked mechanics pre-flight from `docs/README.md`, `docs/implementation/README.md`, and root `README.md`.

## Decisions

- `docs/mechanics/` remains separate from `docs/implementation/` because mechanics cards are routing checklists for Canon interactions, not implementation authority.
- `docs/ROADMAP.md` is no longer required reading for every task; it is used for status, documentation priorities, or phase planning.
- Mechanics cards must not introduce new rules; they point to Canon, ADR, and implementation documents.

## Verification

- Broken docs/root wiki links: 0.
- Checked for accidental literal newline markers in updated navigation files.
- Full worklog wiki-link audit still has old historical placeholder links in earlier worklogs; this change did not modify those archived contents.

## Files Changed

- [[AGENTS]]
- [[README]]
- [[docs/README]]
- [[docs/INDEX]]
- [[docs/world/INDEX]]
- [[docs/biology/INDEX]]
- [[docs/genetics/INDEX]]
- [[docs/evolution/INDEX]]
- [[docs/config/INDEX]]
- [[docs/engine/INDEX]]
- [[docs/implementation/INDEX]]
- [[docs/implementation/README]]
- [[docs/ui/INDEX]]
- [[docs/decisions/INDEX]]
- [[docs/examples/INDEX]]

## Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
