# AGENTS.MD

## Project Context

This repository is currently documentation-first. Treat `docs/` as the source of truth for future implementation.

Documentation language is Ukrainian. Code identifiers and technical class-like terms may stay in English when already used by the docs.

## Required Reading

Before changing requirements, architecture, or future code, read:

1. `docs/PRINCIPLES.md`
2. `docs/GLOSSARY.md`
3. `docs/ROADMAP.md`
4. Relevant Canon document under `docs/`
5. Relevant ADR, if it exists

If a required rule is missing, do not invent it silently. Record it as an open question, TODO, or proposed ADR.

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

- Keep Canon, Research, and ADR separated.
- Canon contains accepted rules, not hypotheses.
- Research may contain alternatives and unresolved ideas, but is not implementation authority.
- Fundamental changes require an ADR.
- Prefer moving duplicated content to the correct single place and linking to it.
- Preserve Obsidian usability: use stable relative links where useful.

## Engineering Rules

- Do not implement behavior that contradicts `docs/PRINCIPLES.md`.
- Do not add hardcoded biological shortcuts such as species IDs, cell classes for roles, organs, brains, predators, or scripted behaviors unless an ADR explicitly changes the rules.
- Favor deterministic базова модель behavior and explicit config boundaries.
- Keep changes scoped to the requested task.


