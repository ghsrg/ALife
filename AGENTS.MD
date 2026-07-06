---
tags:
  - alife
  - process/agents
---

# AGENTS.MD

## Context

`docs/` is the source of truth for project knowledge and future implementation.

Documentation language is Ukrainian. Code identifiers and technical class-like terms may stay in English when already used by the docs.

Use [[docs/README|Documentation README]] for human reading order.

Use [[docs/INDEX|Documentation Index]] to locate relevant knowledge.

Use [[outputs/worklogs/index|Worklogs]] for saved plans and reports.

## Before Work

Before changing requirements, architecture, implementation plans, or future code:

1. Read [[docs/PRINCIPLES|Principles]].
2. Open [[docs/INDEX|Documentation Index]].
3. Select relevant [[docs/mechanics/INDEX|mechanics cards]] when the task touches behavior, accounting, runtime interactions, configs, or TDD planning.
4. Read the `Must Read` sources listed in those mechanics cards.
5. Read relevant Canon documents under `docs/`.
6. Check relevant [[docs/decisions/INDEX|ADR]] or [[docs/implementation/INDEX|implementation]] documents.

Read [[docs/ROADMAP|Roadmap]] only for project status, documentation priorities, or phase planning.

If a required rule is missing, do not invent it silently. Record it as an open question, TODO, proposed ADR, or audit finding.

## Authority

Authority order:

```text
Principles
  -> Canon documents
  -> accepted ADR
  -> Implementation documents
  -> Mechanics cards
  -> Research / Examples / Worklogs
```

Mechanics cards are routing checklists, not sources of truth. They help agents find Canon constraints and TDD checks without rereading the entire repository.

Implementation documents describe how accepted rules are planned or encoded. They do not override Canon or accepted ADR.

Research and Examples are non-authoritative until promoted into Canon or ADR.

## Work Process

Do not create both a plan and a report for every request.

When the user asks for an audit, review, validation, or analysis:

- inspect the relevant documentation or code;
- identify issues, contradictions, risks, and open questions;
- produce a plan for fixing them;
- do not modify Canon documents or code unless explicitly asked.

Save audit/fix plans when useful or requested:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-PLAN-name.md
```

When the user says to fix, implement, refactor, or otherwise apply the plan:

- make the changes;
- verify them;
- then create a report.

Save implementation reports in:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-name.md
```

Use short, factual worklogs: goal, scope, decisions, files changed, verification, and open questions.

## Documentation Rules

- Keep Canon, Research, Implementation, Mechanics cards, and ADR separated.
- Canon contains accepted rules, not hypotheses.
- Mechanics cards route to Canon and list TDD checks; they must not introduce new rules.
- Research may contain alternatives and unresolved ideas, but is not implementation authority.
- Fundamental changes require an ADR.
- Prefer moving duplicated content to the correct single place and linking to it.
- Preserve Obsidian usability: use stable relative links where useful.

## Engineering Rules

- Do not implement behavior that contradicts [[docs/PRINCIPLES|Principles]].
- Before TDD planning for behavior or simulation mechanics, use [[docs/mechanics/INDEX|Mechanics Index]] and the relevant mechanics card.
- Do not add hardcoded biological shortcuts such as species IDs, cell classes for roles, organs, brains, predators, or scripted behaviors unless an ADR explicitly changes the rules.
- Favor deterministic базова модель behavior and explicit config boundaries.
- Keep changes scoped to the requested task.
