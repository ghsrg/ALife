---
tags:
  - alife
  - process/agents
---

# AGENTS.MD

## Context

`docs/` is the source of truth for project knowledge and implementation.

Documentation language is Ukrainian. Code identifiers and technical class-like terms may stay in English when already used by the docs.

Main navigation:

- human entry: [[docs/README|Documentation README]]
- agent entry: [[docs/INDEX|Documentation Index]]
- mechanics pre-flight: [[docs/mechanics/INDEX|Mechanics Index]]
- worklogs: [[outputs/worklogs/index|Worklogs]]

## Before Work

Before changing requirements, architecture, implementation plans, or code:

1. Read [[docs/PRINCIPLES|Principles]].
2. Open [[docs/INDEX|Documentation Index]].
3. Follow the relevant local index and mechanics card.
4. Read the Canon, ADR, and implementation sources referenced there.

Read [[docs/ROADMAP|Roadmap]] only for project status, documentation priorities, or phase planning.

If a required rule is missing, do not invent it silently. Record it as an open question, TODO, proposed ADR, or audit finding.

## Worklogs

Do not create both a plan and a report for every request.

- audits/reviews/analysis usually produce a fix plan;
- implementation/fix/refactor work produces a report after verification.

Use:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-PLAN-name.md
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-name.md
```

## Guardrails

- Do not implement behavior that contradicts [[docs/PRINCIPLES|Principles]].
- Before TDD planning for behavior or simulation mechanics, use [[docs/mechanics/INDEX|Mechanics Index]].
- Do not add hardcoded biological shortcuts such as species IDs, cell classes for roles, organs, brains, predators, or scripted behaviors unless an ADR explicitly changes the rules.
- Keep changes scoped to the requested task.
